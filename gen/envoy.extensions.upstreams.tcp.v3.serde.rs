// @generated
impl serde::Serialize for TcpProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.idle_timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.upstreams.tcp.v3.TcpProtocolOptions", len)?;
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TcpProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idle_timeout",
            "idleTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdleTimeout,
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
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TcpProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.upstreams.tcp.v3.TcpProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TcpProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idle_timeout__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map.next_value()?;
                        }
                    }
                }
                Ok(TcpProtocolOptions {
                    idle_timeout: idle_timeout__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.upstreams.tcp.v3.TcpProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
