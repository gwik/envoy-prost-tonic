// @generated
/// Generated client implementations.
pub mod cluster_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ClusterDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClusterDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> ClusterDiscoveryServiceClient<T>
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
        ) -> ClusterDiscoveryServiceClient<InterceptedService<T, F>>
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
            ClusterDiscoveryServiceClient::new(
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
        pub async fn stream_clusters(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DiscoveryResponse,
                >,
            >,
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
                "/envoy.service.cluster.v3.ClusterDiscoveryService/StreamClusters",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn delta_clusters(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DeltaDiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                >,
            >,
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
                "/envoy.service.cluster.v3.ClusterDiscoveryService/DeltaClusters",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn fetch_clusters(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
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
                "/envoy.service.cluster.v3.ClusterDiscoveryService/FetchClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod cluster_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ClusterDiscoveryServiceServer.
    #[async_trait]
    pub trait ClusterDiscoveryService: Send + Sync + 'static {
        ///Server streaming response type for the StreamClusters method.
        type StreamClustersStream: futures_core::Stream<
                Item = Result<
                    super::super::super::discovery::v3::DiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_clusters(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::discovery::v3::DiscoveryRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamClustersStream>, tonic::Status>;
        ///Server streaming response type for the DeltaClusters method.
        type DeltaClustersStream: futures_core::Stream<
                Item = Result<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        ///
        async fn delta_clusters(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                >,
            >,
        ) -> Result<tonic::Response<Self::DeltaClustersStream>, tonic::Status>;
        ///
        async fn fetch_clusters(
            &self,
            request: tonic::Request<super::super::super::discovery::v3::DiscoveryRequest>,
        ) -> Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ClusterDiscoveryServiceServer<T: ClusterDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ClusterDiscoveryService> ClusterDiscoveryServiceServer<T> {
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
    for ClusterDiscoveryServiceServer<T>
    where
        T: ClusterDiscoveryService,
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
                "/envoy.service.cluster.v3.ClusterDiscoveryService/StreamClusters" => {
                    #[allow(non_camel_case_types)]
                    struct StreamClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ClusterDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for StreamClustersSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type ResponseStream = T::StreamClustersStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stream_clusters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamClustersSvc(inner);
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
                "/envoy.service.cluster.v3.ClusterDiscoveryService/DeltaClusters" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ClusterDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::discovery::v3::DeltaDiscoveryRequest,
                    > for DeltaClustersSvc<T> {
                        type Response = super::super::super::discovery::v3::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaClustersStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delta_clusters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaClustersSvc(inner);
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
                "/envoy.service.cluster.v3.ClusterDiscoveryService/FetchClusters" => {
                    #[allow(non_camel_case_types)]
                    struct FetchClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ClusterDiscoveryService,
                    > tonic::server::UnaryService<
                        super::super::super::discovery::v3::DiscoveryRequest,
                    > for FetchClustersSvc<T> {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::discovery::v3::DiscoveryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_clusters(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchClustersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
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
    impl<T: ClusterDiscoveryService> Clone for ClusterDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ClusterDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ClusterDiscoveryService> tonic::server::NamedService
    for ClusterDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.cluster.v3.ClusterDiscoveryService";
    }
}
