// @generated
impl serde::Serialize for CustomResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.custom_response_matcher.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.custom_response.v3.CustomResponse", len)?;
        if let Some(v) = self.custom_response_matcher.as_ref() {
            struct_ser.serialize_field("customResponseMatcher", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "custom_response_matcher",
            "customResponseMatcher",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CustomResponseMatcher,
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
                            "customResponseMatcher" | "custom_response_matcher" => Ok(GeneratedField::CustomResponseMatcher),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.custom_response.v3.CustomResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CustomResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut custom_response_matcher__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CustomResponseMatcher => {
                            if custom_response_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customResponseMatcher"));
                            }
                            custom_response_matcher__ = map.next_value()?;
                        }
                    }
                }
                Ok(CustomResponse {
                    custom_response_matcher: custom_response_matcher__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.custom_response.v3.CustomResponse", FIELDS, GeneratedVisitor)
    }
}
