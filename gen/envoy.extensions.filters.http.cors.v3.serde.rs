// @generated
impl serde::Serialize for Cors {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.cors.v3.Cors", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cors {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cors;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.cors.v3.Cors")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Cors, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Cors {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.cors.v3.Cors", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CorsPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allow_origin_string_match.is_empty() {
            len += 1;
        }
        if !self.allow_methods.is_empty() {
            len += 1;
        }
        if !self.allow_headers.is_empty() {
            len += 1;
        }
        if !self.expose_headers.is_empty() {
            len += 1;
        }
        if !self.max_age.is_empty() {
            len += 1;
        }
        if self.allow_credentials.is_some() {
            len += 1;
        }
        if self.filter_enabled.is_some() {
            len += 1;
        }
        if self.shadow_enabled.is_some() {
            len += 1;
        }
        if self.allow_private_network_access.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.cors.v3.CorsPolicy", len)?;
        if !self.allow_origin_string_match.is_empty() {
            struct_ser.serialize_field("allowOriginStringMatch", &self.allow_origin_string_match)?;
        }
        if !self.allow_methods.is_empty() {
            struct_ser.serialize_field("allowMethods", &self.allow_methods)?;
        }
        if !self.allow_headers.is_empty() {
            struct_ser.serialize_field("allowHeaders", &self.allow_headers)?;
        }
        if !self.expose_headers.is_empty() {
            struct_ser.serialize_field("exposeHeaders", &self.expose_headers)?;
        }
        if !self.max_age.is_empty() {
            struct_ser.serialize_field("maxAge", &self.max_age)?;
        }
        if let Some(v) = self.allow_credentials.as_ref() {
            struct_ser.serialize_field("allowCredentials", v)?;
        }
        if let Some(v) = self.filter_enabled.as_ref() {
            struct_ser.serialize_field("filterEnabled", v)?;
        }
        if let Some(v) = self.shadow_enabled.as_ref() {
            struct_ser.serialize_field("shadowEnabled", v)?;
        }
        if let Some(v) = self.allow_private_network_access.as_ref() {
            struct_ser.serialize_field("allowPrivateNetworkAccess", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CorsPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_origin_string_match",
            "allowOriginStringMatch",
            "allow_methods",
            "allowMethods",
            "allow_headers",
            "allowHeaders",
            "expose_headers",
            "exposeHeaders",
            "max_age",
            "maxAge",
            "allow_credentials",
            "allowCredentials",
            "filter_enabled",
            "filterEnabled",
            "shadow_enabled",
            "shadowEnabled",
            "allow_private_network_access",
            "allowPrivateNetworkAccess",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowOriginStringMatch,
            AllowMethods,
            AllowHeaders,
            ExposeHeaders,
            MaxAge,
            AllowCredentials,
            FilterEnabled,
            ShadowEnabled,
            AllowPrivateNetworkAccess,
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
                            "allowOriginStringMatch" | "allow_origin_string_match" => Ok(GeneratedField::AllowOriginStringMatch),
                            "allowMethods" | "allow_methods" => Ok(GeneratedField::AllowMethods),
                            "allowHeaders" | "allow_headers" => Ok(GeneratedField::AllowHeaders),
                            "exposeHeaders" | "expose_headers" => Ok(GeneratedField::ExposeHeaders),
                            "maxAge" | "max_age" => Ok(GeneratedField::MaxAge),
                            "allowCredentials" | "allow_credentials" => Ok(GeneratedField::AllowCredentials),
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            "shadowEnabled" | "shadow_enabled" => Ok(GeneratedField::ShadowEnabled),
                            "allowPrivateNetworkAccess" | "allow_private_network_access" => Ok(GeneratedField::AllowPrivateNetworkAccess),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CorsPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.cors.v3.CorsPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CorsPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_origin_string_match__ = None;
                let mut allow_methods__ = None;
                let mut allow_headers__ = None;
                let mut expose_headers__ = None;
                let mut max_age__ = None;
                let mut allow_credentials__ = None;
                let mut filter_enabled__ = None;
                let mut shadow_enabled__ = None;
                let mut allow_private_network_access__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowOriginStringMatch => {
                            if allow_origin_string_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowOriginStringMatch"));
                            }
                            allow_origin_string_match__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowMethods => {
                            if allow_methods__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowMethods"));
                            }
                            allow_methods__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowHeaders => {
                            if allow_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowHeaders"));
                            }
                            allow_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExposeHeaders => {
                            if expose_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exposeHeaders"));
                            }
                            expose_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxAge => {
                            if max_age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxAge"));
                            }
                            max_age__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowCredentials => {
                            if allow_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowCredentials"));
                            }
                            allow_credentials__ = map.next_value()?;
                        }
                        GeneratedField::FilterEnabled => {
                            if filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabled"));
                            }
                            filter_enabled__ = map.next_value()?;
                        }
                        GeneratedField::ShadowEnabled => {
                            if shadow_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shadowEnabled"));
                            }
                            shadow_enabled__ = map.next_value()?;
                        }
                        GeneratedField::AllowPrivateNetworkAccess => {
                            if allow_private_network_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowPrivateNetworkAccess"));
                            }
                            allow_private_network_access__ = map.next_value()?;
                        }
                    }
                }
                Ok(CorsPolicy {
                    allow_origin_string_match: allow_origin_string_match__.unwrap_or_default(),
                    allow_methods: allow_methods__.unwrap_or_default(),
                    allow_headers: allow_headers__.unwrap_or_default(),
                    expose_headers: expose_headers__.unwrap_or_default(),
                    max_age: max_age__.unwrap_or_default(),
                    allow_credentials: allow_credentials__,
                    filter_enabled: filter_enabled__,
                    shadow_enabled: shadow_enabled__,
                    allow_private_network_access: allow_private_network_access__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.cors.v3.CorsPolicy", FIELDS, GeneratedVisitor)
    }
}
