// @generated
impl serde::Serialize for BandwidthLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.enable_mode != 0 {
            len += 1;
        }
        if self.limit_kbps.is_some() {
            len += 1;
        }
        if self.fill_interval.is_some() {
            len += 1;
        }
        if self.runtime_enabled.is_some() {
            len += 1;
        }
        if self.enable_response_trailers {
            len += 1;
        }
        if !self.response_trailer_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.bandwidth_limit.v3.BandwidthLimit", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if self.enable_mode != 0 {
            let v = bandwidth_limit::EnableMode::from_i32(self.enable_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.enable_mode)))?;
            struct_ser.serialize_field("enableMode", &v)?;
        }
        if let Some(v) = self.limit_kbps.as_ref() {
            struct_ser.serialize_field("limitKbps", v)?;
        }
        if let Some(v) = self.fill_interval.as_ref() {
            struct_ser.serialize_field("fillInterval", v)?;
        }
        if let Some(v) = self.runtime_enabled.as_ref() {
            struct_ser.serialize_field("runtimeEnabled", v)?;
        }
        if self.enable_response_trailers {
            struct_ser.serialize_field("enableResponseTrailers", &self.enable_response_trailers)?;
        }
        if !self.response_trailer_prefix.is_empty() {
            struct_ser.serialize_field("responseTrailerPrefix", &self.response_trailer_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BandwidthLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "enable_mode",
            "enableMode",
            "limit_kbps",
            "limitKbps",
            "fill_interval",
            "fillInterval",
            "runtime_enabled",
            "runtimeEnabled",
            "enable_response_trailers",
            "enableResponseTrailers",
            "response_trailer_prefix",
            "responseTrailerPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            EnableMode,
            LimitKbps,
            FillInterval,
            RuntimeEnabled,
            EnableResponseTrailers,
            ResponseTrailerPrefix,
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
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "enableMode" | "enable_mode" => Ok(GeneratedField::EnableMode),
                            "limitKbps" | "limit_kbps" => Ok(GeneratedField::LimitKbps),
                            "fillInterval" | "fill_interval" => Ok(GeneratedField::FillInterval),
                            "runtimeEnabled" | "runtime_enabled" => Ok(GeneratedField::RuntimeEnabled),
                            "enableResponseTrailers" | "enable_response_trailers" => Ok(GeneratedField::EnableResponseTrailers),
                            "responseTrailerPrefix" | "response_trailer_prefix" => Ok(GeneratedField::ResponseTrailerPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BandwidthLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.bandwidth_limit.v3.BandwidthLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BandwidthLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut enable_mode__ = None;
                let mut limit_kbps__ = None;
                let mut fill_interval__ = None;
                let mut runtime_enabled__ = None;
                let mut enable_response_trailers__ = None;
                let mut response_trailer_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnableMode => {
                            if enable_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableMode"));
                            }
                            enable_mode__ = Some(map.next_value::<bandwidth_limit::EnableMode>()? as i32);
                        }
                        GeneratedField::LimitKbps => {
                            if limit_kbps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limitKbps"));
                            }
                            limit_kbps__ = map.next_value()?;
                        }
                        GeneratedField::FillInterval => {
                            if fill_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fillInterval"));
                            }
                            fill_interval__ = map.next_value()?;
                        }
                        GeneratedField::RuntimeEnabled => {
                            if runtime_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeEnabled"));
                            }
                            runtime_enabled__ = map.next_value()?;
                        }
                        GeneratedField::EnableResponseTrailers => {
                            if enable_response_trailers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableResponseTrailers"));
                            }
                            enable_response_trailers__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseTrailerPrefix => {
                            if response_trailer_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTrailerPrefix"));
                            }
                            response_trailer_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BandwidthLimit {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    enable_mode: enable_mode__.unwrap_or_default(),
                    limit_kbps: limit_kbps__,
                    fill_interval: fill_interval__,
                    runtime_enabled: runtime_enabled__,
                    enable_response_trailers: enable_response_trailers__.unwrap_or_default(),
                    response_trailer_prefix: response_trailer_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.bandwidth_limit.v3.BandwidthLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bandwidth_limit::EnableMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Disabled => "DISABLED",
            Self::Request => "REQUEST",
            Self::Response => "RESPONSE",
            Self::RequestAndResponse => "REQUEST_AND_RESPONSE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for bandwidth_limit::EnableMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DISABLED",
            "REQUEST",
            "RESPONSE",
            "REQUEST_AND_RESPONSE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bandwidth_limit::EnableMode;

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
                    .and_then(bandwidth_limit::EnableMode::from_i32)
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
                    .and_then(bandwidth_limit::EnableMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DISABLED" => Ok(bandwidth_limit::EnableMode::Disabled),
                    "REQUEST" => Ok(bandwidth_limit::EnableMode::Request),
                    "RESPONSE" => Ok(bandwidth_limit::EnableMode::Response),
                    "REQUEST_AND_RESPONSE" => Ok(bandwidth_limit::EnableMode::RequestAndResponse),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
