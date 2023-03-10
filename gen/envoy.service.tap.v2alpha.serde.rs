// @generated
impl serde::Serialize for FilePerTapSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.FilePerTapSink", len)?;
        if !self.path_prefix.is_empty() {
            struct_ser.serialize_field("pathPrefix", &self.path_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilePerTapSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path_prefix",
            "pathPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PathPrefix,
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
                            "pathPrefix" | "path_prefix" => Ok(GeneratedField::PathPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilePerTapSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.FilePerTapSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilePerTapSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PathPrefix => {
                            if path_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathPrefix"));
                            }
                            path_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilePerTapSink {
                    path_prefix: path_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.FilePerTapSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpHeadersMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.HttpHeadersMatch", len)?;
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpHeadersMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "headers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Headers,
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
                            "headers" => Ok(GeneratedField::Headers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpHeadersMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.HttpHeadersMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpHeadersMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HttpHeadersMatch {
                    headers: headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.HttpHeadersMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MatchPredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.MatchPredicate", len)?;
        if let Some(v) = self.rule.as_ref() {
            match v {
                match_predicate::Rule::OrMatch(v) => {
                    struct_ser.serialize_field("orMatch", v)?;
                }
                match_predicate::Rule::AndMatch(v) => {
                    struct_ser.serialize_field("andMatch", v)?;
                }
                match_predicate::Rule::NotMatch(v) => {
                    struct_ser.serialize_field("notMatch", v)?;
                }
                match_predicate::Rule::AnyMatch(v) => {
                    struct_ser.serialize_field("anyMatch", v)?;
                }
                match_predicate::Rule::HttpRequestHeadersMatch(v) => {
                    struct_ser.serialize_field("httpRequestHeadersMatch", v)?;
                }
                match_predicate::Rule::HttpRequestTrailersMatch(v) => {
                    struct_ser.serialize_field("httpRequestTrailersMatch", v)?;
                }
                match_predicate::Rule::HttpResponseHeadersMatch(v) => {
                    struct_ser.serialize_field("httpResponseHeadersMatch", v)?;
                }
                match_predicate::Rule::HttpResponseTrailersMatch(v) => {
                    struct_ser.serialize_field("httpResponseTrailersMatch", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MatchPredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "or_match",
            "orMatch",
            "and_match",
            "andMatch",
            "not_match",
            "notMatch",
            "any_match",
            "anyMatch",
            "http_request_headers_match",
            "httpRequestHeadersMatch",
            "http_request_trailers_match",
            "httpRequestTrailersMatch",
            "http_response_headers_match",
            "httpResponseHeadersMatch",
            "http_response_trailers_match",
            "httpResponseTrailersMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrMatch,
            AndMatch,
            NotMatch,
            AnyMatch,
            HttpRequestHeadersMatch,
            HttpRequestTrailersMatch,
            HttpResponseHeadersMatch,
            HttpResponseTrailersMatch,
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
                            "orMatch" | "or_match" => Ok(GeneratedField::OrMatch),
                            "andMatch" | "and_match" => Ok(GeneratedField::AndMatch),
                            "notMatch" | "not_match" => Ok(GeneratedField::NotMatch),
                            "anyMatch" | "any_match" => Ok(GeneratedField::AnyMatch),
                            "httpRequestHeadersMatch" | "http_request_headers_match" => Ok(GeneratedField::HttpRequestHeadersMatch),
                            "httpRequestTrailersMatch" | "http_request_trailers_match" => Ok(GeneratedField::HttpRequestTrailersMatch),
                            "httpResponseHeadersMatch" | "http_response_headers_match" => Ok(GeneratedField::HttpResponseHeadersMatch),
                            "httpResponseTrailersMatch" | "http_response_trailers_match" => Ok(GeneratedField::HttpResponseTrailersMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MatchPredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.MatchPredicate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MatchPredicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OrMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::OrMatch)
;
                        }
                        GeneratedField::AndMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::AndMatch)
;
                        }
                        GeneratedField::NotMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::NotMatch)
;
                        }
                        GeneratedField::AnyMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anyMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::AnyMatch);
                        }
                        GeneratedField::HttpRequestHeadersMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpRequestHeadersMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::HttpRequestHeadersMatch)
;
                        }
                        GeneratedField::HttpRequestTrailersMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpRequestTrailersMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::HttpRequestTrailersMatch)
