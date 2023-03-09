// @generated
impl serde::Serialize for OpenTelemetryAccessLogConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_config.is_some() {
            len += 1;
        }
        if self.resource_attributes.is_some() {
            len += 1;
        }
        if self.body.is_some() {
            len += 1;
        }
        if self.attributes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.access_loggers.open_telemetry.v3.OpenTelemetryAccessLogConfig", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        if let Some(v) = self.resource_attributes.as_ref() {
            struct_ser.serialize_field("resourceAttributes", v)?;
        }
        if let Some(v) = self.body.as_ref() {
            struct_ser.serialize_field("body", v)?;
        }
        if let Some(v) = self.attributes.as_ref() {
            struct_ser.serialize_field("attributes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OpenTelemetryAccessLogConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_config",
            "commonConfig",
            "resource_attributes",
            "resourceAttributes",
            "body",
            "attributes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonConfig,
            ResourceAttributes,
            Body,
            Attributes,
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
                            "commonConfig" | "common_config" => Ok(GeneratedField::CommonConfig),
                            "resourceAttributes" | "resource_attributes" => Ok(GeneratedField::ResourceAttributes),
                            "body" => Ok(GeneratedField::Body),
                            "attributes" => Ok(GeneratedField::Attributes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OpenTelemetryAccessLogConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.access_loggers.open_telemetry.v3.OpenTelemetryAccessLogConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OpenTelemetryAccessLogConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_config__ = None;
                let mut resource_attributes__ = None;
                let mut body__ = None;
                let mut attributes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonConfig => {
                            if common_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonConfig"));
                            }
                            common_config__ = map.next_value()?;
                        }
                        GeneratedField::ResourceAttributes => {
                            if resource_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAttributes"));
                            }
                            resource_attributes__ = map.next_value()?;
                        }
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = map.next_value()?;
                        }
                        GeneratedField::Attributes => {
                            if attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attributes"));
                            }
                            attributes__ = map.next_value()?;
                        }
                    }
                }
                Ok(OpenTelemetryAccessLogConfig {
                    common_config: common_config__,
                    resource_attributes: resource_attributes__,
                    body: body__,
                    attributes: attributes__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.access_loggers.open_telemetry.v3.OpenTelemetryAccessLogConfig", FIELDS, GeneratedVisitor)
    }
}
