// @generated
impl serde::Serialize for Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metadata_namespace.is_empty() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.set_metadata.v3.Config", len)?;
        if !self.metadata_namespace.is_empty() {
            struct_ser.serialize_field("metadataNamespace", &self.metadata_namespace)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_namespace",
            "metadataNamespace",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataNamespace,
            Value,
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
                            "metadataNamespace" | "metadata_namespace" => Ok(GeneratedField::MetadataNamespace),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.set_metadata.v3.Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_namespace__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MetadataNamespace => {
                            if metadata_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataNamespace"));
                            }
                            metadata_namespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(Config {
                    metadata_namespace: metadata_namespace__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.set_metadata.v3.Config", FIELDS, GeneratedVisitor)
    }
}
