// @generated
impl serde::Serialize for Admin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.access_log_path.is_empty() {
            len += 1;
        }
        if !self.profile_path.is_empty() {
            len += 1;
        }
        if self.address.is_some() {
            len += 1;
        }
        if !self.socket_options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.Admin", len)?;
        if !self.access_log_path.is_empty() {
            struct_ser.serialize_field("accessLogPath", &self.access_log_path)?;
        }
        if !self.profile_path.is_empty() {
            struct_ser.serialize_field("profilePath", &self.profile_path)?;
        }
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if !self.socket_options.is_empty() {
            struct_ser.serialize_field("socketOptions", &self.socket_options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Admin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_log_path",
            "accessLogPath",
            "profile_path",
            "profilePath",
            "address",
            "socket_options",
            "socketOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessLogPath,
            ProfilePath,
            Address,
            SocketOptions,
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
                            "accessLogPath" | "access_log_path" => Ok(GeneratedField::AccessLogPath),
                            "profilePath" | "profile_path" => Ok(GeneratedField::ProfilePath),
                            "address" => Ok(GeneratedField::Address),
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Admin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.Admin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Admin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut access_log_path__ = None;
                let mut profile_path__ = None;
                let mut address__ = None;
                let mut socket_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AccessLogPath => {
                            if access_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLogPath"));
                            }
                            access_log_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProfilePath => {
                            if profile_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("profilePath"));
                            }
                            profile_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::SocketOptions => {
                            if socket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketOptions"));
                            }
                            socket_options__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Admin {
                    access_log_path: access_log_path__.unwrap_or_default(),
                    profile_path: profile_path__.unwrap_or_default(),
                    address: address__,
                    socket_options: socket_options__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.Admin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Bootstrap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node.is_some() {
            len += 1;
        }
        if self.static_resources.is_some() {
            len += 1;
        }
        if self.dynamic_resources.is_some() {
            len += 1;
        }
        if self.cluster_manager.is_some() {
            len += 1;
        }
        if self.hds_config.is_some() {
            len += 1;
        }
        if !self.flags_path.is_empty() {
            len += 1;
        }
        if !self.stats_sinks.is_empty() {
            len += 1;
        }
        if self.stats_config.is_some() {
            len += 1;
        }
        if self.stats_flush_interval.is_some() {
            len += 1;
        }
        if self.watchdog.is_some() {
            len += 1;
        }
        if self.tracing.is_some() {
            len += 1;
        }
        if self.runtime.is_some() {
            len += 1;
        }
        if self.layered_runtime.is_some() {
            len += 1;
        }
        if self.admin.is_some() {
            len += 1;
        }
        if self.overload_manager.is_some() {
            len += 1;
        }
        if self.enable_dispatcher_stats {
            len += 1;
        }
        if !self.header_prefix.is_empty() {
            len += 1;
        }
        if self.stats_server_version_override.is_some() {
            len += 1;
        }
        if self.use_tcp_for_dns_lookups {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.Bootstrap", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if let Some(v) = self.static_resources.as_ref() {
            struct_ser.serialize_field("staticResources", v)?;
        }
        if let Some(v) = self.dynamic_resources.as_ref() {
            struct_ser.serialize_field("dynamicResources", v)?;
        }
        if let Some(v) = self.cluster_manager.as_ref() {
            struct_ser.serialize_field("clusterManager", v)?;
        }
        if let Some(v) = self.hds_config.as_ref() {
            struct_ser.serialize_field("hdsConfig", v)?;
        }
        if !self.flags_path.is_empty() {
            struct_ser.serialize_field("flagsPath", &self.flags_path)?;
        }
        if !self.stats_sinks.is_empty() {
            struct_ser.serialize_field("statsSinks", &self.stats_sinks)?;
        }
        if let Some(v) = self.stats_config.as_ref() {
            struct_ser.serialize_field("statsConfig", v)?;
        }
        if let Some(v) = self.stats_flush_interval.as_ref() {
            struct_ser.serialize_field("statsFlushInterval", v)?;
        }
        if let Some(v) = self.watchdog.as_ref() {
            struct_ser.serialize_field("watchdog", v)?;
        }
        if let Some(v) = self.tracing.as_ref() {
            struct_ser.serialize_field("tracing", v)?;
        }
        if let Some(v) = self.runtime.as_ref() {
            struct_ser.serialize_field("runtime", v)?;
        }
        if let Some(v) = self.layered_runtime.as_ref() {
            struct_ser.serialize_field("layeredRuntime", v)?;
        }
        if let Some(v) = self.admin.as_ref() {
            struct_ser.serialize_field("admin", v)?;
        }
        if let Some(v) = self.overload_manager.as_ref() {
            struct_ser.serialize_field("overloadManager", v)?;
        }
        if self.enable_dispatcher_stats {
            struct_ser.serialize_field("enableDispatcherStats", &self.enable_dispatcher_stats)?;
        }
        if !self.header_prefix.is_empty() {
            struct_ser.serialize_field("headerPrefix", &self.header_prefix)?;
        }
        if let Some(v) = self.stats_server_version_override.as_ref() {
            struct_ser.serialize_field("statsServerVersionOverride", v)?;
        }
        if self.use_tcp_for_dns_lookups {
            struct_ser.serialize_field("useTcpForDnsLookups", &self.use_tcp_for_dns_lookups)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bootstrap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "static_resources",
            "staticResources",
            "dynamic_resources",
            "dynamicResources",
            "cluster_manager",
            "clusterManager",
            "hds_config",
            "hdsConfig",
            "flags_path",
            "flagsPath",
            "stats_sinks",
            "statsSinks",
            "stats_config",
            "statsConfig",
            "stats_flush_interval",
            "statsFlushInterval",
            "watchdog",
            "tracing",
            "runtime",
            "layered_runtime",
            "layeredRuntime",
            "admin",
            "overload_manager",
            "overloadManager",
            "enable_dispatcher_stats",
            "enableDispatcherStats",
            "header_prefix",
            "headerPrefix",
            "stats_server_version_override",
            "statsServerVersionOverride",
            "use_tcp_for_dns_lookups",
            "useTcpForDnsLookups",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            StaticResources,
            DynamicResources,
            ClusterManager,
            HdsConfig,
            FlagsPath,
            StatsSinks,
            StatsConfig,
            StatsFlushInterval,
            Watchdog,
            Tracing,
            Runtime,
            LayeredRuntime,
            Admin,
            OverloadManager,
            EnableDispatcherStats,
            HeaderPrefix,
            StatsServerVersionOverride,
            UseTcpForDnsLookups,
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
                            "node" => Ok(GeneratedField::Node),
                            "staticResources" | "static_resources" => Ok(GeneratedField::StaticResources),
                            "dynamicResources" | "dynamic_resources" => Ok(GeneratedField::DynamicResources),
                            "clusterManager" | "cluster_manager" => Ok(GeneratedField::ClusterManager),
                            "hdsConfig" | "hds_config" => Ok(GeneratedField::HdsConfig),
                            "flagsPath" | "flags_path" => Ok(GeneratedField::FlagsPath),
                            "statsSinks" | "stats_sinks" => Ok(GeneratedField::StatsSinks),
                            "statsConfig" | "stats_config" => Ok(GeneratedField::StatsConfig),
                            "statsFlushInterval" | "stats_flush_interval" => Ok(GeneratedField::StatsFlushInterval),
                            "watchdog" => Ok(GeneratedField::Watchdog),
                            "tracing" => Ok(GeneratedField::Tracing),
                            "runtime" => Ok(GeneratedField::Runtime),
                            "layeredRuntime" | "layered_runtime" => Ok(GeneratedField::LayeredRuntime),
                            "admin" => Ok(GeneratedField::Admin),
                            "overloadManager" | "overload_manager" => Ok(GeneratedField::OverloadManager),
                            "enableDispatcherStats" | "enable_dispatcher_stats" => Ok(GeneratedField::EnableDispatcherStats),
                            "headerPrefix" | "header_prefix" => Ok(GeneratedField::HeaderPrefix),
                            "statsServerVersionOverride" | "stats_server_version_override" => Ok(GeneratedField::StatsServerVersionOverride),
                            "useTcpForDnsLookups" | "use_tcp_for_dns_lookups" => Ok(GeneratedField::UseTcpForDnsLookups),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Bootstrap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.Bootstrap")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Bootstrap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut static_resources__ = None;
                let mut dynamic_resources__ = None;
                let mut cluster_manager__ = None;
                let mut hds_config__ = None;
                let mut flags_path__ = None;
                let mut stats_sinks__ = None;
                let mut stats_config__ = None;
                let mut stats_flush_interval__ = None;
                let mut watchdog__ = None;
                let mut tracing__ = None;
                let mut runtime__ = None;
                let mut layered_runtime__ = None;
                let mut admin__ = None;
                let mut overload_manager__ = None;
                let mut enable_dispatcher_stats__ = None;
                let mut header_prefix__ = None;
                let mut stats_server_version_override__ = None;
                let mut use_tcp_for_dns_lookups__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                        GeneratedField::StaticResources => {
                            if static_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticResources"));
                            }
                            static_resources__ = map.next_value()?;
                        }
                        GeneratedField::DynamicResources => {
                            if dynamic_resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicResources"));
                            }
                            dynamic_resources__ = map.next_value()?;
                        }
                        GeneratedField::ClusterManager => {
                            if cluster_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterManager"));
                            }
                            cluster_manager__ = map.next_value()?;
                        }
                        GeneratedField::HdsConfig => {
                            if hds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hdsConfig"));
                            }
                            hds_config__ = map.next_value()?;
                        }
                        GeneratedField::FlagsPath => {
                            if flags_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flagsPath"));
                            }
                            flags_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatsSinks => {
                            if stats_sinks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsSinks"));
                            }
                            stats_sinks__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatsConfig => {
                            if stats_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsConfig"));
                            }
                            stats_config__ = map.next_value()?;
                        }
                        GeneratedField::StatsFlushInterval => {
                            if stats_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsFlushInterval"));
                            }
                            stats_flush_interval__ = map.next_value()?;
                        }
                        GeneratedField::Watchdog => {
                            if watchdog__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchdog"));
                            }
                            watchdog__ = map.next_value()?;
                        }
                        GeneratedField::Tracing => {
                            if tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracing"));
                            }
                            tracing__ = map.next_value()?;
                        }
                        GeneratedField::Runtime => {
                            if runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtime"));
                            }
                            runtime__ = map.next_value()?;
                        }
                        GeneratedField::LayeredRuntime => {
                            if layered_runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layeredRuntime"));
                            }
                            layered_runtime__ = map.next_value()?;
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = map.next_value()?;
                        }
                        GeneratedField::OverloadManager => {
                            if overload_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overloadManager"));
                            }
                            overload_manager__ = map.next_value()?;
                        }
                        GeneratedField::EnableDispatcherStats => {
                            if enable_dispatcher_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableDispatcherStats"));
                            }
                            enable_dispatcher_stats__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeaderPrefix => {
                            if header_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerPrefix"));
                            }
                            header_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatsServerVersionOverride => {
                            if stats_server_version_override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsServerVersionOverride"));
                            }
                            stats_server_version_override__ = map.next_value()?;
                        }
                        GeneratedField::UseTcpForDnsLookups => {
                            if use_tcp_for_dns_lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useTcpForDnsLookups"));
                            }
                            use_tcp_for_dns_lookups__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Bootstrap {
                    node: node__,
                    static_resources: static_resources__,
                    dynamic_resources: dynamic_resources__,
                    cluster_manager: cluster_manager__,
                    hds_config: hds_config__,
                    flags_path: flags_path__.unwrap_or_default(),
                    stats_sinks: stats_sinks__.unwrap_or_default(),
                    stats_config: stats_config__,
                    stats_flush_interval: stats_flush_interval__,
                    watchdog: watchdog__,
                    tracing: tracing__,
                    runtime: runtime__,
                    layered_runtime: layered_runtime__,
                    admin: admin__,
                    overload_manager: overload_manager__,
                    enable_dispatcher_stats: enable_dispatcher_stats__.unwrap_or_default(),
                    header_prefix: header_prefix__.unwrap_or_default(),
                    stats_server_version_override: stats_server_version_override__,
                    use_tcp_for_dns_lookups: use_tcp_for_dns_lookups__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.Bootstrap", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::DynamicResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.lds_config.is_some() {
            len += 1;
        }
        if self.cds_config.is_some() {
            len += 1;
        }
        if self.ads_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.Bootstrap.DynamicResources", len)?;
        if let Some(v) = self.lds_config.as_ref() {
            struct_ser.serialize_field("ldsConfig", v)?;
        }
        if let Some(v) = self.cds_config.as_ref() {
            struct_ser.serialize_field("cdsConfig", v)?;
        }
        if let Some(v) = self.ads_config.as_ref() {
            struct_ser.serialize_field("adsConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::DynamicResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "lds_config",
            "ldsConfig",
            "cds_config",
            "cdsConfig",
            "ads_config",
            "adsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LdsConfig,
            CdsConfig,
            AdsConfig,
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
                            "ldsConfig" | "lds_config" => Ok(GeneratedField::LdsConfig),
                            "cdsConfig" | "cds_config" => Ok(GeneratedField::CdsConfig),
                            "adsConfig" | "ads_config" => Ok(GeneratedField::AdsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::DynamicResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.Bootstrap.DynamicResources")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<bootstrap::DynamicResources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut lds_config__ = None;
                let mut cds_config__ = None;
                let mut ads_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LdsConfig => {
                            if lds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ldsConfig"));
                            }
                            lds_config__ = map.next_value()?;
                        }
                        GeneratedField::CdsConfig => {
                            if cds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cdsConfig"));
                            }
                            cds_config__ = map.next_value()?;
                        }
                        GeneratedField::AdsConfig => {
                            if ads_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adsConfig"));
                            }
                            ads_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(bootstrap::DynamicResources {
                    lds_config: lds_config__,
                    cds_config: cds_config__,
                    ads_config: ads_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.Bootstrap.DynamicResources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bootstrap::StaticResources {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.listeners.is_empty() {
            len += 1;
        }
        if !self.clusters.is_empty() {
            len += 1;
        }
        if !self.secrets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.Bootstrap.StaticResources", len)?;
        if !self.listeners.is_empty() {
            struct_ser.serialize_field("listeners", &self.listeners)?;
        }
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        if !self.secrets.is_empty() {
            struct_ser.serialize_field("secrets", &self.secrets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bootstrap::StaticResources {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "listeners",
            "clusters",
            "secrets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Listeners,
            Clusters,
            Secrets,
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
                            "listeners" => Ok(GeneratedField::Listeners),
                            "clusters" => Ok(GeneratedField::Clusters),
                            "secrets" => Ok(GeneratedField::Secrets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bootstrap::StaticResources;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.Bootstrap.StaticResources")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<bootstrap::StaticResources, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut listeners__ = None;
                let mut clusters__ = None;
                let mut secrets__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Listeners => {
                            if listeners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listeners"));
                            }
                            listeners__ = Some(map.next_value()?);
                        }
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Secrets => {
                            if secrets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secrets"));
                            }
                            secrets__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(bootstrap::StaticResources {
                    listeners: listeners__.unwrap_or_default(),
                    clusters: clusters__.unwrap_or_default(),
                    secrets: secrets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.Bootstrap.StaticResources", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.local_cluster_name.is_empty() {
            len += 1;
        }
        if self.outlier_detection.is_some() {
            len += 1;
        }
        if self.upstream_bind_config.is_some() {
            len += 1;
        }
        if self.load_stats_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.ClusterManager", len)?;
        if !self.local_cluster_name.is_empty() {
            struct_ser.serialize_field("localClusterName", &self.local_cluster_name)?;
        }
        if let Some(v) = self.outlier_detection.as_ref() {
            struct_ser.serialize_field("outlierDetection", v)?;
        }
        if let Some(v) = self.upstream_bind_config.as_ref() {
            struct_ser.serialize_field("upstreamBindConfig", v)?;
        }
        if let Some(v) = self.load_stats_config.as_ref() {
            struct_ser.serialize_field("loadStatsConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_cluster_name",
            "localClusterName",
            "outlier_detection",
            "outlierDetection",
            "upstream_bind_config",
            "upstreamBindConfig",
            "load_stats_config",
            "loadStatsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalClusterName,
            OutlierDetection,
            UpstreamBindConfig,
            LoadStatsConfig,
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
                            "localClusterName" | "local_cluster_name" => Ok(GeneratedField::LocalClusterName),
                            "outlierDetection" | "outlier_detection" => Ok(GeneratedField::OutlierDetection),
                            "upstreamBindConfig" | "upstream_bind_config" => Ok(GeneratedField::UpstreamBindConfig),
                            "loadStatsConfig" | "load_stats_config" => Ok(GeneratedField::LoadStatsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.ClusterManager")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut local_cluster_name__ = None;
                let mut outlier_detection__ = None;
                let mut upstream_bind_config__ = None;
                let mut load_stats_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LocalClusterName => {
                            if local_cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localClusterName"));
                            }
                            local_cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::OutlierDetection => {
                            if outlier_detection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outlierDetection"));
                            }
                            outlier_detection__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamBindConfig => {
                            if upstream_bind_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamBindConfig"));
                            }
                            upstream_bind_config__ = map.next_value()?;
                        }
                        GeneratedField::LoadStatsConfig => {
                            if load_stats_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("loadStatsConfig"));
                            }
                            load_stats_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(ClusterManager {
                    local_cluster_name: local_cluster_name__.unwrap_or_default(),
                    outlier_detection: outlier_detection__,
                    upstream_bind_config: upstream_bind_config__,
                    load_stats_config: load_stats_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.ClusterManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cluster_manager::OutlierDetection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.event_log_path.is_empty() {
            len += 1;
        }
        if self.event_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.ClusterManager.OutlierDetection", len)?;
        if !self.event_log_path.is_empty() {
            struct_ser.serialize_field("eventLogPath", &self.event_log_path)?;
        }
        if let Some(v) = self.event_service.as_ref() {
            struct_ser.serialize_field("eventService", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cluster_manager::OutlierDetection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "event_log_path",
            "eventLogPath",
            "event_service",
            "eventService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EventLogPath,
            EventService,
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
                            "eventLogPath" | "event_log_path" => Ok(GeneratedField::EventLogPath),
                            "eventService" | "event_service" => Ok(GeneratedField::EventService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cluster_manager::OutlierDetection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.ClusterManager.OutlierDetection")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cluster_manager::OutlierDetection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut event_log_path__ = None;
                let mut event_service__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EventLogPath => {
                            if event_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventLogPath"));
                            }
                            event_log_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::EventService => {
                            if event_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventService"));
                            }
                            event_service__ = map.next_value()?;
                        }
                    }
                }
                Ok(cluster_manager::OutlierDetection {
                    event_log_path: event_log_path__.unwrap_or_default(),
                    event_service: event_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.ClusterManager.OutlierDetection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LayeredRuntime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.layers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.LayeredRuntime", len)?;
        if !self.layers.is_empty() {
            struct_ser.serialize_field("layers", &self.layers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LayeredRuntime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "layers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Layers,
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
                            "layers" => Ok(GeneratedField::Layers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LayeredRuntime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.LayeredRuntime")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LayeredRuntime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut layers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Layers => {
                            if layers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layers"));
                            }
                            layers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LayeredRuntime {
                    layers: layers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.LayeredRuntime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Runtime {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symlink_root.is_empty() {
            len += 1;
        }
        if !self.subdirectory.is_empty() {
            len += 1;
        }
        if !self.override_subdirectory.is_empty() {
            len += 1;
        }
        if self.base.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.Runtime", len)?;
        if !self.symlink_root.is_empty() {
            struct_ser.serialize_field("symlinkRoot", &self.symlink_root)?;
        }
        if !self.subdirectory.is_empty() {
            struct_ser.serialize_field("subdirectory", &self.subdirectory)?;
        }
        if !self.override_subdirectory.is_empty() {
            struct_ser.serialize_field("overrideSubdirectory", &self.override_subdirectory)?;
        }
        if let Some(v) = self.base.as_ref() {
            struct_ser.serialize_field("base", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Runtime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "symlink_root",
            "symlinkRoot",
            "subdirectory",
            "override_subdirectory",
            "overrideSubdirectory",
            "base",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SymlinkRoot,
            Subdirectory,
            OverrideSubdirectory,
            Base,
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
                            "symlinkRoot" | "symlink_root" => Ok(GeneratedField::SymlinkRoot),
                            "subdirectory" => Ok(GeneratedField::Subdirectory),
                            "overrideSubdirectory" | "override_subdirectory" => Ok(GeneratedField::OverrideSubdirectory),
                            "base" => Ok(GeneratedField::Base),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Runtime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.Runtime")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Runtime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut symlink_root__ = None;
                let mut subdirectory__ = None;
                let mut override_subdirectory__ = None;
                let mut base__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SymlinkRoot => {
                            if symlink_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symlinkRoot"));
                            }
                            symlink_root__ = Some(map.next_value()?);
                        }
                        GeneratedField::Subdirectory => {
                            if subdirectory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subdirectory"));
                            }
                            subdirectory__ = Some(map.next_value()?);
                        }
                        GeneratedField::OverrideSubdirectory => {
                            if override_subdirectory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideSubdirectory"));
                            }
                            override_subdirectory__ = Some(map.next_value()?);
                        }
                        GeneratedField::Base => {
                            if base__.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            base__ = map.next_value()?;
                        }
                    }
                }
                Ok(Runtime {
                    symlink_root: symlink_root__.unwrap_or_default(),
                    subdirectory: subdirectory__.unwrap_or_default(),
                    override_subdirectory: override_subdirectory__.unwrap_or_default(),
                    base: base__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.Runtime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimeLayer {
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
        if self.layer_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.RuntimeLayer", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.layer_specifier.as_ref() {
            match v {
                runtime_layer::LayerSpecifier::StaticLayer(v) => {
                    struct_ser.serialize_field("staticLayer", v)?;
                }
                runtime_layer::LayerSpecifier::DiskLayer(v) => {
                    struct_ser.serialize_field("diskLayer", v)?;
                }
                runtime_layer::LayerSpecifier::AdminLayer(v) => {
                    struct_ser.serialize_field("adminLayer", v)?;
                }
                runtime_layer::LayerSpecifier::RtdsLayer(v) => {
                    struct_ser.serialize_field("rtdsLayer", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimeLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "static_layer",
            "staticLayer",
            "disk_layer",
            "diskLayer",
            "admin_layer",
            "adminLayer",
            "rtds_layer",
            "rtdsLayer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            StaticLayer,
            DiskLayer,
            AdminLayer,
            RtdsLayer,
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
                            "staticLayer" | "static_layer" => Ok(GeneratedField::StaticLayer),
                            "diskLayer" | "disk_layer" => Ok(GeneratedField::DiskLayer),
                            "adminLayer" | "admin_layer" => Ok(GeneratedField::AdminLayer),
                            "rtdsLayer" | "rtds_layer" => Ok(GeneratedField::RtdsLayer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimeLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.RuntimeLayer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuntimeLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut layer_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::StaticLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticLayer"));
                            }
                            layer_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::StaticLayer)
;
                        }
                        GeneratedField::DiskLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("diskLayer"));
                            }
                            layer_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::DiskLayer)
;
                        }
                        GeneratedField::AdminLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminLayer"));
                            }
                            layer_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::AdminLayer)
;
                        }
                        GeneratedField::RtdsLayer => {
                            if layer_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rtdsLayer"));
                            }
                            layer_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(runtime_layer::LayerSpecifier::RtdsLayer)
;
                        }
                    }
                }
                Ok(RuntimeLayer {
                    name: name__.unwrap_or_default(),
                    layer_specifier: layer_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.RuntimeLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for runtime_layer::AdminLayer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.RuntimeLayer.AdminLayer", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for runtime_layer::AdminLayer {
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
            type Value = runtime_layer::AdminLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.RuntimeLayer.AdminLayer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<runtime_layer::AdminLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(runtime_layer::AdminLayer {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.RuntimeLayer.AdminLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for runtime_layer::DiskLayer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.symlink_root.is_empty() {
            len += 1;
        }
        if !self.subdirectory.is_empty() {
            len += 1;
        }
        if self.append_service_cluster {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.RuntimeLayer.DiskLayer", len)?;
        if !self.symlink_root.is_empty() {
            struct_ser.serialize_field("symlinkRoot", &self.symlink_root)?;
        }
        if !self.subdirectory.is_empty() {
            struct_ser.serialize_field("subdirectory", &self.subdirectory)?;
        }
        if self.append_service_cluster {
            struct_ser.serialize_field("appendServiceCluster", &self.append_service_cluster)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for runtime_layer::DiskLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "symlink_root",
            "symlinkRoot",
            "subdirectory",
            "append_service_cluster",
            "appendServiceCluster",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SymlinkRoot,
            Subdirectory,
            AppendServiceCluster,
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
                            "symlinkRoot" | "symlink_root" => Ok(GeneratedField::SymlinkRoot),
                            "subdirectory" => Ok(GeneratedField::Subdirectory),
                            "appendServiceCluster" | "append_service_cluster" => Ok(GeneratedField::AppendServiceCluster),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = runtime_layer::DiskLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.RuntimeLayer.DiskLayer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<runtime_layer::DiskLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut symlink_root__ = None;
                let mut subdirectory__ = None;
                let mut append_service_cluster__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SymlinkRoot => {
                            if symlink_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("symlinkRoot"));
                            }
                            symlink_root__ = Some(map.next_value()?);
                        }
                        GeneratedField::Subdirectory => {
                            if subdirectory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subdirectory"));
                            }
                            subdirectory__ = Some(map.next_value()?);
                        }
                        GeneratedField::AppendServiceCluster => {
                            if append_service_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendServiceCluster"));
                            }
                            append_service_cluster__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(runtime_layer::DiskLayer {
                    symlink_root: symlink_root__.unwrap_or_default(),
                    subdirectory: subdirectory__.unwrap_or_default(),
                    append_service_cluster: append_service_cluster__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.RuntimeLayer.DiskLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for runtime_layer::RtdsLayer {
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
        if self.rtds_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.RuntimeLayer.RtdsLayer", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.rtds_config.as_ref() {
            struct_ser.serialize_field("rtdsConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for runtime_layer::RtdsLayer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "rtds_config",
            "rtdsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RtdsConfig,
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
                            "rtdsConfig" | "rtds_config" => Ok(GeneratedField::RtdsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = runtime_layer::RtdsLayer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.RuntimeLayer.RtdsLayer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<runtime_layer::RtdsLayer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut rtds_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RtdsConfig => {
                            if rtds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rtdsConfig"));
                            }
                            rtds_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(runtime_layer::RtdsLayer {
                    name: name__.unwrap_or_default(),
                    rtds_config: rtds_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.RuntimeLayer.RtdsLayer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Watchdog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.miss_timeout.is_some() {
            len += 1;
        }
        if self.megamiss_timeout.is_some() {
            len += 1;
        }
        if self.kill_timeout.is_some() {
            len += 1;
        }
        if self.multikill_timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.bootstrap.v2.Watchdog", len)?;
        if let Some(v) = self.miss_timeout.as_ref() {
            struct_ser.serialize_field("missTimeout", v)?;
        }
        if let Some(v) = self.megamiss_timeout.as_ref() {
            struct_ser.serialize_field("megamissTimeout", v)?;
        }
        if let Some(v) = self.kill_timeout.as_ref() {
            struct_ser.serialize_field("killTimeout", v)?;
        }
        if let Some(v) = self.multikill_timeout.as_ref() {
            struct_ser.serialize_field("multikillTimeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Watchdog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "miss_timeout",
            "missTimeout",
            "megamiss_timeout",
            "megamissTimeout",
            "kill_timeout",
            "killTimeout",
            "multikill_timeout",
            "multikillTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MissTimeout,
            MegamissTimeout,
            KillTimeout,
            MultikillTimeout,
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
                            "missTimeout" | "miss_timeout" => Ok(GeneratedField::MissTimeout),
                            "megamissTimeout" | "megamiss_timeout" => Ok(GeneratedField::MegamissTimeout),
                            "killTimeout" | "kill_timeout" => Ok(GeneratedField::KillTimeout),
                            "multikillTimeout" | "multikill_timeout" => Ok(GeneratedField::MultikillTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Watchdog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.bootstrap.v2.Watchdog")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Watchdog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut miss_timeout__ = None;
                let mut megamiss_timeout__ = None;
                let mut kill_timeout__ = None;
                let mut multikill_timeout__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MissTimeout => {
                            if miss_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("missTimeout"));
                            }
                            miss_timeout__ = map.next_value()?;
                        }
                        GeneratedField::MegamissTimeout => {
                            if megamiss_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("megamissTimeout"));
                            }
                            megamiss_timeout__ = map.next_value()?;
                        }
                        GeneratedField::KillTimeout => {
                            if kill_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("killTimeout"));
                            }
                            kill_timeout__ = map.next_value()?;
                        }
                        GeneratedField::MultikillTimeout => {
                            if multikill_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multikillTimeout"));
                            }
                            multikill_timeout__ = map.next_value()?;
                        }
                    }
                }
                Ok(Watchdog {
                    miss_timeout: miss_timeout__,
                    megamiss_timeout: megamiss_timeout__,
                    kill_timeout: kill_timeout__,
                    multikill_timeout: multikill_timeout__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.bootstrap.v2.Watchdog", FIELDS, GeneratedVisitor)
    }
}
