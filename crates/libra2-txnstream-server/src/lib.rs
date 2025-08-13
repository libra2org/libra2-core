use std::net::SocketAddr;
use std::pin::Pin;

use futures::Stream;
use tokio_stream::iter;
use tonic::{transport::Server, Request, Response, Status};
use tonic_health::server::health_reporter;

use libra2_protos_txnstream::txnstream::txn_stream_server::{TxnStream, TxnStreamServer};
use libra2_protos_txnstream::txnstream::{
    ChainIdResponse, Empty, ServerInfoResponse, TransactionsRequest, TransactionsResponse,
};

/// Basic transaction stream service.
#[derive(Clone, Debug)]
pub struct TxnStreamService {
    chain_id: u32,
}

#[tonic::async_trait]
impl TxnStream for TxnStreamService {
    async fn get_chain_id(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ChainIdResponse>, Status> {
        Ok(Response::new(ChainIdResponse {
            chain_id: self.chain_id,
        }))
    }

    async fn get_server_info(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ServerInfoResponse>, Status> {
        Ok(Response::new(ServerInfoResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_timestamp: 0,
        }))
    }

    type StreamTransactionsStream =
        Pin<Box<dyn Stream<Item = Result<TransactionsResponse, Status>> + Send + 'static>>;

    async fn stream_transactions(
        &self,
        _request: Request<TransactionsRequest>,
    ) -> Result<Response<Self::StreamTransactionsStream>, Status> {
        let stream = iter(Vec::<Result<TransactionsResponse, Status>>::new());
        Ok(Response::new(
            Box::pin(stream) as Self::StreamTransactionsStream
        ))
    }
}

/// Start the gRPC server on the given address.
pub async fn serve(addr: SocketAddr, chain_id: u32) -> Result<(), anyhow::Error> {
    let service = TxnStreamService { chain_id };
    let (mut health_reporter, health_service) = health_reporter();
    health_reporter
        .set_serving::<TxnStreamServer<TxnStreamService>>()
        .await;

    Server::builder()
        .add_service(health_service)
        .add_service(TxnStreamServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_chain_id() {
        let service = TxnStreamService { chain_id: 7 };
        let response = service.get_chain_id(Request::new(Empty {})).await.unwrap();
        assert_eq!(response.into_inner().chain_id, 7);
    }
}
