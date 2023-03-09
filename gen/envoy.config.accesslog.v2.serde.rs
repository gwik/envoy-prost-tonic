// @generated
impl serde::Serialize for CommonGrpcAccessLogConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.log_name.is_empty() {
            len += 1;
        }
        if self.grpc_service.is_some() {
            len += 1;
        }
        if self.buffer_flush_interval.is_some() {
            len += 1;
        }
        if self.buffer_size_bytes.is_some() {
            len += 1;
        }
        if !self.filter_state_objects_to_log.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.accesslog.v2.CommonGrpcAccessLogConfig", len)?;
        if !self.log_name.is_empty() {
            struct_ser.serialize_field("logName", &self.log_name)?;
        }
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        if let Some(v) = self.buffer_flush_interval.as_ref() {
            struct_ser.serialize_field("bufferFlushInterval", v)?;
        }
        if let Some(v) = self.buffer_size_bytes.as_ref() {
            struct_ser.serialize_field("bufferSizeBytes", v)?;
        }
        if !self.filter_state_objects_to_log.is_empty() {
            struct_ser.serialize_field("filterStateObjectsToLog", &self.filter_state_objects_to_log)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonGrpcAccessLogConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_name",
            "logName",
            "grpc_service",
            "grpcService",
            "buffer_flush_interval",
            "bufferFlushInterval",
            "buffer_size_bytes",
            "bufferSizeBytes",
            "filter_state_objects_to_log",
            "filterStateObjectsToLog",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LogName,
            GrpcService,
            BufferFlushInterval,
            BufferSizeBytes,
            FilterStateObjectsToLog,
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
                            "logName" | "log_name" => Ok(GeneratedField::LogName),
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "bufferFlushInterval" | "buffer_flush_interval" => Ok(GeneratedField::BufferFlushInterval),
                            "bufferSizeBytes" | "buffer_size_bytes" => Ok(GeneratedField::BufferSizeBytes),
                            "filterStateObjectsToLog" | "filter_state_objects_to_log" => Ok(GeneratedField::FilterStateObjectsToLog),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonGrpcAccessLogConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.accesslog.v2.CommonGrpcAccessLogConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommonGrpcAccessLogConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut log_name__ = None;
                let mut grpc_service__ = None;
                let mut buffer_flush_interval__ = None;
                let mut buffer_size_bytes__ = None;
                let mut filter_state_objects_to_log__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LogName => {
                            if log_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logName"));
                            }
                            log_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                        GeneratedField::BufferFlushInterval => {
                            if buffer_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferFlushInterval"));
                            }
                            buffer_flush_interval__ = map.next_value()?;
                        }
                        GeneratedField::BufferSizeBytes => {
                            if buffer_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferSizeBytes"));
                            }
                            buffer_size_bytes__ = map.next_value()?;
                        }
                        GeneratedField::FilterStateObjectsToLog => {
                            if filter_state_objects_to_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterStateObjectsToLog"));
                            }
                            filter_state_objects_to_log__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommonGrpcAccessLogConfig {
                    log_name: log_name__.unwrap_or_default(),
                    grpc_service: grpc_service__,
                    buffer_flush_interval: buffer_flush_interval__,
                    buffer_size_bytes: buffer_size_bytes__,
                    filter_state_objects_to_log: filter_state_objects_to_log__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.accesslog.v2.CommonGrpcAccessLogConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileAccessLog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.access_log_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.accesslog.v2.FileAccessLog", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.access_log_format.as_ref() {
            match v {
                file_access_log::AccessLogFormat::Format(v) => {
                    struct_ser.serialize_field("format", v)?;
                }
                file_access_log::AccessLogFormat::JsonFormat(v) => {
                    struct_ser.serialize_field("jsonFormat", v)?;
                }
                file_access_log::AccessLogFormat::TypedJsonFormat(v) => {
                    struct_ser.serialize_field("typedJsonFormat", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileAccessLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "format",
            "json_format",
            "jsonFormat",
            "typed_json_format",
            "typedJsonFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Format,
            JsonFormat,
            TypedJsonFormat,
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
                            "path" => Ok(GeneratedField::Path),
                            "format" => Ok(GeneratedField::Format),
                            "jsonFormat" | "json_format" => Ok(GeneratedField::JsonFormat),
                            "typedJsonFormat" | "typed_json_format" => Ok(GeneratedField::TypedJsonFormat),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileAccessLog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.accesslog.v2.FileAccessLog")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileAccessLog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut access_log_format__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Format => {
                            if access_log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            access_log_format__ = map.next_value::<::std::option::Option<_>>()?.map(file_access_log::AccessLogFormat::Format);
                        }
                        GeneratedField::JsonFormat => {
                            if access_log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonFormat"));
                            }
                            access_log_format__ = map.next_value::<::std::option::Option<_>>()?.map(file_access_log::AccessLogFormat::JsonFormat)
;
                        }
                        GeneratedField::TypedJsonFormat => {
                            if access_log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedJsonFormat"));
                            }
                            access_log_format__ = map.next_value::<::std::option::Option<_>>()?.map(file_access_log::AccessLogFormat::TypedJsonFormat)
;
                        }
                    }
                }
                Ok(FileAccessLog {
                    path: path__.unwrap_or_default(),
                    access_log_format: access_log_format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.accesslog.v2.FileAccessLog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpGrpcAccessLogConfig {
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
        if !self.additional_request_headers_to_log.is_empty() {
            len += 1;
        }
        if !self.additional_response_headers_to_log.is_empty() {
            len += 1;
        }
        if !self.additional_response_trailers_to_log.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.accesslog.v2.HttpGrpcAccessLogConfig", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        if !self.additional_request_headers_to_log.is_empty() {
            struct_ser.serialize_field("additionalRequestHeadersToLog", &self.additional_request_headers_to_log)?;
        }
        if !self.additional_response_headers_to_log.is_empty() {
            struct_ser.serialize_field("additionalResponseHeadersToLog", &self.additional_response_headers_to_log)?;
        }
        if !self.additional_response_trailers_to_log.is_empty() {
            struct_ser.serialize_field("additionalResponseTrailersToLog", &self.additional_response_trailers_to_log)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpGrpcAccessLogConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_config",
            "commonConfig",
            "additional_request_headers_to_log",
            "additionalRequestHeadersToLog",
            "additional_response_headers_to_log",
            "additionalResponseHeadersToLog",
            "additional_response_trailers_to_log",
            "additionalResponseTrailersToLog",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonConfig,
            AdditionalRequestHeadersToLog,
            AdditionalResponseHeadersToLog,
            AdditionalResponseTrailersToLog,
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
                            "additionalRequestHeadersToLog" | "additional_request_headers_to_log" => Ok(GeneratedField::AdditionalRequestHeadersToLog),
                            "additionalResponseHeadersToLog" | "additional_response_headers_to_log" => Ok(GeneratedField::AdditionalResponseHeadersToLog),
                            "additionalResponseTrailersToLog" | "additional_response_trailers_to_log" => Ok(GeneratedField::AdditionalResponseTrailersToLog),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpGrpcAccessLogConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.accesslog.v2.HttpGrpcAccessLogConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpGrpcAccessLogConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_config__ = None;
                let mut additional_request_headers_to_log__ = None;
                let mut additional_response_headers_to_log__ = None;
                let mut additional_response_trailers_to_log__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonConfig => {
                            if common_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonConfig"));
                            }
                            common_config__ = map.next_value()?;
                        }
                        GeneratedField::AdditionalRequestHeadersToLog => {
                            if additional_request_headers_to_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalRequestHeadersToLog"));
                            }
                            additional_request_headers_to_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::AdditionalResponseHeadersToLog => {
                            if additional_response_headers_to_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalResponseHeadersToLog"));
                            }
                            additional_response_headers_to_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::AdditionalResponseTrailersToLog => {
                            if additional_response_trailers_to_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalResponseTrailersToLog"));
                            }
                            additional_response_trailers_to_log__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HttpGrpcAccessLogConfig {
                    common_config: common_config__,
                    additional_request_headers_to_log: additional_request_headers_to_log__.unwrap_or_default(),
                    additional_response_headers_to_log: additional_response_headers_to_log__.unwrap_or_default(),
                    additional_response_trailers_to_log: additional_response_trailers_to_log__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.accesslog.v2.HttpGrpcAccessLogConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TcpGrpcAccessLogConfig {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.accesslog.v2.TcpGrpcAccessLogConfig", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TcpGrpcAccessLogConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_config",
            "commonConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonConfig,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TcpGrpcAccessLogConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.accesslog.v2.TcpGrpcAccessLogConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TcpGrpcAccessLogConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonConfig => {
                            if common_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonConfig"));
                            }
                            common_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(TcpGrpcAccessLogConfig {
                    common_config: common_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.accesslog.v2.TcpGrpcAccessLogConfig", FIELDS, GeneratedVisitor)
    }
}
