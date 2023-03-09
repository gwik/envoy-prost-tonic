// @generated
impl serde::Serialize for OverloadAction {
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
        if !self.triggers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v2alpha.OverloadAction", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.triggers.is_empty() {
            struct_ser.serialize_field("triggers", &self.triggers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverloadAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "triggers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Triggers,
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
                            "triggers" => Ok(GeneratedField::Triggers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverloadAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v2alpha.OverloadAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OverloadAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut triggers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Triggers => {
                            if triggers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triggers"));
                            }
                            triggers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OverloadAction {
                    name: name__.unwrap_or_default(),
                    triggers: triggers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v2alpha.OverloadAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OverloadManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.refresh_interval.is_some() {
            len += 1;
        }
        if !self.resource_monitors.is_empty() {
            len += 1;
        }
        if !self.actions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v2alpha.OverloadManager", len)?;
        if let Some(v) = self.refresh_interval.as_ref() {
            struct_ser.serialize_field("refreshInterval", v)?;
        }
        if !self.resource_monitors.is_empty() {
            struct_ser.serialize_field("resourceMonitors", &self.resource_monitors)?;
        }
        if !self.actions.is_empty() {
            struct_ser.serialize_field("actions", &self.actions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OverloadManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "refresh_interval",
            "refreshInterval",
            "resource_monitors",
            "resourceMonitors",
            "actions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RefreshInterval,
            ResourceMonitors,
            Actions,
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
                            "refreshInterval" | "refresh_interval" => Ok(GeneratedField::RefreshInterval),
                            "resourceMonitors" | "resource_monitors" => Ok(GeneratedField::ResourceMonitors),
                            "actions" => Ok(GeneratedField::Actions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OverloadManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v2alpha.OverloadManager")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OverloadManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut refresh_interval__ = None;
                let mut resource_monitors__ = None;
                let mut actions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RefreshInterval => {
                            if refresh_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshInterval"));
                            }
                            refresh_interval__ = map.next_value()?;
                        }
                        GeneratedField::ResourceMonitors => {
                            if resource_monitors__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceMonitors"));
                            }
                            resource_monitors__ = Some(map.next_value()?);
                        }
                        GeneratedField::Actions => {
                            if actions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actions"));
                            }
                            actions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OverloadManager {
                    refresh_interval: refresh_interval__,
                    resource_monitors: resource_monitors__.unwrap_or_default(),
                    actions: actions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v2alpha.OverloadManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceMonitor {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v2alpha.ResourceMonitor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                resource_monitor::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                resource_monitor::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceMonitor {
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
            type Value = ResourceMonitor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v2alpha.ResourceMonitor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResourceMonitor, V::Error>
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
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(resource_monitor::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(resource_monitor::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(ResourceMonitor {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v2alpha.ResourceMonitor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ThresholdTrigger {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v2alpha.ThresholdTrigger", len)?;
        if self.value != 0. {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ThresholdTrigger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = ThresholdTrigger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v2alpha.ThresholdTrigger")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ThresholdTrigger, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ThresholdTrigger {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v2alpha.ThresholdTrigger", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Trigger {
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
        if self.trigger_oneof.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.overload.v2alpha.Trigger", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.trigger_oneof.as_ref() {
            match v {
                trigger::TriggerOneof::Threshold(v) => {
                    struct_ser.serialize_field("threshold", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Trigger {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "threshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Threshold,
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
                            "threshold" => Ok(GeneratedField::Threshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Trigger;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.overload.v2alpha.Trigger")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Trigger, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut trigger_oneof__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Threshold => {
                            if trigger_oneof__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            trigger_oneof__ = map.next_value::<::std::option::Option<_>>()?.map(trigger::TriggerOneof::Threshold)
;
                        }
                    }
                }
                Ok(Trigger {
                    name: name__.unwrap_or_default(),
                    trigger_oneof: trigger_oneof__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.overload.v2alpha.Trigger", FIELDS, GeneratedVisitor)
    }
}
