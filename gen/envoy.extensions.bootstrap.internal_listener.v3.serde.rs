// @generated
impl serde::Serialize for InternalListener {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.buffer_size_kb.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.bootstrap.internal_listener.v3.InternalListener", len)?;
        if let Some(v) = self.buffer_size_kb.as_ref() {
            struct_ser.serialize_field("bufferSizeKb", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InternalListener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "buffer_size_kb",
            "bufferSizeKb",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BufferSizeKb,
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
                            "bufferSizeKb" | "buffer_size_kb" => Ok(GeneratedField::BufferSizeKb),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InternalListener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.bootstrap.internal_listener.v3.InternalListener")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InternalListener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buffer_size_kb__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BufferSizeKb => {
                            if buffer_size_kb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferSizeKb"));
                            }
                            buffer_size_kb__ = map.next_value()?;
                        }
                    }
                }
                Ok(InternalListener {
                    buffer_size_kb: buffer_size_kb__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.bootstrap.internal_listener.v3.InternalListener", FIELDS, GeneratedVisitor)
    }
}
