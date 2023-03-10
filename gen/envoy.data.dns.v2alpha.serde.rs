// @generated
impl serde::Serialize for DnsTable {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.external_retry_count != 0 {
            len += 1;
        }
        if !self.virtual_domains.is_empty() {
            len += 1;
        }
        if !self.known_suffixes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v2alpha.DnsTable", len)?;
        if self.external_retry_count != 0 {
            struct_ser.serialize_field("externalRetryCount", &self.external_retry_count)?;
        }
        if !self.virtual_domains.is_empty() {
            struct_ser.serialize_field("virtualDomains", &self.virtual_domains)?;
        }
        if !self.known_suffixes.is_empty() {
            struct_ser.serialize_field("knownSuffixes", &self.known_suffixes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsTable {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "external_retry_count",
            "externalRetryCount",
            "virtual_domains",
            "virtualDomains",
            "known_suffixes",
            "knownSuffixes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExternalRetryCount,
            VirtualDomains,
            KnownSuffixes,
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
                            "externalRetryCount" | "external_retry_count" => Ok(GeneratedField::ExternalRetryCount),
                            "virtualDomains" | "virtual_domains" => Ok(GeneratedField::VirtualDomains),
                            "knownSuffixes" | "known_suffixes" => Ok(GeneratedField::KnownSuffixes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsTable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v2alpha.DnsTable")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsTable, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut external_retry_count__ = None;
                let mut virtual_domains__ = None;
                let mut known_suffixes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExternalRetryCount => {
                            if external_retry_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalRetryCount"));
                            }
                            external_retry_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::VirtualDomains => {
                            if virtual_domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("virtualDomains"));
                            }
                            virtual_domains__ = Some(map.next_value()?);
                        }
                        GeneratedField::KnownSuffixes => {
                            if known_suffixes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("knownSuffixes"));
                            }
                            known_suffixes__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DnsTable {
                    external_retry_count: external_retry_count__.unwrap_or_default(),
                    virtual_domains: virtual_domains__.unwrap_or_default(),
                    known_suffixes: known_suffixes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v2alpha.DnsTable", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_table::AddressList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v2alpha.DnsTable.AddressList", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_table::AddressList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_table::AddressList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v2alpha.DnsTable.AddressList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_table::AddressList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(dns_table::AddressList {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v2alpha.DnsTable.AddressList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_table::DnsEndpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v2alpha.DnsTable.DnsEndpoint", len)?;
        if let Some(v) = self.endpoint_config.as_ref() {
            match v {
                dns_table::dns_endpoint::EndpointConfig::AddressList(v) => {
                    struct_ser.serialize_field("addressList", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_table::DnsEndpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address_list",
            "addressList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddressList,
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
                            "addressList" | "address_list" => Ok(GeneratedField::AddressList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_table::DnsEndpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v2alpha.DnsTable.DnsEndpoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_table::DnsEndpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AddressList => {
                            if endpoint_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressList"));
                            }
                            endpoint_config__ = map.next_value::<::std::option::Option<_>>()?.map(dns_table::dns_endpoint::EndpointConfig::AddressList)
;
                        }
                    }
                }
                Ok(dns_table::DnsEndpoint {
                    endpoint_config: endpoint_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v2alpha.DnsTable.DnsEndpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_table::DnsVirtualDomain {
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
        if self.endpoint.is_some() {
            len += 1;
        }
        if self.answer_ttl.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v2alpha.DnsTable.DnsVirtualDomain", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
        }
        if let Some(v) = self.answer_ttl.as_ref() {
            struct_ser.serialize_field("answerTtl", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_table::DnsVirtualDomain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "endpoint",
            "answer_ttl",
            "answerTtl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Endpoint,
            AnswerTtl,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "answerTtl" | "answer_ttl" => Ok(GeneratedField::AnswerTtl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_table::DnsVirtualDomain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v2alpha.DnsTable.DnsVirtualDomain")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_table::DnsVirtualDomain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut endpoint__ = None;
                let mut answer_ttl__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map.next_value()?;
                        }
                        GeneratedField::AnswerTtl => {
                            if answer_ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("answerTtl"));
                            }
                            answer_ttl__ = map.next_value()?;
                        }
                    }
                }
                Ok(dns_table::DnsVirtualDomain {
                    name: name__.unwrap_or_default(),
                    endpoint: endpoint__,
                    answer_ttl: answer_ttl__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v2alpha.DnsTable.DnsVirtualDomain", FIELDS, GeneratedVisitor)
    }
}
