// @generated
impl serde::Serialize for Zstd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.compression_level.is_some() {
            len += 1;
        }
        if self.enable_checksum {
            len += 1;
        }
        if self.strategy != 0 {
            len += 1;
        }
        if self.dictionary.is_some() {
            len += 1;
        }
        if self.chunk_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.compression.zstd.compressor.v3.Zstd", len)?;
        if let Some(v) = self.compression_level.as_ref() {
            struct_ser.serialize_field("compressionLevel", v)?;
        }
        if self.enable_checksum {
            struct_ser.serialize_field("enableChecksum", &self.enable_checksum)?;
        }
        if self.strategy != 0 {
            let v = zstd::Strategy::from_i32(self.strategy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.strategy)))?;
            struct_ser.serialize_field("strategy", &v)?;
        }
        if let Some(v) = self.dictionary.as_ref() {
            struct_ser.serialize_field("dictionary", v)?;
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
            "compression_level",
            "compressionLevel",
            "enable_checksum",
            "enableChecksum",
            "strategy",
            "dictionary",
            "chunk_size",
            "chunkSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CompressionLevel,
            EnableChecksum,
            Strategy,
            Dictionary,
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
                            "compressionLevel" | "compression_level" => Ok(GeneratedField::CompressionLevel),
                            "enableChecksum" | "enable_checksum" => Ok(GeneratedField::EnableChecksum),
                            "strategy" => Ok(GeneratedField::Strategy),
                            "dictionary" => Ok(GeneratedField::Dictionary),
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
                formatter.write_str("struct envoy.extensions.compression.zstd.compressor.v3.Zstd")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Zstd, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut compression_level__ = None;
                let mut enable_checksum__ = None;
                let mut strategy__ = None;
                let mut dictionary__ = None;
                let mut chunk_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CompressionLevel => {
                            if compression_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressionLevel"));
                            }
                            compression_level__ = map.next_value()?;
                        }
                        GeneratedField::EnableChecksum => {
                            if enable_checksum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableChecksum"));
                            }
                            enable_checksum__ = Some(map.next_value()?);
                        }
                        GeneratedField::Strategy => {
                            if strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strategy"));
                            }
                            strategy__ = Some(map.next_value::<zstd::Strategy>()? as i32);
                        }
                        GeneratedField::Dictionary => {
                            if dictionary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dictionary"));
                            }
                            dictionary__ = map.next_value()?;
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
                    compression_level: compression_level__,
                    enable_checksum: enable_checksum__.unwrap_or_default(),
                    strategy: strategy__.unwrap_or_default(),
                    dictionary: dictionary__,
                    chunk_size: chunk_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.compression.zstd.compressor.v3.Zstd", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for zstd::Strategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::Fast => "FAST",
            Self::Dfast => "DFAST",
            Self::Greedy => "GREEDY",
            Self::Lazy => "LAZY",
            Self::Lazy2 => "LAZY2",
            Self::Btlazy2 => "BTLAZY2",
            Self::Btopt => "BTOPT",
            Self::Btultra => "BTULTRA",
            Self::Btultra2 => "BTULTRA2",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for zstd::Strategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "FAST",
            "DFAST",
            "GREEDY",
            "LAZY",
            "LAZY2",
            "BTLAZY2",
            "BTOPT",
            "BTULTRA",
            "BTULTRA2",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = zstd::Strategy;

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
                    .and_then(zstd::Strategy::from_i32)
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
                    .and_then(zstd::Strategy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(zstd::Strategy::Default),
                    "FAST" => Ok(zstd::Strategy::Fast),
                    "DFAST" => Ok(zstd::Strategy::Dfast),
                    "GREEDY" => Ok(zstd::Strategy::Greedy),
                    "LAZY" => Ok(zstd::Strategy::Lazy),
                    "LAZY2" => Ok(zstd::Strategy::Lazy2),
                    "BTLAZY2" => Ok(zstd::Strategy::Btlazy2),
                    "BTOPT" => Ok(zstd::Strategy::Btopt),
                    "BTULTRA" => Ok(zstd::Strategy::Btultra),
                    "BTULTRA2" => Ok(zstd::Strategy::Btultra2),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
