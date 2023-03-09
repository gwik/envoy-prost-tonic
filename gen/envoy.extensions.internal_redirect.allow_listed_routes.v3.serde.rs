// @generated
impl serde::Serialize for AllowListedRoutesConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_route_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.internal_redirect.allow_listed_routes.v3.AllowListedRoutesConfig", len)?;
        if !self.allowed_route_names.is_empty() {
            struct_ser.serialize_field("allowedRouteNames", &self.allowed_route_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowListedRoutesConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_route_names",
            "allowedRouteNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedRouteNames,
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
                            "allowedRouteNames" | "allowed_route_names" => Ok(GeneratedField::AllowedRouteNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowListedRoutesConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.internal_redirect.allow_listed_routes.v3.AllowListedRoutesConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllowListedRoutesConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_route_names__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowedRouteNames => {
                            if allowed_route_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedRouteNames"));
                            }
                            allowed_route_names__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AllowListedRoutesConfig {
                    allowed_route_names: allowed_route_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.internal_redirect.allow_listed_routes.v3.AllowListedRoutesConfig", FIELDS, GeneratedVisitor)
    }
}
