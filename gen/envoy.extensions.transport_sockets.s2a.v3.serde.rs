// @generated
impl serde::Serialize for S2aConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.s2a_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.s2a.v3.S2AConfiguration", len)?;
        if !self.s2a_address.is_empty() {
            struct_ser.serialize_field("s2aAddress", &self.s2a_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for S2aConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "s2a_address",
            "s2aAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            S2aAddress,
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
                            "s2aAddress" | "s2a_address" => Ok(GeneratedField::S2aAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = S2aConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.s2a.v3.S2AConfiguration")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<S2aConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut s2a_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::S2aAddress => {
                            if s2a_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("s2aAddress"));
                            }
                            s2a_address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(S2aConfiguration {
                    s2a_address: s2a_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.s2a.v3.S2AConfiguration", FIELDS, GeneratedVisitor)
    }
}
