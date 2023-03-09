// @generated
impl serde::Serialize for ExtensionWithMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.matcher.is_some() {
            len += 1;
        }
        if self.xds_matcher.is_some() {
            len += 1;
        }
        if self.extension_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.common.matching.v3.ExtensionWithMatcher", len)?;
        if let Some(v) = self.matcher.as_ref() {
            struct_ser.serialize_field("matcher", v)?;
        }
        if let Some(v) = self.xds_matcher.as_ref() {
            struct_ser.serialize_field("xdsMatcher", v)?;
        }
        if let Some(v) = self.extension_config.as_ref() {
            struct_ser.serialize_field("extensionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionWithMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "matcher",
            "xds_matcher",
            "xdsMatcher",
            "extension_config",
            "extensionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Matcher,
            XdsMatcher,
            ExtensionConfig,
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
                            "matcher" => Ok(GeneratedField::Matcher),
                            "xdsMatcher" | "xds_matcher" => Ok(GeneratedField::XdsMatcher),
                            "extensionConfig" | "extension_config" => Ok(GeneratedField::ExtensionConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionWithMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.common.matching.v3.ExtensionWithMatcher")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtensionWithMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut matcher__ = None;
                let mut xds_matcher__ = None;
                let mut extension_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Matcher => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            matcher__ = map.next_value()?;
                        }
                        GeneratedField::XdsMatcher => {
                            if xds_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xdsMatcher"));
                            }
                            xds_matcher__ = map.next_value()?;
                        }
                        GeneratedField::ExtensionConfig => {
                            if extension_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensionConfig"));
                            }
                            extension_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExtensionWithMatcher {
                    matcher: matcher__,
                    xds_matcher: xds_matcher__,
                    extension_config: extension_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.common.matching.v3.ExtensionWithMatcher", FIELDS, GeneratedVisitor)
    }
}
