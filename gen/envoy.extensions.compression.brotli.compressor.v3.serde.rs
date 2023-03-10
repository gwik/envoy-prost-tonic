// @generated
impl serde::Serialize for Brotli {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.quality.is_some() {
            len += 1;
        }
        if self.encoder_mode != 0 {
            len += 1;
        }
        if self.window_bits.is_some() {
            len += 1;
        }
        if self.input_block_bits.is_some() {
            len += 1;
        }
        if self.chunk_size.is_some() {
            len += 1;
        }
        if self.disable_literal_context_modeling {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.compression.brotli.compressor.v3.Brotli", len)?;
        if let Some(v) = self.quality.as_ref() {
            struct_ser.serialize_field("quality", v)?;
        }
        if self.encoder_mode != 0 {
            let v = brotli::EncoderMode::from_i32(self.encoder_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.encoder_mode)))?;
            struct_ser.serialize_field("encoderMode", &v)?;
        }
        if let Some(v) = self.window_bits.as_ref() {
            struct_ser.serialize_field("windowBits", v)?;
        }
        if let Some(v) = self.input_block_bits.as_ref() {
            struct_ser.serialize_field("inputBlockBits", v)?;
        }
        if let Some(v) = self.chunk_size.as_ref() {
            struct_ser.serialize_field("chunkSize", v)?;
        }
        if self.disable_literal_context_modeling {
            struct_ser.serialize_field("disableLiteralContextModeling", &self.disable_literal_context_modeling)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Brotli {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quality",
            "encoder_mode",
            "encoderMode",
            "window_bits",
            "windowBits",
            "input_block_bits",
            "inputBlockBits",
            "chunk_size",
            "chunkSize",
            "disable_literal_context_modeling",
            "disableLiteralContextModeling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Quality,
            EncoderMode,
            WindowBits,
            InputBlockBits,
            ChunkSize,
            DisableLiteralContextModeling,
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
                            "quality" => Ok(GeneratedField::Quality),
                            "encoderMode" | "encoder_mode" => Ok(GeneratedField::EncoderMode),
                            "windowBits" | "window_bits" => Ok(GeneratedField::WindowBits),
                            "inputBlockBits" | "input_block_bits" => Ok(GeneratedField::InputBlockBits),
                            "chunkSize" | "chunk_size" => Ok(GeneratedField::ChunkSize),
                            "disableLiteralContextModeling" | "disable_literal_context_modeling" => Ok(GeneratedField::DisableLiteralContextModeling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Brotli;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.compression.brotli.compressor.v3.Brotli")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Brotli, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quality__ = None;
                let mut encoder_mode__ = None;
                let mut window_bits__ = None;
                let mut input_block_bits__ = None;
                let mut chunk_size__ = None;
                let mut disable_literal_context_modeling__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Quality => {
                            if quality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quality"));
                            }
                            quality__ = map.next_value()?;
                        }
                        GeneratedField::EncoderMode => {
                            if encoder_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encoderMode"));
                            }
                            encoder_mode__ = Some(map.next_value::<brotli::EncoderMode>()? as i32);
                        }
                        GeneratedField::WindowBits => {
                            if window_bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowBits"));
                            }
                            window_bits__ = map.next_value()?;
                        }
                        GeneratedField::InputBlockBits => {
                            if input_block_bits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputBlockBits"));
                            }
                            input_block_bits__ = map.next_value()?;
                        }
                        GeneratedField::ChunkSize => {
                            if chunk_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkSize"));
                            }
                            chunk_size__ = map.next_value()?;
                        }
                        GeneratedField::DisableLiteralContextModeling => {
                            if disable_literal_context_modeling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableLiteralContextModeling"));
                            }
                            disable_literal_context_modeling__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Brotli {
                    quality: quality__,
                    encoder_mode: encoder_mode__.unwrap_or_default(),
                    window_bits: window_bits__,
                    input_block_bits: input_block_bits__,
                    chunk_size: chunk_size__,
                    disable_literal_context_modeling: disable_literal_context_modeling__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.compression.brotli.compressor.v3.Brotli", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for brotli::EncoderMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::Generic => "GENERIC",
            Self::Text => "TEXT",
            Self::Font => "FONT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for brotli::EncoderMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "GENERIC",
            "TEXT",
            "FONT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = brotli::EncoderMode;

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
                    .and_then(brotli::EncoderMode::from_i32)
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
                    .and_then(brotli::EncoderMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(brotli::EncoderMode::Default),
                    "GENERIC" => Ok(brotli::EncoderMode::Generic),
                    "TEXT" => Ok(brotli::EncoderMode::Text),
                    "FONT" => Ok(brotli::EncoderMode::Font),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
