// @generated
impl serde::Serialize for CdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.CdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CdsDummy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.CdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.CdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Cluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transport_socket_matches.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.alt_stat_name.is_empty() {
            len += 1;
        }
        if self.eds_cluster_config.is_some() {
            len += 1;
        }
        if self.connect_timeout.is_some() {
            len += 1;
        }
        if self.per_connection_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if self.lb_policy != 0 {
            len += 1;
        }
        if !self.hosts.is_empty() {
            len += 1;
        }
        if self.load_assignment.is_some() {
            len += 1;
        }
        if !self.health_checks.is_empty() {
            len += 1;
        }
        if self.max_requests_per_connection.is_some() {
            len += 1;
        }
        if self.circuit_breakers.is_some() {
            len += 1;
        }
        if self.tls_context.is_some() {
            len += 1;
        }
        if self.upstream_http_protocol_options.is_some() {
            len += 1;
        }
        if self.common_http_protocol_options.is_some() {
            len += 1;
        }
        if self.http_protocol_options.is_some() {
            len += 1;
        }
        if self.http2_protocol_options.is_some() {
            len += 1;
        }
        if !self.extension_protocol_options.is_empty() {
            len += 1;
        }
        if !self.typed_extension_protocol_options.is_empty() {
            len += 1;
        }
        if self.dns_refresh_rate.is_some() {
            len += 1;
        }
        if self.dns_failure_refresh_rate.is_some() {
            len += 1;
        }
        if self.respect_dns_ttl {
            len += 1;
        }
        if self.dns_lookup_family != 0 {
            len += 1;
        }
        if !self.dns_resolvers.is_empty() {
            len += 1;
        }
        if self.use_tcp_for_dns_lookups {
            len += 1;
        }
        if self.outlier_detection.is_some() {
            len += 1;
        }
        if self.cleanup_interval.is_some() {
            len += 1;
        }
        if self.upstream_bind_config.is_some() {
            len += 1;
        }
        if self.lb_subset_config.is_some() {
            len += 1;
        }
        if self.common_lb_config.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.protocol_selection != 0 {
            len += 1;
        }
        if self.upstream_connection_options.is_some() {
            len += 1;
        }
        if self.close_connections_on_host_health_failure {
            len += 1;
        }
        if self.drain_connections_on_host_removal {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.load_balancing_policy.is_some() {
            len += 1;
        }
        if self.lrs_server.is_some() {
            len += 1;
        }
        if self.track_timeout_budgets {
            len += 1;
        }
        if self.cluster_discovery_type.is_some() {
            len += 1;
        }
        if self.lb_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster", len)?;
        if !self.transport_socket_matches.is_empty() {
            struct_ser.serialize_field("transportSocketMatches", &self.transport_socket_matches)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.alt_stat_name.is_empty() {
            struct_ser.serialize_field("altStatName", &self.alt_stat_name)?;
        }
        if let Some(v) = self.eds_cluster_config.as_ref() {
            struct_ser.serialize_field("edsClusterConfig", v)?;
        }
        if let Some(v) = self.connect_timeout.as_ref() {
            struct_ser.serialize_field("connectTimeout", v)?;
        }
        if let Some(v) = self.per_connection_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("perConnectionBufferLimitBytes", v)?;
        }
        if self.lb_policy != 0 {
            let v = cluster::LbPolicy::from_i32(self.lb_policy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.lb_policy)))?;
            struct_ser.serialize_field("lbPolicy", &v)?;
        }
        if !self.hosts.is_empty() {
            struct_ser.serialize_field("hosts", &self.hosts)?;
        }
        if let Some(v) = self.load_assignment.as_ref() {
            struct_ser.serialize_field("loadAssignment", v)?;
        }
        if !self.health_checks.is_empty() {
            struct_ser.serialize_field("healthChecks", &self.health_checks)?;
        }
        if let Some(v) = self.max_requests_per_connection.as_ref() {
            struct_ser.serialize_field("maxRequestsPerConnection", v)?;
        }
        if let Some(v) = self.circuit_breakers.as_ref() {
            struct_ser.serialize_field("circuitBreakers", v)?;
        }
        if let Some(v) = self.tls_context.as_ref() {
            struct_ser.serialize_field("tlsContext", v)?;
        }
        if let Some(v) = self.upstream_http_protocol_options.as_ref() {
            struct_ser.serialize_field("upstreamHttpProtocolOptions", v)?;
        }
        if let Some(v) = self.common_http_protocol_options.as_ref() {
            struct_ser.serialize_field("commonHttpProtocolOptions", v)?;
        }
        if let Some(v) = self.http_protocol_options.as_ref() {
            struct_ser.serialize_field("httpProtocolOptions", v)?;
        }
        if let Some(v) = self.http2_protocol_options.as_ref() {
            struct_ser.serialize_field("http2ProtocolOptions", v)?;
        }
        if !self.extension_protocol_options.is_empty() {
            struct_ser.serialize_field("extensionProtocolOptions", &self.extension_protocol_options)?;
        }
        if !self.typed_extension_protocol_options.is_empty() {
            struct_ser.serialize_field("typedExtensionProtocolOptions", &self.typed_extension_protocol_options)?;
        }
        if let Some(v) = self.dns_refresh_rate.as_ref() {
            struct_ser.serialize_field("dnsRefreshRate", v)?;
        }
        if let Some(v) = self.dns_failure_refresh_rate.as_ref() {
            struct_ser.serialize_field("dnsFailureRefreshRate", v)?;
        }
        if self.respect_dns_ttl {
            struct_ser.serialize_field("respectDnsTtl", &self.respect_dns_ttl)?;
        }
        if self.dns_lookup_family != 0 {
            let v = cluster::DnsLookupFamily::from_i32(self.dns_lookup_family)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.dns_lookup_family)))?;
            struct_ser.serialize_field("dnsLookupFamily", &v)?;
        }
        if !self.dns_resolvers.is_empty() {
            struct_ser.serialize_field("dnsResolvers", &self.dns_resolvers)?;
        }
        if self.use_tcp_for_dns_lookups {
            struct_ser.serialize_field("useTcpForDnsLookups", &self.use_tcp_for_dns_lookups)?;
        }
        if let Some(v) = self.outlier_detection.as_ref() {
            struct_ser.serialize_field("outlierDetection", v)?;
        }
        if let Some(v) = self.cleanup_interval.as_ref() {
            struct_ser.serialize_field("cleanupInterval", v)?;
        }
        if let Some(v) = self.upstream_bind_config.as_ref() {
            struct_ser.serialize_field("upstreamBindConfig", v)?;
        }
        if let Some(v) = self.lb_subset_config.as_ref() {
            struct_ser.serialize_field("lbSubsetConfig", v)?;
        }
        if let Some(v) = self.common_lb_config.as_ref() {
            struct_ser.serialize_field("commonLbConfig", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transportSocket", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.protocol_selection != 0 {
            let v = cluster::ClusterProtocolSelection::from_i32(self.protocol_selection)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol_selection)))?;
            struct_ser.serialize_field("protocolSelection", &v)?;
        }
        if let Some(v) = self.upstream_connection_options.as_ref() {
            struct_ser.serialize_field("upstreamConnectionOptions", v)?;
        }
        if self.close_connections_on_host_health_failure {
            struct_ser.serialize_field("closeConnectionsOnHostHealthFailure", &self.close_connections_on_host_health_failure)?;
        }
        if self.drain_connections_on_host_removal {
            struct_ser.serialize_field("drainConnectionsOnHostRemoval", &self.drain_connections_on_host_removal)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if let Some(v) = self.load_balancing_policy.as_ref() {
            struct_ser.serialize_field("loadBalancingPolicy", v)?;
        }
        if let Some(v) = self.lrs_server.as_ref() {
            struct_ser.serialize_field("lrsServer", v)?;
        }
        if self.track_timeout_budgets {
            struct_ser.serialize_field("trackTimeoutBudgets", &self.track_timeout_budgets)?;
        }
        if let Some(v) = self.cluster_discovery_type.as_ref() {
            match v {
                cluster::ClusterDiscoveryType::Type(v) => {
                    let v = cluster::DiscoveryType::from_i32(*v)
                        .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
                    struct_ser.serialize_field("type", &v)?;
                }
                cluster::ClusterDiscoveryType::ClusterType(v) => {
                    struct_ser.serialize_field("clusterType", v)?;
                }
            }
        }
        if let Some(v) = self.lb_config.as_ref() {
            match v {
                cluster::LbConfig::RingHashLbConfig(v) => {
                    struct_ser.serialize_field("ringHashLbConfig", v)?;
                }
                cluster::LbConfig::OriginalDstLbConfig(v) => {
                    struct_ser.serialize_field("originalDstLbConfig", v)?;
                }
                cluster::LbConfig::LeastRequestLbConfig(v) => {
                    struct_ser.serialize_field("leastRequestLbConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transport_socket_matches",
            "transportSocketMatches",
            "name",
            "alt_stat_name",
            "altStatName",
            "eds_cluster_config",
            "edsClusterConfig",
            "connect_timeout",
            "connectTimeout",
            "per_connection_buffer_limit_bytes",
            "perConnectionBufferLimitBytes",
            "lb_policy",
            "lbPolicy",
            "hosts",
            "load_assignment",
            "loadAssignment",
            "health_checks",
            "healthChecks",
            "max_requests_per_connection",
            "maxRequestsPerConnection",
            "circuit_breakers",
            "circuitBreakers",
            "tls_context",
            "tlsContext",
            "upstream_http_protocol_options",
            "upstreamHttpProtocolOptions",
            "common_http_protocol_options",
            "commonHttpProtocolOptions",
            "http_protocol_options",
            "httpProtocolOptions",
            "http2_protocol_options",
            "http2ProtocolOptions",
            "extension_protocol_options",
            "extensionProtocolOptions",
            "typed_extension_protocol_options",
            "typedExtensionProtocolOptions",
            "dns_refresh_rate",
            "dnsRefreshRate",
            "dns_failure_refresh_rate",
            "dnsFailureRefreshRate",
            "respect_dns_ttl",
            "respectDnsTtl",
            "dns_lookup_family",
            "dnsLookupFamily",
            "dns_resolvers",
            "dnsResolvers",
            "use_tcp_for_dns_lookups",
            "useTcpForDnsLookups",
            "outlier_detection",
            "outlierDetection",
            "cleanup_interval",
            "cleanupInterval",
            "upstream_bind_config",
            "upstreamBindConfig",
            "lb_subset_config",
            "lbSubsetConfig",
            "common_lb_config",
            "commonLbConfig",
            "transport_socket",
            "transportSocket",
            "metadata",
            "protocol_selection",
            "protocolSelection",
            "upstream_connection_options",
            "upstreamConnectionOptions",
            "close_connections_on_host_health_failure",
            "closeConnectionsOnHostHealthFailure",
            "drain_connections_on_host_removal",
            "drainConnectionsOnHostRemoval",
            "filters",
            "load_balancing_policy",
            "loadBalancingPolicy",
            "lrs_server",
            "lrsServer",
            "track_timeout_budgets",
            "trackTimeoutBudgets",
            "type",
            "cluster_type",
            "clusterType",
            "ring_hash_lb_config",
            "ringHashLbConfig",
            "original_dst_lb_config",
            "originalDstLbConfig",
            "least_request_lb_config",
            "leastRequestLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransportSocketMatches,
            Name,
            AltStatName,
            EdsClusterConfig,
            ConnectTimeout,
            PerConnectionBufferLimitBytes,
            LbPolicy,
            Hosts,
            LoadAssignment,
            HealthChecks,
            MaxRequestsPerConnection,
            CircuitBreakers,
            TlsContext,
            UpstreamHttpProtocolOptions,
            CommonHttpProtocolOptions,
            HttpProtocolOptions,
            Http2ProtocolOptions,
            ExtensionProtocolOptions,
            TypedExtensionProtocolOptions,
            DnsRefreshRate,
            DnsFailureRefreshRate,
            RespectDnsTtl,
            DnsLookupFamily,
            DnsResolvers,
            UseTcpForDnsLookups,
            OutlierDetection,
            CleanupInterval,
            UpstreamBindConfig,
            LbSubsetConfig,
            CommonLbConfig,
            TransportSocket,
            Metadata,
            ProtocolSelection,
            UpstreamConnectionOptions,
            CloseConnectionsOnHostHealthFailure,
            DrainConnectionsOnHostRemoval,
            Filters,
            LoadBalancingPolicy,
            LrsServer,
            TrackTimeoutBudgets,
            Type,
            ClusterType,
            RingHashLbConfig,
            OriginalDstLbConfig,
            LeastRequestLbConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transportSocketMatches" | "transport_socket_matches" => Ok(GeneratedField::TransportSocketMatches),
                            "name" => Ok(GeneratedField::Name),
                            "altStatName" | "alt_stat_name" => Ok(GeneratedField::AltStatName),
                            "edsClusterConfig" | "eds_cluster_config" => Ok(GeneratedField::EdsClusterConfig),
                            "connectTimeout" | "connect_timeout" => Ok(GeneratedField::ConnectTimeout),
                            "perConnectionBufferLimitBytes" | "per_connection_buffer_limit_bytes" => Ok(GeneratedField::PerConnectionBufferLimitBytes),
                            "lbPolicy" | "lb_policy" => Ok(GeneratedField::LbPolicy),
                            "hosts" => Ok(GeneratedField::Hosts),
                            "loadAssignment" | "load_assignment" => Ok(GeneratedField::LoadAssignment),
                            "healthChecks" | "health_checks" => Ok(GeneratedField::HealthChecks),
                            "maxRequestsPerConnection" | "max_requests_per_connection" => Ok(GeneratedField::MaxRequestsPerConnection),
                            "circuitBreakers" | "circuit_breakers" => Ok(GeneratedField::CircuitBreakers),
                            "tlsContext" | "tls_context" => Ok(GeneratedField::TlsContext),
                            "upstreamHttpProtocolOptions" | "upstream_http_protocol_options" => Ok(GeneratedField::UpstreamHttpProtocolOptions),
                            "commonHttpProtocolOptions" | "common_http_protocol_options" => Ok(GeneratedField::CommonHttpProtocolOptions),
                            "httpProtocolOptions" | "http_protocol_options" => Ok(GeneratedField::HttpProtocolOptions),
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            "extensionProtocolOptions" | "extension_protocol_options" => Ok(GeneratedField::ExtensionProtocolOptions),
                            "typedExtensionProtocolOptions" | "typed_extension_protocol_options" => Ok(GeneratedField::TypedExtensionProtocolOptions),
                            "dnsRefreshRate" | "dns_refresh_rate" => Ok(GeneratedField::DnsRefreshRate),
                            "dnsFailureRefreshRate" | "dns_failure_refresh_rate" => Ok(GeneratedField::DnsFailureRefreshRate),
                            "respectDnsTtl" | "respect_dns_ttl" => Ok(GeneratedField::RespectDnsTtl),
                            "dnsLookupFamily" | "dns_lookup_family" => Ok(GeneratedField::DnsLookupFamily),
                            "dnsResolvers" | "dns_resolvers" => Ok(GeneratedField::DnsResolvers),
                            "useTcpForDnsLookups" | "use_tcp_for_dns_lookups" => Ok(GeneratedField::UseTcpForDnsLookups),
                            "outlierDetection" | "outlier_detection" => Ok(GeneratedField::OutlierDetection),
                            "cleanupInterval" | "cleanup_interval" => Ok(GeneratedField::CleanupInterval),
                            "upstreamBindConfig" | "upstream_bind_config" => Ok(GeneratedField::UpstreamBindConfig),
                            "lbSubsetConfig" | "lb_subset_config" => Ok(GeneratedField::LbSubsetConfig),
                            "commonLbConfig" | "common_lb_config" => Ok(GeneratedField::CommonLbConfig),
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "protocolSelection" | "protocol_selection" => Ok(GeneratedField::ProtocolSelection),
                            "upstreamConnectionOptions" | "upstream_connection_options" => Ok(GeneratedField::UpstreamConnectionOptions),
                            "closeConnectionsOnHostHealthFailure" | "close_connections_on_host_health_failure" => Ok(GeneratedField::CloseConnectionsOnHostHealthFailure),
                            "drainConnectionsOnHostRemoval" | "drain_connections_on_host_removal" => Ok(GeneratedField::DrainConnectionsOnHostRemoval),
                            "filters" => Ok(GeneratedField::Filters),
                            "loadBalancingPolicy" | "load_balancing_policy" => Ok(GeneratedField::LoadBalancingPolicy),
                            "lrsServer" | "lrs_server" => Ok(GeneratedField::LrsServer),
                            "trackTimeoutBudgets" | "track_timeout_budgets" => Ok(GeneratedField::TrackTimeoutBudgets),
                            "type" => Ok(GeneratedField::Type),
                            "clusterType" | "cluster_type" => Ok(GeneratedField::ClusterType),
                            "ringHashLbConfig" | "ring_hash_lb_config" => Ok(GeneratedField::RingHashLbConfig),
                            "originalDstLbConfig" | "original_dst_lb_config" => Ok(GeneratedField::OriginalDstLbConfig),
                            "leastRequestLbConfig" | "least_request_lb_config" => Ok(GeneratedField::LeastRequestLbConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Cluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport_socket_matches__ = None;
                let mut name__ = None;
                let mut alt_stat_name__ = None;
                let mut eds_cluster_config__ = None;
                let mut connect_timeout__ = None;
                let mut per_connection_buffer_limit_bytes__ = None;
                let mut lb_policy__ = None;
                let mut hosts__ = None;
                let mut load_assignment__ = None;
                let mut health_checks__ = None;
                let mut max_requests_per_connection__ = None;
                let mut circuit_breakers__ = None;
                let mut tls_context__ = None;
                let mut upstream_http_protocol_options__ = None;
                let mut common_http_protocol_options__ = None;
                let mut http_protocol_options__ = None;
                let mut http2_protocol_options__ = None;
                let mut extension_protocol_options__ = None;
                let mut typed_extension_protocol_options__ = None;
                let mut dns_refresh_rate__ = None;
                let mut dns_failure_refresh_rate__ = None;
                let mut respect_dns_ttl__ = None;
                let mut dns_lookup_family__ = None;
                let mut dns_resolvers__ = None;
                let mut use_tcp_for_dns_lookups__ = None;
                let mut outlier_detection__ = None;
                let mut cleanup_interval__ = None;
                let mut upstream_bind_config__ = None;
                let mut lb_subset_config__ = None;
                let mut common_lb_config__ = None;
                let mut transport_socket__ = None;
                let mut metadata__ = None;
                let mut protocol_selection__ = None;
                let mut upstream_connection_options__ = None;
                let mut close_connections_on_host_health_failure__ = None;
                let mut drain_connections_on_host_removal__ = None;
                let mut filters__ = None;
                let mut load_balancing_policy__ = None;
                let mut lrs_server__ = None;
                let mut track_timeout_budgets__ = None;
                let mut cluster_discovery_type__ = None;
                let mut lb_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TransportSocketMatches => {
                            if transport_socket_matches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocketMatches"));
                            }
                            transport_socket_matches__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::AltStatName => {
                            if alt_stat_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altStatName"));
                            }
                            alt_stat_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::EdsClusterConfig => {
                            if eds_cluster_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edsClusterConfig"));
                            }
                            eds_cluster_config__ = map.next_value()?;
                        }
                        GeneratedField::ConnectTimeout => {
                            if connect_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectTimeout"));
                            }
                            connect_timeout__ = map.next_value()?;
                        }
                        GeneratedField::PerConnectionBufferLimitBytes => {
                            if per_connection_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perConnectionBufferLimitBytes"));
                            }
                            per_connection_buffer_limit_bytes__ = map.next_value()?;
                        }
                        GeneratedField::LbPolicy => {
                            if lb_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lbPolicy"));
                            }
                            lb_policy__ = Some(map.next_value::<cluster::LbPolicy>()? as i32);
                        }
                        GeneratedField::Hosts => {
                            if hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hosts"));
                            }
                            hosts__ = Some(map.next_value()?);
                        }
                        GeneratedField::LoadAssignment => {
                            if load_assignment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadAssignment"));
                            }
                            load_assignment__ = map.next_value()?;
                        }
                        GeneratedField::HealthChecks => {
                            if health_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthChecks"));
                            }
                            health_checks__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxRequestsPerConnection => {
                            if max_requests_per_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestsPerConnection"));
                            }
                            max_requests_per_connection__ = map.next_value()?;
                        }
                        GeneratedField::CircuitBreakers => {
                            if circuit_breakers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circuitBreakers"));
                            }
                            circuit_breakers__ = map.next_value()?;
                        }
                        GeneratedField::TlsContext => {
                            if tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsContext"));
                            }
                            tls_context__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamHttpProtocolOptions => {
                            if upstream_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamHttpProtocolOptions"));
                            }
                            upstream_http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::CommonHttpProtocolOptions => {
                            if common_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonHttpProtocolOptions"));
                            }
                            common_http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::HttpProtocolOptions => {
                            if http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpProtocolOptions"));
                            }
                            http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::Http2ProtocolOptions => {
                            if http2_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2ProtocolOptions"));
                            }
                            http2_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::ExtensionProtocolOptions => {
                            if extension_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionProtocolOptions"));
                            }
                            extension_protocol_options__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::TypedExtensionProtocolOptions => {
                            if typed_extension_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedExtensionProtocolOptions"));
                            }
                            typed_extension_protocol_options__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DnsRefreshRate => {
                            if dns_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsRefreshRate"));
                            }
                            dns_refresh_rate__ = map.next_value()?;
                        }
                        GeneratedField::DnsFailureRefreshRate => {
                            if dns_failure_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsFailureRefreshRate"));
                            }
                            dns_failure_refresh_rate__ = map.next_value()?;
                        }
                        GeneratedField::RespectDnsTtl => {
                            if respect_dns_ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("respectDnsTtl"));
                            }
                            respect_dns_ttl__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsLookupFamily => {
                            if dns_lookup_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsLookupFamily"));
                            }
                            dns_lookup_family__ = Some(map.next_value::<cluster::DnsLookupFamily>()? as i32);
                        }
                        GeneratedField::DnsResolvers => {
                            if dns_resolvers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsResolvers"));
                            }
                            dns_resolvers__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseTcpForDnsLookups => {
                            if use_tcp_for_dns_lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useTcpForDnsLookups"));
                            }
                            use_tcp_for_dns_lookups__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutlierDetection => {
                            if outlier_detection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outlierDetection"));
                            }
                            outlier_detection__ = map.next_value()?;
                        }
                        GeneratedField::CleanupInterval => {
                            if cleanup_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cleanupInterval"));
                            }
                            cleanup_interval__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamBindConfig => {
                            if upstream_bind_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamBindConfig"));
                            }
                            upstream_bind_config__ = map.next_value()?;
                        }
                        GeneratedField::LbSubsetConfig => {
                            if lb_subset_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lbSubsetConfig"));
                            }
                            lb_subset_config__ = map.next_value()?;
                        }
                        GeneratedField::CommonLbConfig => {
                            if common_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonLbConfig"));
                            }
                            common_lb_config__ = map.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::ProtocolSelection => {
                            if protocol_selection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolSelection"));
                            }
                            protocol_selection__ = Some(map.next_value::<cluster::ClusterProtocolSelection>()? as i32);
                        }
                        GeneratedField::UpstreamConnectionOptions => {
                            if upstream_connection_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamConnectionOptions"));
                            }
                            upstream_connection_options__ = map.next_value()?;
                        }
                        GeneratedField::CloseConnectionsOnHostHealthFailure => {
                            if close_connections_on_host_health_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closeConnectionsOnHostHealthFailure"));
                            }
                            close_connections_on_host_health_failure__ = Some(map.next_value()?);
                        }
                        GeneratedField::DrainConnectionsOnHostRemoval => {
                            if drain_connections_on_host_removal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainConnectionsOnHostRemoval"));
                            }
                            drain_connections_on_host_removal__ = Some(map.next_value()?);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::LoadBalancingPolicy => {
                            if load_balancing_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancingPolicy"));
                            }
                            load_balancing_policy__ = map.next_value()?;
                        }
                        GeneratedField::LrsServer => {
                            if lrs_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lrsServer"));
                            }
                            lrs_server__ = map.next_value()?;
                        }
                        GeneratedField::TrackTimeoutBudgets => {
                            if track_timeout_budgets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackTimeoutBudgets"));
                            }
                            track_timeout_budgets__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if cluster_discovery_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            cluster_discovery_type__ = map.next_value::<::std::option::Option<cluster::DiscoveryType>>()?.map(|x| cluster::ClusterDiscoveryType::Type(x as i32));
                        }
                        GeneratedField::ClusterType => {
                            if cluster_discovery_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterType"));
                            }
                            cluster_discovery_type__ = map.next_value::<::std::option::Option<_>>()?.map(cluster::ClusterDiscoveryType::ClusterType)
