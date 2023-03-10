// @generated
impl serde::Serialize for Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.library_id.is_empty() {
            len += 1;
        }
        if !self.library_path.is_empty() {
            len += 1;
        }
        if !self.plugin_name.is_empty() {
            len += 1;
        }
        if self.plugin_config.is_some() {
            len += 1;
        }
        if self.merge_policy != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.golang.v3alpha.Config", len)?;
        if !self.library_id.is_empty() {
            struct_ser.serialize_field("libraryId", &self.library_id)?;
        }
        if !self.library_path.is_empty() {
            struct_ser.serialize_field("libraryPath", &self.library_path)?;
        }
        if !self.plugin_name.is_empty() {
            struct_ser.serialize_field("pluginName", &self.plugin_name)?;
        }
        if let Some(v) = self.plugin_config.as_ref() {
            struct_ser.serialize_field("pluginConfig", v)?;
        }
        if self.merge_policy != 0 {
            let v = config::MergePolicy::from_i32(self.merge_policy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.merge_policy)))?;
            struct_ser.serialize_field("mergePolicy", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "library_id",
            "libraryId",
            "library_path",
            "libraryPath",
            "plugin_name",
            "pluginName",
            "plugin_config",
            "pluginConfig",
            "merge_policy",
            "mergePolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LibraryId,
            LibraryPath,
            PluginName,
            PluginConfig,
            MergePolicy,
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
                            "libraryId" | "library_id" => Ok(GeneratedField::LibraryId),
                            "libraryPath" | "library_path" => Ok(GeneratedField::LibraryPath),
                            "pluginName" | "plugin_name" => Ok(GeneratedField::PluginName),
                            "pluginConfig" | "plugin_config" => Ok(GeneratedField::PluginConfig),
                            "mergePolicy" | "merge_policy" => Ok(GeneratedField::MergePolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.golang.v3alpha.Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut library_id__ = None;
                let mut library_path__ = None;
                let mut plugin_name__ = None;
                let mut plugin_config__ = None;
                let mut merge_policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LibraryId => {
                            if library_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("libraryId"));
                            }
                            library_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::LibraryPath => {
                            if library_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("libraryPath"));
                            }
                            library_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::PluginName => {
                            if plugin_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pluginName"));
                            }
                            plugin_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::PluginConfig => {
                            if plugin_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pluginConfig"));
                            }
                            plugin_config__ = map.next_value()?;
                        }
                        GeneratedField::MergePolicy => {
                            if merge_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergePolicy"));
                            }
                            merge_policy__ = Some(map.next_value::<config::MergePolicy>()? as i32);
                        }
                    }
                }
                Ok(Config {
                    library_id: library_id__.unwrap_or_default(),
                    library_path: library_path__.unwrap_or_default(),
                    plugin_name: plugin_name__.unwrap_or_default(),
                    plugin_config: plugin_config__,
                    merge_policy: merge_policy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.golang.v3alpha.Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for config::MergePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MergeVirtualhostRouterFilter => "MERGE_VIRTUALHOST_ROUTER_FILTER",
            Self::MergeVirtualhostRouter => "MERGE_VIRTUALHOST_ROUTER",
            Self::Override => "OVERRIDE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for config::MergePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MERGE_VIRTUALHOST_ROUTER_FILTER",
            "MERGE_VIRTUALHOST_ROUTER",
            "OVERRIDE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = config::MergePolicy;

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
                    .and_then(config::MergePolicy::from_i32)
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
                    .and_then(config::MergePolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MERGE_VIRTUALHOST_ROUTER_FILTER" => Ok(config::MergePolicy::MergeVirtualhostRouterFilter),
                    "MERGE_VIRTUALHOST_ROUTER" => Ok(config::MergePolicy::MergeVirtualhostRouter),
                    "OVERRIDE" => Ok(config::MergePolicy::Override),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigsPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.plugins_config.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.golang.v3alpha.ConfigsPerRoute", len)?;
        if !self.plugins_config.is_empty() {
            struct_ser.serialize_field("pluginsConfig", &self.plugins_config)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigsPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "plugins_config",
            "pluginsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PluginsConfig,
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
                            "pluginsConfig" | "plugins_config" => Ok(GeneratedField::PluginsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigsPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.golang.v3alpha.ConfigsPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigsPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut plugins_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PluginsConfig => {
                            if plugins_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pluginsConfig"));
                            }
                            plugins_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ConfigsPerRoute {
                    plugins_config: plugins_config__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.golang.v3alpha.ConfigsPerRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouterPlugin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.golang.v3alpha.RouterPlugin", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                router_plugin::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                router_plugin::Override::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouterPlugin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
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
                            "disabled" => Ok(GeneratedField::Disabled),
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
            type Value = RouterPlugin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.golang.v3alpha.RouterPlugin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouterPlugin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#override__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(router_plugin::Override::Disabled);
                        }
                        GeneratedField::Config => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(router_plugin::Override::Config)
;
                        }
                    }
                }
                Ok(RouterPlugin {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.golang.v3alpha.RouterPlugin", FIELDS, GeneratedVisitor)
    }
}
