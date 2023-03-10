// @generated
impl serde::Serialize for UriTemplateMatchConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path_template.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.path.r#match.uri_template.v3.UriTemplateMatchConfig", len)?;
        if !self.path_template.is_empty() {
            struct_ser.serialize_field("pathTemplate", &self.path_template)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UriTemplateMatchConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path_template",
            "pathTemplate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PathTemplate,
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
                            "pathTemplate" | "path_template" => Ok(GeneratedField::PathTemplate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UriTemplateMatchConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.path.r#match.uri_template.v3.UriTemplateMatchConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UriTemplateMatchConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path_template__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PathTemplate => {
                            if path_template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathTemplate"));
                            }
                            path_template__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UriTemplateMatchConfig {
                    path_template: path_template__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.path.r#match.uri_template.v3.UriTemplateMatchConfig", FIELDS, GeneratedVisitor)
    }
}
