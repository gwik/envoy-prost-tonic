// @generated
impl serde::Serialize for DownstreamConnectionsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_active_downstream_connections != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.resource_monitors.downstream_connections.v3.DownstreamConnectionsConfig", len)?;
        if self.max_active_downstream_connections != 0 {
            struct_ser.serialize_field("maxActiveDownstreamConnections", ToString::to_string(&self.max_active_downstream_connections).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DownstreamConnectionsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_active_downstream_connections",
            "maxActiveDownstreamConnections",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxActiveDownstreamConnections,
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
                            "maxActiveDownstreamConnections" | "max_active_downstream_connections" => Ok(GeneratedField::MaxActiveDownstreamConnections),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DownstreamConnectionsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.resource_monitors.downstream_connections.v3.DownstreamConnectionsConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DownstreamConnectionsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_active_downstream_connections__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxActiveDownstreamConnections => {
                            if max_active_downstream_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxActiveDownstreamConnections"));
                            }
                            max_active_downstream_connections__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DownstreamConnectionsConfig {
                    max_active_downstream_connections: max_active_downstream_connections__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.resource_monitors.downstream_connections.v3.DownstreamConnectionsConfig", FIELDS, GeneratedVisitor)
    }
}
