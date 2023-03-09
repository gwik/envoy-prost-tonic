// @generated
impl serde::Serialize for Maglev {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.table_size.is_some() {
            len += 1;
        }
        if self.consistent_hashing_lb_config.is_some() {
            len += 1;
        }
        if self.locality_weighted_lb_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.maglev.v3.Maglev", len)?;
        if let Some(v) = self.table_size.as_ref() {
            struct_ser.serialize_field("tableSize", v)?;
        }
        if let Some(v) = self.consistent_hashing_lb_config.as_ref() {
            struct_ser.serialize_field("consistentHashingLbConfig", v)?;
        }
        if let Some(v) = self.locality_weighted_lb_config.as_ref() {
            struct_ser.serialize_field("localityWeightedLbConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Maglev {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "table_size",
            "tableSize",
            "consistent_hashing_lb_config",
            "consistentHashingLbConfig",
            "locality_weighted_lb_config",
            "localityWeightedLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TableSize,
            ConsistentHashingLbConfig,
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
                            "tableSize" | "table_size" => Ok(GeneratedField::TableSize),
                            "consistentHashingLbConfig" | "consistent_hashing_lb_config" => Ok(GeneratedField::ConsistentHashingLbConfig),
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
            type Value = Maglev;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.maglev.v3.Maglev")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Maglev, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut table_size__ = None;
                let mut consistent_hashing_lb_config__ = None;
                let mut locality_weighted_lb_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TableSize => {
                            if table_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tableSize"));
                            }
                            table_size__ = map.next_value()?;
                        }
                        GeneratedField::ConsistentHashingLbConfig => {
                            if consistent_hashing_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistentHashingLbConfig"));
                            }
                            consistent_hashing_lb_config__ = map.next_value()?;
                        }
                        GeneratedField::LocalityWeightedLbConfig => {
                            if locality_weighted_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityWeightedLbConfig"));
                            }
                            locality_weighted_lb_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(Maglev {
                    table_size: table_size__,
                    consistent_hashing_lb_config: consistent_hashing_lb_config__,
                    locality_weighted_lb_config: locality_weighted_lb_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.maglev.v3.Maglev", FIELDS, GeneratedVisitor)
    }
}
