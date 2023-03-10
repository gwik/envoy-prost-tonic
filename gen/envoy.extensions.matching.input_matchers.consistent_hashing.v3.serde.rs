// @generated
impl serde::Serialize for ConsistentHashing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.threshold != 0 {
            len += 1;
        }
        if self.modulo != 0 {
            len += 1;
        }
        if self.seed != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.matching.input_matchers.consistent_hashing.v3.ConsistentHashing", len)?;
        if self.threshold != 0 {
            struct_ser.serialize_field("threshold", &self.threshold)?;
        }
        if self.modulo != 0 {
            struct_ser.serialize_field("modulo", &self.modulo)?;
        }
        if self.seed != 0 {
            struct_ser.serialize_field("seed", ToString::to_string(&self.seed).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsistentHashing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "threshold",
            "modulo",
            "seed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Threshold,
            Modulo,
            Seed,
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
                            "threshold" => Ok(GeneratedField::Threshold),
                            "modulo" => Ok(GeneratedField::Modulo),
                            "seed" => Ok(GeneratedField::Seed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsistentHashing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.matching.input_matchers.consistent_hashing.v3.ConsistentHashing")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConsistentHashing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut threshold__ = None;
                let mut modulo__ = None;
                let mut seed__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Threshold => {
                            if threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threshold"));
                            }
                            threshold__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Modulo => {
                            if modulo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modulo"));
                            }
                            modulo__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Seed => {
                            if seed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seed"));
                            }
                            seed__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConsistentHashing {
                    threshold: threshold__.unwrap_or_default(),
                    modulo: modulo__.unwrap_or_default(),
                    seed: seed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.matching.input_matchers.consistent_hashing.v3.ConsistentHashing", FIELDS, GeneratedVisitor)
    }
}
