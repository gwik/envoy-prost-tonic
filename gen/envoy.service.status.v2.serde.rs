// @generated
impl serde::Serialize for ClientConfig {
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
        if !self.xds_config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v2.ClientConfig", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.xds_config.is_empty() {
            struct_ser.serialize_field("xdsConfig", &self.xds_config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "xds_config",
            "xdsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            XdsConfig,
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
                            "xdsConfig" | "xds_config" => Ok(GeneratedField::XdsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.status.v2.ClientConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut xds_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                        GeneratedField::XdsConfig => {
                            if xds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xdsConfig"));
                            }
                            xds_config__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientConfig {
                    node: node__,
                    xds_config: xds_config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v2.ClientConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.node_matchers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v2.ClientStatusRequest", len)?;
        if !self.node_matchers.is_empty() {
            struct_ser.serialize_field("nodeMatchers", &self.node_matchers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node_matchers",
            "nodeMatchers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NodeMatchers,
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
                            "nodeMatchers" | "node_matchers" => Ok(GeneratedField::NodeMatchers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.status.v2.ClientStatusRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node_matchers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NodeMatchers => {
                            if node_matchers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeMatchers"));
                            }
                            node_matchers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientStatusRequest {
                    node_matchers: node_matchers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v2.ClientStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v2.ClientStatusResponse", len)?;
        if !self.config.is_empty() {
            struct_ser.serialize_field("config", &self.config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
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
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.status.v2.ClientStatusResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientStatusResponse {
                    config: config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v2.ClientStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Synced => "SYNCED",
            Self::NotSent => "NOT_SENT",
            Self::Stale => "STALE",
            Self::Error => "ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ConfigStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "SYNCED",
            "NOT_SENT",
            "STALE",
            "ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigStatus;

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
                    .and_then(ConfigStatus::from_i32)
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
                    .and_then(ConfigStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(ConfigStatus::Unknown),
                    "SYNCED" => Ok(ConfigStatus::Synced),
                    "NOT_SENT" => Ok(ConfigStatus::NotSent),
                    "STALE" => Ok(ConfigStatus::Stale),
                    "ERROR" => Ok(ConfigStatus::Error),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for PerXdsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.per_xds_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v2.PerXdsConfig", len)?;
        if self.status != 0 {
            let v = ConfigStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.per_xds_config.as_ref() {
            match v {
                per_xds_config::PerXdsConfig::ListenerConfig(v) => {
                    struct_ser.serialize_field("listenerConfig", v)?;
                }
                per_xds_config::PerXdsConfig::ClusterConfig(v) => {
                    struct_ser.serialize_field("clusterConfig", v)?;
                }
                per_xds_config::PerXdsConfig::RouteConfig(v) => {
                    struct_ser.serialize_field("routeConfig", v)?;
                }
                per_xds_config::PerXdsConfig::ScopedRouteConfig(v) => {
                    struct_ser.serialize_field("scopedRouteConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PerXdsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "listener_config",
            "listenerConfig",
            "cluster_config",
            "clusterConfig",
            "route_config",
            "routeConfig",
            "scoped_route_config",
            "scopedRouteConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            ListenerConfig,
            ClusterConfig,
            RouteConfig,
            ScopedRouteConfig,
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
                            "status" => Ok(GeneratedField::Status),
                            "listenerConfig" | "listener_config" => Ok(GeneratedField::ListenerConfig),
                            "clusterConfig" | "cluster_config" => Ok(GeneratedField::ClusterConfig),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "scopedRouteConfig" | "scoped_route_config" => Ok(GeneratedField::ScopedRouteConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PerXdsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.status.v2.PerXdsConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PerXdsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut per_xds_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<ConfigStatus>()? as i32);
                        }
                        GeneratedField::ListenerConfig => {
                            if per_xds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listenerConfig"));
                            }
                            per_xds_config__ = map.next_value::<::std::option::Option<_>>()?.map(per_xds_config::PerXdsConfig::ListenerConfig)
;
                        }
                        GeneratedField::ClusterConfig => {
                            if per_xds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterConfig"));
                            }
                            per_xds_config__ = map.next_value::<::std::option::Option<_>>()?.map(per_xds_config::PerXdsConfig::ClusterConfig)
;
                        }
                        GeneratedField::RouteConfig => {
                            if per_xds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            per_xds_config__ = map.next_value::<::std::option::Option<_>>()?.map(per_xds_config::PerXdsConfig::RouteConfig)
;
                        }
                        GeneratedField::ScopedRouteConfig => {
                            if per_xds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfig"));
                            }
                            per_xds_config__ = map.next_value::<::std::option::Option<_>>()?.map(per_xds_config::PerXdsConfig::ScopedRouteConfig)
;
                        }
                    }
                }
                Ok(PerXdsConfig {
                    status: status__.unwrap_or_default(),
                    per_xds_config: per_xds_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v2.PerXdsConfig", FIELDS, GeneratedVisitor)
    }
}
