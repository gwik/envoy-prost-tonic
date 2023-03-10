// @generated
impl serde::Serialize for HeaderToMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata", len)?;
        if !self.request_rules.is_empty() {
            struct_ser.serialize_field("requestRules", &self.request_rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderToMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_rules",
            "requestRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestRules,
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
                            "requestRules" | "request_rules" => Ok(GeneratedField::RequestRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderToMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderToMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestRules => {
                            if request_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestRules"));
                            }
                            request_rules__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HeaderToMetadata {
                    request_rules: request_rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header_to_metadata::KeyValuePair {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metadata_namespace.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.encode != 0 {
            len += 1;
        }
        if self.value_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata.KeyValuePair", len)?;
        if !self.metadata_namespace.is_empty() {
            struct_ser.serialize_field("metadataNamespace", &self.metadata_namespace)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.r#type != 0 {
            let v = header_to_metadata::ValueType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.encode != 0 {
            let v = header_to_metadata::ValueEncode::from_i32(self.encode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.encode)))?;
            struct_ser.serialize_field("encode", &v)?;
        }
        if let Some(v) = self.value_type.as_ref() {
            match v {
                header_to_metadata::key_value_pair::ValueType::Value(v) => {
                    struct_ser.serialize_field("value", v)?;
                }
                header_to_metadata::key_value_pair::ValueType::RegexValueRewrite(v) => {
                    struct_ser.serialize_field("regexValueRewrite", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for header_to_metadata::KeyValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_namespace",
            "metadataNamespace",
            "key",
            "type",
            "encode",
            "value",
            "regex_value_rewrite",
            "regexValueRewrite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataNamespace,
            Key,
            Type,
            Encode,
            Value,
            RegexValueRewrite,
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
                            "metadataNamespace" | "metadata_namespace" => Ok(GeneratedField::MetadataNamespace),
                            "key" => Ok(GeneratedField::Key),
                            "type" => Ok(GeneratedField::Type),
                            "encode" => Ok(GeneratedField::Encode),
                            "value" => Ok(GeneratedField::Value),
                            "regexValueRewrite" | "regex_value_rewrite" => Ok(GeneratedField::RegexValueRewrite),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_to_metadata::KeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata.KeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<header_to_metadata::KeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_namespace__ = None;
                let mut key__ = None;
                let mut r#type__ = None;
                let mut encode__ = None;
                let mut value_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MetadataNamespace => {
                            if metadata_namespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataNamespace"));
                            }
                            metadata_namespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<header_to_metadata::ValueType>()? as i32);
                        }
                        GeneratedField::Encode => {
                            if encode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encode"));
                            }
                            encode__ = Some(map.next_value::<header_to_metadata::ValueEncode>()? as i32);
                        }
                        GeneratedField::Value => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value_type__ = map.next_value::<::std::option::Option<_>>()?.map(header_to_metadata::key_value_pair::ValueType::Value);
                        }
                        GeneratedField::RegexValueRewrite => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexValueRewrite"));
                            }
                            value_type__ = map.next_value::<::std::option::Option<_>>()?.map(header_to_metadata::key_value_pair::ValueType::RegexValueRewrite)
;
                        }
                    }
                }
                Ok(header_to_metadata::KeyValuePair {
                    metadata_namespace: metadata_namespace__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    encode: encode__.unwrap_or_default(),
                    value_type: value_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata.KeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header_to_metadata::Rule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header.is_empty() {
            len += 1;
        }
        if self.on_present.is_some() {
            len += 1;
        }
        if self.on_missing.is_some() {
            len += 1;
        }
        if self.remove {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata.Rule", len)?;
        if !self.header.is_empty() {
            struct_ser.serialize_field("header", &self.header)?;
        }
        if let Some(v) = self.on_present.as_ref() {
            struct_ser.serialize_field("onPresent", v)?;
        }
        if let Some(v) = self.on_missing.as_ref() {
            struct_ser.serialize_field("onMissing", v)?;
        }
        if self.remove {
            struct_ser.serialize_field("remove", &self.remove)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for header_to_metadata::Rule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "on_present",
            "onPresent",
            "on_missing",
            "onMissing",
            "remove",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            OnPresent,
            OnMissing,
            Remove,
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
                            "header" => Ok(GeneratedField::Header),
                            "onPresent" | "on_present" => Ok(GeneratedField::OnPresent),
                            "onMissing" | "on_missing" => Ok(GeneratedField::OnMissing),
                            "remove" => Ok(GeneratedField::Remove),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_to_metadata::Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata.Rule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<header_to_metadata::Rule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut on_present__ = None;
                let mut on_missing__ = None;
                let mut remove__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = Some(map.next_value()?);
                        }
                        GeneratedField::OnPresent => {
                            if on_present__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onPresent"));
                            }
                            on_present__ = map.next_value()?;
                        }
                        GeneratedField::OnMissing => {
                            if on_missing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onMissing"));
                            }
                            on_missing__ = map.next_value()?;
                        }
                        GeneratedField::Remove => {
                            if remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remove"));
                            }
                            remove__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(header_to_metadata::Rule {
                    header: header__.unwrap_or_default(),
                    on_present: on_present__,
                    on_missing: on_missing__,
                    remove: remove__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.header_to_metadata.v3.HeaderToMetadata.Rule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header_to_metadata::ValueEncode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::Base64 => "BASE64",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for header_to_metadata::ValueEncode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "BASE64",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_to_metadata::ValueEncode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(header_to_metadata::ValueEncode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(header_to_metadata::ValueEncode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(header_to_metadata::ValueEncode::None),
                    "BASE64" => Ok(header_to_metadata::ValueEncode::Base64),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for header_to_metadata::ValueType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::String => "STRING",
            Self::Number => "NUMBER",
            Self::ProtobufValue => "PROTOBUF_VALUE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for header_to_metadata::ValueType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STRING",
            "NUMBER",
            "PROTOBUF_VALUE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_to_metadata::ValueType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(header_to_metadata::ValueType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(header_to_metadata::ValueType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STRING" => Ok(header_to_metadata::ValueType::String),
                    "NUMBER" => Ok(header_to_metadata::ValueType::Number),
                    "PROTOBUF_VALUE" => Ok(header_to_metadata::ValueType::ProtobufValue),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
