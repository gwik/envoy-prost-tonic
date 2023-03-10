// @generated
impl serde::Serialize for CookieBasedSessionState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cookie.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.stateful_session.cookie.v3.CookieBasedSessionState", len)?;
        if let Some(v) = self.cookie.as_ref() {
            struct_ser.serialize_field("cookie", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CookieBasedSessionState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cookie",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cookie,
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
                            "cookie" => Ok(GeneratedField::Cookie),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CookieBasedSessionState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.stateful_session.cookie.v3.CookieBasedSessionState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CookieBasedSessionState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cookie__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cookie => {
                            if cookie__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cookie"));
                            }
                            cookie__ = map.next_value()?;
                        }
                    }
                }
                Ok(CookieBasedSessionState {
                    cookie: cookie__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.stateful_session.cookie.v3.CookieBasedSessionState", FIELDS, GeneratedVisitor)
    }
}
