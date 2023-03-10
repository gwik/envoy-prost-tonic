// @generated
impl serde::Serialize for AbortActionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.wait_duration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.watchdog.v3.AbortActionConfig", len)?;
        if let Some(v) = self.wait_duration.as_ref() {
            struct_ser.serialize_field("waitDuration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AbortActionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "wait_duration",
            "waitDuration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            WaitDuration,
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
                            "waitDuration" | "wait_duration" => Ok(GeneratedField::WaitDuration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbortActionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.watchdog.v3.AbortActionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AbortActionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut wait_duration__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::WaitDuration => {
                            if wait_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("waitDuration"));
                            }
                            wait_duration__ = map.next_value()?;
                        }
                    }
                }
                Ok(AbortActionConfig {
                    wait_duration: wait_duration__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.watchdog.v3.AbortActionConfig", FIELDS, GeneratedVisitor)
    }
}