;
                        }
                        GeneratedField::RingHashLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ringHashLbConfig"));
                            }
                            lb_config__ = map.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::RingHashLbConfig)
;
                        }
                        GeneratedField::OriginalDstLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalDstLbConfig"));
                            }
                            lb_config__ = map.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::OriginalDstLbConfig)
;
                        }
                        GeneratedField::LeastRequestLbConfig => {
                            if lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leastRequestLbConfig"));
                            }
                            lb_config__ = map.next_value::<::std::option::Option<_>>()?.map(cluster::LbConfig::LeastRequestLbConfig)
;
                        }
                    }
                }
                Ok(Cluster {
                    transport_socket_matches: transport_socket_matches__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    alt_stat_name: alt_stat_name__.unwrap_or_default(),
                    eds_cluster_config: eds_cluster_config__,
                    connect_timeout: connect_timeout__,
                    per_connection_buffer_limit_bytes: per_connection_buffer_limit_bytes__,
                    lb_policy: lb_policy__.unwrap_or_default(),
                    hosts: hosts__.unwrap_or_default(),
                    load_assignment: load_assignment__,
                    health_checks: health_checks__.unwrap_or_default(),
                    max_requests_per_connection: max_requests_per_connection__,
                    circuit_breakers: circuit_breakers__,
                    tls_context: tls_context__,
                    upstream_http_protocol_options: upstream_http_protocol_options__,
                    common_http_protocol_options: common_http_protocol_options__,
                    http_protocol_options: http_protocol_options__,
                    http2_protocol_options: http2_protocol_options__,
                    extension_protocol_options: extension_protocol_options__.unwrap_or_default(),
                    typed_extension_protocol_options: typed_extension_protocol_options__.unwrap_or_default(),
                    dns_refresh_rate: dns_refresh_rate__,
                    dns_failure_refresh_rate: dns_failure_refresh_rate__,
                    respect_dns_ttl: respect_dns_ttl__.unwrap_or_default(),
                    dns_lookup_family: dns_lookup_family__.unwrap_or_default(),
                    dns_resolvers: dns_resolvers__.unwrap_or_default(),
                    use_tcp_for_dns_lookups: use_tcp_for_dns_lookups__.unwrap_or_default(),
                    outlier_detection: outlier_detection__,
                    cleanup_interval: cleanup_interval__,
                    upstream_bind_config: upstream_bind_config__,
                    lb_subset_config: lb_subset_config__,
                    common_lb_config: common_lb_config__,
                    transport_socket: transport_socket__,
                    metadata: metadata__,
                    protocol_selection: protocol_selection__.unwrap_or_default(),
                    upstream_connection_options: upstream_connection_options__,
                    close_connections_on_host_health_failure: close_connections_on_host_health_failure__.unwrap_or_default(),
                    drain_connections_on_host_removal: drain_connections_on_host_removal__.unwrap_or_default(),
                    filters: filters__.unwrap_or_default(),
                    load_balancing_policy: load_balancing_policy__,
                    lrs_server: lrs_server__,
                    track_timeout_budgets: track_timeout_budgets__.unwrap_or_default(),
                    cluster_discovery_type: cluster_discovery_type__,
                    lb_config: lb_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::ClusterProtocolSelection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UseConfiguredProtocol => "USE_CONFIGURED_PROTOCOL",
            Self::UseDownstreamProtocol => "USE_DOWNSTREAM_PROTOCOL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::ClusterProtocolSelection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USE_CONFIGURED_PROTOCOL",
            "USE_DOWNSTREAM_PROTOCOL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::ClusterProtocolSelection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::ClusterProtocolSelection::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::ClusterProtocolSelection::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "USE_CONFIGURED_PROTOCOL" => Ok(cluster::ClusterProtocolSelection::UseConfiguredProtocol),
                    "USE_DOWNSTREAM_PROTOCOL" => Ok(cluster::ClusterProtocolSelection::UseDownstreamProtocol),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::CommonLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.healthy_panic_threshold.is_some() {
            len += 1;
        }
        if self.update_merge_window.is_some() {
            len += 1;
        }
        if self.ignore_new_hosts_until_first_hc {
            len += 1;
        }
        if self.close_connections_on_host_set_change {
            len += 1;
        }
        if self.consistent_hashing_lb_config.is_some() {
            len += 1;
        }
        if self.locality_config_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.CommonLbConfig", len)?;
        if let Some(v) = self.healthy_panic_threshold.as_ref() {
            struct_ser.serialize_field("healthyPanicThreshold", v)?;
        }
        if let Some(v) = self.update_merge_window.as_ref() {
            struct_ser.serialize_field("updateMergeWindow", v)?;
        }
        if self.ignore_new_hosts_until_first_hc {
            struct_ser.serialize_field("ignoreNewHostsUntilFirstHc", &self.ignore_new_hosts_until_first_hc)?;
        }
        if self.close_connections_on_host_set_change {
            struct_ser.serialize_field("closeConnectionsOnHostSetChange", &self.close_connections_on_host_set_change)?;
        }
        if let Some(v) = self.consistent_hashing_lb_config.as_ref() {
            struct_ser.serialize_field("consistentHashingLbConfig", v)?;
        }
        if let Some(v) = self.locality_config_specifier.as_ref() {
            match v {
                cluster::common_lb_config::LocalityConfigSpecifier::ZoneAwareLbConfig(v) => {
                    struct_ser.serialize_field("zoneAwareLbConfig", v)?;
                }
                cluster::common_lb_config::LocalityConfigSpecifier::LocalityWeightedLbConfig(v) => {
                    struct_ser.serialize_field("localityWeightedLbConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::CommonLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "healthy_panic_threshold",
            "healthyPanicThreshold",
            "update_merge_window",
            "updateMergeWindow",
            "ignore_new_hosts_until_first_hc",
            "ignoreNewHostsUntilFirstHc",
            "close_connections_on_host_set_change",
            "closeConnectionsOnHostSetChange",
            "consistent_hashing_lb_config",
            "consistentHashingLbConfig",
            "zone_aware_lb_config",
            "zoneAwareLbConfig",
            "locality_weighted_lb_config",
            "localityWeightedLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthyPanicThreshold,
            UpdateMergeWindow,
            IgnoreNewHostsUntilFirstHc,
            CloseConnectionsOnHostSetChange,
            ConsistentHashingLbConfig,
            ZoneAwareLbConfig,
            LocalityWeightedLbConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "healthyPanicThreshold" | "healthy_panic_threshold" => Ok(GeneratedField::HealthyPanicThreshold),
                            "updateMergeWindow" | "update_merge_window" => Ok(GeneratedField::UpdateMergeWindow),
                            "ignoreNewHostsUntilFirstHc" | "ignore_new_hosts_until_first_hc" => Ok(GeneratedField::IgnoreNewHostsUntilFirstHc),
                            "closeConnectionsOnHostSetChange" | "close_connections_on_host_set_change" => Ok(GeneratedField::CloseConnectionsOnHostSetChange),
                            "consistentHashingLbConfig" | "consistent_hashing_lb_config" => Ok(GeneratedField::ConsistentHashingLbConfig),
                            "zoneAwareLbConfig" | "zone_aware_lb_config" => Ok(GeneratedField::ZoneAwareLbConfig),
                            "localityWeightedLbConfig" | "locality_weighted_lb_config" => Ok(GeneratedField::LocalityWeightedLbConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::CommonLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.CommonLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::CommonLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut healthy_panic_threshold__ = None;
                let mut update_merge_window__ = None;
                let mut ignore_new_hosts_until_first_hc__ = None;
                let mut close_connections_on_host_set_change__ = None;
                let mut consistent_hashing_lb_config__ = None;
                let mut locality_config_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HealthyPanicThreshold => {
                            if healthy_panic_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthyPanicThreshold"));
                            }
                            healthy_panic_threshold__ = map.next_value()?;
                        }
                        GeneratedField::UpdateMergeWindow => {
                            if update_merge_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateMergeWindow"));
                            }
                            update_merge_window__ = map.next_value()?;
                        }
                        GeneratedField::IgnoreNewHostsUntilFirstHc => {
                            if ignore_new_hosts_until_first_hc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreNewHostsUntilFirstHc"));
                            }
                            ignore_new_hosts_until_first_hc__ = Some(map.next_value()?);
                        }
                        GeneratedField::CloseConnectionsOnHostSetChange => {
                            if close_connections_on_host_set_change__.is_some() {
                                return Err(serde::de::Error::duplicate_field("closeConnectionsOnHostSetChange"));
                            }
                            close_connections_on_host_set_change__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConsistentHashingLbConfig => {
                            if consistent_hashing_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistentHashingLbConfig"));
                            }
                            consistent_hashing_lb_config__ = map.next_value()?;
                        }
                        GeneratedField::ZoneAwareLbConfig => {
                            if locality_config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneAwareLbConfig"));
                            }
                            locality_config_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(cluster::common_lb_config::LocalityConfigSpecifier::ZoneAwareLbConfig)
;
                        }
                        GeneratedField::LocalityWeightedLbConfig => {
                            if locality_config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityWeightedLbConfig"));
                            }
                            locality_config_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(cluster::common_lb_config::LocalityConfigSpecifier::LocalityWeightedLbConfig)
;
                        }
                    }
                }
                Ok(cluster::CommonLbConfig {
                    healthy_panic_threshold: healthy_panic_threshold__,
                    update_merge_window: update_merge_window__,
                    ignore_new_hosts_until_first_hc: ignore_new_hosts_until_first_hc__.unwrap_or_default(),
                    close_connections_on_host_set_change: close_connections_on_host_set_change__.unwrap_or_default(),
                    consistent_hashing_lb_config: consistent_hashing_lb_config__,
                    locality_config_specifier: locality_config_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.CommonLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::common_lb_config::ConsistentHashingLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_hostname_for_hashing {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.CommonLbConfig.ConsistentHashingLbConfig", len)?;
        if self.use_hostname_for_hashing {
            struct_ser.serialize_field("useHostnameForHashing", &self.use_hostname_for_hashing)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::common_lb_config::ConsistentHashingLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_hostname_for_hashing",
            "useHostnameForHashing",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseHostnameForHashing,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "useHostnameForHashing" | "use_hostname_for_hashing" => Ok(GeneratedField::UseHostnameForHashing),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::common_lb_config::ConsistentHashingLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.CommonLbConfig.ConsistentHashingLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::common_lb_config::ConsistentHashingLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_hostname_for_hashing__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UseHostnameForHashing => {
                            if use_hostname_for_hashing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useHostnameForHashing"));
                            }
                            use_hostname_for_hashing__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cluster::common_lb_config::ConsistentHashingLbConfig {
                    use_hostname_for_hashing: use_hostname_for_hashing__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.CommonLbConfig.ConsistentHashingLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::common_lb_config::LocalityWeightedLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.CommonLbConfig.LocalityWeightedLbConfig", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::common_lb_config::LocalityWeightedLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::common_lb_config::LocalityWeightedLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.CommonLbConfig.LocalityWeightedLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::common_lb_config::LocalityWeightedLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(cluster::common_lb_config::LocalityWeightedLbConfig {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.CommonLbConfig.LocalityWeightedLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::common_lb_config::ZoneAwareLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.routing_enabled.is_some() {
            len += 1;
        }
        if self.min_cluster_size.is_some() {
            len += 1;
        }
        if self.fail_traffic_on_panic {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.CommonLbConfig.ZoneAwareLbConfig", len)?;
        if let Some(v) = self.routing_enabled.as_ref() {
            struct_ser.serialize_field("routingEnabled", v)?;
        }
        if let Some(v) = self.min_cluster_size.as_ref() {
            struct_ser.serialize_field("minClusterSize", v)?;
        }
        if self.fail_traffic_on_panic {
            struct_ser.serialize_field("failTrafficOnPanic", &self.fail_traffic_on_panic)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::common_lb_config::ZoneAwareLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routing_enabled",
            "routingEnabled",
            "min_cluster_size",
            "minClusterSize",
            "fail_traffic_on_panic",
            "failTrafficOnPanic",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoutingEnabled,
            MinClusterSize,
            FailTrafficOnPanic,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "routingEnabled" | "routing_enabled" => Ok(GeneratedField::RoutingEnabled),
                            "minClusterSize" | "min_cluster_size" => Ok(GeneratedField::MinClusterSize),
                            "failTrafficOnPanic" | "fail_traffic_on_panic" => Ok(GeneratedField::FailTrafficOnPanic),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::common_lb_config::ZoneAwareLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.CommonLbConfig.ZoneAwareLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::common_lb_config::ZoneAwareLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut routing_enabled__ = None;
                let mut min_cluster_size__ = None;
                let mut fail_traffic_on_panic__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RoutingEnabled => {
                            if routing_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingEnabled"));
                            }
                            routing_enabled__ = map.next_value()?;
                        }
                        GeneratedField::MinClusterSize => {
                            if min_cluster_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minClusterSize"));
                            }
                            min_cluster_size__ = map.next_value()?;
                        }
                        GeneratedField::FailTrafficOnPanic => {
                            if fail_traffic_on_panic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failTrafficOnPanic"));
                            }
                            fail_traffic_on_panic__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cluster::common_lb_config::ZoneAwareLbConfig {
                    routing_enabled: routing_enabled__,
                    min_cluster_size: min_cluster_size__,
                    fail_traffic_on_panic: fail_traffic_on_panic__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.CommonLbConfig.ZoneAwareLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::CustomClusterType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.CustomClusterType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typedConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::CustomClusterType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypedConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::CustomClusterType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.CustomClusterType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::CustomClusterType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut typed_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(cluster::CustomClusterType {
                    name: name__.unwrap_or_default(),
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.CustomClusterType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::DiscoveryType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Static => "STATIC",
            Self::StrictDns => "STRICT_DNS",
            Self::LogicalDns => "LOGICAL_DNS",
            Self::Eds => "EDS",
            Self::OriginalDst => "ORIGINAL_DST",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::DiscoveryType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATIC",
            "STRICT_DNS",
            "LOGICAL_DNS",
            "EDS",
            "ORIGINAL_DST",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::DiscoveryType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::DiscoveryType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::DiscoveryType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATIC" => Ok(cluster::DiscoveryType::Static),
                    "STRICT_DNS" => Ok(cluster::DiscoveryType::StrictDns),
                    "LOGICAL_DNS" => Ok(cluster::DiscoveryType::LogicalDns),
                    "EDS" => Ok(cluster::DiscoveryType::Eds),
                    "ORIGINAL_DST" => Ok(cluster::DiscoveryType::OriginalDst),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::DnsLookupFamily {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Auto => "AUTO",
            Self::V4Only => "V4_ONLY",
            Self::V6Only => "V6_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::DnsLookupFamily {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO",
            "V4_ONLY",
            "V6_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::DnsLookupFamily;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::DnsLookupFamily::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::DnsLookupFamily::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AUTO" => Ok(cluster::DnsLookupFamily::Auto),
                    "V4_ONLY" => Ok(cluster::DnsLookupFamily::V4Only),
                    "V6_ONLY" => Ok(cluster::DnsLookupFamily::V6Only),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::EdsClusterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.eds_config.is_some() {
            len += 1;
        }
        if !self.service_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.EdsClusterConfig", len)?;
        if let Some(v) = self.eds_config.as_ref() {
            struct_ser.serialize_field("edsConfig", v)?;
        }
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::EdsClusterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "eds_config",
            "edsConfig",
            "service_name",
            "serviceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EdsConfig,
            ServiceName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "edsConfig" | "eds_config" => Ok(GeneratedField::EdsConfig),
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::EdsClusterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.EdsClusterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::EdsClusterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut eds_config__ = None;
                let mut service_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EdsConfig => {
                            if eds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edsConfig"));
                            }
                            eds_config__ = map.next_value()?;
                        }
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cluster::EdsClusterConfig {
                    eds_config: eds_config__,
                    service_name: service_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.EdsClusterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::LbPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::RoundRobin => "ROUND_ROBIN",
            Self::LeastRequest => "LEAST_REQUEST",
            Self::RingHash => "RING_HASH",
            Self::Random => "RANDOM",
            Self::OriginalDstLb => "ORIGINAL_DST_LB",
            Self::Maglev => "MAGLEV",
            Self::ClusterProvided => "CLUSTER_PROVIDED",
            Self::LoadBalancingPolicyConfig => "LOAD_BALANCING_POLICY_CONFIG",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::LbPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ROUND_ROBIN",
            "LEAST_REQUEST",
            "RING_HASH",
            "RANDOM",
            "ORIGINAL_DST_LB",
            "MAGLEV",
            "CLUSTER_PROVIDED",
            "LOAD_BALANCING_POLICY_CONFIG",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::LbPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::LbPolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::LbPolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ROUND_ROBIN" => Ok(cluster::LbPolicy::RoundRobin),
                    "LEAST_REQUEST" => Ok(cluster::LbPolicy::LeastRequest),
                    "RING_HASH" => Ok(cluster::LbPolicy::RingHash),
                    "RANDOM" => Ok(cluster::LbPolicy::Random),
                    "ORIGINAL_DST_LB" => Ok(cluster::LbPolicy::OriginalDstLb),
                    "MAGLEV" => Ok(cluster::LbPolicy::Maglev),
                    "CLUSTER_PROVIDED" => Ok(cluster::LbPolicy::ClusterProvided),
                    "LOAD_BALANCING_POLICY_CONFIG" => Ok(cluster::LbPolicy::LoadBalancingPolicyConfig),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::LbSubsetConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fallback_policy != 0 {
            len += 1;
        }
        if self.default_subset.is_some() {
            len += 1;
        }
        if !self.subset_selectors.is_empty() {
            len += 1;
        }
        if self.locality_weight_aware {
            len += 1;
        }
        if self.scale_locality_weight {
            len += 1;
        }
        if self.panic_mode_any {
            len += 1;
        }
        if self.list_as_any {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.LbSubsetConfig", len)?;
        if self.fallback_policy != 0 {
            let v = cluster::lb_subset_config::LbSubsetFallbackPolicy::from_i32(self.fallback_policy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.fallback_policy)))?;
            struct_ser.serialize_field("fallbackPolicy", &v)?;
        }
        if let Some(v) = self.default_subset.as_ref() {
            struct_ser.serialize_field("defaultSubset", v)?;
        }
        if !self.subset_selectors.is_empty() {
            struct_ser.serialize_field("subsetSelectors", &self.subset_selectors)?;
        }
        if self.locality_weight_aware {
            struct_ser.serialize_field("localityWeightAware", &self.locality_weight_aware)?;
        }
        if self.scale_locality_weight {
            struct_ser.serialize_field("scaleLocalityWeight", &self.scale_locality_weight)?;
        }
        if self.panic_mode_any {
            struct_ser.serialize_field("panicModeAny", &self.panic_mode_any)?;
        }
        if self.list_as_any {
            struct_ser.serialize_field("listAsAny", &self.list_as_any)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::LbSubsetConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fallback_policy",
            "fallbackPolicy",
            "default_subset",
            "defaultSubset",
            "subset_selectors",
            "subsetSelectors",
            "locality_weight_aware",
            "localityWeightAware",
            "scale_locality_weight",
            "scaleLocalityWeight",
            "panic_mode_any",
            "panicModeAny",
            "list_as_any",
            "listAsAny",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FallbackPolicy,
            DefaultSubset,
            SubsetSelectors,
            LocalityWeightAware,
            ScaleLocalityWeight,
            PanicModeAny,
            ListAsAny,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fallbackPolicy" | "fallback_policy" => Ok(GeneratedField::FallbackPolicy),
                            "defaultSubset" | "default_subset" => Ok(GeneratedField::DefaultSubset),
                            "subsetSelectors" | "subset_selectors" => Ok(GeneratedField::SubsetSelectors),
                            "localityWeightAware" | "locality_weight_aware" => Ok(GeneratedField::LocalityWeightAware),
                            "scaleLocalityWeight" | "scale_locality_weight" => Ok(GeneratedField::ScaleLocalityWeight),
                            "panicModeAny" | "panic_mode_any" => Ok(GeneratedField::PanicModeAny),
                            "listAsAny" | "list_as_any" => Ok(GeneratedField::ListAsAny),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::LbSubsetConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.LbSubsetConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::LbSubsetConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fallback_policy__ = None;
                let mut default_subset__ = None;
                let mut subset_selectors__ = None;
                let mut locality_weight_aware__ = None;
                let mut scale_locality_weight__ = None;
                let mut panic_mode_any__ = None;
                let mut list_as_any__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FallbackPolicy => {
                            if fallback_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackPolicy"));
                            }
                            fallback_policy__ = Some(map.next_value::<cluster::lb_subset_config::LbSubsetFallbackPolicy>()? as i32);
                        }
                        GeneratedField::DefaultSubset => {
                            if default_subset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultSubset"));
                            }
                            default_subset__ = map.next_value()?;
                        }
                        GeneratedField::SubsetSelectors => {
                            if subset_selectors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subsetSelectors"));
                            }
                            subset_selectors__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalityWeightAware => {
                            if locality_weight_aware__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityWeightAware"));
                            }
                            locality_weight_aware__ = Some(map.next_value()?);
                        }
                        GeneratedField::ScaleLocalityWeight => {
                            if scale_locality_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scaleLocalityWeight"));
                            }
                            scale_locality_weight__ = Some(map.next_value()?);
                        }
                        GeneratedField::PanicModeAny => {
                            if panic_mode_any__.is_some() {
                                return Err(serde::de::Error::duplicate_field("panicModeAny"));
                            }
                            panic_mode_any__ = Some(map.next_value()?);
                        }
                        GeneratedField::ListAsAny => {
                            if list_as_any__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listAsAny"));
                            }
                            list_as_any__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cluster::LbSubsetConfig {
                    fallback_policy: fallback_policy__.unwrap_or_default(),
                    default_subset: default_subset__,
                    subset_selectors: subset_selectors__.unwrap_or_default(),
                    locality_weight_aware: locality_weight_aware__.unwrap_or_default(),
                    scale_locality_weight: scale_locality_weight__.unwrap_or_default(),
                    panic_mode_any: panic_mode_any__.unwrap_or_default(),
                    list_as_any: list_as_any__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.LbSubsetConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::lb_subset_config::LbSubsetFallbackPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NoFallback => "NO_FALLBACK",
            Self::AnyEndpoint => "ANY_ENDPOINT",
            Self::DefaultSubset => "DEFAULT_SUBSET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::lb_subset_config::LbSubsetFallbackPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NO_FALLBACK",
            "ANY_ENDPOINT",
            "DEFAULT_SUBSET",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::lb_subset_config::LbSubsetFallbackPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::lb_subset_config::LbSubsetFallbackPolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::lb_subset_config::LbSubsetFallbackPolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NO_FALLBACK" => Ok(cluster::lb_subset_config::LbSubsetFallbackPolicy::NoFallback),
                    "ANY_ENDPOINT" => Ok(cluster::lb_subset_config::LbSubsetFallbackPolicy::AnyEndpoint),
                    "DEFAULT_SUBSET" => Ok(cluster::lb_subset_config::LbSubsetFallbackPolicy::DefaultSubset),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::lb_subset_config::LbSubsetSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        if self.fallback_policy != 0 {
            len += 1;
        }
        if !self.fallback_keys_subset.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.LbSubsetConfig.LbSubsetSelector", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if self.fallback_policy != 0 {
            let v = cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::from_i32(self.fallback_policy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.fallback_policy)))?;
            struct_ser.serialize_field("fallbackPolicy", &v)?;
        }
        if !self.fallback_keys_subset.is_empty() {
            struct_ser.serialize_field("fallbackKeysSubset", &self.fallback_keys_subset)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::lb_subset_config::LbSubsetSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
            "fallback_policy",
            "fallbackPolicy",
            "fallback_keys_subset",
            "fallbackKeysSubset",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
            FallbackPolicy,
            FallbackKeysSubset,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "keys" => Ok(GeneratedField::Keys),
                            "fallbackPolicy" | "fallback_policy" => Ok(GeneratedField::FallbackPolicy),
                            "fallbackKeysSubset" | "fallback_keys_subset" => Ok(GeneratedField::FallbackKeysSubset),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::lb_subset_config::LbSubsetSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.LbSubsetConfig.LbSubsetSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::lb_subset_config::LbSubsetSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                let mut fallback_policy__ = None;
                let mut fallback_keys_subset__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map.next_value()?);
                        }
                        GeneratedField::FallbackPolicy => {
                            if fallback_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackPolicy"));
                            }
                            fallback_policy__ = Some(map.next_value::<cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy>()? as i32);
                        }
                        GeneratedField::FallbackKeysSubset => {
                            if fallback_keys_subset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackKeysSubset"));
                            }
                            fallback_keys_subset__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cluster::lb_subset_config::LbSubsetSelector {
                    keys: keys__.unwrap_or_default(),
                    fallback_policy: fallback_policy__.unwrap_or_default(),
                    fallback_keys_subset: fallback_keys_subset__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.LbSubsetConfig.LbSubsetSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::NotDefined => "NOT_DEFINED",
            Self::NoFallback => "NO_FALLBACK",
            Self::AnyEndpoint => "ANY_ENDPOINT",
            Self::DefaultSubset => "DEFAULT_SUBSET",
            Self::KeysSubset => "KEYS_SUBSET",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NOT_DEFINED",
            "NO_FALLBACK",
            "ANY_ENDPOINT",
            "DEFAULT_SUBSET",
            "KEYS_SUBSET",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NOT_DEFINED" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::NotDefined),
                    "NO_FALLBACK" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::NoFallback),
                    "ANY_ENDPOINT" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::AnyEndpoint),
                    "DEFAULT_SUBSET" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::DefaultSubset),
                    "KEYS_SUBSET" => Ok(cluster::lb_subset_config::lb_subset_selector::LbSubsetSelectorFallbackPolicy::KeysSubset),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::LeastRequestLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.choice_count.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.LeastRequestLbConfig", len)?;
        if let Some(v) = self.choice_count.as_ref() {
            struct_ser.serialize_field("choiceCount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::LeastRequestLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "choice_count",
            "choiceCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChoiceCount,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "choiceCount" | "choice_count" => Ok(GeneratedField::ChoiceCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::LeastRequestLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.LeastRequestLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::LeastRequestLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut choice_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChoiceCount => {
                            if choice_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("choiceCount"));
                            }
                            choice_count__ = map.next_value()?;
                        }
                    }
                }
                Ok(cluster::LeastRequestLbConfig {
                    choice_count: choice_count__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.LeastRequestLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::OriginalDstLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_http_header {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.OriginalDstLbConfig", len)?;
        if self.use_http_header {
            struct_ser.serialize_field("useHttpHeader", &self.use_http_header)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::OriginalDstLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_http_header",
            "useHttpHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseHttpHeader,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "useHttpHeader" | "use_http_header" => Ok(GeneratedField::UseHttpHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::OriginalDstLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.OriginalDstLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::OriginalDstLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_http_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UseHttpHeader => {
                            if use_http_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useHttpHeader"));
                            }
                            use_http_header__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cluster::OriginalDstLbConfig {
                    use_http_header: use_http_header__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.OriginalDstLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::RefreshRate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_interval.is_some() {
            len += 1;
        }
        if self.max_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.RefreshRate", len)?;
        if let Some(v) = self.base_interval.as_ref() {
            struct_ser.serialize_field("baseInterval", v)?;
        }
        if let Some(v) = self.max_interval.as_ref() {
            struct_ser.serialize_field("maxInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::RefreshRate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_interval",
            "baseInterval",
            "max_interval",
            "maxInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseInterval,
            MaxInterval,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "baseInterval" | "base_interval" => Ok(GeneratedField::BaseInterval),
                            "maxInterval" | "max_interval" => Ok(GeneratedField::MaxInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::RefreshRate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.RefreshRate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::RefreshRate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_interval__ = None;
                let mut max_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BaseInterval => {
                            if base_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseInterval"));
                            }
                            base_interval__ = map.next_value()?;
                        }
                        GeneratedField::MaxInterval => {
                            if max_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInterval"));
                            }
                            max_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(cluster::RefreshRate {
                    base_interval: base_interval__,
                    max_interval: max_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.RefreshRate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::RingHashLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.minimum_ring_size.is_some() {
            len += 1;
        }
        if self.hash_function != 0 {
            len += 1;
        }
        if self.maximum_ring_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.RingHashLbConfig", len)?;
        if let Some(v) = self.minimum_ring_size.as_ref() {
            struct_ser.serialize_field("minimumRingSize", v)?;
        }
        if self.hash_function != 0 {
            let v = cluster::ring_hash_lb_config::HashFunction::from_i32(self.hash_function)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.hash_function)))?;
            struct_ser.serialize_field("hashFunction", &v)?;
        }
        if let Some(v) = self.maximum_ring_size.as_ref() {
            struct_ser.serialize_field("maximumRingSize", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::RingHashLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimum_ring_size",
            "minimumRingSize",
            "hash_function",
            "hashFunction",
            "maximum_ring_size",
            "maximumRingSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinimumRingSize,
            HashFunction,
            MaximumRingSize,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minimumRingSize" | "minimum_ring_size" => Ok(GeneratedField::MinimumRingSize),
                            "hashFunction" | "hash_function" => Ok(GeneratedField::HashFunction),
                            "maximumRingSize" | "maximum_ring_size" => Ok(GeneratedField::MaximumRingSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::RingHashLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.RingHashLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::RingHashLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut minimum_ring_size__ = None;
                let mut hash_function__ = None;
                let mut maximum_ring_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MinimumRingSize => {
                            if minimum_ring_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumRingSize"));
                            }
                            minimum_ring_size__ = map.next_value()?;
                        }
                        GeneratedField::HashFunction => {
                            if hash_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashFunction"));
                            }
                            hash_function__ = Some(map.next_value::<cluster::ring_hash_lb_config::HashFunction>()? as i32);
                        }
                        GeneratedField::MaximumRingSize => {
                            if maximum_ring_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximumRingSize"));
                            }
                            maximum_ring_size__ = map.next_value()?;
                        }
                    }
                }
                Ok(cluster::RingHashLbConfig {
                    minimum_ring_size: minimum_ring_size__,
                    hash_function: hash_function__.unwrap_or_default(),
                    maximum_ring_size: maximum_ring_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.RingHashLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::ring_hash_lb_config::HashFunction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::XxHash => "XX_HASH",
            Self::MurmurHash2 => "MURMUR_HASH_2",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for cluster::ring_hash_lb_config::HashFunction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "XX_HASH",
            "MURMUR_HASH_2",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::ring_hash_lb_config::HashFunction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::ring_hash_lb_config::HashFunction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(cluster::ring_hash_lb_config::HashFunction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "XX_HASH" => Ok(cluster::ring_hash_lb_config::HashFunction::XxHash),
                    "MURMUR_HASH_2" => Ok(cluster::ring_hash_lb_config::HashFunction::MurmurHash2),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for cluster::TransportSocketMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.r#match.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Cluster.TransportSocketMatch", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transportSocket", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster::TransportSocketMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "match",
            "transport_socket",
            "transportSocket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Match,
            TransportSocket,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "match" => Ok(GeneratedField::Match),
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster::TransportSocketMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Cluster.TransportSocketMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster::TransportSocketMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#match__ = None;
                let mut transport_socket__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map.next_value()?;
                        }
                    }
                }
                Ok(cluster::TransportSocketMatch {
                    name: name__.unwrap_or_default(),
                    r#match: r#match__,
                    transport_socket: transport_socket__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Cluster.TransportSocketMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterLoadAssignment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if !self.endpoints.is_empty() {
            len += 1;
        }
        if !self.named_endpoints.is_empty() {
            len += 1;
        }
        if self.policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.ClusterLoadAssignment", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("clusterName", &self.cluster_name)?;
        }
        if !self.endpoints.is_empty() {
            struct_ser.serialize_field("endpoints", &self.endpoints)?;
        }
        if !self.named_endpoints.is_empty() {
            struct_ser.serialize_field("namedEndpoints", &self.named_endpoints)?;
        }
        if let Some(v) = self.policy.as_ref() {
            struct_ser.serialize_field("policy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterLoadAssignment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "endpoints",
            "named_endpoints",
            "namedEndpoints",
            "policy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            Endpoints,
            NamedEndpoints,
            Policy,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            "namedEndpoints" | "named_endpoints" => Ok(GeneratedField::NamedEndpoints),
                            "policy" => Ok(GeneratedField::Policy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterLoadAssignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.ClusterLoadAssignment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterLoadAssignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut endpoints__ = None;
                let mut named_endpoints__ = None;
                let mut policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = Some(map.next_value()?);
                        }
                        GeneratedField::NamedEndpoints => {
                            if named_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namedEndpoints"));
                            }
                            named_endpoints__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Policy => {
                            if policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            policy__ = map.next_value()?;
                        }
                    }
                }
                Ok(ClusterLoadAssignment {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    endpoints: endpoints__.unwrap_or_default(),
                    named_endpoints: named_endpoints__.unwrap_or_default(),
                    policy: policy__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.ClusterLoadAssignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_load_assignment::Policy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.drop_overloads.is_empty() {
            len += 1;
        }
        if self.overprovisioning_factor.is_some() {
            len += 1;
        }
        if self.endpoint_stale_after.is_some() {
            len += 1;
        }
        if self.disable_overprovisioning {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.ClusterLoadAssignment.Policy", len)?;
        if !self.drop_overloads.is_empty() {
            struct_ser.serialize_field("dropOverloads", &self.drop_overloads)?;
        }
        if let Some(v) = self.overprovisioning_factor.as_ref() {
            struct_ser.serialize_field("overprovisioningFactor", v)?;
        }
        if let Some(v) = self.endpoint_stale_after.as_ref() {
            struct_ser.serialize_field("endpointStaleAfter", v)?;
        }
        if self.disable_overprovisioning {
            struct_ser.serialize_field("disableOverprovisioning", &self.disable_overprovisioning)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_load_assignment::Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "drop_overloads",
            "dropOverloads",
            "overprovisioning_factor",
            "overprovisioningFactor",
            "endpoint_stale_after",
            "endpointStaleAfter",
            "disable_overprovisioning",
            "disableOverprovisioning",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DropOverloads,
            OverprovisioningFactor,
            EndpointStaleAfter,
            DisableOverprovisioning,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "dropOverloads" | "drop_overloads" => Ok(GeneratedField::DropOverloads),
                            "overprovisioningFactor" | "overprovisioning_factor" => Ok(GeneratedField::OverprovisioningFactor),
                            "endpointStaleAfter" | "endpoint_stale_after" => Ok(GeneratedField::EndpointStaleAfter),
                            "disableOverprovisioning" | "disable_overprovisioning" => Ok(GeneratedField::DisableOverprovisioning),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_load_assignment::Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.ClusterLoadAssignment.Policy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster_load_assignment::Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut drop_overloads__ = None;
                let mut overprovisioning_factor__ = None;
                let mut endpoint_stale_after__ = None;
                let mut disable_overprovisioning__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DropOverloads => {
                            if drop_overloads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dropOverloads"));
                            }
                            drop_overloads__ = Some(map.next_value()?);
                        }
                        GeneratedField::OverprovisioningFactor => {
                            if overprovisioning_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overprovisioningFactor"));
                            }
                            overprovisioning_factor__ = map.next_value()?;
                        }
                        GeneratedField::EndpointStaleAfter => {
                            if endpoint_stale_after__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointStaleAfter"));
                            }
                            endpoint_stale_after__ = map.next_value()?;
                        }
                        GeneratedField::DisableOverprovisioning => {
                            if disable_overprovisioning__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableOverprovisioning"));
                            }
                            disable_overprovisioning__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cluster_load_assignment::Policy {
                    drop_overloads: drop_overloads__.unwrap_or_default(),
                    overprovisioning_factor: overprovisioning_factor__,
                    endpoint_stale_after: endpoint_stale_after__,
                    disable_overprovisioning: disable_overprovisioning__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.ClusterLoadAssignment.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_load_assignment::policy::DropOverload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.category.is_empty() {
            len += 1;
        }
        if self.drop_percentage.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.ClusterLoadAssignment.Policy.DropOverload", len)?;
        if !self.category.is_empty() {
            struct_ser.serialize_field("category", &self.category)?;
        }
        if let Some(v) = self.drop_percentage.as_ref() {
            struct_ser.serialize_field("dropPercentage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_load_assignment::policy::DropOverload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
            "drop_percentage",
            "dropPercentage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
            DropPercentage,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "category" => Ok(GeneratedField::Category),
                            "dropPercentage" | "drop_percentage" => Ok(GeneratedField::DropPercentage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_load_assignment::policy::DropOverload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.ClusterLoadAssignment.Policy.DropOverload")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster_load_assignment::policy::DropOverload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                let mut drop_percentage__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map.next_value()?);
                        }
                        GeneratedField::DropPercentage => {
                            if drop_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dropPercentage"));
                            }
                            drop_percentage__ = map.next_value()?;
                        }
                    }
                }
                Ok(cluster_load_assignment::policy::DropOverload {
                    category: category__.unwrap_or_default(),
                    drop_percentage: drop_percentage__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.ClusterLoadAssignment.Policy.DropOverload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaDiscoveryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node.is_some() {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.resource_names_subscribe.is_empty() {
            len += 1;
        }
        if !self.resource_names_unsubscribe.is_empty() {
            len += 1;
        }
        if !self.initial_resource_versions.is_empty() {
            len += 1;
        }
        if !self.response_nonce.is_empty() {
            len += 1;
        }
        if self.error_detail.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.DeltaDiscoveryRequest", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        if !self.resource_names_subscribe.is_empty() {
            struct_ser.serialize_field("resourceNamesSubscribe", &self.resource_names_subscribe)?;
        }
        if !self.resource_names_unsubscribe.is_empty() {
            struct_ser.serialize_field("resourceNamesUnsubscribe", &self.resource_names_unsubscribe)?;
        }
        if !self.initial_resource_versions.is_empty() {
            struct_ser.serialize_field("initialResourceVersions", &self.initial_resource_versions)?;
        }
        if !self.response_nonce.is_empty() {
            struct_ser.serialize_field("responseNonce", &self.response_nonce)?;
        }
        if let Some(v) = self.error_detail.as_ref() {
            struct_ser.serialize_field("errorDetail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaDiscoveryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "type_url",
            "typeUrl",
            "resource_names_subscribe",
            "resourceNamesSubscribe",
            "resource_names_unsubscribe",
            "resourceNamesUnsubscribe",
            "initial_resource_versions",
            "initialResourceVersions",
            "response_nonce",
            "responseNonce",
            "error_detail",
            "errorDetail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            TypeUrl,
            ResourceNamesSubscribe,
            ResourceNamesUnsubscribe,
            InitialResourceVersions,
            ResponseNonce,
            ErrorDetail,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "node" => Ok(GeneratedField::Node),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "resourceNamesSubscribe" | "resource_names_subscribe" => Ok(GeneratedField::ResourceNamesSubscribe),
                            "resourceNamesUnsubscribe" | "resource_names_unsubscribe" => Ok(GeneratedField::ResourceNamesUnsubscribe),
                            "initialResourceVersions" | "initial_resource_versions" => Ok(GeneratedField::InitialResourceVersions),
                            "responseNonce" | "response_nonce" => Ok(GeneratedField::ResponseNonce),
                            "errorDetail" | "error_detail" => Ok(GeneratedField::ErrorDetail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaDiscoveryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.DeltaDiscoveryRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeltaDiscoveryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut type_url__ = None;
                let mut resource_names_subscribe__ = None;
                let mut resource_names_unsubscribe__ = None;
                let mut initial_resource_versions__ = None;
                let mut response_nonce__ = None;
                let mut error_detail__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceNamesSubscribe => {
                            if resource_names_subscribe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNamesSubscribe"));
                            }
                            resource_names_subscribe__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResourceNamesUnsubscribe => {
                            if resource_names_unsubscribe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNamesUnsubscribe"));
                            }
                            resource_names_unsubscribe__ = Some(map.next_value()?);
                        }
                        GeneratedField::InitialResourceVersions => {
                            if initial_resource_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialResourceVersions"));
                            }
                            initial_resource_versions__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ResponseNonce => {
                            if response_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseNonce"));
                            }
                            response_nonce__ = Some(map.next_value()?);
                        }
                        GeneratedField::ErrorDetail => {
                            if error_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorDetail"));
                            }
                            error_detail__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeltaDiscoveryRequest {
                    node: node__,
                    type_url: type_url__.unwrap_or_default(),
                    resource_names_subscribe: resource_names_subscribe__.unwrap_or_default(),
                    resource_names_unsubscribe: resource_names_unsubscribe__.unwrap_or_default(),
                    initial_resource_versions: initial_resource_versions__.unwrap_or_default(),
                    response_nonce: response_nonce__.unwrap_or_default(),
                    error_detail: error_detail__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.DeltaDiscoveryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeltaDiscoveryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.system_version_info.is_empty() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.removed_resources.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.DeltaDiscoveryResponse", len)?;
        if !self.system_version_info.is_empty() {
            struct_ser.serialize_field("systemVersionInfo", &self.system_version_info)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        if !self.removed_resources.is_empty() {
            struct_ser.serialize_field("removedResources", &self.removed_resources)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeltaDiscoveryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "system_version_info",
            "systemVersionInfo",
            "resources",
            "type_url",
            "typeUrl",
            "removed_resources",
            "removedResources",
            "nonce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SystemVersionInfo,
            Resources,
            TypeUrl,
            RemovedResources,
            Nonce,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "systemVersionInfo" | "system_version_info" => Ok(GeneratedField::SystemVersionInfo),
                            "resources" => Ok(GeneratedField::Resources),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "removedResources" | "removed_resources" => Ok(GeneratedField::RemovedResources),
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeltaDiscoveryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.DeltaDiscoveryResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeltaDiscoveryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut system_version_info__ = None;
                let mut resources__ = None;
                let mut type_url__ = None;
                let mut removed_resources__ = None;
                let mut nonce__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SystemVersionInfo => {
                            if system_version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemVersionInfo"));
                            }
                            system_version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemovedResources => {
                            if removed_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removedResources"));
                            }
                            removed_resources__ = Some(map.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DeltaDiscoveryResponse {
                    system_version_info: system_version_info__.unwrap_or_default(),
                    resources: resources__.unwrap_or_default(),
                    type_url: type_url__.unwrap_or_default(),
                    removed_resources: removed_resources__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.DeltaDiscoveryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoveryRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.node.is_some() {
            len += 1;
        }
        if !self.resource_names.is_empty() {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.response_nonce.is_empty() {
            len += 1;
        }
        if self.error_detail.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.DiscoveryRequest", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.resource_names.is_empty() {
            struct_ser.serialize_field("resourceNames", &self.resource_names)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        if !self.response_nonce.is_empty() {
            struct_ser.serialize_field("responseNonce", &self.response_nonce)?;
        }
        if let Some(v) = self.error_detail.as_ref() {
            struct_ser.serialize_field("errorDetail", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "node",
            "resource_names",
            "resourceNames",
            "type_url",
            "typeUrl",
            "response_nonce",
            "responseNonce",
            "error_detail",
            "errorDetail",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Node,
            ResourceNames,
            TypeUrl,
            ResponseNonce,
            ErrorDetail,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "node" => Ok(GeneratedField::Node),
                            "resourceNames" | "resource_names" => Ok(GeneratedField::ResourceNames),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "responseNonce" | "response_nonce" => Ok(GeneratedField::ResponseNonce),
                            "errorDetail" | "error_detail" => Ok(GeneratedField::ErrorDetail),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.DiscoveryRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DiscoveryRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut node__ = None;
                let mut resource_names__ = None;
                let mut type_url__ = None;
                let mut response_nonce__ = None;
                let mut error_detail__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                        GeneratedField::ResourceNames => {
                            if resource_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNames"));
                            }
                            resource_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseNonce => {
                            if response_nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseNonce"));
                            }
                            response_nonce__ = Some(map.next_value()?);
                        }
                        GeneratedField::ErrorDetail => {
                            if error_detail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorDetail"));
                            }
                            error_detail__ = map.next_value()?;
                        }
                    }
                }
                Ok(DiscoveryRequest {
                    version_info: version_info__.unwrap_or_default(),
                    node: node__,
                    resource_names: resource_names__.unwrap_or_default(),
                    type_url: type_url__.unwrap_or_default(),
                    response_nonce: response_nonce__.unwrap_or_default(),
                    error_detail: error_detail__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.DiscoveryRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DiscoveryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.resources.is_empty() {
            len += 1;
        }
        if self.canary {
            len += 1;
        }
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if self.control_plane.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.DiscoveryResponse", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if !self.resources.is_empty() {
            struct_ser.serialize_field("resources", &self.resources)?;
        }
        if self.canary {
            struct_ser.serialize_field("canary", &self.canary)?;
        }
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        if !self.nonce.is_empty() {
            struct_ser.serialize_field("nonce", &self.nonce)?;
        }
        if let Some(v) = self.control_plane.as_ref() {
            struct_ser.serialize_field("controlPlane", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DiscoveryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "resources",
            "canary",
            "type_url",
            "typeUrl",
            "nonce",
            "control_plane",
            "controlPlane",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Resources,
            Canary,
            TypeUrl,
            Nonce,
            ControlPlane,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "resources" => Ok(GeneratedField::Resources),
                            "canary" => Ok(GeneratedField::Canary),
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "controlPlane" | "control_plane" => Ok(GeneratedField::ControlPlane),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DiscoveryResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.DiscoveryResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DiscoveryResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut resources__ = None;
                let mut canary__ = None;
                let mut type_url__ = None;
                let mut nonce__ = None;
                let mut control_plane__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = Some(map.next_value()?);
                        }
                        GeneratedField::Canary => {
                            if canary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canary"));
                            }
                            canary__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = Some(map.next_value()?);
                        }
                        GeneratedField::ControlPlane => {
                            if control_plane__.is_some() {
                                return Err(serde::de::Error::duplicate_field("controlPlane"));
                            }
                            control_plane__ = map.next_value()?;
                        }
                    }
                }
                Ok(DiscoveryResponse {
                    version_info: version_info__.unwrap_or_default(),
                    resources: resources__.unwrap_or_default(),
                    canary: canary__.unwrap_or_default(),
                    type_url: type_url__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    control_plane: control_plane__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.DiscoveryResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.EdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EdsDummy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.EdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(EdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.EdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.LdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LdsDummy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.LdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(LdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.LdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Listener {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if !self.filter_chains.is_empty() {
            len += 1;
        }
        if self.use_original_dst.is_some() {
            len += 1;
        }
        if self.per_connection_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.deprecated_v1.is_some() {
            len += 1;
        }
        if self.drain_type != 0 {
            len += 1;
        }
        if !self.listener_filters.is_empty() {
            len += 1;
        }
        if self.listener_filters_timeout.is_some() {
            len += 1;
        }
        if self.continue_on_listener_filters_timeout {
            len += 1;
        }
        if self.transparent.is_some() {
            len += 1;
        }
        if self.freebind.is_some() {
            len += 1;
        }
        if !self.socket_options.is_empty() {
            len += 1;
        }
        if self.tcp_fast_open_queue_length.is_some() {
            len += 1;
        }
        if self.traffic_direction != 0 {
            len += 1;
        }
        if self.udp_listener_config.is_some() {
            len += 1;
        }
        if self.api_listener.is_some() {
            len += 1;
        }
        if self.connection_balance_config.is_some() {
            len += 1;
        }
        if self.reuse_port {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Listener", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if !self.filter_chains.is_empty() {
            struct_ser.serialize_field("filterChains", &self.filter_chains)?;
        }
        if let Some(v) = self.use_original_dst.as_ref() {
            struct_ser.serialize_field("useOriginalDst", v)?;
        }
        if let Some(v) = self.per_connection_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("perConnectionBufferLimitBytes", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.deprecated_v1.as_ref() {
            struct_ser.serialize_field("deprecatedV1", v)?;
        }
        if self.drain_type != 0 {
            let v = listener::DrainType::from_i32(self.drain_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.drain_type)))?;
            struct_ser.serialize_field("drainType", &v)?;
        }
        if !self.listener_filters.is_empty() {
            struct_ser.serialize_field("listenerFilters", &self.listener_filters)?;
        }
        if let Some(v) = self.listener_filters_timeout.as_ref() {
            struct_ser.serialize_field("listenerFiltersTimeout", v)?;
        }
        if self.continue_on_listener_filters_timeout {
            struct_ser.serialize_field("continueOnListenerFiltersTimeout", &self.continue_on_listener_filters_timeout)?;
        }
        if let Some(v) = self.transparent.as_ref() {
            struct_ser.serialize_field("transparent", v)?;
        }
        if let Some(v) = self.freebind.as_ref() {
            struct_ser.serialize_field("freebind", v)?;
        }
        if !self.socket_options.is_empty() {
            struct_ser.serialize_field("socketOptions", &self.socket_options)?;
        }
        if let Some(v) = self.tcp_fast_open_queue_length.as_ref() {
            struct_ser.serialize_field("tcpFastOpenQueueLength", v)?;
        }
        if self.traffic_direction != 0 {
            let v = core::TrafficDirection::from_i32(self.traffic_direction)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.traffic_direction)))?;
            struct_ser.serialize_field("trafficDirection", &v)?;
        }
        if let Some(v) = self.udp_listener_config.as_ref() {
            struct_ser.serialize_field("udpListenerConfig", v)?;
        }
        if let Some(v) = self.api_listener.as_ref() {
            struct_ser.serialize_field("apiListener", v)?;
        }
        if let Some(v) = self.connection_balance_config.as_ref() {
            struct_ser.serialize_field("connectionBalanceConfig", v)?;
        }
        if self.reuse_port {
            struct_ser.serialize_field("reusePort", &self.reuse_port)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("accessLog", &self.access_log)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Listener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "address",
            "filter_chains",
            "filterChains",
            "use_original_dst",
            "useOriginalDst",
            "per_connection_buffer_limit_bytes",
            "perConnectionBufferLimitBytes",
            "metadata",
            "deprecated_v1",
            "deprecatedV1",
            "drain_type",
            "drainType",
            "listener_filters",
            "listenerFilters",
            "listener_filters_timeout",
            "listenerFiltersTimeout",
            "continue_on_listener_filters_timeout",
            "continueOnListenerFiltersTimeout",
            "transparent",
            "freebind",
            "socket_options",
            "socketOptions",
            "tcp_fast_open_queue_length",
            "tcpFastOpenQueueLength",
            "traffic_direction",
            "trafficDirection",
            "udp_listener_config",
            "udpListenerConfig",
            "api_listener",
            "apiListener",
            "connection_balance_config",
            "connectionBalanceConfig",
            "reuse_port",
            "reusePort",
            "access_log",
            "accessLog",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Address,
            FilterChains,
            UseOriginalDst,
            PerConnectionBufferLimitBytes,
            Metadata,
            DeprecatedV1,
            DrainType,
            ListenerFilters,
            ListenerFiltersTimeout,
            ContinueOnListenerFiltersTimeout,
            Transparent,
            Freebind,
            SocketOptions,
            TcpFastOpenQueueLength,
            TrafficDirection,
            UdpListenerConfig,
            ApiListener,
            ConnectionBalanceConfig,
            ReusePort,
            AccessLog,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "address" => Ok(GeneratedField::Address),
                            "filterChains" | "filter_chains" => Ok(GeneratedField::FilterChains),
                            "useOriginalDst" | "use_original_dst" => Ok(GeneratedField::UseOriginalDst),
                            "perConnectionBufferLimitBytes" | "per_connection_buffer_limit_bytes" => Ok(GeneratedField::PerConnectionBufferLimitBytes),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "deprecatedV1" | "deprecated_v1" => Ok(GeneratedField::DeprecatedV1),
                            "drainType" | "drain_type" => Ok(GeneratedField::DrainType),
                            "listenerFilters" | "listener_filters" => Ok(GeneratedField::ListenerFilters),
                            "listenerFiltersTimeout" | "listener_filters_timeout" => Ok(GeneratedField::ListenerFiltersTimeout),
                            "continueOnListenerFiltersTimeout" | "continue_on_listener_filters_timeout" => Ok(GeneratedField::ContinueOnListenerFiltersTimeout),
                            "transparent" => Ok(GeneratedField::Transparent),
                            "freebind" => Ok(GeneratedField::Freebind),
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            "tcpFastOpenQueueLength" | "tcp_fast_open_queue_length" => Ok(GeneratedField::TcpFastOpenQueueLength),
                            "trafficDirection" | "traffic_direction" => Ok(GeneratedField::TrafficDirection),
                            "udpListenerConfig" | "udp_listener_config" => Ok(GeneratedField::UdpListenerConfig),
                            "apiListener" | "api_listener" => Ok(GeneratedField::ApiListener),
                            "connectionBalanceConfig" | "connection_balance_config" => Ok(GeneratedField::ConnectionBalanceConfig),
                            "reusePort" | "reuse_port" => Ok(GeneratedField::ReusePort),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Listener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Listener")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Listener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut address__ = None;
                let mut filter_chains__ = None;
                let mut use_original_dst__ = None;
                let mut per_connection_buffer_limit_bytes__ = None;
                let mut metadata__ = None;
                let mut deprecated_v1__ = None;
                let mut drain_type__ = None;
                let mut listener_filters__ = None;
                let mut listener_filters_timeout__ = None;
                let mut continue_on_listener_filters_timeout__ = None;
                let mut transparent__ = None;
                let mut freebind__ = None;
                let mut socket_options__ = None;
                let mut tcp_fast_open_queue_length__ = None;
                let mut traffic_direction__ = None;
                let mut udp_listener_config__ = None;
                let mut api_listener__ = None;
                let mut connection_balance_config__ = None;
                let mut reuse_port__ = None;
                let mut access_log__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::FilterChains => {
                            if filter_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterChains"));
                            }
                            filter_chains__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseOriginalDst => {
                            if use_original_dst__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useOriginalDst"));
                            }
                            use_original_dst__ = map.next_value()?;
                        }
                        GeneratedField::PerConnectionBufferLimitBytes => {
                            if per_connection_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perConnectionBufferLimitBytes"));
                            }
                            per_connection_buffer_limit_bytes__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::DeprecatedV1 => {
                            if deprecated_v1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecatedV1"));
                            }
                            deprecated_v1__ = map.next_value()?;
                        }
                        GeneratedField::DrainType => {
                            if drain_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainType"));
                            }
                            drain_type__ = Some(map.next_value::<listener::DrainType>()? as i32);
                        }
                        GeneratedField::ListenerFilters => {
                            if listener_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listenerFilters"));
                            }
                            listener_filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::ListenerFiltersTimeout => {
                            if listener_filters_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listenerFiltersTimeout"));
                            }
                            listener_filters_timeout__ = map.next_value()?;
                        }
                        GeneratedField::ContinueOnListenerFiltersTimeout => {
                            if continue_on_listener_filters_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("continueOnListenerFiltersTimeout"));
                            }
                            continue_on_listener_filters_timeout__ = Some(map.next_value()?);
                        }
                        GeneratedField::Transparent => {
                            if transparent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transparent"));
                            }
                            transparent__ = map.next_value()?;
                        }
                        GeneratedField::Freebind => {
                            if freebind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freebind"));
                            }
                            freebind__ = map.next_value()?;
                        }
                        GeneratedField::SocketOptions => {
                            if socket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketOptions"));
                            }
                            socket_options__ = Some(map.next_value()?);
                        }
                        GeneratedField::TcpFastOpenQueueLength => {
                            if tcp_fast_open_queue_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpFastOpenQueueLength"));
                            }
                            tcp_fast_open_queue_length__ = map.next_value()?;
                        }
                        GeneratedField::TrafficDirection => {
                            if traffic_direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trafficDirection"));
                            }
                            traffic_direction__ = Some(map.next_value::<core::TrafficDirection>()? as i32);
                        }
                        GeneratedField::UdpListenerConfig => {
                            if udp_listener_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("udpListenerConfig"));
                            }
                            udp_listener_config__ = map.next_value()?;
                        }
                        GeneratedField::ApiListener => {
                            if api_listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiListener"));
                            }
                            api_listener__ = map.next_value()?;
                        }
                        GeneratedField::ConnectionBalanceConfig => {
                            if connection_balance_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionBalanceConfig"));
                            }
                            connection_balance_config__ = map.next_value()?;
                        }
                        GeneratedField::ReusePort => {
                            if reuse_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reusePort"));
                            }
                            reuse_port__ = Some(map.next_value()?);
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Listener {
                    name: name__.unwrap_or_default(),
                    address: address__,
                    filter_chains: filter_chains__.unwrap_or_default(),
                    use_original_dst: use_original_dst__,
                    per_connection_buffer_limit_bytes: per_connection_buffer_limit_bytes__,
                    metadata: metadata__,
                    deprecated_v1: deprecated_v1__,
                    drain_type: drain_type__.unwrap_or_default(),
                    listener_filters: listener_filters__.unwrap_or_default(),
                    listener_filters_timeout: listener_filters_timeout__,
                    continue_on_listener_filters_timeout: continue_on_listener_filters_timeout__.unwrap_or_default(),
                    transparent: transparent__,
                    freebind: freebind__,
                    socket_options: socket_options__.unwrap_or_default(),
                    tcp_fast_open_queue_length: tcp_fast_open_queue_length__,
                    traffic_direction: traffic_direction__.unwrap_or_default(),
                    udp_listener_config: udp_listener_config__,
                    api_listener: api_listener__,
                    connection_balance_config: connection_balance_config__,
                    reuse_port: reuse_port__.unwrap_or_default(),
                    access_log: access_log__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Listener", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::ConnectionBalanceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.balance_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Listener.ConnectionBalanceConfig", len)?;
        if let Some(v) = self.balance_type.as_ref() {
            match v {
                listener::connection_balance_config::BalanceType::ExactBalance(v) => {
                    struct_ser.serialize_field("exactBalance", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::ConnectionBalanceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exact_balance",
            "exactBalance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExactBalance,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "exactBalance" | "exact_balance" => Ok(GeneratedField::ExactBalance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::ConnectionBalanceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Listener.ConnectionBalanceConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<listener::ConnectionBalanceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balance_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExactBalance => {
                            if balance_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exactBalance"));
                            }
                            balance_type__ = map.next_value::<::std::option::Option<_>>()?.map(listener::connection_balance_config::BalanceType::ExactBalance)
;
                        }
                    }
                }
                Ok(listener::ConnectionBalanceConfig {
                    balance_type: balance_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Listener.ConnectionBalanceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::connection_balance_config::ExactBalance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.Listener.ConnectionBalanceConfig.ExactBalance", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::connection_balance_config::ExactBalance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::connection_balance_config::ExactBalance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Listener.ConnectionBalanceConfig.ExactBalance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<listener::connection_balance_config::ExactBalance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(listener::connection_balance_config::ExactBalance {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Listener.ConnectionBalanceConfig.ExactBalance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::DeprecatedV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bind_to_port.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Listener.DeprecatedV1", len)?;
        if let Some(v) = self.bind_to_port.as_ref() {
            struct_ser.serialize_field("bindToPort", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener::DeprecatedV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bind_to_port",
            "bindToPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BindToPort,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bindToPort" | "bind_to_port" => Ok(GeneratedField::BindToPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::DeprecatedV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Listener.DeprecatedV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<listener::DeprecatedV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bind_to_port__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BindToPort => {
                            if bind_to_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bindToPort"));
                            }
                            bind_to_port__ = map.next_value()?;
                        }
                    }
                }
                Ok(listener::DeprecatedV1 {
                    bind_to_port: bind_to_port__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Listener.DeprecatedV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener::DrainType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::ModifyOnly => "MODIFY_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for listener::DrainType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "MODIFY_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener::DrainType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(listener::DrainType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(listener::DrainType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(listener::DrainType::Default),
                    "MODIFY_ONLY" => Ok(listener::DrainType::ModifyOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LoadBalancingPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.policies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.LoadBalancingPolicy", len)?;
        if !self.policies.is_empty() {
            struct_ser.serialize_field("policies", &self.policies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoadBalancingPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "policies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Policies,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "policies" => Ok(GeneratedField::Policies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoadBalancingPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.LoadBalancingPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LoadBalancingPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policies__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Policies => {
                            if policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("policies"));
                            }
                            policies__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LoadBalancingPolicy {
                    policies: policies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.LoadBalancingPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for load_balancing_policy::Policy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.LoadBalancingPolicy.Policy", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typedConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for load_balancing_policy::Policy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Config,
            TypedConfig,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "config" => Ok(GeneratedField::Config),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = load_balancing_policy::Policy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.LoadBalancingPolicy.Policy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<load_balancing_policy::Policy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config__ = None;
                let mut typed_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map.next_value()?;
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(load_balancing_policy::Policy {
                    name: name__.unwrap_or_default(),
                    config: config__,
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.LoadBalancingPolicy.Policy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.RdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RdsDummy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.RdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.RdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Resource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.aliases.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if self.resource.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Resource", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.aliases.is_empty() {
            struct_ser.serialize_field("aliases", &self.aliases)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.resource.as_ref() {
            struct_ser.serialize_field("resource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Resource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "aliases",
            "version",
            "resource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Aliases,
            Version,
            Resource,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "aliases" => Ok(GeneratedField::Aliases),
                            "version" => Ok(GeneratedField::Version),
                            "resource" => Ok(GeneratedField::Resource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Resource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Resource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Resource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut aliases__ = None;
                let mut version__ = None;
                let mut resource__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Aliases => {
                            if aliases__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aliases"));
                            }
                            aliases__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = map.next_value()?;
                        }
                    }
                }
                Ok(Resource {
                    name: name__.unwrap_or_default(),
                    aliases: aliases__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    resource: resource__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Resource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.virtual_hosts.is_empty() {
            len += 1;
        }
        if self.vhds.is_some() {
            len += 1;
        }
        if !self.internal_only_headers.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if self.most_specific_header_mutations_wins {
            len += 1;
        }
        if self.validate_clusters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.RouteConfiguration", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.virtual_hosts.is_empty() {
            struct_ser.serialize_field("virtualHosts", &self.virtual_hosts)?;
        }
        if let Some(v) = self.vhds.as_ref() {
            struct_ser.serialize_field("vhds", v)?;
        }
        if !self.internal_only_headers.is_empty() {
            struct_ser.serialize_field("internalOnlyHeaders", &self.internal_only_headers)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        if !self.response_headers_to_remove.is_empty() {
            struct_ser.serialize_field("responseHeadersToRemove", &self.response_headers_to_remove)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("requestHeadersToAdd", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("requestHeadersToRemove", &self.request_headers_to_remove)?;
        }
        if self.most_specific_header_mutations_wins {
            struct_ser.serialize_field("mostSpecificHeaderMutationsWins", &self.most_specific_header_mutations_wins)?;
        }
        if let Some(v) = self.validate_clusters.as_ref() {
            struct_ser.serialize_field("validateClusters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "virtual_hosts",
            "virtualHosts",
            "vhds",
            "internal_only_headers",
            "internalOnlyHeaders",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "response_headers_to_remove",
            "responseHeadersToRemove",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "most_specific_header_mutations_wins",
            "mostSpecificHeaderMutationsWins",
            "validate_clusters",
            "validateClusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            VirtualHosts,
            Vhds,
            InternalOnlyHeaders,
            ResponseHeadersToAdd,
            ResponseHeadersToRemove,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            MostSpecificHeaderMutationsWins,
            ValidateClusters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "virtualHosts" | "virtual_hosts" => Ok(GeneratedField::VirtualHosts),
                            "vhds" => Ok(GeneratedField::Vhds),
                            "internalOnlyHeaders" | "internal_only_headers" => Ok(GeneratedField::InternalOnlyHeaders),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "responseHeadersToRemove" | "response_headers_to_remove" => Ok(GeneratedField::ResponseHeadersToRemove),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "mostSpecificHeaderMutationsWins" | "most_specific_header_mutations_wins" => Ok(GeneratedField::MostSpecificHeaderMutationsWins),
                            "validateClusters" | "validate_clusters" => Ok(GeneratedField::ValidateClusters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.RouteConfiguration")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut virtual_hosts__ = None;
                let mut vhds__ = None;
                let mut internal_only_headers__ = None;
                let mut response_headers_to_add__ = None;
                let mut response_headers_to_remove__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut most_specific_header_mutations_wins__ = None;
                let mut validate_clusters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::VirtualHosts => {
                            if virtual_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualHosts"));
                            }
                            virtual_hosts__ = Some(map.next_value()?);
                        }
                        GeneratedField::Vhds => {
                            if vhds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vhds"));
                            }
                            vhds__ = map.next_value()?;
                        }
                        GeneratedField::InternalOnlyHeaders => {
                            if internal_only_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalOnlyHeaders"));
                            }
                            internal_only_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToRemove => {
                            if response_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToRemove"));
                            }
                            response_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::MostSpecificHeaderMutationsWins => {
                            if most_specific_header_mutations_wins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mostSpecificHeaderMutationsWins"));
                            }
                            most_specific_header_mutations_wins__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidateClusters => {
                            if validate_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateClusters"));
                            }
                            validate_clusters__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteConfiguration {
                    name: name__.unwrap_or_default(),
                    virtual_hosts: virtual_hosts__.unwrap_or_default(),
                    vhds: vhds__,
                    internal_only_headers: internal_only_headers__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    response_headers_to_remove: response_headers_to_remove__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    most_specific_header_mutations_wins: most_specific_header_mutations_wins__.unwrap_or_default(),
                    validate_clusters: validate_clusters__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.RouteConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRouteConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.route_configuration_name.is_empty() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.ScopedRouteConfiguration", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.route_configuration_name.is_empty() {
            struct_ser.serialize_field("routeConfigurationName", &self.route_configuration_name)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRouteConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "route_configuration_name",
            "routeConfigurationName",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RouteConfigurationName,
            Key,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "routeConfigurationName" | "route_configuration_name" => Ok(GeneratedField::RouteConfigurationName),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRouteConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.ScopedRouteConfiguration")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScopedRouteConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut route_configuration_name__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RouteConfigurationName => {
                            if route_configuration_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfigurationName"));
                            }
                            route_configuration_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                    }
                }
                Ok(ScopedRouteConfiguration {
                    name: name__.unwrap_or_default(),
                    route_configuration_name: route_configuration_name__.unwrap_or_default(),
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.ScopedRouteConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_route_configuration::Key {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fragments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.ScopedRouteConfiguration.Key", len)?;
        if !self.fragments.is_empty() {
            struct_ser.serialize_field("fragments", &self.fragments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_route_configuration::Key {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fragments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fragments,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "fragments" => Ok(GeneratedField::Fragments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_route_configuration::Key;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.ScopedRouteConfiguration.Key")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_route_configuration::Key, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fragments__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fragments => {
                            if fragments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fragments"));
                            }
                            fragments__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(scoped_route_configuration::Key {
                    fragments: fragments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.ScopedRouteConfiguration.Key", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_route_configuration::key::Fragment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.ScopedRouteConfiguration.Key.Fragment", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                scoped_route_configuration::key::fragment::Type::StringKey(v) => {
                    struct_ser.serialize_field("stringKey", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_route_configuration::key::Fragment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "string_key",
            "stringKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StringKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "stringKey" | "string_key" => Ok(GeneratedField::StringKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_route_configuration::key::Fragment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.ScopedRouteConfiguration.Key.Fragment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_route_configuration::key::Fragment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StringKey => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringKey"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<_>>()?.map(scoped_route_configuration::key::fragment::Type::StringKey);
                        }
                    }
                }
                Ok(scoped_route_configuration::key::Fragment {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.ScopedRouteConfiguration.Key.Fragment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SrdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.SrdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SrdsDummy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SrdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.SrdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SrdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SrdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.SrdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamBindConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.UpstreamBindConfig", len)?;
        if let Some(v) = self.source_address.as_ref() {
            struct_ser.serialize_field("sourceAddress", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamBindConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_address",
            "sourceAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceAddress,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "sourceAddress" | "source_address" => Ok(GeneratedField::SourceAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamBindConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.UpstreamBindConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamBindConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceAddress => {
                            if source_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceAddress"));
                            }
                            source_address__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpstreamBindConfig {
                    source_address: source_address__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.UpstreamBindConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamConnectionOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tcp_keepalive.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.UpstreamConnectionOptions", len)?;
        if let Some(v) = self.tcp_keepalive.as_ref() {
            struct_ser.serialize_field("tcpKeepalive", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamConnectionOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tcp_keepalive",
            "tcpKeepalive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TcpKeepalive,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "tcpKeepalive" | "tcp_keepalive" => Ok(GeneratedField::TcpKeepalive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamConnectionOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.UpstreamConnectionOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamConnectionOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tcp_keepalive__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TcpKeepalive => {
                            if tcp_keepalive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpKeepalive"));
                            }
                            tcp_keepalive__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpstreamConnectionOptions {
                    tcp_keepalive: tcp_keepalive__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.UpstreamConnectionOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vhds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.Vhds", len)?;
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("configSource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vhds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_source",
            "configSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigSource,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "configSource" | "config_source" => Ok(GeneratedField::ConfigSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vhds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.Vhds")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Vhds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfigSource => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSource"));
                            }
                            config_source__ = map.next_value()?;
                        }
                    }
                }
                Ok(Vhds {
                    config_source: config_source__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.Vhds", FIELDS, GeneratedVisitor)
    }
}
