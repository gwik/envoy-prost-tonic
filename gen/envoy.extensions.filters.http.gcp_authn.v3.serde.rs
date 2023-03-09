// @generated
impl serde::Serialize for Audience {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.gcp_authn.v3.Audience", len)?;
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Audience {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "url",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Url,
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
                            "url" => Ok(GeneratedField::Url),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Audience;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.gcp_authn.v3.Audience")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Audience, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut url__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Audience {
                    url: url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.gcp_authn.v3.Audience", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GcpAuthnFilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_uri.is_some() {
            len += 1;
        }
        if self.retry_policy.is_some() {
            len += 1;
        }
        if self.cache_config.is_some() {
            len += 1;
        }
        if self.token_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.gcp_authn.v3.GcpAuthnFilterConfig", len)?;
        if let Some(v) = self.http_uri.as_ref() {
            struct_ser.serialize_field("httpUri", v)?;
        }
        if let Some(v) = self.retry_policy.as_ref() {
            struct_ser.serialize_field("retryPolicy", v)?;
        }
        if let Some(v) = self.cache_config.as_ref() {
            struct_ser.serialize_field("cacheConfig", v)?;
        }
        if let Some(v) = self.token_header.as_ref() {
            struct_ser.serialize_field("tokenHeader", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GcpAuthnFilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_uri",
            "httpUri",
            "retry_policy",
            "retryPolicy",
            "cache_config",
            "cacheConfig",
            "token_header",
            "tokenHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpUri,
            RetryPolicy,
            CacheConfig,
            TokenHeader,
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
                            "httpUri" | "http_uri" => Ok(GeneratedField::HttpUri),
                            "retryPolicy" | "retry_policy" => Ok(GeneratedField::RetryPolicy),
                            "cacheConfig" | "cache_config" => Ok(GeneratedField::CacheConfig),
                            "tokenHeader" | "token_header" => Ok(GeneratedField::TokenHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GcpAuthnFilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.gcp_authn.v3.GcpAuthnFilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GcpAuthnFilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_uri__ = None;
                let mut retry_policy__ = None;
                let mut cache_config__ = None;
                let mut token_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpUri => {
                            if http_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpUri"));
                            }
                            http_uri__ = map.next_value()?;
                        }
                        GeneratedField::RetryPolicy => {
                            if retry_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicy"));
                            }
                            retry_policy__ = map.next_value()?;
                        }
                        GeneratedField::CacheConfig => {
                            if cache_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheConfig"));
                            }
                            cache_config__ = map.next_value()?;
                        }
                        GeneratedField::TokenHeader => {
                            if token_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenHeader"));
                            }
                            token_header__ = map.next_value()?;
                        }
                    }
                }
                Ok(GcpAuthnFilterConfig {
                    http_uri: http_uri__,
                    retry_policy: retry_policy__,
                    cache_config: cache_config__,
                    token_header: token_header__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.gcp_authn.v3.GcpAuthnFilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TokenCacheConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cache_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.gcp_authn.v3.TokenCacheConfig", len)?;
        if let Some(v) = self.cache_size.as_ref() {
            struct_ser.serialize_field("cacheSize", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenCacheConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cache_size",
            "cacheSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CacheSize,
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
                            "cacheSize" | "cache_size" => Ok(GeneratedField::CacheSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenCacheConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.gcp_authn.v3.TokenCacheConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TokenCacheConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cache_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CacheSize => {
                            if cache_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheSize"));
                            }
                            cache_size__ = map.next_value()?;
                        }
                    }
                }
                Ok(TokenCacheConfig {
                    cache_size: cache_size__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.gcp_authn.v3.TokenCacheConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TokenHeader {
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
        if !self.value_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.gcp_authn.v3.TokenHeader", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value_prefix.is_empty() {
            struct_ser.serialize_field("valuePrefix", &self.value_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value_prefix",
            "valuePrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ValuePrefix,
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
                            "valuePrefix" | "value_prefix" => Ok(GeneratedField::ValuePrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.gcp_authn.v3.TokenHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TokenHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValuePrefix => {
                            if value_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePrefix"));
                            }
                            value_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TokenHeader {
                    name: name__.unwrap_or_default(),
                    value_prefix: value_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.gcp_authn.v3.TokenHeader", FIELDS, GeneratedVisitor)
    }
}
