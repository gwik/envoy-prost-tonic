// @generated
impl serde::Serialize for CapabilityRestrictionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_capabilities.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.wasm.v3.CapabilityRestrictionConfig", len)?;
        if !self.allowed_capabilities.is_empty() {
            struct_ser.serialize_field("allowedCapabilities", &self.allowed_capabilities)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CapabilityRestrictionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_capabilities",
            "allowedCapabilities",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedCapabilities,
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
                            "allowedCapabilities" | "allowed_capabilities" => Ok(GeneratedField::AllowedCapabilities),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CapabilityRestrictionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.wasm.v3.CapabilityRestrictionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CapabilityRestrictionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_capabilities__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowedCapabilities => {
                            if allowed_capabilities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedCapabilities"));
                            }
                            allowed_capabilities__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(CapabilityRestrictionConfig {
                    allowed_capabilities: allowed_capabilities__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.wasm.v3.CapabilityRestrictionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnvironmentVariables {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_env_keys.is_empty() {
            len += 1;
        }
        if !self.key_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.wasm.v3.EnvironmentVariables", len)?;
        if !self.host_env_keys.is_empty() {
            struct_ser.serialize_field("hostEnvKeys", &self.host_env_keys)?;
        }
        if !self.key_values.is_empty() {
            struct_ser.serialize_field("keyValues", &self.key_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnvironmentVariables {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_env_keys",
            "hostEnvKeys",
            "key_values",
            "keyValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostEnvKeys,
            KeyValues,
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
                            "hostEnvKeys" | "host_env_keys" => Ok(GeneratedField::HostEnvKeys),
                            "keyValues" | "key_values" => Ok(GeneratedField::KeyValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnvironmentVariables;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.wasm.v3.EnvironmentVariables")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnvironmentVariables, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_env_keys__ = None;
                let mut key_values__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostEnvKeys => {
                            if host_env_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostEnvKeys"));
                            }
                            host_env_keys__ = Some(map.next_value()?);
                        }
                        GeneratedField::KeyValues => {
                            if key_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyValues"));
                            }
                            key_values__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(EnvironmentVariables {
                    host_env_keys: host_env_keys__.unwrap_or_default(),
                    key_values: key_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.wasm.v3.EnvironmentVariables", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PluginConfig {
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
        if !self.root_id.is_empty() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.fail_open {
            len += 1;
        }
        if self.capability_restriction_config.is_some() {
            len += 1;
        }
        if self.vm.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.wasm.v3.PluginConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.root_id.is_empty() {
            struct_ser.serialize_field("rootId", &self.root_id)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if self.fail_open {
            struct_ser.serialize_field("failOpen", &self.fail_open)?;
        }
        if let Some(v) = self.capability_restriction_config.as_ref() {
            struct_ser.serialize_field("capabilityRestrictionConfig", v)?;
        }
        if let Some(v) = self.vm.as_ref() {
            match v {
                plugin_config::Vm::VmConfig(v) => {
                    struct_ser.serialize_field("vmConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PluginConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "root_id",
            "rootId",
            "configuration",
            "fail_open",
            "failOpen",
            "capability_restriction_config",
            "capabilityRestrictionConfig",
            "vm_config",
            "vmConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            RootId,
            Configuration,
            FailOpen,
            CapabilityRestrictionConfig,
            VmConfig,
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
                            "rootId" | "root_id" => Ok(GeneratedField::RootId),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "failOpen" | "fail_open" => Ok(GeneratedField::FailOpen),
                            "capabilityRestrictionConfig" | "capability_restriction_config" => Ok(GeneratedField::CapabilityRestrictionConfig),
                            "vmConfig" | "vm_config" => Ok(GeneratedField::VmConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PluginConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.wasm.v3.PluginConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PluginConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut root_id__ = None;
                let mut configuration__ = None;
                let mut fail_open__ = None;
                let mut capability_restriction_config__ = None;
                let mut vm__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RootId => {
                            if root_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootId"));
                            }
                            root_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map.next_value()?;
                        }
                        GeneratedField::FailOpen => {
                            if fail_open__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failOpen"));
                            }
                            fail_open__ = Some(map.next_value()?);
                        }
                        GeneratedField::CapabilityRestrictionConfig => {
                            if capability_restriction_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capabilityRestrictionConfig"));
                            }
                            capability_restriction_config__ = map.next_value()?;
                        }
                        GeneratedField::VmConfig => {
                            if vm__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmConfig"));
                            }
                            vm__ = map.next_value::<::std::option::Option<_>>()?.map(plugin_config::Vm::VmConfig)
;
                        }
                    }
                }
                Ok(PluginConfig {
                    name: name__.unwrap_or_default(),
                    root_id: root_id__.unwrap_or_default(),
                    configuration: configuration__,
                    fail_open: fail_open__.unwrap_or_default(),
                    capability_restriction_config: capability_restriction_config__,
                    vm: vm__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.wasm.v3.PluginConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SanitizationConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.wasm.v3.SanitizationConfig", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SanitizationConfig {
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
            type Value = SanitizationConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.wasm.v3.SanitizationConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SanitizationConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SanitizationConfig {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.wasm.v3.SanitizationConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VmConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.vm_id.is_empty() {
            len += 1;
        }
        if !self.runtime.is_empty() {
            len += 1;
        }
        if self.code.is_some() {
            len += 1;
        }
        if self.configuration.is_some() {
            len += 1;
        }
        if self.allow_precompiled {
            len += 1;
        }
        if self.nack_on_code_cache_miss {
            len += 1;
        }
        if self.environment_variables.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.wasm.v3.VmConfig", len)?;
        if !self.vm_id.is_empty() {
            struct_ser.serialize_field("vmId", &self.vm_id)?;
        }
        if !self.runtime.is_empty() {
            struct_ser.serialize_field("runtime", &self.runtime)?;
        }
        if let Some(v) = self.code.as_ref() {
            struct_ser.serialize_field("code", v)?;
        }
        if let Some(v) = self.configuration.as_ref() {
            struct_ser.serialize_field("configuration", v)?;
        }
        if self.allow_precompiled {
            struct_ser.serialize_field("allowPrecompiled", &self.allow_precompiled)?;
        }
        if self.nack_on_code_cache_miss {
            struct_ser.serialize_field("nackOnCodeCacheMiss", &self.nack_on_code_cache_miss)?;
        }
        if let Some(v) = self.environment_variables.as_ref() {
            struct_ser.serialize_field("environmentVariables", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VmConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "vm_id",
            "vmId",
            "runtime",
            "code",
            "configuration",
            "allow_precompiled",
            "allowPrecompiled",
            "nack_on_code_cache_miss",
            "nackOnCodeCacheMiss",
            "environment_variables",
            "environmentVariables",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VmId,
            Runtime,
            Code,
            Configuration,
            AllowPrecompiled,
            NackOnCodeCacheMiss,
            EnvironmentVariables,
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
                            "vmId" | "vm_id" => Ok(GeneratedField::VmId),
                            "runtime" => Ok(GeneratedField::Runtime),
                            "code" => Ok(GeneratedField::Code),
                            "configuration" => Ok(GeneratedField::Configuration),
                            "allowPrecompiled" | "allow_precompiled" => Ok(GeneratedField::AllowPrecompiled),
                            "nackOnCodeCacheMiss" | "nack_on_code_cache_miss" => Ok(GeneratedField::NackOnCodeCacheMiss),
                            "environmentVariables" | "environment_variables" => Ok(GeneratedField::EnvironmentVariables),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VmConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.wasm.v3.VmConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<VmConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut vm_id__ = None;
                let mut runtime__ = None;
                let mut code__ = None;
                let mut configuration__ = None;
                let mut allow_precompiled__ = None;
                let mut nack_on_code_cache_miss__ = None;
                let mut environment_variables__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VmId => {
                            if vm_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vmId"));
                            }
                            vm_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Runtime => {
                            if runtime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtime"));
                            }
                            runtime__ = Some(map.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = map.next_value()?;
                        }
                        GeneratedField::Configuration => {
                            if configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configuration"));
                            }
                            configuration__ = map.next_value()?;
                        }
                        GeneratedField::AllowPrecompiled => {
                            if allow_precompiled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowPrecompiled"));
                            }
                            allow_precompiled__ = Some(map.next_value()?);
                        }
                        GeneratedField::NackOnCodeCacheMiss => {
                            if nack_on_code_cache_miss__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nackOnCodeCacheMiss"));
                            }
                            nack_on_code_cache_miss__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnvironmentVariables => {
                            if environment_variables__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environmentVariables"));
                            }
                            environment_variables__ = map.next_value()?;
                        }
                    }
                }
                Ok(VmConfig {
                    vm_id: vm_id__.unwrap_or_default(),
                    runtime: runtime__.unwrap_or_default(),
                    code: code__,
                    configuration: configuration__,
                    allow_precompiled: allow_precompiled__.unwrap_or_default(),
                    nack_on_code_cache_miss: nack_on_code_cache_miss__.unwrap_or_default(),
                    environment_variables: environment_variables__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.wasm.v3.VmConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WasmService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config.is_some() {
            len += 1;
        }
        if self.singleton {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.wasm.v3.WasmService", len)?;
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if self.singleton {
            struct_ser.serialize_field("singleton", &self.singleton)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WasmService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config",
            "singleton",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Config,
            Singleton,
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
                            "singleton" => Ok(GeneratedField::Singleton),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WasmService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.wasm.v3.WasmService")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WasmService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config__ = None;
                let mut singleton__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map.next_value()?;
                        }
                        GeneratedField::Singleton => {
                            if singleton__.is_some() {
                                return Err(serde::de::Error::duplicate_field("singleton"));
                            }
                            singleton__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WasmService {
                    config: config__,
                    singleton: singleton__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.wasm.v3.WasmService", FIELDS, GeneratedVisitor)
    }
}
