// @generated
impl serde::Serialize for Route {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.udp.udp_proxy.v3.Route", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Route {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Route;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.udp.udp_proxy.v3.Route")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Route, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Route {
                    cluster: cluster__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.udp.udp_proxy.v3.Route", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UdpProxyConfig {
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
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.use_original_src_ip {
            len += 1;
        }
        if !self.hash_policies.is_empty() {
            len += 1;
        }
        if self.upstream_socket_config.is_some() {
            len += 1;
        }
        if self.use_per_packet_load_balancing {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        if !self.proxy_access_log.is_empty() {
            len += 1;
        }
        if self.route_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.udp.udp_proxy.v3.UdpProxyConfig", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        if self.use_original_src_ip {
            struct_ser.serialize_field("useOriginalSrcIp", &self.use_original_src_ip)?;
        }
        if !self.hash_policies.is_empty() {
            struct_ser.serialize_field("hashPolicies", &self.hash_policies)?;
        }
        if let Some(v) = self.upstream_socket_config.as_ref() {
            struct_ser.serialize_field("upstreamSocketConfig", v)?;
        }
        if self.use_per_packet_load_balancing {
            struct_ser.serialize_field("usePerPacketLoadBalancing", &self.use_per_packet_load_balancing)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("accessLog", &self.access_log)?;
        }
        if !self.proxy_access_log.is_empty() {
            struct_ser.serialize_field("proxyAccessLog", &self.proxy_access_log)?;
        }
        if let Some(v) = self.route_specifier.as_ref() {
            match v {
                udp_proxy_config::RouteSpecifier::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
                udp_proxy_config::RouteSpecifier::Matcher(v) => {
                    struct_ser.serialize_field("matcher", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UdpProxyConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "idle_timeout",
            "idleTimeout",
            "use_original_src_ip",
            "useOriginalSrcIp",
            "hash_policies",
            "hashPolicies",
            "upstream_socket_config",
            "upstreamSocketConfig",
            "use_per_packet_load_balancing",
            "usePerPacketLoadBalancing",
            "access_log",
            "accessLog",
            "proxy_access_log",
            "proxyAccessLog",
            "cluster",
            "matcher",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            IdleTimeout,
            UseOriginalSrcIp,
            HashPolicies,
            UpstreamSocketConfig,
            UsePerPacketLoadBalancing,
            AccessLog,
            ProxyAccessLog,
            Cluster,
            Matcher,
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
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "useOriginalSrcIp" | "use_original_src_ip" => Ok(GeneratedField::UseOriginalSrcIp),
                            "hashPolicies" | "hash_policies" => Ok(GeneratedField::HashPolicies),
                            "upstreamSocketConfig" | "upstream_socket_config" => Ok(GeneratedField::UpstreamSocketConfig),
                            "usePerPacketLoadBalancing" | "use_per_packet_load_balancing" => Ok(GeneratedField::UsePerPacketLoadBalancing),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "proxyAccessLog" | "proxy_access_log" => Ok(GeneratedField::ProxyAccessLog),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "matcher" => Ok(GeneratedField::Matcher),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UdpProxyConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.udp.udp_proxy.v3.UdpProxyConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UdpProxyConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut idle_timeout__ = None;
                let mut use_original_src_ip__ = None;
                let mut hash_policies__ = None;
                let mut upstream_socket_config__ = None;
                let mut use_per_packet_load_balancing__ = None;
                let mut access_log__ = None;
                let mut proxy_access_log__ = None;
                let mut route_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::UseOriginalSrcIp => {
                            if use_original_src_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useOriginalSrcIp"));
                            }
                            use_original_src_ip__ = Some(map.next_value()?);
                        }
                        GeneratedField::HashPolicies => {
                            if hash_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashPolicies"));
                            }
                            hash_policies__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamSocketConfig => {
                            if upstream_socket_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamSocketConfig"));
                            }
                            upstream_socket_config__ = map.next_value()?;
                        }
                        GeneratedField::UsePerPacketLoadBalancing => {
                            if use_per_packet_load_balancing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("usePerPacketLoadBalancing"));
                            }
                            use_per_packet_load_balancing__ = Some(map.next_value()?);
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProxyAccessLog => {
                            if proxy_access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxyAccessLog"));
                            }
                            proxy_access_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            route_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(udp_proxy_config::RouteSpecifier::Cluster);
                        }
                        GeneratedField::Matcher => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            route_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(udp_proxy_config::RouteSpecifier::Matcher)
;
                        }
                    }
                }
                Ok(UdpProxyConfig {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    idle_timeout: idle_timeout__,
                    use_original_src_ip: use_original_src_ip__.unwrap_or_default(),
                    hash_policies: hash_policies__.unwrap_or_default(),
                    upstream_socket_config: upstream_socket_config__,
                    use_per_packet_load_balancing: use_per_packet_load_balancing__.unwrap_or_default(),
                    access_log: access_log__.unwrap_or_default(),
                    proxy_access_log: proxy_access_log__.unwrap_or_default(),
                    route_specifier: route_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.udp.udp_proxy.v3.UdpProxyConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for udp_proxy_config::HashPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.udp.udp_proxy.v3.UdpProxyConfig.HashPolicy", len)?;
        if let Some(v) = self.policy_specifier.as_ref() {
            match v {
                udp_proxy_config::hash_policy::PolicySpecifier::SourceIp(v) => {
                    struct_ser.serialize_field("sourceIp", v)?;
                }
                udp_proxy_config::hash_policy::PolicySpecifier::Key(v) => {
                    struct_ser.serialize_field("key", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for udp_proxy_config::HashPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_ip",
            "sourceIp",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceIp,
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
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
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
            type Value = udp_proxy_config::HashPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.udp.udp_proxy.v3.UdpProxyConfig.HashPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<udp_proxy_config::HashPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceIp => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(udp_proxy_config::hash_policy::PolicySpecifier::SourceIp);
                        }
                        GeneratedField::Key => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(udp_proxy_config::hash_policy::PolicySpecifier::Key);
                        }
                    }
                }
                Ok(udp_proxy_config::HashPolicy {
                    policy_specifier: policy_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.udp.udp_proxy.v3.UdpProxyConfig.HashPolicy", FIELDS, GeneratedVisitor)
    }
}
