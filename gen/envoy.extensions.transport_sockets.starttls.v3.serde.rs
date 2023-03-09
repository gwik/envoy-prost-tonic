// @generated
impl serde::Serialize for StartTlsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cleartext_socket_config.is_some() {
            len += 1;
        }
        if self.tls_socket_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.starttls.v3.StartTlsConfig", len)?;
        if let Some(v) = self.cleartext_socket_config.as_ref() {
            struct_ser.serialize_field("cleartextSocketConfig", v)?;
        }
        if let Some(v) = self.tls_socket_config.as_ref() {
            struct_ser.serialize_field("tlsSocketConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartTlsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cleartext_socket_config",
            "cleartextSocketConfig",
            "tls_socket_config",
            "tlsSocketConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CleartextSocketConfig,
            TlsSocketConfig,
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
                            "cleartextSocketConfig" | "cleartext_socket_config" => Ok(GeneratedField::CleartextSocketConfig),
                            "tlsSocketConfig" | "tls_socket_config" => Ok(GeneratedField::TlsSocketConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StartTlsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.starttls.v3.StartTlsConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StartTlsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cleartext_socket_config__ = None;
                let mut tls_socket_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CleartextSocketConfig => {
                            if cleartext_socket_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cleartextSocketConfig"));
                            }
                            cleartext_socket_config__ = map.next_value()?;
                        }
                        GeneratedField::TlsSocketConfig => {
                            if tls_socket_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsSocketConfig"));
                            }
                            tls_socket_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(StartTlsConfig {
                    cleartext_socket_config: cleartext_socket_config__,
                    tls_socket_config: tls_socket_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.starttls.v3.StartTlsConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamStartTlsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cleartext_socket_config.is_some() {
            len += 1;
        }
        if self.tls_socket_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.starttls.v3.UpstreamStartTlsConfig", len)?;
        if let Some(v) = self.cleartext_socket_config.as_ref() {
            struct_ser.serialize_field("cleartextSocketConfig", v)?;
        }
        if let Some(v) = self.tls_socket_config.as_ref() {
            struct_ser.serialize_field("tlsSocketConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamStartTlsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cleartext_socket_config",
            "cleartextSocketConfig",
            "tls_socket_config",
            "tlsSocketConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CleartextSocketConfig,
            TlsSocketConfig,
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
                            "cleartextSocketConfig" | "cleartext_socket_config" => Ok(GeneratedField::CleartextSocketConfig),
                            "tlsSocketConfig" | "tls_socket_config" => Ok(GeneratedField::TlsSocketConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamStartTlsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.starttls.v3.UpstreamStartTlsConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamStartTlsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cleartext_socket_config__ = None;
                let mut tls_socket_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CleartextSocketConfig => {
                            if cleartext_socket_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cleartextSocketConfig"));
                            }
                            cleartext_socket_config__ = map.next_value()?;
                        }
                        GeneratedField::TlsSocketConfig => {
                            if tls_socket_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsSocketConfig"));
                            }
                            tls_socket_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpstreamStartTlsConfig {
                    cleartext_socket_config: cleartext_socket_config__,
                    tls_socket_config: tls_socket_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.starttls.v3.UpstreamStartTlsConfig", FIELDS, GeneratedVisitor)
    }
}
