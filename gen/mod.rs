// @generated
pub mod envoy {
    pub mod admin {
        // @@protoc_insertion_point(attribute:envoy.admin.v2alpha)
        pub mod v2alpha {
            include!("envoy.admin.v2alpha.rs");
            // @@protoc_insertion_point(envoy.admin.v2alpha)
        }
        // @@protoc_insertion_point(attribute:envoy.admin.v3)
        pub mod v3 {
            include!("envoy.admin.v3.rs");
            // @@protoc_insertion_point(envoy.admin.v3)
        }
    }
    // @@protoc_insertion_point(attribute:envoy.annotations)
    pub mod annotations {
        include!("envoy.annotations.rs");
        // @@protoc_insertion_point(envoy.annotations)
    }
    pub mod api {
        // @@protoc_insertion_point(attribute:envoy.api.v2)
        pub mod v2 {
            include!("envoy.api.v2.rs");
            // @@protoc_insertion_point(envoy.api.v2)
            // @@protoc_insertion_point(attribute:envoy.api.v2.auth)
            pub mod auth {
                include!("envoy.api.v2.auth.rs");
                // @@protoc_insertion_point(envoy.api.v2.auth)
            }
            // @@protoc_insertion_point(attribute:envoy.api.v2.cluster)
            pub mod cluster {
                include!("envoy.api.v2.cluster.rs");
                // @@protoc_insertion_point(envoy.api.v2.cluster)
            }
            // @@protoc_insertion_point(attribute:envoy.api.v2.core)
            pub mod core {
                include!("envoy.api.v2.core.rs");
                // @@protoc_insertion_point(envoy.api.v2.core)
            }
            // @@protoc_insertion_point(attribute:envoy.api.v2.endpoint)
            pub mod endpoint {
                include!("envoy.api.v2.endpoint.rs");
                // @@protoc_insertion_point(envoy.api.v2.endpoint)
            }
            // @@protoc_insertion_point(attribute:envoy.api.v2.listener)
            pub mod listener {
                include!("envoy.api.v2.listener.rs");
                // @@protoc_insertion_point(envoy.api.v2.listener)
            }
            // @@protoc_insertion_point(attribute:envoy.api.v2.ratelimit)
            pub mod ratelimit {
                include!("envoy.api.v2.ratelimit.rs");
                // @@protoc_insertion_point(envoy.api.v2.ratelimit)
            }
            // @@protoc_insertion_point(attribute:envoy.api.v2.route)
            pub mod route {
                include!("envoy.api.v2.route.rs");
                // @@protoc_insertion_point(envoy.api.v2.route)
            }
        }
    }
    pub mod config {
        pub mod accesslog {
            // @@protoc_insertion_point(attribute:envoy.config.accesslog.v2)
            pub mod v2 {
                include!("envoy.config.accesslog.v2.rs");
                // @@protoc_insertion_point(envoy.config.accesslog.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.config.accesslog.v3)
            pub mod v3 {
                include!("envoy.config.accesslog.v3.rs");
                // @@protoc_insertion_point(envoy.config.accesslog.v3)
            }
        }
        pub mod bootstrap {
            // @@protoc_insertion_point(attribute:envoy.config.bootstrap.v2)
            pub mod v2 {
                include!("envoy.config.bootstrap.v2.rs");
                // @@protoc_insertion_point(envoy.config.bootstrap.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.config.bootstrap.v3)
            pub mod v3 {
                include!("envoy.config.bootstrap.v3.rs");
                // @@protoc_insertion_point(envoy.config.bootstrap.v3)
            }
        }
        pub mod cluster {
            pub mod aggregate {
                // @@protoc_insertion_point(attribute:envoy.config.cluster.aggregate.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.cluster.aggregate.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.cluster.aggregate.v2alpha)
                }
            }
            pub mod dynamic_forward_proxy {
                // @@protoc_insertion_point(attribute:envoy.config.cluster.dynamic_forward_proxy.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.cluster.dynamic_forward_proxy.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.cluster.dynamic_forward_proxy.v2alpha)
                }
            }
            // @@protoc_insertion_point(attribute:envoy.config.cluster.redis)
            pub mod redis {
                include!("envoy.config.cluster.redis.rs");
                // @@protoc_insertion_point(envoy.config.cluster.redis)
            }
            // @@protoc_insertion_point(attribute:envoy.config.cluster.v3)
            pub mod v3 {
                include!("envoy.config.cluster.v3.rs");
                // @@protoc_insertion_point(envoy.config.cluster.v3)
            }
        }
        pub mod common {
            pub mod dynamic_forward_proxy {
                // @@protoc_insertion_point(attribute:envoy.config.common.dynamic_forward_proxy.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.common.dynamic_forward_proxy.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.common.dynamic_forward_proxy.v2alpha)
                }
            }
            pub mod key_value {
                // @@protoc_insertion_point(attribute:envoy.config.common.key_value.v3)
                pub mod v3 {
                    include!("envoy.config.common.key_value.v3.rs");
                    // @@protoc_insertion_point(envoy.config.common.key_value.v3)
                }
            }
            pub mod matcher {
                // @@protoc_insertion_point(attribute:envoy.config.common.matcher.v3)
                pub mod v3 {
                    include!("envoy.config.common.matcher.v3.rs");
                    // @@protoc_insertion_point(envoy.config.common.matcher.v3)
                }
            }
            pub mod mutation_rules {
                // @@protoc_insertion_point(attribute:envoy.config.common.mutation_rules.v3)
                pub mod v3 {
                    include!("envoy.config.common.mutation_rules.v3.rs");
                    // @@protoc_insertion_point(envoy.config.common.mutation_rules.v3)
                }
            }
            pub mod tap {
                // @@protoc_insertion_point(attribute:envoy.config.common.tap.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.common.tap.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.common.tap.v2alpha)
                }
            }
        }
        pub mod core {
            // @@protoc_insertion_point(attribute:envoy.config.core.v3)
            pub mod v3 {
                include!("envoy.config.core.v3.rs");
                // @@protoc_insertion_point(envoy.config.core.v3)
            }
        }
        pub mod endpoint {
            // @@protoc_insertion_point(attribute:envoy.config.endpoint.v3)
            pub mod v3 {
                include!("envoy.config.endpoint.v3.rs");
                // @@protoc_insertion_point(envoy.config.endpoint.v3)
            }
        }
        pub mod filter {
            pub mod accesslog {
                // @@protoc_insertion_point(attribute:envoy.config.filter.accesslog.v2)
                pub mod v2 {
                    include!("envoy.config.filter.accesslog.v2.rs");
                    // @@protoc_insertion_point(envoy.config.filter.accesslog.v2)
                }
            }
            pub mod dubbo {
                pub mod router {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.dubbo.router.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.dubbo.router.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.dubbo.router.v2alpha1)
                    }
                }
            }
            pub mod fault {
                // @@protoc_insertion_point(attribute:envoy.config.filter.fault.v2)
                pub mod v2 {
                    include!("envoy.config.filter.fault.v2.rs");
                    // @@protoc_insertion_point(envoy.config.filter.fault.v2)
                }
            }
            pub mod http {
                pub mod adaptive_concurrency {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.adaptive_concurrency.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.adaptive_concurrency.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.adaptive_concurrency.v2alpha)
                    }
                }
                pub mod aws_lambda {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.aws_lambda.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.aws_lambda.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.aws_lambda.v2alpha)
                    }
                }
                pub mod aws_request_signing {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.aws_request_signing.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.aws_request_signing.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.aws_request_signing.v2alpha)
                    }
                }
                pub mod buffer {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.buffer.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.buffer.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.buffer.v2)
                    }
                }
                pub mod cache {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.cache.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.cache.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.cache.v2alpha)
                    }
                }
                pub mod compressor {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.compressor.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.compressor.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.compressor.v2)
                    }
                }
                pub mod cors {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.cors.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.cors.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.cors.v2)
                    }
                }
                pub mod csrf {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.csrf.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.csrf.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.csrf.v2)
                    }
                }
                pub mod dynamic_forward_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.dynamic_forward_proxy.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.dynamic_forward_proxy.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.dynamic_forward_proxy.v2alpha)
                    }
                }
                pub mod dynamo {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.dynamo.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.dynamo.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.dynamo.v2)
                    }
                }
                pub mod ext_authz {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.ext_authz.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.ext_authz.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.ext_authz.v2)
                    }
                }
                pub mod fault {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.fault.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.fault.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.fault.v2)
                    }
                }
                pub mod grpc_http1_bridge {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.grpc_http1_bridge.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.grpc_http1_bridge.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.grpc_http1_bridge.v2)
                    }
                }
                pub mod grpc_http1_reverse_bridge {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.grpc_http1_reverse_bridge.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.http.grpc_http1_reverse_bridge.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.grpc_http1_reverse_bridge.v2alpha1)
                    }
                }
                pub mod grpc_stats {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.grpc_stats.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.grpc_stats.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.grpc_stats.v2alpha)
                    }
                }
                pub mod grpc_web {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.grpc_web.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.grpc_web.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.grpc_web.v2)
                    }
                }
                pub mod gzip {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.gzip.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.gzip.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.gzip.v2)
                    }
                }
                pub mod header_to_metadata {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.header_to_metadata.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.header_to_metadata.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.header_to_metadata.v2)
                    }
                }
                pub mod health_check {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.health_check.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.health_check.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.health_check.v2)
                    }
                }
                pub mod ip_tagging {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.ip_tagging.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.ip_tagging.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.ip_tagging.v2)
                    }
                }
                pub mod jwt_authn {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.jwt_authn.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.jwt_authn.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.jwt_authn.v2alpha)
                    }
                }
                pub mod lua {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.lua.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.lua.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.lua.v2)
                    }
                }
                pub mod on_demand {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.on_demand.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.on_demand.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.on_demand.v2)
                    }
                }
                pub mod original_src {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.original_src.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.http.original_src.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.original_src.v2alpha1)
                    }
                }
                pub mod rate_limit {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.rate_limit.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.rate_limit.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.rate_limit.v2)
                    }
                }
                pub mod rbac {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.rbac.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.rbac.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.rbac.v2)
                    }
                }
                pub mod router {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.router.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.router.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.router.v2)
                    }
                }
                pub mod squash {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.squash.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.squash.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.squash.v2)
                    }
                }
                pub mod tap {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.tap.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.http.tap.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.tap.v2alpha)
                    }
                }
                pub mod transcoder {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.http.transcoder.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.http.transcoder.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.http.transcoder.v2)
                    }
                }
            }
            pub mod listener {
                pub mod http_inspector {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.listener.http_inspector.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.listener.http_inspector.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.listener.http_inspector.v2)
                    }
                }
                pub mod original_dst {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.listener.original_dst.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.listener.original_dst.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.listener.original_dst.v2)
                    }
                }
                pub mod original_src {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.listener.original_src.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.listener.original_src.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.listener.original_src.v2alpha1)
                    }
                }
                pub mod proxy_protocol {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.listener.proxy_protocol.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.listener.proxy_protocol.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.listener.proxy_protocol.v2)
                    }
                }
                pub mod tls_inspector {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.listener.tls_inspector.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.listener.tls_inspector.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.listener.tls_inspector.v2)
                    }
                }
            }
            pub mod network {
                pub mod client_ssl_auth {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.client_ssl_auth.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.client_ssl_auth.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.client_ssl_auth.v2)
                    }
                }
                pub mod direct_response {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.direct_response.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.direct_response.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.direct_response.v2)
                    }
                }
                pub mod dubbo_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.dubbo_proxy.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.network.dubbo_proxy.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.dubbo_proxy.v2alpha1)
                    }
                }
                pub mod echo {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.echo.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.echo.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.echo.v2)
                    }
                }
                pub mod ext_authz {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.ext_authz.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.ext_authz.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.ext_authz.v2)
                    }
                }
                pub mod http_connection_manager {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.http_connection_manager.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.http_connection_manager.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.http_connection_manager.v2)
                    }
                }
                pub mod kafka_broker {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.kafka_broker.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.network.kafka_broker.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.kafka_broker.v2alpha1)
                    }
                }
                pub mod local_rate_limit {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.local_rate_limit.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.network.local_rate_limit.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.local_rate_limit.v2alpha)
                    }
                }
                pub mod mongo_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.mongo_proxy.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.mongo_proxy.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.mongo_proxy.v2)
                    }
                }
                pub mod mysql_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.mysql_proxy.v1alpha1)
                    pub mod v1alpha1 {
                        include!("envoy.config.filter.network.mysql_proxy.v1alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.mysql_proxy.v1alpha1)
                    }
                }
                pub mod rate_limit {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.rate_limit.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.rate_limit.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.rate_limit.v2)
                    }
                }
                pub mod rbac {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.rbac.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.rbac.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.rbac.v2)
                    }
                }
                pub mod redis_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.redis_proxy.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.redis_proxy.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.redis_proxy.v2)
                    }
                }
                pub mod sni_cluster {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.sni_cluster.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.sni_cluster.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.sni_cluster.v2)
                    }
                }
                pub mod tcp_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.tcp_proxy.v2)
                    pub mod v2 {
                        include!("envoy.config.filter.network.tcp_proxy.v2.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.tcp_proxy.v2)
                    }
                }
                pub mod thrift_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.thrift_proxy.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.network.thrift_proxy.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.thrift_proxy.v2alpha1)
                    }
                }
                pub mod zookeeper_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.network.zookeeper_proxy.v1alpha1)
                    pub mod v1alpha1 {
                        include!("envoy.config.filter.network.zookeeper_proxy.v1alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.network.zookeeper_proxy.v1alpha1)
                    }
                }
            }
            pub mod thrift {
                pub mod rate_limit {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.thrift.rate_limit.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.thrift.rate_limit.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.thrift.rate_limit.v2alpha1)
                    }
                }
                pub mod router {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.thrift.router.v2alpha1)
                    pub mod v2alpha1 {
                        include!("envoy.config.filter.thrift.router.v2alpha1.rs");
                        // @@protoc_insertion_point(envoy.config.filter.thrift.router.v2alpha1)
                    }
                }
            }
            pub mod udp {
                pub mod udp_proxy {
                    // @@protoc_insertion_point(attribute:envoy.config.filter.udp.udp_proxy.v2alpha)
                    pub mod v2alpha {
                        include!("envoy.config.filter.udp.udp_proxy.v2alpha.rs");
                        // @@protoc_insertion_point(envoy.config.filter.udp.udp_proxy.v2alpha)
                    }
                }
            }
        }
        pub mod grpc_credential {
            // @@protoc_insertion_point(attribute:envoy.config.grpc_credential.v2alpha)
            pub mod v2alpha {
                include!("envoy.config.grpc_credential.v2alpha.rs");
                // @@protoc_insertion_point(envoy.config.grpc_credential.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.config.grpc_credential.v3)
            pub mod v3 {
                include!("envoy.config.grpc_credential.v3.rs");
                // @@protoc_insertion_point(envoy.config.grpc_credential.v3)
            }
        }
        pub mod health_checker {
            pub mod redis {
                // @@protoc_insertion_point(attribute:envoy.config.health_checker.redis.v2)
                pub mod v2 {
                    include!("envoy.config.health_checker.redis.v2.rs");
                    // @@protoc_insertion_point(envoy.config.health_checker.redis.v2)
                }
            }
        }
        pub mod listener {
            // @@protoc_insertion_point(attribute:envoy.config.listener.v2)
            pub mod v2 {
                include!("envoy.config.listener.v2.rs");
                // @@protoc_insertion_point(envoy.config.listener.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.config.listener.v3)
            pub mod v3 {
                include!("envoy.config.listener.v3.rs");
                // @@protoc_insertion_point(envoy.config.listener.v3)
            }
        }
        pub mod metrics {
            // @@protoc_insertion_point(attribute:envoy.config.metrics.v2)
            pub mod v2 {
                include!("envoy.config.metrics.v2.rs");
                // @@protoc_insertion_point(envoy.config.metrics.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.config.metrics.v3)
            pub mod v3 {
                include!("envoy.config.metrics.v3.rs");
                // @@protoc_insertion_point(envoy.config.metrics.v3)
            }
        }
        pub mod overload {
            // @@protoc_insertion_point(attribute:envoy.config.overload.v2alpha)
            pub mod v2alpha {
                include!("envoy.config.overload.v2alpha.rs");
                // @@protoc_insertion_point(envoy.config.overload.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.config.overload.v3)
            pub mod v3 {
                include!("envoy.config.overload.v3.rs");
                // @@protoc_insertion_point(envoy.config.overload.v3)
            }
        }
        pub mod ratelimit {
            // @@protoc_insertion_point(attribute:envoy.config.ratelimit.v2)
            pub mod v2 {
                include!("envoy.config.ratelimit.v2.rs");
                // @@protoc_insertion_point(envoy.config.ratelimit.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.config.ratelimit.v3)
            pub mod v3 {
                include!("envoy.config.ratelimit.v3.rs");
                // @@protoc_insertion_point(envoy.config.ratelimit.v3)
            }
        }
        pub mod rbac {
            // @@protoc_insertion_point(attribute:envoy.config.rbac.v2)
            pub mod v2 {
                include!("envoy.config.rbac.v2.rs");
                // @@protoc_insertion_point(envoy.config.rbac.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.config.rbac.v3)
            pub mod v3 {
                include!("envoy.config.rbac.v3.rs");
                // @@protoc_insertion_point(envoy.config.rbac.v3)
            }
        }
        pub mod resource_monitor {
            pub mod fixed_heap {
                // @@protoc_insertion_point(attribute:envoy.config.resource_monitor.fixed_heap.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.resource_monitor.fixed_heap.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.resource_monitor.fixed_heap.v2alpha)
                }
            }
            pub mod injected_resource {
                // @@protoc_insertion_point(attribute:envoy.config.resource_monitor.injected_resource.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.resource_monitor.injected_resource.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.resource_monitor.injected_resource.v2alpha)
                }
            }
        }
        pub mod retry {
            pub mod omit_canary_hosts {
                // @@protoc_insertion_point(attribute:envoy.config.retry.omit_canary_hosts.v2)
                pub mod v2 {
                    include!("envoy.config.retry.omit_canary_hosts.v2.rs");
                    // @@protoc_insertion_point(envoy.config.retry.omit_canary_hosts.v2)
                }
            }
            pub mod omit_host_metadata {
                // @@protoc_insertion_point(attribute:envoy.config.retry.omit_host_metadata.v2)
                pub mod v2 {
                    include!("envoy.config.retry.omit_host_metadata.v2.rs");
                    // @@protoc_insertion_point(envoy.config.retry.omit_host_metadata.v2)
                }
            }
            pub mod previous_hosts {
                // @@protoc_insertion_point(attribute:envoy.config.retry.previous_hosts.v2)
                pub mod v2 {
                    include!("envoy.config.retry.previous_hosts.v2.rs");
                    // @@protoc_insertion_point(envoy.config.retry.previous_hosts.v2)
                }
            }
            // @@protoc_insertion_point(attribute:envoy.config.retry.previous_priorities)
            pub mod previous_priorities {
                include!("envoy.config.retry.previous_priorities.rs");
                // @@protoc_insertion_point(envoy.config.retry.previous_priorities)
            }
        }
        pub mod route {
            // @@protoc_insertion_point(attribute:envoy.config.route.v3)
            pub mod v3 {
                include!("envoy.config.route.v3.rs");
                // @@protoc_insertion_point(envoy.config.route.v3)
            }
        }
        pub mod tap {
            // @@protoc_insertion_point(attribute:envoy.config.tap.v3)
            pub mod v3 {
                include!("envoy.config.tap.v3.rs");
                // @@protoc_insertion_point(envoy.config.tap.v3)
            }
        }
        pub mod trace {
            // @@protoc_insertion_point(attribute:envoy.config.trace.v2)
            pub mod v2 {
                include!("envoy.config.trace.v2.rs");
                // @@protoc_insertion_point(envoy.config.trace.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.config.trace.v2alpha)
            pub mod v2alpha {
                include!("envoy.config.trace.v2alpha.rs");
                // @@protoc_insertion_point(envoy.config.trace.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.config.trace.v3)
            pub mod v3 {
                include!("envoy.config.trace.v3.rs");
                // @@protoc_insertion_point(envoy.config.trace.v3)
            }
        }
        pub mod transport_socket {
            pub mod alts {
                // @@protoc_insertion_point(attribute:envoy.config.transport_socket.alts.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.transport_socket.alts.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.transport_socket.alts.v2alpha)
                }
            }
            pub mod raw_buffer {
                // @@protoc_insertion_point(attribute:envoy.config.transport_socket.raw_buffer.v2)
                pub mod v2 {
                    include!("envoy.config.transport_socket.raw_buffer.v2.rs");
                    // @@protoc_insertion_point(envoy.config.transport_socket.raw_buffer.v2)
                }
            }
            pub mod tap {
                // @@protoc_insertion_point(attribute:envoy.config.transport_socket.tap.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.transport_socket.tap.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.transport_socket.tap.v2alpha)
                }
            }
        }
    }
    pub mod data {
        pub mod accesslog {
            // @@protoc_insertion_point(attribute:envoy.data.accesslog.v2)
            pub mod v2 {
                include!("envoy.data.accesslog.v2.rs");
                // @@protoc_insertion_point(envoy.data.accesslog.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.data.accesslog.v3)
            pub mod v3 {
                include!("envoy.data.accesslog.v3.rs");
                // @@protoc_insertion_point(envoy.data.accesslog.v3)
            }
        }
        pub mod cluster {
            // @@protoc_insertion_point(attribute:envoy.data.cluster.v2alpha)
            pub mod v2alpha {
                include!("envoy.data.cluster.v2alpha.rs");
                // @@protoc_insertion_point(envoy.data.cluster.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.data.cluster.v3)
            pub mod v3 {
                include!("envoy.data.cluster.v3.rs");
                // @@protoc_insertion_point(envoy.data.cluster.v3)
            }
        }
        pub mod core {
            // @@protoc_insertion_point(attribute:envoy.data.core.v2alpha)
            pub mod v2alpha {
                include!("envoy.data.core.v2alpha.rs");
                // @@protoc_insertion_point(envoy.data.core.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.data.core.v3)
            pub mod v3 {
                include!("envoy.data.core.v3.rs");
                // @@protoc_insertion_point(envoy.data.core.v3)
            }
        }
        pub mod dns {
            // @@protoc_insertion_point(attribute:envoy.data.dns.v2alpha)
            pub mod v2alpha {
                include!("envoy.data.dns.v2alpha.rs");
                // @@protoc_insertion_point(envoy.data.dns.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.data.dns.v3)
            pub mod v3 {
                include!("envoy.data.dns.v3.rs");
                // @@protoc_insertion_point(envoy.data.dns.v3)
            }
        }
        pub mod tap {
            // @@protoc_insertion_point(attribute:envoy.data.tap.v2alpha)
            pub mod v2alpha {
                include!("envoy.data.tap.v2alpha.rs");
                // @@protoc_insertion_point(envoy.data.tap.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.data.tap.v3)
            pub mod v3 {
                include!("envoy.data.tap.v3.rs");
                // @@protoc_insertion_point(envoy.data.tap.v3)
            }
        }
    }
    pub mod extensions {
        pub mod access_loggers {
            pub mod file {
                // @@protoc_insertion_point(attribute:envoy.extensions.access_loggers.file.v3)
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.file.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.access_loggers.file.v3)
                }
            }
            pub mod filters {
                pub mod cel {
                    // @@protoc_insertion_point(attribute:envoy.extensions.access_loggers.filters.cel.v3)
                    pub mod v3 {
                        include!("envoy.extensions.access_loggers.filters.cel.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.access_loggers.filters.cel.v3)
                    }
                }
            }
            pub mod grpc {
                // @@protoc_insertion_point(attribute:envoy.extensions.access_loggers.grpc.v3)
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.grpc.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.access_loggers.grpc.v3)
                }
            }
            pub mod open_telemetry {
                // @@protoc_insertion_point(attribute:envoy.extensions.access_loggers.open_telemetry.v3)
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.open_telemetry.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.access_loggers.open_telemetry.v3)
                }
            }
            pub mod stream {
                // @@protoc_insertion_point(attribute:envoy.extensions.access_loggers.stream.v3)
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.stream.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.access_loggers.stream.v3)
                }
            }
            pub mod wasm {
                // @@protoc_insertion_point(attribute:envoy.extensions.access_loggers.wasm.v3)
                pub mod v3 {
                    include!("envoy.extensions.access_loggers.wasm.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.access_loggers.wasm.v3)
                }
            }
        }
        pub mod bootstrap {
            pub mod internal_listener {
                // @@protoc_insertion_point(attribute:envoy.extensions.bootstrap.internal_listener.v3)
                pub mod v3 {
                    include!("envoy.extensions.bootstrap.internal_listener.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.bootstrap.internal_listener.v3)
                }
            }
        }
        pub mod clusters {
            pub mod aggregate {
                // @@protoc_insertion_point(attribute:envoy.extensions.clusters.aggregate.v3)
                pub mod v3 {
                    include!("envoy.extensions.clusters.aggregate.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.clusters.aggregate.v3)
                }
            }
            pub mod dynamic_forward_proxy {
                // @@protoc_insertion_point(attribute:envoy.extensions.clusters.dynamic_forward_proxy.v3)
                pub mod v3 {
                    include!("envoy.extensions.clusters.dynamic_forward_proxy.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.clusters.dynamic_forward_proxy.v3)
                }
            }
            pub mod redis {
                // @@protoc_insertion_point(attribute:envoy.extensions.clusters.redis.v3)
                pub mod v3 {
                    include!("envoy.extensions.clusters.redis.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.clusters.redis.v3)
                }
            }
        }
        pub mod common {
            pub mod async_files {
                // @@protoc_insertion_point(attribute:envoy.extensions.common.async_files.v3)
                pub mod v3 {
                    include!("envoy.extensions.common.async_files.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.common.async_files.v3)
                }
            }
            pub mod dynamic_forward_proxy {
                // @@protoc_insertion_point(attribute:envoy.extensions.common.dynamic_forward_proxy.v3)
                pub mod v3 {
                    include!("envoy.extensions.common.dynamic_forward_proxy.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.common.dynamic_forward_proxy.v3)
                }
            }
            pub mod matching {
                // @@protoc_insertion_point(attribute:envoy.extensions.common.matching.v3)
                pub mod v3 {
                    include!("envoy.extensions.common.matching.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.common.matching.v3)
                }
            }
            pub mod ratelimit {
                // @@protoc_insertion_point(attribute:envoy.extensions.common.ratelimit.v3)
                pub mod v3 {
                    include!("envoy.extensions.common.ratelimit.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.common.ratelimit.v3)
                }
            }
            pub mod tap {
                // @@protoc_insertion_point(attribute:envoy.extensions.common.tap.v3)
                pub mod v3 {
                    include!("envoy.extensions.common.tap.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.common.tap.v3)
                }
            }
        }
        pub mod compression {
            pub mod brotli {
                pub mod compressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.compression.brotli.compressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.compression.brotli.compressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.compression.brotli.compressor.v3)
                    }
                }
                pub mod decompressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.compression.brotli.decompressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.compression.brotli.decompressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.compression.brotli.decompressor.v3)
                    }
                }
            }
            pub mod gzip {
                pub mod compressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.compression.gzip.compressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.compression.gzip.compressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.compression.gzip.compressor.v3)
                    }
                }
                pub mod decompressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.compression.gzip.decompressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.compression.gzip.decompressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.compression.gzip.decompressor.v3)
                    }
                }
            }
            pub mod zstd {
                pub mod compressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.compression.zstd.compressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.compression.zstd.compressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.compression.zstd.compressor.v3)
                    }
                }
                pub mod decompressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.compression.zstd.decompressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.compression.zstd.decompressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.compression.zstd.decompressor.v3)
                    }
                }
            }
        }
        pub mod config {
            // @@protoc_insertion_point(attribute:envoy.extensions.config.v3alpha)
            pub mod v3alpha {
                include!("envoy.extensions.config.v3alpha.rs");
                // @@protoc_insertion_point(envoy.extensions.config.v3alpha)
            }
            pub mod validators {
                pub mod minimum_clusters {
                    // @@protoc_insertion_point(attribute:envoy.extensions.config.validators.minimum_clusters.v3)
                    pub mod v3 {
                        include!("envoy.extensions.config.validators.minimum_clusters.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.config.validators.minimum_clusters.v3)
                    }
                }
            }
        }
        pub mod early_data {
            // @@protoc_insertion_point(attribute:envoy.extensions.early_data.v3)
            pub mod v3 {
                include!("envoy.extensions.early_data.v3.rs");
                // @@protoc_insertion_point(envoy.extensions.early_data.v3)
            }
        }
        pub mod filters {
            pub mod common {
                pub mod dependency {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.common.dependency.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.common.dependency.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.common.dependency.v3)
                    }
                }
                pub mod fault {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.common.fault.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.common.fault.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.common.fault.v3)
                    }
                }
                pub mod matcher {
                    pub mod action {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.common.matcher.action.v3)
                        pub mod v3 {
                            include!("envoy.extensions.filters.common.matcher.action.v3.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.common.matcher.action.v3)
                        }
                    }
                }
            }
            pub mod http {
                pub mod adaptive_concurrency {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.adaptive_concurrency.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.adaptive_concurrency.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.adaptive_concurrency.v3)
                    }
                }
                pub mod admission_control {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.admission_control.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.admission_control.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.admission_control.v3)
                    }
                }
                pub mod alternate_protocols_cache {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.alternate_protocols_cache.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.alternate_protocols_cache.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.alternate_protocols_cache.v3)
                    }
                }
                pub mod aws_lambda {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.aws_lambda.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.aws_lambda.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.aws_lambda.v3)
                    }
                }
                pub mod aws_request_signing {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.aws_request_signing.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.aws_request_signing.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.aws_request_signing.v3)
                    }
                }
                pub mod bandwidth_limit {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.bandwidth_limit.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.bandwidth_limit.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.bandwidth_limit.v3)
                    }
                }
                pub mod buffer {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.buffer.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.buffer.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.buffer.v3)
                    }
                }
                pub mod cache {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.cache.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.cache.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.cache.v3)
                    }
                }
                pub mod cdn_loop {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.cdn_loop.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.cdn_loop.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.cdn_loop.v3)
                    }
                }
                pub mod composite {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.composite.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.composite.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.composite.v3)
                    }
                }
                pub mod compressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.compressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.compressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.compressor.v3)
                    }
                }
                pub mod connect_grpc_bridge {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.connect_grpc_bridge.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.connect_grpc_bridge.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.connect_grpc_bridge.v3)
                    }
                }
                pub mod cors {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.cors.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.cors.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.cors.v3)
                    }
                }
                pub mod csrf {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.csrf.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.csrf.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.csrf.v3)
                    }
                }
                pub mod custom_response {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.custom_response.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.custom_response.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.custom_response.v3)
                    }
                }
                pub mod decompressor {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.decompressor.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.decompressor.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.decompressor.v3)
                    }
                }
                pub mod dynamic_forward_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.dynamic_forward_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.dynamic_forward_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.dynamic_forward_proxy.v3)
                    }
                }
                pub mod dynamo {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.dynamo.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.dynamo.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.dynamo.v3)
                    }
                }
                pub mod ext_authz {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.ext_authz.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ext_authz.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.ext_authz.v3)
                    }
                }
                pub mod ext_proc {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.ext_proc.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ext_proc.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.ext_proc.v3)
                    }
                }
                pub mod fault {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.fault.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.fault.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.fault.v3)
                    }
                }
                pub mod file_system_buffer {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.file_system_buffer.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.file_system_buffer.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.file_system_buffer.v3)
                    }
                }
                pub mod gcp_authn {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.gcp_authn.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.gcp_authn.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.gcp_authn.v3)
                    }
                }
                pub mod golang {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.golang.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.filters.http.golang.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.golang.v3alpha)
                    }
                }
                pub mod grpc_http1_bridge {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.grpc_http1_bridge.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_http1_bridge.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.grpc_http1_bridge.v3)
                    }
                }
                pub mod grpc_http1_reverse_bridge {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3)
                    }
                }
                pub mod grpc_json_transcoder {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.grpc_json_transcoder.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_json_transcoder.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.grpc_json_transcoder.v3)
                    }
                }
                pub mod grpc_stats {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.grpc_stats.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_stats.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.grpc_stats.v3)
                    }
                }
                pub mod grpc_web {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.grpc_web.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.grpc_web.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.grpc_web.v3)
                    }
                }
                pub mod gzip {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.gzip.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.gzip.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.gzip.v3)
                    }
                }
                pub mod header_to_metadata {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.header_to_metadata.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.header_to_metadata.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.header_to_metadata.v3)
                    }
                }
                pub mod health_check {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.health_check.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.health_check.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.health_check.v3)
                    }
                }
                pub mod ip_tagging {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.ip_tagging.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ip_tagging.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.ip_tagging.v3)
                    }
                }
                pub mod jwt_authn {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.jwt_authn.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.jwt_authn.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.jwt_authn.v3)
                    }
                }
                pub mod kill_request {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.kill_request.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.kill_request.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.kill_request.v3)
                    }
                }
                pub mod language {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.language.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.filters.http.language.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.language.v3alpha)
                    }
                }
                pub mod local_ratelimit {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.local_ratelimit.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.local_ratelimit.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.local_ratelimit.v3)
                    }
                }
                pub mod lua {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.lua.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.lua.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.lua.v3)
                    }
                }
                pub mod oauth2 {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.oauth2.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.oauth2.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.oauth2.v3)
                    }
                }
                pub mod on_demand {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.on_demand.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.on_demand.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.on_demand.v3)
                    }
                }
                pub mod original_src {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.original_src.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.original_src.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.original_src.v3)
                    }
                }
                pub mod rate_limit_quota {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.rate_limit_quota.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.rate_limit_quota.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.rate_limit_quota.v3)
                    }
                }
                pub mod ratelimit {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.ratelimit.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.ratelimit.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.ratelimit.v3)
                    }
                }
                pub mod rbac {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.rbac.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.rbac.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.rbac.v3)
                    }
                }
                pub mod router {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.router.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.router.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.router.v3)
                    }
                }
                pub mod set_metadata {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.set_metadata.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.set_metadata.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.set_metadata.v3)
                    }
                }
                pub mod squash {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.squash.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.squash.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.squash.v3)
                    }
                }
                pub mod stateful_session {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.stateful_session.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.stateful_session.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.stateful_session.v3)
                    }
                }
                pub mod sxg {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.sxg.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.filters.http.sxg.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.sxg.v3alpha)
                    }
                }
                pub mod tap {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.tap.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.tap.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.tap.v3)
                    }
                }
                pub mod upstream_codec {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.upstream_codec.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.upstream_codec.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.upstream_codec.v3)
                    }
                }
                pub mod wasm {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.http.wasm.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.http.wasm.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.http.wasm.v3)
                    }
                }
            }
            pub mod listener {
                pub mod http_inspector {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.listener.http_inspector.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.http_inspector.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.listener.http_inspector.v3)
                    }
                }
                pub mod original_dst {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.listener.original_dst.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.original_dst.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.listener.original_dst.v3)
                    }
                }
                pub mod original_src {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.listener.original_src.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.original_src.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.listener.original_src.v3)
                    }
                }
                pub mod proxy_protocol {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.listener.proxy_protocol.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.proxy_protocol.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.listener.proxy_protocol.v3)
                    }
                }
                pub mod tls_inspector {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.listener.tls_inspector.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.listener.tls_inspector.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.listener.tls_inspector.v3)
                    }
                }
            }
            pub mod network {
                pub mod client_ssl_auth {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.client_ssl_auth.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.client_ssl_auth.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.client_ssl_auth.v3)
                    }
                }
                pub mod connection_limit {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.connection_limit.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.connection_limit.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.connection_limit.v3)
                    }
                }
                pub mod direct_response {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.direct_response.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.direct_response.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.direct_response.v3)
                    }
                }
                pub mod dubbo_proxy {
                    pub mod router {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.dubbo_proxy.router.v3)
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.dubbo_proxy.router.v3.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.network.dubbo_proxy.router.v3)
                        }
                    }
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.dubbo_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.dubbo_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.dubbo_proxy.v3)
                    }
                }
                pub mod echo {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.echo.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.echo.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.echo.v3)
                    }
                }
                pub mod ext_authz {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.ext_authz.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.ext_authz.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.ext_authz.v3)
                    }
                }
                pub mod generic_proxy {
                    pub mod action {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.generic_proxy.action.v3)
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.generic_proxy.action.v3.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.network.generic_proxy.action.v3)
                        }
                    }
                    pub mod codecs {
                        pub mod dubbo {
                            // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.generic_proxy.codecs.dubbo.v3)
                            pub mod v3 {
                                include!("envoy.extensions.filters.network.generic_proxy.codecs.dubbo.v3.rs");
                                // @@protoc_insertion_point(envoy.extensions.filters.network.generic_proxy.codecs.dubbo.v3)
                            }
                        }
                    }
                    pub mod matcher {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.generic_proxy.matcher.v3)
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.generic_proxy.matcher.v3.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.network.generic_proxy.matcher.v3)
                        }
                    }
                    pub mod router {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.generic_proxy.router.v3)
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.generic_proxy.router.v3.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.network.generic_proxy.router.v3)
                        }
                    }
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.generic_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.generic_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.generic_proxy.v3)
                    }
                }
                pub mod http_connection_manager {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.http_connection_manager.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.http_connection_manager.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.http_connection_manager.v3)
                    }
                }
                pub mod kafka_broker {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.kafka_broker.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.kafka_broker.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.kafka_broker.v3)
                    }
                }
                pub mod kafka_mesh {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.kafka_mesh.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.filters.network.kafka_mesh.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.kafka_mesh.v3alpha)
                    }
                }
                pub mod local_ratelimit {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.local_ratelimit.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.local_ratelimit.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.local_ratelimit.v3)
                    }
                }
                pub mod mongo_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.mongo_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.mongo_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.mongo_proxy.v3)
                    }
                }
                pub mod mysql_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.mysql_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.mysql_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.mysql_proxy.v3)
                    }
                }
                pub mod postgres_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.postgres_proxy.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.filters.network.postgres_proxy.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.postgres_proxy.v3alpha)
                    }
                }
                pub mod ratelimit {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.ratelimit.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.ratelimit.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.ratelimit.v3)
                    }
                }
                pub mod rbac {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.rbac.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.rbac.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.rbac.v3)
                    }
                }
                pub mod redis_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.redis_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.redis_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.redis_proxy.v3)
                    }
                }
                pub mod rocketmq_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.rocketmq_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.rocketmq_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.rocketmq_proxy.v3)
                    }
                }
                pub mod sip_proxy {
                    pub mod router {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.sip_proxy.router.v3alpha)
                        pub mod v3alpha {
                            include!("envoy.extensions.filters.network.sip_proxy.router.v3alpha.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.network.sip_proxy.router.v3alpha)
                        }
                    }
                    pub mod tra {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.sip_proxy.tra.v3alpha)
                        pub mod v3alpha {
                            include!("envoy.extensions.filters.network.sip_proxy.tra.v3alpha.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.network.sip_proxy.tra.v3alpha)
                        }
                    }
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.sip_proxy.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.filters.network.sip_proxy.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.sip_proxy.v3alpha)
                    }
                }
                pub mod sni_cluster {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.sni_cluster.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.sni_cluster.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.sni_cluster.v3)
                    }
                }
                pub mod sni_dynamic_forward_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3)
                    }
                }
                pub mod tcp_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.tcp_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.tcp_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.tcp_proxy.v3)
                    }
                }
                pub mod thrift_proxy {
                    pub mod filters {
                        pub mod header_to_metadata {
                            // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3)
                            pub mod v3 {
                                include!("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.rs");
                                // @@protoc_insertion_point(envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3)
                            }
                        }
                        pub mod payload_to_metadata {
                            // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3)
                            pub mod v3 {
                                include!("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.rs");
                                // @@protoc_insertion_point(envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3)
                            }
                        }
                        pub mod ratelimit {
                            // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.thrift_proxy.filters.ratelimit.v3)
                            pub mod v3 {
                                include!("envoy.extensions.filters.network.thrift_proxy.filters.ratelimit.v3.rs");
                                // @@protoc_insertion_point(envoy.extensions.filters.network.thrift_proxy.filters.ratelimit.v3)
                            }
                        }
                    }
                    pub mod router {
                        // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.thrift_proxy.router.v3)
                        pub mod v3 {
                            include!("envoy.extensions.filters.network.thrift_proxy.router.v3.rs");
                            // @@protoc_insertion_point(envoy.extensions.filters.network.thrift_proxy.router.v3)
                        }
                    }
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.thrift_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.thrift_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.thrift_proxy.v3)
                    }
                }
                pub mod wasm {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.wasm.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.wasm.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.wasm.v3)
                    }
                }
                pub mod zookeeper_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.network.zookeeper_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.network.zookeeper_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.network.zookeeper_proxy.v3)
                    }
                }
            }
            pub mod udp {
                pub mod dns_filter {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.udp.dns_filter.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.udp.dns_filter.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.udp.dns_filter.v3)
                    }
                }
                pub mod udp_proxy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.filters.udp.udp_proxy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.filters.udp.udp_proxy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.filters.udp.udp_proxy.v3)
                    }
                }
            }
        }
        pub mod formatter {
            pub mod metadata {
                // @@protoc_insertion_point(attribute:envoy.extensions.formatter.metadata.v3)
                pub mod v3 {
                    include!("envoy.extensions.formatter.metadata.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.formatter.metadata.v3)
                }
            }
            pub mod req_without_query {
                // @@protoc_insertion_point(attribute:envoy.extensions.formatter.req_without_query.v3)
                pub mod v3 {
                    include!("envoy.extensions.formatter.req_without_query.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.formatter.req_without_query.v3)
                }
            }
        }
        pub mod health_checkers {
            pub mod redis {
                // @@protoc_insertion_point(attribute:envoy.extensions.health_checkers.redis.v3)
                pub mod v3 {
                    include!("envoy.extensions.health_checkers.redis.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.health_checkers.redis.v3)
                }
            }
            pub mod thrift {
                // @@protoc_insertion_point(attribute:envoy.extensions.health_checkers.thrift.v3)
                pub mod v3 {
                    include!("envoy.extensions.health_checkers.thrift.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.health_checkers.thrift.v3)
                }
            }
        }
        pub mod http {
            pub mod cache {
                pub mod file_system_http_cache {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.cache.file_system_http_cache.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.cache.file_system_http_cache.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.cache.file_system_http_cache.v3)
                    }
                }
                pub mod simple_http_cache {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.cache.simple_http_cache.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.cache.simple_http_cache.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.cache.simple_http_cache.v3)
                    }
                }
            }
            pub mod custom_response {
                pub mod local_response_policy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.custom_response.local_response_policy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.custom_response.local_response_policy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.custom_response.local_response_policy.v3)
                    }
                }
                pub mod redirect_policy {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.custom_response.redirect_policy.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.custom_response.redirect_policy.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.custom_response.redirect_policy.v3)
                    }
                }
            }
            pub mod early_header_mutation {
                pub mod header_mutation {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.early_header_mutation.header_mutation.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.early_header_mutation.header_mutation.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.early_header_mutation.header_mutation.v3)
                    }
                }
            }
            pub mod header_formatters {
                pub mod preserve_case {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.header_formatters.preserve_case.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.header_formatters.preserve_case.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.header_formatters.preserve_case.v3)
                    }
                }
            }
            pub mod header_validators {
                pub mod envoy_default {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.header_validators.envoy_default.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.header_validators.envoy_default.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.header_validators.envoy_default.v3)
                    }
                }
            }
            pub mod original_ip_detection {
                pub mod custom_header {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.original_ip_detection.custom_header.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.original_ip_detection.custom_header.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.original_ip_detection.custom_header.v3)
                    }
                }
                pub mod xff {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.original_ip_detection.xff.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.original_ip_detection.xff.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.original_ip_detection.xff.v3)
                    }
                }
            }
            pub mod stateful_session {
                pub mod cookie {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.stateful_session.cookie.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.stateful_session.cookie.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.stateful_session.cookie.v3)
                    }
                }
                pub mod header {
                    // @@protoc_insertion_point(attribute:envoy.extensions.http.stateful_session.header.v3)
                    pub mod v3 {
                        include!("envoy.extensions.http.stateful_session.header.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.http.stateful_session.header.v3)
                    }
                }
            }
        }
        pub mod internal_redirect {
            pub mod allow_listed_routes {
                // @@protoc_insertion_point(attribute:envoy.extensions.internal_redirect.allow_listed_routes.v3)
                pub mod v3 {
                    include!("envoy.extensions.internal_redirect.allow_listed_routes.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.internal_redirect.allow_listed_routes.v3)
                }
            }
            pub mod previous_routes {
                // @@protoc_insertion_point(attribute:envoy.extensions.internal_redirect.previous_routes.v3)
                pub mod v3 {
                    include!("envoy.extensions.internal_redirect.previous_routes.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.internal_redirect.previous_routes.v3)
                }
            }
            pub mod safe_cross_scheme {
                // @@protoc_insertion_point(attribute:envoy.extensions.internal_redirect.safe_cross_scheme.v3)
                pub mod v3 {
                    include!("envoy.extensions.internal_redirect.safe_cross_scheme.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.internal_redirect.safe_cross_scheme.v3)
                }
            }
        }
        pub mod key_value {
            pub mod file_based {
                // @@protoc_insertion_point(attribute:envoy.extensions.key_value.file_based.v3)
                pub mod v3 {
                    include!("envoy.extensions.key_value.file_based.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.key_value.file_based.v3)
                }
            }
        }
        pub mod load_balancing_policies {
            pub mod client_side_weighted_round_robin {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.client_side_weighted_round_robin.v3)
                }
            }
            pub mod common {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.common.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.common.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.common.v3)
                }
            }
            pub mod least_request {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.least_request.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.least_request.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.least_request.v3)
                }
            }
            pub mod maglev {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.maglev.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.maglev.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.maglev.v3)
                }
            }
            pub mod random {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.random.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.random.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.random.v3)
                }
            }
            pub mod ring_hash {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.ring_hash.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.ring_hash.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.ring_hash.v3)
                }
            }
            pub mod round_robin {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.round_robin.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.round_robin.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.round_robin.v3)
                }
            }
            pub mod wrr_locality {
                // @@protoc_insertion_point(attribute:envoy.extensions.load_balancing_policies.wrr_locality.v3)
                pub mod v3 {
                    include!("envoy.extensions.load_balancing_policies.wrr_locality.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.load_balancing_policies.wrr_locality.v3)
                }
            }
        }
        pub mod matching {
            pub mod common_inputs {
                pub mod environment_variable {
                    // @@protoc_insertion_point(attribute:envoy.extensions.matching.common_inputs.environment_variable.v3)
                    pub mod v3 {
                        include!("envoy.extensions.matching.common_inputs.environment_variable.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.matching.common_inputs.environment_variable.v3)
                    }
                }
                pub mod network {
                    // @@protoc_insertion_point(attribute:envoy.extensions.matching.common_inputs.network.v3)
                    pub mod v3 {
                        include!("envoy.extensions.matching.common_inputs.network.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.matching.common_inputs.network.v3)
                    }
                }
                pub mod ssl {
                    // @@protoc_insertion_point(attribute:envoy.extensions.matching.common_inputs.ssl.v3)
                    pub mod v3 {
                        include!("envoy.extensions.matching.common_inputs.ssl.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.matching.common_inputs.ssl.v3)
                    }
                }
            }
            pub mod input_matchers {
                pub mod consistent_hashing {
                    // @@protoc_insertion_point(attribute:envoy.extensions.matching.input_matchers.consistent_hashing.v3)
                    pub mod v3 {
                        include!("envoy.extensions.matching.input_matchers.consistent_hashing.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.matching.input_matchers.consistent_hashing.v3)
                    }
                }
                pub mod hyperscan {
                    // @@protoc_insertion_point(attribute:envoy.extensions.matching.input_matchers.hyperscan.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.matching.input_matchers.hyperscan.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.matching.input_matchers.hyperscan.v3alpha)
                    }
                }
                pub mod ip {
                    // @@protoc_insertion_point(attribute:envoy.extensions.matching.input_matchers.ip.v3)
                    pub mod v3 {
                        include!("envoy.extensions.matching.input_matchers.ip.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.matching.input_matchers.ip.v3)
                    }
                }
            }
        }
        pub mod network {
            pub mod connection_balance {
                pub mod dlb {
                    // @@protoc_insertion_point(attribute:envoy.extensions.network.connection_balance.dlb.v3alpha)
                    pub mod v3alpha {
                        include!("envoy.extensions.network.connection_balance.dlb.v3alpha.rs");
                        // @@protoc_insertion_point(envoy.extensions.network.connection_balance.dlb.v3alpha)
                    }
                }
            }
            pub mod dns_resolver {
                pub mod apple {
                    // @@protoc_insertion_point(attribute:envoy.extensions.network.dns_resolver.apple.v3)
                    pub mod v3 {
                        include!("envoy.extensions.network.dns_resolver.apple.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.network.dns_resolver.apple.v3)
                    }
                }
                pub mod cares {
                    // @@protoc_insertion_point(attribute:envoy.extensions.network.dns_resolver.cares.v3)
                    pub mod v3 {
                        include!("envoy.extensions.network.dns_resolver.cares.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.network.dns_resolver.cares.v3)
                    }
                }
                pub mod getaddrinfo {
                    // @@protoc_insertion_point(attribute:envoy.extensions.network.dns_resolver.getaddrinfo.v3)
                    pub mod v3 {
                        include!("envoy.extensions.network.dns_resolver.getaddrinfo.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.network.dns_resolver.getaddrinfo.v3)
                    }
                }
            }
            pub mod socket_interface {
                // @@protoc_insertion_point(attribute:envoy.extensions.network.socket_interface.v3)
                pub mod v3 {
                    include!("envoy.extensions.network.socket_interface.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.network.socket_interface.v3)
                }
            }
        }
        pub mod path {
            pub mod r#match {
                pub mod uri_template {
                    // @@protoc_insertion_point(attribute:envoy.extensions.path.match.uri_template.v3)
                    pub mod v3 {
                        include!("envoy.extensions.path.match.uri_template.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.path.match.uri_template.v3)
                    }
                }
            }
            pub mod rewrite {
                pub mod uri_template {
                    // @@protoc_insertion_point(attribute:envoy.extensions.path.rewrite.uri_template.v3)
                    pub mod v3 {
                        include!("envoy.extensions.path.rewrite.uri_template.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.path.rewrite.uri_template.v3)
                    }
                }
            }
        }
        pub mod private_key_providers {
            pub mod cryptomb {
                // @@protoc_insertion_point(attribute:envoy.extensions.private_key_providers.cryptomb.v3alpha)
                pub mod v3alpha {
                    include!("envoy.extensions.private_key_providers.cryptomb.v3alpha.rs");
                    // @@protoc_insertion_point(envoy.extensions.private_key_providers.cryptomb.v3alpha)
                }
            }
            pub mod qat {
                // @@protoc_insertion_point(attribute:envoy.extensions.private_key_providers.qat.v3alpha)
                pub mod v3alpha {
                    include!("envoy.extensions.private_key_providers.qat.v3alpha.rs");
                    // @@protoc_insertion_point(envoy.extensions.private_key_providers.qat.v3alpha)
                }
            }
        }
        pub mod quic {
            pub mod connection_id_generator {
                // @@protoc_insertion_point(attribute:envoy.extensions.quic.connection_id_generator.v3)
                pub mod v3 {
                    include!("envoy.extensions.quic.connection_id_generator.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.quic.connection_id_generator.v3)
                }
            }
            pub mod crypto_stream {
                // @@protoc_insertion_point(attribute:envoy.extensions.quic.crypto_stream.v3)
                pub mod v3 {
                    include!("envoy.extensions.quic.crypto_stream.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.quic.crypto_stream.v3)
                }
            }
            pub mod proof_source {
                // @@protoc_insertion_point(attribute:envoy.extensions.quic.proof_source.v3)
                pub mod v3 {
                    include!("envoy.extensions.quic.proof_source.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.quic.proof_source.v3)
                }
            }
            pub mod server_preferred_address {
                // @@protoc_insertion_point(attribute:envoy.extensions.quic.server_preferred_address.v3)
                pub mod v3 {
                    include!("envoy.extensions.quic.server_preferred_address.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.quic.server_preferred_address.v3)
                }
            }
        }
        pub mod rate_limit_descriptors {
            pub mod expr {
                // @@protoc_insertion_point(attribute:envoy.extensions.rate_limit_descriptors.expr.v3)
                pub mod v3 {
                    include!("envoy.extensions.rate_limit_descriptors.expr.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.rate_limit_descriptors.expr.v3)
                }
            }
        }
        pub mod rbac {
            pub mod matchers {
                pub mod upstream_ip_port {
                    // @@protoc_insertion_point(attribute:envoy.extensions.rbac.matchers.upstream_ip_port.v3)
                    pub mod v3 {
                        include!("envoy.extensions.rbac.matchers.upstream_ip_port.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.rbac.matchers.upstream_ip_port.v3)
                    }
                }
            }
        }
        pub mod regex_engines {
            pub mod hyperscan {
                // @@protoc_insertion_point(attribute:envoy.extensions.regex_engines.hyperscan.v3alpha)
                pub mod v3alpha {
                    include!("envoy.extensions.regex_engines.hyperscan.v3alpha.rs");
                    // @@protoc_insertion_point(envoy.extensions.regex_engines.hyperscan.v3alpha)
                }
            }
            // @@protoc_insertion_point(attribute:envoy.extensions.regex_engines.v3)
            pub mod v3 {
                include!("envoy.extensions.regex_engines.v3.rs");
                // @@protoc_insertion_point(envoy.extensions.regex_engines.v3)
            }
        }
        pub mod request_id {
            pub mod uuid {
                // @@protoc_insertion_point(attribute:envoy.extensions.request_id.uuid.v3)
                pub mod v3 {
                    include!("envoy.extensions.request_id.uuid.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.request_id.uuid.v3)
                }
            }
        }
        pub mod resource_monitors {
            pub mod downstream_connections {
                // @@protoc_insertion_point(attribute:envoy.extensions.resource_monitors.downstream_connections.v3)
                pub mod v3 {
                    include!("envoy.extensions.resource_monitors.downstream_connections.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.resource_monitors.downstream_connections.v3)
                }
            }
            pub mod fixed_heap {
                // @@protoc_insertion_point(attribute:envoy.extensions.resource_monitors.fixed_heap.v3)
                pub mod v3 {
                    include!("envoy.extensions.resource_monitors.fixed_heap.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.resource_monitors.fixed_heap.v3)
                }
            }
            pub mod injected_resource {
                // @@protoc_insertion_point(attribute:envoy.extensions.resource_monitors.injected_resource.v3)
                pub mod v3 {
                    include!("envoy.extensions.resource_monitors.injected_resource.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.resource_monitors.injected_resource.v3)
                }
            }
        }
        pub mod retry {
            pub mod host {
                pub mod omit_canary_hosts {
                    // @@protoc_insertion_point(attribute:envoy.extensions.retry.host.omit_canary_hosts.v3)
                    pub mod v3 {
                        include!("envoy.extensions.retry.host.omit_canary_hosts.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.retry.host.omit_canary_hosts.v3)
                    }
                }
                pub mod omit_host_metadata {
                    // @@protoc_insertion_point(attribute:envoy.extensions.retry.host.omit_host_metadata.v3)
                    pub mod v3 {
                        include!("envoy.extensions.retry.host.omit_host_metadata.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.retry.host.omit_host_metadata.v3)
                    }
                }
                pub mod previous_hosts {
                    // @@protoc_insertion_point(attribute:envoy.extensions.retry.host.previous_hosts.v3)
                    pub mod v3 {
                        include!("envoy.extensions.retry.host.previous_hosts.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.retry.host.previous_hosts.v3)
                    }
                }
            }
            pub mod priority {
                pub mod previous_priorities {
                    // @@protoc_insertion_point(attribute:envoy.extensions.retry.priority.previous_priorities.v3)
                    pub mod v3 {
                        include!("envoy.extensions.retry.priority.previous_priorities.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.retry.priority.previous_priorities.v3)
                    }
                }
            }
        }
        pub mod stat_sinks {
            pub mod graphite_statsd {
                // @@protoc_insertion_point(attribute:envoy.extensions.stat_sinks.graphite_statsd.v3)
                pub mod v3 {
                    include!("envoy.extensions.stat_sinks.graphite_statsd.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.stat_sinks.graphite_statsd.v3)
                }
            }
            pub mod wasm {
                // @@protoc_insertion_point(attribute:envoy.extensions.stat_sinks.wasm.v3)
                pub mod v3 {
                    include!("envoy.extensions.stat_sinks.wasm.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.stat_sinks.wasm.v3)
                }
            }
        }
        pub mod transport_sockets {
            pub mod alts {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.alts.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.alts.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.alts.v3)
                }
            }
            pub mod http_11_proxy {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.http_11_proxy.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.http_11_proxy.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.http_11_proxy.v3)
                }
            }
            pub mod internal_upstream {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.internal_upstream.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.internal_upstream.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.internal_upstream.v3)
                }
            }
            pub mod proxy_protocol {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.proxy_protocol.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.proxy_protocol.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.proxy_protocol.v3)
                }
            }
            pub mod quic {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.quic.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.quic.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.quic.v3)
                }
            }
            pub mod raw_buffer {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.raw_buffer.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.raw_buffer.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.raw_buffer.v3)
                }
            }
            pub mod s2a {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.s2a.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.s2a.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.s2a.v3)
                }
            }
            pub mod starttls {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.starttls.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.starttls.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.starttls.v3)
                }
            }
            pub mod tap {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.tap.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.tap.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.tap.v3)
                }
            }
            pub mod tcp_stats {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.tcp_stats.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.tcp_stats.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.tcp_stats.v3)
                }
            }
            pub mod tls {
                // @@protoc_insertion_point(attribute:envoy.extensions.transport_sockets.tls.v3)
                pub mod v3 {
                    include!("envoy.extensions.transport_sockets.tls.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.transport_sockets.tls.v3)
                }
            }
        }
        pub mod udp_packet_writer {
            // @@protoc_insertion_point(attribute:envoy.extensions.udp_packet_writer.v3)
            pub mod v3 {
                include!("envoy.extensions.udp_packet_writer.v3.rs");
                // @@protoc_insertion_point(envoy.extensions.udp_packet_writer.v3)
            }
        }
        pub mod upstreams {
            pub mod http {
                pub mod generic {
                    // @@protoc_insertion_point(attribute:envoy.extensions.upstreams.http.generic.v3)
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.generic.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.upstreams.http.generic.v3)
                    }
                }
                pub mod http {
                    // @@protoc_insertion_point(attribute:envoy.extensions.upstreams.http.http.v3)
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.http.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.upstreams.http.http.v3)
                    }
                }
                pub mod tcp {
                    // @@protoc_insertion_point(attribute:envoy.extensions.upstreams.http.tcp.v3)
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.http.tcp.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.upstreams.http.tcp.v3)
                    }
                }
                // @@protoc_insertion_point(attribute:envoy.extensions.upstreams.http.v3)
                pub mod v3 {
                    include!("envoy.extensions.upstreams.http.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.upstreams.http.v3)
                }
            }
            pub mod tcp {
                pub mod generic {
                    // @@protoc_insertion_point(attribute:envoy.extensions.upstreams.tcp.generic.v3)
                    pub mod v3 {
                        include!("envoy.extensions.upstreams.tcp.generic.v3.rs");
                        // @@protoc_insertion_point(envoy.extensions.upstreams.tcp.generic.v3)
                    }
                }
                // @@protoc_insertion_point(attribute:envoy.extensions.upstreams.tcp.v3)
                pub mod v3 {
                    include!("envoy.extensions.upstreams.tcp.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.upstreams.tcp.v3)
                }
            }
        }
        pub mod vcl {
            // @@protoc_insertion_point(attribute:envoy.extensions.vcl.v3alpha)
            pub mod v3alpha {
                include!("envoy.extensions.vcl.v3alpha.rs");
                // @@protoc_insertion_point(envoy.extensions.vcl.v3alpha)
            }
        }
        pub mod wasm {
            // @@protoc_insertion_point(attribute:envoy.extensions.wasm.v3)
            pub mod v3 {
                include!("envoy.extensions.wasm.v3.rs");
                // @@protoc_insertion_point(envoy.extensions.wasm.v3)
            }
        }
        pub mod watchdog {
            pub mod profile_action {
                // @@protoc_insertion_point(attribute:envoy.extensions.watchdog.profile_action.v3)
                pub mod v3 {
                    include!("envoy.extensions.watchdog.profile_action.v3.rs");
                    // @@protoc_insertion_point(envoy.extensions.watchdog.profile_action.v3)
                }
            }
        }
    }
    // @@protoc_insertion_point(attribute:envoy.type)
    pub mod r#type {
        include!("envoy.type.rs");
        // @@protoc_insertion_point(envoy.type)
        pub mod http {
            // @@protoc_insertion_point(attribute:envoy.type.http.v3)
            pub mod v3 {
                include!("envoy.type.http.v3.rs");
                // @@protoc_insertion_point(envoy.type.http.v3)
            }
        }
        // @@protoc_insertion_point(attribute:envoy.type.matcher)
        pub mod matcher {
            include!("envoy.type.matcher.rs");
            // @@protoc_insertion_point(envoy.type.matcher)
            // @@protoc_insertion_point(attribute:envoy.type.matcher.v3)
            pub mod v3 {
                include!("envoy.type.matcher.v3.rs");
                // @@protoc_insertion_point(envoy.type.matcher.v3)
            }
        }
        pub mod metadata {
            // @@protoc_insertion_point(attribute:envoy.type.metadata.v2)
            pub mod v2 {
                include!("envoy.type.metadata.v2.rs");
                // @@protoc_insertion_point(envoy.type.metadata.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.type.metadata.v3)
            pub mod v3 {
                include!("envoy.type.metadata.v3.rs");
                // @@protoc_insertion_point(envoy.type.metadata.v3)
            }
        }
        pub mod tracing {
            // @@protoc_insertion_point(attribute:envoy.type.tracing.v2)
            pub mod v2 {
                include!("envoy.type.tracing.v2.rs");
                // @@protoc_insertion_point(envoy.type.tracing.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.type.tracing.v3)
            pub mod v3 {
                include!("envoy.type.tracing.v3.rs");
                // @@protoc_insertion_point(envoy.type.tracing.v3)
            }
        }
        // @@protoc_insertion_point(attribute:envoy.type.v3)
        pub mod v3 {
            include!("envoy.type.v3.rs");
            // @@protoc_insertion_point(envoy.type.v3)
        }
    }
    pub mod service {
        pub mod accesslog {
            // @@protoc_insertion_point(attribute:envoy.service.accesslog.v2)
            pub mod v2 {
                include!("envoy.service.accesslog.v2.rs");
                // @@protoc_insertion_point(envoy.service.accesslog.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.accesslog.v3)
            pub mod v3 {
                include!("envoy.service.accesslog.v3.rs");
                // @@protoc_insertion_point(envoy.service.accesslog.v3)
            }
        }
        pub mod auth {
            // @@protoc_insertion_point(attribute:envoy.service.auth.v2)
            pub mod v2 {
                include!("envoy.service.auth.v2.rs");
                // @@protoc_insertion_point(envoy.service.auth.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.auth.v2alpha)
            pub mod v2alpha {
                include!("envoy.service.auth.v2alpha.rs");
                // @@protoc_insertion_point(envoy.service.auth.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.service.auth.v3)
            pub mod v3 {
                include!("envoy.service.auth.v3.rs");
                // @@protoc_insertion_point(envoy.service.auth.v3)
            }
        }
        pub mod cluster {
            // @@protoc_insertion_point(attribute:envoy.service.cluster.v3)
            pub mod v3 {
                include!("envoy.service.cluster.v3.rs");
                // @@protoc_insertion_point(envoy.service.cluster.v3)
            }
        }
        pub mod discovery {
            // @@protoc_insertion_point(attribute:envoy.service.discovery.v2)
            pub mod v2 {
                include!("envoy.service.discovery.v2.rs");
                // @@protoc_insertion_point(envoy.service.discovery.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.discovery.v3)
            pub mod v3 {
                include!("envoy.service.discovery.v3.rs");
                // @@protoc_insertion_point(envoy.service.discovery.v3)
            }
        }
        pub mod endpoint {
            // @@protoc_insertion_point(attribute:envoy.service.endpoint.v3)
            pub mod v3 {
                include!("envoy.service.endpoint.v3.rs");
                // @@protoc_insertion_point(envoy.service.endpoint.v3)
            }
        }
        pub mod event_reporting {
            // @@protoc_insertion_point(attribute:envoy.service.event_reporting.v2alpha)
            pub mod v2alpha {
                include!("envoy.service.event_reporting.v2alpha.rs");
                // @@protoc_insertion_point(envoy.service.event_reporting.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.service.event_reporting.v3)
            pub mod v3 {
                include!("envoy.service.event_reporting.v3.rs");
                // @@protoc_insertion_point(envoy.service.event_reporting.v3)
            }
        }
        pub mod ext_proc {
            // @@protoc_insertion_point(attribute:envoy.service.ext_proc.v3)
            pub mod v3 {
                include!("envoy.service.ext_proc.v3.rs");
                // @@protoc_insertion_point(envoy.service.ext_proc.v3)
            }
        }
        pub mod extension {
            // @@protoc_insertion_point(attribute:envoy.service.extension.v3)
            pub mod v3 {
                include!("envoy.service.extension.v3.rs");
                // @@protoc_insertion_point(envoy.service.extension.v3)
            }
        }
        pub mod health {
            // @@protoc_insertion_point(attribute:envoy.service.health.v3)
            pub mod v3 {
                include!("envoy.service.health.v3.rs");
                // @@protoc_insertion_point(envoy.service.health.v3)
            }
        }
        pub mod listener {
            // @@protoc_insertion_point(attribute:envoy.service.listener.v3)
            pub mod v3 {
                include!("envoy.service.listener.v3.rs");
                // @@protoc_insertion_point(envoy.service.listener.v3)
            }
        }
        pub mod load_stats {
            // @@protoc_insertion_point(attribute:envoy.service.load_stats.v2)
            pub mod v2 {
                include!("envoy.service.load_stats.v2.rs");
                // @@protoc_insertion_point(envoy.service.load_stats.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.load_stats.v3)
            pub mod v3 {
                include!("envoy.service.load_stats.v3.rs");
                // @@protoc_insertion_point(envoy.service.load_stats.v3)
            }
        }
        pub mod metrics {
            // @@protoc_insertion_point(attribute:envoy.service.metrics.v2)
            pub mod v2 {
                include!("envoy.service.metrics.v2.rs");
                // @@protoc_insertion_point(envoy.service.metrics.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.metrics.v3)
            pub mod v3 {
                include!("envoy.service.metrics.v3.rs");
                // @@protoc_insertion_point(envoy.service.metrics.v3)
            }
        }
        pub mod rate_limit_quota {
            // @@protoc_insertion_point(attribute:envoy.service.rate_limit_quota.v3)
            pub mod v3 {
                include!("envoy.service.rate_limit_quota.v3.rs");
                // @@protoc_insertion_point(envoy.service.rate_limit_quota.v3)
            }
        }
        pub mod ratelimit {
            // @@protoc_insertion_point(attribute:envoy.service.ratelimit.v2)
            pub mod v2 {
                include!("envoy.service.ratelimit.v2.rs");
                // @@protoc_insertion_point(envoy.service.ratelimit.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.ratelimit.v3)
            pub mod v3 {
                include!("envoy.service.ratelimit.v3.rs");
                // @@protoc_insertion_point(envoy.service.ratelimit.v3)
            }
        }
        pub mod route {
            // @@protoc_insertion_point(attribute:envoy.service.route.v3)
            pub mod v3 {
                include!("envoy.service.route.v3.rs");
                // @@protoc_insertion_point(envoy.service.route.v3)
            }
        }
        pub mod runtime {
            // @@protoc_insertion_point(attribute:envoy.service.runtime.v3)
            pub mod v3 {
                include!("envoy.service.runtime.v3.rs");
                // @@protoc_insertion_point(envoy.service.runtime.v3)
            }
        }
        pub mod secret {
            // @@protoc_insertion_point(attribute:envoy.service.secret.v3)
            pub mod v3 {
                include!("envoy.service.secret.v3.rs");
                // @@protoc_insertion_point(envoy.service.secret.v3)
            }
        }
        pub mod status {
            // @@protoc_insertion_point(attribute:envoy.service.status.v2)
            pub mod v2 {
                include!("envoy.service.status.v2.rs");
                // @@protoc_insertion_point(envoy.service.status.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.status.v3)
            pub mod v3 {
                include!("envoy.service.status.v3.rs");
                // @@protoc_insertion_point(envoy.service.status.v3)
            }
        }
        pub mod tap {
            // @@protoc_insertion_point(attribute:envoy.service.tap.v2alpha)
            pub mod v2alpha {
                include!("envoy.service.tap.v2alpha.rs");
                // @@protoc_insertion_point(envoy.service.tap.v2alpha)
            }
            // @@protoc_insertion_point(attribute:envoy.service.tap.v3)
            pub mod v3 {
                include!("envoy.service.tap.v3.rs");
                // @@protoc_insertion_point(envoy.service.tap.v3)
            }
        }
        pub mod trace {
            // @@protoc_insertion_point(attribute:envoy.service.trace.v2)
            pub mod v2 {
                include!("envoy.service.trace.v2.rs");
                // @@protoc_insertion_point(envoy.service.trace.v2)
            }
            // @@protoc_insertion_point(attribute:envoy.service.trace.v3)
            pub mod v3 {
                include!("envoy.service.trace.v3.rs");
                // @@protoc_insertion_point(envoy.service.trace.v3)
            }
        }
    }
    pub mod watchdog {
        // @@protoc_insertion_point(attribute:envoy.watchdog.v3)
        pub mod v3 {
            include!("envoy.watchdog.v3.rs");
            // @@protoc_insertion_point(envoy.watchdog.v3)
        }
    }
}
pub mod google {
    // @@protoc_insertion_point(attribute:google.api)
    pub mod api {
        include!("google.api.rs");
        // @@protoc_insertion_point(google.api)
        pub mod expr {
            // @@protoc_insertion_point(attribute:google.api.expr.v1alpha1)
            pub mod v1alpha1 {
                include!("google.api.expr.v1alpha1.rs");
                // @@protoc_insertion_point(google.api.expr.v1alpha1)
            }
        }
    }
    // @@protoc_insertion_point(attribute:google.protobuf)
    pub mod protobuf {
        include!("google.protobuf.rs");
        // @@protoc_insertion_point(google.protobuf)
    }
    // @@protoc_insertion_point(attribute:google.rpc)
    pub mod rpc {
        include!("google.rpc.rs");
        // @@protoc_insertion_point(google.rpc)
    }
}
pub mod io {
    pub mod prometheus {
        // @@protoc_insertion_point(attribute:io.prometheus.client)
        pub mod client {
            include!("io.prometheus.client.rs");
            // @@protoc_insertion_point(io.prometheus.client)
        }
    }
}
pub mod opencensus {
    pub mod proto {
        pub mod resource {
            // @@protoc_insertion_point(attribute:opencensus.proto.resource.v1)
            pub mod v1 {
                include!("opencensus.proto.resource.v1.rs");
                // @@protoc_insertion_point(opencensus.proto.resource.v1)
            }
        }
        pub mod trace {
            // @@protoc_insertion_point(attribute:opencensus.proto.trace.v1)
            pub mod v1 {
                include!("opencensus.proto.trace.v1.rs");
                // @@protoc_insertion_point(opencensus.proto.trace.v1)
            }
        }
    }
}
pub mod opentelemetry {
    pub mod proto {
        pub mod common {
            // @@protoc_insertion_point(attribute:opentelemetry.proto.common.v1)
            pub mod v1 {
                include!("opentelemetry.proto.common.v1.rs");
                // @@protoc_insertion_point(opentelemetry.proto.common.v1)
            }
        }
    }
}
pub mod udpa {
    // @@protoc_insertion_point(attribute:udpa.annotations)
    pub mod annotations {
        include!("udpa.annotations.rs");
        // @@protoc_insertion_point(udpa.annotations)
    }
}
// @@protoc_insertion_point(attribute:validate)
pub mod validate {
    include!("validate.rs");
    // @@protoc_insertion_point(validate)
}
pub mod xds {
    pub mod annotations {
        // @@protoc_insertion_point(attribute:xds.annotations.v3)
        pub mod v3 {
            include!("xds.annotations.v3.rs");
            // @@protoc_insertion_point(xds.annotations.v3)
        }
    }
    pub mod core {
        // @@protoc_insertion_point(attribute:xds.core.v3)
        pub mod v3 {
            include!("xds.core.v3.rs");
            // @@protoc_insertion_point(xds.core.v3)
        }
    }
    pub mod r#type {
        pub mod matcher {
            // @@protoc_insertion_point(attribute:xds.type.matcher.v3)
            pub mod v3 {
                include!("xds.type.matcher.v3.rs");
                // @@protoc_insertion_point(xds.type.matcher.v3)
            }
        }
    }
}
