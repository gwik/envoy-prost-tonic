// @generated
impl serde::Serialize for ApiListener {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.api_listener.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.listener.v2.ApiListener", len)?;
        if let Some(v) = self.api_listener.as_ref() {
            struct_ser.serialize_field("apiListener", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiListener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "api_listener",
            "apiListener",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiListener,
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
                            "apiListener" | "api_listener" => Ok(GeneratedField::ApiListener),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiListener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.listener.v2.ApiListener")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiListener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut api_listener__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ApiListener => {
                            if api_listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiListener"));
                            }
                            api_listener__ = map.next_value()?;
                        }
                    }
                }
                Ok(ApiListener {
                    api_listener: api_listener__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.listener.v2.ApiListener", FIELDS, GeneratedVisitor)
    }
}
