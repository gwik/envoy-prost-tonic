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
        if self.content_length.is_some() {
            len += 1;
        }
        if self.compression_level != 0 {
            len += 1;
        }
        if self.compression_strategy != 0 {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        if self.disable_on_etag_header {
            len += 1;
        }
        if self.remove_accept_encoding_header {
            len += 1;
        }
        if self.window_bits.is_some() {
            len += 1;
        }
        if self.compressor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.gzip.v2.Gzip", len)?;
        if let Some(v) = self.memory_level.as_ref() {
            struct_ser.serialize_field("memoryLevel", v)?;
        }
        if let Some(v) = self.content_length.as_ref() {
            struct_ser.serialize_field("contentLength", v)?;
        }
        if self.compression_level != 0 {
            let v = gzip::compression_level::Enum::from_i32(self.compression_level)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.compression_level)))?;
            struct_ser.serialize_field("compressionLevel", &v)?;
        }
        if self.compression_strategy != 0 {
            let v = gzip::CompressionStrategy::from_i32(self.compression_strategy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.compression_strategy)))?;
            struct_ser.serialize_field("compressionStrategy", &v)?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if self.disable_on_etag_header {
            struct_ser.serialize_field("disableOnEtagHeader", &self.disable_on_etag_header)?;
        }
        if self.remove_accept_encoding_header {
            struct_ser.serialize_field("removeAcceptEncodingHeader", &self.remove_accept_encoding_header)?;
        }
        if let Some(v) = self.window_bits.as_ref() {
            struct_ser.serialize_field("windowBits", v)?;
        }
        if let Some(v) = self.compressor.as_ref() {
            struct_ser.serialize_field("compressor", v)?;
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
            "content_length",
            "contentLength",
            "compression_level",
            "compressionLevel",
            "compression_strategy",
            "compressionStrategy",
            "content_type",
            "contentType",
            "disable_on_etag_header",
            "disableOnEtagHeader",
            "remove_accept_encoding_header",
            "removeAcceptEncodingHeader",
            "window_bits",
            "windowBits",
            "compressor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MemoryLevel,
            ContentLength,
            CompressionLevel,
            CompressionStrategy,
            ContentType,
            DisableOnEtagHeader,
            RemoveAcceptEncodingHeader,
            WindowBits,
            Compressor,
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
                            "memoryLevel" | "memory_level" => Ok(GeneratedField::MemoryLevel),
                            "contentLength" | "content_length" => Ok(GeneratedField::ContentLength),
                            "compressionLevel" | "compression_level" => Ok(GeneratedField::CompressionLevel),
                            "compressionStrategy" | "compression_strategy" => Ok(GeneratedField::CompressionStrategy),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "disableOnEtagHeader" | "disable_on_etag_header" => Ok(GeneratedField::DisableOnEtagHeader),
                            "removeAcceptEncodingHeader" | "remove_accept_encoding_header" => Ok(GeneratedField::RemoveAcceptEncodingHeader),
                            "windowBits" | "window_bits" => Ok(GeneratedField::WindowBits),
                            "compressor" => Ok(GeneratedField::Compressor),
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
                formatter.write_str("struct envoy.config.filter.http.gzip.v2.Gzip")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Gzip, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut memory_level__ = None;
                let mut content_length__ = None;
                let mut compression_level__ = None;
                let mut compression_strategy__ = None;
                let mut content_type__ = None;
                let mut disable_on_etag_header__ = None;
                let mut remove_accept_encoding_header__ = None;
                let mut window_bits__ = None;
                let mut compressor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MemoryLevel => {
                            if memory_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryLevel"));
                            }
                            memory_level__ = map.next_value()?;
                        }
                        GeneratedField::ContentLength => {
                            if content_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentLength"));
                            }
                            content_length__ = map.next_value()?;
                        }
                        GeneratedField::CompressionLevel => {
                            if compression_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressionLevel"));
                            }
                            compression_level__ = Some(map.next_value::<gzip::compression_level::Enum>()? as i32);
                        }
                        GeneratedField::CompressionStrategy => {
                            if compression_strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressionStrategy"));
                            }
                            compression_strategy__ = Some(map.next_value::<gzip::CompressionStrategy>()? as i32);
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisableOnEtagHeader => {
                            if disable_on_etag_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableOnEtagHeader"));
                            }
                            disable_on_etag_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoveAcceptEncodingHeader => {
                            if remove_accept_encoding_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeAcceptEncodingHeader"));
                            }
                            remove_accept_encoding_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::WindowBits => {
                            if window_bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowBits"));
                            }
                            window_bits__ = map.next_value()?;
                        }
                        GeneratedField::Compressor => {
                            if compressor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressor"));
                            }
                            compressor__ = map.next_value()?;
                        }
                    }
                }
                Ok(Gzip {
                    memory_level: memory_level__,
                    content_length: content_length__,
                    compression_level: compression_level__.unwrap_or_default(),
                    compression_strategy: compression_strategy__.unwrap_or_default(),
                    content_type: content_type__.unwrap_or_default(),
                    disable_on_etag_header: disable_on_etag_header__.unwrap_or_default(),
                    remove_accept_encoding_header: remove_accept_encoding_header__.unwrap_or_default(),
                    window_bits: window_bits__,
                    compressor: compressor__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.gzip.v2.Gzip", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for gzip::CompressionLevel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.filter.http.gzip.v2.Gzip.CompressionLevel", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for gzip::CompressionLevel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = gzip::CompressionLevel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.gzip.v2.Gzip.CompressionLevel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<gzip::CompressionLevel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(gzip::CompressionLevel {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.gzip.v2.Gzip.CompressionLevel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for gzip::compression_level::Enum {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::Best => "BEST",
            Self::Speed => "SPEED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for gzip::compression_level::Enum {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "BEST",
            "SPEED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = gzip::compression_level::Enum;

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
                    .and_then(gzip::compression_level::Enum::from_i32)
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
                    .and_then(gzip::compression_level::Enum::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(gzip::compression_level::Enum::Default),
                    "BEST" => Ok(gzip::compression_level::Enum::Best),
                    "SPEED" => Ok(gzip::compression_level::Enum::Speed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for gzip::CompressionStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::Filtered => "FILTERED",
            Self::Huffman => "HUFFMAN",
            Self::Rle => "RLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for gzip::CompressionStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "FILTERED",
            "HUFFMAN",
            "RLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = gzip::CompressionStrategy;

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
                    .and_then(gzip::CompressionStrategy::from_i32)
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
                    .and_then(gzip::CompressionStrategy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(gzip::CompressionStrategy::Default),
                    "FILTERED" => Ok(gzip::CompressionStrategy::Filtered),
                    "HUFFMAN" => Ok(gzip::CompressionStrategy::Huffman),
                    "RLE" => Ok(gzip::CompressionStrategy::Rle),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
