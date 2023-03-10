// @generated
impl serde::Serialize for Rbac {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rules.is_some() {
            len += 1;
        }
        if self.shadow_rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.rbac.v2.RBAC", len)?;
        if let Some(v) = self.rules.as_ref() {
            struct_ser.serialize_field("rules", v)?;
        }
        if let Some(v) = self.shadow_rules.as_ref() {
            struct_ser.serialize_field("shadowRules", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rbac {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
            "shadow_rules",
            "shadowRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
            ShadowRules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            "shadowRules" | "shadow_rules" => Ok(GeneratedField::ShadowRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rbac;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.rbac.v2.RBAC")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Rbac, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                let mut shadow_rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = map.next_value()?;
                        }
                        GeneratedField::ShadowRules => {
                            if shadow_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shadowRules"));
                            }
                            shadow_rules__ = map.next_value()?;
                        }
                    }
                }
                Ok(Rbac {
                    rules: rules__,
                    shadow_rules: shadow_rules__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.rbac.v2.RBAC", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RbacPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rbac.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.rbac.v2.RBACPerRoute", len)?;
        if let Some(v) = self.rbac.as_ref() {
            struct_ser.serialize_field("rbac", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RbacPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rbac",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rbac,
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
                            "rbac" => Ok(GeneratedField::Rbac),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RbacPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.rbac.v2.RBACPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RbacPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rbac__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rbac => {
                            if rbac__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rbac"));
                            }
                            rbac__ = map.next_value()?;
                        }
                    }
                }
                Ok(RbacPerRoute {
                    rbac: rbac__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.rbac.v2.RBACPerRoute", FIELDS, GeneratedVisitor)
    }
}
