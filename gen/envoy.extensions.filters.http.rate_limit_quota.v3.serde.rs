// @generated
impl serde::Serialize for RateLimitQuotaBucketSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bucket_id_builder.is_some() {
            len += 1;
        }
        if self.reporting_interval.is_some() {
            len += 1;
        }
        if self.deny_response_settings.is_some() {
            len += 1;
        }
        if self.no_assignment_behavior.is_some() {
            len += 1;
        }
        if self.expired_assignment_behavior.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings", len)?;
        if let Some(v) = self.bucket_id_builder.as_ref() {
            struct_ser.serialize_field("bucketIdBuilder", v)?;
        }
        if let Some(v) = self.reporting_interval.as_ref() {
            struct_ser.serialize_field("reportingInterval", v)?;
        }
        if let Some(v) = self.deny_response_settings.as_ref() {
            struct_ser.serialize_field("denyResponseSettings", v)?;
        }
        if let Some(v) = self.no_assignment_behavior.as_ref() {
            struct_ser.serialize_field("noAssignmentBehavior", v)?;
        }
        if let Some(v) = self.expired_assignment_behavior.as_ref() {
            struct_ser.serialize_field("expiredAssignmentBehavior", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitQuotaBucketSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bucket_id_builder",
            "bucketIdBuilder",
            "reporting_interval",
            "reportingInterval",
            "deny_response_settings",
            "denyResponseSettings",
            "no_assignment_behavior",
            "noAssignmentBehavior",
            "expired_assignment_behavior",
            "expiredAssignmentBehavior",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BucketIdBuilder,
            ReportingInterval,
            DenyResponseSettings,
            NoAssignmentBehavior,
            ExpiredAssignmentBehavior,
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
                            "bucketIdBuilder" | "bucket_id_builder" => Ok(GeneratedField::BucketIdBuilder),
                            "reportingInterval" | "reporting_interval" => Ok(GeneratedField::ReportingInterval),
                            "denyResponseSettings" | "deny_response_settings" => Ok(GeneratedField::DenyResponseSettings),
                            "noAssignmentBehavior" | "no_assignment_behavior" => Ok(GeneratedField::NoAssignmentBehavior),
                            "expiredAssignmentBehavior" | "expired_assignment_behavior" => Ok(GeneratedField::ExpiredAssignmentBehavior),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitQuotaBucketSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitQuotaBucketSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bucket_id_builder__ = None;
                let mut reporting_interval__ = None;
                let mut deny_response_settings__ = None;
                let mut no_assignment_behavior__ = None;
                let mut expired_assignment_behavior__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BucketIdBuilder => {
                            if bucket_id_builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketIdBuilder"));
                            }
                            bucket_id_builder__ = map.next_value()?;
                        }
                        GeneratedField::ReportingInterval => {
                            if reporting_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reportingInterval"));
                            }
                            reporting_interval__ = map.next_value()?;
                        }
                        GeneratedField::DenyResponseSettings => {
                            if deny_response_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denyResponseSettings"));
                            }
                            deny_response_settings__ = map.next_value()?;
                        }
                        GeneratedField::NoAssignmentBehavior => {
                            if no_assignment_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noAssignmentBehavior"));
                            }
                            no_assignment_behavior__ = map.next_value()?;
                        }
                        GeneratedField::ExpiredAssignmentBehavior => {
                            if expired_assignment_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiredAssignmentBehavior"));
                            }
                            expired_assignment_behavior__ = map.next_value()?;
                        }
                    }
                }
                Ok(RateLimitQuotaBucketSettings {
                    bucket_id_builder: bucket_id_builder__,
                    reporting_interval: reporting_interval__,
                    deny_response_settings: deny_response_settings__,
                    no_assignment_behavior: no_assignment_behavior__,
                    expired_assignment_behavior: expired_assignment_behavior__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_bucket_settings::BucketIdBuilder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bucket_id_builder.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder", len)?;
        if !self.bucket_id_builder.is_empty() {
            struct_ser.serialize_field("bucketIdBuilder", &self.bucket_id_builder)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_bucket_settings::BucketIdBuilder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bucket_id_builder",
            "bucketIdBuilder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BucketIdBuilder,
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
                            "bucketIdBuilder" | "bucket_id_builder" => Ok(GeneratedField::BucketIdBuilder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_quota_bucket_settings::BucketIdBuilder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_quota_bucket_settings::BucketIdBuilder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bucket_id_builder__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BucketIdBuilder => {
                            if bucket_id_builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketIdBuilder"));
                            }
                            bucket_id_builder__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(rate_limit_quota_bucket_settings::BucketIdBuilder {
                    bucket_id_builder: bucket_id_builder__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder.ValueBuilder", len)?;
        if let Some(v) = self.value_specifier.as_ref() {
            match v {
                rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifier::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifier::CustomValue(v) => {
                    struct_ser.serialize_field("customValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "string_value",
            "stringValue",
            "custom_value",
            "customValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StringValue,
            CustomValue,
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
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "customValue" | "custom_value" => Ok(GeneratedField::CustomValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder.ValueBuilder")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StringValue => {
                            if value_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifier::StringValue);
                        }
                        GeneratedField::CustomValue => {
                            if value_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customValue"));
                            }
                            value_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_quota_bucket_settings::bucket_id_builder::value_builder::ValueSpecifier::CustomValue)
;
                        }
                    }
                }
                Ok(rate_limit_quota_bucket_settings::bucket_id_builder::ValueBuilder {
                    value_specifier: value_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.BucketIdBuilder.ValueBuilder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_bucket_settings::DenyResponseSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_status.is_some() {
            len += 1;
        }
        if self.http_body.is_some() {
            len += 1;
        }
        if self.grpc_status.is_some() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.DenyResponseSettings", len)?;
        if let Some(v) = self.http_status.as_ref() {
            struct_ser.serialize_field("httpStatus", v)?;
        }
        if let Some(v) = self.http_body.as_ref() {
            struct_ser.serialize_field("httpBody", v)?;
        }
        if let Some(v) = self.grpc_status.as_ref() {
            struct_ser.serialize_field("grpcStatus", v)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_bucket_settings::DenyResponseSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_status",
            "httpStatus",
            "http_body",
            "httpBody",
            "grpc_status",
            "grpcStatus",
            "response_headers_to_add",
            "responseHeadersToAdd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpStatus,
            HttpBody,
            GrpcStatus,
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
                            "httpStatus" | "http_status" => Ok(GeneratedField::HttpStatus),
                            "httpBody" | "http_body" => Ok(GeneratedField::HttpBody),
                            "grpcStatus" | "grpc_status" => Ok(GeneratedField::GrpcStatus),
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
            type Value = rate_limit_quota_bucket_settings::DenyResponseSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.DenyResponseSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_quota_bucket_settings::DenyResponseSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_status__ = None;
                let mut http_body__ = None;
                let mut grpc_status__ = None;
                let mut response_headers_to_add__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpStatus => {
                            if http_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpStatus"));
                            }
                            http_status__ = map.next_value()?;
                        }
                        GeneratedField::HttpBody => {
                            if http_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpBody"));
                            }
                            http_body__ = map.next_value()?;
                        }
                        GeneratedField::GrpcStatus => {
                            if grpc_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcStatus"));
                            }
                            grpc_status__ = map.next_value()?;
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(rate_limit_quota_bucket_settings::DenyResponseSettings {
                    http_status: http_status__,
                    http_body: http_body__,
                    grpc_status: grpc_status__,
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.DenyResponseSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expired_assignment_behavior_timeout.is_some() {
            len += 1;
        }
        if self.expired_assignment_behavior.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior", len)?;
        if let Some(v) = self.expired_assignment_behavior_timeout.as_ref() {
            struct_ser.serialize_field("expiredAssignmentBehaviorTimeout", v)?;
        }
        if let Some(v) = self.expired_assignment_behavior.as_ref() {
            match v {
                rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehavior::FallbackRateLimit(v) => {
                    struct_ser.serialize_field("fallbackRateLimit", v)?;
                }
                rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehavior::ReuseLastAssignment(v) => {
                    struct_ser.serialize_field("reuseLastAssignment", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expired_assignment_behavior_timeout",
            "expiredAssignmentBehaviorTimeout",
            "fallback_rate_limit",
            "fallbackRateLimit",
            "reuse_last_assignment",
            "reuseLastAssignment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpiredAssignmentBehaviorTimeout,
            FallbackRateLimit,
            ReuseLastAssignment,
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
                            "expiredAssignmentBehaviorTimeout" | "expired_assignment_behavior_timeout" => Ok(GeneratedField::ExpiredAssignmentBehaviorTimeout),
                            "fallbackRateLimit" | "fallback_rate_limit" => Ok(GeneratedField::FallbackRateLimit),
                            "reuseLastAssignment" | "reuse_last_assignment" => Ok(GeneratedField::ReuseLastAssignment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expired_assignment_behavior_timeout__ = None;
                let mut expired_assignment_behavior__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExpiredAssignmentBehaviorTimeout => {
                            if expired_assignment_behavior_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiredAssignmentBehaviorTimeout"));
                            }
                            expired_assignment_behavior_timeout__ = map.next_value()?;
                        }
                        GeneratedField::FallbackRateLimit => {
                            if expired_assignment_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackRateLimit"));
                            }
                            expired_assignment_behavior__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehavior::FallbackRateLimit)
;
                        }
                        GeneratedField::ReuseLastAssignment => {
                            if expired_assignment_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reuseLastAssignment"));
                            }
                            expired_assignment_behavior__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_quota_bucket_settings::expired_assignment_behavior::ExpiredAssignmentBehavior::ReuseLastAssignment)
;
                        }
                    }
                }
                Ok(rate_limit_quota_bucket_settings::ExpiredAssignmentBehavior {
                    expired_assignment_behavior_timeout: expired_assignment_behavior_timeout__,
                    expired_assignment_behavior: expired_assignment_behavior__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior.ReuseLastAssignment", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment {
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
            type Value = rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior.ReuseLastAssignment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(rate_limit_quota_bucket_settings::expired_assignment_behavior::ReuseLastAssignment {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.ExpiredAssignmentBehavior.ReuseLastAssignment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_quota_bucket_settings::NoAssignmentBehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.no_assignment_behavior.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.NoAssignmentBehavior", len)?;
        if let Some(v) = self.no_assignment_behavior.as_ref() {
            match v {
                rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehavior::FallbackRateLimit(v) => {
                    struct_ser.serialize_field("fallbackRateLimit", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_quota_bucket_settings::NoAssignmentBehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fallback_rate_limit",
            "fallbackRateLimit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FallbackRateLimit,
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
                            "fallbackRateLimit" | "fallback_rate_limit" => Ok(GeneratedField::FallbackRateLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_quota_bucket_settings::NoAssignmentBehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.NoAssignmentBehavior")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_quota_bucket_settings::NoAssignmentBehavior, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut no_assignment_behavior__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FallbackRateLimit => {
                            if no_assignment_behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fallbackRateLimit"));
                            }
                            no_assignment_behavior__ = map.next_value::<::std::option::Option<_>>()?.map(rate_limit_quota_bucket_settings::no_assignment_behavior::NoAssignmentBehavior::FallbackRateLimit)
;
                        }
                    }
                }
                Ok(rate_limit_quota_bucket_settings::NoAssignmentBehavior {
                    no_assignment_behavior: no_assignment_behavior__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaBucketSettings.NoAssignmentBehavior", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitQuotaFilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rlqs_server.is_some() {
            len += 1;
        }
        if !self.domain.is_empty() {
            len += 1;
        }
        if self.bucket_matchers.is_some() {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaFilterConfig", len)?;
        if let Some(v) = self.rlqs_server.as_ref() {
            struct_ser.serialize_field("rlqsServer", v)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if let Some(v) = self.bucket_matchers.as_ref() {
            struct_ser.serialize_field("bucketMatchers", v)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitQuotaFilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rlqs_server",
            "rlqsServer",
            "domain",
            "bucket_matchers",
            "bucketMatchers",
            "filter_enabled",
            "filterEnabled",
            "filter_enforced",
            "filterEnforced",
            "request_headers_to_add_when_not_enforced",
            "requestHeadersToAddWhenNotEnforced",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RlqsServer,
            Domain,
            BucketMatchers,
            FilterEnabled,
            FilterEnforced,
            RequestHeadersToAddWhenNotEnforced,
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
                            "rlqsServer" | "rlqs_server" => Ok(GeneratedField::RlqsServer),
                            "domain" => Ok(GeneratedField::Domain),
                            "bucketMatchers" | "bucket_matchers" => Ok(GeneratedField::BucketMatchers),
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            "filterEnforced" | "filter_enforced" => Ok(GeneratedField::FilterEnforced),
                            "requestHeadersToAddWhenNotEnforced" | "request_headers_to_add_when_not_enforced" => Ok(GeneratedField::RequestHeadersToAddWhenNotEnforced),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitQuotaFilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaFilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitQuotaFilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rlqs_server__ = None;
                let mut domain__ = None;
                let mut bucket_matchers__ = None;
                let mut filter_enabled__ = None;
                let mut filter_enforced__ = None;
                let mut request_headers_to_add_when_not_enforced__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RlqsServer => {
                            if rlqs_server__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rlqsServer"));
                            }
                            rlqs_server__ = map.next_value()?;
                        }
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map.next_value()?);
                        }
                        GeneratedField::BucketMatchers => {
                            if bucket_matchers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketMatchers"));
                            }
                            bucket_matchers__ = map.next_value()?;
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
                    }
                }
                Ok(RateLimitQuotaFilterConfig {
                    rlqs_server: rlqs_server__,
                    domain: domain__.unwrap_or_default(),
                    bucket_matchers: bucket_matchers__,
                    filter_enabled: filter_enabled__,
                    filter_enforced: filter_enforced__,
                    request_headers_to_add_when_not_enforced: request_headers_to_add_when_not_enforced__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaFilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitQuotaOverride {
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
        if self.bucket_matchers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaOverride", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if let Some(v) = self.bucket_matchers.as_ref() {
            struct_ser.serialize_field("bucketMatchers", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitQuotaOverride {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "bucket_matchers",
            "bucketMatchers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            BucketMatchers,
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
                            "bucketMatchers" | "bucket_matchers" => Ok(GeneratedField::BucketMatchers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitQuotaOverride;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaOverride")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitQuotaOverride, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut bucket_matchers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map.next_value()?);
                        }
                        GeneratedField::BucketMatchers => {
                            if bucket_matchers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bucketMatchers"));
                            }
                            bucket_matchers__ = map.next_value()?;
                        }
                    }
                }
                Ok(RateLimitQuotaOverride {
                    domain: domain__.unwrap_or_default(),
                    bucket_matchers: bucket_matchers__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.rate_limit_quota.v3.RateLimitQuotaOverride", FIELDS, GeneratedVisitor)
    }
}
