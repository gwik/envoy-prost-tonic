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
                "/envoy.api.v2.ClusterDiscoveryService/StreamClusters",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn delta_clusters(
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
                "/envoy.api.v2.ClusterDiscoveryService/DeltaClusters",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn fetch_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status> {
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
                "/envoy.api.v2.ClusterDiscoveryService/FetchClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod cluster_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ClusterDiscoveryServiceServer.
    #[async_trait]
    pub trait ClusterDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamClusters method.
        type StreamClustersStream: futures_core::Stream<
                Item = Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_clusters(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::StreamClustersStream>, tonic::Status>;
        /// Server streaming response type for the DeltaClusters method.
        type DeltaClustersStream: futures_core::Stream<
                Item = Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn delta_clusters(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::DeltaClustersStream>, tonic::Status>;
        async fn fetch_clusters(
            &self,
            request: tonic::Request<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status>;
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
                "/envoy.api.v2.ClusterDiscoveryService/StreamClusters" => {
                    #[allow(non_camel_case_types)]
                    struct StreamClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ClusterDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamClustersSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamClustersStream;
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
                "/envoy.api.v2.ClusterDiscoveryService/DeltaClusters" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ClusterDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaClustersSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaClustersStream;
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
                "/envoy.api.v2.ClusterDiscoveryService/FetchClusters" => {
                    #[allow(non_camel_case_types)]
                    struct FetchClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ClusterDiscoveryService,
                    > tonic::server::UnaryService<super::DiscoveryRequest>
                    for FetchClustersSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoveryRequest>,
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
        const NAME: &'static str = "envoy.api.v2.ClusterDiscoveryService";
    }
}
/// Generated client implementations.
pub mod endpoint_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct EndpointDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EndpointDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> EndpointDiscoveryServiceClient<T>
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
        ) -> EndpointDiscoveryServiceClient<InterceptedService<T, F>>
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
            EndpointDiscoveryServiceClient::new(
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
        pub async fn stream_endpoints(
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
                "/envoy.api.v2.EndpointDiscoveryService/StreamEndpoints",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn delta_endpoints(
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
                "/envoy.api.v2.EndpointDiscoveryService/DeltaEndpoints",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn fetch_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status> {
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
                "/envoy.api.v2.EndpointDiscoveryService/FetchEndpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod endpoint_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EndpointDiscoveryServiceServer.
    #[async_trait]
    pub trait EndpointDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamEndpoints method.
        type StreamEndpointsStream: futures_core::Stream<
                Item = Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_endpoints(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::StreamEndpointsStream>, tonic::Status>;
        /// Server streaming response type for the DeltaEndpoints method.
        type DeltaEndpointsStream: futures_core::Stream<
                Item = Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn delta_endpoints(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::DeltaEndpointsStream>, tonic::Status>;
        async fn fetch_endpoints(
            &self,
            request: tonic::Request<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct EndpointDiscoveryServiceServer<T: EndpointDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EndpointDiscoveryService> EndpointDiscoveryServiceServer<T> {
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
    for EndpointDiscoveryServiceServer<T>
    where
        T: EndpointDiscoveryService,
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
                "/envoy.api.v2.EndpointDiscoveryService/StreamEndpoints" => {
                    #[allow(non_camel_case_types)]
                    struct StreamEndpointsSvc<T: EndpointDiscoveryService>(pub Arc<T>);
                    impl<
                        T: EndpointDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamEndpointsSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamEndpointsStream;
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
                                (*inner).stream_endpoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamEndpointsSvc(inner);
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
                "/envoy.api.v2.EndpointDiscoveryService/DeltaEndpoints" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaEndpointsSvc<T: EndpointDiscoveryService>(pub Arc<T>);
                    impl<
                        T: EndpointDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaEndpointsSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaEndpointsStream;
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
                                (*inner).delta_endpoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaEndpointsSvc(inner);
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
                "/envoy.api.v2.EndpointDiscoveryService/FetchEndpoints" => {
                    #[allow(non_camel_case_types)]
                    struct FetchEndpointsSvc<T: EndpointDiscoveryService>(pub Arc<T>);
                    impl<
                        T: EndpointDiscoveryService,
                    > tonic::server::UnaryService<super::DiscoveryRequest>
                    for FetchEndpointsSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoveryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_endpoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchEndpointsSvc(inner);
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
    impl<T: EndpointDiscoveryService> Clone for EndpointDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: EndpointDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EndpointDiscoveryService> tonic::server::NamedService
    for EndpointDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.api.v2.EndpointDiscoveryService";
    }
}
/// Generated client implementations.
pub mod listener_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ListenerDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ListenerDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> ListenerDiscoveryServiceClient<T>
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
        ) -> ListenerDiscoveryServiceClient<InterceptedService<T, F>>
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
            ListenerDiscoveryServiceClient::new(
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
        pub async fn delta_listeners(
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
                "/envoy.api.v2.ListenerDiscoveryService/DeltaListeners",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn stream_listeners(
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
                "/envoy.api.v2.ListenerDiscoveryService/StreamListeners",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn fetch_listeners(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status> {
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
                "/envoy.api.v2.ListenerDiscoveryService/FetchListeners",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod listener_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ListenerDiscoveryServiceServer.
    #[async_trait]
    pub trait ListenerDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the DeltaListeners method.
        type DeltaListenersStream: futures_core::Stream<
                Item = Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn delta_listeners(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::DeltaListenersStream>, tonic::Status>;
        /// Server streaming response type for the StreamListeners method.
        type StreamListenersStream: futures_core::Stream<
                Item = Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_listeners(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::StreamListenersStream>, tonic::Status>;
        async fn fetch_listeners(
            &self,
            request: tonic::Request<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ListenerDiscoveryServiceServer<T: ListenerDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ListenerDiscoveryService> ListenerDiscoveryServiceServer<T> {
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
    for ListenerDiscoveryServiceServer<T>
    where
        T: ListenerDiscoveryService,
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
                "/envoy.api.v2.ListenerDiscoveryService/DeltaListeners" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaListenersSvc<T: ListenerDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ListenerDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaListenersSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaListenersStream;
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
                                (*inner).delta_listeners(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaListenersSvc(inner);
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
                "/envoy.api.v2.ListenerDiscoveryService/StreamListeners" => {
                    #[allow(non_camel_case_types)]
                    struct StreamListenersSvc<T: ListenerDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ListenerDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamListenersSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamListenersStream;
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
                                (*inner).stream_listeners(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamListenersSvc(inner);
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
                "/envoy.api.v2.ListenerDiscoveryService/FetchListeners" => {
                    #[allow(non_camel_case_types)]
                    struct FetchListenersSvc<T: ListenerDiscoveryService>(pub Arc<T>);
                    impl<
                        T: ListenerDiscoveryService,
                    > tonic::server::UnaryService<super::DiscoveryRequest>
                    for FetchListenersSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoveryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_listeners(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchListenersSvc(inner);
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
    impl<T: ListenerDiscoveryService> Clone for ListenerDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ListenerDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ListenerDiscoveryService> tonic::server::NamedService
    for ListenerDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.api.v2.ListenerDiscoveryService";
    }
}
/// Generated client implementations.
pub mod route_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RouteDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RouteDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> RouteDiscoveryServiceClient<T>
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
        ) -> RouteDiscoveryServiceClient<InterceptedService<T, F>>
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
            RouteDiscoveryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn stream_routes(
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
                "/envoy.api.v2.RouteDiscoveryService/StreamRoutes",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn delta_routes(
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
                "/envoy.api.v2.RouteDiscoveryService/DeltaRoutes",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn fetch_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status> {
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
                "/envoy.api.v2.RouteDiscoveryService/FetchRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod route_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RouteDiscoveryServiceServer.
    #[async_trait]
    pub trait RouteDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamRoutes method.
        type StreamRoutesStream: futures_core::Stream<
                Item = Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_routes(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::StreamRoutesStream>, tonic::Status>;
        /// Server streaming response type for the DeltaRoutes method.
        type DeltaRoutesStream: futures_core::Stream<
                Item = Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        async fn delta_routes(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::DeltaRoutesStream>, tonic::Status>;
        ///
        async fn fetch_routes(
            &self,
            request: tonic::Request<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RouteDiscoveryServiceServer<T: RouteDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RouteDiscoveryService> RouteDiscoveryServiceServer<T> {
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
    for RouteDiscoveryServiceServer<T>
    where
        T: RouteDiscoveryService,
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
                "/envoy.api.v2.RouteDiscoveryService/StreamRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct StreamRoutesSvc<T: RouteDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RouteDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamRoutesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamRoutesStream;
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
                                (*inner).stream_routes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamRoutesSvc(inner);
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
                "/envoy.api.v2.RouteDiscoveryService/DeltaRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaRoutesSvc<T: RouteDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RouteDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaRoutesSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaRoutesStream;
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
                                (*inner).delta_routes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaRoutesSvc(inner);
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
                "/envoy.api.v2.RouteDiscoveryService/FetchRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct FetchRoutesSvc<T: RouteDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RouteDiscoveryService,
                    > tonic::server::UnaryService<super::DiscoveryRequest>
                    for FetchRoutesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoveryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_routes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchRoutesSvc(inner);
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
    impl<T: RouteDiscoveryService> Clone for RouteDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RouteDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RouteDiscoveryService> tonic::server::NamedService
    for RouteDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.api.v2.RouteDiscoveryService";
    }
}
/// Generated client implementations.
pub mod virtual_host_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Virtual Host Discovery Service (VHDS) is used to dynamically update the list of virtual hosts for
 a given RouteConfiguration. If VHDS is configured a virtual host list update will be triggered
 during the processing of an HTTP request if a route for the request cannot be resolved. The
 :ref:`resource_names_subscribe <envoy_api_field_DeltaDiscoveryRequest.resource_names_subscribe>`
 field contains a list of virtual host names or aliases to track. The contents of an alias would
 be the contents of a *host* or *authority* header used to make an http request. An xDS server
 will match an alias to a virtual host based on the content of :ref:`domains'
 <envoy_api_field_route.VirtualHost.domains>` field. The *resource_names_unsubscribe* field
 contains a list of virtual host names that have been :ref:`unsubscribed
 <xds_protocol_unsubscribe>` from the routing table associated with the RouteConfiguration.
*/
    #[derive(Debug, Clone)]
    pub struct VirtualHostDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VirtualHostDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> VirtualHostDiscoveryServiceClient<T>
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
        ) -> VirtualHostDiscoveryServiceClient<InterceptedService<T, F>>
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
            VirtualHostDiscoveryServiceClient::new(
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
        pub async fn delta_virtual_hosts(
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
                "/envoy.api.v2.VirtualHostDiscoveryService/DeltaVirtualHosts",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod virtual_host_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VirtualHostDiscoveryServiceServer.
    #[async_trait]
    pub trait VirtualHostDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the DeltaVirtualHosts method.
        type DeltaVirtualHostsStream: futures_core::Stream<
                Item = Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn delta_virtual_hosts(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::DeltaVirtualHostsStream>, tonic::Status>;
    }
    /** Virtual Host Discovery Service (VHDS) is used to dynamically update the list of virtual hosts for
 a given RouteConfiguration. If VHDS is configured a virtual host list update will be triggered
 during the processing of an HTTP request if a route for the request cannot be resolved. The
 :ref:`resource_names_subscribe <envoy_api_field_DeltaDiscoveryRequest.resource_names_subscribe>`
 field contains a list of virtual host names or aliases to track. The contents of an alias would
 be the contents of a *host* or *authority* header used to make an http request. An xDS server
 will match an alias to a virtual host based on the content of :ref:`domains'
 <envoy_api_field_route.VirtualHost.domains>` field. The *resource_names_unsubscribe* field
 contains a list of virtual host names that have been :ref:`unsubscribed
 <xds_protocol_unsubscribe>` from the routing table associated with the RouteConfiguration.
*/
    #[derive(Debug)]
    pub struct VirtualHostDiscoveryServiceServer<T: VirtualHostDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VirtualHostDiscoveryService> VirtualHostDiscoveryServiceServer<T> {
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
    for VirtualHostDiscoveryServiceServer<T>
    where
        T: VirtualHostDiscoveryService,
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
                "/envoy.api.v2.VirtualHostDiscoveryService/DeltaVirtualHosts" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaVirtualHostsSvc<T: VirtualHostDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: VirtualHostDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaVirtualHostsSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaVirtualHostsStream;
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
                                (*inner).delta_virtual_hosts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaVirtualHostsSvc(inner);
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
    impl<T: VirtualHostDiscoveryService> Clone for VirtualHostDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: VirtualHostDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VirtualHostDiscoveryService> tonic::server::NamedService
    for VirtualHostDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.api.v2.VirtualHostDiscoveryService";
    }
}
/// Generated client implementations.
pub mod scoped_routes_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ScopedRoutesDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ScopedRoutesDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> ScopedRoutesDiscoveryServiceClient<T>
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
        ) -> ScopedRoutesDiscoveryServiceClient<InterceptedService<T, F>>
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
            ScopedRoutesDiscoveryServiceClient::new(
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
        pub async fn stream_scoped_routes(
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
                "/envoy.api.v2.ScopedRoutesDiscoveryService/StreamScopedRoutes",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn delta_scoped_routes(
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
                "/envoy.api.v2.ScopedRoutesDiscoveryService/DeltaScopedRoutes",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn fetch_scoped_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status> {
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
                "/envoy.api.v2.ScopedRoutesDiscoveryService/FetchScopedRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod scoped_routes_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ScopedRoutesDiscoveryServiceServer.
    #[async_trait]
    pub trait ScopedRoutesDiscoveryService: Send + Sync + 'static {
        /// Server streaming response type for the StreamScopedRoutes method.
        type StreamScopedRoutesStream: futures_core::Stream<
                Item = Result<super::DiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_scoped_routes(
            &self,
            request: tonic::Request<tonic::Streaming<super::DiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::StreamScopedRoutesStream>, tonic::Status>;
        /// Server streaming response type for the DeltaScopedRoutes method.
        type DeltaScopedRoutesStream: futures_core::Stream<
                Item = Result<super::DeltaDiscoveryResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn delta_scoped_routes(
            &self,
            request: tonic::Request<tonic::Streaming<super::DeltaDiscoveryRequest>>,
        ) -> Result<tonic::Response<Self::DeltaScopedRoutesStream>, tonic::Status>;
        async fn fetch_scoped_routes(
            &self,
            request: tonic::Request<super::DiscoveryRequest>,
        ) -> Result<tonic::Response<super::DiscoveryResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ScopedRoutesDiscoveryServiceServer<T: ScopedRoutesDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ScopedRoutesDiscoveryService> ScopedRoutesDiscoveryServiceServer<T> {
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
    for ScopedRoutesDiscoveryServiceServer<T>
    where
        T: ScopedRoutesDiscoveryService,
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
                "/envoy.api.v2.ScopedRoutesDiscoveryService/StreamScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct StreamScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::StreamingService<super::DiscoveryRequest>
                    for StreamScopedRoutesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type ResponseStream = T::StreamScopedRoutesStream;
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
                                (*inner).stream_scoped_routes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamScopedRoutesSvc(inner);
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
                "/envoy.api.v2.ScopedRoutesDiscoveryService/DeltaScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::StreamingService<super::DeltaDiscoveryRequest>
                    for DeltaScopedRoutesSvc<T> {
                        type Response = super::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaScopedRoutesStream;
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
                                (*inner).delta_scoped_routes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaScopedRoutesSvc(inner);
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
                "/envoy.api.v2.ScopedRoutesDiscoveryService/FetchScopedRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct FetchScopedRoutesSvc<T: ScopedRoutesDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ScopedRoutesDiscoveryService,
                    > tonic::server::UnaryService<super::DiscoveryRequest>
                    for FetchScopedRoutesSvc<T> {
                        type Response = super::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoveryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_scoped_routes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchScopedRoutesSvc(inner);
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
    impl<T: ScopedRoutesDiscoveryService> Clone
    for ScopedRoutesDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ScopedRoutesDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ScopedRoutesDiscoveryService> tonic::server::NamedService
    for ScopedRoutesDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.api.v2.ScopedRoutesDiscoveryService";
    }
}
