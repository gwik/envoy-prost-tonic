// @generated
impl serde::Serialize for HealthCheck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pass_through_mode.is_some() {
            len += 1;
        }
        if self.cache_time.is_some() {
            len += 1;
        }
        if !self.cluster_min_healthy_percentages.is_empty() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.health_check.v2.HealthCheck", len)?;
        if let Some(v) = self.pass_through_mode.as_ref() {
            struct_ser.serialize_field("passThroughMode", v)?;
        }
        if let Some(v) = self.cache_time.as_ref() {
            struct_ser.serialize_field("cacheTime", v)?;
        }
        if !self.cluster_min_healthy_percentages.is_empty() {
            struct_ser.serialize_field("clusterMinHealthyPercentages", &self.cluster_min_healthy_percentages)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pass_through_mode",
            "passThroughMode",
            "cache_time",
            "cacheTime",
            "cluster_min_healthy_percentages",
            "clusterMinHealthyPercentages",
            "headers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PassThroughMode,
            CacheTime,
            ClusterMinHealthyPercentages,
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
                            "passThroughMode" | "pass_through_mode" => Ok(GeneratedField::PassThroughMode),
                            "cacheTime" | "cache_time" => Ok(GeneratedField::CacheTime),
                            "clusterMinHealthyPercentages" | "cluster_min_healthy_percentages" => Ok(GeneratedField::ClusterMinHealthyPercentages),
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
            type Value = HealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.health_check.v2.HealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pass_through_mode__ = None;
                let mut cache_time__ = None;
                let mut cluster_min_healthy_percentages__ = None;
                let mut headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PassThroughMode => {
                            if pass_through_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passThroughMode"));
                            }
                            pass_through_mode__ = map.next_value()?;
                        }
                        GeneratedField::CacheTime => {
                            if cache_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheTime"));
                            }
                            cache_time__ = map.next_value()?;
                        }
                        GeneratedField::ClusterMinHealthyPercentages => {
                            if cluster_min_healthy_percentages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterMinHealthyPercentages"));
                            }
                            cluster_min_healthy_percentages__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HealthCheck {
                    pass_through_mode: pass_through_mode__,
                    cache_time: cache_time__,
                    cluster_min_healthy_percentages: cluster_min_healthy_percentages__.unwrap_or_default(),
                    headers: headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.health_check.v2.HealthCheck", FIELDS, GeneratedVisitor)
    }
}
