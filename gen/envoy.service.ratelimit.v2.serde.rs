// @generated
impl serde::Serialize for RateLimitRequest {
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
        if !self.descriptors.is_empty() {
            len += 1;
        }
        if self.hits_addend != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v2.RateLimitRequest", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.descriptors.is_empty() {
            struct_ser.serialize_field("descriptors", &self.descriptors)?;
        }
        if self.hits_addend != 0 {
            struct_ser.serialize_field("hitsAddend", &self.hits_addend)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "descriptors",
            "hits_addend",
            "hitsAddend",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            Descriptors,
            HitsAddend,
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
                            "descriptors" => Ok(GeneratedField::Descriptors),
                            "hitsAddend" | "hits_addend" => Ok(GeneratedField::HitsAddend),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v2.RateLimitRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut descriptors__ = None;
                let mut hits_addend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map.next_value()?);
                        }
                        GeneratedField::Descriptors => {
                            if descriptors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptors"));
                            }
                            descriptors__ = Some(map.next_value()?);
                        }
                        GeneratedField::HitsAddend => {
                            if hits_addend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hitsAddend"));
                            }
                            hits_addend__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RateLimitRequest {
                    domain: domain__.unwrap_or_default(),
                    descriptors: descriptors__.unwrap_or_default(),
                    hits_addend: hits_addend__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v2.RateLimitRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.overall_code != 0 {
            len += 1;
        }
        if !self.statuses.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v2.RateLimitResponse", len)?;
        if self.overall_code != 0 {
            let v = rate_limit_response::Code::from_i32(self.overall_code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.overall_code)))?;
            struct_ser.serialize_field("overallCode", &v)?;
        }
        if !self.statuses.is_empty() {
            struct_ser.serialize_field("statuses", &self.statuses)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("requestHeadersToAdd", &self.request_headers_to_add)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "overall_code",
            "overallCode",
            "statuses",
            "headers",
            "request_headers_to_add",
            "requestHeadersToAdd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OverallCode,
            Statuses,
            Headers,
            RequestHeadersToAdd,
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
                            "overallCode" | "overall_code" => Ok(GeneratedField::OverallCode),
                            "statuses" => Ok(GeneratedField::Statuses),
                            "headers" => Ok(GeneratedField::Headers),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v2.RateLimitResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut overall_code__ = None;
                let mut statuses__ = None;
                let mut headers__ = None;
                let mut request_headers_to_add__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OverallCode => {
                            if overall_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overallCode"));
                            }
                            overall_code__ = Some(map.next_value::<rate_limit_response::Code>()? as i32);
                        }
                        GeneratedField::Statuses => {
                            if statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statuses"));
                            }
                            statuses__ = Some(map.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RateLimitResponse {
                    overall_code: overall_code__.unwrap_or_default(),
                    statuses: statuses__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v2.RateLimitResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::Code {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Ok => "OK",
            Self::OverLimit => "OVER_LIMIT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::Code {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "OK",
            "OVER_LIMIT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::Code;

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
                    .and_then(rate_limit_response::Code::from_i32)
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
                    .and_then(rate_limit_response::Code::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(rate_limit_response::Code::Unknown),
                    "OK" => Ok(rate_limit_response::Code::Ok),
                    "OVER_LIMIT" => Ok(rate_limit_response::Code::OverLimit),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::DescriptorStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        if self.current_limit.is_some() {
            len += 1;
        }
        if self.limit_remaining != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v2.RateLimitResponse.DescriptorStatus", len)?;
        if self.code != 0 {
            let v = rate_limit_response::Code::from_i32(self.code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        if let Some(v) = self.current_limit.as_ref() {
            struct_ser.serialize_field("currentLimit", v)?;
        }
        if self.limit_remaining != 0 {
            struct_ser.serialize_field("limitRemaining", &self.limit_remaining)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::DescriptorStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
            "current_limit",
            "currentLimit",
            "limit_remaining",
            "limitRemaining",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
            CurrentLimit,
            LimitRemaining,
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
                            "code" => Ok(GeneratedField::Code),
                            "currentLimit" | "current_limit" => Ok(GeneratedField::CurrentLimit),
                            "limitRemaining" | "limit_remaining" => Ok(GeneratedField::LimitRemaining),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::DescriptorStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v2.RateLimitResponse.DescriptorStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_response::DescriptorStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                let mut current_limit__ = None;
                let mut limit_remaining__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value::<rate_limit_response::Code>()? as i32);
                        }
                        GeneratedField::CurrentLimit => {
                            if current_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentLimit"));
                            }
                            current_limit__ = map.next_value()?;
                        }
                        GeneratedField::LimitRemaining => {
                            if limit_remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limitRemaining"));
                            }
                            limit_remaining__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(rate_limit_response::DescriptorStatus {
                    code: code__.unwrap_or_default(),
                    current_limit: current_limit__,
                    limit_remaining: limit_remaining__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v2.RateLimitResponse.DescriptorStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::RateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.requests_per_unit != 0 {
            len += 1;
        }
        if self.unit != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.ratelimit.v2.RateLimitResponse.RateLimit", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.requests_per_unit != 0 {
            struct_ser.serialize_field("requestsPerUnit", &self.requests_per_unit)?;
        }
        if self.unit != 0 {
            let v = rate_limit_response::rate_limit::Unit::from_i32(self.unit)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.unit)))?;
            struct_ser.serialize_field("unit", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "requests_per_unit",
            "requestsPerUnit",
            "unit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RequestsPerUnit,
            Unit,
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
                            "name" => Ok(GeneratedField::Name),
                            "requestsPerUnit" | "requests_per_unit" => Ok(GeneratedField::RequestsPerUnit),
                            "unit" => Ok(GeneratedField::Unit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::RateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.ratelimit.v2.RateLimitResponse.RateLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rate_limit_response::RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut requests_per_unit__ = None;
                let mut unit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestsPerUnit => {
                            if requests_per_unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestsPerUnit"));
                            }
                            requests_per_unit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map.next_value::<rate_limit_response::rate_limit::Unit>()? as i32);
                        }
                    }
                }
                Ok(rate_limit_response::RateLimit {
                    name: name__.unwrap_or_default(),
                    requests_per_unit: requests_per_unit__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.ratelimit.v2.RateLimitResponse.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rate_limit_response::rate_limit::Unit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Second => "SECOND",
            Self::Minute => "MINUTE",
            Self::Hour => "HOUR",
            Self::Day => "DAY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for rate_limit_response::rate_limit::Unit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "SECOND",
            "MINUTE",
            "HOUR",
            "DAY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rate_limit_response::rate_limit::Unit;

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
                    .and_then(rate_limit_response::rate_limit::Unit::from_i32)
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
                    .and_then(rate_limit_response::rate_limit::Unit::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(rate_limit_response::rate_limit::Unit::Unknown),
                    "SECOND" => Ok(rate_limit_response::rate_limit::Unit::Second),
                    "MINUTE" => Ok(rate_limit_response::rate_limit::Unit::Minute),
                    "HOUR" => Ok(rate_limit_response::rate_limit::Unit::Hour),
                    "DAY" => Ok(rate_limit_response::rate_limit::Unit::Day),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
