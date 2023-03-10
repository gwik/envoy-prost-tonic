// @generated
impl serde::Serialize for AccessLog {
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
        if self.filter.is_some() {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.AccessLog", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                access_log::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                access_log::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "filter",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Filter,
            Config,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "config" => Ok(GeneratedField::Config),
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
            type Value = AccessLog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.AccessLog")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AccessLog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut filter__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(access_log::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(access_log::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(AccessLog {
                    name: name__.unwrap_or_default(),
                    filter: filter__,
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.AccessLog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AccessLogFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.AccessLogFilter", len)?;
        if let Some(v) = self.filter_specifier.as_ref() {
            match v {
                access_log_filter::FilterSpecifier::StatusCodeFilter(v) => {
                    struct_ser.serialize_field("statusCodeFilter", v)?;
                }
                access_log_filter::FilterSpecifier::DurationFilter(v) => {
                    struct_ser.serialize_field("durationFilter", v)?;
                }
                access_log_filter::FilterSpecifier::NotHealthCheckFilter(v) => {
                    struct_ser.serialize_field("notHealthCheckFilter", v)?;
                }
                access_log_filter::FilterSpecifier::TraceableFilter(v) => {
                    struct_ser.serialize_field("traceableFilter", v)?;
                }
                access_log_filter::FilterSpecifier::RuntimeFilter(v) => {
                    struct_ser.serialize_field("runtimeFilter", v)?;
                }
                access_log_filter::FilterSpecifier::AndFilter(v) => {
                    struct_ser.serialize_field("andFilter", v)?;
                }
                access_log_filter::FilterSpecifier::OrFilter(v) => {
                    struct_ser.serialize_field("orFilter", v)?;
                }
                access_log_filter::FilterSpecifier::HeaderFilter(v) => {
                    struct_ser.serialize_field("headerFilter", v)?;
                }
                access_log_filter::FilterSpecifier::ResponseFlagFilter(v) => {
                    struct_ser.serialize_field("responseFlagFilter", v)?;
                }
                access_log_filter::FilterSpecifier::GrpcStatusFilter(v) => {
                    struct_ser.serialize_field("grpcStatusFilter", v)?;
                }
                access_log_filter::FilterSpecifier::ExtensionFilter(v) => {
                    struct_ser.serialize_field("extensionFilter", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessLogFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status_code_filter",
            "statusCodeFilter",
            "duration_filter",
            "durationFilter",
            "not_health_check_filter",
            "notHealthCheckFilter",
            "traceable_filter",
            "traceableFilter",
            "runtime_filter",
            "runtimeFilter",
            "and_filter",
            "andFilter",
            "or_filter",
            "orFilter",
            "header_filter",
            "headerFilter",
            "response_flag_filter",
            "responseFlagFilter",
            "grpc_status_filter",
            "grpcStatusFilter",
            "extension_filter",
            "extensionFilter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatusCodeFilter,
            DurationFilter,
            NotHealthCheckFilter,
            TraceableFilter,
            RuntimeFilter,
            AndFilter,
            OrFilter,
            HeaderFilter,
            ResponseFlagFilter,
            GrpcStatusFilter,
            ExtensionFilter,
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
                            "statusCodeFilter" | "status_code_filter" => Ok(GeneratedField::StatusCodeFilter),
                            "durationFilter" | "duration_filter" => Ok(GeneratedField::DurationFilter),
                            "notHealthCheckFilter" | "not_health_check_filter" => Ok(GeneratedField::NotHealthCheckFilter),
                            "traceableFilter" | "traceable_filter" => Ok(GeneratedField::TraceableFilter),
                            "runtimeFilter" | "runtime_filter" => Ok(GeneratedField::RuntimeFilter),
                            "andFilter" | "and_filter" => Ok(GeneratedField::AndFilter),
                            "orFilter" | "or_filter" => Ok(GeneratedField::OrFilter),
                            "headerFilter" | "header_filter" => Ok(GeneratedField::HeaderFilter),
                            "responseFlagFilter" | "response_flag_filter" => Ok(GeneratedField::ResponseFlagFilter),
                            "grpcStatusFilter" | "grpc_status_filter" => Ok(GeneratedField::GrpcStatusFilter),
                            "extensionFilter" | "extension_filter" => Ok(GeneratedField::ExtensionFilter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessLogFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.AccessLogFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AccessLogFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatusCodeFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusCodeFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::StatusCodeFilter)
;
                        }
                        GeneratedField::DurationFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::DurationFilter)
;
                        }
                        GeneratedField::NotHealthCheckFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notHealthCheckFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::NotHealthCheckFilter)
;
                        }
                        GeneratedField::TraceableFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceableFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::TraceableFilter)
