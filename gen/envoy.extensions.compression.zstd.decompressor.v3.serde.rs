// @generated
impl serde::Serialize for Zstd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.dictionaries.is_empty() {
            len += 1;
        }
        if self.chunk_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.compression.zstd.decompressor.v3.Zstd", len)?;
        if !self.dictionaries.is_empty() {
            struct_ser.serialize_field("dictionaries", &self.dictionaries)?;
        }
        if let Some(v) = self.chunk_size.as_ref() {
            struct_ser.serialize_field("chunkSize", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Zstd {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dictionaries",
            "chunk_size",
            "chunkSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dictionaries,
            ChunkSize,
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
                            "dictionaries" => Ok(GeneratedField::Dictionaries),
                            "chunkSize" | "chunk_size" => Ok(GeneratedField::ChunkSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Zstd;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.compression.zstd.decompressor.v3.Zstd")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Zstd, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dictionaries__ = None;
                let mut chunk_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Dictionaries => {
                            if dictionaries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dictionaries"));
                            }
                            dictionaries__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChunkSize => {
                            if chunk_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkSize"));
                            }
                            chunk_size__ = map.next_value()?;
                        }
                    }
                }
                Ok(Zstd {
                    dictionaries: dictionaries__.unwrap_or_default(),
                    chunk_size: chunk_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.compression.zstd.decompressor.v3.Zstd", FIELDS, GeneratedVisitor)
    }
}
