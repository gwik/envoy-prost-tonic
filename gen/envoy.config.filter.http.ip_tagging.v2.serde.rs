// @generated
impl serde::Serialize for IpTagging {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_type != 0 {
            len += 1;
        }
        if !self.ip_tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ip_tagging.v2.IPTagging", len)?;
        if self.request_type != 0 {
            let v = ip_tagging::RequestType::from_i32(self.request_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.request_type)))?;
            struct_ser.serialize_field("requestType", &v)?;
        }
        if !self.ip_tags.is_empty() {
            struct_ser.serialize_field("ipTags", &self.ip_tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IpTagging {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_type",
            "requestType",
            "ip_tags",
            "ipTags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestType,
            IpTags,
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
                            "requestType" | "request_type" => Ok(GeneratedField::RequestType),
                            "ipTags" | "ip_tags" => Ok(GeneratedField::IpTags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IpTagging;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ip_tagging.v2.IPTagging")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IpTagging, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_type__ = None;
                let mut ip_tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestType => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestType"));
                            }
                            request_type__ = Some(map.next_value::<ip_tagging::RequestType>()? as i32);
                        }
                        GeneratedField::IpTags => {
                            if ip_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipTags"));
                            }
                            ip_tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IpTagging {
                    request_type: request_type__.unwrap_or_default(),
                    ip_tags: ip_tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ip_tagging.v2.IPTagging", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ip_tagging::IpTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ip_tag_name.is_empty() {
            len += 1;
        }
        if !self.ip_list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ip_tagging.v2.IPTagging.IPTag", len)?;
        if !self.ip_tag_name.is_empty() {
            struct_ser.serialize_field("ipTagName", &self.ip_tag_name)?;
        }
        if !self.ip_list.is_empty() {
            struct_ser.serialize_field("ipList", &self.ip_list)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ip_tagging::IpTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ip_tag_name",
            "ipTagName",
            "ip_list",
            "ipList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IpTagName,
            IpList,
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
                            "ipTagName" | "ip_tag_name" => Ok(GeneratedField::IpTagName),
                            "ipList" | "ip_list" => Ok(GeneratedField::IpList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ip_tagging::IpTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ip_tagging.v2.IPTagging.IPTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ip_tagging::IpTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ip_tag_name__ = None;
                let mut ip_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IpTagName => {
                            if ip_tag_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipTagName"));
                            }
                            ip_tag_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::IpList => {
                            if ip_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipList"));
                            }
                            ip_list__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ip_tagging::IpTag {
                    ip_tag_name: ip_tag_name__.unwrap_or_default(),
                    ip_list: ip_list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ip_tagging.v2.IPTagging.IPTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ip_tagging::RequestType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Both => "BOTH",
            Self::Internal => "INTERNAL",
            Self::External => "EXTERNAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ip_tagging::RequestType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BOTH",
            "INTERNAL",
            "EXTERNAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ip_tagging::RequestType;

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
                    .and_then(ip_tagging::RequestType::from_i32)
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
                    .and_then(ip_tagging::RequestType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "BOTH" => Ok(ip_tagging::RequestType::Both),
                    "INTERNAL" => Ok(ip_tagging::RequestType::Internal),
                    "EXTERNAL" => Ok(ip_tagging::RequestType::External),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
