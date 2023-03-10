// @generated
impl serde::Serialize for LocalRateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.token_bucket.is_some() {
            len += 1;
        }
        if self.filter_enabled.is_some() {
            len += 1;
        }
        if self.filter_enforced.is_some() {
            len += 1;
        }
        if !self.request_headers_to_add_when_not_enforced.is_empty() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.descriptors.is_empty() {
            len += 1;
        }
        if self.stage != 0 {
            len += 1;
        }
        if self.local_rate_limit_per_downstream_connection {
            len += 1;
        }
        if self.enable_x_ratelimit_headers != 0 {
            len += 1;
        }
        if self.vh_rate_limits != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.local_ratelimit.v3.LocalRateLimit", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        if let Some(v) = self.token_bucket.as_ref() {
            struct_ser.serialize_field("tokenBucket", v)?;
        }
        if let Some(v) = self.filter_enabled.as_ref() {
            struct_ser.serialize_field("filterEnabled", v)?;
        }
        if let Some(v) = self.filter_enforced.as_ref() {
            struct_ser.serialize_field("filterEnforced", v)?;
        }
        if !self.request_headers_to_add_when_not_enforced.is_empty() {
            struct_ser.serialize_field("requestHeadersToAddWhenNotEnforced", &self.request_headers_to_add_when_not_enforced)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        if !self.descriptors.is_empty() {
            struct_ser.serialize_field("descriptors", &self.descriptors)?;
        }
        if self.stage != 0 {
            struct_ser.serialize_field("stage", &self.stage)?;
        }
        if self.local_rate_limit_per_downstream_connection {
            struct_ser.serialize_field("localRateLimitPerDownstreamConnection", &self.local_rate_limit_per_downstream_connection)?;
        }
        if self.enable_x_ratelimit_headers != 0 {
            let v = super::super::super::super::common::ratelimit::v3::XRateLimitHeadersRfcVersion::from_i32(self.enable_x_ratelimit_headers)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.enable_x_ratelimit_headers)))?;
            struct_ser.serialize_field("enableXRatelimitHeaders", &v)?;
        }
        if self.vh_rate_limits != 0 {
            let v = super::super::super::super::common::ratelimit::v3::VhRateLimitsOptions::from_i32(self.vh_rate_limits)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.vh_rate_limits)))?;
            struct_ser.serialize_field("vhRateLimits", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalRateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "status",
            "token_bucket",
            "tokenBucket",
            "filter_enabled",
            "filterEnabled",
            "filter_enforced",
            "filterEnforced",
            "request_headers_to_add_when_not_enforced",
            "requestHeadersToAddWhenNotEnforced",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "descriptors",
            "stage",
            "local_rate_limit_per_downstream_connection",
            "localRateLimitPerDownstreamConnection",
            "enable_x_ratelimit_headers",
            "enableXRatelimitHeaders",
            "vh_rate_limits",
            "vhRateLimits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            Status,
            TokenBucket,
            FilterEnabled,
            FilterEnforced,
            RequestHeadersToAddWhenNotEnforced,
            ResponseHeadersToAdd,
            Descriptors,
            Stage,
            LocalRateLimitPerDownstreamConnection,
            EnableXRatelimitHeaders,
            VhRateLimits,
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
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "status" => Ok(GeneratedField::Status),
                            "tokenBucket" | "token_bucket" => Ok(GeneratedField::TokenBucket),
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            "filterEnforced" | "filter_enforced" => Ok(GeneratedField::FilterEnforced),
                            "requestHeadersToAddWhenNotEnforced" | "request_headers_to_add_when_not_enforced" => Ok(GeneratedField::RequestHeadersToAddWhenNotEnforced),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "descriptors" => Ok(GeneratedField::Descriptors),
                            "stage" => Ok(GeneratedField::Stage),
                            "localRateLimitPerDownstreamConnection" | "local_rate_limit_per_downstream_connection" => Ok(GeneratedField::LocalRateLimitPerDownstreamConnection),
                            "enableXRatelimitHeaders" | "enable_x_ratelimit_headers" => Ok(GeneratedField::EnableXRatelimitHeaders),
                            "vhRateLimits" | "vh_rate_limits" => Ok(GeneratedField::VhRateLimits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalRateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.local_ratelimit.v3.LocalRateLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LocalRateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut status__ = None;
                let mut token_bucket__ = None;
                let mut filter_enabled__ = None;
                let mut filter_enforced__ = None;
                let mut request_headers_to_add_when_not_enforced__ = None;
                let mut response_headers_to_add__ = None;
                let mut descriptors__ = None;
                let mut stage__ = None;
                let mut local_rate_limit_per_downstream_connection__ = None;
                let mut enable_x_ratelimit_headers__ = None;
                let mut vh_rate_limits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value()?;
                        }
                        GeneratedField::TokenBucket => {
                            if token_bucket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenBucket"));
                            }
                            token_bucket__ = map.next_value()?;
                        }
                        GeneratedField::FilterEnabled => {
                            if filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabled"));
                            }
                            filter_enabled__ = map.next_value()?;
                        }
                        GeneratedField::FilterEnforced => {
                            if filter_enforced__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnforced"));
                            }
                            filter_enforced__ = map.next_value()?;
                        }
                        GeneratedField::RequestHeadersToAddWhenNotEnforced => {
                            if request_headers_to_add_when_not_enforced__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAddWhenNotEnforced"));
                            }
                            request_headers_to_add_when_not_enforced__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::Descriptors => {
                            if descriptors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptors"));
                            }
                            descriptors__ = Some(map.next_value()?);
                        }
                        GeneratedField::Stage => {
                            if stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            stage__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LocalRateLimitPerDownstreamConnection => {
                            if local_rate_limit_per_downstream_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localRateLimitPerDownstreamConnection"));
                            }
                            local_rate_limit_per_downstream_connection__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnableXRatelimitHeaders => {
                            if enable_x_ratelimit_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableXRatelimitHeaders"));
                            }
                            enable_x_ratelimit_headers__ = Some(map.next_value::<super::super::super::super::common::ratelimit::v3::XRateLimitHeadersRfcVersion>()? as i32);
                        }
                        GeneratedField::VhRateLimits => {
                            if vh_rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vhRateLimits"));
                            }
                            vh_rate_limits__ = Some(map.next_value::<super::super::super::super::common::ratelimit::v3::VhRateLimitsOptions>()? as i32);
                        }
                    }
                }
                Ok(LocalRateLimit {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    status: status__,
                    token_bucket: token_bucket__,
                    filter_enabled: filter_enabled__,
                    filter_enforced: filter_enforced__,
                    request_headers_to_add_when_not_enforced: request_headers_to_add_when_not_enforced__.unwrap_or_default(),
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    descriptors: descriptors__.unwrap_or_default(),
                    stage: stage__.unwrap_or_default(),
                    local_rate_limit_per_downstream_connection: local_rate_limit_per_downstream_connection__.unwrap_or_default(),
                    enable_x_ratelimit_headers: enable_x_ratelimit_headers__.unwrap_or_default(),
                    vh_rate_limits: vh_rate_limits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.local_ratelimit.v3.LocalRateLimit", FIELDS, GeneratedVisitor)
    }
}
