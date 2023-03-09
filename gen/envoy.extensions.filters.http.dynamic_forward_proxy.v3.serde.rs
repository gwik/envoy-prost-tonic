// @generated
impl serde::Serialize for FilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dns_cache_config.is_some() {
            len += 1;
        }
        if self.save_upstream_address {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.dynamic_forward_proxy.v3.FilterConfig", len)?;
        if let Some(v) = self.dns_cache_config.as_ref() {
            struct_ser.serialize_field("dnsCacheConfig", v)?;
        }
        if self.save_upstream_address {
            struct_ser.serialize_field("saveUpstreamAddress", &self.save_upstream_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dns_cache_config",
            "dnsCacheConfig",
            "save_upstream_address",
            "saveUpstreamAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DnsCacheConfig,
            SaveUpstreamAddress,
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
                            "dnsCacheConfig" | "dns_cache_config" => Ok(GeneratedField::DnsCacheConfig),
                            "saveUpstreamAddress" | "save_upstream_address" => Ok(GeneratedField::SaveUpstreamAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.dynamic_forward_proxy.v3.FilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dns_cache_config__ = None;
                let mut save_upstream_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DnsCacheConfig => {
                            if dns_cache_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsCacheConfig"));
                            }
                            dns_cache_config__ = map.next_value()?;
                        }
                        GeneratedField::SaveUpstreamAddress => {
                            if save_upstream_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("saveUpstreamAddress"));
                            }
                            save_upstream_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilterConfig {
                    dns_cache_config: dns_cache_config__,
                    save_upstream_address: save_upstream_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.dynamic_forward_proxy.v3.FilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PerRouteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_rewrite_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.dynamic_forward_proxy.v3.PerRouteConfig", len)?;
        if let Some(v) = self.host_rewrite_specifier.as_ref() {
            match v {
                per_route_config::HostRewriteSpecifier::HostRewriteLiteral(v) => {
                    struct_ser.serialize_field("hostRewriteLiteral", v)?;
                }
                per_route_config::HostRewriteSpecifier::HostRewriteHeader(v) => {
                    struct_ser.serialize_field("hostRewriteHeader", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PerRouteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_rewrite_literal",
            "hostRewriteLiteral",
            "host_rewrite_header",
            "hostRewriteHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostRewriteLiteral,
            HostRewriteHeader,
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
                            "hostRewriteLiteral" | "host_rewrite_literal" => Ok(GeneratedField::HostRewriteLiteral),
                            "hostRewriteHeader" | "host_rewrite_header" => Ok(GeneratedField::HostRewriteHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PerRouteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.dynamic_forward_proxy.v3.PerRouteConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PerRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_rewrite_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostRewriteLiteral => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewriteLiteral"));
                            }
                            host_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(per_route_config::HostRewriteSpecifier::HostRewriteLiteral);
                        }
                        GeneratedField::HostRewriteHeader => {
                            if host_rewrite_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewriteHeader"));
                            }
                            host_rewrite_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(per_route_config::HostRewriteSpecifier::HostRewriteHeader);
                        }
                    }
                }
                Ok(PerRouteConfig {
                    host_rewrite_specifier: host_rewrite_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.dynamic_forward_proxy.v3.PerRouteConfig", FIELDS, GeneratedVisitor)
    }
}
