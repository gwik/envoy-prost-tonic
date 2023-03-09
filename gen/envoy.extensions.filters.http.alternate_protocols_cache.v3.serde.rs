// @generated
impl serde::Serialize for FilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.alternate_protocols_cache_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.alternate_protocols_cache.v3.FilterConfig", len)?;
        if let Some(v) = self.alternate_protocols_cache_options.as_ref() {
            struct_ser.serialize_field("alternateProtocolsCacheOptions", v)?;
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
            "alternate_protocols_cache_options",
            "alternateProtocolsCacheOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AlternateProtocolsCacheOptions,
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
                            "alternateProtocolsCacheOptions" | "alternate_protocols_cache_options" => Ok(GeneratedField::AlternateProtocolsCacheOptions),
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
                formatter.write_str("struct envoy.extensions.filters.http.alternate_protocols_cache.v3.FilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut alternate_protocols_cache_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AlternateProtocolsCacheOptions => {
                            if alternate_protocols_cache_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alternateProtocolsCacheOptions"));
                            }
                            alternate_protocols_cache_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(FilterConfig {
                    alternate_protocols_cache_options: alternate_protocols_cache_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.alternate_protocols_cache.v3.FilterConfig", FIELDS, GeneratedVisitor)
    }
}
