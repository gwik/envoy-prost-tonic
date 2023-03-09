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
        if self.deprecated_v1.is_some() {
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
        if self.cluster_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
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
        if let Some(v) = self.deprecated_v1.as_ref() {
            struct_ser.serialize_field("deprecatedV1", v)?;
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
            "deprecated_v1",
            "deprecatedV1",
            "max_connect_attempts",
            "maxConnectAttempts",
            "hash_policy",
            "hashPolicy",
            "tunneling_config",
            "tunnelingConfig",
            "cluster",
            "weighted_clusters",
            "weightedClusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            MetadataMatch,
            IdleTimeout,
            DownstreamIdleTimeout,
            UpstreamIdleTimeout,
            AccessLog,
            DeprecatedV1,
            MaxConnectAttempts,
            HashPolicy,
            TunnelingConfig,
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
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "downstreamIdleTimeout" | "downstream_idle_timeout" => Ok(GeneratedField::DownstreamIdleTimeout),
                            "upstreamIdleTimeout" | "upstream_idle_timeout" => Ok(GeneratedField::UpstreamIdleTimeout),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "deprecatedV1" | "deprecated_v1" => Ok(GeneratedField::DeprecatedV1),
                            "maxConnectAttempts" | "max_connect_attempts" => Ok(GeneratedField::MaxConnectAttempts),
                            "hashPolicy" | "hash_policy" => Ok(GeneratedField::HashPolicy),
                            "tunnelingConfig" | "tunneling_config" => Ok(GeneratedField::TunnelingConfig),
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
                formatter.write_str("struct envoy.config.filter.network.tcp_proxy.v2.TcpProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TcpProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut metadata_match__ = None;
                let mut idle_timeout__ = None;
                let mut downstream_idle_timeout__ = None;
                let mut upstream_idle_timeout__ = None;
                let mut access_log__ = None;
                let mut deprecated_v1__ = None;
                let mut max_connect_attempts__ = None;
                let mut hash_policy__ = None;
                let mut tunneling_config__ = None;
                let mut cluster_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
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
                        GeneratedField::DeprecatedV1 => {
                            if deprecated_v1__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deprecatedV1"));
                            }
                            deprecated_v1__ = map.next_value()?;
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
                    metadata_match: metadata_match__,
                    idle_timeout: idle_timeout__,
                    downstream_idle_timeout: downstream_idle_timeout__,
                    upstream_idle_timeout: upstream_idle_timeout__,
                    access_log: access_log__.unwrap_or_default(),
                    deprecated_v1: deprecated_v1__,
                    max_connect_attempts: max_connect_attempts__,
                    hash_policy: hash_policy__.unwrap_or_default(),
                    tunneling_config: tunneling_config__,
                    cluster_specifier: cluster_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tcp_proxy::DeprecatedV1 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.routes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.DeprecatedV1", len)?;
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tcp_proxy::DeprecatedV1 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Routes,
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
                            "routes" => Ok(GeneratedField::Routes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tcp_proxy::DeprecatedV1;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.tcp_proxy.v2.TcpProxy.DeprecatedV1")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tcp_proxy::DeprecatedV1, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut routes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(tcp_proxy::DeprecatedV1 {
                    routes: routes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.DeprecatedV1", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tcp_proxy::deprecated_v1::TcpRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster.is_empty() {
            len += 1;
        }
        if !self.destination_ip_list.is_empty() {
            len += 1;
        }
        if !self.destination_ports.is_empty() {
            len += 1;
        }
        if !self.source_ip_list.is_empty() {
            len += 1;
        }
        if !self.source_ports.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.DeprecatedV1.TCPRoute", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if !self.destination_ip_list.is_empty() {
            struct_ser.serialize_field("destinationIpList", &self.destination_ip_list)?;
        }
        if !self.destination_ports.is_empty() {
            struct_ser.serialize_field("destinationPorts", &self.destination_ports)?;
        }
        if !self.source_ip_list.is_empty() {
            struct_ser.serialize_field("sourceIpList", &self.source_ip_list)?;
        }
        if !self.source_ports.is_empty() {
            struct_ser.serialize_field("sourcePorts", &self.source_ports)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tcp_proxy::deprecated_v1::TcpRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "destination_ip_list",
            "destinationIpList",
            "destination_ports",
            "destinationPorts",
            "source_ip_list",
            "sourceIpList",
            "source_ports",
            "sourcePorts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            DestinationIpList,
            DestinationPorts,
            SourceIpList,
            SourcePorts,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "destinationIpList" | "destination_ip_list" => Ok(GeneratedField::DestinationIpList),
                            "destinationPorts" | "destination_ports" => Ok(GeneratedField::DestinationPorts),
                            "sourceIpList" | "source_ip_list" => Ok(GeneratedField::SourceIpList),
                            "sourcePorts" | "source_ports" => Ok(GeneratedField::SourcePorts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tcp_proxy::deprecated_v1::TcpRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.tcp_proxy.v2.TcpProxy.DeprecatedV1.TCPRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tcp_proxy::deprecated_v1::TcpRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut destination_ip_list__ = None;
                let mut destination_ports__ = None;
                let mut source_ip_list__ = None;
                let mut source_ports__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::DestinationIpList => {
                            if destination_ip_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationIpList"));
                            }
                            destination_ip_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::DestinationPorts => {
                            if destination_ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPorts"));
                            }
                            destination_ports__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceIpList => {
                            if source_ip_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIpList"));
                            }
                            source_ip_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourcePorts => {
                            if source_ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePorts"));
                            }
                            source_ports__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(tcp_proxy::deprecated_v1::TcpRoute {
                    cluster: cluster__.unwrap_or_default(),
                    destination_ip_list: destination_ip_list__.unwrap_or_default(),
                    destination_ports: destination_ports__.unwrap_or_default(),
                    source_ip_list: source_ip_list__.unwrap_or_default(),
                    source_ports: source_ports__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.DeprecatedV1.TCPRoute", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.TunnelingConfig", len)?;
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hostname,
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
                formatter.write_str("struct envoy.config.filter.network.tcp_proxy.v2.TcpProxy.TunnelingConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tcp_proxy::TunnelingConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hostname__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(tcp_proxy::TunnelingConfig {
                    hostname: hostname__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.TunnelingConfig", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.WeightedCluster", len)?;
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
                formatter.write_str("struct envoy.config.filter.network.tcp_proxy.v2.TcpProxy.WeightedCluster")
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
        deserializer.deserialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.WeightedCluster", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.WeightedCluster.ClusterWeight", len)?;
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
                formatter.write_str("struct envoy.config.filter.network.tcp_proxy.v2.TcpProxy.WeightedCluster.ClusterWeight")
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
        deserializer.deserialize_struct("envoy.config.filter.network.tcp_proxy.v2.TcpProxy.WeightedCluster.ClusterWeight", FIELDS, GeneratedVisitor)
    }
}
