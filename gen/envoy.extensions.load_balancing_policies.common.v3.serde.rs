// @generated
impl serde::Serialize for ConsistentHashingLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_hostname_for_hashing {
            len += 1;
        }
        if self.hash_balance_factor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.common.v3.ConsistentHashingLbConfig", len)?;
        if self.use_hostname_for_hashing {
            struct_ser.serialize_field("useHostnameForHashing", &self.use_hostname_for_hashing)?;
        }
        if let Some(v) = self.hash_balance_factor.as_ref() {
            struct_ser.serialize_field("hashBalanceFactor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsistentHashingLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_hostname_for_hashing",
            "useHostnameForHashing",
            "hash_balance_factor",
            "hashBalanceFactor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseHostnameForHashing,
            HashBalanceFactor,
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
                            "useHostnameForHashing" | "use_hostname_for_hashing" => Ok(GeneratedField::UseHostnameForHashing),
                            "hashBalanceFactor" | "hash_balance_factor" => Ok(GeneratedField::HashBalanceFactor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsistentHashingLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.common.v3.ConsistentHashingLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConsistentHashingLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_hostname_for_hashing__ = None;
                let mut hash_balance_factor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UseHostnameForHashing => {
                            if use_hostname_for_hashing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useHostnameForHashing"));
                            }
                            use_hostname_for_hashing__ = Some(map.next_value()?);
                        }
                        GeneratedField::HashBalanceFactor => {
                            if hash_balance_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashBalanceFactor"));
                            }
                            hash_balance_factor__ = map.next_value()?;
                        }
                    }
                }
                Ok(ConsistentHashingLbConfig {
                    use_hostname_for_hashing: use_hostname_for_hashing__.unwrap_or_default(),
                    hash_balance_factor: hash_balance_factor__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.common.v3.ConsistentHashingLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalityLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.locality_config_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig", len)?;
        if let Some(v) = self.locality_config_specifier.as_ref() {
            match v {
                locality_lb_config::LocalityConfigSpecifier::ZoneAwareLbConfig(v) => {
                    struct_ser.serialize_field("zoneAwareLbConfig", v)?;
                }
                locality_lb_config::LocalityConfigSpecifier::LocalityWeightedLbConfig(v) => {
                    struct_ser.serialize_field("localityWeightedLbConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalityLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "zone_aware_lb_config",
            "zoneAwareLbConfig",
            "locality_weighted_lb_config",
            "localityWeightedLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ZoneAwareLbConfig,
            LocalityWeightedLbConfig,
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
                            "zoneAwareLbConfig" | "zone_aware_lb_config" => Ok(GeneratedField::ZoneAwareLbConfig),
                            "localityWeightedLbConfig" | "locality_weighted_lb_config" => Ok(GeneratedField::LocalityWeightedLbConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalityLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LocalityLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality_config_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ZoneAwareLbConfig => {
                            if locality_config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zoneAwareLbConfig"));
                            }
                            locality_config_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(locality_lb_config::LocalityConfigSpecifier::ZoneAwareLbConfig)
;
                        }
                        GeneratedField::LocalityWeightedLbConfig => {
                            if locality_config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityWeightedLbConfig"));
                            }
                            locality_config_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(locality_lb_config::LocalityConfigSpecifier::LocalityWeightedLbConfig)
;
                        }
                    }
                }
                Ok(LocalityLbConfig {
                    locality_config_specifier: locality_config_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for locality_lb_config::LocalityWeightedLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for locality_lb_config::LocalityWeightedLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = locality_lb_config::LocalityWeightedLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<locality_lb_config::LocalityWeightedLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(locality_lb_config::LocalityWeightedLbConfig {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.LocalityWeightedLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for locality_lb_config::ZoneAwareLbConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.routing_enabled.is_some() {
            len += 1;
        }
        if self.min_cluster_size.is_some() {
            len += 1;
        }
        if self.fail_traffic_on_panic {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig", len)?;
        if let Some(v) = self.routing_enabled.as_ref() {
            struct_ser.serialize_field("routingEnabled", v)?;
        }
        if let Some(v) = self.min_cluster_size.as_ref() {
            struct_ser.serialize_field("minClusterSize", v)?;
        }
        if self.fail_traffic_on_panic {
            struct_ser.serialize_field("failTrafficOnPanic", &self.fail_traffic_on_panic)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for locality_lb_config::ZoneAwareLbConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "routing_enabled",
            "routingEnabled",
            "min_cluster_size",
            "minClusterSize",
            "fail_traffic_on_panic",
            "failTrafficOnPanic",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoutingEnabled,
            MinClusterSize,
            FailTrafficOnPanic,
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
                            "routingEnabled" | "routing_enabled" => Ok(GeneratedField::RoutingEnabled),
                            "minClusterSize" | "min_cluster_size" => Ok(GeneratedField::MinClusterSize),
                            "failTrafficOnPanic" | "fail_traffic_on_panic" => Ok(GeneratedField::FailTrafficOnPanic),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = locality_lb_config::ZoneAwareLbConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<locality_lb_config::ZoneAwareLbConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut routing_enabled__ = None;
                let mut min_cluster_size__ = None;
                let mut fail_traffic_on_panic__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RoutingEnabled => {
                            if routing_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routingEnabled"));
                            }
                            routing_enabled__ = map.next_value()?;
                        }
                        GeneratedField::MinClusterSize => {
                            if min_cluster_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minClusterSize"));
                            }
                            min_cluster_size__ = map.next_value()?;
                        }
                        GeneratedField::FailTrafficOnPanic => {
                            if fail_traffic_on_panic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failTrafficOnPanic"));
                            }
                            fail_traffic_on_panic__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(locality_lb_config::ZoneAwareLbConfig {
                    routing_enabled: routing_enabled__,
                    min_cluster_size: min_cluster_size__,
                    fail_traffic_on_panic: fail_traffic_on_panic__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.common.v3.LocalityLbConfig.ZoneAwareLbConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SlowStartConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.slow_start_window.is_some() {
            len += 1;
        }
        if self.aggression.is_some() {
            len += 1;
        }
        if self.min_weight_percent.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig", len)?;
        if let Some(v) = self.slow_start_window.as_ref() {
            struct_ser.serialize_field("slowStartWindow", v)?;
        }
        if let Some(v) = self.aggression.as_ref() {
            struct_ser.serialize_field("aggression", v)?;
        }
        if let Some(v) = self.min_weight_percent.as_ref() {
            struct_ser.serialize_field("minWeightPercent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SlowStartConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "slow_start_window",
            "slowStartWindow",
            "aggression",
            "min_weight_percent",
            "minWeightPercent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SlowStartWindow,
            Aggression,
            MinWeightPercent,
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
                            "slowStartWindow" | "slow_start_window" => Ok(GeneratedField::SlowStartWindow),
                            "aggression" => Ok(GeneratedField::Aggression),
                            "minWeightPercent" | "min_weight_percent" => Ok(GeneratedField::MinWeightPercent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SlowStartConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SlowStartConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut slow_start_window__ = None;
                let mut aggression__ = None;
                let mut min_weight_percent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SlowStartWindow => {
                            if slow_start_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slowStartWindow"));
                            }
                            slow_start_window__ = map.next_value()?;
                        }
                        GeneratedField::Aggression => {
                            if aggression__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggression"));
                            }
                            aggression__ = map.next_value()?;
                        }
                        GeneratedField::MinWeightPercent => {
                            if min_weight_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minWeightPercent"));
                            }
                            min_weight_percent__ = map.next_value()?;
                        }
                    }
                }
                Ok(SlowStartConfig {
                    slow_start_window: slow_start_window__,
                    aggression: aggression__,
                    min_weight_percent: min_weight_percent__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.common.v3.SlowStartConfig", FIELDS, GeneratedVisitor)
    }
}
