// @generated
impl serde::Serialize for CacheConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.typed_config.is_some() {
            len += 1;
        }
        if !self.allowed_vary_headers.is_empty() {
            len += 1;
        }
        if self.key_creator_params.is_some() {
            len += 1;
        }
        if self.max_body_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.cache.v2alpha.CacheConfig", len)?;
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typedConfig", v)?;
        }
        if !self.allowed_vary_headers.is_empty() {
            struct_ser.serialize_field("allowedVaryHeaders", &self.allowed_vary_headers)?;
        }
        if let Some(v) = self.key_creator_params.as_ref() {
            struct_ser.serialize_field("keyCreatorParams", v)?;
        }
        if self.max_body_bytes != 0 {
            struct_ser.serialize_field("maxBodyBytes", &self.max_body_bytes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CacheConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "typed_config",
            "typedConfig",
            "allowed_vary_headers",
            "allowedVaryHeaders",
            "key_creator_params",
            "keyCreatorParams",
            "max_body_bytes",
            "maxBodyBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypedConfig,
            AllowedVaryHeaders,
            KeyCreatorParams,
            MaxBodyBytes,
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
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            "allowedVaryHeaders" | "allowed_vary_headers" => Ok(GeneratedField::AllowedVaryHeaders),
                            "keyCreatorParams" | "key_creator_params" => Ok(GeneratedField::KeyCreatorParams),
                            "maxBodyBytes" | "max_body_bytes" => Ok(GeneratedField::MaxBodyBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CacheConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.cache.v2alpha.CacheConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CacheConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut typed_config__ = None;
                let mut allowed_vary_headers__ = None;
                let mut key_creator_params__ = None;
                let mut max_body_bytes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map.next_value()?;
                        }
                        GeneratedField::AllowedVaryHeaders => {
                            if allowed_vary_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedVaryHeaders"));
                            }
                            allowed_vary_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::KeyCreatorParams => {
                            if key_creator_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyCreatorParams"));
                            }
                            key_creator_params__ = map.next_value()?;
                        }
                        GeneratedField::MaxBodyBytes => {
                            if max_body_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBodyBytes"));
                            }
                            max_body_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CacheConfig {
                    typed_config: typed_config__,
                    allowed_vary_headers: allowed_vary_headers__.unwrap_or_default(),
                    key_creator_params: key_creator_params__,
                    max_body_bytes: max_body_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.cache.v2alpha.CacheConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for cache_config::KeyCreatorParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.exclude_scheme {
            len += 1;
        }
        if self.exclude_host {
            len += 1;
        }
        if !self.query_parameters_included.is_empty() {
            len += 1;
        }
        if !self.query_parameters_excluded.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.cache.v2alpha.CacheConfig.KeyCreatorParams", len)?;
        if self.exclude_scheme {
            struct_ser.serialize_field("excludeScheme", &self.exclude_scheme)?;
        }
        if self.exclude_host {
            struct_ser.serialize_field("excludeHost", &self.exclude_host)?;
        }
        if !self.query_parameters_included.is_empty() {
            struct_ser.serialize_field("queryParametersIncluded", &self.query_parameters_included)?;
        }
        if !self.query_parameters_excluded.is_empty() {
            struct_ser.serialize_field("queryParametersExcluded", &self.query_parameters_excluded)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for cache_config::KeyCreatorParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "exclude_scheme",
            "excludeScheme",
            "exclude_host",
            "excludeHost",
            "query_parameters_included",
            "queryParametersIncluded",
            "query_parameters_excluded",
            "queryParametersExcluded",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExcludeScheme,
            ExcludeHost,
            QueryParametersIncluded,
            QueryParametersExcluded,
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
                            "excludeScheme" | "exclude_scheme" => Ok(GeneratedField::ExcludeScheme),
                            "excludeHost" | "exclude_host" => Ok(GeneratedField::ExcludeHost),
                            "queryParametersIncluded" | "query_parameters_included" => Ok(GeneratedField::QueryParametersIncluded),
                            "queryParametersExcluded" | "query_parameters_excluded" => Ok(GeneratedField::QueryParametersExcluded),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = cache_config::KeyCreatorParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.cache.v2alpha.CacheConfig.KeyCreatorParams")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<cache_config::KeyCreatorParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut exclude_scheme__ = None;
                let mut exclude_host__ = None;
                let mut query_parameters_included__ = None;
                let mut query_parameters_excluded__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExcludeScheme => {
                            if exclude_scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("excludeScheme"));
                            }
                            exclude_scheme__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExcludeHost => {
                            if exclude_host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("excludeHost"));
                            }
                            exclude_host__ = Some(map.next_value()?);
                        }
                        GeneratedField::QueryParametersIncluded => {
                            if query_parameters_included__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParametersIncluded"));
                            }
                            query_parameters_included__ = Some(map.next_value()?);
                        }
                        GeneratedField::QueryParametersExcluded => {
                            if query_parameters_excluded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParametersExcluded"));
                            }
                            query_parameters_excluded__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(cache_config::KeyCreatorParams {
                    exclude_scheme: exclude_scheme__.unwrap_or_default(),
                    exclude_host: exclude_host__.unwrap_or_default(),
                    query_parameters_included: query_parameters_included__.unwrap_or_default(),
                    query_parameters_excluded: query_parameters_excluded__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.cache.v2alpha.CacheConfig.KeyCreatorParams", FIELDS, GeneratedVisitor)
    }
}
