// @generated
impl serde::Serialize for AdminConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.config_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.common.tap.v2alpha.AdminConfig", len)?;
        if !self.config_id.is_empty() {
            struct_ser.serialize_field("configId", &self.config_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdminConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_id",
            "configId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigId,
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
                            "configId" | "config_id" => Ok(GeneratedField::ConfigId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AdminConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.common.tap.v2alpha.AdminConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AdminConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfigId => {
                            if config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configId"));
                            }
                            config_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AdminConfig {
                    config_id: config_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.common.tap.v2alpha.AdminConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommonExtensionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.common.tap.v2alpha.CommonExtensionConfig", len)?;
        if let Some(v) = self.config_type.as_ref() {
            match v {
                common_extension_config::ConfigType::AdminConfig(v) => {
                    struct_ser.serialize_field("adminConfig", v)?;
                }
                common_extension_config::ConfigType::StaticConfig(v) => {
                    struct_ser.serialize_field("staticConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonExtensionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin_config",
            "adminConfig",
            "static_config",
            "staticConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AdminConfig,
            StaticConfig,
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
                            "adminConfig" | "admin_config" => Ok(GeneratedField::AdminConfig),
                            "staticConfig" | "static_config" => Ok(GeneratedField::StaticConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonExtensionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.common.tap.v2alpha.CommonExtensionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommonExtensionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AdminConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(common_extension_config::ConfigType::AdminConfig)
;
                        }
                        GeneratedField::StaticConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(common_extension_config::ConfigType::StaticConfig)
;
                        }
                    }
                }
                Ok(CommonExtensionConfig {
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.common.tap.v2alpha.CommonExtensionConfig", FIELDS, GeneratedVisitor)
    }
}
