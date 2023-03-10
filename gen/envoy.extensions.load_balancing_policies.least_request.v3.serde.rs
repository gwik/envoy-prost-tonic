// @generated
impl serde::Serialize for LeastRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.choice_count.is_some() {
            len += 1;
        }
        if self.active_request_bias.is_some() {
            len += 1;
        }
        if self.slow_start_config.is_some() {
            len += 1;
        }
        if self.locality_lb_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.least_request.v3.LeastRequest", len)?;
        if let Some(v) = self.choice_count.as_ref() {
            struct_ser.serialize_field("choiceCount", v)?;
        }
        if let Some(v) = self.active_request_bias.as_ref() {
            struct_ser.serialize_field("activeRequestBias", v)?;
        }
        if let Some(v) = self.slow_start_config.as_ref() {
            struct_ser.serialize_field("slowStartConfig", v)?;
        }
        if let Some(v) = self.locality_lb_config.as_ref() {
            struct_ser.serialize_field("localityLbConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LeastRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "choice_count",
            "choiceCount",
            "active_request_bias",
            "activeRequestBias",
            "slow_start_config",
            "slowStartConfig",
            "locality_lb_config",
            "localityLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChoiceCount,
            ActiveRequestBias,
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
                            "choiceCount" | "choice_count" => Ok(GeneratedField::ChoiceCount),
                            "activeRequestBias" | "active_request_bias" => Ok(GeneratedField::ActiveRequestBias),
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
            type Value = LeastRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.least_request.v3.LeastRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LeastRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut choice_count__ = None;
                let mut active_request_bias__ = None;
                let mut slow_start_config__ = None;
                let mut locality_lb_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChoiceCount => {
                            if choice_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("choiceCount"));
                            }
                            choice_count__ = map.next_value()?;
                        }
                        GeneratedField::ActiveRequestBias => {
                            if active_request_bias__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeRequestBias"));
                            }
                            active_request_bias__ = map.next_value()?;
                        }
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
                Ok(LeastRequest {
                    choice_count: choice_count__,
                    active_request_bias: active_request_bias__,
                    slow_start_config: slow_start_config__,
                    locality_lb_config: locality_lb_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.least_request.v3.LeastRequest", FIELDS, GeneratedVisitor)
    }
}
