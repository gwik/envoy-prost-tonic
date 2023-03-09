// @generated
impl serde::Serialize for ProxyProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        if self.allow_requests_without_proxy_protocol {
            len += 1;
        }
        if self.pass_through_tlvs.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        if self.allow_requests_without_proxy_protocol {
            struct_ser.serialize_field("allowRequestsWithoutProxyProtocol", &self.allow_requests_without_proxy_protocol)?;
        }
        if let Some(v) = self.pass_through_tlvs.as_ref() {
            struct_ser.serialize_field("passThroughTlvs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProxyProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
            "allow_requests_without_proxy_protocol",
            "allowRequestsWithoutProxyProtocol",
            "pass_through_tlvs",
            "passThroughTlvs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
            AllowRequestsWithoutProxyProtocol,
            PassThroughTlvs,
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
                            "rules" => Ok(GeneratedField::Rules),
                            "allowRequestsWithoutProxyProtocol" | "allow_requests_without_proxy_protocol" => Ok(GeneratedField::AllowRequestsWithoutProxyProtocol),
                            "passThroughTlvs" | "pass_through_tlvs" => Ok(GeneratedField::PassThroughTlvs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProxyProtocol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProxyProtocol, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                let mut allow_requests_without_proxy_protocol__ = None;
                let mut pass_through_tlvs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowRequestsWithoutProxyProtocol => {
                            if allow_requests_without_proxy_protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowRequestsWithoutProxyProtocol"));
                            }
                            allow_requests_without_proxy_protocol__ = Some(map.next_value()?);
                        }
                        GeneratedField::PassThroughTlvs => {
                            if pass_through_tlvs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passThroughTlvs"));
                            }
                            pass_through_tlvs__ = map.next_value()?;
                        }
                    }
                }
                Ok(ProxyProtocol {
                    rules: rules__.unwrap_or_default(),
                    allow_requests_without_proxy_protocol: allow_requests_without_proxy_protocol__.unwrap_or_default(),
                    pass_through_tlvs: pass_through_tlvs__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for proxy_protocol::KeyValuePair {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol.KeyValuePair", len)?;
        if !self.metadata_namespace.is_empty() {
            struct_ser.serialize_field("metadataNamespace", &self.metadata_namespace)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for proxy_protocol::KeyValuePair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata_namespace",
            "metadataNamespace",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MetadataNamespace,
            Key,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = proxy_protocol::KeyValuePair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol.KeyValuePair")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<proxy_protocol::KeyValuePair, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata_namespace__ = None;
                let mut key__ = None;
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
                    }
                }
                Ok(proxy_protocol::KeyValuePair {
                    metadata_namespace: metadata_namespace__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol.KeyValuePair", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for proxy_protocol::Rule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tlv_type != 0 {
            len += 1;
        }
        if self.on_tlv_present.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol.Rule", len)?;
        if self.tlv_type != 0 {
            struct_ser.serialize_field("tlvType", &self.tlv_type)?;
        }
        if let Some(v) = self.on_tlv_present.as_ref() {
            struct_ser.serialize_field("onTlvPresent", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for proxy_protocol::Rule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tlv_type",
            "tlvType",
            "on_tlv_present",
            "onTlvPresent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TlvType,
            OnTlvPresent,
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
                            "tlvType" | "tlv_type" => Ok(GeneratedField::TlvType),
                            "onTlvPresent" | "on_tlv_present" => Ok(GeneratedField::OnTlvPresent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = proxy_protocol::Rule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol.Rule")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<proxy_protocol::Rule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tlv_type__ = None;
                let mut on_tlv_present__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TlvType => {
                            if tlv_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlvType"));
                            }
                            tlv_type__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OnTlvPresent => {
                            if on_tlv_present__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onTlvPresent"));
                            }
                            on_tlv_present__ = map.next_value()?;
                        }
                    }
                }
                Ok(proxy_protocol::Rule {
                    tlv_type: tlv_type__.unwrap_or_default(),
                    on_tlv_present: on_tlv_present__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.listener.proxy_protocol.v3.ProxyProtocol.Rule", FIELDS, GeneratedVisitor)
    }
}
