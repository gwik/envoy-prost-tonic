// @generated
impl serde::Serialize for Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transport_socket.is_some() {
            len += 1;
        }
        if self.update_period.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tcp_stats.v3.Config", len)?;
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transportSocket", v)?;
        }
        if let Some(v) = self.update_period.as_ref() {
            struct_ser.serialize_field("updatePeriod", v)?;
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
            "transport_socket",
            "transportSocket",
            "update_period",
            "updatePeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransportSocket,
            UpdatePeriod,
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
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
                            "updatePeriod" | "update_period" => Ok(GeneratedField::UpdatePeriod),
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
                formatter.write_str("struct envoy.extensions.transport_sockets.tcp_stats.v3.Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport_socket__ = None;
                let mut update_period__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map.next_value()?;
                        }
                        GeneratedField::UpdatePeriod => {
                            if update_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatePeriod"));
                            }
                            update_period__ = map.next_value()?;
                        }
                    }
                }
                Ok(Config {
                    transport_socket: transport_socket__,
                    update_period: update_period__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tcp_stats.v3.Config", FIELDS, GeneratedVisitor)
    }
}
