// @generated
impl serde::Serialize for RoundRobin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.slow_start_config.is_some() {
            len += 1;
        }
        if self.locality_lb_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.round_robin.v3.RoundRobin", len)?;
        if let Some(v) = self.slow_start_config.as_ref() {
            struct_ser.serialize_field("slowStartConfig", v)?;
        }
        if let Some(v) = self.locality_lb_config.as_ref() {
            struct_ser.serialize_field("localityLbConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoundRobin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "slow_start_config",
            "slowStartConfig",
            "locality_lb_config",
            "localityLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SlowStartConfig,
            LocalityLbConfig,
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
                            "slowStartConfig" | "slow_start_config" => Ok(GeneratedField::SlowStartConfig),
                            "localityLbConfig" | "locality_lb_config" => Ok(GeneratedField::LocalityLbConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoundRobin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.round_robin.v3.RoundRobin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RoundRobin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut slow_start_config__ = None;
                let mut locality_lb_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SlowStartConfig => {
                            if slow_start_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("slowStartConfig"));
                            }
                            slow_start_config__ = map.next_value()?;
                        }
                        GeneratedField::LocalityLbConfig => {
                            if locality_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityLbConfig"));
                            }
                            locality_lb_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(RoundRobin {
                    slow_start_config: slow_start_config__,
                    locality_lb_config: locality_lb_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.round_robin.v3.RoundRobin", FIELDS, GeneratedVisitor)
    }
}
