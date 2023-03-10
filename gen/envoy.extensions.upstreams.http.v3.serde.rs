// @generated
impl serde::Serialize for HttpProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_http_protocol_options.is_some() {
            len += 1;
        }
        if self.upstream_http_protocol_options.is_some() {
            len += 1;
        }
        if !self.http_filters.is_empty() {
            len += 1;
        }
        if self.header_validation_config.is_some() {
            len += 1;
        }
        if self.upstream_protocol_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions", len)?;
        if let Some(v) = self.common_http_protocol_options.as_ref() {
            struct_ser.serialize_field("commonHttpProtocolOptions", v)?;
        }
        if let Some(v) = self.upstream_http_protocol_options.as_ref() {
            struct_ser.serialize_field("upstreamHttpProtocolOptions", v)?;
        }
        if !self.http_filters.is_empty() {
            struct_ser.serialize_field("httpFilters", &self.http_filters)?;
        }
        if let Some(v) = self.header_validation_config.as_ref() {
            struct_ser.serialize_field("headerValidationConfig", v)?;
        }
        if let Some(v) = self.upstream_protocol_options.as_ref() {
            match v {
                http_protocol_options::UpstreamProtocolOptions::ExplicitHttpConfig(v) => {
                    struct_ser.serialize_field("explicitHttpConfig", v)?;
                }
                http_protocol_options::UpstreamProtocolOptions::UseDownstreamProtocolConfig(v) => {
                    struct_ser.serialize_field("useDownstreamProtocolConfig", v)?;
                }
                http_protocol_options::UpstreamProtocolOptions::AutoConfig(v) => {
                    struct_ser.serialize_field("autoConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_http_protocol_options",
            "commonHttpProtocolOptions",
            "upstream_http_protocol_options",
            "upstreamHttpProtocolOptions",
            "http_filters",
            "httpFilters",
            "header_validation_config",
            "headerValidationConfig",
            "explicit_http_config",
            "explicitHttpConfig",
            "use_downstream_protocol_config",
            "useDownstreamProtocolConfig",
            "auto_config",
            "autoConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonHttpProtocolOptions,
            UpstreamHttpProtocolOptions,
            HttpFilters,
            HeaderValidationConfig,
            ExplicitHttpConfig,
            UseDownstreamProtocolConfig,
            AutoConfig,
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
                            "commonHttpProtocolOptions" | "common_http_protocol_options" => Ok(GeneratedField::CommonHttpProtocolOptions),
                            "upstreamHttpProtocolOptions" | "upstream_http_protocol_options" => Ok(GeneratedField::UpstreamHttpProtocolOptions),
                            "httpFilters" | "http_filters" => Ok(GeneratedField::HttpFilters),
                            "headerValidationConfig" | "header_validation_config" => Ok(GeneratedField::HeaderValidationConfig),
                            "explicitHttpConfig" | "explicit_http_config" => Ok(GeneratedField::ExplicitHttpConfig),
                            "useDownstreamProtocolConfig" | "use_downstream_protocol_config" => Ok(GeneratedField::UseDownstreamProtocolConfig),
                            "autoConfig" | "auto_config" => Ok(GeneratedField::AutoConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.upstreams.http.v3.HttpProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_http_protocol_options__ = None;
                let mut upstream_http_protocol_options__ = None;
                let mut http_filters__ = None;
                let mut header_validation_config__ = None;
                let mut upstream_protocol_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonHttpProtocolOptions => {
                            if common_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonHttpProtocolOptions"));
                            }
                            common_http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamHttpProtocolOptions => {
                            if upstream_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamHttpProtocolOptions"));
                            }
                            upstream_http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::HttpFilters => {
                            if http_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpFilters"));
                            }
                            http_filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeaderValidationConfig => {
                            if header_validation_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerValidationConfig"));
                            }
                            header_validation_config__ = map.next_value()?;
                        }
                        GeneratedField::ExplicitHttpConfig => {
                            if upstream_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("explicitHttpConfig"));
                            }
                            upstream_protocol_options__ = map.next_value::<::std::option::Option<_>>()?.map(http_protocol_options::UpstreamProtocolOptions::ExplicitHttpConfig)
;
                        }
                        GeneratedField::UseDownstreamProtocolConfig => {
                            if upstream_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useDownstreamProtocolConfig"));
                            }
                            upstream_protocol_options__ = map.next_value::<::std::option::Option<_>>()?.map(http_protocol_options::UpstreamProtocolOptions::UseDownstreamProtocolConfig)
;
                        }
                        GeneratedField::AutoConfig => {
                            if upstream_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoConfig"));
                            }
                            upstream_protocol_options__ = map.next_value::<::std::option::Option<_>>()?.map(http_protocol_options::UpstreamProtocolOptions::AutoConfig)
;
                        }
                    }
                }
                Ok(HttpProtocolOptions {
                    common_http_protocol_options: common_http_protocol_options__,
                    upstream_http_protocol_options: upstream_http_protocol_options__,
                    http_filters: http_filters__.unwrap_or_default(),
                    header_validation_config: header_validation_config__,
                    upstream_protocol_options: upstream_protocol_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_protocol_options::AutoHttpConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_protocol_options.is_some() {
            len += 1;
        }
        if self.http2_protocol_options.is_some() {
            len += 1;
        }
        if self.http3_protocol_options.is_some() {
            len += 1;
        }
        if self.alternate_protocols_cache_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions.AutoHttpConfig", len)?;
        if let Some(v) = self.http_protocol_options.as_ref() {
            struct_ser.serialize_field("httpProtocolOptions", v)?;
        }
        if let Some(v) = self.http2_protocol_options.as_ref() {
            struct_ser.serialize_field("http2ProtocolOptions", v)?;
        }
        if let Some(v) = self.http3_protocol_options.as_ref() {
            struct_ser.serialize_field("http3ProtocolOptions", v)?;
        }
        if let Some(v) = self.alternate_protocols_cache_options.as_ref() {
            struct_ser.serialize_field("alternateProtocolsCacheOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_protocol_options::AutoHttpConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_protocol_options",
            "httpProtocolOptions",
            "http2_protocol_options",
            "http2ProtocolOptions",
            "http3_protocol_options",
            "http3ProtocolOptions",
            "alternate_protocols_cache_options",
            "alternateProtocolsCacheOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpProtocolOptions,
            Http2ProtocolOptions,
            Http3ProtocolOptions,
            AlternateProtocolsCacheOptions,
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
                            "httpProtocolOptions" | "http_protocol_options" => Ok(GeneratedField::HttpProtocolOptions),
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            "http3ProtocolOptions" | "http3_protocol_options" => Ok(GeneratedField::Http3ProtocolOptions),
                            "alternateProtocolsCacheOptions" | "alternate_protocols_cache_options" => Ok(GeneratedField::AlternateProtocolsCacheOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_protocol_options::AutoHttpConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.upstreams.http.v3.HttpProtocolOptions.AutoHttpConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http_protocol_options::AutoHttpConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_protocol_options__ = None;
                let mut http2_protocol_options__ = None;
                let mut http3_protocol_options__ = None;
                let mut alternate_protocols_cache_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpProtocolOptions => {
                            if http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpProtocolOptions"));
                            }
                            http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::Http2ProtocolOptions => {
                            if http2_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2ProtocolOptions"));
                            }
                            http2_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::Http3ProtocolOptions => {
                            if http3_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http3ProtocolOptions"));
                            }
                            http3_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::AlternateProtocolsCacheOptions => {
                            if alternate_protocols_cache_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alternateProtocolsCacheOptions"));
                            }
                            alternate_protocols_cache_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(http_protocol_options::AutoHttpConfig {
                    http_protocol_options: http_protocol_options__,
                    http2_protocol_options: http2_protocol_options__,
                    http3_protocol_options: http3_protocol_options__,
                    alternate_protocols_cache_options: alternate_protocols_cache_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions.AutoHttpConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_protocol_options::ExplicitHttpConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions.ExplicitHttpConfig", len)?;
        if let Some(v) = self.protocol_config.as_ref() {
            match v {
                http_protocol_options::explicit_http_config::ProtocolConfig::HttpProtocolOptions(v) => {
                    struct_ser.serialize_field("httpProtocolOptions", v)?;
                }
                http_protocol_options::explicit_http_config::ProtocolConfig::Http2ProtocolOptions(v) => {
                    struct_ser.serialize_field("http2ProtocolOptions", v)?;
                }
                http_protocol_options::explicit_http_config::ProtocolConfig::Http3ProtocolOptions(v) => {
                    struct_ser.serialize_field("http3ProtocolOptions", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_protocol_options::ExplicitHttpConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_protocol_options",
            "httpProtocolOptions",
            "http2_protocol_options",
            "http2ProtocolOptions",
            "http3_protocol_options",
            "http3ProtocolOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpProtocolOptions,
            Http2ProtocolOptions,
            Http3ProtocolOptions,
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
                            "httpProtocolOptions" | "http_protocol_options" => Ok(GeneratedField::HttpProtocolOptions),
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            "http3ProtocolOptions" | "http3_protocol_options" => Ok(GeneratedField::Http3ProtocolOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_protocol_options::ExplicitHttpConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.upstreams.http.v3.HttpProtocolOptions.ExplicitHttpConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http_protocol_options::ExplicitHttpConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpProtocolOptions => {
                            if protocol_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpProtocolOptions"));
                            }
                            protocol_config__ = map.next_value::<::std::option::Option<_>>()?.map(http_protocol_options::explicit_http_config::ProtocolConfig::HttpProtocolOptions)
;
                        }
                        GeneratedField::Http2ProtocolOptions => {
                            if protocol_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2ProtocolOptions"));
                            }
                            protocol_config__ = map.next_value::<::std::option::Option<_>>()?.map(http_protocol_options::explicit_http_config::ProtocolConfig::Http2ProtocolOptions)
;
                        }
                        GeneratedField::Http3ProtocolOptions => {
                            if protocol_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http3ProtocolOptions"));
                            }
                            protocol_config__ = map.next_value::<::std::option::Option<_>>()?.map(http_protocol_options::explicit_http_config::ProtocolConfig::Http3ProtocolOptions)
;
                        }
                    }
                }
                Ok(http_protocol_options::ExplicitHttpConfig {
                    protocol_config: protocol_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions.ExplicitHttpConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_protocol_options::UseDownstreamHttpConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_protocol_options.is_some() {
            len += 1;
        }
        if self.http2_protocol_options.is_some() {
            len += 1;
        }
        if self.http3_protocol_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions.UseDownstreamHttpConfig", len)?;
        if let Some(v) = self.http_protocol_options.as_ref() {
            struct_ser.serialize_field("httpProtocolOptions", v)?;
        }
        if let Some(v) = self.http2_protocol_options.as_ref() {
            struct_ser.serialize_field("http2ProtocolOptions", v)?;
        }
        if let Some(v) = self.http3_protocol_options.as_ref() {
            struct_ser.serialize_field("http3ProtocolOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_protocol_options::UseDownstreamHttpConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_protocol_options",
            "httpProtocolOptions",
            "http2_protocol_options",
            "http2ProtocolOptions",
            "http3_protocol_options",
            "http3ProtocolOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpProtocolOptions,
            Http2ProtocolOptions,
            Http3ProtocolOptions,
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
                            "httpProtocolOptions" | "http_protocol_options" => Ok(GeneratedField::HttpProtocolOptions),
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            "http3ProtocolOptions" | "http3_protocol_options" => Ok(GeneratedField::Http3ProtocolOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_protocol_options::UseDownstreamHttpConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.upstreams.http.v3.HttpProtocolOptions.UseDownstreamHttpConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http_protocol_options::UseDownstreamHttpConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_protocol_options__ = None;
                let mut http2_protocol_options__ = None;
                let mut http3_protocol_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpProtocolOptions => {
                            if http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpProtocolOptions"));
                            }
                            http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::Http2ProtocolOptions => {
                            if http2_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2ProtocolOptions"));
                            }
                            http2_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::Http3ProtocolOptions => {
                            if http3_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http3ProtocolOptions"));
                            }
                            http3_protocol_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(http_protocol_options::UseDownstreamHttpConfig {
                    http_protocol_options: http_protocol_options__,
                    http2_protocol_options: http2_protocol_options__,
                    http3_protocol_options: http3_protocol_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.upstreams.http.v3.HttpProtocolOptions.UseDownstreamHttpConfig", FIELDS, GeneratedVisitor)
    }
}
