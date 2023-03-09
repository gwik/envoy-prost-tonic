// @generated
impl serde::Serialize for ClusterStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if !self.cluster_service_name.is_empty() {
            len += 1;
        }
        if !self.upstream_locality_stats.is_empty() {
            len += 1;
        }
        if self.total_dropped_requests != 0 {
            len += 1;
        }
        if !self.dropped_requests.is_empty() {
            len += 1;
        }
        if self.load_report_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.ClusterStats", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("clusterName", &self.cluster_name)?;
        }
        if !self.cluster_service_name.is_empty() {
            struct_ser.serialize_field("clusterServiceName", &self.cluster_service_name)?;
        }
        if !self.upstream_locality_stats.is_empty() {
            struct_ser.serialize_field("upstreamLocalityStats", &self.upstream_locality_stats)?;
        }
        if self.total_dropped_requests != 0 {
            struct_ser.serialize_field("totalDroppedRequests", ToString::to_string(&self.total_dropped_requests).as_str())?;
        }
        if !self.dropped_requests.is_empty() {
            struct_ser.serialize_field("droppedRequests", &self.dropped_requests)?;
        }
        if let Some(v) = self.load_report_interval.as_ref() {
            struct_ser.serialize_field("loadReportInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "cluster_service_name",
            "clusterServiceName",
            "upstream_locality_stats",
            "upstreamLocalityStats",
            "total_dropped_requests",
            "totalDroppedRequests",
            "dropped_requests",
            "droppedRequests",
            "load_report_interval",
            "loadReportInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            ClusterServiceName,
            UpstreamLocalityStats,
            TotalDroppedRequests,
            DroppedRequests,
            LoadReportInterval,
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
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "clusterServiceName" | "cluster_service_name" => Ok(GeneratedField::ClusterServiceName),
                            "upstreamLocalityStats" | "upstream_locality_stats" => Ok(GeneratedField::UpstreamLocalityStats),
                            "totalDroppedRequests" | "total_dropped_requests" => Ok(GeneratedField::TotalDroppedRequests),
                            "droppedRequests" | "dropped_requests" => Ok(GeneratedField::DroppedRequests),
                            "loadReportInterval" | "load_report_interval" => Ok(GeneratedField::LoadReportInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.ClusterStats")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut cluster_service_name__ = None;
                let mut upstream_locality_stats__ = None;
                let mut total_dropped_requests__ = None;
                let mut dropped_requests__ = None;
                let mut load_report_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClusterServiceName => {
                            if cluster_service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterServiceName"));
                            }
                            cluster_service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamLocalityStats => {
                            if upstream_locality_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamLocalityStats"));
                            }
                            upstream_locality_stats__ = Some(map.next_value()?);
                        }
                        GeneratedField::TotalDroppedRequests => {
                            if total_dropped_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalDroppedRequests"));
                            }
                            total_dropped_requests__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DroppedRequests => {
                            if dropped_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedRequests"));
                            }
                            dropped_requests__ = Some(map.next_value()?);
                        }
                        GeneratedField::LoadReportInterval => {
                            if load_report_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadReportInterval"));
                            }
                            load_report_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(ClusterStats {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    cluster_service_name: cluster_service_name__.unwrap_or_default(),
                    upstream_locality_stats: upstream_locality_stats__.unwrap_or_default(),
                    total_dropped_requests: total_dropped_requests__.unwrap_or_default(),
                    dropped_requests: dropped_requests__.unwrap_or_default(),
                    load_report_interval: load_report_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.ClusterStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_stats::DroppedRequests {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.category.is_empty() {
            len += 1;
        }
        if self.dropped_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.ClusterStats.DroppedRequests", len)?;
        if !self.category.is_empty() {
            struct_ser.serialize_field("category", &self.category)?;
        }
        if self.dropped_count != 0 {
            struct_ser.serialize_field("droppedCount", ToString::to_string(&self.dropped_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_stats::DroppedRequests {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "category",
            "dropped_count",
            "droppedCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Category,
            DroppedCount,
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
                            "category" => Ok(GeneratedField::Category),
                            "droppedCount" | "dropped_count" => Ok(GeneratedField::DroppedCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_stats::DroppedRequests;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.ClusterStats.DroppedRequests")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster_stats::DroppedRequests, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut category__ = None;
                let mut dropped_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map.next_value()?);
                        }
                        GeneratedField::DroppedCount => {
                            if dropped_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("droppedCount"));
                            }
                            dropped_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(cluster_stats::DroppedRequests {
                    category: category__.unwrap_or_default(),
                    dropped_count: dropped_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.ClusterStats.DroppedRequests", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Endpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if self.health_check_config.is_some() {
            len += 1;
        }
        if !self.hostname.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.Endpoint", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.health_check_config.as_ref() {
            struct_ser.serialize_field("healthCheckConfig", v)?;
        }
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Endpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "health_check_config",
            "healthCheckConfig",
            "hostname",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            HealthCheckConfig,
            Hostname,
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
                            "address" => Ok(GeneratedField::Address),
                            "healthCheckConfig" | "health_check_config" => Ok(GeneratedField::HealthCheckConfig),
                            "hostname" => Ok(GeneratedField::Hostname),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Endpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.Endpoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Endpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut health_check_config__ = None;
                let mut hostname__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::HealthCheckConfig => {
                            if health_check_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckConfig"));
                            }
                            health_check_config__ = map.next_value()?;
                        }
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Endpoint {
                    address: address__,
                    health_check_config: health_check_config__,
                    hostname: hostname__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.Endpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for endpoint::HealthCheckConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.port_value != 0 {
            len += 1;
        }
        if !self.hostname.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.Endpoint.HealthCheckConfig", len)?;
        if self.port_value != 0 {
            struct_ser.serialize_field("portValue", &self.port_value)?;
        }
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for endpoint::HealthCheckConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "port_value",
            "portValue",
            "hostname",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PortValue,
            Hostname,
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
                            "portValue" | "port_value" => Ok(GeneratedField::PortValue),
                            "hostname" => Ok(GeneratedField::Hostname),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = endpoint::HealthCheckConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.Endpoint.HealthCheckConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<endpoint::HealthCheckConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut port_value__ = None;
                let mut hostname__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PortValue => {
                            if port_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portValue"));
                            }
                            port_value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(endpoint::HealthCheckConfig {
                    port_value: port_value__.unwrap_or_default(),
                    hostname: hostname__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.Endpoint.HealthCheckConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointLoadMetricStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metric_name.is_empty() {
            len += 1;
        }
        if self.num_requests_finished_with_metric != 0 {
            len += 1;
        }
        if self.total_metric_value != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.EndpointLoadMetricStats", len)?;
        if !self.metric_name.is_empty() {
            struct_ser.serialize_field("metricName", &self.metric_name)?;
        }
        if self.num_requests_finished_with_metric != 0 {
            struct_ser.serialize_field("numRequestsFinishedWithMetric", ToString::to_string(&self.num_requests_finished_with_metric).as_str())?;
        }
        if self.total_metric_value != 0. {
            struct_ser.serialize_field("totalMetricValue", &self.total_metric_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointLoadMetricStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metric_name",
            "metricName",
            "num_requests_finished_with_metric",
            "numRequestsFinishedWithMetric",
            "total_metric_value",
            "totalMetricValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetricName,
            NumRequestsFinishedWithMetric,
            TotalMetricValue,
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
                            "metricName" | "metric_name" => Ok(GeneratedField::MetricName),
                            "numRequestsFinishedWithMetric" | "num_requests_finished_with_metric" => Ok(GeneratedField::NumRequestsFinishedWithMetric),
                            "totalMetricValue" | "total_metric_value" => Ok(GeneratedField::TotalMetricValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointLoadMetricStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.EndpointLoadMetricStats")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EndpointLoadMetricStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metric_name__ = None;
                let mut num_requests_finished_with_metric__ = None;
                let mut total_metric_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MetricName => {
                            if metric_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricName"));
                            }
                            metric_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::NumRequestsFinishedWithMetric => {
                            if num_requests_finished_with_metric__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRequestsFinishedWithMetric"));
                            }
                            num_requests_finished_with_metric__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalMetricValue => {
                            if total_metric_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalMetricValue"));
                            }
                            total_metric_value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EndpointLoadMetricStats {
                    metric_name: metric_name__.unwrap_or_default(),
                    num_requests_finished_with_metric: num_requests_finished_with_metric__.unwrap_or_default(),
                    total_metric_value: total_metric_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.EndpointLoadMetricStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LbEndpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.health_status != 0 {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.load_balancing_weight.is_some() {
            len += 1;
        }
        if self.host_identifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.LbEndpoint", len)?;
        if self.health_status != 0 {
            let v = super::core::HealthStatus::from_i32(self.health_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.health_status)))?;
            struct_ser.serialize_field("healthStatus", &v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.load_balancing_weight.as_ref() {
            struct_ser.serialize_field("loadBalancingWeight", v)?;
        }
        if let Some(v) = self.host_identifier.as_ref() {
            match v {
                lb_endpoint::HostIdentifier::Endpoint(v) => {
                    struct_ser.serialize_field("endpoint", v)?;
                }
                lb_endpoint::HostIdentifier::EndpointName(v) => {
                    struct_ser.serialize_field("endpointName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LbEndpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_status",
            "healthStatus",
            "metadata",
            "load_balancing_weight",
            "loadBalancingWeight",
            "endpoint",
            "endpoint_name",
            "endpointName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthStatus,
            Metadata,
            LoadBalancingWeight,
            Endpoint,
            EndpointName,
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
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "loadBalancingWeight" | "load_balancing_weight" => Ok(GeneratedField::LoadBalancingWeight),
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "endpointName" | "endpoint_name" => Ok(GeneratedField::EndpointName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LbEndpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.LbEndpoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LbEndpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut health_status__ = None;
                let mut metadata__ = None;
                let mut load_balancing_weight__ = None;
                let mut host_identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = Some(map.next_value::<super::core::HealthStatus>()? as i32);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::LoadBalancingWeight => {
                            if load_balancing_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancingWeight"));
                            }
                            load_balancing_weight__ = map.next_value()?;
                        }
                        GeneratedField::Endpoint => {
                            if host_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            host_identifier__ = map.next_value::<::std::option::Option<_>>()?.map(lb_endpoint::HostIdentifier::Endpoint)
;
                        }
                        GeneratedField::EndpointName => {
                            if host_identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointName"));
                            }
                            host_identifier__ = map.next_value::<::std::option::Option<_>>()?.map(lb_endpoint::HostIdentifier::EndpointName);
                        }
                    }
                }
                Ok(LbEndpoint {
                    health_status: health_status__.unwrap_or_default(),
                    metadata: metadata__,
                    load_balancing_weight: load_balancing_weight__,
                    host_identifier: host_identifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.LbEndpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalityLbEndpoints {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.locality.is_some() {
            len += 1;
        }
        if !self.lb_endpoints.is_empty() {
            len += 1;
        }
        if self.load_balancing_weight.is_some() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.proximity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.LocalityLbEndpoints", len)?;
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if !self.lb_endpoints.is_empty() {
            struct_ser.serialize_field("lbEndpoints", &self.lb_endpoints)?;
        }
        if let Some(v) = self.load_balancing_weight.as_ref() {
            struct_ser.serialize_field("loadBalancingWeight", v)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if let Some(v) = self.proximity.as_ref() {
            struct_ser.serialize_field("proximity", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalityLbEndpoints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locality",
            "lb_endpoints",
            "lbEndpoints",
            "load_balancing_weight",
            "loadBalancingWeight",
            "priority",
            "proximity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locality,
            LbEndpoints,
            LoadBalancingWeight,
            Priority,
            Proximity,
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
                            "locality" => Ok(GeneratedField::Locality),
                            "lbEndpoints" | "lb_endpoints" => Ok(GeneratedField::LbEndpoints),
                            "loadBalancingWeight" | "load_balancing_weight" => Ok(GeneratedField::LoadBalancingWeight),
                            "priority" => Ok(GeneratedField::Priority),
                            "proximity" => Ok(GeneratedField::Proximity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalityLbEndpoints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.LocalityLbEndpoints")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LocalityLbEndpoints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality__ = None;
                let mut lb_endpoints__ = None;
                let mut load_balancing_weight__ = None;
                let mut priority__ = None;
                let mut proximity__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map.next_value()?;
                        }
                        GeneratedField::LbEndpoints => {
                            if lb_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lbEndpoints"));
                            }
                            lb_endpoints__ = Some(map.next_value()?);
                        }
                        GeneratedField::LoadBalancingWeight => {
                            if load_balancing_weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadBalancingWeight"));
                            }
                            load_balancing_weight__ = map.next_value()?;
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Proximity => {
                            if proximity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proximity"));
                            }
                            proximity__ = map.next_value()?;
                        }
                    }
                }
                Ok(LocalityLbEndpoints {
                    locality: locality__,
                    lb_endpoints: lb_endpoints__.unwrap_or_default(),
                    load_balancing_weight: load_balancing_weight__,
                    priority: priority__.unwrap_or_default(),
                    proximity: proximity__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.LocalityLbEndpoints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamEndpointStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.total_successful_requests != 0 {
            len += 1;
        }
        if self.total_requests_in_progress != 0 {
            len += 1;
        }
        if self.total_error_requests != 0 {
            len += 1;
        }
        if self.total_issued_requests != 0 {
            len += 1;
        }
        if !self.load_metric_stats.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.UpstreamEndpointStats", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if self.total_successful_requests != 0 {
            struct_ser.serialize_field("totalSuccessfulRequests", ToString::to_string(&self.total_successful_requests).as_str())?;
        }
        if self.total_requests_in_progress != 0 {
            struct_ser.serialize_field("totalRequestsInProgress", ToString::to_string(&self.total_requests_in_progress).as_str())?;
        }
        if self.total_error_requests != 0 {
            struct_ser.serialize_field("totalErrorRequests", ToString::to_string(&self.total_error_requests).as_str())?;
        }
        if self.total_issued_requests != 0 {
            struct_ser.serialize_field("totalIssuedRequests", ToString::to_string(&self.total_issued_requests).as_str())?;
        }
        if !self.load_metric_stats.is_empty() {
            struct_ser.serialize_field("loadMetricStats", &self.load_metric_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamEndpointStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "metadata",
            "total_successful_requests",
            "totalSuccessfulRequests",
            "total_requests_in_progress",
            "totalRequestsInProgress",
            "total_error_requests",
            "totalErrorRequests",
            "total_issued_requests",
            "totalIssuedRequests",
            "load_metric_stats",
            "loadMetricStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Metadata,
            TotalSuccessfulRequests,
            TotalRequestsInProgress,
            TotalErrorRequests,
            TotalIssuedRequests,
            LoadMetricStats,
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
                            "address" => Ok(GeneratedField::Address),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "totalSuccessfulRequests" | "total_successful_requests" => Ok(GeneratedField::TotalSuccessfulRequests),
                            "totalRequestsInProgress" | "total_requests_in_progress" => Ok(GeneratedField::TotalRequestsInProgress),
                            "totalErrorRequests" | "total_error_requests" => Ok(GeneratedField::TotalErrorRequests),
                            "totalIssuedRequests" | "total_issued_requests" => Ok(GeneratedField::TotalIssuedRequests),
                            "loadMetricStats" | "load_metric_stats" => Ok(GeneratedField::LoadMetricStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamEndpointStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.UpstreamEndpointStats")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamEndpointStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut metadata__ = None;
                let mut total_successful_requests__ = None;
                let mut total_requests_in_progress__ = None;
                let mut total_error_requests__ = None;
                let mut total_issued_requests__ = None;
                let mut load_metric_stats__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::TotalSuccessfulRequests => {
                            if total_successful_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSuccessfulRequests"));
                            }
                            total_successful_requests__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalRequestsInProgress => {
                            if total_requests_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRequestsInProgress"));
                            }
                            total_requests_in_progress__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalErrorRequests => {
                            if total_error_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalErrorRequests"));
                            }
                            total_error_requests__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalIssuedRequests => {
                            if total_issued_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalIssuedRequests"));
                            }
                            total_issued_requests__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LoadMetricStats => {
                            if load_metric_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadMetricStats"));
                            }
                            load_metric_stats__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpstreamEndpointStats {
                    address: address__,
                    metadata: metadata__,
                    total_successful_requests: total_successful_requests__.unwrap_or_default(),
                    total_requests_in_progress: total_requests_in_progress__.unwrap_or_default(),
                    total_error_requests: total_error_requests__.unwrap_or_default(),
                    total_issued_requests: total_issued_requests__.unwrap_or_default(),
                    load_metric_stats: load_metric_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.UpstreamEndpointStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamLocalityStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.locality.is_some() {
            len += 1;
        }
        if self.total_successful_requests != 0 {
            len += 1;
        }
        if self.total_requests_in_progress != 0 {
            len += 1;
        }
        if self.total_error_requests != 0 {
            len += 1;
        }
        if self.total_issued_requests != 0 {
            len += 1;
        }
        if !self.load_metric_stats.is_empty() {
            len += 1;
        }
        if !self.upstream_endpoint_stats.is_empty() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.endpoint.UpstreamLocalityStats", len)?;
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if self.total_successful_requests != 0 {
            struct_ser.serialize_field("totalSuccessfulRequests", ToString::to_string(&self.total_successful_requests).as_str())?;
        }
        if self.total_requests_in_progress != 0 {
            struct_ser.serialize_field("totalRequestsInProgress", ToString::to_string(&self.total_requests_in_progress).as_str())?;
        }
        if self.total_error_requests != 0 {
            struct_ser.serialize_field("totalErrorRequests", ToString::to_string(&self.total_error_requests).as_str())?;
        }
        if self.total_issued_requests != 0 {
            struct_ser.serialize_field("totalIssuedRequests", ToString::to_string(&self.total_issued_requests).as_str())?;
        }
        if !self.load_metric_stats.is_empty() {
            struct_ser.serialize_field("loadMetricStats", &self.load_metric_stats)?;
        }
        if !self.upstream_endpoint_stats.is_empty() {
            struct_ser.serialize_field("upstreamEndpointStats", &self.upstream_endpoint_stats)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamLocalityStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locality",
            "total_successful_requests",
            "totalSuccessfulRequests",
            "total_requests_in_progress",
            "totalRequestsInProgress",
            "total_error_requests",
            "totalErrorRequests",
            "total_issued_requests",
            "totalIssuedRequests",
            "load_metric_stats",
            "loadMetricStats",
            "upstream_endpoint_stats",
            "upstreamEndpointStats",
            "priority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locality,
            TotalSuccessfulRequests,
            TotalRequestsInProgress,
            TotalErrorRequests,
            TotalIssuedRequests,
            LoadMetricStats,
            UpstreamEndpointStats,
            Priority,
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
                            "locality" => Ok(GeneratedField::Locality),
                            "totalSuccessfulRequests" | "total_successful_requests" => Ok(GeneratedField::TotalSuccessfulRequests),
                            "totalRequestsInProgress" | "total_requests_in_progress" => Ok(GeneratedField::TotalRequestsInProgress),
                            "totalErrorRequests" | "total_error_requests" => Ok(GeneratedField::TotalErrorRequests),
                            "totalIssuedRequests" | "total_issued_requests" => Ok(GeneratedField::TotalIssuedRequests),
                            "loadMetricStats" | "load_metric_stats" => Ok(GeneratedField::LoadMetricStats),
                            "upstreamEndpointStats" | "upstream_endpoint_stats" => Ok(GeneratedField::UpstreamEndpointStats),
                            "priority" => Ok(GeneratedField::Priority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamLocalityStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.endpoint.UpstreamLocalityStats")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamLocalityStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality__ = None;
                let mut total_successful_requests__ = None;
                let mut total_requests_in_progress__ = None;
                let mut total_error_requests__ = None;
                let mut total_issued_requests__ = None;
                let mut load_metric_stats__ = None;
                let mut upstream_endpoint_stats__ = None;
                let mut priority__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map.next_value()?;
                        }
                        GeneratedField::TotalSuccessfulRequests => {
                            if total_successful_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalSuccessfulRequests"));
                            }
                            total_successful_requests__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalRequestsInProgress => {
                            if total_requests_in_progress__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalRequestsInProgress"));
                            }
                            total_requests_in_progress__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalErrorRequests => {
                            if total_error_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalErrorRequests"));
                            }
                            total_error_requests__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalIssuedRequests => {
                            if total_issued_requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalIssuedRequests"));
                            }
                            total_issued_requests__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LoadMetricStats => {
                            if load_metric_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadMetricStats"));
                            }
                            load_metric_stats__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamEndpointStats => {
                            if upstream_endpoint_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamEndpointStats"));
                            }
                            upstream_endpoint_stats__ = Some(map.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(UpstreamLocalityStats {
                    locality: locality__,
                    total_successful_requests: total_successful_requests__.unwrap_or_default(),
                    total_requests_in_progress: total_requests_in_progress__.unwrap_or_default(),
                    total_error_requests: total_error_requests__.unwrap_or_default(),
                    total_issued_requests: total_issued_requests__.unwrap_or_default(),
                    load_metric_stats: load_metric_stats__.unwrap_or_default(),
                    upstream_endpoint_stats: upstream_endpoint_stats__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.endpoint.UpstreamLocalityStats", FIELDS, GeneratedVisitor)
    }
}
