// @generated
impl serde::Serialize for Config {
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
        if !self.response_rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.header_to_metadata.v2.Config", len)?;
        if !self.request_rules.is_empty() {
            struct_ser.serialize_field("requestRules", &self.request_rules)?;
        }
        if !self.response_rules.is_empty() {
            struct_ser.serialize_field("responseRules", &self.response_rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_rules",
            "requestRules",
            "response_rules",
            "responseRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestRules,
            ResponseRules,
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
                            "responseRules" | "response_rules" => Ok(GeneratedField::ResponseRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.header_to_metadata.v2.Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_rules__ = None;
                let mut response_rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestRules => {
                            if request_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestRules"));
                            }
                            request_rules__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseRules => {
                            if response_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseRules"));
                            }
                            response_rules__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Config {
                    request_rules: request_rules__.unwrap_or_default(),
                    response_rules: response_rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.header_to_metadata.v2.Config", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for config::KeyValuePair {
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
        if !self.value.is_empty() {
            len += 1;
        }
        if self.r#type != 0 {
            len += 1;
        }
        if self.encode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.header_to_metadata.v2.Config.KeyValuePair", len)?;
        if !self.metadata_namespace.is_empty() {
            struct_ser.serialize_field("metadataNamespace", &self.metadata_namespace)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        if self.r#type != 0 {
            let v = config::ValueType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.encode != 0 {
            let v = config::ValueEncode::from_i32(self.encode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.encode)))?;
            struct_ser.serialize_field("encode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for config::KeyValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_namespace",
            "metadataNamespace",
            "key",
            "value",
            "type",
            "encode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataNamespace,
            Key,
            Value,
            Type,
            Encode,
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
                            "value" => Ok(GeneratedField::Value),
                            "type" => Ok(GeneratedField::Type),
                            "encode" => Ok(GeneratedField::Encode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = config::KeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.header_to_metadata.v2.Config.KeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<config::KeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_namespace__ = None;
                let mut key__ = None;
                let mut value__ = None;
                let mut r#type__ = None;
                let mut encode__ = None;
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
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<config::ValueType>()? as i32);
                        }
                        GeneratedField::Encode => {
                            if encode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encode"));
                            }
                            encode__ = Some(map.next_value::<config::ValueEncode>()? as i32);
                        }
                    }
                }
                Ok(config::KeyValuePair {
                    metadata_namespace: metadata_namespace__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    encode: encode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.header_to_metadata.v2.Config.KeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for config::Rule {
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
        if self.on_header_present.is_some() {
            len += 1;
        }
        if self.on_header_missing.is_some() {
            len += 1;
        }
        if self.remove {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.header_to_metadata.v2.Config.Rule", len)?;
        if !self.header.is_empty() {
            struct_ser.serialize_field("header", &self.header)?;
        }
        if let Some(v) = self.on_header_present.as_ref() {
            struct_ser.serialize_field("onHeaderPresent", v)?;
        }
        if let Some(v) = self.on_header_missing.as_ref() {
            struct_ser.serialize_field("onHeaderMissing", v)?;
        }
        if self.remove {
            struct_ser.serialize_field("remove", &self.remove)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for config::Rule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "on_header_present",
            "onHeaderPresent",
            "on_header_missing",
            "onHeaderMissing",
            "remove",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            OnHeaderPresent,
            OnHeaderMissing,
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
                            "onHeaderPresent" | "on_header_present" => Ok(GeneratedField::OnHeaderPresent),
                            "onHeaderMissing" | "on_header_missing" => Ok(GeneratedField::OnHeaderMissing),
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
            type Value = config::Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.header_to_metadata.v2.Config.Rule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<config::Rule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut on_header_present__ = None;
                let mut on_header_missing__ = None;
                let mut remove__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = Some(map.next_value()?);
                        }
                        GeneratedField::OnHeaderPresent => {
                            if on_header_present__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onHeaderPresent"));
                            }
                            on_header_present__ = map.next_value()?;
                        }
                        GeneratedField::OnHeaderMissing => {
                            if on_header_missing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onHeaderMissing"));
                            }
                            on_header_missing__ = map.next_value()?;
                        }
                        GeneratedField::Remove => {
                            if remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remove"));
                            }
                            remove__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(config::Rule {
                    header: header__.unwrap_or_default(),
                    on_header_present: on_header_present__,
                    on_header_missing: on_header_missing__,
                    remove: remove__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.header_to_metadata.v2.Config.Rule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for config::ValueEncode {
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
impl<'de> serde::Deserialize<'de> for config::ValueEncode {
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
            type Value = config::ValueEncode;

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
                    .and_then(config::ValueEncode::from_i32)
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
                    .and_then(config::ValueEncode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(config::ValueEncode::None),
                    "BASE64" => Ok(config::ValueEncode::Base64),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for config::ValueType {
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
impl<'de> serde::Deserialize<'de> for config::ValueType {
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
            type Value = config::ValueType;

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
                    .and_then(config::ValueType::from_i32)
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
                    .and_then(config::ValueType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STRING" => Ok(config::ValueType::String),
                    "NUMBER" => Ok(config::ValueType::Number),
                    "PROTOBUF_VALUE" => Ok(config::ValueType::ProtobufValue),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
