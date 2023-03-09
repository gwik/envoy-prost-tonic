// @generated
impl serde::Serialize for PayloadToMetadata {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata", len)?;
        if !self.request_rules.is_empty() {
            struct_ser.serialize_field("requestRules", &self.request_rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PayloadToMetadata {
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
            type Value = PayloadToMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PayloadToMetadata, V::Error>
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
                Ok(PayloadToMetadata {
                    request_rules: request_rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for payload_to_metadata::FieldSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.id != 0 {
            len += 1;
        }
        if self.child.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.FieldSelector", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.id != 0 {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.child.as_ref() {
            struct_ser.serialize_field("child", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for payload_to_metadata::FieldSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "id",
            "child",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Id,
            Child,
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
                            "name" => Ok(GeneratedField::Name),
                            "id" => Ok(GeneratedField::Id),
                            "child" => Ok(GeneratedField::Child),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = payload_to_metadata::FieldSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.FieldSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<payload_to_metadata::FieldSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut id__ = None;
                let mut child__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Child => {
                            if child__.is_some() {
                                return Err(serde::de::Error::duplicate_field("child"));
                            }
                            child__ = map.next_value()?;
                        }
                    }
                }
                Ok(payload_to_metadata::FieldSelector {
                    name: name__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    child: child__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.FieldSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for payload_to_metadata::KeyValuePair {
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
        if self.value_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.KeyValuePair", len)?;
        if !self.metadata_namespace.is_empty() {
            struct_ser.serialize_field("metadataNamespace", &self.metadata_namespace)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if self.r#type != 0 {
            let v = payload_to_metadata::ValueType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.value_type.as_ref() {
            match v {
                payload_to_metadata::key_value_pair::ValueType::Value(v) => {
                    struct_ser.serialize_field("value", v)?;
                }
                payload_to_metadata::key_value_pair::ValueType::RegexValueRewrite(v) => {
                    struct_ser.serialize_field("regexValueRewrite", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for payload_to_metadata::KeyValuePair {
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
            "value",
            "regex_value_rewrite",
            "regexValueRewrite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataNamespace,
            Key,
            Type,
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
            type Value = payload_to_metadata::KeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.KeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<payload_to_metadata::KeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_namespace__ = None;
                let mut key__ = None;
                let mut r#type__ = None;
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
                            r#type__ = Some(map.next_value::<payload_to_metadata::ValueType>()? as i32);
                        }
                        GeneratedField::Value => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value_type__ = map.next_value::<::std::option::Option<_>>()?.map(payload_to_metadata::key_value_pair::ValueType::Value);
                        }
                        GeneratedField::RegexValueRewrite => {
                            if value_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("regexValueRewrite"));
                            }
                            value_type__ = map.next_value::<::std::option::Option<_>>()?.map(payload_to_metadata::key_value_pair::ValueType::RegexValueRewrite)
;
                        }
                    }
                }
                Ok(payload_to_metadata::KeyValuePair {
                    metadata_namespace: metadata_namespace__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    value_type: value_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.KeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for payload_to_metadata::Rule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.field_selector.is_some() {
            len += 1;
        }
        if self.on_present.is_some() {
            len += 1;
        }
        if self.on_missing.is_some() {
            len += 1;
        }
        if self.match_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.Rule", len)?;
        if let Some(v) = self.field_selector.as_ref() {
            struct_ser.serialize_field("fieldSelector", v)?;
        }
        if let Some(v) = self.on_present.as_ref() {
            struct_ser.serialize_field("onPresent", v)?;
        }
        if let Some(v) = self.on_missing.as_ref() {
            struct_ser.serialize_field("onMissing", v)?;
        }
        if let Some(v) = self.match_specifier.as_ref() {
            match v {
                payload_to_metadata::rule::MatchSpecifier::MethodName(v) => {
                    struct_ser.serialize_field("methodName", v)?;
                }
                payload_to_metadata::rule::MatchSpecifier::ServiceName(v) => {
                    struct_ser.serialize_field("serviceName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for payload_to_metadata::Rule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field_selector",
            "fieldSelector",
            "on_present",
            "onPresent",
            "on_missing",
            "onMissing",
            "method_name",
            "methodName",
            "service_name",
            "serviceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FieldSelector,
            OnPresent,
            OnMissing,
            MethodName,
            ServiceName,
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
                            "fieldSelector" | "field_selector" => Ok(GeneratedField::FieldSelector),
                            "onPresent" | "on_present" => Ok(GeneratedField::OnPresent),
                            "onMissing" | "on_missing" => Ok(GeneratedField::OnMissing),
                            "methodName" | "method_name" => Ok(GeneratedField::MethodName),
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = payload_to_metadata::Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.Rule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<payload_to_metadata::Rule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field_selector__ = None;
                let mut on_present__ = None;
                let mut on_missing__ = None;
                let mut match_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FieldSelector => {
                            if field_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fieldSelector"));
                            }
                            field_selector__ = map.next_value()?;
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
                        GeneratedField::MethodName => {
                            if match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methodName"));
                            }
                            match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(payload_to_metadata::rule::MatchSpecifier::MethodName);
                        }
                        GeneratedField::ServiceName => {
                            if match_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            match_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(payload_to_metadata::rule::MatchSpecifier::ServiceName);
                        }
                    }
                }
                Ok(payload_to_metadata::Rule {
                    field_selector: field_selector__,
                    on_present: on_present__,
                    on_missing: on_missing__,
                    match_specifier: match_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.thrift_proxy.filters.payload_to_metadata.v3.PayloadToMetadata.Rule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for payload_to_metadata::ValueType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::String => "STRING",
            Self::Number => "NUMBER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for payload_to_metadata::ValueType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STRING",
            "NUMBER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = payload_to_metadata::ValueType;

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
                    .and_then(payload_to_metadata::ValueType::from_i32)
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
                    .and_then(payload_to_metadata::ValueType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STRING" => Ok(payload_to_metadata::ValueType::String),
                    "NUMBER" => Ok(payload_to_metadata::ValueType::Number),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
