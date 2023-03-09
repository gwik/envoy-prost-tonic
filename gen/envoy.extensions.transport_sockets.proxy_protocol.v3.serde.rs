// @generated
impl serde::Serialize for ProxyProtocolUpstreamTransport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.proxy_protocol.v3.ProxyProtocolUpstreamTransport", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transportSocket", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProxyProtocolUpstreamTransport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
            "transport_socket",
            "transportSocket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
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
                            "config" => Ok(GeneratedField::Config),
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
            type Value = ProxyProtocolUpstreamTransport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.proxy_protocol.v3.ProxyProtocolUpstreamTransport")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProxyProtocolUpstreamTransport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                let mut transport_socket__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map.next_value()?;
                        }
                    }
                }
                Ok(ProxyProtocolUpstreamTransport {
                    config: config__,
                    transport_socket: transport_socket__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.proxy_protocol.v3.ProxyProtocolUpstreamTransport", FIELDS, GeneratedVisitor)
    }
}
