// @generated
impl serde::Serialize for RouteAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.per_filter_config.is_empty() {
            len += 1;
        }
        if self.cluster_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.generic_proxy.action.v3.RouteAction", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.per_filter_config.is_empty() {
            struct_ser.serialize_field("perFilterConfig", &self.per_filter_config)?;
        }
        if let Some(v) = self.cluster_specifier.as_ref() {
            match v {
                route_action::ClusterSpecifier::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
                route_action::ClusterSpecifier::WeightedClusters(v) => {
                    struct_ser.serialize_field("weightedClusters", v)?;
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
            "metadata",
            "per_filter_config",
            "perFilterConfig",
            "cluster",
            "weighted_clusters",
            "weightedClusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            PerFilterConfig,
            Cluster,
            WeightedClusters,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            "perFilterConfig" | "per_filter_config" => Ok(GeneratedField::PerFilterConfig),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "weightedClusters" | "weighted_clusters" => Ok(GeneratedField::WeightedClusters),
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
                formatter.write_str("struct envoy.extensions.filters.network.generic_proxy.action.v3.RouteAction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RouteAction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut per_filter_config__ = None;
                let mut cluster_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::PerFilterConfig => {
                            if per_filter_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perFilterConfig"));
                            }
                            per_filter_config__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
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
                    }
                }
                Ok(RouteAction {
                    metadata: metadata__,
                    per_filter_config: per_filter_config__.unwrap_or_default(),
                    cluster_specifier: cluster_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.generic_proxy.action.v3.RouteAction", FIELDS, GeneratedVisitor)
    }
}
