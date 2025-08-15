// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::{
    net::SocketAddr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};
use tokio::time::sleep;
use tonic::{Request, Response, Status};
use tonic_health::server::{health_reporter, HealthReporter};

use libra2_protos::txnstream::v1::{
    txn_stream_server::{TxnStream, TxnStreamServer},
    ChainIdResponse, Empty, ServerInfoResponse, TransactionOutput, TransactionsRequest,
    TransactionsResponse,
};

/// Service context pulling chain id and a read handle to committed transactions.
/// Replace the MockStore with your real storage adapter.
#[derive(Clone)]
pub struct ServiceContext {
    chain_id: u8,
    store: Arc<dyn TxnStore + Send + Sync>,
}

impl ServiceContext {
    pub async fn from_env() -> Result<Self> {
        // TODO: wire real chain id and storage
        // e.g., read from on-chain config or your NodeConfig
        let chain_id = 4u8;
        let store = Arc::new(MockStore::default());
        Ok(Self { chain_id, store })
    }
}

#[tonic::async_trait]
pub trait TxnStore {
    async fn latest_version(&self) -> Result<u64>;
    async fn fetch_batch(
        &self,
        start: u64,
        limit: u64,
        include_events: bool,
    ) -> Result<Vec<TransactionOutput>>;
}

// ---- TEMP MOCK (replace with real storage) ----
#[derive(Default)]
struct MockStore;
#[tonic::async_trait]
impl TxnStore for MockStore {
    async fn latest_version(&self) -> Result<u64> {
        Ok(0)
    }
    async fn fetch_batch(
        &self,
        start: u64,
        limit: u64,
        _include_events: bool,
    ) -> Result<Vec<TransactionOutput>> {
        let mut out = vec![];
        for v in start..start + limit {
            out.push(TransactionOutput {
                version: v,
                txn: bcs::to_bytes(&format!("tx{v}")).unwrap(),
                events: vec![],
                info: vec![],
            });
        }
        Ok(out)
    }
}
// ----------------------------------------------

#[derive(Clone)]
pub struct TxnStreamSvc {
    ctx: ServiceContext,
}

impl TxnStreamSvc {
    pub fn new(ctx: ServiceContext) -> Self {
        Self { ctx }
    }
}

#[tonic::async_trait]
impl TxnStream for TxnStreamSvc {
    async fn get_chain_id(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<ChainIdResponse>, Status> {
        Ok(Response::new(ChainIdResponse {
            chain_id: self.ctx.chain_id as u32,
        }))
    }

    async fn get_server_info(
        &self,
        _req: Request<Empty>,
    ) -> Result<Response<ServerInfoResponse>, Status> {
        let build_ts = option_env!("VERGEN_BUILD_TIMESTAMP")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or_else(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            });

        Ok(Response::new(ServerInfoResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_timestamp: build_ts,
        }))
    }

    type StreamTransactionsStream =
        tokio_stream::wrappers::ReceiverStream<Result<TransactionsResponse, Status>>;

    async fn stream_transactions(
        &self,
        req: Request<TransactionsRequest>,
    ) -> Result<Response<Self::StreamTransactionsStream>, Status> {
        let params = req.into_inner();
        let mut next = params.start_version;
        let include_events = params.include_events;
        let batch_size = params.batch_size.clamp(1, 1000) as u64;
        let max_wait = Duration::from_millis(params.max_wait_ms as u64);

        let (tx, rx) = tokio::sync::mpsc::channel(8);
        let store = self.ctx.store.clone();

        tokio::spawn(async move {
            loop {
                match store.latest_version().await {
                    Ok(latest) if next <= latest => {
                        let limit = std::cmp::min(batch_size, (latest + 1).saturating_sub(next));
                        match store.fetch_batch(next, limit, include_events).await {
                            Ok(batch) => {
                                next += batch.len() as u64;
                                if tx.send(Ok(TransactionsResponse { batch })).await.is_err() {
                                    break; // client disconnected
                                }
                            },
                            Err(e) => {
                                let _ = tx
                                    .send(Err(Status::internal(format!("fetch error: {e:#}"))))
                                    .await;
                                break;
                            },
                        }
                    },
                    Ok(_) => {
                        if max_wait.is_zero() {
                            // short-poll: return empty to wake client
                            if tx
                                .send(Ok(TransactionsResponse { batch: vec![] }))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        } else {
                            sleep(max_wait).await;
                        }
                    },
                    Err(e) => {
                        let _ = tx
                            .send(Err(Status::unavailable(format!("store not ready: {e:#}"))))
                            .await;
                        break;
                    },
                }
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
            rx,
        )))
    }
}

/// Start the gRPC server on given addr (e.g. "127.0.0.1:50051")
pub async fn serve(addr: &str) -> Result<()> {
    let addr = addr.parse().context("invalid addr")?;
    let ctx = ServiceContext::from_env().await?;

    let (mut health, health_svc) = health_reporter();
    health.set_serving::<TxnStreamServer<TxnStreamSvc>>().await;

    let svc = TxnStreamServer::new(TxnStreamSvc::new(ctx));

    tonic::transport::Server::builder()
        .add_service(health_svc)
        .add_service(svc)
        .serve_with_shutdown(addr, async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .context("serve txnstream")?;

    Ok(())
}

/// Run the gRPC server using a CLI-style signature.
/// `rest` is currently unused but kept for API compatibility.
pub async fn run_server(addr: SocketAddr, _rest: String) -> Result<()> {
    serve(&addr.to_string()).await
}
