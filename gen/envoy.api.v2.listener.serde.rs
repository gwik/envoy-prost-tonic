// @generated
impl serde::Serialize for ActiveRawUdpListenerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.api.v2.listener.ActiveRawUdpListenerConfig", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ActiveRawUdpListenerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ActiveRawUdpListenerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.ActiveRawUdpListenerConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ActiveRawUdpListenerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ActiveRawUdpListenerConfig {
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.ActiveRawUdpListenerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Filter {
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
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.Filter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                filter::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Filter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Config,
            TypedConfig,
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
                            "config" => Ok(GeneratedField::Config),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Filter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.Filter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Filter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(filter::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(filter::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(Filter {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.Filter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter_chain_match.is_some() {
            len += 1;
        }
        if self.tls_context.is_some() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.use_proxy_proto.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if self.transport_socket.is_some() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.FilterChain", len)?;
        if let Some(v) = self.filter_chain_match.as_ref() {
            struct_ser.serialize_field("filterChainMatch", v)?;
        }
        if let Some(v) = self.tls_context.as_ref() {
            struct_ser.serialize_field("tlsContext", v)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if let Some(v) = self.use_proxy_proto.as_ref() {
            struct_ser.serialize_field("useProxyProto", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.transport_socket.as_ref() {
            struct_ser.serialize_field("transportSocket", v)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter_chain_match",
            "filterChainMatch",
            "tls_context",
            "tlsContext",
            "filters",
            "use_proxy_proto",
            "useProxyProto",
            "metadata",
            "transport_socket",
            "transportSocket",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilterChainMatch,
            TlsContext,
            Filters,
            UseProxyProto,
            Metadata,
            TransportSocket,
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
                            "filterChainMatch" | "filter_chain_match" => Ok(GeneratedField::FilterChainMatch),
                            "tlsContext" | "tls_context" => Ok(GeneratedField::TlsContext),
                            "filters" => Ok(GeneratedField::Filters),
                            "useProxyProto" | "use_proxy_proto" => Ok(GeneratedField::UseProxyProto),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "transportSocket" | "transport_socket" => Ok(GeneratedField::TransportSocket),
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
            type Value = FilterChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.FilterChain")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_chain_match__ = None;
                let mut tls_context__ = None;
                let mut filters__ = None;
                let mut use_proxy_proto__ = None;
                let mut metadata__ = None;
                let mut transport_socket__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FilterChainMatch => {
                            if filter_chain_match__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterChainMatch"));
                            }
                            filter_chain_match__ = map.next_value()?;
                        }
                        GeneratedField::TlsContext => {
                            if tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsContext"));
                            }
                            tls_context__ = map.next_value()?;
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseProxyProto => {
                            if use_proxy_proto__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useProxyProto"));
                            }
                            use_proxy_proto__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::TransportSocket => {
                            if transport_socket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocket"));
                            }
                            transport_socket__ = map.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilterChain {
                    filter_chain_match: filter_chain_match__,
                    tls_context: tls_context__,
                    filters: filters__.unwrap_or_default(),
                    use_proxy_proto: use_proxy_proto__,
                    metadata: metadata__,
                    transport_socket: transport_socket__,
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.FilterChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FilterChainMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.destination_port.is_some() {
            len += 1;
        }
        if !self.prefix_ranges.is_empty() {
            len += 1;
        }
        if !self.address_suffix.is_empty() {
            len += 1;
        }
        if self.suffix_len.is_some() {
            len += 1;
        }
        if self.source_type != 0 {
            len += 1;
        }
        if !self.source_prefix_ranges.is_empty() {
            len += 1;
        }
        if !self.source_ports.is_empty() {
            len += 1;
        }
        if !self.server_names.is_empty() {
            len += 1;
        }
        if !self.transport_protocol.is_empty() {
            len += 1;
        }
        if !self.application_protocols.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.FilterChainMatch", len)?;
        if let Some(v) = self.destination_port.as_ref() {
            struct_ser.serialize_field("destinationPort", v)?;
        }
        if !self.prefix_ranges.is_empty() {
            struct_ser.serialize_field("prefixRanges", &self.prefix_ranges)?;
        }
        if !self.address_suffix.is_empty() {
            struct_ser.serialize_field("addressSuffix", &self.address_suffix)?;
        }
        if let Some(v) = self.suffix_len.as_ref() {
            struct_ser.serialize_field("suffixLen", v)?;
        }
        if self.source_type != 0 {
            let v = filter_chain_match::ConnectionSourceType::from_i32(self.source_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.source_type)))?;
            struct_ser.serialize_field("sourceType", &v)?;
        }
        if !self.source_prefix_ranges.is_empty() {
            struct_ser.serialize_field("sourcePrefixRanges", &self.source_prefix_ranges)?;
        }
        if !self.source_ports.is_empty() {
            struct_ser.serialize_field("sourcePorts", &self.source_ports)?;
        }
        if !self.server_names.is_empty() {
            struct_ser.serialize_field("serverNames", &self.server_names)?;
        }
        if !self.transport_protocol.is_empty() {
            struct_ser.serialize_field("transportProtocol", &self.transport_protocol)?;
        }
        if !self.application_protocols.is_empty() {
            struct_ser.serialize_field("applicationProtocols", &self.application_protocols)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FilterChainMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "destination_port",
            "destinationPort",
            "prefix_ranges",
            "prefixRanges",
            "address_suffix",
            "addressSuffix",
            "suffix_len",
            "suffixLen",
            "source_type",
            "sourceType",
            "source_prefix_ranges",
            "sourcePrefixRanges",
            "source_ports",
            "sourcePorts",
            "server_names",
            "serverNames",
            "transport_protocol",
            "transportProtocol",
            "application_protocols",
            "applicationProtocols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DestinationPort,
            PrefixRanges,
            AddressSuffix,
            SuffixLen,
            SourceType,
            SourcePrefixRanges,
            SourcePorts,
            ServerNames,
            TransportProtocol,
            ApplicationProtocols,
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
                            "destinationPort" | "destination_port" => Ok(GeneratedField::DestinationPort),
                            "prefixRanges" | "prefix_ranges" => Ok(GeneratedField::PrefixRanges),
                            "addressSuffix" | "address_suffix" => Ok(GeneratedField::AddressSuffix),
                            "suffixLen" | "suffix_len" => Ok(GeneratedField::SuffixLen),
                            "sourceType" | "source_type" => Ok(GeneratedField::SourceType),
                            "sourcePrefixRanges" | "source_prefix_ranges" => Ok(GeneratedField::SourcePrefixRanges),
                            "sourcePorts" | "source_ports" => Ok(GeneratedField::SourcePorts),
                            "serverNames" | "server_names" => Ok(GeneratedField::ServerNames),
                            "transportProtocol" | "transport_protocol" => Ok(GeneratedField::TransportProtocol),
                            "applicationProtocols" | "application_protocols" => Ok(GeneratedField::ApplicationProtocols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FilterChainMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.FilterChainMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FilterChainMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut destination_port__ = None;
                let mut prefix_ranges__ = None;
                let mut address_suffix__ = None;
                let mut suffix_len__ = None;
                let mut source_type__ = None;
                let mut source_prefix_ranges__ = None;
                let mut source_ports__ = None;
                let mut server_names__ = None;
                let mut transport_protocol__ = None;
                let mut application_protocols__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DestinationPort => {
                            if destination_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPort"));
                            }
                            destination_port__ = map.next_value()?;
                        }
                        GeneratedField::PrefixRanges => {
                            if prefix_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixRanges"));
                            }
                            prefix_ranges__ = Some(map.next_value()?);
                        }
                        GeneratedField::AddressSuffix => {
                            if address_suffix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressSuffix"));
                            }
                            address_suffix__ = Some(map.next_value()?);
                        }
                        GeneratedField::SuffixLen => {
                            if suffix_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suffixLen"));
                            }
                            suffix_len__ = map.next_value()?;
                        }
                        GeneratedField::SourceType => {
                            if source_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceType"));
                            }
                            source_type__ = Some(map.next_value::<filter_chain_match::ConnectionSourceType>()? as i32);
                        }
                        GeneratedField::SourcePrefixRanges => {
                            if source_prefix_ranges__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePrefixRanges"));
                            }
                            source_prefix_ranges__ = Some(map.next_value()?);
                        }
                        GeneratedField::SourcePorts => {
                            if source_ports__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourcePorts"));
                            }
                            source_ports__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ServerNames => {
                            if server_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverNames"));
                            }
                            server_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransportProtocol => {
                            if transport_protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportProtocol"));
                            }
                            transport_protocol__ = Some(map.next_value()?);
                        }
                        GeneratedField::ApplicationProtocols => {
                            if application_protocols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationProtocols"));
                            }
                            application_protocols__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FilterChainMatch {
                    destination_port: destination_port__,
                    prefix_ranges: prefix_ranges__.unwrap_or_default(),
                    address_suffix: address_suffix__.unwrap_or_default(),
                    suffix_len: suffix_len__,
                    source_type: source_type__.unwrap_or_default(),
                    source_prefix_ranges: source_prefix_ranges__.unwrap_or_default(),
                    source_ports: source_ports__.unwrap_or_default(),
                    server_names: server_names__.unwrap_or_default(),
                    transport_protocol: transport_protocol__.unwrap_or_default(),
                    application_protocols: application_protocols__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.FilterChainMatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for filter_chain_match::ConnectionSourceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Any => "ANY",
            Self::Local => "LOCAL",
            Self::External => "EXTERNAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for filter_chain_match::ConnectionSourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ANY",
            "LOCAL",
            "EXTERNAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = filter_chain_match::ConnectionSourceType;

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
                    .and_then(filter_chain_match::ConnectionSourceType::from_i32)
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
                    .and_then(filter_chain_match::ConnectionSourceType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ANY" => Ok(filter_chain_match::ConnectionSourceType::Any),
                    "LOCAL" => Ok(filter_chain_match::ConnectionSourceType::Local),
                    "EXTERNAL" => Ok(filter_chain_match::ConnectionSourceType::External),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ListenerFilter {
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
        if self.filter_disabled.is_some() {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.ListenerFilter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.filter_disabled.as_ref() {
            struct_ser.serialize_field("filterDisabled", v)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                listener_filter::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                listener_filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "filter_disabled",
            "filterDisabled",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            FilterDisabled,
            Config,
            TypedConfig,
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
                            "filterDisabled" | "filter_disabled" => Ok(GeneratedField::FilterDisabled),
                            "config" => Ok(GeneratedField::Config),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListenerFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.ListenerFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListenerFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut filter_disabled__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::FilterDisabled => {
                            if filter_disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterDisabled"));
                            }
                            filter_disabled__ = map.next_value()?;
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(listener_filter::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(listener_filter::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(ListenerFilter {
                    name: name__.unwrap_or_default(),
                    filter_disabled: filter_disabled__,
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.ListenerFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenerFilterChainMatchPredicate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rule.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.ListenerFilterChainMatchPredicate", len)?;
        if let Some(v) = self.rule.as_ref() {
            match v {
                listener_filter_chain_match_predicate::Rule::OrMatch(v) => {
                    struct_ser.serialize_field("orMatch", v)?;
                }
                listener_filter_chain_match_predicate::Rule::AndMatch(v) => {
                    struct_ser.serialize_field("andMatch", v)?;
                }
                listener_filter_chain_match_predicate::Rule::NotMatch(v) => {
                    struct_ser.serialize_field("notMatch", v)?;
                }
                listener_filter_chain_match_predicate::Rule::AnyMatch(v) => {
                    struct_ser.serialize_field("anyMatch", v)?;
                }
                listener_filter_chain_match_predicate::Rule::DestinationPortRange(v) => {
                    struct_ser.serialize_field("destinationPortRange", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerFilterChainMatchPredicate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "or_match",
            "orMatch",
            "and_match",
            "andMatch",
            "not_match",
            "notMatch",
            "any_match",
            "anyMatch",
            "destination_port_range",
            "destinationPortRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OrMatch,
            AndMatch,
            NotMatch,
            AnyMatch,
            DestinationPortRange,
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
                            "orMatch" | "or_match" => Ok(GeneratedField::OrMatch),
                            "andMatch" | "and_match" => Ok(GeneratedField::AndMatch),
                            "notMatch" | "not_match" => Ok(GeneratedField::NotMatch),
                            "anyMatch" | "any_match" => Ok(GeneratedField::AnyMatch),
                            "destinationPortRange" | "destination_port_range" => Ok(GeneratedField::DestinationPortRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListenerFilterChainMatchPredicate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.ListenerFilterChainMatchPredicate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListenerFilterChainMatchPredicate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rule__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OrMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::OrMatch)
;
                        }
                        GeneratedField::AndMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("andMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::AndMatch)
;
                        }
                        GeneratedField::NotMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("notMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::NotMatch)
;
                        }
                        GeneratedField::AnyMatch => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anyMatch"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::AnyMatch);
                        }
                        GeneratedField::DestinationPortRange => {
                            if rule__.is_some() {
                                return Err(serde::de::Error::duplicate_field("destinationPortRange"));
                            }
                            rule__ = map.next_value::<::std::option::Option<_>>()?.map(listener_filter_chain_match_predicate::Rule::DestinationPortRange)
;
                        }
                    }
                }
                Ok(ListenerFilterChainMatchPredicate {
                    rule: rule__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.ListenerFilterChainMatchPredicate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listener_filter_chain_match_predicate::MatchSet {
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
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.ListenerFilterChainMatchPredicate.MatchSet", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listener_filter_chain_match_predicate::MatchSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listener_filter_chain_match_predicate::MatchSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.ListenerFilterChainMatchPredicate.MatchSet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<listener_filter_chain_match_predicate::MatchSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(listener_filter_chain_match_predicate::MatchSet {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.ListenerFilterChainMatchPredicate.MatchSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuicProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_concurrent_streams.is_some() {
            len += 1;
        }
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.crypto_handshake_timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.QuicProtocolOptions", len)?;
        if let Some(v) = self.max_concurrent_streams.as_ref() {
            struct_ser.serialize_field("maxConcurrentStreams", v)?;
        }
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        if let Some(v) = self.crypto_handshake_timeout.as_ref() {
            struct_ser.serialize_field("cryptoHandshakeTimeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuicProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_concurrent_streams",
            "maxConcurrentStreams",
            "idle_timeout",
            "idleTimeout",
            "crypto_handshake_timeout",
            "cryptoHandshakeTimeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxConcurrentStreams,
            IdleTimeout,
            CryptoHandshakeTimeout,
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
                            "maxConcurrentStreams" | "max_concurrent_streams" => Ok(GeneratedField::MaxConcurrentStreams),
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "cryptoHandshakeTimeout" | "crypto_handshake_timeout" => Ok(GeneratedField::CryptoHandshakeTimeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuicProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.QuicProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuicProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_concurrent_streams__ = None;
                let mut idle_timeout__ = None;
                let mut crypto_handshake_timeout__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxConcurrentStreams => {
                            if max_concurrent_streams__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConcurrentStreams"));
                            }
                            max_concurrent_streams__ = map.next_value()?;
                        }
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::CryptoHandshakeTimeout => {
                            if crypto_handshake_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cryptoHandshakeTimeout"));
                            }
                            crypto_handshake_timeout__ = map.next_value()?;
                        }
                    }
                }
                Ok(QuicProtocolOptions {
                    max_concurrent_streams: max_concurrent_streams__,
                    idle_timeout: idle_timeout__,
                    crypto_handshake_timeout: crypto_handshake_timeout__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.QuicProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UdpListenerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.udp_listener_name.is_empty() {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.api.v2.listener.UdpListenerConfig", len)?;
        if !self.udp_listener_name.is_empty() {
            struct_ser.serialize_field("udpListenerName", &self.udp_listener_name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                udp_listener_config::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                udp_listener_config::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UdpListenerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "udp_listener_name",
            "udpListenerName",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UdpListenerName,
            Config,
            TypedConfig,
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
                            "udpListenerName" | "udp_listener_name" => Ok(GeneratedField::UdpListenerName),
                            "config" => Ok(GeneratedField::Config),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UdpListenerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.api.v2.listener.UdpListenerConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UdpListenerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut udp_listener_name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UdpListenerName => {
                            if udp_listener_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("udpListenerName"));
                            }
                            udp_listener_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(udp_listener_config::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(udp_listener_config::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(UdpListenerConfig {
                    udp_listener_name: udp_listener_name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.api.v2.listener.UdpListenerConfig", FIELDS, GeneratedVisitor)
    }
}
