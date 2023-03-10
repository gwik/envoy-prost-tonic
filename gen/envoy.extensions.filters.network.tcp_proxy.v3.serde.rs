// @generated
impl serde::Serialize for TcpProxy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.on_demand.is_some() {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.downstream_idle_timeout.is_some() {
            len += 1;
        }
        if self.upstream_idle_timeout.is_some() {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        if self.max_connect_attempts.is_some() {
            len += 1;
        }
        if !self.hash_policy.is_empty() {
            len += 1;
        }
        if self.tunneling_config.is_some() {
            len += 1;
        }
        if self.max_downstream_connection_duration.is_some() {
            len += 1;
        }
        if self.access_log_flush_interval.is_some() {
            len += 1;
        }
        if self.cluster_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.on_demand.as_ref() {
            struct_ser.serialize_field("onDemand", v)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        if let Some(v) = self.downstream_idle_timeout.as_ref() {
            struct_ser.serialize_field("downstreamIdleTimeout", v)?;
        }
        if let Some(v) = self.upstream_idle_timeout.as_ref() {
            struct_ser.serialize_field("upstreamIdleTimeout", v)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("accessLog", &self.access_log)?;
        }
        if let Some(v) = self.max_connect_attempts.as_ref() {
            struct_ser.serialize_field("maxConnectAttempts", v)?;
        }
        if !self.hash_policy.is_empty() {
            struct_ser.serialize_field("hashPolicy", &self.hash_policy)?;
        }
        if let Some(v) = self.tunneling_config.as_ref() {
            struct_ser.serialize_field("tunnelingConfig", v)?;
        }
        if let Some(v) = self.max_downstream_connection_duration.as_ref() {
            struct_ser.serialize_field("maxDownstreamConnectionDuration", v)?;
        }
        if let Some(v) = self.access_log_flush_interval.as_ref() {
            struct_ser.serialize_field("accessLogFlushInterval", v)?;
        }
        if let Some(v) = self.cluster_specifier.as_ref() {
            match v {
                tcp_proxy::ClusterSpecifier::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
                tcp_proxy::ClusterSpecifier::WeightedClusters(v) => {
                    struct_ser.serialize_field("weightedClusters", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TcpProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "on_demand",
            "onDemand",
            "metadata_match",
            "metadataMatch",
            "idle_timeout",
            "idleTimeout",
            "downstream_idle_timeout",
            "downstreamIdleTimeout",
            "upstream_idle_timeout",
            "upstreamIdleTimeout",
            "access_log",
            "accessLog",
            "max_connect_attempts",
            "maxConnectAttempts",
            "hash_policy",
            "hashPolicy",
            "tunneling_config",
            "tunnelingConfig",
            "max_downstream_connection_duration",
            "maxDownstreamConnectionDuration",
            "access_log_flush_interval",
            "accessLogFlushInterval",
            "cluster",
            "weighted_clusters",
            "weightedClusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            OnDemand,
            MetadataMatch,
            IdleTimeout,
            DownstreamIdleTimeout,
            UpstreamIdleTimeout,
            AccessLog,
            MaxConnectAttempts,
            HashPolicy,
            TunnelingConfig,
            MaxDownstreamConnectionDuration,
            AccessLogFlushInterval,
            Cluster,
            WeightedClusters,
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
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "onDemand" | "on_demand" => Ok(GeneratedField::OnDemand),
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "downstreamIdleTimeout" | "downstream_idle_timeout" => Ok(GeneratedField::DownstreamIdleTimeout),
                            "upstreamIdleTimeout" | "upstream_idle_timeout" => Ok(GeneratedField::UpstreamIdleTimeout),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "maxConnectAttempts" | "max_connect_attempts" => Ok(GeneratedField::MaxConnectAttempts),
                            "hashPolicy" | "hash_policy" => Ok(GeneratedField::HashPolicy),
                            "tunnelingConfig" | "tunneling_config" => Ok(GeneratedField::TunnelingConfig),
                            "maxDownstreamConnectionDuration" | "max_downstream_connection_duration" => Ok(GeneratedField::MaxDownstreamConnectionDuration),
                            "accessLogFlushInterval" | "access_log_flush_interval" => Ok(GeneratedField::AccessLogFlushInterval),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "weightedClusters" | "weighted_clusters" => Ok(GeneratedField::WeightedClusters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TcpProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TcpProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut on_demand__ = None;
                let mut metadata_match__ = None;
                let mut idle_timeout__ = None;
                let mut downstream_idle_timeout__ = None;
                let mut upstream_idle_timeout__ = None;
                let mut access_log__ = None;
                let mut max_connect_attempts__ = None;
                let mut hash_policy__ = None;
                let mut tunneling_config__ = None;
                let mut max_downstream_connection_duration__ = None;
                let mut access_log_flush_interval__ = None;
                let mut cluster_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::OnDemand => {
                            if on_demand__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onDemand"));
                            }
                            on_demand__ = map.next_value()?;
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::DownstreamIdleTimeout => {
                            if downstream_idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamIdleTimeout"));
                            }
                            downstream_idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamIdleTimeout => {
                            if upstream_idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamIdleTimeout"));
                            }
                            upstream_idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxConnectAttempts => {
                            if max_connect_attempts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnectAttempts"));
                            }
                            max_connect_attempts__ = map.next_value()?;
                        }
                        GeneratedField::HashPolicy => {
                            if hash_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashPolicy"));
                            }
                            hash_policy__ = Some(map.next_value()?);
                        }
                        GeneratedField::TunnelingConfig => {
                            if tunneling_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tunnelingConfig"));
                            }
                            tunneling_config__ = map.next_value()?;
                        }
                        GeneratedField::MaxDownstreamConnectionDuration => {
                            if max_downstream_connection_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDownstreamConnectionDuration"));
                            }
                            max_downstream_connection_duration__ = map.next_value()?;
                        }
                        GeneratedField::AccessLogFlushInterval => {
                            if access_log_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLogFlushInterval"));
                            }
                            access_log_flush_interval__ = map.next_value()?;
                        }
                        GeneratedField::Cluster => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(tcp_proxy::ClusterSpecifier::Cluster);
                        }
                        GeneratedField::WeightedClusters => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightedClusters"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(tcp_proxy::ClusterSpecifier::WeightedClusters)
;
                        }
                    }
                }
                Ok(TcpProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    on_demand: on_demand__,
                    metadata_match: metadata_match__,
                    idle_timeout: idle_timeout__,
                    downstream_idle_timeout: downstream_idle_timeout__,
                    upstream_idle_timeout: upstream_idle_timeout__,
                    access_log: access_log__.unwrap_or_default(),
                    max_connect_attempts: max_connect_attempts__,
                    hash_policy: hash_policy__.unwrap_or_default(),
                    tunneling_config: tunneling_config__,
                    max_downstream_connection_duration: max_downstream_connection_duration__,
                    access_log_flush_interval: access_log_flush_interval__,
                    cluster_specifier: cluster_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tcp_proxy::OnDemand {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.odcds_config.is_some() {
            len += 1;
        }
        if !self.resources_locator.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.OnDemand", len)?;
        if let Some(v) = self.odcds_config.as_ref() {
            struct_ser.serialize_field("odcdsConfig", v)?;
        }
        if !self.resources_locator.is_empty() {
            struct_ser.serialize_field("resourcesLocator", &self.resources_locator)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tcp_proxy::OnDemand {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "odcds_config",
            "odcdsConfig",
            "resources_locator",
            "resourcesLocator",
            "timeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OdcdsConfig,
            ResourcesLocator,
            Timeout,
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
                            "odcdsConfig" | "odcds_config" => Ok(GeneratedField::OdcdsConfig),
                            "resourcesLocator" | "resources_locator" => Ok(GeneratedField::ResourcesLocator),
                            "timeout" => Ok(GeneratedField::Timeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tcp_proxy::OnDemand;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.OnDemand")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tcp_proxy::OnDemand, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut odcds_config__ = None;
                let mut resources_locator__ = None;
                let mut timeout__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OdcdsConfig => {
                            if odcds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("odcdsConfig"));
                            }
                            odcds_config__ = map.next_value()?;
                        }
                        GeneratedField::ResourcesLocator => {
                            if resources_locator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourcesLocator"));
                            }
                            resources_locator__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                    }
                }
                Ok(tcp_proxy::OnDemand {
                    odcds_config: odcds_config__,
                    resources_locator: resources_locator__.unwrap_or_default(),
                    timeout: timeout__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.OnDemand", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tcp_proxy::TunnelingConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hostname.is_empty() {
            len += 1;
        }
        if self.use_post {
            len += 1;
        }
        if !self.headers_to_add.is_empty() {
            len += 1;
        }
        if self.propagate_response_headers {
            len += 1;
        }
        if !self.post_path.is_empty() {
            len += 1;
        }
        if self.propagate_response_trailers {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.TunnelingConfig", len)?;
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        if self.use_post {
            struct_ser.serialize_field("usePost", &self.use_post)?;
        }
        if !self.headers_to_add.is_empty() {
            struct_ser.serialize_field("headersToAdd", &self.headers_to_add)?;
        }
        if self.propagate_response_headers {
            struct_ser.serialize_field("propagateResponseHeaders", &self.propagate_response_headers)?;
        }
        if !self.post_path.is_empty() {
            struct_ser.serialize_field("postPath", &self.post_path)?;
        }
        if self.propagate_response_trailers {
            struct_ser.serialize_field("propagateResponseTrailers", &self.propagate_response_trailers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tcp_proxy::TunnelingConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hostname",
            "use_post",
            "usePost",
            "headers_to_add",
            "headersToAdd",
            "propagate_response_headers",
            "propagateResponseHeaders",
            "post_path",
            "postPath",
            "propagate_response_trailers",
            "propagateResponseTrailers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hostname,
            UsePost,
            HeadersToAdd,
            PropagateResponseHeaders,
            PostPath,
            PropagateResponseTrailers,
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
                            "hostname" => Ok(GeneratedField::Hostname),
                            "usePost" | "use_post" => Ok(GeneratedField::UsePost),
                            "headersToAdd" | "headers_to_add" => Ok(GeneratedField::HeadersToAdd),
                            "propagateResponseHeaders" | "propagate_response_headers" => Ok(GeneratedField::PropagateResponseHeaders),
                            "postPath" | "post_path" => Ok(GeneratedField::PostPath),
                            "propagateResponseTrailers" | "propagate_response_trailers" => Ok(GeneratedField::PropagateResponseTrailers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tcp_proxy::TunnelingConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.TunnelingConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tcp_proxy::TunnelingConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hostname__ = None;
                let mut use_post__ = None;
                let mut headers_to_add__ = None;
                let mut propagate_response_headers__ = None;
                let mut post_path__ = None;
                let mut propagate_response_trailers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map.next_value()?);
                        }
                        GeneratedField::UsePost => {
                            if use_post__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usePost"));
                            }
                            use_post__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeadersToAdd => {
                            if headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersToAdd"));
                            }
                            headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::PropagateResponseHeaders => {
                            if propagate_response_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagateResponseHeaders"));
                            }
                            propagate_response_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::PostPath => {
                            if post_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postPath"));
                            }
                            post_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::PropagateResponseTrailers => {
                            if propagate_response_trailers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagateResponseTrailers"));
                            }
                            propagate_response_trailers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(tcp_proxy::TunnelingConfig {
                    hostname: hostname__.unwrap_or_default(),
                    use_post: use_post__.unwrap_or_default(),
                    headers_to_add: headers_to_add__.unwrap_or_default(),
                    propagate_response_headers: propagate_response_headers__.unwrap_or_default(),
                    post_path: post_path__.unwrap_or_default(),
                    propagate_response_trailers: propagate_response_trailers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.TunnelingConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tcp_proxy::WeightedCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.clusters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.WeightedCluster", len)?;
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tcp_proxy::WeightedCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Clusters,
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
                            "clusters" => Ok(GeneratedField::Clusters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tcp_proxy::WeightedCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.WeightedCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tcp_proxy::WeightedCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut clusters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(tcp_proxy::WeightedCluster {
                    clusters: clusters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.WeightedCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tcp_proxy::weighted_cluster::ClusterWeight {
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
        if self.weight != 0 {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.WeightedCluster.ClusterWeight", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tcp_proxy::weighted_cluster::ClusterWeight {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "weight",
            "metadata_match",
            "metadataMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Weight,
            MetadataMatch,
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
                            "weight" => Ok(GeneratedField::Weight),
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tcp_proxy::weighted_cluster::ClusterWeight;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.WeightedCluster.ClusterWeight")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tcp_proxy::weighted_cluster::ClusterWeight, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut weight__ = None;
                let mut metadata_match__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                    }
                }
                Ok(tcp_proxy::weighted_cluster::ClusterWeight {
                    name: name__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    metadata_match: metadata_match__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.tcp_proxy.v3.TcpProxy.WeightedCluster.ClusterWeight", FIELDS, GeneratedVisitor)
    }
}
