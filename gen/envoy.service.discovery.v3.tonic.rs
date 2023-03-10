// @generated
/// Generated client implementations.
pub mod aggregated_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AggregatedDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AggregatedDiscoveryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AggregatedDiscoveryServiceClient<T>
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
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AggregatedDiscoveryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AggregatedDiscoveryServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /** This is a gRPC-only API.
*/
        pub async fn stream_aggregated_resources(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::DiscoveryRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DiscoveryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn delta_aggregated_resources(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::DeltaDiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DeltaDiscoveryResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/DeltaAggregatedResources",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod aggregated_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AggregatedDiscoveryServiceServer.
    #[async_trait]
    pub trait AggregatedDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamAggregatedResources method.
        type StreamAggregatedResourcesStream: futures_core::Stream<
                Item = Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /** This is a gRPC-only API.
*/
        async fn stream_aggregated_resources(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> Result<
            tonic::Response<Self::StreamAggregatedResourcesStream>,
            tonic::Status,
        >;
        /// Server streaming response type for the DeltaAggregatedResources method.
        type DeltaAggregatedResourcesStream: futures_core::Stream<
                Item = Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        async fn delta_aggregated_resources(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> Result<
            tonic::Response<Self::DeltaAggregatedResourcesStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AggregatedDiscoveryServiceServer<T: AggregatedDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AggregatedDiscoveryService> AggregatedDiscoveryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for AggregatedDiscoveryServiceServer<T>
    where
        T: AggregatedDiscoveryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/StreamAggregatedResources" => {
                    #[allow(non_camel_case_types)]
                    struct StreamAggregatedResourcesSvc<T: AggregatedDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AggregatedDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamAggregatedResourcesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamAggregatedResourcesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DiscoveryRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stream_aggregated_resources(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamAggregatedResourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.service.discovery.v3.AggregatedDiscoveryService/DeltaAggregatedResources" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaAggregatedResourcesSvc<T: AggregatedDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AggregatedDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaAggregatedResourcesSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaAggregatedResourcesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::DeltaDiscoveryRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delta_aggregated_resources(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaAggregatedResourcesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AggregatedDiscoveryService> Clone for AggregatedDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AggregatedDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AggregatedDiscoveryService> tonic::server::NamedService
    for AggregatedDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.discovery.v3.AggregatedDiscoveryService";
    }
}
