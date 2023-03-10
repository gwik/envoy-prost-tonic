// @generated
impl serde::Serialize for DogStatsdSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prefix.is_empty() {
            len += 1;
        }
        if self.dog_statsd_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.DogStatsdSink", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        if let Some(v) = self.dog_statsd_specifier.as_ref() {
            match v {
                dog_statsd_sink::DogStatsdSpecifier::Address(v) => {
                    struct_ser.serialize_field("address", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DogStatsdSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prefix",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            Address,
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
                            "prefix" => Ok(GeneratedField::Prefix),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DogStatsdSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.DogStatsdSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DogStatsdSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                let mut dog_statsd_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if dog_statsd_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            dog_statsd_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(dog_statsd_sink::DogStatsdSpecifier::Address)
;
                        }
                    }
                }
                Ok(DogStatsdSink {
                    prefix: prefix__.unwrap_or_default(),
                    dog_statsd_specifier: dog_statsd_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.DogStatsdSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HystrixSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_buckets != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.HystrixSink", len)?;
        if self.num_buckets != 0 {
            struct_ser.serialize_field("numBuckets", ToString::to_string(&self.num_buckets).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HystrixSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_buckets",
            "numBuckets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumBuckets,
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
                            "numBuckets" | "num_buckets" => Ok(GeneratedField::NumBuckets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HystrixSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.HystrixSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HystrixSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_buckets__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NumBuckets => {
                            if num_buckets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numBuckets"));
                            }
                            num_buckets__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(HystrixSink {
                    num_buckets: num_buckets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.HystrixSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MetricsServiceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.MetricsServiceConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MetricsServiceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MetricsServiceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.MetricsServiceConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MetricsServiceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                    }
                }
                Ok(MetricsServiceConfig {
                    grpc_service: grpc_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.MetricsServiceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stats_tags.is_empty() {
            len += 1;
        }
        if self.use_all_default_tags.is_some() {
            len += 1;
        }
        if self.stats_matcher.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.StatsConfig", len)?;
        if !self.stats_tags.is_empty() {
            struct_ser.serialize_field("statsTags", &self.stats_tags)?;
        }
        if let Some(v) = self.use_all_default_tags.as_ref() {
            struct_ser.serialize_field("useAllDefaultTags", v)?;
        }
        if let Some(v) = self.stats_matcher.as_ref() {
            struct_ser.serialize_field("statsMatcher", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stats_tags",
            "statsTags",
            "use_all_default_tags",
            "useAllDefaultTags",
            "stats_matcher",
            "statsMatcher",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatsTags,
            UseAllDefaultTags,
            StatsMatcher,
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
                            "statsTags" | "stats_tags" => Ok(GeneratedField::StatsTags),
                            "useAllDefaultTags" | "use_all_default_tags" => Ok(GeneratedField::UseAllDefaultTags),
                            "statsMatcher" | "stats_matcher" => Ok(GeneratedField::StatsMatcher),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.StatsConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stats_tags__ = None;
                let mut use_all_default_tags__ = None;
                let mut stats_matcher__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatsTags => {
                            if stats_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsTags"));
                            }
                            stats_tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseAllDefaultTags => {
                            if use_all_default_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useAllDefaultTags"));
                            }
                            use_all_default_tags__ = map.next_value()?;
                        }
                        GeneratedField::StatsMatcher => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsMatcher"));
                            }
                            stats_matcher__ = map.next_value()?;
                        }
                    }
                }
                Ok(StatsConfig {
                    stats_tags: stats_tags__.unwrap_or_default(),
                    use_all_default_tags: use_all_default_tags__,
                    stats_matcher: stats_matcher__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.StatsConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stats_matcher.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.StatsMatcher", len)?;
        if let Some(v) = self.stats_matcher.as_ref() {
            match v {
                stats_matcher::StatsMatcher::RejectAll(v) => {
                    struct_ser.serialize_field("rejectAll", v)?;
                }
                stats_matcher::StatsMatcher::ExclusionList(v) => {
                    struct_ser.serialize_field("exclusionList", v)?;
                }
                stats_matcher::StatsMatcher::InclusionList(v) => {
                    struct_ser.serialize_field("inclusionList", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reject_all",
            "rejectAll",
            "exclusion_list",
            "exclusionList",
            "inclusion_list",
            "inclusionList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RejectAll,
            ExclusionList,
            InclusionList,
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
                            "rejectAll" | "reject_all" => Ok(GeneratedField::RejectAll),
                            "exclusionList" | "exclusion_list" => Ok(GeneratedField::ExclusionList),
                            "inclusionList" | "inclusion_list" => Ok(GeneratedField::InclusionList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatsMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.StatsMatcher")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatsMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stats_matcher__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RejectAll => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectAll"));
                            }
                            stats_matcher__ = map.next_value::<::std::option::Option<_>>()?.map(stats_matcher::StatsMatcher::RejectAll);
                        }
                        GeneratedField::ExclusionList => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exclusionList"));
                            }
                            stats_matcher__ = map.next_value::<::std::option::Option<_>>()?.map(stats_matcher::StatsMatcher::ExclusionList)
;
                        }
                        GeneratedField::InclusionList => {
                            if stats_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inclusionList"));
                            }
                            stats_matcher__ = map.next_value::<::std::option::Option<_>>()?.map(stats_matcher::StatsMatcher::InclusionList)
;
                        }
                    }
                }
                Ok(StatsMatcher {
                    stats_matcher: stats_matcher__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.StatsMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsSink {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.StatsSink", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                stats_sink::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                stats_sink::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsSink {
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
            type Value = StatsSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.StatsSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatsSink, V::Error>
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
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(stats_sink::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(stats_sink::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(StatsSink {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.StatsSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatsdSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prefix.is_empty() {
            len += 1;
        }
        if self.statsd_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.StatsdSink", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        if let Some(v) = self.statsd_specifier.as_ref() {
            match v {
                statsd_sink::StatsdSpecifier::Address(v) => {
                    struct_ser.serialize_field("address", v)?;
                }
                statsd_sink::StatsdSpecifier::TcpClusterName(v) => {
                    struct_ser.serialize_field("tcpClusterName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatsdSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prefix",
            "address",
            "tcp_cluster_name",
            "tcpClusterName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            Address,
            TcpClusterName,
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
                            "prefix" => Ok(GeneratedField::Prefix),
                            "address" => Ok(GeneratedField::Address),
                            "tcpClusterName" | "tcp_cluster_name" => Ok(GeneratedField::TcpClusterName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatsdSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.StatsdSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatsdSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                let mut statsd_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if statsd_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            statsd_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(statsd_sink::StatsdSpecifier::Address)
;
                        }
                        GeneratedField::TcpClusterName => {
                            if statsd_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpClusterName"));
                            }
                            statsd_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(statsd_sink::StatsdSpecifier::TcpClusterName);
                        }
                    }
                }
                Ok(StatsdSink {
                    prefix: prefix__.unwrap_or_default(),
                    statsd_specifier: statsd_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.StatsdSink", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TagSpecifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tag_name.is_empty() {
            len += 1;
        }
        if self.tag_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.metrics.v2.TagSpecifier", len)?;
        if !self.tag_name.is_empty() {
            struct_ser.serialize_field("tagName", &self.tag_name)?;
        }
        if let Some(v) = self.tag_value.as_ref() {
            match v {
                tag_specifier::TagValue::Regex(v) => {
                    struct_ser.serialize_field("regex", v)?;
                }
                tag_specifier::TagValue::FixedValue(v) => {
                    struct_ser.serialize_field("fixedValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TagSpecifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tag_name",
            "tagName",
            "regex",
            "fixed_value",
            "fixedValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TagName,
            Regex,
            FixedValue,
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
                            "tagName" | "tag_name" => Ok(GeneratedField::TagName),
                            "regex" => Ok(GeneratedField::Regex),
                            "fixedValue" | "fixed_value" => Ok(GeneratedField::FixedValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TagSpecifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.metrics.v2.TagSpecifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TagSpecifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tag_name__ = None;
                let mut tag_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TagName => {
                            if tag_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tagName"));
                            }
                            tag_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Regex => {
                            if tag_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regex"));
                            }
                            tag_value__ = map.next_value::<::std::option::Option<_>>()?.map(tag_specifier::TagValue::Regex);
                        }
                        GeneratedField::FixedValue => {
                            if tag_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedValue"));
                            }
                            tag_value__ = map.next_value::<::std::option::Option<_>>()?.map(tag_specifier::TagValue::FixedValue);
                        }
                    }
                }
                Ok(TagSpecifier {
                    tag_name: tag_name__.unwrap_or_default(),
                    tag_value: tag_value__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.metrics.v2.TagSpecifier", FIELDS, GeneratedVisitor)
    }
}
