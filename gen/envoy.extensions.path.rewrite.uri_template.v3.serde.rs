// @generated
impl serde::Serialize for UriTemplateRewriteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path_template_rewrite.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.path.rewrite.uri_template.v3.UriTemplateRewriteConfig", len)?;
        if !self.path_template_rewrite.is_empty() {
            struct_ser.serialize_field("pathTemplateRewrite", &self.path_template_rewrite)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UriTemplateRewriteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path_template_rewrite",
            "pathTemplateRewrite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PathTemplateRewrite,
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
                            "pathTemplateRewrite" | "path_template_rewrite" => Ok(GeneratedField::PathTemplateRewrite),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UriTemplateRewriteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.path.rewrite.uri_template.v3.UriTemplateRewriteConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UriTemplateRewriteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path_template_rewrite__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PathTemplateRewrite => {
                            if path_template_rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathTemplateRewrite"));
                            }
                            path_template_rewrite__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UriTemplateRewriteConfig {
                    path_template_rewrite: path_template_rewrite__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.path.rewrite.uri_template.v3.UriTemplateRewriteConfig", FIELDS, GeneratedVisitor)
    }
}
