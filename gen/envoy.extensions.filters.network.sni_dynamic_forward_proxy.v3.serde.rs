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
        if self.port_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3.FilterConfig", len)?;
        if let Some(v) = self.dns_cache_config.as_ref() {
            struct_ser.serialize_field("dnsCacheConfig", v)?;
        }
        if let Some(v) = self.port_specifier.as_ref() {
            match v {
                filter_config::PortSpecifier::PortValue(v) => {
                    struct_ser.serialize_field("portValue", v)?;
                }
            }
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
            "port_value",
            "portValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DnsCacheConfig,
            PortValue,
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
                            "portValue" | "port_value" => Ok(GeneratedField::PortValue),
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
                formatter.write_str("struct envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3.FilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dns_cache_config__ = None;
                let mut port_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DnsCacheConfig => {
                            if dns_cache_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsCacheConfig"));
                            }
                            dns_cache_config__ = map.next_value()?;
                        }
                        GeneratedField::PortValue => {
                            if port_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portValue"));
                            }
                            port_specifier__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| filter_config::PortSpecifier::PortValue(x.0));
                        }
                    }
                }
                Ok(FilterConfig {
                    dns_cache_config: dns_cache_config__,
                    port_specifier: port_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sni_dynamic_forward_proxy.v3.FilterConfig", FIELDS, GeneratedVisitor)
    }
}
