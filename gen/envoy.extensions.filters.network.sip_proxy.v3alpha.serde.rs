// @generated
impl serde::Serialize for Cache {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_cache_item != 0 {
            len += 1;
        }
        if self.add_query_to_cache {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.Cache", len)?;
        if self.max_cache_item != 0 {
            struct_ser.serialize_field("maxCacheItem", &self.max_cache_item)?;
        }
        if self.add_query_to_cache {
            struct_ser.serialize_field("addQueryToCache", &self.add_query_to_cache)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cache {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_cache_item",
            "maxCacheItem",
            "add_query_to_cache",
            "addQueryToCache",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxCacheItem,
            AddQueryToCache,
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
                            "maxCacheItem" | "max_cache_item" => Ok(GeneratedField::MaxCacheItem),
                            "addQueryToCache" | "add_query_to_cache" => Ok(GeneratedField::AddQueryToCache),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cache;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.Cache")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Cache, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_cache_item__ = None;
                let mut add_query_to_cache__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxCacheItem => {
                            if max_cache_item__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCacheItem"));
                            }
                            max_cache_item__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AddQueryToCache => {
                            if add_query_to_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addQueryToCache"));
                            }
                            add_query_to_cache__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Cache {
                    max_cache_item: max_cache_item__.unwrap_or_default(),
                    add_query_to_cache: add_query_to_cache__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.Cache", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CustomizedAffinity {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entries.is_empty() {
            len += 1;
        }
        if self.stop_load_balance {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.CustomizedAffinity", len)?;
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        if self.stop_load_balance {
            struct_ser.serialize_field("stopLoadBalance", &self.stop_load_balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomizedAffinity {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entries",
            "stop_load_balance",
            "stopLoadBalance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Entries,
            StopLoadBalance,
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
                            "entries" => Ok(GeneratedField::Entries),
                            "stopLoadBalance" | "stop_load_balance" => Ok(GeneratedField::StopLoadBalance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomizedAffinity;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.CustomizedAffinity")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CustomizedAffinity, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entries__ = None;
                let mut stop_load_balance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::StopLoadBalance => {
                            if stop_load_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stopLoadBalance"));
                            }
                            stop_load_balance__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CustomizedAffinity {
                    entries: entries__.unwrap_or_default(),
                    stop_load_balance: stop_load_balance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.CustomizedAffinity", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CustomizedAffinityEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header.is_empty() {
            len += 1;
        }
        if !self.key_name.is_empty() {
            len += 1;
        }
        if self.subscribe {
            len += 1;
        }
        if self.query {
            len += 1;
        }
        if self.cache.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.CustomizedAffinityEntry", len)?;
        if !self.header.is_empty() {
            struct_ser.serialize_field("header", &self.header)?;
        }
        if !self.key_name.is_empty() {
            struct_ser.serialize_field("keyName", &self.key_name)?;
        }
        if self.subscribe {
            struct_ser.serialize_field("subscribe", &self.subscribe)?;
        }
        if self.query {
            struct_ser.serialize_field("query", &self.query)?;
        }
        if let Some(v) = self.cache.as_ref() {
            struct_ser.serialize_field("cache", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomizedAffinityEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "key_name",
            "keyName",
            "subscribe",
            "query",
            "cache",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            KeyName,
            Subscribe,
            Query,
            Cache,
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
                            "header" => Ok(GeneratedField::Header),
                            "keyName" | "key_name" => Ok(GeneratedField::KeyName),
                            "subscribe" => Ok(GeneratedField::Subscribe),
                            "query" => Ok(GeneratedField::Query),
                            "cache" => Ok(GeneratedField::Cache),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomizedAffinityEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.CustomizedAffinityEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CustomizedAffinityEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut key_name__ = None;
                let mut subscribe__ = None;
                let mut query__ = None;
                let mut cache__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = Some(map.next_value()?);
                        }
                        GeneratedField::KeyName => {
                            if key_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyName"));
                            }
                            key_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Subscribe => {
                            if subscribe__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subscribe"));
                            }
                            subscribe__ = Some(map.next_value()?);
                        }
                        GeneratedField::Query => {
                            if query__.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            query__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cache => {
                            if cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cache"));
                            }
                            cache__ = map.next_value()?;
                        }
                    }
                }
                Ok(CustomizedAffinityEntry {
                    header: header__.unwrap_or_default(),
                    key_name: key_name__.unwrap_or_default(),
                    subscribe: subscribe__.unwrap_or_default(),
                    query: query__.unwrap_or_default(),
                    cache: cache__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.CustomizedAffinityEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.domain.is_empty() {
            len += 1;
        }
        if !self.parameter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.LocalService", len)?;
        if !self.domain.is_empty() {
            struct_ser.serialize_field("domain", &self.domain)?;
        }
        if !self.parameter.is_empty() {
            struct_ser.serialize_field("parameter", &self.parameter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "domain",
            "parameter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Domain,
            Parameter,
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
                            "domain" => Ok(GeneratedField::Domain),
                            "parameter" => Ok(GeneratedField::Parameter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.LocalService")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LocalService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut domain__ = None;
                let mut parameter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Domain => {
                            if domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            domain__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameter => {
                            if parameter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            parameter__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LocalService {
                    domain: domain__.unwrap_or_default(),
                    parameter: parameter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.LocalService", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.Route", len)?;
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
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.Route")
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
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.Route", FIELDS, GeneratedVisitor)
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
        if self.cluster_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.RouteAction", len)?;
        if let Some(v) = self.cluster_specifier.as_ref() {
            match v {
                route_action::ClusterSpecifier::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
            }
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
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
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.RouteAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::Cluster);
                        }
                    }
                }
                Ok(RouteAction {
                    cluster_specifier: cluster_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.RouteAction", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.RouteConfiguration", len)?;
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
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.RouteConfiguration")
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
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.RouteConfiguration", FIELDS, GeneratedVisitor)
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
        if !self.header.is_empty() {
            len += 1;
        }
        if !self.parameter.is_empty() {
            len += 1;
        }
        if self.match_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.RouteMatch", len)?;
        if !self.header.is_empty() {
            struct_ser.serialize_field("header", &self.header)?;
        }
        if !self.parameter.is_empty() {
            struct_ser.serialize_field("parameter", &self.parameter)?;
        }
        if let Some(v) = self.match_specifier.as_ref() {
            match v {
                route_match::MatchSpecifier::Domain(v) => {
                    struct_ser.serialize_field("domain", v)?;
                }
            }
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
            "header",
            "parameter",
            "domain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Parameter,
            Domain,
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
                            "header" => Ok(GeneratedField::Header),
                            "parameter" => Ok(GeneratedField::Parameter),
                            "domain" => Ok(GeneratedField::Domain),
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
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.RouteMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut parameter__ = None;
                let mut match_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameter => {
                            if parameter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameter"));
                            }
                            parameter__ = Some(map.next_value()?);
                        }
                        GeneratedField::Domain => {
                            if match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_match::MatchSpecifier::Domain);
                        }
                    }
                }
                Ok(RouteMatch {
                    header: header__.unwrap_or_default(),
                    parameter: parameter__.unwrap_or_default(),
                    match_specifier: match_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.RouteMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SipFilter {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipFilter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                sip_filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SipFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
            type Value = SipFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.SipFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SipFilter, V::Error>
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
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(sip_filter::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(SipFilter {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SipProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.session_affinity {
            len += 1;
        }
        if self.registration_affinity {
            len += 1;
        }
        if self.customized_affinity.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipProtocolOptions", len)?;
        if self.session_affinity {
            struct_ser.serialize_field("sessionAffinity", &self.session_affinity)?;
        }
        if self.registration_affinity {
            struct_ser.serialize_field("registrationAffinity", &self.registration_affinity)?;
        }
        if let Some(v) = self.customized_affinity.as_ref() {
            struct_ser.serialize_field("customizedAffinity", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SipProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_affinity",
            "sessionAffinity",
            "registration_affinity",
            "registrationAffinity",
            "customized_affinity",
            "customizedAffinity",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionAffinity,
            RegistrationAffinity,
            CustomizedAffinity,
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
                            "sessionAffinity" | "session_affinity" => Ok(GeneratedField::SessionAffinity),
                            "registrationAffinity" | "registration_affinity" => Ok(GeneratedField::RegistrationAffinity),
                            "customizedAffinity" | "customized_affinity" => Ok(GeneratedField::CustomizedAffinity),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SipProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.SipProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SipProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_affinity__ = None;
                let mut registration_affinity__ = None;
                let mut customized_affinity__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SessionAffinity => {
                            if session_affinity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionAffinity"));
                            }
                            session_affinity__ = Some(map.next_value()?);
                        }
                        GeneratedField::RegistrationAffinity => {
                            if registration_affinity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registrationAffinity"));
                            }
                            registration_affinity__ = Some(map.next_value()?);
                        }
                        GeneratedField::CustomizedAffinity => {
                            if customized_affinity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customizedAffinity"));
                            }
                            customized_affinity__ = map.next_value()?;
                        }
                    }
                }
                Ok(SipProtocolOptions {
                    session_affinity: session_affinity__.unwrap_or_default(),
                    registration_affinity: registration_affinity__.unwrap_or_default(),
                    customized_affinity: customized_affinity__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SipProxy {
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
        if !self.sip_filters.is_empty() {
            len += 1;
        }
        if self.settings.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.route_config.as_ref() {
            struct_ser.serialize_field("routeConfig", v)?;
        }
        if !self.sip_filters.is_empty() {
            struct_ser.serialize_field("sipFilters", &self.sip_filters)?;
        }
        if let Some(v) = self.settings.as_ref() {
            struct_ser.serialize_field("settings", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SipProxy {
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
            "sip_filters",
            "sipFilters",
            "settings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            RouteConfig,
            SipFilters,
            Settings,
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
                            "sipFilters" | "sip_filters" => Ok(GeneratedField::SipFilters),
                            "settings" => Ok(GeneratedField::Settings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SipProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.SipProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SipProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut route_config__ = None;
                let mut sip_filters__ = None;
                let mut settings__ = None;
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
                        GeneratedField::SipFilters => {
                            if sip_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sipFilters"));
                            }
                            sip_filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Settings => {
                            if settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            settings__ = map.next_value()?;
                        }
                    }
                }
                Ok(SipProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    route_config: route_config__,
                    sip_filters: sip_filters__.unwrap_or_default(),
                    settings: settings__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for sip_proxy::SipSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transaction_timeout.is_some() {
            len += 1;
        }
        if !self.local_services.is_empty() {
            len += 1;
        }
        if self.tra_service_config.is_some() {
            len += 1;
        }
        if self.operate_via {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipProxy.SipSettings", len)?;
        if let Some(v) = self.transaction_timeout.as_ref() {
            struct_ser.serialize_field("transactionTimeout", v)?;
        }
        if !self.local_services.is_empty() {
            struct_ser.serialize_field("localServices", &self.local_services)?;
        }
        if let Some(v) = self.tra_service_config.as_ref() {
            struct_ser.serialize_field("traServiceConfig", v)?;
        }
        if self.operate_via {
            struct_ser.serialize_field("operateVia", &self.operate_via)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for sip_proxy::SipSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_timeout",
            "transactionTimeout",
            "local_services",
            "localServices",
            "tra_service_config",
            "traServiceConfig",
            "operate_via",
            "operateVia",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionTimeout,
            LocalServices,
            TraServiceConfig,
            OperateVia,
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
                            "transactionTimeout" | "transaction_timeout" => Ok(GeneratedField::TransactionTimeout),
                            "localServices" | "local_services" => Ok(GeneratedField::LocalServices),
                            "traServiceConfig" | "tra_service_config" => Ok(GeneratedField::TraServiceConfig),
                            "operateVia" | "operate_via" => Ok(GeneratedField::OperateVia),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = sip_proxy::SipSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.sip_proxy.v3alpha.SipProxy.SipSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<sip_proxy::SipSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction_timeout__ = None;
                let mut local_services__ = None;
                let mut tra_service_config__ = None;
                let mut operate_via__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TransactionTimeout => {
                            if transaction_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionTimeout"));
                            }
                            transaction_timeout__ = map.next_value()?;
                        }
                        GeneratedField::LocalServices => {
                            if local_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localServices"));
                            }
                            local_services__ = Some(map.next_value()?);
                        }
                        GeneratedField::TraServiceConfig => {
                            if tra_service_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traServiceConfig"));
                            }
                            tra_service_config__ = map.next_value()?;
                        }
                        GeneratedField::OperateVia => {
                            if operate_via__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operateVia"));
                            }
                            operate_via__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(sip_proxy::SipSettings {
                    transaction_timeout: transaction_timeout__,
                    local_services: local_services__.unwrap_or_default(),
                    tra_service_config: tra_service_config__,
                    operate_via: operate_via__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.sip_proxy.v3alpha.SipProxy.SipSettings", FIELDS, GeneratedVisitor)
    }
}
