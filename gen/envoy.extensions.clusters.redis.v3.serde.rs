// @generated
impl serde::Serialize for RedisClusterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cluster_refresh_rate.is_some() {
            len += 1;
        }
        if self.cluster_refresh_timeout.is_some() {
            len += 1;
        }
        if self.redirect_refresh_interval.is_some() {
            len += 1;
        }
        if self.redirect_refresh_threshold.is_some() {
            len += 1;
        }
        if self.failure_refresh_threshold != 0 {
            len += 1;
        }
        if self.host_degraded_refresh_threshold != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.clusters.redis.v3.RedisClusterConfig", len)?;
        if let Some(v) = self.cluster_refresh_rate.as_ref() {
            struct_ser.serialize_field("clusterRefreshRate", v)?;
        }
        if let Some(v) = self.cluster_refresh_timeout.as_ref() {
            struct_ser.serialize_field("clusterRefreshTimeout", v)?;
        }
        if let Some(v) = self.redirect_refresh_interval.as_ref() {
            struct_ser.serialize_field("redirectRefreshInterval", v)?;
        }
        if let Some(v) = self.redirect_refresh_threshold.as_ref() {
            struct_ser.serialize_field("redirectRefreshThreshold", v)?;
        }
        if self.failure_refresh_threshold != 0 {
            struct_ser.serialize_field("failureRefreshThreshold", &self.failure_refresh_threshold)?;
        }
        if self.host_degraded_refresh_threshold != 0 {
            struct_ser.serialize_field("hostDegradedRefreshThreshold", &self.host_degraded_refresh_threshold)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedisClusterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_refresh_rate",
            "clusterRefreshRate",
            "cluster_refresh_timeout",
            "clusterRefreshTimeout",
            "redirect_refresh_interval",
            "redirectRefreshInterval",
            "redirect_refresh_threshold",
            "redirectRefreshThreshold",
            "failure_refresh_threshold",
            "failureRefreshThreshold",
            "host_degraded_refresh_threshold",
            "hostDegradedRefreshThreshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterRefreshRate,
            ClusterRefreshTimeout,
            RedirectRefreshInterval,
            RedirectRefreshThreshold,
            FailureRefreshThreshold,
            HostDegradedRefreshThreshold,
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
                            "clusterRefreshRate" | "cluster_refresh_rate" => Ok(GeneratedField::ClusterRefreshRate),
                            "clusterRefreshTimeout" | "cluster_refresh_timeout" => Ok(GeneratedField::ClusterRefreshTimeout),
                            "redirectRefreshInterval" | "redirect_refresh_interval" => Ok(GeneratedField::RedirectRefreshInterval),
                            "redirectRefreshThreshold" | "redirect_refresh_threshold" => Ok(GeneratedField::RedirectRefreshThreshold),
                            "failureRefreshThreshold" | "failure_refresh_threshold" => Ok(GeneratedField::FailureRefreshThreshold),
                            "hostDegradedRefreshThreshold" | "host_degraded_refresh_threshold" => Ok(GeneratedField::HostDegradedRefreshThreshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedisClusterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.clusters.redis.v3.RedisClusterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RedisClusterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_refresh_rate__ = None;
                let mut cluster_refresh_timeout__ = None;
                let mut redirect_refresh_interval__ = None;
                let mut redirect_refresh_threshold__ = None;
                let mut failure_refresh_threshold__ = None;
                let mut host_degraded_refresh_threshold__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterRefreshRate => {
                            if cluster_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterRefreshRate"));
                            }
                            cluster_refresh_rate__ = map.next_value()?;
                        }
                        GeneratedField::ClusterRefreshTimeout => {
                            if cluster_refresh_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterRefreshTimeout"));
                            }
                            cluster_refresh_timeout__ = map.next_value()?;
                        }
                        GeneratedField::RedirectRefreshInterval => {
                            if redirect_refresh_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectRefreshInterval"));
                            }
                            redirect_refresh_interval__ = map.next_value()?;
                        }
                        GeneratedField::RedirectRefreshThreshold => {
                            if redirect_refresh_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectRefreshThreshold"));
                            }
                            redirect_refresh_threshold__ = map.next_value()?;
                        }
                        GeneratedField::FailureRefreshThreshold => {
                            if failure_refresh_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureRefreshThreshold"));
                            }
                            failure_refresh_threshold__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HostDegradedRefreshThreshold => {
                            if host_degraded_refresh_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostDegradedRefreshThreshold"));
                            }
                            host_degraded_refresh_threshold__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RedisClusterConfig {
                    cluster_refresh_rate: cluster_refresh_rate__,
                    cluster_refresh_timeout: cluster_refresh_timeout__,
                    redirect_refresh_interval: redirect_refresh_interval__,
                    redirect_refresh_threshold: redirect_refresh_threshold__,
                    failure_refresh_threshold: failure_refresh_threshold__.unwrap_or_default(),
                    host_degraded_refresh_threshold: host_degraded_refresh_threshold__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.clusters.redis.v3.RedisClusterConfig", FIELDS, GeneratedVisitor)
    }
}
