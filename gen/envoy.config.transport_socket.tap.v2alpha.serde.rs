// @generated
impl serde::Serialize for Tap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_config.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.transport_socket.tap.v2alpha.Tap", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transportSocket", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_config",
            "commonConfig",
            "transport_socket",
            "transportSocket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonConfig,
            TransportSocket,
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
                            "commonConfig" | "common_config" => Ok(GeneratedField::CommonConfig),
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.transport_socket.tap.v2alpha.Tap")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Tap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_config__ = None;
                let mut transport_socket__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonConfig => {
                            if common_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonConfig"));
                            }
                            common_config__ = map.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map.next_value()?;
                        }
                    }
                }
                Ok(Tap {
                    common_config: common_config__,
                    transport_socket: transport_socket__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.transport_socket.tap.v2alpha.Tap", FIELDS, GeneratedVisitor)
    }
}
