// @generated
impl serde::Serialize for HeaderValidatorConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http1_protocol_options.is_some() {
            len += 1;
        }
        if self.uri_path_normalization_options.is_some() {
            len += 1;
        }
        if self.restrict_http_methods {
            len += 1;
        }
        if self.headers_with_underscores_action != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig", len)?;
        if let Some(v) = self.http1_protocol_options.as_ref() {
            struct_ser.serialize_field("http1ProtocolOptions", v)?;
        }
        if let Some(v) = self.uri_path_normalization_options.as_ref() {
            struct_ser.serialize_field("uriPathNormalizationOptions", v)?;
        }
        if self.restrict_http_methods {
            struct_ser.serialize_field("restrictHttpMethods", &self.restrict_http_methods)?;
        }
        if self.headers_with_underscores_action != 0 {
            let v = header_validator_config::HeadersWithUnderscoresAction::from_i32(self.headers_with_underscores_action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.headers_with_underscores_action)))?;
            struct_ser.serialize_field("headersWithUnderscoresAction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderValidatorConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http1_protocol_options",
            "http1ProtocolOptions",
            "uri_path_normalization_options",
            "uriPathNormalizationOptions",
            "restrict_http_methods",
            "restrictHttpMethods",
            "headers_with_underscores_action",
            "headersWithUnderscoresAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Http1ProtocolOptions,
            UriPathNormalizationOptions,
            RestrictHttpMethods,
            HeadersWithUnderscoresAction,
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
                            "http1ProtocolOptions" | "http1_protocol_options" => Ok(GeneratedField::Http1ProtocolOptions),
                            "uriPathNormalizationOptions" | "uri_path_normalization_options" => Ok(GeneratedField::UriPathNormalizationOptions),
                            "restrictHttpMethods" | "restrict_http_methods" => Ok(GeneratedField::RestrictHttpMethods),
                            "headersWithUnderscoresAction" | "headers_with_underscores_action" => Ok(GeneratedField::HeadersWithUnderscoresAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderValidatorConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderValidatorConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http1_protocol_options__ = None;
                let mut uri_path_normalization_options__ = None;
                let mut restrict_http_methods__ = None;
                let mut headers_with_underscores_action__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Http1ProtocolOptions => {
                            if http1_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http1ProtocolOptions"));
                            }
                            http1_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::UriPathNormalizationOptions => {
                            if uri_path_normalization_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uriPathNormalizationOptions"));
                            }
                            uri_path_normalization_options__ = map.next_value()?;
                        }
                        GeneratedField::RestrictHttpMethods => {
                            if restrict_http_methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restrictHttpMethods"));
                            }
                            restrict_http_methods__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeadersWithUnderscoresAction => {
                            if headers_with_underscores_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersWithUnderscoresAction"));
                            }
                            headers_with_underscores_action__ = Some(map.next_value::<header_validator_config::HeadersWithUnderscoresAction>()? as i32);
                        }
                    }
                }
                Ok(HeaderValidatorConfig {
                    http1_protocol_options: http1_protocol_options__,
                    uri_path_normalization_options: uri_path_normalization_options__,
                    restrict_http_methods: restrict_http_methods__.unwrap_or_default(),
                    headers_with_underscores_action: headers_with_underscores_action__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header_validator_config::HeadersWithUnderscoresAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Allow => "ALLOW",
            Self::RejectRequest => "REJECT_REQUEST",
            Self::DropHeader => "DROP_HEADER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for header_validator_config::HeadersWithUnderscoresAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALLOW",
            "REJECT_REQUEST",
            "DROP_HEADER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_validator_config::HeadersWithUnderscoresAction;

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
                    .and_then(header_validator_config::HeadersWithUnderscoresAction::from_i32)
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
                    .and_then(header_validator_config::HeadersWithUnderscoresAction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ALLOW" => Ok(header_validator_config::HeadersWithUnderscoresAction::Allow),
                    "REJECT_REQUEST" => Ok(header_validator_config::HeadersWithUnderscoresAction::RejectRequest),
                    "DROP_HEADER" => Ok(header_validator_config::HeadersWithUnderscoresAction::DropHeader),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for header_validator_config::Http1ProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allow_chunked_length {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig.Http1ProtocolOptions", len)?;
        if self.allow_chunked_length {
            struct_ser.serialize_field("allowChunkedLength", &self.allow_chunked_length)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for header_validator_config::Http1ProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_chunked_length",
            "allowChunkedLength",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowChunkedLength,
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
                            "allowChunkedLength" | "allow_chunked_length" => Ok(GeneratedField::AllowChunkedLength),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_validator_config::Http1ProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig.Http1ProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<header_validator_config::Http1ProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_chunked_length__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowChunkedLength => {
                            if allow_chunked_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowChunkedLength"));
                            }
                            allow_chunked_length__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(header_validator_config::Http1ProtocolOptions {
                    allow_chunked_length: allow_chunked_length__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig.Http1ProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header_validator_config::UriPathNormalizationOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.skip_path_normalization {
            len += 1;
        }
        if self.skip_merging_slashes {
            len += 1;
        }
        if self.path_with_escaped_slashes_action != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig.UriPathNormalizationOptions", len)?;
        if self.skip_path_normalization {
            struct_ser.serialize_field("skipPathNormalization", &self.skip_path_normalization)?;
        }
        if self.skip_merging_slashes {
            struct_ser.serialize_field("skipMergingSlashes", &self.skip_merging_slashes)?;
        }
        if self.path_with_escaped_slashes_action != 0 {
            let v = header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::from_i32(self.path_with_escaped_slashes_action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.path_with_escaped_slashes_action)))?;
            struct_ser.serialize_field("pathWithEscapedSlashesAction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for header_validator_config::UriPathNormalizationOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "skip_path_normalization",
            "skipPathNormalization",
            "skip_merging_slashes",
            "skipMergingSlashes",
            "path_with_escaped_slashes_action",
            "pathWithEscapedSlashesAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SkipPathNormalization,
            SkipMergingSlashes,
            PathWithEscapedSlashesAction,
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
                            "skipPathNormalization" | "skip_path_normalization" => Ok(GeneratedField::SkipPathNormalization),
                            "skipMergingSlashes" | "skip_merging_slashes" => Ok(GeneratedField::SkipMergingSlashes),
                            "pathWithEscapedSlashesAction" | "path_with_escaped_slashes_action" => Ok(GeneratedField::PathWithEscapedSlashesAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_validator_config::UriPathNormalizationOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig.UriPathNormalizationOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<header_validator_config::UriPathNormalizationOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut skip_path_normalization__ = None;
                let mut skip_merging_slashes__ = None;
                let mut path_with_escaped_slashes_action__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SkipPathNormalization => {
                            if skip_path_normalization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipPathNormalization"));
                            }
                            skip_path_normalization__ = Some(map.next_value()?);
                        }
                        GeneratedField::SkipMergingSlashes => {
                            if skip_merging_slashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipMergingSlashes"));
                            }
                            skip_merging_slashes__ = Some(map.next_value()?);
                        }
                        GeneratedField::PathWithEscapedSlashesAction => {
                            if path_with_escaped_slashes_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathWithEscapedSlashesAction"));
                            }
                            path_with_escaped_slashes_action__ = Some(map.next_value::<header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction>()? as i32);
                        }
                    }
                }
                Ok(header_validator_config::UriPathNormalizationOptions {
                    skip_path_normalization: skip_path_normalization__.unwrap_or_default(),
                    skip_merging_slashes: skip_merging_slashes__.unwrap_or_default(),
                    path_with_escaped_slashes_action: path_with_escaped_slashes_action__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.header_validators.envoy_default.v3.HeaderValidatorConfig.UriPathNormalizationOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ImplementationSpecificDefault => "IMPLEMENTATION_SPECIFIC_DEFAULT",
            Self::KeepUnchanged => "KEEP_UNCHANGED",
            Self::RejectRequest => "REJECT_REQUEST",
            Self::UnescapeAndRedirect => "UNESCAPE_AND_REDIRECT",
            Self::UnescapeAndForward => "UNESCAPE_AND_FORWARD",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IMPLEMENTATION_SPECIFIC_DEFAULT",
            "KEEP_UNCHANGED",
            "REJECT_REQUEST",
            "UNESCAPE_AND_REDIRECT",
            "UNESCAPE_AND_FORWARD",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction;

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
                    .and_then(header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::from_i32)
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
                    .and_then(header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "IMPLEMENTATION_SPECIFIC_DEFAULT" => Ok(header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::ImplementationSpecificDefault),
                    "KEEP_UNCHANGED" => Ok(header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::KeepUnchanged),
                    "REJECT_REQUEST" => Ok(header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::RejectRequest),
                    "UNESCAPE_AND_REDIRECT" => Ok(header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::UnescapeAndRedirect),
                    "UNESCAPE_AND_FORWARD" => Ok(header_validator_config::uri_path_normalization_options::PathWithEscapedSlashesAction::UnescapeAndForward),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
