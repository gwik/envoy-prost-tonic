// @generated
impl serde::Serialize for RedisProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.auth_password.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProtocolOptions", len)?;
        if let Some(v) = self.auth_password.as_ref() {
            struct_ser.serialize_field("authPassword", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedisProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_password",
            "authPassword",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthPassword,
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
                            "authPassword" | "auth_password" => Ok(GeneratedField::AuthPassword),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedisProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.redis_proxy.v2.RedisProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RedisProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_password__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AuthPassword => {
                            if auth_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authPassword"));
                            }
                            auth_password__ = map.next_value()?;
                        }
                    }
                }
                Ok(RedisProtocolOptions {
                    auth_password: auth_password__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RedisProxy {
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
        if !self.cluster.is_empty() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        if self.latency_in_micros {
            len += 1;
        }
        if self.prefix_routes.is_some() {
            len += 1;
        }
        if self.downstream_auth_password.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        if self.latency_in_micros {
            struct_ser.serialize_field("latencyInMicros", &self.latency_in_micros)?;
        }
        if let Some(v) = self.prefix_routes.as_ref() {
            struct_ser.serialize_field("prefixRoutes", v)?;
        }
        if let Some(v) = self.downstream_auth_password.as_ref() {
            struct_ser.serialize_field("downstreamAuthPassword", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedisProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "cluster",
            "settings",
            "latency_in_micros",
            "latencyInMicros",
            "prefix_routes",
            "prefixRoutes",
            "downstream_auth_password",
            "downstreamAuthPassword",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            Cluster,
            Settings,
            LatencyInMicros,
            PrefixRoutes,
            DownstreamAuthPassword,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "settings" => Ok(GeneratedField::Settings),
                            "latencyInMicros" | "latency_in_micros" => Ok(GeneratedField::LatencyInMicros),
                            "prefixRoutes" | "prefix_routes" => Ok(GeneratedField::PrefixRoutes),
                            "downstreamAuthPassword" | "downstream_auth_password" => Ok(GeneratedField::DownstreamAuthPassword),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedisProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.redis_proxy.v2.RedisProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RedisProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut cluster__ = None;
                let mut settings__ = None;
                let mut latency_in_micros__ = None;
                let mut prefix_routes__ = None;
                let mut downstream_auth_password__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map.next_value()?;
                        }
                        GeneratedField::LatencyInMicros => {
                            if latency_in_micros__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latencyInMicros"));
                            }
                            latency_in_micros__ = Some(map.next_value()?);
                        }
                        GeneratedField::PrefixRoutes => {
                            if prefix_routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixRoutes"));
                            }
                            prefix_routes__ = map.next_value()?;
                        }
                        GeneratedField::DownstreamAuthPassword => {
                            if downstream_auth_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamAuthPassword"));
                            }
                            downstream_auth_password__ = map.next_value()?;
                        }
                    }
                }
                Ok(RedisProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    cluster: cluster__.unwrap_or_default(),
                    settings: settings__,
                    latency_in_micros: latency_in_micros__.unwrap_or_default(),
                    prefix_routes: prefix_routes__,
                    downstream_auth_password: downstream_auth_password__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for redis_proxy::ConnPoolSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.op_timeout.is_some() {
            len += 1;
        }
        if self.enable_hashtagging {
            len += 1;
        }
        if self.enable_redirection {
            len += 1;
        }
        if self.max_buffer_size_before_flush != 0 {
            len += 1;
        }
        if self.buffer_flush_timeout.is_some() {
            len += 1;
        }
        if self.max_upstream_unknown_connections.is_some() {
            len += 1;
        }
        if self.enable_command_stats {
            len += 1;
        }
        if self.read_policy != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.ConnPoolSettings", len)?;
        if let Some(v) = self.op_timeout.as_ref() {
            struct_ser.serialize_field("opTimeout", v)?;
        }
        if self.enable_hashtagging {
            struct_ser.serialize_field("enableHashtagging", &self.enable_hashtagging)?;
        }
        if self.enable_redirection {
            struct_ser.serialize_field("enableRedirection", &self.enable_redirection)?;
        }
        if self.max_buffer_size_before_flush != 0 {
            struct_ser.serialize_field("maxBufferSizeBeforeFlush", &self.max_buffer_size_before_flush)?;
        }
        if let Some(v) = self.buffer_flush_timeout.as_ref() {
            struct_ser.serialize_field("bufferFlushTimeout", v)?;
        }
        if let Some(v) = self.max_upstream_unknown_connections.as_ref() {
            struct_ser.serialize_field("maxUpstreamUnknownConnections", v)?;
        }
        if self.enable_command_stats {
            struct_ser.serialize_field("enableCommandStats", &self.enable_command_stats)?;
        }
        if self.read_policy != 0 {
            let v = redis_proxy::conn_pool_settings::ReadPolicy::from_i32(self.read_policy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.read_policy)))?;
            struct_ser.serialize_field("readPolicy", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for redis_proxy::ConnPoolSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "op_timeout",
            "opTimeout",
            "enable_hashtagging",
            "enableHashtagging",
            "enable_redirection",
            "enableRedirection",
            "max_buffer_size_before_flush",
            "maxBufferSizeBeforeFlush",
            "buffer_flush_timeout",
            "bufferFlushTimeout",
            "max_upstream_unknown_connections",
            "maxUpstreamUnknownConnections",
            "enable_command_stats",
            "enableCommandStats",
            "read_policy",
            "readPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OpTimeout,
            EnableHashtagging,
            EnableRedirection,
            MaxBufferSizeBeforeFlush,
            BufferFlushTimeout,
            MaxUpstreamUnknownConnections,
            EnableCommandStats,
            ReadPolicy,
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
                            "opTimeout" | "op_timeout" => Ok(GeneratedField::OpTimeout),
                            "enableHashtagging" | "enable_hashtagging" => Ok(GeneratedField::EnableHashtagging),
                            "enableRedirection" | "enable_redirection" => Ok(GeneratedField::EnableRedirection),
                            "maxBufferSizeBeforeFlush" | "max_buffer_size_before_flush" => Ok(GeneratedField::MaxBufferSizeBeforeFlush),
                            "bufferFlushTimeout" | "buffer_flush_timeout" => Ok(GeneratedField::BufferFlushTimeout),
                            "maxUpstreamUnknownConnections" | "max_upstream_unknown_connections" => Ok(GeneratedField::MaxUpstreamUnknownConnections),
                            "enableCommandStats" | "enable_command_stats" => Ok(GeneratedField::EnableCommandStats),
                            "readPolicy" | "read_policy" => Ok(GeneratedField::ReadPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = redis_proxy::ConnPoolSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.redis_proxy.v2.RedisProxy.ConnPoolSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<redis_proxy::ConnPoolSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut op_timeout__ = None;
                let mut enable_hashtagging__ = None;
                let mut enable_redirection__ = None;
                let mut max_buffer_size_before_flush__ = None;
                let mut buffer_flush_timeout__ = None;
                let mut max_upstream_unknown_connections__ = None;
                let mut enable_command_stats__ = None;
                let mut read_policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OpTimeout => {
                            if op_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("opTimeout"));
                            }
                            op_timeout__ = map.next_value()?;
                        }
                        GeneratedField::EnableHashtagging => {
                            if enable_hashtagging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableHashtagging"));
                            }
                            enable_hashtagging__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnableRedirection => {
                            if enable_redirection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableRedirection"));
                            }
                            enable_redirection__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxBufferSizeBeforeFlush => {
                            if max_buffer_size_before_flush__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBufferSizeBeforeFlush"));
                            }
                            max_buffer_size_before_flush__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BufferFlushTimeout => {
                            if buffer_flush_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufferFlushTimeout"));
                            }
                            buffer_flush_timeout__ = map.next_value()?;
                        }
                        GeneratedField::MaxUpstreamUnknownConnections => {
                            if max_upstream_unknown_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxUpstreamUnknownConnections"));
                            }
                            max_upstream_unknown_connections__ = map.next_value()?;
                        }
                        GeneratedField::EnableCommandStats => {
                            if enable_command_stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableCommandStats"));
                            }
                            enable_command_stats__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReadPolicy => {
                            if read_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readPolicy"));
                            }
                            read_policy__ = Some(map.next_value::<redis_proxy::conn_pool_settings::ReadPolicy>()? as i32);
                        }
                    }
                }
                Ok(redis_proxy::ConnPoolSettings {
                    op_timeout: op_timeout__,
                    enable_hashtagging: enable_hashtagging__.unwrap_or_default(),
                    enable_redirection: enable_redirection__.unwrap_or_default(),
                    max_buffer_size_before_flush: max_buffer_size_before_flush__.unwrap_or_default(),
                    buffer_flush_timeout: buffer_flush_timeout__,
                    max_upstream_unknown_connections: max_upstream_unknown_connections__,
                    enable_command_stats: enable_command_stats__.unwrap_or_default(),
                    read_policy: read_policy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.ConnPoolSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for redis_proxy::conn_pool_settings::ReadPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Master => "MASTER",
            Self::PreferMaster => "PREFER_MASTER",
            Self::Replica => "REPLICA",
            Self::PreferReplica => "PREFER_REPLICA",
            Self::Any => "ANY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for redis_proxy::conn_pool_settings::ReadPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MASTER",
            "PREFER_MASTER",
            "REPLICA",
            "PREFER_REPLICA",
            "ANY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = redis_proxy::conn_pool_settings::ReadPolicy;

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
                    .and_then(redis_proxy::conn_pool_settings::ReadPolicy::from_i32)
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
                    .and_then(redis_proxy::conn_pool_settings::ReadPolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MASTER" => Ok(redis_proxy::conn_pool_settings::ReadPolicy::Master),
                    "PREFER_MASTER" => Ok(redis_proxy::conn_pool_settings::ReadPolicy::PreferMaster),
                    "REPLICA" => Ok(redis_proxy::conn_pool_settings::ReadPolicy::Replica),
                    "PREFER_REPLICA" => Ok(redis_proxy::conn_pool_settings::ReadPolicy::PreferReplica),
                    "ANY" => Ok(redis_proxy::conn_pool_settings::ReadPolicy::Any),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for redis_proxy::PrefixRoutes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.routes.is_empty() {
            len += 1;
        }
        if self.case_insensitive {
            len += 1;
        }
        if !self.catch_all_cluster.is_empty() {
            len += 1;
        }
        if self.catch_all_route.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes", len)?;
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        if self.case_insensitive {
            struct_ser.serialize_field("caseInsensitive", &self.case_insensitive)?;
        }
        if !self.catch_all_cluster.is_empty() {
            struct_ser.serialize_field("catchAllCluster", &self.catch_all_cluster)?;
        }
        if let Some(v) = self.catch_all_route.as_ref() {
            struct_ser.serialize_field("catchAllRoute", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for redis_proxy::PrefixRoutes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routes",
            "case_insensitive",
            "caseInsensitive",
            "catch_all_cluster",
            "catchAllCluster",
            "catch_all_route",
            "catchAllRoute",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Routes,
            CaseInsensitive,
            CatchAllCluster,
            CatchAllRoute,
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
                            "routes" => Ok(GeneratedField::Routes),
                            "caseInsensitive" | "case_insensitive" => Ok(GeneratedField::CaseInsensitive),
                            "catchAllCluster" | "catch_all_cluster" => Ok(GeneratedField::CatchAllCluster),
                            "catchAllRoute" | "catch_all_route" => Ok(GeneratedField::CatchAllRoute),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = redis_proxy::PrefixRoutes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<redis_proxy::PrefixRoutes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut routes__ = None;
                let mut case_insensitive__ = None;
                let mut catch_all_cluster__ = None;
                let mut catch_all_route__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = Some(map.next_value()?);
                        }
                        GeneratedField::CaseInsensitive => {
                            if case_insensitive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caseInsensitive"));
                            }
                            case_insensitive__ = Some(map.next_value()?);
                        }
                        GeneratedField::CatchAllCluster => {
                            if catch_all_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catchAllCluster"));
                            }
                            catch_all_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::CatchAllRoute => {
                            if catch_all_route__.is_some() {
                                return Err(serde::de::Error::duplicate_field("catchAllRoute"));
                            }
                            catch_all_route__ = map.next_value()?;
                        }
                    }
                }
                Ok(redis_proxy::PrefixRoutes {
                    routes: routes__.unwrap_or_default(),
                    case_insensitive: case_insensitive__.unwrap_or_default(),
                    catch_all_cluster: catch_all_cluster__.unwrap_or_default(),
                    catch_all_route: catch_all_route__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for redis_proxy::prefix_routes::Route {
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
        if self.remove_prefix {
            len += 1;
        }
        if !self.cluster.is_empty() {
            len += 1;
        }
        if !self.request_mirror_policy.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes.Route", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        if self.remove_prefix {
            struct_ser.serialize_field("removePrefix", &self.remove_prefix)?;
        }
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if !self.request_mirror_policy.is_empty() {
            struct_ser.serialize_field("requestMirrorPolicy", &self.request_mirror_policy)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for redis_proxy::prefix_routes::Route {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prefix",
            "remove_prefix",
            "removePrefix",
            "cluster",
            "request_mirror_policy",
            "requestMirrorPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            RemovePrefix,
            Cluster,
            RequestMirrorPolicy,
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
                            "removePrefix" | "remove_prefix" => Ok(GeneratedField::RemovePrefix),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "requestMirrorPolicy" | "request_mirror_policy" => Ok(GeneratedField::RequestMirrorPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = redis_proxy::prefix_routes::Route;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes.Route")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<redis_proxy::prefix_routes::Route, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                let mut remove_prefix__ = None;
                let mut cluster__ = None;
                let mut request_mirror_policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemovePrefix => {
                            if remove_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removePrefix"));
                            }
                            remove_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestMirrorPolicy => {
                            if request_mirror_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMirrorPolicy"));
                            }
                            request_mirror_policy__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(redis_proxy::prefix_routes::Route {
                    prefix: prefix__.unwrap_or_default(),
                    remove_prefix: remove_prefix__.unwrap_or_default(),
                    cluster: cluster__.unwrap_or_default(),
                    request_mirror_policy: request_mirror_policy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes.Route", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for redis_proxy::prefix_routes::route::RequestMirrorPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster.is_empty() {
            len += 1;
        }
        if self.runtime_fraction.is_some() {
            len += 1;
        }
        if self.exclude_read_commands {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes.Route.RequestMirrorPolicy", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if let Some(v) = self.runtime_fraction.as_ref() {
            struct_ser.serialize_field("runtimeFraction", v)?;
        }
        if self.exclude_read_commands {
            struct_ser.serialize_field("excludeReadCommands", &self.exclude_read_commands)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for redis_proxy::prefix_routes::route::RequestMirrorPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "runtime_fraction",
            "runtimeFraction",
            "exclude_read_commands",
            "excludeReadCommands",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            RuntimeFraction,
            ExcludeReadCommands,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "runtimeFraction" | "runtime_fraction" => Ok(GeneratedField::RuntimeFraction),
                            "excludeReadCommands" | "exclude_read_commands" => Ok(GeneratedField::ExcludeReadCommands),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = redis_proxy::prefix_routes::route::RequestMirrorPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes.Route.RequestMirrorPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<redis_proxy::prefix_routes::route::RequestMirrorPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut runtime_fraction__ = None;
                let mut exclude_read_commands__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::RuntimeFraction => {
                            if runtime_fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeFraction"));
                            }
                            runtime_fraction__ = map.next_value()?;
                        }
                        GeneratedField::ExcludeReadCommands => {
                            if exclude_read_commands__.is_some() {
                                return Err(serde::de::Error::duplicate_field("excludeReadCommands"));
                            }
                            exclude_read_commands__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(redis_proxy::prefix_routes::route::RequestMirrorPolicy {
                    cluster: cluster__.unwrap_or_default(),
                    runtime_fraction: runtime_fraction__,
                    exclude_read_commands: exclude_read_commands__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.redis_proxy.v2.RedisProxy.PrefixRoutes.Route.RequestMirrorPolicy", FIELDS, GeneratedVisitor)
    }
}
