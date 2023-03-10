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
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable", len)?;
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
                formatter.write_str("struct envoy.data.dns.v3.DnsTable")
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
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable.AddressList", len)?;
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
                formatter.write_str("struct envoy.data.dns.v3.DnsTable.AddressList")
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
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable.AddressList", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable.DnsEndpoint", len)?;
        if let Some(v) = self.endpoint_config.as_ref() {
            match v {
                dns_table::dns_endpoint::EndpointConfig::AddressList(v) => {
                    struct_ser.serialize_field("addressList", v)?;
                }
                dns_table::dns_endpoint::EndpointConfig::ClusterName(v) => {
                    struct_ser.serialize_field("clusterName", v)?;
                }
                dns_table::dns_endpoint::EndpointConfig::ServiceList(v) => {
                    struct_ser.serialize_field("serviceList", v)?;
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
            "cluster_name",
            "clusterName",
            "service_list",
            "serviceList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddressList,
            ClusterName,
            ServiceList,
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
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "serviceList" | "service_list" => Ok(GeneratedField::ServiceList),
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
                formatter.write_str("struct envoy.data.dns.v3.DnsTable.DnsEndpoint")
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
                        GeneratedField::ClusterName => {
                            if endpoint_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            endpoint_config__ = map.next_value::<::std::option::Option<_>>()?.map(dns_table::dns_endpoint::EndpointConfig::ClusterName);
                        }
                        GeneratedField::ServiceList => {
                            if endpoint_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceList"));
                            }
                            endpoint_config__ = map.next_value::<::std::option::Option<_>>()?.map(dns_table::dns_endpoint::EndpointConfig::ServiceList)
;
                        }
                    }
                }
                Ok(dns_table::DnsEndpoint {
                    endpoint_config: endpoint_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable.DnsEndpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_table::DnsService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if self.protocol.is_some() {
            len += 1;
        }
        if self.ttl.is_some() {
            len += 1;
        }
        if !self.targets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable.DnsService", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if let Some(v) = self.protocol.as_ref() {
            struct_ser.serialize_field("protocol", v)?;
        }
        if let Some(v) = self.ttl.as_ref() {
            struct_ser.serialize_field("ttl", v)?;
        }
        if !self.targets.is_empty() {
            struct_ser.serialize_field("targets", &self.targets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_table::DnsService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "protocol",
            "ttl",
            "targets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            Protocol,
            Ttl,
            Targets,
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
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "protocol" => Ok(GeneratedField::Protocol),
                            "ttl" => Ok(GeneratedField::Ttl),
                            "targets" => Ok(GeneratedField::Targets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_table::DnsService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v3.DnsTable.DnsService")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_table::DnsService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut protocol__ = None;
                let mut ttl__ = None;
                let mut targets__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = map.next_value()?;
                        }
                        GeneratedField::Ttl => {
                            if ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttl"));
                            }
                            ttl__ = map.next_value()?;
                        }
                        GeneratedField::Targets => {
                            if targets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targets"));
                            }
                            targets__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(dns_table::DnsService {
                    service_name: service_name__.unwrap_or_default(),
                    protocol: protocol__,
                    ttl: ttl__,
                    targets: targets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable.DnsService", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_table::DnsServiceList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.services.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable.DnsServiceList", len)?;
        if !self.services.is_empty() {
            struct_ser.serialize_field("services", &self.services)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_table::DnsServiceList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "services",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Services,
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
                            "services" => Ok(GeneratedField::Services),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_table::DnsServiceList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v3.DnsTable.DnsServiceList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_table::DnsServiceList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut services__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Services => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("services"));
                            }
                            services__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(dns_table::DnsServiceList {
                    services: services__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable.DnsServiceList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_table::DnsServiceProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable.DnsServiceProtocol", len)?;
        if let Some(v) = self.protocol_config.as_ref() {
            match v {
                dns_table::dns_service_protocol::ProtocolConfig::Number(v) => {
                    struct_ser.serialize_field("number", v)?;
                }
                dns_table::dns_service_protocol::ProtocolConfig::Name(v) => {
                    struct_ser.serialize_field("name", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_table::DnsServiceProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "number",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Number,
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
                            "number" => Ok(GeneratedField::Number),
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
            type Value = dns_table::DnsServiceProtocol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v3.DnsTable.DnsServiceProtocol")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_table::DnsServiceProtocol, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Number => {
                            if protocol_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            protocol_config__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| dns_table::dns_service_protocol::ProtocolConfig::Number(x.0));
                        }
                        GeneratedField::Name => {
                            if protocol_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            protocol_config__ = map.next_value::<::std::option::Option<_>>()?.map(dns_table::dns_service_protocol::ProtocolConfig::Name);
                        }
                    }
                }
                Ok(dns_table::DnsServiceProtocol {
                    protocol_config: protocol_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable.DnsServiceProtocol", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for dns_table::DnsServiceTarget {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.priority != 0 {
            len += 1;
        }
        if self.weight != 0 {
            len += 1;
        }
        if self.port != 0 {
            len += 1;
        }
        if self.endpoint_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable.DnsServiceTarget", len)?;
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if self.port != 0 {
            struct_ser.serialize_field("port", &self.port)?;
        }
        if let Some(v) = self.endpoint_type.as_ref() {
            match v {
                dns_table::dns_service_target::EndpointType::HostName(v) => {
                    struct_ser.serialize_field("hostName", v)?;
                }
                dns_table::dns_service_target::EndpointType::ClusterName(v) => {
                    struct_ser.serialize_field("clusterName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for dns_table::DnsServiceTarget {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "priority",
            "weight",
            "port",
            "host_name",
            "hostName",
            "cluster_name",
            "clusterName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Priority,
            Weight,
            Port,
            HostName,
            ClusterName,
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
                            "priority" => Ok(GeneratedField::Priority),
                            "weight" => Ok(GeneratedField::Weight),
                            "port" => Ok(GeneratedField::Port),
                            "hostName" | "host_name" => Ok(GeneratedField::HostName),
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = dns_table::DnsServiceTarget;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.dns.v3.DnsTable.DnsServiceTarget")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<dns_table::DnsServiceTarget, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut priority__ = None;
                let mut weight__ = None;
                let mut port__ = None;
                let mut endpoint_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HostName => {
                            if endpoint_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostName"));
                            }
                            endpoint_type__ = map.next_value::<::std::option::Option<_>>()?.map(dns_table::dns_service_target::EndpointType::HostName);
                        }
                        GeneratedField::ClusterName => {
                            if endpoint_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            endpoint_type__ = map.next_value::<::std::option::Option<_>>()?.map(dns_table::dns_service_target::EndpointType::ClusterName);
                        }
                    }
                }
                Ok(dns_table::DnsServiceTarget {
                    priority: priority__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    port: port__.unwrap_or_default(),
                    endpoint_type: endpoint_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable.DnsServiceTarget", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.data.dns.v3.DnsTable.DnsVirtualDomain", len)?;
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
                formatter.write_str("struct envoy.data.dns.v3.DnsTable.DnsVirtualDomain")
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
        deserializer.deserialize_struct("envoy.data.dns.v3.DnsTable.DnsVirtualDomain", FIELDS, GeneratedVisitor)
    }
}
