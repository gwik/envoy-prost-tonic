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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.lua.v2.Lua", len)?;
        if !self.inline_code.is_empty() {
            struct_ser.serialize_field("inlineCode", &self.inline_code)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InlineCode,
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
                formatter.write_str("struct envoy.config.filter.http.lua.v2.Lua")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Lua, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inline_code__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InlineCode => {
                            if inline_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineCode"));
                            }
                            inline_code__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Lua {
                    inline_code: inline_code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.lua.v2.Lua", FIELDS, GeneratedVisitor)
    }
}
