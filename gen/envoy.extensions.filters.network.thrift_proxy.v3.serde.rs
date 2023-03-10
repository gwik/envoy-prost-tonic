// @generated
impl serde::Serialize for ProtocolType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AutoProtocol => "AUTO_PROTOCOL",
            Self::Binary => "BINARY",
            Self::LaxBinary => "LAX_BINARY",
            Self::Compact => "COMPACT",
            Self::Twitter => "TWITTER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProtocolType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO_PROTOCOL",
            "BINARY",
            "LAX_BINARY",
            "COMPACT",
            "TWITTER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProtocolType;

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
                    .and_then(ProtocolType::from_i32)
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
                    .and_then(ProtocolType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AUTO_PROTOCOL" => Ok(ProtocolType::AutoProtocol),
                    "BINARY" => Ok(ProtocolType::Binary),
                    "LAX_BINARY" => Ok(ProtocolType::LaxBinary),
                    "COMPACT" => Ok(ProtocolType::Compact),
                    "TWITTER" => Ok(ProtocolType::Twitter),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.Route", len)?;
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
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.Route")
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
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.Route", FIELDS, GeneratedVisitor)
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
        if self.metadata_match.is_some() {
            len += 1;
        }
        if !self.rate_limits.is_empty() {
            len += 1;
        }
        if self.strip_service_name {
            len += 1;
        }
        if !self.request_mirror_policies.is_empty() {
            len += 1;
        }
        if self.cluster_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteAction", len)?;
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        if !self.rate_limits.is_empty() {
            struct_ser.serialize_field("rateLimits", &self.rate_limits)?;
        }
        if self.strip_service_name {
            struct_ser.serialize_field("stripServiceName", &self.strip_service_name)?;
        }
        if !self.request_mirror_policies.is_empty() {
            struct_ser.serialize_field("requestMirrorPolicies", &self.request_mirror_policies)?;
        }
        if let Some(v) = self.cluster_specifier.as_ref() {
            match v {
                route_action::ClusterSpecifier::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
                route_action::ClusterSpecifier::WeightedClusters(v) => {
                    struct_ser.serialize_field("weightedClusters", v)?;
                }
                route_action::ClusterSpecifier::ClusterHeader(v) => {
                    struct_ser.serialize_field("clusterHeader", v)?;
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
            "metadata_match",
            "metadataMatch",
            "rate_limits",
            "rateLimits",
            "strip_service_name",
            "stripServiceName",
            "request_mirror_policies",
            "requestMirrorPolicies",
            "cluster",
            "weighted_clusters",
            "weightedClusters",
            "cluster_header",
            "clusterHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataMatch,
            RateLimits,
            StripServiceName,
            RequestMirrorPolicies,
            Cluster,
            WeightedClusters,
            ClusterHeader,
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
                            "metadataMatch" | "metadata_match" => Ok(GeneratedField::MetadataMatch),
                            "rateLimits" | "rate_limits" => Ok(GeneratedField::RateLimits),
                            "stripServiceName" | "strip_service_name" => Ok(GeneratedField::StripServiceName),
                            "requestMirrorPolicies" | "request_mirror_policies" => Ok(GeneratedField::RequestMirrorPolicies),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "weightedClusters" | "weighted_clusters" => Ok(GeneratedField::WeightedClusters),
                            "clusterHeader" | "cluster_header" => Ok(GeneratedField::ClusterHeader),
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
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.RouteAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_match__ = None;
                let mut rate_limits__ = None;
                let mut strip_service_name__ = None;
                let mut request_mirror_policies__ = None;
                let mut cluster_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                        GeneratedField::RateLimits => {
                            if rate_limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimits"));
                            }
                            rate_limits__ = Some(map.next_value()?);
                        }
                        GeneratedField::StripServiceName => {
                            if strip_service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stripServiceName"));
                            }
                            strip_service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestMirrorPolicies => {
                            if request_mirror_policies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMirrorPolicies"));
                            }
                            request_mirror_policies__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::Cluster);
                        }
                        GeneratedField::WeightedClusters => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weightedClusters"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::WeightedClusters)
;
                        }
                        GeneratedField::ClusterHeader => {
                            if cluster_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterHeader"));
                            }
                            cluster_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_action::ClusterSpecifier::ClusterHeader);
                        }
                    }
                }
                Ok(RouteAction {
                    metadata_match: metadata_match__,
                    rate_limits: rate_limits__.unwrap_or_default(),
                    strip_service_name: strip_service_name__.unwrap_or_default(),
                    request_mirror_policies: request_mirror_policies__.unwrap_or_default(),
                    cluster_specifier: cluster_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteAction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for route_action::RequestMirrorPolicy {
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
        if self.runtime_fraction.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteAction.RequestMirrorPolicy", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if let Some(v) = self.runtime_fraction.as_ref() {
            struct_ser.serialize_field("runtimeFraction", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for route_action::RequestMirrorPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "runtime_fraction",
            "runtimeFraction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            RuntimeFraction,
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
                            "runtimeFraction" | "runtime_fraction" => Ok(GeneratedField::RuntimeFraction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = route_action::RequestMirrorPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.RouteAction.RequestMirrorPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<route_action::RequestMirrorPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut runtime_fraction__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::RuntimeFraction => {
                            if runtime_fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeFraction"));
                            }
                            runtime_fraction__ = map.next_value()?;
                        }
                    }
                }
                Ok(route_action::RequestMirrorPolicy {
                    cluster: cluster__.unwrap_or_default(),
                    runtime_fraction: runtime_fraction__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteAction.RequestMirrorPolicy", FIELDS, GeneratedVisitor)
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
        if self.validate_clusters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteConfiguration", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.routes.is_empty() {
            struct_ser.serialize_field("routes", &self.routes)?;
        }
        if let Some(v) = self.validate_clusters.as_ref() {
            struct_ser.serialize_field("validateClusters", v)?;
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
            "validate_clusters",
            "validateClusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Routes,
            ValidateClusters,
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
                            "validateClusters" | "validate_clusters" => Ok(GeneratedField::ValidateClusters),
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
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.RouteConfiguration")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut routes__ = None;
                let mut validate_clusters__ = None;
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
                        GeneratedField::ValidateClusters => {
                            if validate_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validateClusters"));
                            }
                            validate_clusters__ = map.next_value()?;
                        }
                    }
                }
                Ok(RouteConfiguration {
                    name: name__.unwrap_or_default(),
                    routes: routes__.unwrap_or_default(),
                    validate_clusters: validate_clusters__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteConfiguration", FIELDS, GeneratedVisitor)
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
        if self.invert {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if self.match_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteMatch", len)?;
        if self.invert {
            struct_ser.serialize_field("invert", &self.invert)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if let Some(v) = self.match_specifier.as_ref() {
            match v {
                route_match::MatchSpecifier::MethodName(v) => {
                    struct_ser.serialize_field("methodName", v)?;
                }
                route_match::MatchSpecifier::ServiceName(v) => {
                    struct_ser.serialize_field("serviceName", v)?;
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
            "invert",
            "headers",
            "method_name",
            "methodName",
            "service_name",
            "serviceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Invert,
            Headers,
            MethodName,
            ServiceName,
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
                            "invert" => Ok(GeneratedField::Invert),
                            "headers" => Ok(GeneratedField::Headers),
                            "methodName" | "method_name" => Ok(GeneratedField::MethodName),
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
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
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.RouteMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut invert__ = None;
                let mut headers__ = None;
                let mut match_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Invert => {
                            if invert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invert"));
                            }
                            invert__ = Some(map.next_value()?);
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::MethodName => {
                            if match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methodName"));
                            }
                            match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_match::MatchSpecifier::MethodName);
                        }
                        GeneratedField::ServiceName => {
                            if match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(route_match::MatchSpecifier::ServiceName);
                        }
                    }
                }
                Ok(RouteMatch {
                    invert: invert__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                    match_specifier: match_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.RouteMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ThriftFilter {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.ThriftFilter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                thrift_filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ThriftFilter {
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
            type Value = ThriftFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.ThriftFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ThriftFilter, V::Error>
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
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(thrift_filter::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(ThriftFilter {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.ThriftFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ThriftProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transport != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.ThriftProtocolOptions", len)?;
        if self.transport != 0 {
            let v = TransportType::from_i32(self.transport)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.transport)))?;
            struct_ser.serialize_field("transport", &v)?;
        }
        if self.protocol != 0 {
            let v = ProtocolType::from_i32(self.protocol)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ThriftProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transport",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transport,
            Protocol,
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
                            "transport" => Ok(GeneratedField::Transport),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ThriftProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.ThriftProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ThriftProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport__ = None;
                let mut protocol__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Transport => {
                            if transport__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            transport__ = Some(map.next_value::<TransportType>()? as i32);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map.next_value::<ProtocolType>()? as i32);
                        }
                    }
                }
                Ok(ThriftProtocolOptions {
                    transport: transport__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.ThriftProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ThriftProxy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transport != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.route_config.is_some() {
            len += 1;
        }
        if self.trds.is_some() {
            len += 1;
        }
        if !self.thrift_filters.is_empty() {
            len += 1;
        }
        if self.payload_passthrough {
            len += 1;
        }
        if self.max_requests_per_connection.is_some() {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        if self.header_keys_preserve_case {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.ThriftProxy", len)?;
        if self.transport != 0 {
            let v = TransportType::from_i32(self.transport)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.transport)))?;
            struct_ser.serialize_field("transport", &v)?;
        }
        if self.protocol != 0 {
            let v = ProtocolType::from_i32(self.protocol)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.route_config.as_ref() {
            struct_ser.serialize_field("routeConfig", v)?;
        }
        if let Some(v) = self.trds.as_ref() {
            struct_ser.serialize_field("trds", v)?;
        }
        if !self.thrift_filters.is_empty() {
            struct_ser.serialize_field("thriftFilters", &self.thrift_filters)?;
        }
        if self.payload_passthrough {
            struct_ser.serialize_field("payloadPassthrough", &self.payload_passthrough)?;
        }
        if let Some(v) = self.max_requests_per_connection.as_ref() {
            struct_ser.serialize_field("maxRequestsPerConnection", v)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("accessLog", &self.access_log)?;
        }
        if self.header_keys_preserve_case {
            struct_ser.serialize_field("headerKeysPreserveCase", &self.header_keys_preserve_case)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ThriftProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transport",
            "protocol",
            "stat_prefix",
            "statPrefix",
            "route_config",
            "routeConfig",
            "trds",
            "thrift_filters",
            "thriftFilters",
            "payload_passthrough",
            "payloadPassthrough",
            "max_requests_per_connection",
            "maxRequestsPerConnection",
            "access_log",
            "accessLog",
            "header_keys_preserve_case",
            "headerKeysPreserveCase",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transport,
            Protocol,
            StatPrefix,
            RouteConfig,
            Trds,
            ThriftFilters,
            PayloadPassthrough,
            MaxRequestsPerConnection,
            AccessLog,
            HeaderKeysPreserveCase,
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
                            "transport" => Ok(GeneratedField::Transport),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "trds" => Ok(GeneratedField::Trds),
                            "thriftFilters" | "thrift_filters" => Ok(GeneratedField::ThriftFilters),
                            "payloadPassthrough" | "payload_passthrough" => Ok(GeneratedField::PayloadPassthrough),
                            "maxRequestsPerConnection" | "max_requests_per_connection" => Ok(GeneratedField::MaxRequestsPerConnection),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "headerKeysPreserveCase" | "header_keys_preserve_case" => Ok(GeneratedField::HeaderKeysPreserveCase),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ThriftProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.ThriftProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ThriftProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport__ = None;
                let mut protocol__ = None;
                let mut stat_prefix__ = None;
                let mut route_config__ = None;
                let mut trds__ = None;
                let mut thrift_filters__ = None;
                let mut payload_passthrough__ = None;
                let mut max_requests_per_connection__ = None;
                let mut access_log__ = None;
                let mut header_keys_preserve_case__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Transport => {
                            if transport__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            transport__ = Some(map.next_value::<TransportType>()? as i32);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map.next_value::<ProtocolType>()? as i32);
                        }
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
                        GeneratedField::Trds => {
                            if trds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trds"));
                            }
                            trds__ = map.next_value()?;
                        }
                        GeneratedField::ThriftFilters => {
                            if thrift_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("thriftFilters"));
                            }
                            thrift_filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::PayloadPassthrough => {
                            if payload_passthrough__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadPassthrough"));
                            }
                            payload_passthrough__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxRequestsPerConnection => {
                            if max_requests_per_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestsPerConnection"));
                            }
                            max_requests_per_connection__ = map.next_value()?;
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeaderKeysPreserveCase => {
                            if header_keys_preserve_case__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerKeysPreserveCase"));
                            }
                            header_keys_preserve_case__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ThriftProxy {
                    transport: transport__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    route_config: route_config__,
                    trds: trds__,
                    thrift_filters: thrift_filters__.unwrap_or_default(),
                    payload_passthrough: payload_passthrough__.unwrap_or_default(),
                    max_requests_per_connection: max_requests_per_connection__,
                    access_log: access_log__.unwrap_or_default(),
                    header_keys_preserve_case: header_keys_preserve_case__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.ThriftProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransportType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AutoTransport => "AUTO_TRANSPORT",
            Self::Framed => "FRAMED",
            Self::Unframed => "UNFRAMED",
            Self::Header => "HEADER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TransportType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO_TRANSPORT",
            "FRAMED",
            "UNFRAMED",
            "HEADER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransportType;

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
                    .and_then(TransportType::from_i32)
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
                    .and_then(TransportType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AUTO_TRANSPORT" => Ok(TransportType::AutoTransport),
                    "FRAMED" => Ok(TransportType::Framed),
                    "UNFRAMED" => Ok(TransportType::Unframed),
                    "HEADER" => Ok(TransportType::Header),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Trds {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.Trds", len)?;
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("configSource", v)?;
        }
        if !self.route_config_name.is_empty() {
            struct_ser.serialize_field("routeConfigName", &self.route_config_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Trds {
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
            type Value = Trds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.Trds")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Trds, V::Error>
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
                Ok(Trds {
                    config_source: config_source__,
                    route_config_name: route_config_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.Trds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WeightedCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.clusters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.WeightedCluster", len)?;
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WeightedCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Clusters,
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
                            "clusters" => Ok(GeneratedField::Clusters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WeightedCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.WeightedCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WeightedCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut clusters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WeightedCluster {
                    clusters: clusters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.WeightedCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for weighted_cluster::ClusterWeight {
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
        if self.weight.is_some() {
            len += 1;
        }
        if self.metadata_match.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.WeightedCluster.ClusterWeight", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.weight.as_ref() {
            struct_ser.serialize_field("weight", v)?;
        }
        if let Some(v) = self.metadata_match.as_ref() {
            struct_ser.serialize_field("metadataMatch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for weighted_cluster::ClusterWeight {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "weight",
            "metadata_match",
            "metadataMatch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Weight,
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
                            "name" => Ok(GeneratedField::Name),
                            "weight" => Ok(GeneratedField::Weight),
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
            type Value = weighted_cluster::ClusterWeight;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.v3.WeightedCluster.ClusterWeight")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<weighted_cluster::ClusterWeight, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut weight__ = None;
                let mut metadata_match__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = map.next_value()?;
                        }
                        GeneratedField::MetadataMatch => {
                            if metadata_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataMatch"));
                            }
                            metadata_match__ = map.next_value()?;
                        }
                    }
                }
                Ok(weighted_cluster::ClusterWeight {
                    name: name__.unwrap_or_default(),
                    weight: weight__,
                    metadata_match: metadata_match__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.v3.WeightedCluster.ClusterWeight", FIELDS, GeneratedVisitor)
    }
}
