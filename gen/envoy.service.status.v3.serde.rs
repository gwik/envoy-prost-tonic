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
        if !self.generic_xds_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v3.ClientConfig", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if !self.xds_config.is_empty() {
            struct_ser.serialize_field("xdsConfig", &self.xds_config)?;
        }
        if !self.generic_xds_configs.is_empty() {
            struct_ser.serialize_field("genericXdsConfigs", &self.generic_xds_configs)?;
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
            "generic_xds_configs",
            "genericXdsConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            XdsConfig,
            GenericXdsConfigs,
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
                            "genericXdsConfigs" | "generic_xds_configs" => Ok(GeneratedField::GenericXdsConfigs),
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
                formatter.write_str("struct envoy.service.status.v3.ClientConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut xds_config__ = None;
                let mut generic_xds_configs__ = None;
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
                        GeneratedField::GenericXdsConfigs => {
                            if generic_xds_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericXdsConfigs"));
                            }
                            generic_xds_configs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientConfig {
                    node: node__,
                    xds_config: xds_config__.unwrap_or_default(),
                    generic_xds_configs: generic_xds_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v3.ClientConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for client_config::GenericXdsConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.type_url.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.xds_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.config_status != 0 {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.is_static_resource {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v3.ClientConfig.GenericXdsConfig", len)?;
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.xds_config.as_ref() {
            struct_ser.serialize_field("xdsConfig", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if self.config_status != 0 {
            let v = ConfigStatus::from_i32(self.config_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.config_status)))?;
            struct_ser.serialize_field("configStatus", &v)?;
        }
        if self.client_status != 0 {
            let v = super::super::super::admin::v3::ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.is_static_resource {
            struct_ser.serialize_field("isStaticResource", &self.is_static_resource)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for client_config::GenericXdsConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type_url",
            "typeUrl",
            "name",
            "version_info",
            "versionInfo",
            "xds_config",
            "xdsConfig",
            "last_updated",
            "lastUpdated",
            "config_status",
            "configStatus",
            "client_status",
            "clientStatus",
            "error_state",
            "errorState",
            "is_static_resource",
            "isStaticResource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeUrl,
            Name,
            VersionInfo,
            XdsConfig,
            LastUpdated,
            ConfigStatus,
            ClientStatus,
            ErrorState,
            IsStaticResource,
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
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            "name" => Ok(GeneratedField::Name),
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "xdsConfig" | "xds_config" => Ok(GeneratedField::XdsConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "configStatus" | "config_status" => Ok(GeneratedField::ConfigStatus),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "isStaticResource" | "is_static_resource" => Ok(GeneratedField::IsStaticResource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = client_config::GenericXdsConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.status.v3.ClientConfig.GenericXdsConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<client_config::GenericXdsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_url__ = None;
                let mut name__ = None;
                let mut version_info__ = None;
                let mut xds_config__ = None;
                let mut last_updated__ = None;
                let mut config_status__ = None;
                let mut client_status__ = None;
                let mut error_state__ = None;
                let mut is_static_resource__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::XdsConfig => {
                            if xds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xdsConfig"));
                            }
                            xds_config__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::ConfigStatus => {
                            if config_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configStatus"));
                            }
                            config_status__ = Some(map.next_value::<ConfigStatus>()? as i32);
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<super::super::super::admin::v3::ClientResourceStatus>()? as i32);
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::IsStaticResource => {
                            if is_static_resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isStaticResource"));
                            }
                            is_static_resource__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(client_config::GenericXdsConfig {
                    type_url: type_url__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    version_info: version_info__.unwrap_or_default(),
                    xds_config: xds_config__,
                    last_updated: last_updated__,
                    config_status: config_status__.unwrap_or_default(),
                    client_status: client_status__.unwrap_or_default(),
                    error_state: error_state__,
                    is_static_resource: is_static_resource__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v3.ClientConfig.GenericXdsConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientConfigStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ClientUnknown => "CLIENT_UNKNOWN",
            Self::ClientRequested => "CLIENT_REQUESTED",
            Self::ClientAcked => "CLIENT_ACKED",
            Self::ClientNacked => "CLIENT_NACKED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ClientConfigStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CLIENT_UNKNOWN",
            "CLIENT_REQUESTED",
            "CLIENT_ACKED",
            "CLIENT_NACKED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientConfigStatus;

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
                    .and_then(ClientConfigStatus::from_i32)
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
                    .and_then(ClientConfigStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CLIENT_UNKNOWN" => Ok(ClientConfigStatus::ClientUnknown),
                    "CLIENT_REQUESTED" => Ok(ClientConfigStatus::ClientRequested),
                    "CLIENT_ACKED" => Ok(ClientConfigStatus::ClientAcked),
                    "CLIENT_NACKED" => Ok(ClientConfigStatus::ClientNacked),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        if self.node.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v3.ClientStatusRequest", len)?;
        if !self.node_matchers.is_empty() {
            struct_ser.serialize_field("nodeMatchers", &self.node_matchers)?;
        }
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
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
            "node",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NodeMatchers,
            Node,
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
                            "node" => Ok(GeneratedField::Node),
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
                formatter.write_str("struct envoy.service.status.v3.ClientStatusRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node_matchers__ = None;
                let mut node__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NodeMatchers => {
                            if node_matchers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeMatchers"));
                            }
                            node_matchers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                    }
                }
                Ok(ClientStatusRequest {
                    node_matchers: node_matchers__.unwrap_or_default(),
                    node: node__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v3.ClientStatusRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v3.ClientStatusResponse", len)?;
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
                formatter.write_str("struct envoy.service.status.v3.ClientStatusResponse")
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
        deserializer.deserialize_struct("envoy.service.status.v3.ClientStatusResponse", FIELDS, GeneratedVisitor)
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
        if self.client_status != 0 {
            len += 1;
        }
        if self.per_xds_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.status.v3.PerXdsConfig", len)?;
        if self.status != 0 {
            let v = ConfigStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.client_status != 0 {
            let v = ClientConfigStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
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
                per_xds_config::PerXdsConfig::EndpointConfig(v) => {
                    struct_ser.serialize_field("endpointConfig", v)?;
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
            "client_status",
            "clientStatus",
            "listener_config",
            "listenerConfig",
            "cluster_config",
            "clusterConfig",
            "route_config",
            "routeConfig",
            "scoped_route_config",
            "scopedRouteConfig",
            "endpoint_config",
            "endpointConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            ClientStatus,
            ListenerConfig,
            ClusterConfig,
            RouteConfig,
            ScopedRouteConfig,
            EndpointConfig,
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
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            "listenerConfig" | "listener_config" => Ok(GeneratedField::ListenerConfig),
                            "clusterConfig" | "cluster_config" => Ok(GeneratedField::ClusterConfig),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "scopedRouteConfig" | "scoped_route_config" => Ok(GeneratedField::ScopedRouteConfig),
                            "endpointConfig" | "endpoint_config" => Ok(GeneratedField::EndpointConfig),
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
                formatter.write_str("struct envoy.service.status.v3.PerXdsConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PerXdsConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut client_status__ = None;
                let mut per_xds_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<ConfigStatus>()? as i32);
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientConfigStatus>()? as i32);
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
                        GeneratedField::EndpointConfig => {
                            if per_xds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointConfig"));
                            }
                            per_xds_config__ = map.next_value::<::std::option::Option<_>>()?.map(per_xds_config::PerXdsConfig::EndpointConfig)
;
                        }
                    }
                }
                Ok(PerXdsConfig {
                    status: status__.unwrap_or_default(),
                    client_status: client_status__.unwrap_or_default(),
                    per_xds_config: per_xds_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.status.v3.PerXdsConfig", FIELDS, GeneratedVisitor)
    }
}
