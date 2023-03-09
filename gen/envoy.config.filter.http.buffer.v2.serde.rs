// @generated
impl serde::Serialize for Buffer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_request_bytes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.buffer.v2.Buffer", len)?;
        if let Some(v) = self.max_request_bytes.as_ref() {
            struct_ser.serialize_field("maxRequestBytes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Buffer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_request_bytes",
            "maxRequestBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxRequestBytes,
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
                            "maxRequestBytes" | "max_request_bytes" => Ok(GeneratedField::MaxRequestBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Buffer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.buffer.v2.Buffer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Buffer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_request_bytes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxRequestBytes => {
                            if max_request_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestBytes"));
                            }
                            max_request_bytes__ = map.next_value()?;
                        }
                    }
                }
                Ok(Buffer {
                    max_request_bytes: max_request_bytes__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.buffer.v2.Buffer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BufferPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.buffer.v2.BufferPerRoute", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                buffer_per_route::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                buffer_per_route::Override::Buffer(v) => {
                    struct_ser.serialize_field("buffer", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BufferPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "buffer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            Buffer,
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
                            "disabled" => Ok(GeneratedField::Disabled),
                            "buffer" => Ok(GeneratedField::Buffer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BufferPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.buffer.v2.BufferPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BufferPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#override__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(buffer_per_route::Override::Disabled);
                        }
                        GeneratedField::Buffer => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buffer"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(buffer_per_route::Override::Buffer)
;
                        }
                    }
                }
                Ok(BufferPerRoute {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.buffer.v2.BufferPerRoute", FIELDS, GeneratedVisitor)
    }
}
