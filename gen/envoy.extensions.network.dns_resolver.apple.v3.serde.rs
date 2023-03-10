// @generated
impl serde::Serialize for AppleDnsResolverConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.include_unroutable_families {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.network.dns_resolver.apple.v3.AppleDnsResolverConfig", len)?;
        if self.include_unroutable_families {
            struct_ser.serialize_field("includeUnroutableFamilies", &self.include_unroutable_families)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AppleDnsResolverConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "include_unroutable_families",
            "includeUnroutableFamilies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IncludeUnroutableFamilies,
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
                            "includeUnroutableFamilies" | "include_unroutable_families" => Ok(GeneratedField::IncludeUnroutableFamilies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AppleDnsResolverConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.network.dns_resolver.apple.v3.AppleDnsResolverConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AppleDnsResolverConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut include_unroutable_families__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IncludeUnroutableFamilies => {
                            if include_unroutable_families__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includeUnroutableFamilies"));
                            }
                            include_unroutable_families__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AppleDnsResolverConfig {
                    include_unroutable_families: include_unroutable_families__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.network.dns_resolver.apple.v3.AppleDnsResolverConfig", FIELDS, GeneratedVisitor)
    }
}
