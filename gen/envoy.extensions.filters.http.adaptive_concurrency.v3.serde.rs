// @generated
impl serde::Serialize for AdaptiveConcurrency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled.is_some() {
            len += 1;
        }
        if self.concurrency_limit_exceeded_status.is_some() {
            len += 1;
        }
        if self.concurrency_controller_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.AdaptiveConcurrency", len)?;
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        if let Some(v) = self.concurrency_limit_exceeded_status.as_ref() {
            struct_ser.serialize_field("concurrencyLimitExceededStatus", v)?;
        }
        if let Some(v) = self.concurrency_controller_config.as_ref() {
            match v {
                adaptive_concurrency::ConcurrencyControllerConfig::GradientControllerConfig(v) => {
                    struct_ser.serialize_field("gradientControllerConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdaptiveConcurrency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
            "concurrency_limit_exceeded_status",
            "concurrencyLimitExceededStatus",
            "gradient_controller_config",
            "gradientControllerConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            ConcurrencyLimitExceededStatus,
            GradientControllerConfig,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "concurrencyLimitExceededStatus" | "concurrency_limit_exceeded_status" => Ok(GeneratedField::ConcurrencyLimitExceededStatus),
                            "gradientControllerConfig" | "gradient_controller_config" => Ok(GeneratedField::GradientControllerConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdaptiveConcurrency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.adaptive_concurrency.v3.AdaptiveConcurrency")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AdaptiveConcurrency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut concurrency_limit_exceeded_status__ = None;
                let mut concurrency_controller_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map.next_value()?;
                        }
                        GeneratedField::ConcurrencyLimitExceededStatus => {
                            if concurrency_limit_exceeded_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrencyLimitExceededStatus"));
                            }
                            concurrency_limit_exceeded_status__ = map.next_value()?;
                        }
                        GeneratedField::GradientControllerConfig => {
                            if concurrency_controller_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gradientControllerConfig"));
                            }
                            concurrency_controller_config__ = map.next_value::<::std::option::Option<_>>()?.map(adaptive_concurrency::ConcurrencyControllerConfig::GradientControllerConfig)
;
                        }
                    }
                }
                Ok(AdaptiveConcurrency {
                    enabled: enabled__,
                    concurrency_limit_exceeded_status: concurrency_limit_exceeded_status__,
                    concurrency_controller_config: concurrency_controller_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.AdaptiveConcurrency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GradientControllerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sample_aggregate_percentile.is_some() {
            len += 1;
        }
        if self.concurrency_limit_params.is_some() {
            len += 1;
        }
        if self.min_rtt_calc_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig", len)?;
        if let Some(v) = self.sample_aggregate_percentile.as_ref() {
            struct_ser.serialize_field("sampleAggregatePercentile", v)?;
        }
        if let Some(v) = self.concurrency_limit_params.as_ref() {
            struct_ser.serialize_field("concurrencyLimitParams", v)?;
        }
        if let Some(v) = self.min_rtt_calc_params.as_ref() {
            struct_ser.serialize_field("minRttCalcParams", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GradientControllerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sample_aggregate_percentile",
            "sampleAggregatePercentile",
            "concurrency_limit_params",
            "concurrencyLimitParams",
            "min_rtt_calc_params",
            "minRttCalcParams",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SampleAggregatePercentile,
            ConcurrencyLimitParams,
            MinRttCalcParams,
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
                            "sampleAggregatePercentile" | "sample_aggregate_percentile" => Ok(GeneratedField::SampleAggregatePercentile),
                            "concurrencyLimitParams" | "concurrency_limit_params" => Ok(GeneratedField::ConcurrencyLimitParams),
                            "minRttCalcParams" | "min_rtt_calc_params" => Ok(GeneratedField::MinRttCalcParams),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GradientControllerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GradientControllerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sample_aggregate_percentile__ = None;
                let mut concurrency_limit_params__ = None;
                let mut min_rtt_calc_params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SampleAggregatePercentile => {
                            if sample_aggregate_percentile__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampleAggregatePercentile"));
                            }
                            sample_aggregate_percentile__ = map.next_value()?;
                        }
                        GeneratedField::ConcurrencyLimitParams => {
                            if concurrency_limit_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrencyLimitParams"));
                            }
                            concurrency_limit_params__ = map.next_value()?;
                        }
                        GeneratedField::MinRttCalcParams => {
                            if min_rtt_calc_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRttCalcParams"));
                            }
                            min_rtt_calc_params__ = map.next_value()?;
                        }
                    }
                }
                Ok(GradientControllerConfig {
                    sample_aggregate_percentile: sample_aggregate_percentile__,
                    concurrency_limit_params: concurrency_limit_params__,
                    min_rtt_calc_params: min_rtt_calc_params__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for gradient_controller_config::ConcurrencyLimitCalculationParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_concurrency_limit.is_some() {
            len += 1;
        }
        if self.concurrency_update_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig.ConcurrencyLimitCalculationParams", len)?;
        if let Some(v) = self.max_concurrency_limit.as_ref() {
            struct_ser.serialize_field("maxConcurrencyLimit", v)?;
        }
        if let Some(v) = self.concurrency_update_interval.as_ref() {
            struct_ser.serialize_field("concurrencyUpdateInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for gradient_controller_config::ConcurrencyLimitCalculationParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_concurrency_limit",
            "maxConcurrencyLimit",
            "concurrency_update_interval",
            "concurrencyUpdateInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxConcurrencyLimit,
            ConcurrencyUpdateInterval,
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
                            "maxConcurrencyLimit" | "max_concurrency_limit" => Ok(GeneratedField::MaxConcurrencyLimit),
                            "concurrencyUpdateInterval" | "concurrency_update_interval" => Ok(GeneratedField::ConcurrencyUpdateInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = gradient_controller_config::ConcurrencyLimitCalculationParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig.ConcurrencyLimitCalculationParams")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<gradient_controller_config::ConcurrencyLimitCalculationParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_concurrency_limit__ = None;
                let mut concurrency_update_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxConcurrencyLimit => {
                            if max_concurrency_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConcurrencyLimit"));
                            }
                            max_concurrency_limit__ = map.next_value()?;
                        }
                        GeneratedField::ConcurrencyUpdateInterval => {
                            if concurrency_update_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrencyUpdateInterval"));
                            }
                            concurrency_update_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(gradient_controller_config::ConcurrencyLimitCalculationParams {
                    max_concurrency_limit: max_concurrency_limit__,
                    concurrency_update_interval: concurrency_update_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig.ConcurrencyLimitCalculationParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for gradient_controller_config::MinimumRttCalculationParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interval.is_some() {
            len += 1;
        }
        if self.request_count.is_some() {
            len += 1;
        }
        if self.jitter.is_some() {
            len += 1;
        }
        if self.min_concurrency.is_some() {
            len += 1;
        }
        if self.buffer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig.MinimumRTTCalculationParams", len)?;
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if let Some(v) = self.request_count.as_ref() {
            struct_ser.serialize_field("requestCount", v)?;
        }
        if let Some(v) = self.jitter.as_ref() {
            struct_ser.serialize_field("jitter", v)?;
        }
        if let Some(v) = self.min_concurrency.as_ref() {
            struct_ser.serialize_field("minConcurrency", v)?;
        }
        if let Some(v) = self.buffer.as_ref() {
            struct_ser.serialize_field("buffer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for gradient_controller_config::MinimumRttCalculationParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interval",
            "request_count",
            "requestCount",
            "jitter",
            "min_concurrency",
            "minConcurrency",
            "buffer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Interval,
            RequestCount,
            Jitter,
            MinConcurrency,
            Buffer,
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
                            "interval" => Ok(GeneratedField::Interval),
                            "requestCount" | "request_count" => Ok(GeneratedField::RequestCount),
                            "jitter" => Ok(GeneratedField::Jitter),
                            "minConcurrency" | "min_concurrency" => Ok(GeneratedField::MinConcurrency),
                            "buffer" => Ok(GeneratedField::Buffer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = gradient_controller_config::MinimumRttCalculationParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig.MinimumRTTCalculationParams")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<gradient_controller_config::MinimumRttCalculationParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interval__ = None;
                let mut request_count__ = None;
                let mut jitter__ = None;
                let mut min_concurrency__ = None;
                let mut buffer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::RequestCount => {
                            if request_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestCount"));
                            }
                            request_count__ = map.next_value()?;
                        }
                        GeneratedField::Jitter => {
                            if jitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jitter"));
                            }
                            jitter__ = map.next_value()?;
                        }
                        GeneratedField::MinConcurrency => {
                            if min_concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minConcurrency"));
                            }
                            min_concurrency__ = map.next_value()?;
                        }
                        GeneratedField::Buffer => {
                            if buffer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buffer"));
                            }
                            buffer__ = map.next_value()?;
                        }
                    }
                }
                Ok(gradient_controller_config::MinimumRttCalculationParams {
                    interval: interval__,
                    request_count: request_count__,
                    jitter: jitter__,
                    min_concurrency: min_concurrency__,
                    buffer: buffer__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.adaptive_concurrency.v3.GradientControllerConfig.MinimumRTTCalculationParams", FIELDS, GeneratedVisitor)
    }
}
