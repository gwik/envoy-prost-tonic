// @generated
impl serde::Serialize for DnsFilterConfig {
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
        if self.server_config.is_some() {
            len += 1;
        }
        if self.client_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.server_config.as_ref() {
            struct_ser.serialize_field("serverConfig", v)?;
        }
        if let Some(v) = self.client_config.as_ref() {
            struct_ser.serialize_field("clientConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsFilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "server_config",
            "serverConfig",
            "client_config",
            "clientConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            ServerConfig,
            ClientConfig,
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
                            "serverConfig" | "server_config" => Ok(GeneratedField::ServerConfig),
                            "clientConfig" | "client_config" => Ok(GeneratedField::ClientConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsFilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsFilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut server_config__ = None;
                let mut client_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServerConfig => {
                            if server_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverConfig"));
                            }
                            server_config__ = map.next_value()?;
                        }
                        GeneratedField::ClientConfig => {
                            if client_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientConfig"));
                            }
                            client_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(DnsFilterConfig {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    server_config: server_config__,
                    client_config: client_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_filter_config::ClientContextConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resolver_timeout.is_some() {
            len += 1;
        }
        if !self.upstream_resolvers.is_empty() {
            len += 1;
        }
        if self.dns_resolution_config.is_some() {
            len += 1;
        }
        if self.typed_dns_resolver_config.is_some() {
            len += 1;
        }
        if self.max_pending_lookups != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ClientContextConfig", len)?;
        if let Some(v) = self.resolver_timeout.as_ref() {
            struct_ser.serialize_field("resolverTimeout", v)?;
        }
        if !self.upstream_resolvers.is_empty() {
            struct_ser.serialize_field("upstreamResolvers", &self.upstream_resolvers)?;
        }
        if let Some(v) = self.dns_resolution_config.as_ref() {
            struct_ser.serialize_field("dnsResolutionConfig", v)?;
        }
        if let Some(v) = self.typed_dns_resolver_config.as_ref() {
            struct_ser.serialize_field("typedDnsResolverConfig", v)?;
        }
        if self.max_pending_lookups != 0 {
            struct_ser.serialize_field("maxPendingLookups", ToString::to_string(&self.max_pending_lookups).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_filter_config::ClientContextConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resolver_timeout",
            "resolverTimeout",
            "upstream_resolvers",
            "upstreamResolvers",
            "dns_resolution_config",
            "dnsResolutionConfig",
            "typed_dns_resolver_config",
            "typedDnsResolverConfig",
            "max_pending_lookups",
            "maxPendingLookups",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResolverTimeout,
            UpstreamResolvers,
            DnsResolutionConfig,
            TypedDnsResolverConfig,
            MaxPendingLookups,
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
                            "resolverTimeout" | "resolver_timeout" => Ok(GeneratedField::ResolverTimeout),
                            "upstreamResolvers" | "upstream_resolvers" => Ok(GeneratedField::UpstreamResolvers),
                            "dnsResolutionConfig" | "dns_resolution_config" => Ok(GeneratedField::DnsResolutionConfig),
                            "typedDnsResolverConfig" | "typed_dns_resolver_config" => Ok(GeneratedField::TypedDnsResolverConfig),
                            "maxPendingLookups" | "max_pending_lookups" => Ok(GeneratedField::MaxPendingLookups),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_filter_config::ClientContextConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ClientContextConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_filter_config::ClientContextConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resolver_timeout__ = None;
                let mut upstream_resolvers__ = None;
                let mut dns_resolution_config__ = None;
                let mut typed_dns_resolver_config__ = None;
                let mut max_pending_lookups__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResolverTimeout => {
                            if resolver_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolverTimeout"));
                            }
                            resolver_timeout__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamResolvers => {
                            if upstream_resolvers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamResolvers"));
                            }
                            upstream_resolvers__ = Some(map.next_value()?);
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
                        GeneratedField::MaxPendingLookups => {
                            if max_pending_lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPendingLookups"));
                            }
                            max_pending_lookups__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(dns_filter_config::ClientContextConfig {
                    resolver_timeout: resolver_timeout__,
                    upstream_resolvers: upstream_resolvers__.unwrap_or_default(),
                    dns_resolution_config: dns_resolution_config__,
                    typed_dns_resolver_config: typed_dns_resolver_config__,
                    max_pending_lookups: max_pending_lookups__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ClientContextConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_filter_config::ServerContextConfig {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ServerContextConfig", len)?;
        if let Some(v) = self.config_source.as_ref() {
            match v {
                dns_filter_config::server_context_config::ConfigSource::InlineDnsTable(v) => {
                    struct_ser.serialize_field("inlineDnsTable", v)?;
                }
                dns_filter_config::server_context_config::ConfigSource::ExternalDnsTable(v) => {
                    struct_ser.serialize_field("externalDnsTable", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_filter_config::ServerContextConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inline_dns_table",
            "inlineDnsTable",
            "external_dns_table",
            "externalDnsTable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InlineDnsTable,
            ExternalDnsTable,
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
                            "inlineDnsTable" | "inline_dns_table" => Ok(GeneratedField::InlineDnsTable),
                            "externalDnsTable" | "external_dns_table" => Ok(GeneratedField::ExternalDnsTable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_filter_config::ServerContextConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ServerContextConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_filter_config::ServerContextConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InlineDnsTable => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineDnsTable"));
                            }
                            config_source__ = map.next_value::<::std::option::Option<_>>()?.map(dns_filter_config::server_context_config::ConfigSource::InlineDnsTable)
;
                        }
                        GeneratedField::ExternalDnsTable => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalDnsTable"));
                            }
                            config_source__ = map.next_value::<::std::option::Option<_>>()?.map(dns_filter_config::server_context_config::ConfigSource::ExternalDnsTable)
;
                        }
                    }
                }
                Ok(dns_filter_config::ServerContextConfig {
                    config_source: config_source__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ServerContextConfig", FIELDS, GeneratedVisitor)
    }
}
