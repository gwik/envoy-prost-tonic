// @generated
impl serde::Serialize for QuicDownstreamTransport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.downstream_tls_context.is_some() {
            len += 1;
        }
        if self.enable_early_data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.quic.v3.QuicDownstreamTransport", len)?;
        if let Some(v) = self.downstream_tls_context.as_ref() {
            struct_ser.serialize_field("downstreamTlsContext", v)?;
        }
        if let Some(v) = self.enable_early_data.as_ref() {
            struct_ser.serialize_field("enableEarlyData", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuicDownstreamTransport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "downstream_tls_context",
            "downstreamTlsContext",
            "enable_early_data",
            "enableEarlyData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DownstreamTlsContext,
            EnableEarlyData,
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
                            "downstreamTlsContext" | "downstream_tls_context" => Ok(GeneratedField::DownstreamTlsContext),
                            "enableEarlyData" | "enable_early_data" => Ok(GeneratedField::EnableEarlyData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuicDownstreamTransport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.quic.v3.QuicDownstreamTransport")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuicDownstreamTransport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut downstream_tls_context__ = None;
                let mut enable_early_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DownstreamTlsContext => {
                            if downstream_tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamTlsContext"));
                            }
                            downstream_tls_context__ = map.next_value()?;
                        }
                        GeneratedField::EnableEarlyData => {
                            if enable_early_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableEarlyData"));
                            }
                            enable_early_data__ = map.next_value()?;
                        }
                    }
                }
                Ok(QuicDownstreamTransport {
                    downstream_tls_context: downstream_tls_context__,
                    enable_early_data: enable_early_data__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.quic.v3.QuicDownstreamTransport", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuicUpstreamTransport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.upstream_tls_context.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.quic.v3.QuicUpstreamTransport", len)?;
        if let Some(v) = self.upstream_tls_context.as_ref() {
            struct_ser.serialize_field("upstreamTlsContext", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuicUpstreamTransport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upstream_tls_context",
            "upstreamTlsContext",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpstreamTlsContext,
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
                            "upstreamTlsContext" | "upstream_tls_context" => Ok(GeneratedField::UpstreamTlsContext),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuicUpstreamTransport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.quic.v3.QuicUpstreamTransport")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuicUpstreamTransport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upstream_tls_context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UpstreamTlsContext => {
                            if upstream_tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamTlsContext"));
                            }
                            upstream_tls_context__ = map.next_value()?;
                        }
                    }
                }
                Ok(QuicUpstreamTransport {
                    upstream_tls_context: upstream_tls_context__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.quic.v3.QuicUpstreamTransport", FIELDS, GeneratedVisitor)
    }
}
