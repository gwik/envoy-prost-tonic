// @generated
impl serde::Serialize for FaultAbort {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.percentage.is_some() {
            len += 1;
        }
        if self.error_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.fault.v3.FaultAbort", len)?;
        if let Some(v) = self.percentage.as_ref() {
            struct_ser.serialize_field("percentage", v)?;
        }
        if let Some(v) = self.error_type.as_ref() {
            match v {
                fault_abort::ErrorType::HttpStatus(v) => {
                    struct_ser.serialize_field("httpStatus", v)?;
                }
                fault_abort::ErrorType::GrpcStatus(v) => {
                    struct_ser.serialize_field("grpcStatus", v)?;
                }
                fault_abort::ErrorType::HeaderAbort(v) => {
                    struct_ser.serialize_field("headerAbort", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FaultAbort {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "percentage",
            "http_status",
            "httpStatus",
            "grpc_status",
            "grpcStatus",
            "header_abort",
            "headerAbort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Percentage,
            HttpStatus,
            GrpcStatus,
            HeaderAbort,
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
                            "percentage" => Ok(GeneratedField::Percentage),
                            "httpStatus" | "http_status" => Ok(GeneratedField::HttpStatus),
                            "grpcStatus" | "grpc_status" => Ok(GeneratedField::GrpcStatus),
                            "headerAbort" | "header_abort" => Ok(GeneratedField::HeaderAbort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FaultAbort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.fault.v3.FaultAbort")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FaultAbort, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut percentage__ = None;
                let mut error_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Percentage => {
                            if percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percentage"));
                            }
                            percentage__ = map.next_value()?;
                        }
                        GeneratedField::HttpStatus => {
                            if error_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpStatus"));
                            }
                            error_type__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fault_abort::ErrorType::HttpStatus(x.0));
                        }
                        GeneratedField::GrpcStatus => {
                            if error_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcStatus"));
                            }
                            error_type__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| fault_abort::ErrorType::GrpcStatus(x.0));
                        }
                        GeneratedField::HeaderAbort => {
                            if error_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerAbort"));
                            }
                            error_type__ = map.next_value::<::std::option::Option<_>>()?.map(fault_abort::ErrorType::HeaderAbort)
;
                        }
                    }
                }
                Ok(FaultAbort {
                    percentage: percentage__,
                    error_type: error_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.fault.v3.FaultAbort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for fault_abort::HeaderAbort {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.fault.v3.FaultAbort.HeaderAbort", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for fault_abort::HeaderAbort {
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
            type Value = fault_abort::HeaderAbort;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.fault.v3.FaultAbort.HeaderAbort")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<fault_abort::HeaderAbort, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(fault_abort::HeaderAbort {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.fault.v3.FaultAbort.HeaderAbort", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpFault {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delay.is_some() {
            len += 1;
        }
        if self.abort.is_some() {
            len += 1;
        }
        if !self.upstream_cluster.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if !self.downstream_nodes.is_empty() {
            len += 1;
        }
        if self.max_active_faults.is_some() {
            len += 1;
        }
        if self.response_rate_limit.is_some() {
            len += 1;
        }
        if !self.delay_percent_runtime.is_empty() {
            len += 1;
        }
        if !self.abort_percent_runtime.is_empty() {
            len += 1;
        }
        if !self.delay_duration_runtime.is_empty() {
            len += 1;
        }
        if !self.abort_http_status_runtime.is_empty() {
            len += 1;
        }
        if !self.max_active_faults_runtime.is_empty() {
            len += 1;
        }
        if !self.response_rate_limit_percent_runtime.is_empty() {
            len += 1;
        }
        if !self.abort_grpc_status_runtime.is_empty() {
            len += 1;
        }
        if self.disable_downstream_cluster_stats {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.fault.v3.HTTPFault", len)?;
        if let Some(v) = self.delay.as_ref() {
            struct_ser.serialize_field("delay", v)?;
        }
        if let Some(v) = self.abort.as_ref() {
            struct_ser.serialize_field("abort", v)?;
        }
        if !self.upstream_cluster.is_empty() {
            struct_ser.serialize_field("upstreamCluster", &self.upstream_cluster)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if !self.downstream_nodes.is_empty() {
            struct_ser.serialize_field("downstreamNodes", &self.downstream_nodes)?;
        }
        if let Some(v) = self.max_active_faults.as_ref() {
            struct_ser.serialize_field("maxActiveFaults", v)?;
        }
        if let Some(v) = self.response_rate_limit.as_ref() {
            struct_ser.serialize_field("responseRateLimit", v)?;
        }
        if !self.delay_percent_runtime.is_empty() {
            struct_ser.serialize_field("delayPercentRuntime", &self.delay_percent_runtime)?;
        }
        if !self.abort_percent_runtime.is_empty() {
            struct_ser.serialize_field("abortPercentRuntime", &self.abort_percent_runtime)?;
        }
        if !self.delay_duration_runtime.is_empty() {
            struct_ser.serialize_field("delayDurationRuntime", &self.delay_duration_runtime)?;
        }
        if !self.abort_http_status_runtime.is_empty() {
            struct_ser.serialize_field("abortHttpStatusRuntime", &self.abort_http_status_runtime)?;
        }
        if !self.max_active_faults_runtime.is_empty() {
            struct_ser.serialize_field("maxActiveFaultsRuntime", &self.max_active_faults_runtime)?;
        }
        if !self.response_rate_limit_percent_runtime.is_empty() {
            struct_ser.serialize_field("responseRateLimitPercentRuntime", &self.response_rate_limit_percent_runtime)?;
        }
        if !self.abort_grpc_status_runtime.is_empty() {
            struct_ser.serialize_field("abortGrpcStatusRuntime", &self.abort_grpc_status_runtime)?;
        }
        if self.disable_downstream_cluster_stats {
            struct_ser.serialize_field("disableDownstreamClusterStats", &self.disable_downstream_cluster_stats)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpFault {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delay",
            "abort",
            "upstream_cluster",
            "upstreamCluster",
            "headers",
            "downstream_nodes",
            "downstreamNodes",
            "max_active_faults",
            "maxActiveFaults",
            "response_rate_limit",
            "responseRateLimit",
            "delay_percent_runtime",
            "delayPercentRuntime",
            "abort_percent_runtime",
            "abortPercentRuntime",
            "delay_duration_runtime",
            "delayDurationRuntime",
            "abort_http_status_runtime",
            "abortHttpStatusRuntime",
            "max_active_faults_runtime",
            "maxActiveFaultsRuntime",
            "response_rate_limit_percent_runtime",
            "responseRateLimitPercentRuntime",
            "abort_grpc_status_runtime",
            "abortGrpcStatusRuntime",
            "disable_downstream_cluster_stats",
            "disableDownstreamClusterStats",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Delay,
            Abort,
            UpstreamCluster,
            Headers,
            DownstreamNodes,
            MaxActiveFaults,
            ResponseRateLimit,
            DelayPercentRuntime,
            AbortPercentRuntime,
            DelayDurationRuntime,
            AbortHttpStatusRuntime,
            MaxActiveFaultsRuntime,
            ResponseRateLimitPercentRuntime,
            AbortGrpcStatusRuntime,
            DisableDownstreamClusterStats,
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
                            "delay" => Ok(GeneratedField::Delay),
                            "abort" => Ok(GeneratedField::Abort),
                            "upstreamCluster" | "upstream_cluster" => Ok(GeneratedField::UpstreamCluster),
                            "headers" => Ok(GeneratedField::Headers),
                            "downstreamNodes" | "downstream_nodes" => Ok(GeneratedField::DownstreamNodes),
                            "maxActiveFaults" | "max_active_faults" => Ok(GeneratedField::MaxActiveFaults),
                            "responseRateLimit" | "response_rate_limit" => Ok(GeneratedField::ResponseRateLimit),
                            "delayPercentRuntime" | "delay_percent_runtime" => Ok(GeneratedField::DelayPercentRuntime),
                            "abortPercentRuntime" | "abort_percent_runtime" => Ok(GeneratedField::AbortPercentRuntime),
                            "delayDurationRuntime" | "delay_duration_runtime" => Ok(GeneratedField::DelayDurationRuntime),
                            "abortHttpStatusRuntime" | "abort_http_status_runtime" => Ok(GeneratedField::AbortHttpStatusRuntime),
                            "maxActiveFaultsRuntime" | "max_active_faults_runtime" => Ok(GeneratedField::MaxActiveFaultsRuntime),
                            "responseRateLimitPercentRuntime" | "response_rate_limit_percent_runtime" => Ok(GeneratedField::ResponseRateLimitPercentRuntime),
                            "abortGrpcStatusRuntime" | "abort_grpc_status_runtime" => Ok(GeneratedField::AbortGrpcStatusRuntime),
                            "disableDownstreamClusterStats" | "disable_downstream_cluster_stats" => Ok(GeneratedField::DisableDownstreamClusterStats),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpFault;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.fault.v3.HTTPFault")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpFault, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut delay__ = None;
                let mut abort__ = None;
                let mut upstream_cluster__ = None;
                let mut headers__ = None;
                let mut downstream_nodes__ = None;
                let mut max_active_faults__ = None;
                let mut response_rate_limit__ = None;
                let mut delay_percent_runtime__ = None;
                let mut abort_percent_runtime__ = None;
                let mut delay_duration_runtime__ = None;
                let mut abort_http_status_runtime__ = None;
                let mut max_active_faults_runtime__ = None;
                let mut response_rate_limit_percent_runtime__ = None;
                let mut abort_grpc_status_runtime__ = None;
                let mut disable_downstream_cluster_stats__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Delay => {
                            if delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            delay__ = map.next_value()?;
                        }
                        GeneratedField::Abort => {
                            if abort__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abort"));
                            }
                            abort__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamCluster => {
                            if upstream_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamCluster"));
                            }
                            upstream_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::DownstreamNodes => {
                            if downstream_nodes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamNodes"));
                            }
                            downstream_nodes__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxActiveFaults => {
                            if max_active_faults__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxActiveFaults"));
                            }
                            max_active_faults__ = map.next_value()?;
                        }
                        GeneratedField::ResponseRateLimit => {
                            if response_rate_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseRateLimit"));
                            }
                            response_rate_limit__ = map.next_value()?;
                        }
                        GeneratedField::DelayPercentRuntime => {
                            if delay_percent_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayPercentRuntime"));
                            }
                            delay_percent_runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::AbortPercentRuntime => {
                            if abort_percent_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abortPercentRuntime"));
                            }
                            abort_percent_runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::DelayDurationRuntime => {
                            if delay_duration_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayDurationRuntime"));
                            }
                            delay_duration_runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::AbortHttpStatusRuntime => {
                            if abort_http_status_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abortHttpStatusRuntime"));
                            }
                            abort_http_status_runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxActiveFaultsRuntime => {
                            if max_active_faults_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxActiveFaultsRuntime"));
                            }
                            max_active_faults_runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseRateLimitPercentRuntime => {
                            if response_rate_limit_percent_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseRateLimitPercentRuntime"));
                            }
                            response_rate_limit_percent_runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::AbortGrpcStatusRuntime => {
                            if abort_grpc_status_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abortGrpcStatusRuntime"));
                            }
                            abort_grpc_status_runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisableDownstreamClusterStats => {
                            if disable_downstream_cluster_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableDownstreamClusterStats"));
                            }
                            disable_downstream_cluster_stats__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HttpFault {
                    delay: delay__,
                    abort: abort__,
                    upstream_cluster: upstream_cluster__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    downstream_nodes: downstream_nodes__.unwrap_or_default(),
                    max_active_faults: max_active_faults__,
                    response_rate_limit: response_rate_limit__,
                    delay_percent_runtime: delay_percent_runtime__.unwrap_or_default(),
                    abort_percent_runtime: abort_percent_runtime__.unwrap_or_default(),
                    delay_duration_runtime: delay_duration_runtime__.unwrap_or_default(),
                    abort_http_status_runtime: abort_http_status_runtime__.unwrap_or_default(),
                    max_active_faults_runtime: max_active_faults_runtime__.unwrap_or_default(),
                    response_rate_limit_percent_runtime: response_rate_limit_percent_runtime__.unwrap_or_default(),
                    abort_grpc_status_runtime: abort_grpc_status_runtime__.unwrap_or_default(),
                    disable_downstream_cluster_stats: disable_downstream_cluster_stats__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.fault.v3.HTTPFault", FIELDS, GeneratedVisitor)
    }
}
