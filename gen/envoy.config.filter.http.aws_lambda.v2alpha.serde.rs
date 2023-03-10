// @generated
impl serde::Serialize for Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.arn.is_empty() {
            len += 1;
        }
        if self.payload_passthrough {
            len += 1;
        }
        if self.invocation_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.aws_lambda.v2alpha.Config", len)?;
        if !self.arn.is_empty() {
            struct_ser.serialize_field("arn", &self.arn)?;
        }
        if self.payload_passthrough {
            struct_ser.serialize_field("payloadPassthrough", &self.payload_passthrough)?;
        }
        if self.invocation_mode != 0 {
            let v = config::InvocationMode::from_i32(self.invocation_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.invocation_mode)))?;
            struct_ser.serialize_field("invocationMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "arn",
            "payload_passthrough",
            "payloadPassthrough",
            "invocation_mode",
            "invocationMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Arn,
            PayloadPassthrough,
            InvocationMode,
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
                            "arn" => Ok(GeneratedField::Arn),
                            "payloadPassthrough" | "payload_passthrough" => Ok(GeneratedField::PayloadPassthrough),
                            "invocationMode" | "invocation_mode" => Ok(GeneratedField::InvocationMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.aws_lambda.v2alpha.Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut arn__ = None;
                let mut payload_passthrough__ = None;
                let mut invocation_mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Arn => {
                            if arn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arn"));
                            }
                            arn__ = Some(map.next_value()?);
                        }
                        GeneratedField::PayloadPassthrough => {
                            if payload_passthrough__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadPassthrough"));
                            }
                            payload_passthrough__ = Some(map.next_value()?);
                        }
                        GeneratedField::InvocationMode => {
                            if invocation_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invocationMode"));
                            }
                            invocation_mode__ = Some(map.next_value::<config::InvocationMode>()? as i32);
                        }
                    }
                }
                Ok(Config {
                    arn: arn__.unwrap_or_default(),
                    payload_passthrough: payload_passthrough__.unwrap_or_default(),
                    invocation_mode: invocation_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.aws_lambda.v2alpha.Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for config::InvocationMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Synchronous => "SYNCHRONOUS",
            Self::Asynchronous => "ASYNCHRONOUS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for config::InvocationMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SYNCHRONOUS",
            "ASYNCHRONOUS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = config::InvocationMode;

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
                    .and_then(config::InvocationMode::from_i32)
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
                    .and_then(config::InvocationMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SYNCHRONOUS" => Ok(config::InvocationMode::Synchronous),
                    "ASYNCHRONOUS" => Ok(config::InvocationMode::Asynchronous),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PerRouteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.invoke_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.aws_lambda.v2alpha.PerRouteConfig", len)?;
        if let Some(v) = self.invoke_config.as_ref() {
            struct_ser.serialize_field("invokeConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PerRouteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "invoke_config",
            "invokeConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InvokeConfig,
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
                            "invokeConfig" | "invoke_config" => Ok(GeneratedField::InvokeConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PerRouteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.aws_lambda.v2alpha.PerRouteConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PerRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut invoke_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InvokeConfig => {
                            if invoke_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invokeConfig"));
                            }
                            invoke_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(PerRouteConfig {
                    invoke_config: invoke_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.aws_lambda.v2alpha.PerRouteConfig", FIELDS, GeneratedVisitor)
    }
}
