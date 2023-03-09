// @generated
impl serde::Serialize for Lua {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inline_code.is_empty() {
            len += 1;
        }
        if !self.source_codes.is_empty() {
            len += 1;
        }
        if self.default_source_code.is_some() {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.lua.v3.Lua", len)?;
        if !self.inline_code.is_empty() {
            struct_ser.serialize_field("inlineCode", &self.inline_code)?;
        }
        if !self.source_codes.is_empty() {
            struct_ser.serialize_field("sourceCodes", &self.source_codes)?;
        }
        if let Some(v) = self.default_source_code.as_ref() {
            struct_ser.serialize_field("defaultSourceCode", v)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Lua {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inline_code",
            "inlineCode",
            "source_codes",
            "sourceCodes",
            "default_source_code",
            "defaultSourceCode",
            "stat_prefix",
            "statPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InlineCode,
            SourceCodes,
            DefaultSourceCode,
            StatPrefix,
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
                            "inlineCode" | "inline_code" => Ok(GeneratedField::InlineCode),
                            "sourceCodes" | "source_codes" => Ok(GeneratedField::SourceCodes),
                            "defaultSourceCode" | "default_source_code" => Ok(GeneratedField::DefaultSourceCode),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Lua;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.lua.v3.Lua")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Lua, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inline_code__ = None;
                let mut source_codes__ = None;
                let mut default_source_code__ = None;
                let mut stat_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InlineCode => {
                            if inline_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineCode"));
                            }
                            inline_code__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourceCodes => {
                            if source_codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceCodes"));
                            }
                            source_codes__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::DefaultSourceCode => {
                            if default_source_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultSourceCode"));
                            }
                            default_source_code__ = map.next_value()?;
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Lua {
                    inline_code: inline_code__.unwrap_or_default(),
                    source_codes: source_codes__.unwrap_or_default(),
                    default_source_code: default_source_code__,
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.lua.v3.Lua", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LuaPerRoute {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.lua.v3.LuaPerRoute", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                lua_per_route::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                lua_per_route::Override::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
                lua_per_route::Override::SourceCode(v) => {
                    struct_ser.serialize_field("sourceCode", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LuaPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "name",
            "source_code",
            "sourceCode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            Name,
            SourceCode,
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
                            "name" => Ok(GeneratedField::Name),
                            "sourceCode" | "source_code" => Ok(GeneratedField::SourceCode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LuaPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.lua.v3.LuaPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LuaPerRoute, V::Error>
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
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(lua_per_route::Override::Disabled);
                        }
                        GeneratedField::Name => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(lua_per_route::Override::Name);
                        }
                        GeneratedField::SourceCode => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceCode"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(lua_per_route::Override::SourceCode)
;
                        }
                    }
                }
                Ok(LuaPerRoute {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.lua.v3.LuaPerRoute", FIELDS, GeneratedVisitor)
    }
}
