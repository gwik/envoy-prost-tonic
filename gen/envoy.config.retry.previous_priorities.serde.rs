// @generated
impl serde::Serialize for PreviousPrioritiesConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.update_frequency != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.retry.previous_priorities.PreviousPrioritiesConfig", len)?;
        if self.update_frequency != 0 {
            struct_ser.serialize_field("updateFrequency", &self.update_frequency)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PreviousPrioritiesConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "update_frequency",
            "updateFrequency",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpdateFrequency,
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
                            "updateFrequency" | "update_frequency" => Ok(GeneratedField::UpdateFrequency),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PreviousPrioritiesConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.retry.previous_priorities.PreviousPrioritiesConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PreviousPrioritiesConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut update_frequency__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UpdateFrequency => {
                            if update_frequency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateFrequency"));
                            }
                            update_frequency__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PreviousPrioritiesConfig {
                    update_frequency: update_frequency__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.retry.previous_priorities.PreviousPrioritiesConfig", FIELDS, GeneratedVisitor)
    }
}
