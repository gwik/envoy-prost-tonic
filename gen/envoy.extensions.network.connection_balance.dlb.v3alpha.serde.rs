// @generated
impl serde::Serialize for Dlb {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if self.max_retries != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.network.connection_balance.dlb.v3alpha.Dlb", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.max_retries != 0 {
            struct_ser.serialize_field("maxRetries", &self.max_retries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dlb {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "max_retries",
            "maxRetries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            MaxRetries,
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
                            "id" => Ok(GeneratedField::Id),
                            "maxRetries" | "max_retries" => Ok(GeneratedField::MaxRetries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dlb;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.network.connection_balance.dlb.v3alpha.Dlb")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Dlb, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut max_retries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxRetries => {
                            if max_retries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRetries"));
                            }
                            max_retries__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Dlb {
                    id: id__.unwrap_or_default(),
                    max_retries: max_retries__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.network.connection_balance.dlb.v3alpha.Dlb", FIELDS, GeneratedVisitor)
    }
}
