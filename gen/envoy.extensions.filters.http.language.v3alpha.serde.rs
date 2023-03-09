// @generated
impl serde::Serialize for Language {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.default_language.is_empty() {
            len += 1;
        }
        if !self.supported_languages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.language.v3alpha.Language", len)?;
        if !self.default_language.is_empty() {
            struct_ser.serialize_field("defaultLanguage", &self.default_language)?;
        }
        if !self.supported_languages.is_empty() {
            struct_ser.serialize_field("supportedLanguages", &self.supported_languages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Language {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_language",
            "defaultLanguage",
            "supported_languages",
            "supportedLanguages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultLanguage,
            SupportedLanguages,
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
                            "defaultLanguage" | "default_language" => Ok(GeneratedField::DefaultLanguage),
                            "supportedLanguages" | "supported_languages" => Ok(GeneratedField::SupportedLanguages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Language;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.language.v3alpha.Language")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Language, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_language__ = None;
                let mut supported_languages__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefaultLanguage => {
                            if default_language__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultLanguage"));
                            }
                            default_language__ = Some(map.next_value()?);
                        }
                        GeneratedField::SupportedLanguages => {
                            if supported_languages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportedLanguages"));
                            }
                            supported_languages__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Language {
                    default_language: default_language__.unwrap_or_default(),
                    supported_languages: supported_languages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.language.v3alpha.Language", FIELDS, GeneratedVisitor)
    }
}
