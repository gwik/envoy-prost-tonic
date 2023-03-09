// @generated
impl serde::Serialize for GenericProxy {
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
        if self.codec_config.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.tracing.is_some() {
            len += 1;
        }
        if self.route_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.generic_proxy.v3.GenericProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.codec_config.as_ref() {
            struct_ser.serialize_field("codecConfig", v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if let Some(v) = self.tracing.as_ref() {
            struct_ser.serialize_field("tracing", v)?;
        }
        if let Some(v) = self.route_specifier.as_ref() {
            match v {
                generic_proxy::RouteSpecifier::GenericRds(v) => {
                    struct_ser.serialize_field("genericRds", v)?;
                }
                generic_proxy::RouteSpecifier::RouteConfig(v) => {
                    struct_ser.serialize_field("routeConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenericProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "codec_config",
            "codecConfig",
            "filters",
            "tracing",
            "generic_rds",
            "genericRds",
            "route_config",
            "routeConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            CodecConfig,
            Filters,
            Tracing,
            GenericRds,
            RouteConfig,
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
                            "codecConfig" | "codec_config" => Ok(GeneratedField::CodecConfig),
                            "filters" => Ok(GeneratedField::Filters),
                            "tracing" => Ok(GeneratedField::Tracing),
                            "genericRds" | "generic_rds" => Ok(GeneratedField::GenericRds),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenericProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.generic_proxy.v3.GenericProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenericProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut codec_config__ = None;
                let mut filters__ = None;
                let mut tracing__ = None;
                let mut route_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::CodecConfig => {
                            if codec_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codecConfig"));
                            }
                            codec_config__ = map.next_value()?;
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Tracing => {
                            if tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracing"));
                            }
                            tracing__ = map.next_value()?;
                        }
                        GeneratedField::GenericRds => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericRds"));
                            }
                            route_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(generic_proxy::RouteSpecifier::GenericRds)
;
                        }
                        GeneratedField::RouteConfig => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(generic_proxy::RouteSpecifier::RouteConfig)
;
                        }
                    }
                }
                Ok(GenericProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    codec_config: codec_config__,
                    filters: filters__.unwrap_or_default(),
                    tracing: tracing__,
                    route_specifier: route_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.generic_proxy.v3.GenericProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenericRds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_source.is_some() {
            len += 1;
        }
        if !self.route_config_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.generic_proxy.v3.GenericRds", len)?;
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("configSource", v)?;
        }
        if !self.route_config_name.is_empty() {
            struct_ser.serialize_field("routeConfigName", &self.route_config_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenericRds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_source",
            "configSource",
            "route_config_name",
            "routeConfigName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigSource,
            RouteConfigName,
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
                            "configSource" | "config_source" => Ok(GeneratedField::ConfigSource),
                            "routeConfigName" | "route_config_name" => Ok(GeneratedField::RouteConfigName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenericRds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.generic_proxy.v3.GenericRds")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenericRds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source__ = None;
                let mut route_config_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfigSource => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSource"));
                            }
                            config_source__ = map.next_value()?;
                        }
                        GeneratedField::RouteConfigName => {
                            if route_config_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfigName"));
                            }
                            route_config_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenericRds {
                    config_source: config_source__,
                    route_config_name: route_config_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.generic_proxy.v3.GenericRds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteConfiguration {
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
        if self.routes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.generic_proxy.v3.RouteConfiguration", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.routes.as_ref() {
            struct_ser.serialize_field("routes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "routes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Routes,
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
                            "routes" => Ok(GeneratedField::Routes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.generic_proxy.v3.RouteConfiguration")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut routes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Routes => {
                            if routes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routes"));
                            }
                            routes__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteConfiguration {
                    name: name__.unwrap_or_default(),
                    routes: routes__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.generic_proxy.v3.RouteConfiguration", FIELDS, GeneratedVisitor)
    }
}
