// @generated
impl serde::Serialize for StatefulSession {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.session_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.stateful_session.v3.StatefulSession", len)?;
        if let Some(v) = self.session_state.as_ref() {
            struct_ser.serialize_field("sessionState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatefulSession {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "session_state",
            "sessionState",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SessionState,
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
                            "sessionState" | "session_state" => Ok(GeneratedField::SessionState),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatefulSession;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.stateful_session.v3.StatefulSession")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatefulSession, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut session_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SessionState => {
                            if session_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionState"));
                            }
                            session_state__ = map.next_value()?;
                        }
                    }
                }
                Ok(StatefulSession {
                    session_state: session_state__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.stateful_session.v3.StatefulSession", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatefulSessionPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.stateful_session.v3.StatefulSessionPerRoute", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                stateful_session_per_route::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                stateful_session_per_route::Override::StatefulSession(v) => {
                    struct_ser.serialize_field("statefulSession", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StatefulSessionPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "stateful_session",
            "statefulSession",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            StatefulSession,
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
                            "disabled" => Ok(GeneratedField::Disabled),
                            "statefulSession" | "stateful_session" => Ok(GeneratedField::StatefulSession),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatefulSessionPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.stateful_session.v3.StatefulSessionPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StatefulSessionPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#override__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(stateful_session_per_route::Override::Disabled);
                        }
                        GeneratedField::StatefulSession => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statefulSession"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(stateful_session_per_route::Override::StatefulSession)
;
                        }
                    }
                }
                Ok(StatefulSessionPerRoute {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.stateful_session.v3.StatefulSessionPerRoute", FIELDS, GeneratedVisitor)
    }
}
