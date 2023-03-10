// @generated
impl serde::Serialize for Router {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dynamic_stats.is_some() {
            len += 1;
        }
        if self.start_child_span {
            len += 1;
        }
        if !self.upstream_log.is_empty() {
            len += 1;
        }
        if self.suppress_envoy_headers {
            len += 1;
        }
        if !self.strict_check_headers.is_empty() {
            len += 1;
        }
        if self.respect_expected_rq_timeout {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.router.v2.Router", len)?;
        if let Some(v) = self.dynamic_stats.as_ref() {
            struct_ser.serialize_field("dynamicStats", v)?;
        }
        if self.start_child_span {
            struct_ser.serialize_field("startChildSpan", &self.start_child_span)?;
        }
        if !self.upstream_log.is_empty() {
            struct_ser.serialize_field("upstreamLog", &self.upstream_log)?;
        }
        if self.suppress_envoy_headers {
            struct_ser.serialize_field("suppressEnvoyHeaders", &self.suppress_envoy_headers)?;
        }
        if !self.strict_check_headers.is_empty() {
            struct_ser.serialize_field("strictCheckHeaders", &self.strict_check_headers)?;
        }
        if self.respect_expected_rq_timeout {
            struct_ser.serialize_field("respectExpectedRqTimeout", &self.respect_expected_rq_timeout)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Router {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_stats",
            "dynamicStats",
            "start_child_span",
            "startChildSpan",
            "upstream_log",
            "upstreamLog",
            "suppress_envoy_headers",
            "suppressEnvoyHeaders",
            "strict_check_headers",
            "strictCheckHeaders",
            "respect_expected_rq_timeout",
            "respectExpectedRqTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicStats,
            StartChildSpan,
            UpstreamLog,
            SuppressEnvoyHeaders,
            StrictCheckHeaders,
            RespectExpectedRqTimeout,
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
                            "dynamicStats" | "dynamic_stats" => Ok(GeneratedField::DynamicStats),
                            "startChildSpan" | "start_child_span" => Ok(GeneratedField::StartChildSpan),
                            "upstreamLog" | "upstream_log" => Ok(GeneratedField::UpstreamLog),
                            "suppressEnvoyHeaders" | "suppress_envoy_headers" => Ok(GeneratedField::SuppressEnvoyHeaders),
                            "strictCheckHeaders" | "strict_check_headers" => Ok(GeneratedField::StrictCheckHeaders),
                            "respectExpectedRqTimeout" | "respect_expected_rq_timeout" => Ok(GeneratedField::RespectExpectedRqTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Router;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.router.v2.Router")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Router, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dynamic_stats__ = None;
                let mut start_child_span__ = None;
                let mut upstream_log__ = None;
                let mut suppress_envoy_headers__ = None;
                let mut strict_check_headers__ = None;
                let mut respect_expected_rq_timeout__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DynamicStats => {
                            if dynamic_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicStats"));
                            }
                            dynamic_stats__ = map.next_value()?;
                        }
                        GeneratedField::StartChildSpan => {
                            if start_child_span__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startChildSpan"));
                            }
                            start_child_span__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamLog => {
                            if upstream_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamLog"));
                            }
                            upstream_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::SuppressEnvoyHeaders => {
                            if suppress_envoy_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suppressEnvoyHeaders"));
                            }
                            suppress_envoy_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::StrictCheckHeaders => {
                            if strict_check_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strictCheckHeaders"));
                            }
                            strict_check_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::RespectExpectedRqTimeout => {
                            if respect_expected_rq_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("respectExpectedRqTimeout"));
                            }
                            respect_expected_rq_timeout__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Router {
                    dynamic_stats: dynamic_stats__,
                    start_child_span: start_child_span__.unwrap_or_default(),
                    upstream_log: upstream_log__.unwrap_or_default(),
                    suppress_envoy_headers: suppress_envoy_headers__.unwrap_or_default(),
                    strict_check_headers: strict_check_headers__.unwrap_or_default(),
                    respect_expected_rq_timeout: respect_expected_rq_timeout__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.router.v2.Router", FIELDS, GeneratedVisitor)
    }
}
