// @generated
impl serde::Serialize for StderrAccessLog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.access_log_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.access_loggers.stream.v3.StderrAccessLog", len)?;
        if let Some(v) = self.access_log_format.as_ref() {
            match v {
                stderr_access_log::AccessLogFormat::LogFormat(v) => {
                    struct_ser.serialize_field("logFormat", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StderrAccessLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_format",
            "logFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = StderrAccessLog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.access_loggers.stream.v3.StderrAccessLog")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StderrAccessLog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_log_format__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LogFormat => {
                            if access_log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logFormat"));
                            }
                            access_log_format__ = map.next_value::<::std::option::Option<_>>()?.map(stderr_access_log::AccessLogFormat::LogFormat)
;
                        }
                    }
                }
                Ok(StderrAccessLog {
                    access_log_format: access_log_format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.access_loggers.stream.v3.StderrAccessLog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StdoutAccessLog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.access_log_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.access_loggers.stream.v3.StdoutAccessLog", len)?;
        if let Some(v) = self.access_log_format.as_ref() {
            match v {
                stdout_access_log::AccessLogFormat::LogFormat(v) => {
                    struct_ser.serialize_field("logFormat", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StdoutAccessLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "log_format",
            "logFormat",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = StdoutAccessLog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.access_loggers.stream.v3.StdoutAccessLog")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StdoutAccessLog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_log_format__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LogFormat => {
                            if access_log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logFormat"));
                            }
                            access_log_format__ = map.next_value::<::std::option::Option<_>>()?.map(stdout_access_log::AccessLogFormat::LogFormat)
;
                        }
                    }
                }
                Ok(StdoutAccessLog {
                    access_log_format: access_log_format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.access_loggers.stream.v3.StdoutAccessLog", FIELDS, GeneratedVisitor)
    }
}
