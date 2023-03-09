// @generated
impl serde::Serialize for KeyValueStoreXdsDelegateConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key_value_store_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.config.v3alpha.KeyValueStoreXdsDelegateConfig", len)?;
        if let Some(v) = self.key_value_store_config.as_ref() {
            struct_ser.serialize_field("keyValueStoreConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyValueStoreXdsDelegateConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_value_store_config",
            "keyValueStoreConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyValueStoreConfig,
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
                            "keyValueStoreConfig" | "key_value_store_config" => Ok(GeneratedField::KeyValueStoreConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeyValueStoreXdsDelegateConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.config.v3alpha.KeyValueStoreXdsDelegateConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeyValueStoreXdsDelegateConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_value_store_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KeyValueStoreConfig => {
                            if key_value_store_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyValueStoreConfig"));
                            }
                            key_value_store_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(KeyValueStoreXdsDelegateConfig {
                    key_value_store_config: key_value_store_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.config.v3alpha.KeyValueStoreXdsDelegateConfig", FIELDS, GeneratedVisitor)
    }
}
