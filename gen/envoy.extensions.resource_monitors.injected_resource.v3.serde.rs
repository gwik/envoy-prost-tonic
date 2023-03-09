// @generated
impl serde::Serialize for InjectedResourceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filename.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.resource_monitors.injected_resource.v3.InjectedResourceConfig", len)?;
        if !self.filename.is_empty() {
            struct_ser.serialize_field("filename", &self.filename)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InjectedResourceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filename",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filename,
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
                            "filename" => Ok(GeneratedField::Filename),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InjectedResourceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.resource_monitors.injected_resource.v3.InjectedResourceConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InjectedResourceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filename__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(InjectedResourceConfig {
                    filename: filename__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.resource_monitors.injected_resource.v3.InjectedResourceConfig", FIELDS, GeneratedVisitor)
    }
}
