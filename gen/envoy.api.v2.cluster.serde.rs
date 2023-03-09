// @generated
impl serde::Serialize for CircuitBreakers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.thresholds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.cluster.CircuitBreakers", len)?;
        if !self.thresholds.is_empty() {
            struct_ser.serialize_field("thresholds", &self.thresholds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CircuitBreakers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "thresholds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Thresholds,
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
                            "thresholds" => Ok(GeneratedField::Thresholds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CircuitBreakers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.cluster.CircuitBreakers")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CircuitBreakers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut thresholds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Thresholds => {
                            if thresholds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thresholds"));
                            }
                            thresholds__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CircuitBreakers {
                    thresholds: thresholds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.cluster.CircuitBreakers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for circuit_breakers::Thresholds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.priority != 0 {
            len += 1;
        }
        if self.max_connections.is_some() {
            len += 1;
        }
        if self.max_pending_requests.is_some() {
            len += 1;
        }
        if self.max_requests.is_some() {
            len += 1;
        }
        if self.max_retries.is_some() {
            len += 1;
        }
        if self.retry_budget.is_some() {
            len += 1;
        }
        if self.track_remaining {
            len += 1;
        }
        if self.max_connection_pools.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.cluster.CircuitBreakers.Thresholds", len)?;
        if self.priority != 0 {
            let v = super::core::RoutingPriority::from_i32(self.priority)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.priority)))?;
            struct_ser.serialize_field("priority", &v)?;
        }
        if let Some(v) = self.max_connections.as_ref() {
            struct_ser.serialize_field("maxConnections", v)?;
        }
        if let Some(v) = self.max_pending_requests.as_ref() {
            struct_ser.serialize_field("maxPendingRequests", v)?;
        }
        if let Some(v) = self.max_requests.as_ref() {
            struct_ser.serialize_field("maxRequests", v)?;
        }
        if let Some(v) = self.max_retries.as_ref() {
            struct_ser.serialize_field("maxRetries", v)?;
        }
        if let Some(v) = self.retry_budget.as_ref() {
            struct_ser.serialize_field("retryBudget", v)?;
        }
        if self.track_remaining {
            struct_ser.serialize_field("trackRemaining", &self.track_remaining)?;
        }
        if let Some(v) = self.max_connection_pools.as_ref() {
            struct_ser.serialize_field("maxConnectionPools", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for circuit_breakers::Thresholds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "priority",
            "max_connections",
            "maxConnections",
            "max_pending_requests",
            "maxPendingRequests",
            "max_requests",
            "maxRequests",
            "max_retries",
            "maxRetries",
            "retry_budget",
            "retryBudget",
            "track_remaining",
            "trackRemaining",
            "max_connection_pools",
            "maxConnectionPools",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Priority,
            MaxConnections,
            MaxPendingRequests,
            MaxRequests,
            MaxRetries,
            RetryBudget,
            TrackRemaining,
            MaxConnectionPools,
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
                            "priority" => Ok(GeneratedField::Priority),
                            "maxConnections" | "max_connections" => Ok(GeneratedField::MaxConnections),
                            "maxPendingRequests" | "max_pending_requests" => Ok(GeneratedField::MaxPendingRequests),
                            "maxRequests" | "max_requests" => Ok(GeneratedField::MaxRequests),
                            "maxRetries" | "max_retries" => Ok(GeneratedField::MaxRetries),
                            "retryBudget" | "retry_budget" => Ok(GeneratedField::RetryBudget),
                            "trackRemaining" | "track_remaining" => Ok(GeneratedField::TrackRemaining),
                            "maxConnectionPools" | "max_connection_pools" => Ok(GeneratedField::MaxConnectionPools),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = circuit_breakers::Thresholds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.cluster.CircuitBreakers.Thresholds")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<circuit_breakers::Thresholds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut priority__ = None;
                let mut max_connections__ = None;
                let mut max_pending_requests__ = None;
                let mut max_requests__ = None;
                let mut max_retries__ = None;
                let mut retry_budget__ = None;
                let mut track_remaining__ = None;
                let mut max_connection_pools__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = Some(map.next_value::<super::core::RoutingPriority>()? as i32);
                        }
                        GeneratedField::MaxConnections => {
                            if max_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnections"));
                            }
                            max_connections__ = map.next_value()?;
                        }
                        GeneratedField::MaxPendingRequests => {
                            if max_pending_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPendingRequests"));
                            }
                            max_pending_requests__ = map.next_value()?;
                        }
                        GeneratedField::MaxRequests => {
                            if max_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequests"));
                            }
                            max_requests__ = map.next_value()?;
                        }
                        GeneratedField::MaxRetries => {
                            if max_retries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRetries"));
                            }
                            max_retries__ = map.next_value()?;
                        }
                        GeneratedField::RetryBudget => {
                            if retry_budget__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryBudget"));
                            }
                            retry_budget__ = map.next_value()?;
                        }
                        GeneratedField::TrackRemaining => {
                            if track_remaining__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackRemaining"));
                            }
                            track_remaining__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxConnectionPools => {
                            if max_connection_pools__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnectionPools"));
                            }
                            max_connection_pools__ = map.next_value()?;
                        }
                    }
                }
                Ok(circuit_breakers::Thresholds {
                    priority: priority__.unwrap_or_default(),
                    max_connections: max_connections__,
                    max_pending_requests: max_pending_requests__,
                    max_requests: max_requests__,
                    max_retries: max_retries__,
                    retry_budget: retry_budget__,
                    track_remaining: track_remaining__.unwrap_or_default(),
                    max_connection_pools: max_connection_pools__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.cluster.CircuitBreakers.Thresholds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for circuit_breakers::thresholds::RetryBudget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.budget_percent.is_some() {
            len += 1;
        }
        if self.min_retry_concurrency.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.cluster.CircuitBreakers.Thresholds.RetryBudget", len)?;
        if let Some(v) = self.budget_percent.as_ref() {
            struct_ser.serialize_field("budgetPercent", v)?;
        }
        if let Some(v) = self.min_retry_concurrency.as_ref() {
            struct_ser.serialize_field("minRetryConcurrency", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for circuit_breakers::thresholds::RetryBudget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "budget_percent",
            "budgetPercent",
            "min_retry_concurrency",
            "minRetryConcurrency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BudgetPercent,
            MinRetryConcurrency,
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
                            "budgetPercent" | "budget_percent" => Ok(GeneratedField::BudgetPercent),
                            "minRetryConcurrency" | "min_retry_concurrency" => Ok(GeneratedField::MinRetryConcurrency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = circuit_breakers::thresholds::RetryBudget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.cluster.CircuitBreakers.Thresholds.RetryBudget")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<circuit_breakers::thresholds::RetryBudget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut budget_percent__ = None;
                let mut min_retry_concurrency__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BudgetPercent => {
                            if budget_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("budgetPercent"));
                            }
                            budget_percent__ = map.next_value()?;
                        }
                        GeneratedField::MinRetryConcurrency => {
                            if min_retry_concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minRetryConcurrency"));
                            }
                            min_retry_concurrency__ = map.next_value()?;
                        }
                    }
                }
                Ok(circuit_breakers::thresholds::RetryBudget {
                    budget_percent: budget_percent__,
                    min_retry_concurrency: min_retry_concurrency__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.cluster.CircuitBreakers.Thresholds.RetryBudget", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Filter {
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
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.cluster.Filter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typedConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Filter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypedConfig,
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
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Filter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.cluster.Filter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Filter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut typed_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(Filter {
                    name: name__.unwrap_or_default(),
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.cluster.Filter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutlierDetection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consecutive_5xx.is_some() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        if self.base_ejection_time.is_some() {
            len += 1;
        }
        if self.max_ejection_percent.is_some() {
            len += 1;
        }
        if self.enforcing_consecutive_5xx.is_some() {
            len += 1;
        }
        if self.enforcing_success_rate.is_some() {
            len += 1;
        }
        if self.success_rate_minimum_hosts.is_some() {
            len += 1;
        }
        if self.success_rate_request_volume.is_some() {
            len += 1;
        }
        if self.success_rate_stdev_factor.is_some() {
            len += 1;
        }
        if self.consecutive_gateway_failure.is_some() {
            len += 1;
        }
        if self.enforcing_consecutive_gateway_failure.is_some() {
            len += 1;
        }
        if self.split_external_local_origin_errors {
            len += 1;
        }
        if self.consecutive_local_origin_failure.is_some() {
            len += 1;
        }
        if self.enforcing_consecutive_local_origin_failure.is_some() {
            len += 1;
        }
        if self.enforcing_local_origin_success_rate.is_some() {
            len += 1;
        }
        if self.failure_percentage_threshold.is_some() {
            len += 1;
        }
        if self.enforcing_failure_percentage.is_some() {
            len += 1;
        }
        if self.enforcing_failure_percentage_local_origin.is_some() {
            len += 1;
        }
        if self.failure_percentage_minimum_hosts.is_some() {
            len += 1;
        }
        if self.failure_percentage_request_volume.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.cluster.OutlierDetection", len)?;
        if let Some(v) = self.consecutive_5xx.as_ref() {
            struct_ser.serialize_field("consecutive5xx", v)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if let Some(v) = self.base_ejection_time.as_ref() {
            struct_ser.serialize_field("baseEjectionTime", v)?;
        }
        if let Some(v) = self.max_ejection_percent.as_ref() {
            struct_ser.serialize_field("maxEjectionPercent", v)?;
        }
        if let Some(v) = self.enforcing_consecutive_5xx.as_ref() {
            struct_ser.serialize_field("enforcingConsecutive5xx", v)?;
        }
        if let Some(v) = self.enforcing_success_rate.as_ref() {
            struct_ser.serialize_field("enforcingSuccessRate", v)?;
        }
        if let Some(v) = self.success_rate_minimum_hosts.as_ref() {
            struct_ser.serialize_field("successRateMinimumHosts", v)?;
        }
        if let Some(v) = self.success_rate_request_volume.as_ref() {
            struct_ser.serialize_field("successRateRequestVolume", v)?;
        }
        if let Some(v) = self.success_rate_stdev_factor.as_ref() {
            struct_ser.serialize_field("successRateStdevFactor", v)?;
        }
        if let Some(v) = self.consecutive_gateway_failure.as_ref() {
            struct_ser.serialize_field("consecutiveGatewayFailure", v)?;
        }
        if let Some(v) = self.enforcing_consecutive_gateway_failure.as_ref() {
            struct_ser.serialize_field("enforcingConsecutiveGatewayFailure", v)?;
        }
        if self.split_external_local_origin_errors {
            struct_ser.serialize_field("splitExternalLocalOriginErrors", &self.split_external_local_origin_errors)?;
        }
        if let Some(v) = self.consecutive_local_origin_failure.as_ref() {
            struct_ser.serialize_field("consecutiveLocalOriginFailure", v)?;
        }
        if let Some(v) = self.enforcing_consecutive_local_origin_failure.as_ref() {
            struct_ser.serialize_field("enforcingConsecutiveLocalOriginFailure", v)?;
        }
        if let Some(v) = self.enforcing_local_origin_success_rate.as_ref() {
            struct_ser.serialize_field("enforcingLocalOriginSuccessRate", v)?;
        }
        if let Some(v) = self.failure_percentage_threshold.as_ref() {
            struct_ser.serialize_field("failurePercentageThreshold", v)?;
        }
        if let Some(v) = self.enforcing_failure_percentage.as_ref() {
            struct_ser.serialize_field("enforcingFailurePercentage", v)?;
        }
        if let Some(v) = self.enforcing_failure_percentage_local_origin.as_ref() {
            struct_ser.serialize_field("enforcingFailurePercentageLocalOrigin", v)?;
        }
        if let Some(v) = self.failure_percentage_minimum_hosts.as_ref() {
            struct_ser.serialize_field("failurePercentageMinimumHosts", v)?;
        }
        if let Some(v) = self.failure_percentage_request_volume.as_ref() {
            struct_ser.serialize_field("failurePercentageRequestVolume", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutlierDetection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consecutive_5xx",
            "consecutive5xx",
            "interval",
            "base_ejection_time",
            "baseEjectionTime",
            "max_ejection_percent",
            "maxEjectionPercent",
            "enforcing_consecutive_5xx",
            "enforcingConsecutive5xx",
            "enforcing_success_rate",
            "enforcingSuccessRate",
            "success_rate_minimum_hosts",
            "successRateMinimumHosts",
            "success_rate_request_volume",
            "successRateRequestVolume",
            "success_rate_stdev_factor",
            "successRateStdevFactor",
            "consecutive_gateway_failure",
            "consecutiveGatewayFailure",
            "enforcing_consecutive_gateway_failure",
            "enforcingConsecutiveGatewayFailure",
            "split_external_local_origin_errors",
            "splitExternalLocalOriginErrors",
            "consecutive_local_origin_failure",
            "consecutiveLocalOriginFailure",
            "enforcing_consecutive_local_origin_failure",
            "enforcingConsecutiveLocalOriginFailure",
            "enforcing_local_origin_success_rate",
            "enforcingLocalOriginSuccessRate",
            "failure_percentage_threshold",
            "failurePercentageThreshold",
            "enforcing_failure_percentage",
            "enforcingFailurePercentage",
            "enforcing_failure_percentage_local_origin",
            "enforcingFailurePercentageLocalOrigin",
            "failure_percentage_minimum_hosts",
            "failurePercentageMinimumHosts",
            "failure_percentage_request_volume",
            "failurePercentageRequestVolume",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consecutive5xx,
            Interval,
            BaseEjectionTime,
            MaxEjectionPercent,
            EnforcingConsecutive5xx,
            EnforcingSuccessRate,
            SuccessRateMinimumHosts,
            SuccessRateRequestVolume,
            SuccessRateStdevFactor,
            ConsecutiveGatewayFailure,
            EnforcingConsecutiveGatewayFailure,
            SplitExternalLocalOriginErrors,
            ConsecutiveLocalOriginFailure,
            EnforcingConsecutiveLocalOriginFailure,
            EnforcingLocalOriginSuccessRate,
            FailurePercentageThreshold,
            EnforcingFailurePercentage,
            EnforcingFailurePercentageLocalOrigin,
            FailurePercentageMinimumHosts,
            FailurePercentageRequestVolume,
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
                            "consecutive5xx" | "consecutive_5xx" => Ok(GeneratedField::Consecutive5xx),
                            "interval" => Ok(GeneratedField::Interval),
                            "baseEjectionTime" | "base_ejection_time" => Ok(GeneratedField::BaseEjectionTime),
                            "maxEjectionPercent" | "max_ejection_percent" => Ok(GeneratedField::MaxEjectionPercent),
                            "enforcingConsecutive5xx" | "enforcing_consecutive_5xx" => Ok(GeneratedField::EnforcingConsecutive5xx),
                            "enforcingSuccessRate" | "enforcing_success_rate" => Ok(GeneratedField::EnforcingSuccessRate),
                            "successRateMinimumHosts" | "success_rate_minimum_hosts" => Ok(GeneratedField::SuccessRateMinimumHosts),
                            "successRateRequestVolume" | "success_rate_request_volume" => Ok(GeneratedField::SuccessRateRequestVolume),
                            "successRateStdevFactor" | "success_rate_stdev_factor" => Ok(GeneratedField::SuccessRateStdevFactor),
                            "consecutiveGatewayFailure" | "consecutive_gateway_failure" => Ok(GeneratedField::ConsecutiveGatewayFailure),
                            "enforcingConsecutiveGatewayFailure" | "enforcing_consecutive_gateway_failure" => Ok(GeneratedField::EnforcingConsecutiveGatewayFailure),
                            "splitExternalLocalOriginErrors" | "split_external_local_origin_errors" => Ok(GeneratedField::SplitExternalLocalOriginErrors),
                            "consecutiveLocalOriginFailure" | "consecutive_local_origin_failure" => Ok(GeneratedField::ConsecutiveLocalOriginFailure),
                            "enforcingConsecutiveLocalOriginFailure" | "enforcing_consecutive_local_origin_failure" => Ok(GeneratedField::EnforcingConsecutiveLocalOriginFailure),
                            "enforcingLocalOriginSuccessRate" | "enforcing_local_origin_success_rate" => Ok(GeneratedField::EnforcingLocalOriginSuccessRate),
                            "failurePercentageThreshold" | "failure_percentage_threshold" => Ok(GeneratedField::FailurePercentageThreshold),
                            "enforcingFailurePercentage" | "enforcing_failure_percentage" => Ok(GeneratedField::EnforcingFailurePercentage),
                            "enforcingFailurePercentageLocalOrigin" | "enforcing_failure_percentage_local_origin" => Ok(GeneratedField::EnforcingFailurePercentageLocalOrigin),
                            "failurePercentageMinimumHosts" | "failure_percentage_minimum_hosts" => Ok(GeneratedField::FailurePercentageMinimumHosts),
                            "failurePercentageRequestVolume" | "failure_percentage_request_volume" => Ok(GeneratedField::FailurePercentageRequestVolume),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutlierDetection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.cluster.OutlierDetection")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OutlierDetection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consecutive_5xx__ = None;
                let mut interval__ = None;
                let mut base_ejection_time__ = None;
                let mut max_ejection_percent__ = None;
                let mut enforcing_consecutive_5xx__ = None;
                let mut enforcing_success_rate__ = None;
                let mut success_rate_minimum_hosts__ = None;
                let mut success_rate_request_volume__ = None;
                let mut success_rate_stdev_factor__ = None;
                let mut consecutive_gateway_failure__ = None;
                let mut enforcing_consecutive_gateway_failure__ = None;
                let mut split_external_local_origin_errors__ = None;
                let mut consecutive_local_origin_failure__ = None;
                let mut enforcing_consecutive_local_origin_failure__ = None;
                let mut enforcing_local_origin_success_rate__ = None;
                let mut failure_percentage_threshold__ = None;
                let mut enforcing_failure_percentage__ = None;
                let mut enforcing_failure_percentage_local_origin__ = None;
                let mut failure_percentage_minimum_hosts__ = None;
                let mut failure_percentage_request_volume__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Consecutive5xx => {
                            if consecutive_5xx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consecutive5xx"));
                            }
                            consecutive_5xx__ = map.next_value()?;
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::BaseEjectionTime => {
                            if base_ejection_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseEjectionTime"));
                            }
                            base_ejection_time__ = map.next_value()?;
                        }
                        GeneratedField::MaxEjectionPercent => {
                            if max_ejection_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEjectionPercent"));
                            }
                            max_ejection_percent__ = map.next_value()?;
                        }
                        GeneratedField::EnforcingConsecutive5xx => {
                            if enforcing_consecutive_5xx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingConsecutive5xx"));
                            }
                            enforcing_consecutive_5xx__ = map.next_value()?;
                        }
                        GeneratedField::EnforcingSuccessRate => {
                            if enforcing_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingSuccessRate"));
                            }
                            enforcing_success_rate__ = map.next_value()?;
                        }
                        GeneratedField::SuccessRateMinimumHosts => {
                            if success_rate_minimum_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRateMinimumHosts"));
                            }
                            success_rate_minimum_hosts__ = map.next_value()?;
                        }
                        GeneratedField::SuccessRateRequestVolume => {
                            if success_rate_request_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRateRequestVolume"));
                            }
                            success_rate_request_volume__ = map.next_value()?;
                        }
                        GeneratedField::SuccessRateStdevFactor => {
                            if success_rate_stdev_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRateStdevFactor"));
                            }
                            success_rate_stdev_factor__ = map.next_value()?;
                        }
                        GeneratedField::ConsecutiveGatewayFailure => {
                            if consecutive_gateway_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consecutiveGatewayFailure"));
                            }
                            consecutive_gateway_failure__ = map.next_value()?;
                        }
                        GeneratedField::EnforcingConsecutiveGatewayFailure => {
                            if enforcing_consecutive_gateway_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingConsecutiveGatewayFailure"));
                            }
                            enforcing_consecutive_gateway_failure__ = map.next_value()?;
                        }
                        GeneratedField::SplitExternalLocalOriginErrors => {
                            if split_external_local_origin_errors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splitExternalLocalOriginErrors"));
                            }
                            split_external_local_origin_errors__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConsecutiveLocalOriginFailure => {
                            if consecutive_local_origin_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consecutiveLocalOriginFailure"));
                            }
                            consecutive_local_origin_failure__ = map.next_value()?;
                        }
                        GeneratedField::EnforcingConsecutiveLocalOriginFailure => {
                            if enforcing_consecutive_local_origin_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingConsecutiveLocalOriginFailure"));
                            }
                            enforcing_consecutive_local_origin_failure__ = map.next_value()?;
                        }
                        GeneratedField::EnforcingLocalOriginSuccessRate => {
                            if enforcing_local_origin_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingLocalOriginSuccessRate"));
                            }
                            enforcing_local_origin_success_rate__ = map.next_value()?;
                        }
                        GeneratedField::FailurePercentageThreshold => {
                            if failure_percentage_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failurePercentageThreshold"));
                            }
                            failure_percentage_threshold__ = map.next_value()?;
                        }
                        GeneratedField::EnforcingFailurePercentage => {
                            if enforcing_failure_percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingFailurePercentage"));
                            }
                            enforcing_failure_percentage__ = map.next_value()?;
                        }
                        GeneratedField::EnforcingFailurePercentageLocalOrigin => {
                            if enforcing_failure_percentage_local_origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforcingFailurePercentageLocalOrigin"));
                            }
                            enforcing_failure_percentage_local_origin__ = map.next_value()?;
                        }
                        GeneratedField::FailurePercentageMinimumHosts => {
                            if failure_percentage_minimum_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failurePercentageMinimumHosts"));
                            }
                            failure_percentage_minimum_hosts__ = map.next_value()?;
                        }
                        GeneratedField::FailurePercentageRequestVolume => {
                            if failure_percentage_request_volume__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failurePercentageRequestVolume"));
                            }
                            failure_percentage_request_volume__ = map.next_value()?;
                        }
                    }
                }
                Ok(OutlierDetection {
                    consecutive_5xx: consecutive_5xx__,
                    interval: interval__,
                    base_ejection_time: base_ejection_time__,
                    max_ejection_percent: max_ejection_percent__,
                    enforcing_consecutive_5xx: enforcing_consecutive_5xx__,
                    enforcing_success_rate: enforcing_success_rate__,
                    success_rate_minimum_hosts: success_rate_minimum_hosts__,
                    success_rate_request_volume: success_rate_request_volume__,
                    success_rate_stdev_factor: success_rate_stdev_factor__,
                    consecutive_gateway_failure: consecutive_gateway_failure__,
                    enforcing_consecutive_gateway_failure: enforcing_consecutive_gateway_failure__,
                    split_external_local_origin_errors: split_external_local_origin_errors__.unwrap_or_default(),
                    consecutive_local_origin_failure: consecutive_local_origin_failure__,
                    enforcing_consecutive_local_origin_failure: enforcing_consecutive_local_origin_failure__,
                    enforcing_local_origin_success_rate: enforcing_local_origin_success_rate__,
                    failure_percentage_threshold: failure_percentage_threshold__,
                    enforcing_failure_percentage: enforcing_failure_percentage__,
                    enforcing_failure_percentage_local_origin: enforcing_failure_percentage_local_origin__,
                    failure_percentage_minimum_hosts: failure_percentage_minimum_hosts__,
                    failure_percentage_request_volume: failure_percentage_request_volume__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.cluster.OutlierDetection", FIELDS, GeneratedVisitor)
    }
}
