// @generated
impl serde::Serialize for RateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        if self.stage != 0 {
            len += 1;
        }
        if !self.request_type.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.failure_mode_deny {
            len += 1;
        }
        if self.rate_limited_as_resource_exhausted {
            len += 1;
        }
        if self.rate_limit_service.is_some() {
            len += 1;
        }
        if self.enable_x_ratelimit_headers != 0 {
            len += 1;
        }
        if self.disable_x_envoy_ratelimited_header {
            len += 1;
        }
        if self.rate_limited_status.is_some() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimit", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if self.stage != 0 {
            struct_ser.serialize_field("stage", &self.stage)?;
        }
        if !self.request_type.is_empty() {
            struct_ser.serialize_field("requestType", &self.request_type)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if self.failure_mode_deny {
            struct_ser.serialize_field("failureModeDeny", &self.failure_mode_deny)?;
        }
        if self.rate_limited_as_resource_exhausted {
            struct_ser.serialize_field("rateLimitedAsResourceExhausted", &self.rate_limited_as_resource_exhausted)?;
        }
        if let Some(v) = self.rate_limit_service.as_ref() {
            struct_ser.serialize_field("rateLimitService", v)?;
        }
        if self.enable_x_ratelimit_headers != 0 {
            let v = rate_limit::XRateLimitHeadersRfcVersion::from_i32(self.enable_x_ratelimit_headers)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.enable_x_ratelimit_headers)))?;
            struct_ser.serialize_field("enableXRatelimitHeaders", &v)?;
        }
        if self.disable_x_envoy_ratelimited_header {
            struct_ser.serialize_field("disableXEnvoyRatelimitedHeader", &self.disable_x_envoy_ratelimited_header)?;
        }
        if let Some(v) = self.rate_limited_status.as_ref() {
            struct_ser.serialize_field("rateLimitedStatus", v)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "stage",
            "request_type",
            "requestType",
            "timeout",
            "failure_mode_deny",
            "failureModeDeny",
            "rate_limited_as_resource_exhausted",
            "rateLimitedAsResourceExhausted",
            "rate_limit_service",
            "rateLimitService",
            "enable_x_ratelimit_headers",
            "enableXRatelimitHeaders",
            "disable_x_envoy_ratelimited_header",
            "disableXEnvoyRatelimitedHeader",
            "rate_limited_status",
            "rateLimitedStatus",
            "response_headers_to_add",
            "responseHeadersToAdd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            Stage,
            RequestType,
            Timeout,
            FailureModeDeny,
            RateLimitedAsResourceExhausted,
            RateLimitService,
            EnableXRatelimitHeaders,
            DisableXEnvoyRatelimitedHeader,
            RateLimitedStatus,
            ResponseHeadersToAdd,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "stage" => Ok(GeneratedField::Stage),
                            "requestType" | "request_type" => Ok(GeneratedField::RequestType),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "failureModeDeny" | "failure_mode_deny" => Ok(GeneratedField::FailureModeDeny),
                            "rateLimitedAsResourceExhausted" | "rate_limited_as_resource_exhausted" => Ok(GeneratedField::RateLimitedAsResourceExhausted),
                            "rateLimitService" | "rate_limit_service" => Ok(GeneratedField::RateLimitService),
                            "enableXRatelimitHeaders" | "enable_x_ratelimit_headers" => Ok(GeneratedField::EnableXRatelimitHeaders),
                            "disableXEnvoyRatelimitedHeader" | "disable_x_envoy_ratelimited_header" => Ok(GeneratedField::DisableXEnvoyRatelimitedHeader),
                            "rateLimitedStatus" | "rate_limited_status" => Ok(GeneratedField::RateLimitedStatus),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut stage__ = None;
                let mut request_type__ = None;
                let mut timeout__ = None;
                let mut failure_mode_deny__ = None;
                let mut rate_limited_as_resource_exhausted__ = None;
                let mut rate_limit_service__ = None;
                let mut enable_x_ratelimit_headers__ = None;
                let mut disable_x_envoy_ratelimited_header__ = None;
                let mut rate_limited_status__ = None;
                let mut response_headers_to_add__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map.next_value()?);
                        }
                        GeneratedField::Stage => {
                            if stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            stage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequestType => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestType"));
                            }
                            request_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                        GeneratedField::FailureModeDeny => {
                            if failure_mode_deny__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureModeDeny"));
                            }
                            failure_mode_deny__ = Some(map.next_value()?);
                        }
                        GeneratedField::RateLimitedAsResourceExhausted => {
                            if rate_limited_as_resource_exhausted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitedAsResourceExhausted"));
                            }
                            rate_limited_as_resource_exhausted__ = Some(map.next_value()?);
                        }
                        GeneratedField::RateLimitService => {
                            if rate_limit_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitService"));
                            }
                            rate_limit_service__ = map.next_value()?;
                        }
                        GeneratedField::EnableXRatelimitHeaders => {
                            if enable_x_ratelimit_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableXRatelimitHeaders"));
                            }
                            enable_x_ratelimit_headers__ = Some(map.next_value::<rate_limit::XRateLimitHeadersRfcVersion>()? as i32);
                        }
                        GeneratedField::DisableXEnvoyRatelimitedHeader => {
                            if disable_x_envoy_ratelimited_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableXEnvoyRatelimitedHeader"));
                            }
                            disable_x_envoy_ratelimited_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::RateLimitedStatus => {
                            if rate_limited_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitedStatus"));
                            }
                            rate_limited_status__ = map.next_value()?;
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RateLimit {
                    domain: domain__.unwrap_or_default(),
                    stage: stage__.unwrap_or_default(),
                    request_type: request_type__.unwrap_or_default(),
                    timeout: timeout__,
                    failure_mode_deny: failure_mode_deny__.unwrap_or_default(),
                    rate_limited_as_resource_exhausted: rate_limited_as_resource_exhausted__.unwrap_or_default(),
                    rate_limit_service: rate_limit_service__,
                    enable_x_ratelimit_headers: enable_x_ratelimit_headers__.unwrap_or_default(),
                    disable_x_envoy_ratelimited_header: disable_x_envoy_ratelimited_header__.unwrap_or_default(),
                    rate_limited_status: rate_limited_status__,
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit::XRateLimitHeadersRfcVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Off => "OFF",
            Self::DraftVersion03 => "DRAFT_VERSION_03",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit::XRateLimitHeadersRfcVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OFF",
            "DRAFT_VERSION_03",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit::XRateLimitHeadersRfcVersion;

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
                    .and_then(rate_limit::XRateLimitHeadersRfcVersion::from_i32)
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
                    .and_then(rate_limit::XRateLimitHeadersRfcVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OFF" => Ok(rate_limit::XRateLimitHeadersRfcVersion::Off),
                    "DRAFT_VERSION_03" => Ok(rate_limit::XRateLimitHeadersRfcVersion::DraftVersion03),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stage != 0 {
            len += 1;
        }
        if !self.disable_key.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        if self.limit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig", len)?;
        if self.stage != 0 {
            struct_ser.serialize_field("stage", &self.stage)?;
        }
        if !self.disable_key.is_empty() {
            struct_ser.serialize_field("disableKey", &self.disable_key)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        if let Some(v) = self.limit.as_ref() {
            struct_ser.serialize_field("limit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stage",
            "disable_key",
            "disableKey",
            "actions",
            "limit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Stage,
            DisableKey,
            Actions,
            Limit,
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
                            "stage" => Ok(GeneratedField::Stage),
                            "disableKey" | "disable_key" => Ok(GeneratedField::DisableKey),
                            "actions" => Ok(GeneratedField::Actions),
                            "limit" => Ok(GeneratedField::Limit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stage__ = None;
                let mut disable_key__ = None;
                let mut actions__ = None;
                let mut limit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Stage => {
                            if stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            stage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DisableKey => {
                            if disable_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableKey"));
                            }
                            disable_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = map.next_value()?;
                        }
                    }
                }
                Ok(RateLimitConfig {
                    stage: stage__.unwrap_or_default(),
                    disable_key: disable_key__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                    limit: limit__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action", len)?;
        if let Some(v) = self.action_specifier.as_ref() {
            match v {
                rate_limit_config::action::ActionSpecifier::SourceCluster(v) => {
                    struct_ser.serialize_field("sourceCluster", v)?;
                }
                rate_limit_config::action::ActionSpecifier::DestinationCluster(v) => {
                    struct_ser.serialize_field("destinationCluster", v)?;
                }
                rate_limit_config::action::ActionSpecifier::RequestHeaders(v) => {
                    struct_ser.serialize_field("requestHeaders", v)?;
                }
                rate_limit_config::action::ActionSpecifier::RemoteAddress(v) => {
                    struct_ser.serialize_field("remoteAddress", v)?;
                }
                rate_limit_config::action::ActionSpecifier::GenericKey(v) => {
                    struct_ser.serialize_field("genericKey", v)?;
                }
                rate_limit_config::action::ActionSpecifier::HeaderValueMatch(v) => {
                    struct_ser.serialize_field("headerValueMatch", v)?;
                }
                rate_limit_config::action::ActionSpecifier::Metadata(v) => {
                    struct_ser.serialize_field("metadata", v)?;
                }
                rate_limit_config::action::ActionSpecifier::Extension(v) => {
                    struct_ser.serialize_field("extension", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_cluster",
            "sourceCluster",
            "destination_cluster",
            "destinationCluster",
            "request_headers",
            "requestHeaders",
            "remote_address",
            "remoteAddress",
            "generic_key",
            "genericKey",
            "header_value_match",
            "headerValueMatch",
            "metadata",
            "extension",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceCluster,
            DestinationCluster,
            RequestHeaders,
            RemoteAddress,
            GenericKey,
            HeaderValueMatch,
            Metadata,
            Extension,
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
                            "sourceCluster" | "source_cluster" => Ok(GeneratedField::SourceCluster),
                            "destinationCluster" | "destination_cluster" => Ok(GeneratedField::DestinationCluster),
                            "requestHeaders" | "request_headers" => Ok(GeneratedField::RequestHeaders),
                            "remoteAddress" | "remote_address" => Ok(GeneratedField::RemoteAddress),
                            "genericKey" | "generic_key" => Ok(GeneratedField::GenericKey),
                            "headerValueMatch" | "header_value_match" => Ok(GeneratedField::HeaderValueMatch),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "extension" => Ok(GeneratedField::Extension),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_config::Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::Action, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceCluster => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceCluster"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::SourceCluster)
;
                        }
                        GeneratedField::DestinationCluster => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationCluster"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::DestinationCluster)
;
                        }
                        GeneratedField::RequestHeaders => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaders"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::RequestHeaders)
;
                        }
                        GeneratedField::RemoteAddress => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteAddress"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::RemoteAddress)
;
                        }
                        GeneratedField::GenericKey => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericKey"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::GenericKey)
;
                        }
                        GeneratedField::HeaderValueMatch => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerValueMatch"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::HeaderValueMatch)
;
                        }
                        GeneratedField::Metadata => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::Metadata)
;
                        }
                        GeneratedField::Extension => {
                            if action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::action::ActionSpecifier::Extension)
;
                        }
                    }
                }
                Ok(rate_limit_config::Action {
                    action_specifier: action_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::DestinationCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.DestinationCluster", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::DestinationCluster {
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
            type Value = rate_limit_config::action::DestinationCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.DestinationCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::action::DestinationCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit_config::action::DestinationCluster {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.DestinationCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::GenericKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_value.is_empty() {
            len += 1;
        }
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.GenericKey", len)?;
        if !self.descriptor_value.is_empty() {
            struct_ser.serialize_field("descriptorValue", &self.descriptor_value)?;
        }
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptorKey", &self.descriptor_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::GenericKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_value",
            "descriptorValue",
            "descriptor_key",
            "descriptorKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorValue,
            DescriptorKey,
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
                            "descriptorValue" | "descriptor_value" => Ok(GeneratedField::DescriptorValue),
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_config::action::GenericKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.GenericKey")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::action::GenericKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_value__ = None;
                let mut descriptor_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DescriptorValue => {
                            if descriptor_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorValue"));
                            }
                            descriptor_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit_config::action::GenericKey {
                    descriptor_value: descriptor_value__.unwrap_or_default(),
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.GenericKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::HeaderValueMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_value.is_empty() {
            len += 1;
        }
        if self.expect_match {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.HeaderValueMatch", len)?;
        if !self.descriptor_value.is_empty() {
            struct_ser.serialize_field("descriptorValue", &self.descriptor_value)?;
        }
        if self.expect_match {
            struct_ser.serialize_field("expectMatch", &self.expect_match)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::HeaderValueMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_value",
            "descriptorValue",
            "expect_match",
            "expectMatch",
            "headers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorValue,
            ExpectMatch,
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
                            "descriptorValue" | "descriptor_value" => Ok(GeneratedField::DescriptorValue),
                            "expectMatch" | "expect_match" => Ok(GeneratedField::ExpectMatch),
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
            type Value = rate_limit_config::action::HeaderValueMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.HeaderValueMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::action::HeaderValueMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_value__ = None;
                let mut expect_match__ = None;
                let mut headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DescriptorValue => {
                            if descriptor_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorValue"));
                            }
                            descriptor_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExpectMatch => {
                            if expect_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectMatch"));
                            }
                            expect_match__ = Some(map.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit_config::action::HeaderValueMatch {
                    descriptor_value: descriptor_value__.unwrap_or_default(),
                    expect_match: expect_match__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.HeaderValueMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::MetaData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if self.metadata_key.is_some() {
            len += 1;
        }
        if !self.default_value.is_empty() {
            len += 1;
        }
        if self.source != 0 {
            len += 1;
        }
        if self.skip_if_absent {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.MetaData", len)?;
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptorKey", &self.descriptor_key)?;
        }
        if let Some(v) = self.metadata_key.as_ref() {
            struct_ser.serialize_field("metadataKey", v)?;
        }
        if !self.default_value.is_empty() {
            struct_ser.serialize_field("defaultValue", &self.default_value)?;
        }
        if self.source != 0 {
            let v = rate_limit_config::action::meta_data::Source::from_i32(self.source)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.source)))?;
            struct_ser.serialize_field("source", &v)?;
        }
        if self.skip_if_absent {
            struct_ser.serialize_field("skipIfAbsent", &self.skip_if_absent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::MetaData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_key",
            "descriptorKey",
            "metadata_key",
            "metadataKey",
            "default_value",
            "defaultValue",
            "source",
            "skip_if_absent",
            "skipIfAbsent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorKey,
            MetadataKey,
            DefaultValue,
            Source,
            SkipIfAbsent,
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
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "metadataKey" | "metadata_key" => Ok(GeneratedField::MetadataKey),
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "source" => Ok(GeneratedField::Source),
                            "skipIfAbsent" | "skip_if_absent" => Ok(GeneratedField::SkipIfAbsent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_config::action::MetaData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.MetaData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::action::MetaData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_key__ = None;
                let mut metadata_key__ = None;
                let mut default_value__ = None;
                let mut source__ = None;
                let mut skip_if_absent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::MetadataKey => {
                            if metadata_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataKey"));
                            }
                            metadata_key__ = map.next_value()?;
                        }
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value::<rate_limit_config::action::meta_data::Source>()? as i32);
                        }
                        GeneratedField::SkipIfAbsent => {
                            if skip_if_absent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipIfAbsent"));
                            }
                            skip_if_absent__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit_config::action::MetaData {
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    metadata_key: metadata_key__,
                    default_value: default_value__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    skip_if_absent: skip_if_absent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.MetaData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::meta_data::Source {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Dynamic => "DYNAMIC",
            Self::RouteEntry => "ROUTE_ENTRY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::meta_data::Source {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DYNAMIC",
            "ROUTE_ENTRY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_config::action::meta_data::Source;

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
                    .and_then(rate_limit_config::action::meta_data::Source::from_i32)
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
                    .and_then(rate_limit_config::action::meta_data::Source::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DYNAMIC" => Ok(rate_limit_config::action::meta_data::Source::Dynamic),
                    "ROUTE_ENTRY" => Ok(rate_limit_config::action::meta_data::Source::RouteEntry),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::RemoteAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.RemoteAddress", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::RemoteAddress {
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
            type Value = rate_limit_config::action::RemoteAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.RemoteAddress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::action::RemoteAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit_config::action::RemoteAddress {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.RemoteAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::RequestHeaders {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header_name.is_empty() {
            len += 1;
        }
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if self.skip_if_absent {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.RequestHeaders", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("headerName", &self.header_name)?;
        }
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptorKey", &self.descriptor_key)?;
        }
        if self.skip_if_absent {
            struct_ser.serialize_field("skipIfAbsent", &self.skip_if_absent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::RequestHeaders {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
            "descriptor_key",
            "descriptorKey",
            "skip_if_absent",
            "skipIfAbsent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderName,
            DescriptorKey,
            SkipIfAbsent,
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
                            "headerName" | "header_name" => Ok(GeneratedField::HeaderName),
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "skipIfAbsent" | "skip_if_absent" => Ok(GeneratedField::SkipIfAbsent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_config::action::RequestHeaders;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.RequestHeaders")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::action::RequestHeaders, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                let mut descriptor_key__ = None;
                let mut skip_if_absent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::SkipIfAbsent => {
                            if skip_if_absent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipIfAbsent"));
                            }
                            skip_if_absent__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit_config::action::RequestHeaders {
                    header_name: header_name__.unwrap_or_default(),
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    skip_if_absent: skip_if_absent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.RequestHeaders", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::action::SourceCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.SourceCluster", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::action::SourceCluster {
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
            type Value = rate_limit_config::action::SourceCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.SourceCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::action::SourceCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit_config::action::SourceCluster {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Action.SourceCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::Override {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.override_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Override", len)?;
        if let Some(v) = self.override_specifier.as_ref() {
            match v {
                rate_limit_config::r#override::OverrideSpecifier::DynamicMetadata(v) => {
                    struct_ser.serialize_field("dynamicMetadata", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::Override {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dynamic_metadata",
            "dynamicMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DynamicMetadata,
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
                            "dynamicMetadata" | "dynamic_metadata" => Ok(GeneratedField::DynamicMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_config::Override;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Override")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::Override, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut override_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DynamicMetadata => {
                            if override_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicMetadata"));
                            }
                            override_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_config::r#override::OverrideSpecifier::DynamicMetadata)
;
                        }
                    }
                }
                Ok(rate_limit_config::Override {
                    override_specifier: override_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Override", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_config::r#override::DynamicMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Override.DynamicMetadata", len)?;
        if let Some(v) = self.metadata_key.as_ref() {
            struct_ser.serialize_field("metadataKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_config::r#override::DynamicMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_key",
            "metadataKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataKey,
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
                            "metadataKey" | "metadata_key" => Ok(GeneratedField::MetadataKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_config::r#override::DynamicMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Override.DynamicMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_config::r#override::DynamicMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MetadataKey => {
                            if metadata_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataKey"));
                            }
                            metadata_key__ = map.next_value()?;
                        }
                    }
                }
                Ok(rate_limit_config::r#override::DynamicMetadata {
                    metadata_key: metadata_key__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitConfig.Override.DynamicMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.vh_rate_limits != 0 {
            len += 1;
        }
        if self.override_option != 0 {
            len += 1;
        }
        if !self.rate_limits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitPerRoute", len)?;
        if self.vh_rate_limits != 0 {
            let v = rate_limit_per_route::VhRateLimitsOptions::from_i32(self.vh_rate_limits)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.vh_rate_limits)))?;
            struct_ser.serialize_field("vhRateLimits", &v)?;
        }
        if self.override_option != 0 {
            let v = rate_limit_per_route::OverrideOptions::from_i32(self.override_option)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.override_option)))?;
            struct_ser.serialize_field("overrideOption", &v)?;
        }
        if !self.rate_limits.is_empty() {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vh_rate_limits",
            "vhRateLimits",
            "override_option",
            "overrideOption",
            "rate_limits",
            "rateLimits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VhRateLimits,
            OverrideOption,
            RateLimits,
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
                            "vhRateLimits" | "vh_rate_limits" => Ok(GeneratedField::VhRateLimits),
                            "overrideOption" | "override_option" => Ok(GeneratedField::OverrideOption),
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ratelimit.v3.RateLimitPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vh_rate_limits__ = None;
                let mut override_option__ = None;
                let mut rate_limits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VhRateLimits => {
                            if vh_rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vhRateLimits"));
                            }
                            vh_rate_limits__ = Some(map.next_value::<rate_limit_per_route::VhRateLimitsOptions>()? as i32);
                        }
                        GeneratedField::OverrideOption => {
                            if override_option__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideOption"));
                            }
                            override_option__ = Some(map.next_value::<rate_limit_per_route::OverrideOptions>()? as i32);
                        }
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RateLimitPerRoute {
                    vh_rate_limits: vh_rate_limits__.unwrap_or_default(),
                    override_option: override_option__.unwrap_or_default(),
                    rate_limits: rate_limits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ratelimit.v3.RateLimitPerRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_per_route::OverrideOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::OverridePolicy => "OVERRIDE_POLICY",
            Self::IncludePolicy => "INCLUDE_POLICY",
            Self::IgnorePolicy => "IGNORE_POLICY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_per_route::OverrideOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "OVERRIDE_POLICY",
            "INCLUDE_POLICY",
            "IGNORE_POLICY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_per_route::OverrideOptions;

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
                    .and_then(rate_limit_per_route::OverrideOptions::from_i32)
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
                    .and_then(rate_limit_per_route::OverrideOptions::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(rate_limit_per_route::OverrideOptions::Default),
                    "OVERRIDE_POLICY" => Ok(rate_limit_per_route::OverrideOptions::OverridePolicy),
                    "INCLUDE_POLICY" => Ok(rate_limit_per_route::OverrideOptions::IncludePolicy),
                    "IGNORE_POLICY" => Ok(rate_limit_per_route::OverrideOptions::IgnorePolicy),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_per_route::VhRateLimitsOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Override => "OVERRIDE",
            Self::Include => "INCLUDE",
            Self::Ignore => "IGNORE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_per_route::VhRateLimitsOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OVERRIDE",
            "INCLUDE",
            "IGNORE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_per_route::VhRateLimitsOptions;

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
                    .and_then(rate_limit_per_route::VhRateLimitsOptions::from_i32)
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
                    .and_then(rate_limit_per_route::VhRateLimitsOptions::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OVERRIDE" => Ok(rate_limit_per_route::VhRateLimitsOptions::Override),
                    "INCLUDE" => Ok(rate_limit_per_route::VhRateLimitsOptions::Include),
                    "IGNORE" => Ok(rate_limit_per_route::VhRateLimitsOptions::Ignore),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
