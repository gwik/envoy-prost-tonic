// @generated
impl serde::Serialize for PreserveCaseFormatterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.forward_reason_phrase {
            len += 1;
        }
        if self.formatter_type_on_envoy_headers != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.header_formatters.preserve_case.v3.PreserveCaseFormatterConfig", len)?;
        if self.forward_reason_phrase {
            struct_ser.serialize_field("forwardReasonPhrase", &self.forward_reason_phrase)?;
        }
        if self.formatter_type_on_envoy_headers != 0 {
            let v = preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders::from_i32(self.formatter_type_on_envoy_headers)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.formatter_type_on_envoy_headers)))?;
            struct_ser.serialize_field("formatterTypeOnEnvoyHeaders", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PreserveCaseFormatterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "forward_reason_phrase",
            "forwardReasonPhrase",
            "formatter_type_on_envoy_headers",
            "formatterTypeOnEnvoyHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ForwardReasonPhrase,
            FormatterTypeOnEnvoyHeaders,
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
                            "forwardReasonPhrase" | "forward_reason_phrase" => Ok(GeneratedField::ForwardReasonPhrase),
                            "formatterTypeOnEnvoyHeaders" | "formatter_type_on_envoy_headers" => Ok(GeneratedField::FormatterTypeOnEnvoyHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PreserveCaseFormatterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.header_formatters.preserve_case.v3.PreserveCaseFormatterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PreserveCaseFormatterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut forward_reason_phrase__ = None;
                let mut formatter_type_on_envoy_headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ForwardReasonPhrase => {
                            if forward_reason_phrase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardReasonPhrase"));
                            }
                            forward_reason_phrase__ = Some(map.next_value()?);
                        }
                        GeneratedField::FormatterTypeOnEnvoyHeaders => {
                            if formatter_type_on_envoy_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("formatterTypeOnEnvoyHeaders"));
                            }
                            formatter_type_on_envoy_headers__ = Some(map.next_value::<preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders>()? as i32);
                        }
                    }
                }
                Ok(PreserveCaseFormatterConfig {
                    forward_reason_phrase: forward_reason_phrase__.unwrap_or_default(),
                    formatter_type_on_envoy_headers: formatter_type_on_envoy_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.header_formatters.preserve_case.v3.PreserveCaseFormatterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::ProperCase => "PROPER_CASE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "PROPER_CASE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders;

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
                    .and_then(preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders::from_i32)
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
                    .and_then(preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders::Default),
                    "PROPER_CASE" => Ok(preserve_case_formatter_config::FormatterTypeOnEnvoyHeaders::ProperCase),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
