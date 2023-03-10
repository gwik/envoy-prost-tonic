// @generated
impl serde::Serialize for CsrfPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter_enabled.is_some() {
            len += 1;
        }
        if self.shadow_enabled.is_some() {
            len += 1;
        }
        if !self.additional_origins.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.csrf.v2.CsrfPolicy", len)?;
        if let Some(v) = self.filter_enabled.as_ref() {
            struct_ser.serialize_field("filterEnabled", v)?;
        }
        if let Some(v) = self.shadow_enabled.as_ref() {
            struct_ser.serialize_field("shadowEnabled", v)?;
        }
        if !self.additional_origins.is_empty() {
            struct_ser.serialize_field("additionalOrigins", &self.additional_origins)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CsrfPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter_enabled",
            "filterEnabled",
            "shadow_enabled",
            "shadowEnabled",
            "additional_origins",
            "additionalOrigins",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilterEnabled,
            ShadowEnabled,
            AdditionalOrigins,
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
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            "shadowEnabled" | "shadow_enabled" => Ok(GeneratedField::ShadowEnabled),
                            "additionalOrigins" | "additional_origins" => Ok(GeneratedField::AdditionalOrigins),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CsrfPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.csrf.v2.CsrfPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CsrfPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_enabled__ = None;
                let mut shadow_enabled__ = None;
                let mut additional_origins__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FilterEnabled => {
                            if filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabled"));
                            }
                            filter_enabled__ = map.next_value()?;
                        }
                        GeneratedField::ShadowEnabled => {
                            if shadow_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shadowEnabled"));
                            }
                            shadow_enabled__ = map.next_value()?;
                        }
                        GeneratedField::AdditionalOrigins => {
                            if additional_origins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalOrigins"));
                            }
                            additional_origins__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CsrfPolicy {
                    filter_enabled: filter_enabled__,
                    shadow_enabled: shadow_enabled__,
                    additional_origins: additional_origins__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.csrf.v2.CsrfPolicy", FIELDS, GeneratedVisitor)
    }
}
