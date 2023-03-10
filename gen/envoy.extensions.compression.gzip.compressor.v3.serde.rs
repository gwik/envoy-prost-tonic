// @generated
impl serde::Serialize for Gzip {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.memory_level.is_some() {
            len += 1;
        }
        if self.compression_level != 0 {
            len += 1;
        }
        if self.compression_strategy != 0 {
            len += 1;
        }
        if self.window_bits.is_some() {
            len += 1;
        }
        if self.chunk_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("envoy.extensions.compression.gzip.compressor.v3.Gzip", len)?;
        if let Some(v) = self.memory_level.as_ref() {
            struct_ser.serialize_field("memoryLevel", v)?;
        }
        if self.compression_level != 0 {
            let v = gzip_::CompressionLevel::from_i32(self.compression_level).ok_or_else(|| {
                serde::ser::Error::custom(format!("Invalid variant {}", self.compression_level))
            })?;
            struct_ser.serialize_field("compressionLevel", &v)?;
        }
        if self.compression_strategy != 0 {
            let v = gzip_::CompressionStrategy::from_i32(self.compression_strategy).ok_or_else(
                || {
                    serde::ser::Error::custom(format!(
                        "Invalid variant {}",
                        self.compression_strategy
                    ))
                },
            )?;
            struct_ser.serialize_field("compressionStrategy", &v)?;
        }
        if let Some(v) = self.window_bits.as_ref() {
            struct_ser.serialize_field("windowBits", v)?;
        }
        if let Some(v) = self.chunk_size.as_ref() {
            struct_ser.serialize_field("chunkSize", v)?;
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
            "memory_level",
            "memoryLevel",
            "compression_level",
            "compressionLevel",
            "compression_strategy",
            "compressionStrategy",
            "window_bits",
            "windowBits",
            "chunk_size",
            "chunkSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MemoryLevel,
            CompressionLevel,
            CompressionStrategy,
            WindowBits,
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

                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "memoryLevel" | "memory_level" => Ok(GeneratedField::MemoryLevel),
                            "compressionLevel" | "compression_level" => {
                                Ok(GeneratedField::CompressionLevel)
                            }
                            "compressionStrategy" | "compression_strategy" => {
                                Ok(GeneratedField::CompressionStrategy)
                            }
                            "windowBits" | "window_bits" => Ok(GeneratedField::WindowBits),
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
            type Value = Gzip;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.compression.gzip.compressor.v3.Gzip")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Gzip, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut memory_level__ = None;
                let mut compression_level__ = None;
                let mut compression_strategy__ = None;
                let mut window_bits__ = None;
                let mut chunk_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MemoryLevel => {
                            if memory_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryLevel"));
                            }
                            memory_level__ = map.next_value()?;
                        }
                        GeneratedField::CompressionLevel => {
                            if compression_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressionLevel"));
                            }
                            compression_level__ =
                                Some(map.next_value::<gzip_::CompressionLevel>()? as i32);
                        }
                        GeneratedField::CompressionStrategy => {
                            if compression_strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "compressionStrategy",
                                ));
                            }
                            compression_strategy__ =
                                Some(map.next_value::<gzip_::CompressionStrategy>()? as i32);
                        }
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
                    }
                }
                Ok(Gzip {
                    memory_level: memory_level__,
                    compression_level: compression_level__.unwrap_or_default(),
                    compression_strategy: compression_strategy__.unwrap_or_default(),
                    window_bits: window_bits__,
                    chunk_size: chunk_size__,
                })
            }
        }
        deserializer.deserialize_struct(
            "envoy.extensions.compression.gzip.compressor.v3.Gzip",
            FIELDS,
            GeneratedVisitor,
        )
    }
}
impl serde::Serialize for gzip_::CompressionLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DefaultCompression => "DEFAULT_COMPRESSION",
            Self::BestSpeed => "BEST_SPEED",
            Self::CompressionLevel2 => "COMPRESSION_LEVEL_2",
            Self::CompressionLevel3 => "COMPRESSION_LEVEL_3",
            Self::CompressionLevel4 => "COMPRESSION_LEVEL_4",
            Self::CompressionLevel5 => "COMPRESSION_LEVEL_5",
            Self::CompressionLevel6 => "COMPRESSION_LEVEL_6",
            Self::CompressionLevel7 => "COMPRESSION_LEVEL_7",
            Self::CompressionLevel8 => "COMPRESSION_LEVEL_8",
            Self::CompressionLevel9 => "COMPRESSION_LEVEL_9",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for gzip_::CompressionLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT_COMPRESSION",
            "BEST_SPEED",
            "COMPRESSION_LEVEL_1",
            "COMPRESSION_LEVEL_2",
            "COMPRESSION_LEVEL_3",
            "COMPRESSION_LEVEL_4",
            "COMPRESSION_LEVEL_5",
            "COMPRESSION_LEVEL_6",
            "COMPRESSION_LEVEL_7",
            "COMPRESSION_LEVEL_8",
            "COMPRESSION_LEVEL_9",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = gzip_::CompressionLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(gzip_::CompressionLevel::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(gzip_::CompressionLevel::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT_COMPRESSION" => Ok(gzip_::CompressionLevel::DefaultCompression),
                    "BEST_SPEED" => Ok(gzip_::CompressionLevel::BestSpeed),
                    "COMPRESSION_LEVEL_2" => Ok(gzip_::CompressionLevel::CompressionLevel2),
                    "COMPRESSION_LEVEL_3" => Ok(gzip_::CompressionLevel::CompressionLevel3),
                    "COMPRESSION_LEVEL_4" => Ok(gzip_::CompressionLevel::CompressionLevel4),
                    "COMPRESSION_LEVEL_5" => Ok(gzip_::CompressionLevel::CompressionLevel5),
                    "COMPRESSION_LEVEL_6" => Ok(gzip_::CompressionLevel::CompressionLevel6),
                    "COMPRESSION_LEVEL_7" => Ok(gzip_::CompressionLevel::CompressionLevel7),
                    "COMPRESSION_LEVEL_8" => Ok(gzip_::CompressionLevel::CompressionLevel8),
                    "COMPRESSION_LEVEL_9" => Ok(gzip_::CompressionLevel::CompressionLevel9),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for gzip_::CompressionStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DefaultStrategy => "DEFAULT_STRATEGY",
            Self::Filtered => "FILTERED",
            Self::HuffmanOnly => "HUFFMAN_ONLY",
            Self::Rle => "RLE",
            Self::Fixed => "FIXED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for gzip_::CompressionStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT_STRATEGY",
            "FILTERED",
            "HUFFMAN_ONLY",
            "RLE",
            "FIXED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = gzip_::CompressionStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(gzip_::CompressionStrategy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(gzip_::CompressionStrategy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT_STRATEGY" => Ok(gzip_::CompressionStrategy::DefaultStrategy),
                    "FILTERED" => Ok(gzip_::CompressionStrategy::Filtered),
                    "HUFFMAN_ONLY" => Ok(gzip_::CompressionStrategy::HuffmanOnly),
                    "RLE" => Ok(gzip_::CompressionStrategy::Rle),
                    "FIXED" => Ok(gzip_::CompressionStrategy::Fixed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
