// @generated
impl serde::Serialize for KillRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.probability.is_some() {
            len += 1;
        }
        if !self.kill_request_header.is_empty() {
            len += 1;
        }
        if self.direction != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.kill_request.v3.KillRequest", len)?;
        if let Some(v) = self.probability.as_ref() {
            struct_ser.serialize_field("probability", v)?;
        }
        if !self.kill_request_header.is_empty() {
            struct_ser.serialize_field("killRequestHeader", &self.kill_request_header)?;
        }
        if self.direction != 0 {
            let v = kill_request::Direction::from_i32(self.direction)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.direction)))?;
            struct_ser.serialize_field("direction", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KillRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "probability",
            "kill_request_header",
            "killRequestHeader",
            "direction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Probability,
            KillRequestHeader,
            Direction,
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
                            "probability" => Ok(GeneratedField::Probability),
                            "killRequestHeader" | "kill_request_header" => Ok(GeneratedField::KillRequestHeader),
                            "direction" => Ok(GeneratedField::Direction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KillRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.kill_request.v3.KillRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KillRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut probability__ = None;
                let mut kill_request_header__ = None;
                let mut direction__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Probability => {
                            if probability__.is_some() {
                                return Err(serde::de::Error::duplicate_field("probability"));
                            }
                            probability__ = map.next_value()?;
                        }
                        GeneratedField::KillRequestHeader => {
                            if kill_request_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("killRequestHeader"));
                            }
                            kill_request_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::Direction => {
                            if direction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("direction"));
                            }
                            direction__ = Some(map.next_value::<kill_request::Direction>()? as i32);
                        }
                    }
                }
                Ok(KillRequest {
                    probability: probability__,
                    kill_request_header: kill_request_header__.unwrap_or_default(),
                    direction: direction__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.kill_request.v3.KillRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for kill_request::Direction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Request => "REQUEST",
            Self::Response => "RESPONSE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for kill_request::Direction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REQUEST",
            "RESPONSE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = kill_request::Direction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(kill_request::Direction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(kill_request::Direction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REQUEST" => Ok(kill_request::Direction::Request),
                    "RESPONSE" => Ok(kill_request::Direction::Response),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
