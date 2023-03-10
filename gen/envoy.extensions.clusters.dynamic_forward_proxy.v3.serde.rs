// @generated
impl serde::Serialize for ClusterConfig {
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
        if self.allow_insecure_cluster_options {
            len += 1;
        }
        if self.allow_coalesced_connections {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.clusters.dynamic_forward_proxy.v3.ClusterConfig", len)?;
        if let Some(v) = self.dns_cache_config.as_ref() {
            struct_ser.serialize_field("dnsCacheConfig", v)?;
        }
        if self.allow_insecure_cluster_options {
            struct_ser.serialize_field("allowInsecureClusterOptions", &self.allow_insecure_cluster_options)?;
        }
        if self.allow_coalesced_connections {
            struct_ser.serialize_field("allowCoalescedConnections", &self.allow_coalesced_connections)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dns_cache_config",
            "dnsCacheConfig",
            "allow_insecure_cluster_options",
            "allowInsecureClusterOptions",
            "allow_coalesced_connections",
            "allowCoalescedConnections",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DnsCacheConfig,
            AllowInsecureClusterOptions,
            AllowCoalescedConnections,
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
                            "allowInsecureClusterOptions" | "allow_insecure_cluster_options" => Ok(GeneratedField::AllowInsecureClusterOptions),
                            "allowCoalescedConnections" | "allow_coalesced_connections" => Ok(GeneratedField::AllowCoalescedConnections),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.clusters.dynamic_forward_proxy.v3.ClusterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dns_cache_config__ = None;
                let mut allow_insecure_cluster_options__ = None;
                let mut allow_coalesced_connections__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DnsCacheConfig => {
                            if dns_cache_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsCacheConfig"));
                            }
                            dns_cache_config__ = map.next_value()?;
                        }
                        GeneratedField::AllowInsecureClusterOptions => {
                            if allow_insecure_cluster_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowInsecureClusterOptions"));
                            }
                            allow_insecure_cluster_options__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowCoalescedConnections => {
                            if allow_coalesced_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowCoalescedConnections"));
                            }
                            allow_coalesced_connections__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClusterConfig {
                    dns_cache_config: dns_cache_config__,
                    allow_insecure_cluster_options: allow_insecure_cluster_options__.unwrap_or_default(),
                    allow_coalesced_connections: allow_coalesced_connections__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.clusters.dynamic_forward_proxy.v3.ClusterConfig", FIELDS, GeneratedVisitor)
    }
}