;
                        }
                        GeneratedField::RuntimeFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::RuntimeFilter)
;
                        }
                        GeneratedField::AndFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::AndFilter)
;
                        }
                        GeneratedField::OrFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::OrFilter)
;
                        }
                        GeneratedField::HeaderFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::HeaderFilter)
;
                        }
                        GeneratedField::ResponseFlagFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseFlagFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::ResponseFlagFilter)
;
                        }
                        GeneratedField::GrpcStatusFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcStatusFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::GrpcStatusFilter)
;
                        }
                        GeneratedField::ExtensionFilter => {
                            if filter_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionFilter"));
                            }
                            filter_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(access_log_filter::FilterSpecifier::ExtensionFilter)
;
                        }
                    }
                }
                Ok(AccessLogFilter {
                    filter_specifier: filter_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.AccessLogFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AndFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.AndFilter", len)?;
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AndFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filters,
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
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AndFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.AndFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AndFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AndFilter {
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.AndFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ComparisonFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.op != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.ComparisonFilter", len)?;
        if self.op != 0 {
            let v = comparison_filter::Op::from_i32(self.op)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.op)))?;
            struct_ser.serialize_field("op", &v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ComparisonFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "op",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Op,
            Value,
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
                            "op" => Ok(GeneratedField::Op),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ComparisonFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.ComparisonFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ComparisonFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut op__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Op => {
                            if op__.is_some() {
                                return Err(serde::de::Error::duplicate_field("op"));
                            }
                            op__ = Some(map.next_value::<comparison_filter::Op>()? as i32);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(ComparisonFilter {
                    op: op__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.ComparisonFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for comparison_filter::Op {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Eq => "EQ",
            Self::Ge => "GE",
            Self::Le => "LE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for comparison_filter::Op {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EQ",
            "GE",
            "LE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = comparison_filter::Op;

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
                    .and_then(comparison_filter::Op::from_i32)
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
                    .and_then(comparison_filter::Op::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EQ" => Ok(comparison_filter::Op::Eq),
                    "GE" => Ok(comparison_filter::Op::Ge),
                    "LE" => Ok(comparison_filter::Op::Le),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DurationFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.comparison.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.DurationFilter", len)?;
        if let Some(v) = self.comparison.as_ref() {
            struct_ser.serialize_field("comparison", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DurationFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "comparison",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Comparison,
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
                            "comparison" => Ok(GeneratedField::Comparison),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DurationFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.DurationFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DurationFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut comparison__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Comparison => {
                            if comparison__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparison"));
                            }
                            comparison__ = map.next_value()?;
                        }
                    }
                }
                Ok(DurationFilter {
                    comparison: comparison__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.DurationFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtensionFilter {
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
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.ExtensionFilter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                extension_filter::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                extension_filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Config,
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
                            "config" => Ok(GeneratedField::Config),
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
            type Value = ExtensionFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.ExtensionFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtensionFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(extension_filter::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(extension_filter::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(ExtensionFilter {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.ExtensionFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrpcStatusFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.statuses.is_empty() {
            len += 1;
        }
        if self.exclude {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.GrpcStatusFilter", len)?;
        if !self.statuses.is_empty() {
            let v = self.statuses.iter().cloned().map(|v| {
                grpc_status_filter::Status::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("statuses", &v)?;
        }
        if self.exclude {
            struct_ser.serialize_field("exclude", &self.exclude)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrpcStatusFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "statuses",
            "exclude",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Statuses,
            Exclude,
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
                            "statuses" => Ok(GeneratedField::Statuses),
                            "exclude" => Ok(GeneratedField::Exclude),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrpcStatusFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.GrpcStatusFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GrpcStatusFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut statuses__ = None;
                let mut exclude__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Statuses => {
                            if statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statuses"));
                            }
                            statuses__ = Some(map.next_value::<Vec<grpc_status_filter::Status>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::Exclude => {
                            if exclude__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclude"));
                            }
                            exclude__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GrpcStatusFilter {
                    statuses: statuses__.unwrap_or_default(),
                    exclude: exclude__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.GrpcStatusFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_status_filter::Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Ok => "OK",
            Self::Canceled => "CANCELED",
            Self::Unknown => "UNKNOWN",
            Self::InvalidArgument => "INVALID_ARGUMENT",
            Self::DeadlineExceeded => "DEADLINE_EXCEEDED",
            Self::NotFound => "NOT_FOUND",
            Self::AlreadyExists => "ALREADY_EXISTS",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Self::FailedPrecondition => "FAILED_PRECONDITION",
            Self::Aborted => "ABORTED",
            Self::OutOfRange => "OUT_OF_RANGE",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::Internal => "INTERNAL",
            Self::Unavailable => "UNAVAILABLE",
            Self::DataLoss => "DATA_LOSS",
            Self::Unauthenticated => "UNAUTHENTICATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for grpc_status_filter::Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OK",
            "CANCELED",
            "UNKNOWN",
            "INVALID_ARGUMENT",
            "DEADLINE_EXCEEDED",
            "NOT_FOUND",
            "ALREADY_EXISTS",
            "PERMISSION_DENIED",
            "RESOURCE_EXHAUSTED",
            "FAILED_PRECONDITION",
            "ABORTED",
            "OUT_OF_RANGE",
            "UNIMPLEMENTED",
            "INTERNAL",
            "UNAVAILABLE",
            "DATA_LOSS",
            "UNAUTHENTICATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_status_filter::Status;

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
                    .and_then(grpc_status_filter::Status::from_i32)
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
                    .and_then(grpc_status_filter::Status::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OK" => Ok(grpc_status_filter::Status::Ok),
                    "CANCELED" => Ok(grpc_status_filter::Status::Canceled),
                    "UNKNOWN" => Ok(grpc_status_filter::Status::Unknown),
                    "INVALID_ARGUMENT" => Ok(grpc_status_filter::Status::InvalidArgument),
                    "DEADLINE_EXCEEDED" => Ok(grpc_status_filter::Status::DeadlineExceeded),
                    "NOT_FOUND" => Ok(grpc_status_filter::Status::NotFound),
                    "ALREADY_EXISTS" => Ok(grpc_status_filter::Status::AlreadyExists),
                    "PERMISSION_DENIED" => Ok(grpc_status_filter::Status::PermissionDenied),
                    "RESOURCE_EXHAUSTED" => Ok(grpc_status_filter::Status::ResourceExhausted),
                    "FAILED_PRECONDITION" => Ok(grpc_status_filter::Status::FailedPrecondition),
                    "ABORTED" => Ok(grpc_status_filter::Status::Aborted),
                    "OUT_OF_RANGE" => Ok(grpc_status_filter::Status::OutOfRange),
                    "UNIMPLEMENTED" => Ok(grpc_status_filter::Status::Unimplemented),
                    "INTERNAL" => Ok(grpc_status_filter::Status::Internal),
                    "UNAVAILABLE" => Ok(grpc_status_filter::Status::Unavailable),
                    "DATA_LOSS" => Ok(grpc_status_filter::Status::DataLoss),
                    "UNAUTHENTICATED" => Ok(grpc_status_filter::Status::Unauthenticated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.HeaderFilter", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
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
                            "header" => Ok(GeneratedField::Header),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.HeaderFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                    }
                }
                Ok(HeaderFilter {
                    header: header__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.HeaderFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NotHealthCheckFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.NotHealthCheckFilter", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NotHealthCheckFilter {
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
            type Value = NotHealthCheckFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.NotHealthCheckFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NotHealthCheckFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(NotHealthCheckFilter {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.NotHealthCheckFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OrFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.OrFilter", len)?;
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OrFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filters,
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
                            "filters" => Ok(GeneratedField::Filters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OrFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.OrFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OrFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OrFilter {
                    filters: filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.OrFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseFlagFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.flags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.ResponseFlagFilter", len)?;
        if !self.flags.is_empty() {
            struct_ser.serialize_field("flags", &self.flags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseFlagFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "flags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Flags,
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
                            "flags" => Ok(GeneratedField::Flags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseFlagFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.ResponseFlagFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResponseFlagFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut flags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Flags => {
                            if flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flags"));
                            }
                            flags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ResponseFlagFilter {
                    flags: flags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.ResponseFlagFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimeFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.runtime_key.is_empty() {
            len += 1;
        }
        if self.percent_sampled.is_some() {
            len += 1;
        }
        if self.use_independent_randomness {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.RuntimeFilter", len)?;
        if !self.runtime_key.is_empty() {
            struct_ser.serialize_field("runtimeKey", &self.runtime_key)?;
        }
        if let Some(v) = self.percent_sampled.as_ref() {
            struct_ser.serialize_field("percentSampled", v)?;
        }
        if self.use_independent_randomness {
            struct_ser.serialize_field("useIndependentRandomness", &self.use_independent_randomness)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimeFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "runtime_key",
            "runtimeKey",
            "percent_sampled",
            "percentSampled",
            "use_independent_randomness",
            "useIndependentRandomness",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RuntimeKey,
            PercentSampled,
            UseIndependentRandomness,
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
                            "runtimeKey" | "runtime_key" => Ok(GeneratedField::RuntimeKey),
                            "percentSampled" | "percent_sampled" => Ok(GeneratedField::PercentSampled),
                            "useIndependentRandomness" | "use_independent_randomness" => Ok(GeneratedField::UseIndependentRandomness),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimeFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.RuntimeFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuntimeFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut runtime_key__ = None;
                let mut percent_sampled__ = None;
                let mut use_independent_randomness__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RuntimeKey => {
                            if runtime_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKey"));
                            }
                            runtime_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::PercentSampled => {
                            if percent_sampled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percentSampled"));
                            }
                            percent_sampled__ = map.next_value()?;
                        }
                        GeneratedField::UseIndependentRandomness => {
                            if use_independent_randomness__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useIndependentRandomness"));
                            }
                            use_independent_randomness__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RuntimeFilter {
                    runtime_key: runtime_key__.unwrap_or_default(),
                    percent_sampled: percent_sampled__,
                    use_independent_randomness: use_independent_randomness__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.RuntimeFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatusCodeFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.comparison.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.StatusCodeFilter", len)?;
        if let Some(v) = self.comparison.as_ref() {
            struct_ser.serialize_field("comparison", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatusCodeFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "comparison",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Comparison,
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
                            "comparison" => Ok(GeneratedField::Comparison),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusCodeFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.StatusCodeFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatusCodeFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut comparison__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Comparison => {
                            if comparison__.is_some() {
                                return Err(serde::de::Error::duplicate_field("comparison"));
                            }
                            comparison__ = map.next_value()?;
                        }
                    }
                }
                Ok(StatusCodeFilter {
                    comparison: comparison__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.StatusCodeFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TraceableFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.filter.accesslog.v2.TraceableFilter", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TraceableFilter {
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
            type Value = TraceableFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.accesslog.v2.TraceableFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TraceableFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(TraceableFilter {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.accesslog.v2.TraceableFilter", FIELDS, GeneratedVisitor)
    }
}