;
                        }
                        GeneratedField::HttpResponseHeadersMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpResponseHeadersMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::HttpResponseHeadersMatch)
;
                        }
                        GeneratedField::HttpResponseTrailersMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpResponseTrailersMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(match_predicate::Rule::HttpResponseTrailersMatch)
;
                        }
                    }
                }
                Ok(MatchPredicate {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.MatchPredicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for match_predicate::MatchSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.MatchPredicate.MatchSet", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for match_predicate::MatchSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = match_predicate::MatchSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.MatchPredicate.MatchSet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<match_predicate::MatchSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(match_predicate::MatchSet {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.MatchPredicate.MatchSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutputConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sinks.is_empty() {
            len += 1;
        }
        if self.max_buffered_rx_bytes.is_some() {
            len += 1;
        }
        if self.max_buffered_tx_bytes.is_some() {
            len += 1;
        }
        if self.streaming {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.OutputConfig", len)?;
        if !self.sinks.is_empty() {
            struct_ser.serialize_field("sinks", &self.sinks)?;
        }
        if let Some(v) = self.max_buffered_rx_bytes.as_ref() {
            struct_ser.serialize_field("maxBufferedRxBytes", v)?;
        }
        if let Some(v) = self.max_buffered_tx_bytes.as_ref() {
            struct_ser.serialize_field("maxBufferedTxBytes", v)?;
        }
        if self.streaming {
            struct_ser.serialize_field("streaming", &self.streaming)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutputConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sinks",
            "max_buffered_rx_bytes",
            "maxBufferedRxBytes",
            "max_buffered_tx_bytes",
            "maxBufferedTxBytes",
            "streaming",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sinks,
            MaxBufferedRxBytes,
            MaxBufferedTxBytes,
            Streaming,
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
                            "sinks" => Ok(GeneratedField::Sinks),
                            "maxBufferedRxBytes" | "max_buffered_rx_bytes" => Ok(GeneratedField::MaxBufferedRxBytes),
                            "maxBufferedTxBytes" | "max_buffered_tx_bytes" => Ok(GeneratedField::MaxBufferedTxBytes),
                            "streaming" => Ok(GeneratedField::Streaming),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutputConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.OutputConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OutputConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sinks__ = None;
                let mut max_buffered_rx_bytes__ = None;
                let mut max_buffered_tx_bytes__ = None;
                let mut streaming__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sinks => {
                            if sinks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sinks"));
                            }
                            sinks__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxBufferedRxBytes => {
                            if max_buffered_rx_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBufferedRxBytes"));
                            }
                            max_buffered_rx_bytes__ = map.next_value()?;
                        }
                        GeneratedField::MaxBufferedTxBytes => {
                            if max_buffered_tx_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBufferedTxBytes"));
                            }
                            max_buffered_tx_bytes__ = map.next_value()?;
                        }
                        GeneratedField::Streaming => {
                            if streaming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streaming"));
                            }
                            streaming__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OutputConfig {
                    sinks: sinks__.unwrap_or_default(),
                    max_buffered_rx_bytes: max_buffered_rx_bytes__,
                    max_buffered_tx_bytes: max_buffered_tx_bytes__,
                    streaming: streaming__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.OutputConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutputSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.format != 0 {
            len += 1;
        }
        if self.output_sink_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.OutputSink", len)?;
        if self.format != 0 {
            let v = output_sink::Format::from_i32(self.format)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.format)))?;
            struct_ser.serialize_field("format", &v)?;
        }
        if let Some(v) = self.output_sink_type.as_ref() {
            match v {
                output_sink::OutputSinkType::StreamingAdmin(v) => {
                    struct_ser.serialize_field("streamingAdmin", v)?;
                }
                output_sink::OutputSinkType::FilePerTap(v) => {
                    struct_ser.serialize_field("filePerTap", v)?;
                }
                output_sink::OutputSinkType::StreamingGrpc(v) => {
                    struct_ser.serialize_field("streamingGrpc", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutputSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "format",
            "streaming_admin",
            "streamingAdmin",
            "file_per_tap",
            "filePerTap",
            "streaming_grpc",
            "streamingGrpc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Format,
            StreamingAdmin,
            FilePerTap,
            StreamingGrpc,
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
                            "format" => Ok(GeneratedField::Format),
                            "streamingAdmin" | "streaming_admin" => Ok(GeneratedField::StreamingAdmin),
                            "filePerTap" | "file_per_tap" => Ok(GeneratedField::FilePerTap),
                            "streamingGrpc" | "streaming_grpc" => Ok(GeneratedField::StreamingGrpc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutputSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.OutputSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OutputSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut format__ = None;
                let mut output_sink_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Format => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            format__ = Some(map.next_value::<output_sink::Format>()? as i32);
                        }
                        GeneratedField::StreamingAdmin => {
                            if output_sink_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamingAdmin"));
                            }
                            output_sink_type__ = map.next_value::<::std::option::Option<_>>()?.map(output_sink::OutputSinkType::StreamingAdmin)
;
                        }
                        GeneratedField::FilePerTap => {
                            if output_sink_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filePerTap"));
                            }
                            output_sink_type__ = map.next_value::<::std::option::Option<_>>()?.map(output_sink::OutputSinkType::FilePerTap)
;
                        }
                        GeneratedField::StreamingGrpc => {
                            if output_sink_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamingGrpc"));
                            }
                            output_sink_type__ = map.next_value::<::std::option::Option<_>>()?.map(output_sink::OutputSinkType::StreamingGrpc)
;
                        }
                    }
                }
                Ok(OutputSink {
                    format: format__.unwrap_or_default(),
                    output_sink_type: output_sink_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.OutputSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for output_sink::Format {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::JsonBodyAsBytes => "JSON_BODY_AS_BYTES",
            Self::JsonBodyAsString => "JSON_BODY_AS_STRING",
            Self::ProtoBinary => "PROTO_BINARY",
            Self::ProtoBinaryLengthDelimited => "PROTO_BINARY_LENGTH_DELIMITED",
            Self::ProtoText => "PROTO_TEXT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for output_sink::Format {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JSON_BODY_AS_BYTES",
            "JSON_BODY_AS_STRING",
            "PROTO_BINARY",
            "PROTO_BINARY_LENGTH_DELIMITED",
            "PROTO_TEXT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = output_sink::Format;

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
                    .and_then(output_sink::Format::from_i32)
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
                    .and_then(output_sink::Format::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "JSON_BODY_AS_BYTES" => Ok(output_sink::Format::JsonBodyAsBytes),
                    "JSON_BODY_AS_STRING" => Ok(output_sink::Format::JsonBodyAsString),
                    "PROTO_BINARY" => Ok(output_sink::Format::ProtoBinary),
                    "PROTO_BINARY_LENGTH_DELIMITED" => Ok(output_sink::Format::ProtoBinaryLengthDelimited),
                    "PROTO_TEXT" => Ok(output_sink::Format::ProtoText),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for StreamTapsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        if self.trace_id != 0 {
            len += 1;
        }
        if self.trace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.StreamTapsRequest", len)?;
        if let Some(v) = self.identifier.as_ref() {
            struct_ser.serialize_field("identifier", v)?;
        }
        if self.trace_id != 0 {
            struct_ser.serialize_field("traceId", ToString::to_string(&self.trace_id).as_str())?;
        }
        if let Some(v) = self.trace.as_ref() {
            struct_ser.serialize_field("trace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamTapsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifier",
            "trace_id",
            "traceId",
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
            TraceId,
            Trace,
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
                            "identifier" => Ok(GeneratedField::Identifier),
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            "trace" => Ok(GeneratedField::Trace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamTapsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.StreamTapsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamTapsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                let mut trace_id__ = None;
                let mut trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = map.next_value()?;
                        }
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamTapsRequest {
                    identifier: identifier__,
                    trace_id: trace_id__.unwrap_or_default(),
                    trace: trace__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.StreamTapsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for stream_taps_request::Identifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node.is_some() {
            len += 1;
        }
        if !self.tap_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.StreamTapsRequest.Identifier", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.tap_id.is_empty() {
            struct_ser.serialize_field("tapId", &self.tap_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for stream_taps_request::Identifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "tap_id",
            "tapId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            TapId,
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
                            "node" => Ok(GeneratedField::Node),
                            "tapId" | "tap_id" => Ok(GeneratedField::TapId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = stream_taps_request::Identifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.StreamTapsRequest.Identifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<stream_taps_request::Identifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut tap_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                        GeneratedField::TapId => {
                            if tap_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tapId"));
                            }
                            tap_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(stream_taps_request::Identifier {
                    node: node__,
                    tap_id: tap_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.StreamTapsRequest.Identifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamTapsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.StreamTapsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamTapsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamTapsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.StreamTapsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamTapsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StreamTapsResponse {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.StreamTapsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingAdminSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.StreamingAdminSink", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamingAdminSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingAdminSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.StreamingAdminSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamingAdminSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StreamingAdminSink {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.StreamingAdminSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamingGrpcSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tap_id.is_empty() {
            len += 1;
        }
        if self.grpc_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.StreamingGrpcSink", len)?;
        if !self.tap_id.is_empty() {
            struct_ser.serialize_field("tapId", &self.tap_id)?;
        }
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamingGrpcSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tap_id",
            "tapId",
            "grpc_service",
            "grpcService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TapId,
            GrpcService,
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
                            "tapId" | "tap_id" => Ok(GeneratedField::TapId),
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamingGrpcSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.StreamingGrpcSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamingGrpcSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tap_id__ = None;
                let mut grpc_service__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TapId => {
                            if tap_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tapId"));
                            }
                            tap_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamingGrpcSink {
                    tap_id: tap_id__.unwrap_or_default(),
                    grpc_service: grpc_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.StreamingGrpcSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TapConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.match_config.is_some() {
            len += 1;
        }
        if self.output_config.is_some() {
            len += 1;
        }
        if self.tap_enabled.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.tap.v2alpha.TapConfig", len)?;
        if let Some(v) = self.match_config.as_ref() {
            struct_ser.serialize_field("matchConfig", v)?;
        }
        if let Some(v) = self.output_config.as_ref() {
            struct_ser.serialize_field("outputConfig", v)?;
        }
        if let Some(v) = self.tap_enabled.as_ref() {
            struct_ser.serialize_field("tapEnabled", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TapConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "match_config",
            "matchConfig",
            "output_config",
            "outputConfig",
            "tap_enabled",
            "tapEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MatchConfig,
            OutputConfig,
            TapEnabled,
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
                            "matchConfig" | "match_config" => Ok(GeneratedField::MatchConfig),
                            "outputConfig" | "output_config" => Ok(GeneratedField::OutputConfig),
                            "tapEnabled" | "tap_enabled" => Ok(GeneratedField::TapEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TapConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.tap.v2alpha.TapConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TapConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_config__ = None;
                let mut output_config__ = None;
                let mut tap_enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MatchConfig => {
                            if match_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchConfig"));
                            }
                            match_config__ = map.next_value()?;
                        }
                        GeneratedField::OutputConfig => {
                            if output_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputConfig"));
                            }
                            output_config__ = map.next_value()?;
                        }
                        GeneratedField::TapEnabled => {
                            if tap_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tapEnabled"));
                            }
                            tap_enabled__ = map.next_value()?;
                        }
                    }
                }
                Ok(TapConfig {
                    match_config: match_config__,
                    output_config: output_config__,
                    tap_enabled: tap_enabled__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.tap.v2alpha.TapConfig", FIELDS, GeneratedVisitor)
    }
}
