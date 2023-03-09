// @generated
impl serde::Serialize for OmitHostMetadataConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata_match.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.retry.host.omit_host_metadata.v3.OmitHostMetadataConfig", len)?;
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OmitHostMetadataConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_match",
            "metadataMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = OmitHostMetadataConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.retry.host.omit_host_metadata.v3.OmitHostMetadataConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OmitHostMetadataConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_match__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                    }
                }
                Ok(OmitHostMetadataConfig {
                    metadata_match: metadata_match__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.retry.host.omit_host_metadata.v3.OmitHostMetadataConfig", FIELDS, GeneratedVisitor)
    }
}
