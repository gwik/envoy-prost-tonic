// @generated
impl serde::Serialize for RocketmqProxy {
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
        if self.route_config.is_some() {
            len += 1;
        }
        if self.transient_object_life_span.is_some() {
            len += 1;
        }
        if self.develop_mode {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RocketmqProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.route_config.as_ref() {
            struct_ser.serialize_field("routeConfig", v)?;
        }
        if let Some(v) = self.transient_object_life_span.as_ref() {
            struct_ser.serialize_field("transientObjectLifeSpan", v)?;
        }
        if self.develop_mode {
            struct_ser.serialize_field("developMode", &self.develop_mode)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RocketmqProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "route_config",
            "routeConfig",
            "transient_object_life_span",
            "transientObjectLifeSpan",
            "develop_mode",
            "developMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            RouteConfig,
            TransientObjectLifeSpan,
            DevelopMode,
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
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "transientObjectLifeSpan" | "transient_object_life_span" => Ok(GeneratedField::TransientObjectLifeSpan),
                            "developMode" | "develop_mode" => Ok(GeneratedField::DevelopMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RocketmqProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.rocketmq_proxy.v3.RocketmqProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RocketmqProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut route_config__ = None;
                let mut transient_object_life_span__ = None;
                let mut develop_mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::RouteConfig => {
                            if route_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_config__ = map.next_value()?;
                        }
                        GeneratedField::TransientObjectLifeSpan => {
                            if transient_object_life_span__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transientObjectLifeSpan"));
                            }
                            transient_object_life_span__ = map.next_value()?;
                        }
                        GeneratedField::DevelopMode => {
                            if develop_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("developMode"));
                            }
                            develop_mode__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RocketmqProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    route_config: route_config__,
                    transient_object_life_span: transient_object_life_span__,
                    develop_mode: develop_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RocketmqProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Route {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#match.is_some() {
            len += 1;
        }
        if self.route.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.Route", len)?;
        if let Some(v) = self.r#match.as_ref() {
            struct_ser.serialize_field("match", v)?;
        }
        if let Some(v) = self.route.as_ref() {
            struct_ser.serialize_field("route", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Route {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "match",
            "route",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Match,
            Route,
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
                            "match" => Ok(GeneratedField::Match),
                            "route" => Ok(GeneratedField::Route),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Route;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.rocketmq_proxy.v3.Route")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Route, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#match__ = None;
                let mut route__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Match => {
                            if r#match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("match"));
                            }
                            r#match__ = map.next_value()?;
                        }
                        GeneratedField::Route => {
                            if route__.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            route__ = map.next_value()?;
                        }
                    }
                }
                Ok(Route {
                    r#match: r#match__,
                    route: route__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.Route", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster.is_empty() {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RouteAction", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "metadata_match",
            "metadataMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            MetadataMatch,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.rocketmq_proxy.v3.RouteAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut metadata_match__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteAction {
                    cluster: cluster__.unwrap_or_default(),
                    metadata_match: metadata_match__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RouteAction", FIELDS, GeneratedVisitor)
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
        if !self.routes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RouteConfiguration", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
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
                formatter.write_str("struct envoy.extensions.filters.network.rocketmq_proxy.v3.RouteConfiguration")
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
                            routes__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RouteConfiguration {
                    name: name__.unwrap_or_default(),
                    routes: routes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RouteConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RouteMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.topic.is_some() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RouteMatch", len)?;
        if let Some(v) = self.topic.as_ref() {
            struct_ser.serialize_field("topic", v)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RouteMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "topic",
            "headers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Topic,
            Headers,
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
                            "topic" => Ok(GeneratedField::Topic),
                            "headers" => Ok(GeneratedField::Headers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RouteMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.rocketmq_proxy.v3.RouteMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut topic__ = None;
                let mut headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Topic => {
                            if topic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("topic"));
                            }
                            topic__ = map.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RouteMatch {
                    topic: topic__,
                    headers: headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.rocketmq_proxy.v3.RouteMatch", FIELDS, GeneratedVisitor)
    }
}
