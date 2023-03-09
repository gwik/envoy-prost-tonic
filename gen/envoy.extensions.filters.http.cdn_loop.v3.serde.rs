// @generated
impl serde::Serialize for CdnLoopConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cdn_id.is_empty() {
            len += 1;
        }
        if self.max_allowed_occurrences != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.cdn_loop.v3.CdnLoopConfig", len)?;
        if !self.cdn_id.is_empty() {
            struct_ser.serialize_field("cdnId", &self.cdn_id)?;
        }
        if self.max_allowed_occurrences != 0 {
            struct_ser.serialize_field("maxAllowedOccurrences", &self.max_allowed_occurrences)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CdnLoopConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cdn_id",
            "cdnId",
            "max_allowed_occurrences",
            "maxAllowedOccurrences",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CdnId,
            MaxAllowedOccurrences,
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
                            "cdnId" | "cdn_id" => Ok(GeneratedField::CdnId),
                            "maxAllowedOccurrences" | "max_allowed_occurrences" => Ok(GeneratedField::MaxAllowedOccurrences),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CdnLoopConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.cdn_loop.v3.CdnLoopConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CdnLoopConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cdn_id__ = None;
                let mut max_allowed_occurrences__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CdnId => {
                            if cdn_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cdnId"));
                            }
                            cdn_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxAllowedOccurrences => {
                            if max_allowed_occurrences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAllowedOccurrences"));
                            }
                            max_allowed_occurrences__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CdnLoopConfig {
                    cdn_id: cdn_id__.unwrap_or_default(),
                    max_allowed_occurrences: max_allowed_occurrences__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.cdn_loop.v3.CdnLoopConfig", FIELDS, GeneratedVisitor)
    }
}
