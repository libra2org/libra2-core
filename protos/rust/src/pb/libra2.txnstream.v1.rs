// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// Manually generated prost/tonic definitions for libra2.txnstream.v1
use ::prost::alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct Empty {}

#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct ChainIdResponse {
    #[prost(uint32, tag="1")]
    pub chain_id: u32,
}

#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct ServerInfoResponse {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub build_timestamp: u64,
}

#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct TransactionsRequest {
    #[prost(uint64, tag="1")]
    pub start_version: u64,
    #[prost(bool, tag="2")]
    pub include_events: bool,
    #[prost(uint32, tag="3")]
    pub batch_size: u32,
    #[prost(uint32, tag="4")]
    pub max_wait_ms: u32,
    #[prost(bool, tag="5")]
    pub exclude_ledger_changes: bool,
}

#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct TransactionOutput {
    #[prost(uint64, tag="1")]
    pub version: u64,
    #[prost(bytes, tag="2")]
    pub txn: Vec<u8>,
    #[prost(bytes, repeated, tag="3")]
    pub events: Vec<Vec<u8>>,
    #[prost(bytes, tag="4")]
    pub info: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, ::prost::Message, ::serde::Serialize, ::serde::Deserialize)]
pub struct TransactionsResponse {
    #[prost(message, repeated, tag="1")]
    pub batch: Vec<TransactionOutput>,
}

/// gRPC client and server implementations
pub mod txn_stream_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::wildcard_imports, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct TxnStreamClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TxnStreamClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TxnStreamClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        /// Check if the service is ready.
        pub async fn get_chain_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::ChainIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/libra2.txnstream.v1.TxnStream/GetChainId");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_server_info(
            &mut self,
            request: impl tonic::IntoRequest<super::Empty>,
        ) -> Result<tonic::Response<super::ServerInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/libra2.txnstream.v1.TxnStream/GetServerInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stream_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::TransactionsResponse>>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| tonic::Status::unknown(format!("Service was not ready: {}", e.into())))?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/libra2.txnstream.v1.TxnStream/StreamTransactions");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}

pub mod txn_stream_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::wildcard_imports, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[async_trait]
    pub trait TxnStream: Send + Sync + 'static {
        async fn get_chain_id(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::ChainIdResponse>, tonic::Status>;
        async fn get_server_info(
            &self,
            request: tonic::Request<super::Empty>,
        ) -> Result<tonic::Response<super::ServerInfoResponse>, tonic::Status>;
        type StreamTransactionsStream: tonic::codegen::tokio_stream::Stream<
                Item = Result<super::TransactionsResponse, tonic::Status>,
            > + Send
            + 'static;
        async fn stream_transactions(
            &self,
            request: tonic::Request<super::TransactionsRequest>,
        ) -> Result<tonic::Response<Self::StreamTransactionsStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TxnStreamServer<T: TxnStream> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: TxnStream> TxnStreamServer<T> {
        pub fn new(inner: T) -> Self { Self::from_arc(Arc::new(inner)) }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TxnStreamServer<T>
    where
        T: TxnStream,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/libra2.txnstream.v1.TxnStream/GetChainId" => {
                    struct GetChainIdSvc<T: TxnStream>(pub Arc<T>);
                    impl<T: TxnStream> tonic::server::UnaryService<super::Empty> for GetChainIdSvc<T> {
                        type Response = super::ChainIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Empty>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { T::get_chain_id(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let accept = self.accept_compression_encodings;
                    let send = self.send_compression_encodings;
                    let max_dec = self.max_decoding_message_size;
                    let max_enc = self.max_encoding_message_size;
                    let fut = async move {
                        let method = GetChainIdSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept, send)
                            .apply_max_message_size_config(max_dec, max_enc);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/libra2.txnstream.v1.TxnStream/GetServerInfo" => {
                    struct GetServerInfoSvc<T: TxnStream>(pub Arc<T>);
                    impl<T: TxnStream> tonic::server::UnaryService<super::Empty> for GetServerInfoSvc<T> {
                        type Response = super::ServerInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Empty>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { T::get_server_info(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let accept = self.accept_compression_encodings;
                    let send = self.send_compression_encodings;
                    let max_dec = self.max_decoding_message_size;
                    let max_enc = self.max_encoding_message_size;
                    let fut = async move {
                        let method = GetServerInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept, send)
                            .apply_max_message_size_config(max_dec, max_enc);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/libra2.txnstream.v1.TxnStream/StreamTransactions" => {
                    struct StreamTransactionsSvc<T: TxnStream>(pub Arc<T>);
                    impl<T: TxnStream> tonic::server::ServerStreamingService<super::TransactionsRequest>
                        for StreamTransactionsSvc<T>
                    {
                        type Response = super::TransactionsResponse;
                        type ResponseStream = T::StreamTransactionsStream;
                        type Future = BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::TransactionsRequest>) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { T::stream_transactions(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let accept = self.accept_compression_encodings;
                    let send = self.send_compression_encodings;
                    let max_dec = self.max_decoding_message_size;
                    let max_enc = self.max_encoding_message_size;
                    let fut = async move {
                        let method = StreamTransactionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(accept, send)
                            .apply_max_message_size_config(max_dec, max_enc);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(tonic::body::empty_body());
                        let headers = response.headers_mut();
                        headers.insert(tonic::Status::GRPC_STATUS, (tonic::Code::Unimplemented as i32).into());
                        headers.insert(http::header::CONTENT_TYPE, tonic::metadata::GRPC_CONTENT_TYPE);
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T: TxnStream> Clone for TxnStreamServer<T> {
        fn clone(&self) -> Self { Self::from_arc(self.inner.clone()) }
    }
    impl<T: TxnStream> tonic::server::NamedService for TxnStreamServer<T> {
        const NAME: &'static str = "libra2.txnstream.v1.TxnStream";
    }
}
