// @generated
impl serde::Serialize for HeaderMutation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.mutations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.early_header_mutation.header_mutation.v3.HeaderMutation", len)?;
        if !self.mutations.is_empty() {
            struct_ser.serialize_field("mutations", &self.mutations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMutation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mutations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Mutations,
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
                            "mutations" => Ok(GeneratedField::Mutations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderMutation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.early_header_mutation.header_mutation.v3.HeaderMutation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderMutation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut mutations__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Mutations => {
                            if mutations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutations"));
                            }
                            mutations__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HeaderMutation {
                    mutations: mutations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.early_header_mutation.header_mutation.v3.HeaderMutation", FIELDS, GeneratedVisitor)
    }
}
