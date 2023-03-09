// @generated
impl serde::Serialize for Descriptor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.descriptor_key.is_empty() {
            len += 1;
        }
        if self.skip_if_error {
            len += 1;
        }
        if self.expr_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.rate_limit_descriptors.expr.v3.Descriptor", len)?;
        if !self.descriptor_key.is_empty() {
            struct_ser.serialize_field("descriptorKey", &self.descriptor_key)?;
        }
        if self.skip_if_error {
            struct_ser.serialize_field("skipIfError", &self.skip_if_error)?;
        }
        if let Some(v) = self.expr_specifier.as_ref() {
            match v {
                descriptor::ExprSpecifier::Text(v) => {
                    struct_ser.serialize_field("text", v)?;
                }
                descriptor::ExprSpecifier::Parsed(v) => {
                    struct_ser.serialize_field("parsed", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Descriptor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "descriptor_key",
            "descriptorKey",
            "skip_if_error",
            "skipIfError",
            "text",
            "parsed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DescriptorKey,
            SkipIfError,
            Text,
            Parsed,
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
                            "descriptorKey" | "descriptor_key" => Ok(GeneratedField::DescriptorKey),
                            "skipIfError" | "skip_if_error" => Ok(GeneratedField::SkipIfError),
                            "text" => Ok(GeneratedField::Text),
                            "parsed" => Ok(GeneratedField::Parsed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Descriptor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.rate_limit_descriptors.expr.v3.Descriptor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Descriptor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut descriptor_key__ = None;
                let mut skip_if_error__ = None;
                let mut expr_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DescriptorKey => {
                            if descriptor_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("descriptorKey"));
                            }
                            descriptor_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::SkipIfError => {
                            if skip_if_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipIfError"));
                            }
                            skip_if_error__ = Some(map.next_value()?);
                        }
                        GeneratedField::Text => {
                            if expr_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            expr_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(descriptor::ExprSpecifier::Text);
                        }
                        GeneratedField::Parsed => {
                            if expr_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parsed"));
                            }
                            expr_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(descriptor::ExprSpecifier::Parsed)
;
                        }
                    }
                }
                Ok(Descriptor {
                    descriptor_key: descriptor_key__.unwrap_or_default(),
                    skip_if_error: skip_if_error__.unwrap_or_default(),
                    expr_specifier: expr_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.rate_limit_descriptors.expr.v3.Descriptor", FIELDS, GeneratedVisitor)
    }
}
