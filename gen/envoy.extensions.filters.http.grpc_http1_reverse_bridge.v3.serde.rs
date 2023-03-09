// @generated
impl serde::Serialize for FilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content_type.is_empty() {
            len += 1;
        }
        if self.withhold_grpc_frames {
            len += 1;
        }
        if !self.response_size_header.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfig", len)?;
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if self.withhold_grpc_frames {
            struct_ser.serialize_field("withholdGrpcFrames", &self.withhold_grpc_frames)?;
        }
        if !self.response_size_header.is_empty() {
            struct_ser.serialize_field("responseSizeHeader", &self.response_size_header)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "content_type",
            "contentType",
            "withhold_grpc_frames",
            "withholdGrpcFrames",
            "response_size_header",
            "responseSizeHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContentType,
            WithholdGrpcFrames,
            ResponseSizeHeader,
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
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "withholdGrpcFrames" | "withhold_grpc_frames" => Ok(GeneratedField::WithholdGrpcFrames),
                            "responseSizeHeader" | "response_size_header" => Ok(GeneratedField::ResponseSizeHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut content_type__ = None;
                let mut withhold_grpc_frames__ = None;
                let mut response_size_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::WithholdGrpcFrames => {
                            if withhold_grpc_frames__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withholdGrpcFrames"));
                            }
                            withhold_grpc_frames__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseSizeHeader => {
                            if response_size_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseSizeHeader"));
                            }
                            response_size_header__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilterConfig {
                    content_type: content_type__.unwrap_or_default(),
                    withhold_grpc_frames: withhold_grpc_frames__.unwrap_or_default(),
                    response_size_header: response_size_header__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterConfigPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.disabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfigPerRoute", len)?;
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterConfigPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
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
                            "disabled" => Ok(GeneratedField::Disabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterConfigPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfigPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterConfigPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut disabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilterConfigPerRoute {
                    disabled: disabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_http1_reverse_bridge.v3.FilterConfigPerRoute", FIELDS, GeneratedVisitor)
    }
}
