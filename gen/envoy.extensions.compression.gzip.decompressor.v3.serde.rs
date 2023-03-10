// @generated
impl serde::Serialize for Gzip {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.window_bits.is_some() {
            len += 1;
        }
        if self.chunk_size.is_some() {
            len += 1;
        }
        if self.max_inflate_ratio.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.compression.gzip.decompressor.v3.Gzip", len)?;
        if let Some(v) = self.window_bits.as_ref() {
            struct_ser.serialize_field("windowBits", v)?;
        }
        if let Some(v) = self.chunk_size.as_ref() {
            struct_ser.serialize_field("chunkSize", v)?;
        }
        if let Some(v) = self.max_inflate_ratio.as_ref() {
            struct_ser.serialize_field("maxInflateRatio", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Gzip {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "window_bits",
            "windowBits",
            "chunk_size",
            "chunkSize",
            "max_inflate_ratio",
            "maxInflateRatio",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WindowBits,
            ChunkSize,
            MaxInflateRatio,
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
                            "windowBits" | "window_bits" => Ok(GeneratedField::WindowBits),
                            "chunkSize" | "chunk_size" => Ok(GeneratedField::ChunkSize),
                            "maxInflateRatio" | "max_inflate_ratio" => Ok(GeneratedField::MaxInflateRatio),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Gzip;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.compression.gzip.decompressor.v3.Gzip")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Gzip, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut window_bits__ = None;
                let mut chunk_size__ = None;
                let mut max_inflate_ratio__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WindowBits => {
                            if window_bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowBits"));
                            }
                            window_bits__ = map.next_value()?;
                        }
                        GeneratedField::ChunkSize => {
                            if chunk_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkSize"));
                            }
                            chunk_size__ = map.next_value()?;
                        }
                        GeneratedField::MaxInflateRatio => {
                            if max_inflate_ratio__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInflateRatio"));
                            }
                            max_inflate_ratio__ = map.next_value()?;
                        }
                    }
                }
                Ok(Gzip {
                    window_bits: window_bits__,
                    chunk_size: chunk_size__,
                    max_inflate_ratio: max_inflate_ratio__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.compression.gzip.decompressor.v3.Gzip", FIELDS, GeneratedVisitor)
    }
}
