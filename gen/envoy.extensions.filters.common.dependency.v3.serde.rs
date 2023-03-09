// @generated
impl serde::Serialize for Dependency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.dependency.v3.Dependency", len)?;
        if self.r#type != 0 {
            let v = dependency::DependencyType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dependency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Name,
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
                            "type" => Ok(GeneratedField::Type),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Dependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.dependency.v3.Dependency")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Dependency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<dependency::DependencyType>()? as i32);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Dependency {
                    r#type: r#type__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.dependency.v3.Dependency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dependency::DependencyType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Header => "HEADER",
            Self::FilterStateKey => "FILTER_STATE_KEY",
            Self::DynamicMetadata => "DYNAMIC_METADATA",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for dependency::DependencyType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HEADER",
            "FILTER_STATE_KEY",
            "DYNAMIC_METADATA",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dependency::DependencyType;

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
                    .and_then(dependency::DependencyType::from_i32)
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
                    .and_then(dependency::DependencyType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HEADER" => Ok(dependency::DependencyType::Header),
                    "FILTER_STATE_KEY" => Ok(dependency::DependencyType::FilterStateKey),
                    "DYNAMIC_METADATA" => Ok(dependency::DependencyType::DynamicMetadata),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for FilterDependencies {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.decode_required.is_empty() {
            len += 1;
        }
        if !self.decode_provided.is_empty() {
            len += 1;
        }
        if !self.encode_required.is_empty() {
            len += 1;
        }
        if !self.encode_provided.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.dependency.v3.FilterDependencies", len)?;
        if !self.decode_required.is_empty() {
            struct_ser.serialize_field("decodeRequired", &self.decode_required)?;
        }
        if !self.decode_provided.is_empty() {
            struct_ser.serialize_field("decodeProvided", &self.decode_provided)?;
        }
        if !self.encode_required.is_empty() {
            struct_ser.serialize_field("encodeRequired", &self.encode_required)?;
        }
        if !self.encode_provided.is_empty() {
            struct_ser.serialize_field("encodeProvided", &self.encode_provided)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterDependencies {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decode_required",
            "decodeRequired",
            "decode_provided",
            "decodeProvided",
            "encode_required",
            "encodeRequired",
            "encode_provided",
            "encodeProvided",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DecodeRequired,
            DecodeProvided,
            EncodeRequired,
            EncodeProvided,
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
                            "decodeRequired" | "decode_required" => Ok(GeneratedField::DecodeRequired),
                            "decodeProvided" | "decode_provided" => Ok(GeneratedField::DecodeProvided),
                            "encodeRequired" | "encode_required" => Ok(GeneratedField::EncodeRequired),
                            "encodeProvided" | "encode_provided" => Ok(GeneratedField::EncodeProvided),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterDependencies;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.dependency.v3.FilterDependencies")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterDependencies, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decode_required__ = None;
                let mut decode_provided__ = None;
                let mut encode_required__ = None;
                let mut encode_provided__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DecodeRequired => {
                            if decode_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decodeRequired"));
                            }
                            decode_required__ = Some(map.next_value()?);
                        }
                        GeneratedField::DecodeProvided => {
                            if decode_provided__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decodeProvided"));
                            }
                            decode_provided__ = Some(map.next_value()?);
                        }
                        GeneratedField::EncodeRequired => {
                            if encode_required__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encodeRequired"));
                            }
                            encode_required__ = Some(map.next_value()?);
                        }
                        GeneratedField::EncodeProvided => {
                            if encode_provided__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encodeProvided"));
                            }
                            encode_provided__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilterDependencies {
                    decode_required: decode_required__.unwrap_or_default(),
                    decode_provided: decode_provided__.unwrap_or_default(),
                    encode_required: encode_required__.unwrap_or_default(),
                    encode_provided: encode_provided__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.dependency.v3.FilterDependencies", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MatchingRequirements {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.data_input_allow_list.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.dependency.v3.MatchingRequirements", len)?;
        if let Some(v) = self.data_input_allow_list.as_ref() {
            struct_ser.serialize_field("dataInputAllowList", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MatchingRequirements {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data_input_allow_list",
            "dataInputAllowList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DataInputAllowList,
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
                            "dataInputAllowList" | "data_input_allow_list" => Ok(GeneratedField::DataInputAllowList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MatchingRequirements;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.dependency.v3.MatchingRequirements")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MatchingRequirements, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data_input_allow_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DataInputAllowList => {
                            if data_input_allow_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataInputAllowList"));
                            }
                            data_input_allow_list__ = map.next_value()?;
                        }
                    }
                }
                Ok(MatchingRequirements {
                    data_input_allow_list: data_input_allow_list__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.dependency.v3.MatchingRequirements", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for matching_requirements::DataInputAllowList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.type_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.dependency.v3.MatchingRequirements.DataInputAllowList", len)?;
        if !self.type_url.is_empty() {
            struct_ser.serialize_field("typeUrl", &self.type_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for matching_requirements::DataInputAllowList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type_url",
            "typeUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeUrl,
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
                            "typeUrl" | "type_url" => Ok(GeneratedField::TypeUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = matching_requirements::DataInputAllowList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.dependency.v3.MatchingRequirements.DataInputAllowList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<matching_requirements::DataInputAllowList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_url__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TypeUrl => {
                            if type_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrl"));
                            }
                            type_url__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(matching_requirements::DataInputAllowList {
                    type_url: type_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.dependency.v3.MatchingRequirements.DataInputAllowList", FIELDS, GeneratedVisitor)
    }
}
