// @generated
/// Generated client implementations.
pub mod aggregated_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** See https://github.com/envoyproxy/envoy-api#apis for a description of the role of
 ADS and how it is intended to be used by a management server. ADS requests
 have the same structure as their singleton xDS counterparts, but can
 multiplex many resource types on a single stream. The type_url in the
 DiscoveryRequest/DiscoveryResponse provides sufficient information to recover
 the multiplexed singleton APIs at the Envoy instance and management server.
*/
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
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::super::api::v2::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::api::v2::DiscoveryResponse,
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
                "/envoy.service.discovery.v2.AggregatedDiscoveryService/StreamAggregatedResources",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn delta_aggregated_resources(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::super::api::v2::DeltaDiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::api::v2::DeltaDiscoveryResponse,
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
                "/envoy.service.discovery.v2.AggregatedDiscoveryService/DeltaAggregatedResources",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod aggregated_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AggregatedDiscoveryServiceServer.
    #[async_trait]
    pub trait AggregatedDiscoveryService: Send + Sync + 'static {
        ///Server streaming response type for the StreamAggregatedResources method.
        type StreamAggregatedResourcesStream: futures_core::Stream<
                Item = Result<
                    super::super::super::super::api::v2::DiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /** This is a gRPC-only API.
*/
        async fn stream_aggregated_resources(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::super::api::v2::DiscoveryRequest>,
            >,
        ) -> Result<
            tonic::Response<Self::StreamAggregatedResourcesStream>,
            tonic::Status,
        >;
        ///Server streaming response type for the DeltaAggregatedResources method.
        type DeltaAggregatedResourcesStream: futures_core::Stream<
                Item = Result<
                    super::super::super::super::api::v2::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        ///
        async fn delta_aggregated_resources(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::super::api::v2::DeltaDiscoveryRequest,
                >,
            >,
        ) -> Result<
            tonic::Response<Self::DeltaAggregatedResourcesStream>,
            tonic::Status,
        >;
    }
    /** See https://github.com/envoyproxy/envoy-api#apis for a description of the role of
 ADS and how it is intended to be used by a management server. ADS requests
 have the same structure as their singleton xDS counterparts, but can
 multiplex many resource types on a single stream. The type_url in the
 DiscoveryRequest/DiscoveryResponse provides sufficient information to recover
 the multiplexed singleton APIs at the Envoy instance and management server.
*/
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
                "/envoy.service.discovery.v2.AggregatedDiscoveryService/StreamAggregatedResources" => {
                    #[allow(non_camel_case_types)]
                    struct StreamAggregatedResourcesSvc<T: AggregatedDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AggregatedDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::super::api::v2::DiscoveryRequest,
                    > for StreamAggregatedResourcesSvc<T> {
                        type Response = super::super::super::super::api::v2::DiscoveryResponse;
                        type ResponseStream = T::StreamAggregatedResourcesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::super::api::v2::DiscoveryRequest,
                                >,
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
                "/envoy.service.discovery.v2.AggregatedDiscoveryService/DeltaAggregatedResources" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaAggregatedResourcesSvc<T: AggregatedDiscoveryService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AggregatedDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::super::api::v2::DeltaDiscoveryRequest,
                    > for DeltaAggregatedResourcesSvc<T> {
                        type Response = super::super::super::super::api::v2::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaAggregatedResourcesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::super::api::v2::DeltaDiscoveryRequest,
                                >,
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
        const NAME: &'static str = "envoy.service.discovery.v2.AggregatedDiscoveryService";
    }
}
/// Generated client implementations.
pub mod health_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct HealthDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HealthDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> HealthDiscoveryServiceClient<T>
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
        ) -> HealthDiscoveryServiceClient<InterceptedService<T, F>>
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
            HealthDiscoveryServiceClient::new(
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
        pub async fn stream_health_check(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::HealthCheckRequestOrEndpointHealthResponse,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::HealthCheckSpecifier>>,
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
                "/envoy.service.discovery.v2.HealthDiscoveryService/StreamHealthCheck",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn fetch_health_check(
            &mut self,
            request: impl tonic::IntoRequest<
                super::HealthCheckRequestOrEndpointHealthResponse,
            >,
        ) -> Result<tonic::Response<super::HealthCheckSpecifier>, tonic::Status> {
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
                "/envoy.service.discovery.v2.HealthDiscoveryService/FetchHealthCheck",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod health_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with HealthDiscoveryServiceServer.
    #[async_trait]
    pub trait HealthDiscoveryService: Send + Sync + 'static {
        ///Server streaming response type for the StreamHealthCheck method.
        type StreamHealthCheckStream: futures_core::Stream<
                Item = Result<super::HealthCheckSpecifier, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_health_check(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::HealthCheckRequestOrEndpointHealthResponse>,
            >,
        ) -> Result<tonic::Response<Self::StreamHealthCheckStream>, tonic::Status>;
        async fn fetch_health_check(
            &self,
            request: tonic::Request<super::HealthCheckRequestOrEndpointHealthResponse>,
        ) -> Result<tonic::Response<super::HealthCheckSpecifier>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct HealthDiscoveryServiceServer<T: HealthDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: HealthDiscoveryService> HealthDiscoveryServiceServer<T> {
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
    for HealthDiscoveryServiceServer<T>
    where
        T: HealthDiscoveryService,
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
                "/envoy.service.discovery.v2.HealthDiscoveryService/StreamHealthCheck" => {
                    #[allow(non_camel_case_types)]
                    struct StreamHealthCheckSvc<T: HealthDiscoveryService>(pub Arc<T>);
                    impl<
                        T: HealthDiscoveryService,
                    > tonic::server::StreamingService<
                        super::HealthCheckRequestOrEndpointHealthResponse,
                    > for StreamHealthCheckSvc<T> {
                        type Response = super::HealthCheckSpecifier;
                        type ResponseStream = T::StreamHealthCheckStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::HealthCheckRequestOrEndpointHealthResponse,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stream_health_check(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamHealthCheckSvc(inner);
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
                "/envoy.service.discovery.v2.HealthDiscoveryService/FetchHealthCheck" => {
                    #[allow(non_camel_case_types)]
                    struct FetchHealthCheckSvc<T: HealthDiscoveryService>(pub Arc<T>);
                    impl<
                        T: HealthDiscoveryService,
                    > tonic::server::UnaryService<
                        super::HealthCheckRequestOrEndpointHealthResponse,
                    > for FetchHealthCheckSvc<T> {
                        type Response = super::HealthCheckSpecifier;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::HealthCheckRequestOrEndpointHealthResponse,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_health_check(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchHealthCheckSvc(inner);
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
    impl<T: HealthDiscoveryService> Clone for HealthDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: HealthDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: HealthDiscoveryService> tonic::server::NamedService
    for HealthDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.discovery.v2.HealthDiscoveryService";
    }
}
/// Generated client implementations.
pub mod runtime_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RuntimeDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RuntimeDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> RuntimeDiscoveryServiceClient<T>
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
        ) -> RuntimeDiscoveryServiceClient<InterceptedService<T, F>>
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
            RuntimeDiscoveryServiceClient::new(
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
        pub async fn stream_runtime(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::super::api::v2::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::api::v2::DiscoveryResponse,
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
                "/envoy.service.discovery.v2.RuntimeDiscoveryService/StreamRuntime",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn delta_runtime(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::super::api::v2::DeltaDiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::api::v2::DeltaDiscoveryResponse,
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
                "/envoy.service.discovery.v2.RuntimeDiscoveryService/DeltaRuntime",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        ///
        pub async fn fetch_runtime(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::api::v2::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::api::v2::DiscoveryResponse>,
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
                "/envoy.service.discovery.v2.RuntimeDiscoveryService/FetchRuntime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod runtime_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with RuntimeDiscoveryServiceServer.
    #[async_trait]
    pub trait RuntimeDiscoveryService: Send + Sync + 'static {
        ///Server streaming response type for the StreamRuntime method.
        type StreamRuntimeStream: futures_core::Stream<
                Item = Result<
                    super::super::super::super::api::v2::DiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_runtime(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::super::api::v2::DiscoveryRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamRuntimeStream>, tonic::Status>;
        ///Server streaming response type for the DeltaRuntime method.
        type DeltaRuntimeStream: futures_core::Stream<
                Item = Result<
                    super::super::super::super::api::v2::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        ///
        async fn delta_runtime(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::super::api::v2::DeltaDiscoveryRequest,
                >,
            >,
        ) -> Result<tonic::Response<Self::DeltaRuntimeStream>, tonic::Status>;
        ///
        async fn fetch_runtime(
            &self,
            request: tonic::Request<
                super::super::super::super::api::v2::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::api::v2::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RuntimeDiscoveryServiceServer<T: RuntimeDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RuntimeDiscoveryService> RuntimeDiscoveryServiceServer<T> {
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
    for RuntimeDiscoveryServiceServer<T>
    where
        T: RuntimeDiscoveryService,
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
                "/envoy.service.discovery.v2.RuntimeDiscoveryService/StreamRuntime" => {
                    #[allow(non_camel_case_types)]
                    struct StreamRuntimeSvc<T: RuntimeDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RuntimeDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::super::api::v2::DiscoveryRequest,
                    > for StreamRuntimeSvc<T> {
                        type Response = super::super::super::super::api::v2::DiscoveryResponse;
                        type ResponseStream = T::StreamRuntimeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::super::api::v2::DiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stream_runtime(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamRuntimeSvc(inner);
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
                "/envoy.service.discovery.v2.RuntimeDiscoveryService/DeltaRuntime" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaRuntimeSvc<T: RuntimeDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RuntimeDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::super::api::v2::DeltaDiscoveryRequest,
                    > for DeltaRuntimeSvc<T> {
                        type Response = super::super::super::super::api::v2::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaRuntimeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::super::api::v2::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delta_runtime(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaRuntimeSvc(inner);
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
                "/envoy.service.discovery.v2.RuntimeDiscoveryService/FetchRuntime" => {
                    #[allow(non_camel_case_types)]
                    struct FetchRuntimeSvc<T: RuntimeDiscoveryService>(pub Arc<T>);
                    impl<
                        T: RuntimeDiscoveryService,
                    > tonic::server::UnaryService<
                        super::super::super::super::api::v2::DiscoveryRequest,
                    > for FetchRuntimeSvc<T> {
                        type Response = super::super::super::super::api::v2::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::api::v2::DiscoveryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_runtime(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchRuntimeSvc(inner);
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
    impl<T: RuntimeDiscoveryService> Clone for RuntimeDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RuntimeDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RuntimeDiscoveryService> tonic::server::NamedService
    for RuntimeDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.discovery.v2.RuntimeDiscoveryService";
    }
}
/// Generated client implementations.
pub mod secret_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SecretDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SecretDiscoveryServiceClient<tonic::transport::Channel> {
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
    impl<T> SecretDiscoveryServiceClient<T>
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
        ) -> SecretDiscoveryServiceClient<InterceptedService<T, F>>
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
            SecretDiscoveryServiceClient::new(
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
        pub async fn delta_secrets(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::super::api::v2::DeltaDiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::api::v2::DeltaDiscoveryResponse,
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
                "/envoy.service.discovery.v2.SecretDiscoveryService/DeltaSecrets",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn stream_secrets(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::super::api::v2::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::super::super::api::v2::DiscoveryResponse,
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
                "/envoy.service.discovery.v2.SecretDiscoveryService/StreamSecrets",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        pub async fn fetch_secrets(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::api::v2::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::api::v2::DiscoveryResponse>,
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
                "/envoy.service.discovery.v2.SecretDiscoveryService/FetchSecrets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod secret_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SecretDiscoveryServiceServer.
    #[async_trait]
    pub trait SecretDiscoveryService: Send + Sync + 'static {
        ///Server streaming response type for the DeltaSecrets method.
        type DeltaSecretsStream: futures_core::Stream<
                Item = Result<
                    super::super::super::super::api::v2::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn delta_secrets(
            &self,
            request: tonic::Request<
                tonic::Streaming<
                    super::super::super::super::api::v2::DeltaDiscoveryRequest,
                >,
            >,
        ) -> Result<tonic::Response<Self::DeltaSecretsStream>, tonic::Status>;
        ///Server streaming response type for the StreamSecrets method.
        type StreamSecretsStream: futures_core::Stream<
                Item = Result<
                    super::super::super::super::api::v2::DiscoveryResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn stream_secrets(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::super::api::v2::DiscoveryRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamSecretsStream>, tonic::Status>;
        async fn fetch_secrets(
            &self,
            request: tonic::Request<
                super::super::super::super::api::v2::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::api::v2::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SecretDiscoveryServiceServer<T: SecretDiscoveryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SecretDiscoveryService> SecretDiscoveryServiceServer<T> {
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
    for SecretDiscoveryServiceServer<T>
    where
        T: SecretDiscoveryService,
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
                "/envoy.service.discovery.v2.SecretDiscoveryService/DeltaSecrets" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaSecretsSvc<T: SecretDiscoveryService>(pub Arc<T>);
                    impl<
                        T: SecretDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::super::api::v2::DeltaDiscoveryRequest,
                    > for DeltaSecretsSvc<T> {
                        type Response = super::super::super::super::api::v2::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaSecretsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::super::api::v2::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delta_secrets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeltaSecretsSvc(inner);
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
                "/envoy.service.discovery.v2.SecretDiscoveryService/StreamSecrets" => {
                    #[allow(non_camel_case_types)]
                    struct StreamSecretsSvc<T: SecretDiscoveryService>(pub Arc<T>);
                    impl<
                        T: SecretDiscoveryService,
                    > tonic::server::StreamingService<
                        super::super::super::super::api::v2::DiscoveryRequest,
                    > for StreamSecretsSvc<T> {
                        type Response = super::super::super::super::api::v2::DiscoveryResponse;
                        type ResponseStream = T::StreamSecretsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::super::api::v2::DiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stream_secrets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamSecretsSvc(inner);
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
                "/envoy.service.discovery.v2.SecretDiscoveryService/FetchSecrets" => {
                    #[allow(non_camel_case_types)]
                    struct FetchSecretsSvc<T: SecretDiscoveryService>(pub Arc<T>);
                    impl<
                        T: SecretDiscoveryService,
                    > tonic::server::UnaryService<
                        super::super::super::super::api::v2::DiscoveryRequest,
                    > for FetchSecretsSvc<T> {
                        type Response = super::super::super::super::api::v2::DiscoveryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::api::v2::DiscoveryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).fetch_secrets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FetchSecretsSvc(inner);
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
    impl<T: SecretDiscoveryService> Clone for SecretDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SecretDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SecretDiscoveryService> tonic::server::NamedService
    for SecretDiscoveryServiceServer<T> {
        const NAME: &'static str = "envoy.service.discovery.v2.SecretDiscoveryService";
    }
}
