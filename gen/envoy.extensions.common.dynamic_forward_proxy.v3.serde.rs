// @generated
impl serde::Serialize for DnsCacheCircuitBreakers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_pending_requests.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.common.dynamic_forward_proxy.v3.DnsCacheCircuitBreakers", len)?;
        if let Some(v) = self.max_pending_requests.as_ref() {
            struct_ser.serialize_field("maxPendingRequests", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsCacheCircuitBreakers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_pending_requests",
            "maxPendingRequests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxPendingRequests,
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
                            "maxPendingRequests" | "max_pending_requests" => Ok(GeneratedField::MaxPendingRequests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsCacheCircuitBreakers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.common.dynamic_forward_proxy.v3.DnsCacheCircuitBreakers")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsCacheCircuitBreakers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_pending_requests__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxPendingRequests => {
                            if max_pending_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPendingRequests"));
                            }
                            max_pending_requests__ = map.next_value()?;
                        }
                    }
                }
                Ok(DnsCacheCircuitBreakers {
                    max_pending_requests: max_pending_requests__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.common.dynamic_forward_proxy.v3.DnsCacheCircuitBreakers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DnsCacheConfig {
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
        if self.dns_lookup_family != 0 {
            len += 1;
        }
        if self.dns_refresh_rate.is_some() {
            len += 1;
        }
        if self.dns_min_refresh_rate.is_some() {
            len += 1;
        }
        if self.host_ttl.is_some() {
            len += 1;
        }
        if self.max_hosts.is_some() {
            len += 1;
        }
        if self.dns_failure_refresh_rate.is_some() {
            len += 1;
        }
        if self.dns_cache_circuit_breaker.is_some() {
            len += 1;
        }
        if self.use_tcp_for_dns_lookups {
            len += 1;
        }
        if self.dns_resolution_config.is_some() {
            len += 1;
        }
        if self.typed_dns_resolver_config.is_some() {
            len += 1;
        }
        if !self.preresolve_hostnames.is_empty() {
            len += 1;
        }
        if self.dns_query_timeout.is_some() {
            len += 1;
        }
        if self.key_value_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.common.dynamic_forward_proxy.v3.DnsCacheConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.dns_lookup_family != 0 {
            let v = super::super::super::super::config::cluster::v3::cluster::DnsLookupFamily::from_i32(self.dns_lookup_family)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.dns_lookup_family)))?;
            struct_ser.serialize_field("dnsLookupFamily", &v)?;
        }
        if let Some(v) = self.dns_refresh_rate.as_ref() {
            struct_ser.serialize_field("dnsRefreshRate", v)?;
        }
        if let Some(v) = self.dns_min_refresh_rate.as_ref() {
            struct_ser.serialize_field("dnsMinRefreshRate", v)?;
        }
        if let Some(v) = self.host_ttl.as_ref() {
            struct_ser.serialize_field("hostTtl", v)?;
        }
        if let Some(v) = self.max_hosts.as_ref() {
            struct_ser.serialize_field("maxHosts", v)?;
        }
        if let Some(v) = self.dns_failure_refresh_rate.as_ref() {
            struct_ser.serialize_field("dnsFailureRefreshRate", v)?;
        }
        if let Some(v) = self.dns_cache_circuit_breaker.as_ref() {
            struct_ser.serialize_field("dnsCacheCircuitBreaker", v)?;
        }
        if self.use_tcp_for_dns_lookups {
            struct_ser.serialize_field("useTcpForDnsLookups", &self.use_tcp_for_dns_lookups)?;
        }
        if let Some(v) = self.dns_resolution_config.as_ref() {
            struct_ser.serialize_field("dnsResolutionConfig", v)?;
        }
        if let Some(v) = self.typed_dns_resolver_config.as_ref() {
            struct_ser.serialize_field("typedDnsResolverConfig", v)?;
        }
        if !self.preresolve_hostnames.is_empty() {
            struct_ser.serialize_field("preresolveHostnames", &self.preresolve_hostnames)?;
        }
        if let Some(v) = self.dns_query_timeout.as_ref() {
            struct_ser.serialize_field("dnsQueryTimeout", v)?;
        }
        if let Some(v) = self.key_value_config.as_ref() {
            struct_ser.serialize_field("keyValueConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsCacheConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "dns_lookup_family",
            "dnsLookupFamily",
            "dns_refresh_rate",
            "dnsRefreshRate",
            "dns_min_refresh_rate",
            "dnsMinRefreshRate",
            "host_ttl",
            "hostTtl",
            "max_hosts",
            "maxHosts",
            "dns_failure_refresh_rate",
            "dnsFailureRefreshRate",
            "dns_cache_circuit_breaker",
            "dnsCacheCircuitBreaker",
            "use_tcp_for_dns_lookups",
            "useTcpForDnsLookups",
            "dns_resolution_config",
            "dnsResolutionConfig",
            "typed_dns_resolver_config",
            "typedDnsResolverConfig",
            "preresolve_hostnames",
            "preresolveHostnames",
            "dns_query_timeout",
            "dnsQueryTimeout",
            "key_value_config",
            "keyValueConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DnsLookupFamily,
            DnsRefreshRate,
            DnsMinRefreshRate,
            HostTtl,
            MaxHosts,
            DnsFailureRefreshRate,
            DnsCacheCircuitBreaker,
            UseTcpForDnsLookups,
            DnsResolutionConfig,
            TypedDnsResolverConfig,
            PreresolveHostnames,
            DnsQueryTimeout,
            KeyValueConfig,
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
                            "dnsLookupFamily" | "dns_lookup_family" => Ok(GeneratedField::DnsLookupFamily),
                            "dnsRefreshRate" | "dns_refresh_rate" => Ok(GeneratedField::DnsRefreshRate),
                            "dnsMinRefreshRate" | "dns_min_refresh_rate" => Ok(GeneratedField::DnsMinRefreshRate),
                            "hostTtl" | "host_ttl" => Ok(GeneratedField::HostTtl),
                            "maxHosts" | "max_hosts" => Ok(GeneratedField::MaxHosts),
                            "dnsFailureRefreshRate" | "dns_failure_refresh_rate" => Ok(GeneratedField::DnsFailureRefreshRate),
                            "dnsCacheCircuitBreaker" | "dns_cache_circuit_breaker" => Ok(GeneratedField::DnsCacheCircuitBreaker),
                            "useTcpForDnsLookups" | "use_tcp_for_dns_lookups" => Ok(GeneratedField::UseTcpForDnsLookups),
                            "dnsResolutionConfig" | "dns_resolution_config" => Ok(GeneratedField::DnsResolutionConfig),
                            "typedDnsResolverConfig" | "typed_dns_resolver_config" => Ok(GeneratedField::TypedDnsResolverConfig),
                            "preresolveHostnames" | "preresolve_hostnames" => Ok(GeneratedField::PreresolveHostnames),
                            "dnsQueryTimeout" | "dns_query_timeout" => Ok(GeneratedField::DnsQueryTimeout),
                            "keyValueConfig" | "key_value_config" => Ok(GeneratedField::KeyValueConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsCacheConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.common.dynamic_forward_proxy.v3.DnsCacheConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsCacheConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut dns_lookup_family__ = None;
                let mut dns_refresh_rate__ = None;
                let mut dns_min_refresh_rate__ = None;
                let mut host_ttl__ = None;
                let mut max_hosts__ = None;
                let mut dns_failure_refresh_rate__ = None;
                let mut dns_cache_circuit_breaker__ = None;
                let mut use_tcp_for_dns_lookups__ = None;
                let mut dns_resolution_config__ = None;
                let mut typed_dns_resolver_config__ = None;
                let mut preresolve_hostnames__ = None;
                let mut dns_query_timeout__ = None;
                let mut key_value_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsLookupFamily => {
                            if dns_lookup_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsLookupFamily"));
                            }
                            dns_lookup_family__ = Some(map.next_value::<super::super::super::super::config::cluster::v3::cluster::DnsLookupFamily>()? as i32);
                        }
                        GeneratedField::DnsRefreshRate => {
                            if dns_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsRefreshRate"));
                            }
                            dns_refresh_rate__ = map.next_value()?;
                        }
                        GeneratedField::DnsMinRefreshRate => {
                            if dns_min_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsMinRefreshRate"));
                            }
                            dns_min_refresh_rate__ = map.next_value()?;
                        }
                        GeneratedField::HostTtl => {
                            if host_ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostTtl"));
                            }
                            host_ttl__ = map.next_value()?;
                        }
                        GeneratedField::MaxHosts => {
                            if max_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxHosts"));
                            }
                            max_hosts__ = map.next_value()?;
                        }
                        GeneratedField::DnsFailureRefreshRate => {
                            if dns_failure_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsFailureRefreshRate"));
                            }
                            dns_failure_refresh_rate__ = map.next_value()?;
                        }
                        GeneratedField::DnsCacheCircuitBreaker => {
                            if dns_cache_circuit_breaker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsCacheCircuitBreaker"));
                            }
                            dns_cache_circuit_breaker__ = map.next_value()?;
                        }
                        GeneratedField::UseTcpForDnsLookups => {
                            if use_tcp_for_dns_lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useTcpForDnsLookups"));
                            }
                            use_tcp_for_dns_lookups__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsResolutionConfig => {
                            if dns_resolution_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsResolutionConfig"));
                            }
                            dns_resolution_config__ = map.next_value()?;
                        }
                        GeneratedField::TypedDnsResolverConfig => {
                            if typed_dns_resolver_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedDnsResolverConfig"));
                            }
                            typed_dns_resolver_config__ = map.next_value()?;
                        }
                        GeneratedField::PreresolveHostnames => {
                            if preresolve_hostnames__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preresolveHostnames"));
                            }
                            preresolve_hostnames__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsQueryTimeout => {
                            if dns_query_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsQueryTimeout"));
                            }
                            dns_query_timeout__ = map.next_value()?;
                        }
                        GeneratedField::KeyValueConfig => {
                            if key_value_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyValueConfig"));
                            }
                            key_value_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(DnsCacheConfig {
                    name: name__.unwrap_or_default(),
                    dns_lookup_family: dns_lookup_family__.unwrap_or_default(),
                    dns_refresh_rate: dns_refresh_rate__,
                    dns_min_refresh_rate: dns_min_refresh_rate__,
                    host_ttl: host_ttl__,
                    max_hosts: max_hosts__,
                    dns_failure_refresh_rate: dns_failure_refresh_rate__,
                    dns_cache_circuit_breaker: dns_cache_circuit_breaker__,
                    use_tcp_for_dns_lookups: use_tcp_for_dns_lookups__.unwrap_or_default(),
                    dns_resolution_config: dns_resolution_config__,
                    typed_dns_resolver_config: typed_dns_resolver_config__,
                    preresolve_hostnames: preresolve_hostnames__.unwrap_or_default(),
                    dns_query_timeout: dns_query_timeout__,
                    key_value_config: key_value_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.common.dynamic_forward_proxy.v3.DnsCacheConfig", FIELDS, GeneratedVisitor)
    }
}
