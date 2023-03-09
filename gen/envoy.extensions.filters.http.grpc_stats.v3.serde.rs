// @generated
impl serde::Serialize for FilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.emit_filter_state {
            len += 1;
        }
        if self.enable_upstream_stats {
            len += 1;
        }
        if self.replace_dots_in_grpc_service_name {
            len += 1;
        }
        if self.per_method_stat_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_stats.v3.FilterConfig", len)?;
        if self.emit_filter_state {
            struct_ser.serialize_field("emitFilterState", &self.emit_filter_state)?;
        }
        if self.enable_upstream_stats {
            struct_ser.serialize_field("enableUpstreamStats", &self.enable_upstream_stats)?;
        }
        if self.replace_dots_in_grpc_service_name {
            struct_ser.serialize_field("replaceDotsInGrpcServiceName", &self.replace_dots_in_grpc_service_name)?;
        }
        if let Some(v) = self.per_method_stat_specifier.as_ref() {
            match v {
                filter_config::PerMethodStatSpecifier::IndividualMethodStatsAllowlist(v) => {
                    struct_ser.serialize_field("individualMethodStatsAllowlist", v)?;
                }
                filter_config::PerMethodStatSpecifier::StatsForAllMethods(v) => {
                    struct_ser.serialize_field("statsForAllMethods", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "emit_filter_state",
            "emitFilterState",
            "enable_upstream_stats",
            "enableUpstreamStats",
            "replace_dots_in_grpc_service_name",
            "replaceDotsInGrpcServiceName",
            "individual_method_stats_allowlist",
            "individualMethodStatsAllowlist",
            "stats_for_all_methods",
            "statsForAllMethods",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EmitFilterState,
            EnableUpstreamStats,
            ReplaceDotsInGrpcServiceName,
            IndividualMethodStatsAllowlist,
            StatsForAllMethods,
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
                            "emitFilterState" | "emit_filter_state" => Ok(GeneratedField::EmitFilterState),
                            "enableUpstreamStats" | "enable_upstream_stats" => Ok(GeneratedField::EnableUpstreamStats),
                            "replaceDotsInGrpcServiceName" | "replace_dots_in_grpc_service_name" => Ok(GeneratedField::ReplaceDotsInGrpcServiceName),
                            "individualMethodStatsAllowlist" | "individual_method_stats_allowlist" => Ok(GeneratedField::IndividualMethodStatsAllowlist),
                            "statsForAllMethods" | "stats_for_all_methods" => Ok(GeneratedField::StatsForAllMethods),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_stats.v3.FilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut emit_filter_state__ = None;
                let mut enable_upstream_stats__ = None;
                let mut replace_dots_in_grpc_service_name__ = None;
                let mut per_method_stat_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EmitFilterState => {
                            if emit_filter_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitFilterState"));
                            }
                            emit_filter_state__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnableUpstreamStats => {
                            if enable_upstream_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableUpstreamStats"));
                            }
                            enable_upstream_stats__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReplaceDotsInGrpcServiceName => {
                            if replace_dots_in_grpc_service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("replaceDotsInGrpcServiceName"));
                            }
                            replace_dots_in_grpc_service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::IndividualMethodStatsAllowlist => {
                            if per_method_stat_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("individualMethodStatsAllowlist"));
                            }
                            per_method_stat_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(filter_config::PerMethodStatSpecifier::IndividualMethodStatsAllowlist)
;
                        }
                        GeneratedField::StatsForAllMethods => {
                            if per_method_stat_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsForAllMethods"));
                            }
                            per_method_stat_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(filter_config::PerMethodStatSpecifier::StatsForAllMethods)
;
                        }
                    }
                }
                Ok(FilterConfig {
                    emit_filter_state: emit_filter_state__.unwrap_or_default(),
                    enable_upstream_stats: enable_upstream_stats__.unwrap_or_default(),
                    replace_dots_in_grpc_service_name: replace_dots_in_grpc_service_name__.unwrap_or_default(),
                    per_method_stat_specifier: per_method_stat_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_stats.v3.FilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterObject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_message_count != 0 {
            len += 1;
        }
        if self.response_message_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_stats.v3.FilterObject", len)?;
        if self.request_message_count != 0 {
            struct_ser.serialize_field("requestMessageCount", ToString::to_string(&self.request_message_count).as_str())?;
        }
        if self.response_message_count != 0 {
            struct_ser.serialize_field("responseMessageCount", ToString::to_string(&self.response_message_count).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterObject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_message_count",
            "requestMessageCount",
            "response_message_count",
            "responseMessageCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestMessageCount,
            ResponseMessageCount,
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
                            "requestMessageCount" | "request_message_count" => Ok(GeneratedField::RequestMessageCount),
                            "responseMessageCount" | "response_message_count" => Ok(GeneratedField::ResponseMessageCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_stats.v3.FilterObject")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterObject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_message_count__ = None;
                let mut response_message_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestMessageCount => {
                            if request_message_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMessageCount"));
                            }
                            request_message_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseMessageCount => {
                            if response_message_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseMessageCount"));
                            }
                            response_message_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FilterObject {
                    request_message_count: request_message_count__.unwrap_or_default(),
                    response_message_count: response_message_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_stats.v3.FilterObject", FIELDS, GeneratedVisitor)
    }
}
