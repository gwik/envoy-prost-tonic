// @generated
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.access_loggers.file.v3.FileAccessLog", len)?;
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
                file_access_log::AccessLogFormat::LogFormat(v) => {
                    struct_ser.serialize_field("logFormat", v)?;
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
            "log_format",
            "logFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Format,
            JsonFormat,
            TypedJsonFormat,
            LogFormat,
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
                            "logFormat" | "log_format" => Ok(GeneratedField::LogFormat),
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
                formatter.write_str("struct envoy.extensions.access_loggers.file.v3.FileAccessLog")
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
                        GeneratedField::LogFormat => {
                            if access_log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logFormat"));
                            }
                            access_log_format__ = map.next_value::<::std::option::Option<_>>()?.map(file_access_log::AccessLogFormat::LogFormat)
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
        deserializer.deserialize_struct("envoy.extensions.access_loggers.file.v3.FileAccessLog", FIELDS, GeneratedVisitor)
    }
}
