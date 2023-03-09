// @generated
impl serde::Serialize for CryptoMbPrivateKeyMethodConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.private_key.is_some() {
            len += 1;
        }
        if self.poll_delay.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.private_key_providers.cryptomb.v3alpha.CryptoMbPrivateKeyMethodConfig", len)?;
        if let Some(v) = self.private_key.as_ref() {
            struct_ser.serialize_field("privateKey", v)?;
        }
        if let Some(v) = self.poll_delay.as_ref() {
            struct_ser.serialize_field("pollDelay", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CryptoMbPrivateKeyMethodConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "private_key",
            "privateKey",
            "poll_delay",
            "pollDelay",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrivateKey,
            PollDelay,
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
                            "privateKey" | "private_key" => Ok(GeneratedField::PrivateKey),
                            "pollDelay" | "poll_delay" => Ok(GeneratedField::PollDelay),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CryptoMbPrivateKeyMethodConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.private_key_providers.cryptomb.v3alpha.CryptoMbPrivateKeyMethodConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CryptoMbPrivateKeyMethodConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut private_key__ = None;
                let mut poll_delay__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PrivateKey => {
                            if private_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKey"));
                            }
                            private_key__ = map.next_value()?;
                        }
                        GeneratedField::PollDelay => {
                            if poll_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pollDelay"));
                            }
                            poll_delay__ = map.next_value()?;
                        }
                    }
                }
                Ok(CryptoMbPrivateKeyMethodConfig {
                    private_key: private_key__,
                    poll_delay: poll_delay__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.private_key_providers.cryptomb.v3alpha.CryptoMbPrivateKeyMethodConfig", FIELDS, GeneratedVisitor)
    }
}
