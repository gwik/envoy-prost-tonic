// @generated
impl serde::Serialize for OriginalSrc {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bind_port {
            len += 1;
        }
        if self.mark != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.listener.original_src.v3.OriginalSrc", len)?;
        if self.bind_port {
            struct_ser.serialize_field("bindPort", &self.bind_port)?;
        }
        if self.mark != 0 {
            struct_ser.serialize_field("mark", &self.mark)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OriginalSrc {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bind_port",
            "bindPort",
            "mark",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BindPort,
            Mark,
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
                            "bindPort" | "bind_port" => Ok(GeneratedField::BindPort),
                            "mark" => Ok(GeneratedField::Mark),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OriginalSrc;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.listener.original_src.v3.OriginalSrc")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OriginalSrc, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bind_port__ = None;
                let mut mark__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BindPort => {
                            if bind_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bindPort"));
                            }
                            bind_port__ = Some(map.next_value()?);
                        }
                        GeneratedField::Mark => {
                            if mark__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mark"));
                            }
                            mark__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(OriginalSrc {
                    bind_port: bind_port__.unwrap_or_default(),
                    mark: mark__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.listener.original_src.v3.OriginalSrc", FIELDS, GeneratedVisitor)
    }
}
