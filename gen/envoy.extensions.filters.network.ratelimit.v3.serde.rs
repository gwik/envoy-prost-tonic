// @generated
impl serde::Serialize for RateLimit {
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
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.descriptors.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.failure_mode_deny {
            len += 1;
        }
        if self.rate_limit_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.ratelimit.v3.RateLimit", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.descriptors.is_empty() {
            struct_ser.serialize_field("descriptors", &self.descriptors)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if self.failure_mode_deny {
            struct_ser.serialize_field("failureModeDeny", &self.failure_mode_deny)?;
        }
        if let Some(v) = self.rate_limit_service.as_ref() {
            struct_ser.serialize_field("rateLimitService", v)?;
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
            "stat_prefix",
            "statPrefix",
            "domain",
            "descriptors",
            "timeout",
            "failure_mode_deny",
            "failureModeDeny",
            "rate_limit_service",
            "rateLimitService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            Domain,
            Descriptors,
            Timeout,
            FailureModeDeny,
            RateLimitService,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "descriptors" => Ok(GeneratedField::Descriptors),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "failureModeDeny" | "failure_mode_deny" => Ok(GeneratedField::FailureModeDeny),
                            "rateLimitService" | "rate_limit_service" => Ok(GeneratedField::RateLimitService),
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
                formatter.write_str("struct envoy.extensions.filters.network.ratelimit.v3.RateLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut domain__ = None;
                let mut descriptors__ = None;
                let mut timeout__ = None;
                let mut failure_mode_deny__ = None;
                let mut rate_limit_service__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
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
                        GeneratedField::RateLimitService => {
                            if rate_limit_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitService"));
                            }
                            rate_limit_service__ = map.next_value()?;
                        }
                    }
                }
                Ok(RateLimit {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    domain: domain__.unwrap_or_default(),
                    descriptors: descriptors__.unwrap_or_default(),
                    timeout: timeout__,
                    failure_mode_deny: failure_mode_deny__.unwrap_or_default(),
                    rate_limit_service: rate_limit_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.ratelimit.v3.RateLimit", FIELDS, GeneratedVisitor)
    }
}
