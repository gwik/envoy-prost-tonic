// @generated
impl serde::Serialize for InternalUpstreamTransport {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.passthrough_metadata.is_empty() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.internal_upstream.v3.InternalUpstreamTransport", len)?;
        if !self.passthrough_metadata.is_empty() {
            struct_ser.serialize_field("passthroughMetadata", &self.passthrough_metadata)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transportSocket", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InternalUpstreamTransport {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "passthrough_metadata",
            "passthroughMetadata",
            "transport_socket",
            "transportSocket",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PassthroughMetadata,
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
                            "passthroughMetadata" | "passthrough_metadata" => Ok(GeneratedField::PassthroughMetadata),
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
            type Value = InternalUpstreamTransport;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.internal_upstream.v3.InternalUpstreamTransport")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InternalUpstreamTransport, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut passthrough_metadata__ = None;
                let mut transport_socket__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PassthroughMetadata => {
                            if passthrough_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passthroughMetadata"));
                            }
                            passthrough_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map.next_value()?;
                        }
                    }
                }
                Ok(InternalUpstreamTransport {
                    passthrough_metadata: passthrough_metadata__.unwrap_or_default(),
                    transport_socket: transport_socket__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.internal_upstream.v3.InternalUpstreamTransport", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for internal_upstream_transport::MetadataValueSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.internal_upstream.v3.InternalUpstreamTransport.MetadataValueSource", len)?;
        if let Some(v) = self.kind.as_ref() {
            struct_ser.serialize_field("kind", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for internal_upstream_transport::MetadataValueSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Name,
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
                            "kind" => Ok(GeneratedField::Kind),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = internal_upstream_transport::MetadataValueSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.internal_upstream.v3.InternalUpstreamTransport.MetadataValueSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<internal_upstream_transport::MetadataValueSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = map.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(internal_upstream_transport::MetadataValueSource {
                    kind: kind__,
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.internal_upstream.v3.InternalUpstreamTransport.MetadataValueSource", FIELDS, GeneratedVisitor)
    }
}
