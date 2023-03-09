// @generated
impl serde::Serialize for LocalResponsePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.body.is_some() {
            len += 1;
        }
        if self.body_format.is_some() {
            len += 1;
        }
        if self.status_code.is_some() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.custom_response.local_response_policy.v3.LocalResponsePolicy", len)?;
        if let Some(v) = self.body.as_ref() {
            struct_ser.serialize_field("body", v)?;
        }
        if let Some(v) = self.body_format.as_ref() {
            struct_ser.serialize_field("bodyFormat", v)?;
        }
        if let Some(v) = self.status_code.as_ref() {
            struct_ser.serialize_field("statusCode", v)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalResponsePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "body",
            "body_format",
            "bodyFormat",
            "status_code",
            "statusCode",
            "response_headers_to_add",
            "responseHeadersToAdd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Body,
            BodyFormat,
            StatusCode,
            ResponseHeadersToAdd,
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
                            "body" => Ok(GeneratedField::Body),
                            "bodyFormat" | "body_format" => Ok(GeneratedField::BodyFormat),
                            "statusCode" | "status_code" => Ok(GeneratedField::StatusCode),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalResponsePolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.custom_response.local_response_policy.v3.LocalResponsePolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LocalResponsePolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut body__ = None;
                let mut body_format__ = None;
                let mut status_code__ = None;
                let mut response_headers_to_add__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Body => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("body"));
                            }
                            body__ = map.next_value()?;
                        }
                        GeneratedField::BodyFormat => {
                            if body_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodyFormat"));
                            }
                            body_format__ = map.next_value()?;
                        }
                        GeneratedField::StatusCode => {
                            if status_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusCode"));
                            }
                            status_code__ = map.next_value()?;
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LocalResponsePolicy {
                    body: body__,
                    body_format: body_format__,
                    status_code: status_code__,
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.custom_response.local_response_policy.v3.LocalResponsePolicy", FIELDS, GeneratedVisitor)
    }
}
