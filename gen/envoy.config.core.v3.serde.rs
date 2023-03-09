// @generated
impl serde::Serialize for Address {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Address", len)?;
        if let Some(v) = self.address.as_ref() {
            match v {
                address::Address::SocketAddress(v) => {
                    struct_ser.serialize_field("socketAddress", v)?;
                }
                address::Address::Pipe(v) => {
                    struct_ser.serialize_field("pipe", v)?;
                }
                address::Address::EnvoyInternalAddress(v) => {
                    struct_ser.serialize_field("envoyInternalAddress", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Address {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "socket_address",
            "socketAddress",
            "pipe",
            "envoy_internal_address",
            "envoyInternalAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SocketAddress,
            Pipe,
            EnvoyInternalAddress,
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
                            "socketAddress" | "socket_address" => Ok(GeneratedField::SocketAddress),
                            "pipe" => Ok(GeneratedField::Pipe),
                            "envoyInternalAddress" | "envoy_internal_address" => Ok(GeneratedField::EnvoyInternalAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Address")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Address, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SocketAddress => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketAddress"));
                            }
                            address__ = map.next_value::<::std::option::Option<_>>()?.map(address::Address::SocketAddress)
;
                        }
                        GeneratedField::Pipe => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pipe"));
                            }
                            address__ = map.next_value::<::std::option::Option<_>>()?.map(address::Address::Pipe)
;
                        }
                        GeneratedField::EnvoyInternalAddress => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("envoyInternalAddress"));
                            }
                            address__ = map.next_value::<::std::option::Option<_>>()?.map(address::Address::EnvoyInternalAddress)
;
                        }
                    }
                }
                Ok(Address {
                    address: address__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Address", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AggregatedConfigSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.core.v3.AggregatedConfigSource", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggregatedConfigSource {
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
            type Value = AggregatedConfigSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.AggregatedConfigSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AggregatedConfigSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AggregatedConfigSource {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.AggregatedConfigSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AlternateProtocolsCacheOptions {
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
        if self.max_entries.is_some() {
            len += 1;
        }
        if self.key_value_store_config.is_some() {
            len += 1;
        }
        if !self.prepopulated_entries.is_empty() {
            len += 1;
        }
        if !self.canonical_suffixes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.AlternateProtocolsCacheOptions", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.max_entries.as_ref() {
            struct_ser.serialize_field("maxEntries", v)?;
        }
        if let Some(v) = self.key_value_store_config.as_ref() {
            struct_ser.serialize_field("keyValueStoreConfig", v)?;
        }
        if !self.prepopulated_entries.is_empty() {
            struct_ser.serialize_field("prepopulatedEntries", &self.prepopulated_entries)?;
        }
        if !self.canonical_suffixes.is_empty() {
            struct_ser.serialize_field("canonicalSuffixes", &self.canonical_suffixes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AlternateProtocolsCacheOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "max_entries",
            "maxEntries",
            "key_value_store_config",
            "keyValueStoreConfig",
            "prepopulated_entries",
            "prepopulatedEntries",
            "canonical_suffixes",
            "canonicalSuffixes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            MaxEntries,
            KeyValueStoreConfig,
            PrepopulatedEntries,
            CanonicalSuffixes,
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
                            "maxEntries" | "max_entries" => Ok(GeneratedField::MaxEntries),
                            "keyValueStoreConfig" | "key_value_store_config" => Ok(GeneratedField::KeyValueStoreConfig),
                            "prepopulatedEntries" | "prepopulated_entries" => Ok(GeneratedField::PrepopulatedEntries),
                            "canonicalSuffixes" | "canonical_suffixes" => Ok(GeneratedField::CanonicalSuffixes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AlternateProtocolsCacheOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.AlternateProtocolsCacheOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AlternateProtocolsCacheOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut max_entries__ = None;
                let mut key_value_store_config__ = None;
                let mut prepopulated_entries__ = None;
                let mut canonical_suffixes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxEntries => {
                            if max_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEntries"));
                            }
                            max_entries__ = map.next_value()?;
                        }
                        GeneratedField::KeyValueStoreConfig => {
                            if key_value_store_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyValueStoreConfig"));
                            }
                            key_value_store_config__ = map.next_value()?;
                        }
                        GeneratedField::PrepopulatedEntries => {
                            if prepopulated_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prepopulatedEntries"));
                            }
                            prepopulated_entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::CanonicalSuffixes => {
                            if canonical_suffixes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canonicalSuffixes"));
                            }
                            canonical_suffixes__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AlternateProtocolsCacheOptions {
                    name: name__.unwrap_or_default(),
                    max_entries: max_entries__,
                    key_value_store_config: key_value_store_config__,
                    prepopulated_entries: prepopulated_entries__.unwrap_or_default(),
                    canonical_suffixes: canonical_suffixes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.AlternateProtocolsCacheOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for alternate_protocols_cache_options::AlternateProtocolsCacheEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hostname.is_empty() {
            len += 1;
        }
        if self.port != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.AlternateProtocolsCacheOptions.AlternateProtocolsCacheEntry", len)?;
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        if self.port != 0 {
            struct_ser.serialize_field("port", &self.port)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for alternate_protocols_cache_options::AlternateProtocolsCacheEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hostname",
            "port",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hostname,
            Port,
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
                            "hostname" => Ok(GeneratedField::Hostname),
                            "port" => Ok(GeneratedField::Port),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = alternate_protocols_cache_options::AlternateProtocolsCacheEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.AlternateProtocolsCacheOptions.AlternateProtocolsCacheEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<alternate_protocols_cache_options::AlternateProtocolsCacheEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hostname__ = None;
                let mut port__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map.next_value()?);
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(alternate_protocols_cache_options::AlternateProtocolsCacheEntry {
                    hostname: hostname__.unwrap_or_default(),
                    port: port__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.AlternateProtocolsCacheOptions.AlternateProtocolsCacheEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ApiConfigSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.api_type != 0 {
            len += 1;
        }
        if self.transport_api_version != 0 {
            len += 1;
        }
        if !self.cluster_names.is_empty() {
            len += 1;
        }
        if !self.grpc_services.is_empty() {
            len += 1;
        }
        if self.refresh_delay.is_some() {
            len += 1;
        }
        if self.request_timeout.is_some() {
            len += 1;
        }
        if self.rate_limit_settings.is_some() {
            len += 1;
        }
        if self.set_node_on_first_message_only {
            len += 1;
        }
        if !self.config_validators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.ApiConfigSource", len)?;
        if self.api_type != 0 {
            let v = api_config_source::ApiType::from_i32(self.api_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.api_type)))?;
            struct_ser.serialize_field("apiType", &v)?;
        }
        if self.transport_api_version != 0 {
            let v = ApiVersion::from_i32(self.transport_api_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.transport_api_version)))?;
            struct_ser.serialize_field("transportApiVersion", &v)?;
        }
        if !self.cluster_names.is_empty() {
            struct_ser.serialize_field("clusterNames", &self.cluster_names)?;
        }
        if !self.grpc_services.is_empty() {
            struct_ser.serialize_field("grpcServices", &self.grpc_services)?;
        }
        if let Some(v) = self.refresh_delay.as_ref() {
            struct_ser.serialize_field("refreshDelay", v)?;
        }
        if let Some(v) = self.request_timeout.as_ref() {
            struct_ser.serialize_field("requestTimeout", v)?;
        }
        if let Some(v) = self.rate_limit_settings.as_ref() {
            struct_ser.serialize_field("rateLimitSettings", v)?;
        }
        if self.set_node_on_first_message_only {
            struct_ser.serialize_field("setNodeOnFirstMessageOnly", &self.set_node_on_first_message_only)?;
        }
        if !self.config_validators.is_empty() {
            struct_ser.serialize_field("configValidators", &self.config_validators)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiConfigSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "api_type",
            "apiType",
            "transport_api_version",
            "transportApiVersion",
            "cluster_names",
            "clusterNames",
            "grpc_services",
            "grpcServices",
            "refresh_delay",
            "refreshDelay",
            "request_timeout",
            "requestTimeout",
            "rate_limit_settings",
            "rateLimitSettings",
            "set_node_on_first_message_only",
            "setNodeOnFirstMessageOnly",
            "config_validators",
            "configValidators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ApiType,
            TransportApiVersion,
            ClusterNames,
            GrpcServices,
            RefreshDelay,
            RequestTimeout,
            RateLimitSettings,
            SetNodeOnFirstMessageOnly,
            ConfigValidators,
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
                            "apiType" | "api_type" => Ok(GeneratedField::ApiType),
                            "transportApiVersion" | "transport_api_version" => Ok(GeneratedField::TransportApiVersion),
                            "clusterNames" | "cluster_names" => Ok(GeneratedField::ClusterNames),
                            "grpcServices" | "grpc_services" => Ok(GeneratedField::GrpcServices),
                            "refreshDelay" | "refresh_delay" => Ok(GeneratedField::RefreshDelay),
                            "requestTimeout" | "request_timeout" => Ok(GeneratedField::RequestTimeout),
                            "rateLimitSettings" | "rate_limit_settings" => Ok(GeneratedField::RateLimitSettings),
                            "setNodeOnFirstMessageOnly" | "set_node_on_first_message_only" => Ok(GeneratedField::SetNodeOnFirstMessageOnly),
                            "configValidators" | "config_validators" => Ok(GeneratedField::ConfigValidators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiConfigSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.ApiConfigSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiConfigSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut api_type__ = None;
                let mut transport_api_version__ = None;
                let mut cluster_names__ = None;
                let mut grpc_services__ = None;
                let mut refresh_delay__ = None;
                let mut request_timeout__ = None;
                let mut rate_limit_settings__ = None;
                let mut set_node_on_first_message_only__ = None;
                let mut config_validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ApiType => {
                            if api_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiType"));
                            }
                            api_type__ = Some(map.next_value::<api_config_source::ApiType>()? as i32);
                        }
                        GeneratedField::TransportApiVersion => {
                            if transport_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportApiVersion"));
                            }
                            transport_api_version__ = Some(map.next_value::<ApiVersion>()? as i32);
                        }
                        GeneratedField::ClusterNames => {
                            if cluster_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterNames"));
                            }
                            cluster_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::GrpcServices => {
                            if grpc_services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcServices"));
                            }
                            grpc_services__ = Some(map.next_value()?);
                        }
                        GeneratedField::RefreshDelay => {
                            if refresh_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshDelay"));
                            }
                            refresh_delay__ = map.next_value()?;
                        }
                        GeneratedField::RequestTimeout => {
                            if request_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTimeout"));
                            }
                            request_timeout__ = map.next_value()?;
                        }
                        GeneratedField::RateLimitSettings => {
                            if rate_limit_settings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitSettings"));
                            }
                            rate_limit_settings__ = map.next_value()?;
                        }
                        GeneratedField::SetNodeOnFirstMessageOnly => {
                            if set_node_on_first_message_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setNodeOnFirstMessageOnly"));
                            }
                            set_node_on_first_message_only__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConfigValidators => {
                            if config_validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configValidators"));
                            }
                            config_validators__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ApiConfigSource {
                    api_type: api_type__.unwrap_or_default(),
                    transport_api_version: transport_api_version__.unwrap_or_default(),
                    cluster_names: cluster_names__.unwrap_or_default(),
                    grpc_services: grpc_services__.unwrap_or_default(),
                    refresh_delay: refresh_delay__,
                    request_timeout: request_timeout__,
                    rate_limit_settings: rate_limit_settings__,
                    set_node_on_first_message_only: set_node_on_first_message_only__.unwrap_or_default(),
                    config_validators: config_validators__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.ApiConfigSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for api_config_source::ApiType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DeprecatedAndUnavailableDoNotUse => "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE",
            Self::Rest => "REST",
            Self::Grpc => "GRPC",
            Self::DeltaGrpc => "DELTA_GRPC",
            Self::AggregatedGrpc => "AGGREGATED_GRPC",
            Self::AggregatedDeltaGrpc => "AGGREGATED_DELTA_GRPC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for api_config_source::ApiType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE",
            "REST",
            "GRPC",
            "DELTA_GRPC",
            "AGGREGATED_GRPC",
            "AGGREGATED_DELTA_GRPC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = api_config_source::ApiType;

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
                    .and_then(api_config_source::ApiType::from_i32)
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
                    .and_then(api_config_source::ApiType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE" => Ok(api_config_source::ApiType::DeprecatedAndUnavailableDoNotUse),
                    "REST" => Ok(api_config_source::ApiType::Rest),
                    "GRPC" => Ok(api_config_source::ApiType::Grpc),
                    "DELTA_GRPC" => Ok(api_config_source::ApiType::DeltaGrpc),
                    "AGGREGATED_GRPC" => Ok(api_config_source::ApiType::AggregatedGrpc),
                    "AGGREGATED_DELTA_GRPC" => Ok(api_config_source::ApiType::AggregatedDeltaGrpc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ApiVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Auto => "AUTO",
            Self::V2 => "V2",
            Self::V3 => "V3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ApiVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO",
            "V2",
            "V3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiVersion;

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
                    .and_then(ApiVersion::from_i32)
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
                    .and_then(ApiVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AUTO" => Ok(ApiVersion::Auto),
                    "V2" => Ok(ApiVersion::V2),
                    "V3" => Ok(ApiVersion::V3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AsyncDataSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.AsyncDataSource", len)?;
        if let Some(v) = self.specifier.as_ref() {
            match v {
                async_data_source::Specifier::Local(v) => {
                    struct_ser.serialize_field("local", v)?;
                }
                async_data_source::Specifier::Remote(v) => {
                    struct_ser.serialize_field("remote", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AsyncDataSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local",
            "remote",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Local,
            Remote,
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
                            "local" => Ok(GeneratedField::Local),
                            "remote" => Ok(GeneratedField::Remote),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AsyncDataSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.AsyncDataSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AsyncDataSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Local => {
                            if specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("local"));
                            }
                            specifier__ = map.next_value::<::std::option::Option<_>>()?.map(async_data_source::Specifier::Local)
;
                        }
                        GeneratedField::Remote => {
                            if specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remote"));
                            }
                            specifier__ = map.next_value::<::std::option::Option<_>>()?.map(async_data_source::Specifier::Remote)
;
                        }
                    }
                }
                Ok(AsyncDataSource {
                    specifier: specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.AsyncDataSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BackoffStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_interval.is_some() {
            len += 1;
        }
        if self.max_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.BackoffStrategy", len)?;
        if let Some(v) = self.base_interval.as_ref() {
            struct_ser.serialize_field("baseInterval", v)?;
        }
        if let Some(v) = self.max_interval.as_ref() {
            struct_ser.serialize_field("maxInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BackoffStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_interval",
            "baseInterval",
            "max_interval",
            "maxInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseInterval,
            MaxInterval,
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
                            "baseInterval" | "base_interval" => Ok(GeneratedField::BaseInterval),
                            "maxInterval" | "max_interval" => Ok(GeneratedField::MaxInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BackoffStrategy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.BackoffStrategy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BackoffStrategy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_interval__ = None;
                let mut max_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BaseInterval => {
                            if base_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseInterval"));
                            }
                            base_interval__ = map.next_value()?;
                        }
                        GeneratedField::MaxInterval => {
                            if max_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInterval"));
                            }
                            max_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(BackoffStrategy {
                    base_interval: base_interval__,
                    max_interval: max_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.BackoffStrategy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BindConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.source_address.is_some() {
            len += 1;
        }
        if self.freebind.is_some() {
            len += 1;
        }
        if !self.socket_options.is_empty() {
            len += 1;
        }
        if !self.extra_source_addresses.is_empty() {
            len += 1;
        }
        if !self.additional_source_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.BindConfig", len)?;
        if let Some(v) = self.source_address.as_ref() {
            struct_ser.serialize_field("sourceAddress", v)?;
        }
        if let Some(v) = self.freebind.as_ref() {
            struct_ser.serialize_field("freebind", v)?;
        }
        if !self.socket_options.is_empty() {
            struct_ser.serialize_field("socketOptions", &self.socket_options)?;
        }
        if !self.extra_source_addresses.is_empty() {
            struct_ser.serialize_field("extraSourceAddresses", &self.extra_source_addresses)?;
        }
        if !self.additional_source_addresses.is_empty() {
            struct_ser.serialize_field("additionalSourceAddresses", &self.additional_source_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BindConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_address",
            "sourceAddress",
            "freebind",
            "socket_options",
            "socketOptions",
            "extra_source_addresses",
            "extraSourceAddresses",
            "additional_source_addresses",
            "additionalSourceAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceAddress,
            Freebind,
            SocketOptions,
            ExtraSourceAddresses,
            AdditionalSourceAddresses,
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
                            "sourceAddress" | "source_address" => Ok(GeneratedField::SourceAddress),
                            "freebind" => Ok(GeneratedField::Freebind),
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            "extraSourceAddresses" | "extra_source_addresses" => Ok(GeneratedField::ExtraSourceAddresses),
                            "additionalSourceAddresses" | "additional_source_addresses" => Ok(GeneratedField::AdditionalSourceAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BindConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.BindConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BindConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut source_address__ = None;
                let mut freebind__ = None;
                let mut socket_options__ = None;
                let mut extra_source_addresses__ = None;
                let mut additional_source_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceAddress => {
                            if source_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceAddress"));
                            }
                            source_address__ = map.next_value()?;
                        }
                        GeneratedField::Freebind => {
                            if freebind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("freebind"));
                            }
                            freebind__ = map.next_value()?;
                        }
                        GeneratedField::SocketOptions => {
                            if socket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketOptions"));
                            }
                            socket_options__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExtraSourceAddresses => {
                            if extra_source_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extraSourceAddresses"));
                            }
                            extra_source_addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::AdditionalSourceAddresses => {
                            if additional_source_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalSourceAddresses"));
                            }
                            additional_source_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BindConfig {
                    source_address: source_address__,
                    freebind: freebind__,
                    socket_options: socket_options__.unwrap_or_default(),
                    extra_source_addresses: extra_source_addresses__.unwrap_or_default(),
                    additional_source_addresses: additional_source_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.BindConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BuildVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.BuildVersion", len)?;
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BuildVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            Metadata,
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
                            "version" => Ok(GeneratedField::Version),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BuildVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.BuildVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BuildVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                    }
                }
                Ok(BuildVersion {
                    version: version__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.BuildVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CidrRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address_prefix.is_empty() {
            len += 1;
        }
        if self.prefix_len.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.CidrRange", len)?;
        if !self.address_prefix.is_empty() {
            struct_ser.serialize_field("addressPrefix", &self.address_prefix)?;
        }
        if let Some(v) = self.prefix_len.as_ref() {
            struct_ser.serialize_field("prefixLen", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CidrRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address_prefix",
            "addressPrefix",
            "prefix_len",
            "prefixLen",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddressPrefix,
            PrefixLen,
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
                            "addressPrefix" | "address_prefix" => Ok(GeneratedField::AddressPrefix),
                            "prefixLen" | "prefix_len" => Ok(GeneratedField::PrefixLen),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CidrRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.CidrRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CidrRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address_prefix__ = None;
                let mut prefix_len__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AddressPrefix => {
                            if address_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addressPrefix"));
                            }
                            address_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::PrefixLen => {
                            if prefix_len__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefixLen"));
                            }
                            prefix_len__ = map.next_value()?;
                        }
                    }
                }
                Ok(CidrRange {
                    address_prefix: address_prefix__.unwrap_or_default(),
                    prefix_len: prefix_len__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.CidrRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authorities.is_empty() {
            len += 1;
        }
        if self.initial_fetch_timeout.is_some() {
            len += 1;
        }
        if self.resource_api_version != 0 {
            len += 1;
        }
        if self.config_source_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.ConfigSource", len)?;
        if !self.authorities.is_empty() {
            struct_ser.serialize_field("authorities", &self.authorities)?;
        }
        if let Some(v) = self.initial_fetch_timeout.as_ref() {
            struct_ser.serialize_field("initialFetchTimeout", v)?;
        }
        if self.resource_api_version != 0 {
            let v = ApiVersion::from_i32(self.resource_api_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.resource_api_version)))?;
            struct_ser.serialize_field("resourceApiVersion", &v)?;
        }
        if let Some(v) = self.config_source_specifier.as_ref() {
            match v {
                config_source::ConfigSourceSpecifier::Path(v) => {
                    struct_ser.serialize_field("path", v)?;
                }
                config_source::ConfigSourceSpecifier::PathConfigSource(v) => {
                    struct_ser.serialize_field("pathConfigSource", v)?;
                }
                config_source::ConfigSourceSpecifier::ApiConfigSource(v) => {
                    struct_ser.serialize_field("apiConfigSource", v)?;
                }
                config_source::ConfigSourceSpecifier::Ads(v) => {
                    struct_ser.serialize_field("ads", v)?;
                }
                config_source::ConfigSourceSpecifier::Self_(v) => {
                    struct_ser.serialize_field("self", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorities",
            "initial_fetch_timeout",
            "initialFetchTimeout",
            "resource_api_version",
            "resourceApiVersion",
            "path",
            "path_config_source",
            "pathConfigSource",
            "api_config_source",
            "apiConfigSource",
            "ads",
            "self",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authorities,
            InitialFetchTimeout,
            ResourceApiVersion,
            Path,
            PathConfigSource,
            ApiConfigSource,
            Ads,
            Self_,
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
                            "authorities" => Ok(GeneratedField::Authorities),
                            "initialFetchTimeout" | "initial_fetch_timeout" => Ok(GeneratedField::InitialFetchTimeout),
                            "resourceApiVersion" | "resource_api_version" => Ok(GeneratedField::ResourceApiVersion),
                            "path" => Ok(GeneratedField::Path),
                            "pathConfigSource" | "path_config_source" => Ok(GeneratedField::PathConfigSource),
                            "apiConfigSource" | "api_config_source" => Ok(GeneratedField::ApiConfigSource),
                            "ads" => Ok(GeneratedField::Ads),
                            "self" => Ok(GeneratedField::Self_),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.ConfigSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorities__ = None;
                let mut initial_fetch_timeout__ = None;
                let mut resource_api_version__ = None;
                let mut config_source_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authorities => {
                            if authorities__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorities"));
                            }
                            authorities__ = Some(map.next_value()?);
                        }
                        GeneratedField::InitialFetchTimeout => {
                            if initial_fetch_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialFetchTimeout"));
                            }
                            initial_fetch_timeout__ = map.next_value()?;
                        }
                        GeneratedField::ResourceApiVersion => {
                            if resource_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceApiVersion"));
                            }
                            resource_api_version__ = Some(map.next_value::<ApiVersion>()? as i32);
                        }
                        GeneratedField::Path => {
                            if config_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            config_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(config_source::ConfigSourceSpecifier::Path);
                        }
                        GeneratedField::PathConfigSource => {
                            if config_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathConfigSource"));
                            }
                            config_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(config_source::ConfigSourceSpecifier::PathConfigSource)
;
                        }
                        GeneratedField::ApiConfigSource => {
                            if config_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("apiConfigSource"));
                            }
                            config_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(config_source::ConfigSourceSpecifier::ApiConfigSource)
;
                        }
                        GeneratedField::Ads => {
                            if config_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ads"));
                            }
                            config_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(config_source::ConfigSourceSpecifier::Ads)
;
                        }
                        GeneratedField::Self_ => {
                            if config_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("self"));
                            }
                            config_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(config_source::ConfigSourceSpecifier::Self_)
;
                        }
                    }
                }
                Ok(ConfigSource {
                    authorities: authorities__.unwrap_or_default(),
                    initial_fetch_timeout: initial_fetch_timeout__,
                    resource_api_version: resource_api_version__.unwrap_or_default(),
                    config_source_specifier: config_source_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.ConfigSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlPlane {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.identifier.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.ControlPlane", len)?;
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlPlane {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
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
                            "identifier" => Ok(GeneratedField::Identifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlPlane;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.ControlPlane")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ControlPlane, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ControlPlane {
                    identifier: identifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.ControlPlane", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DataSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.DataSource", len)?;
        if let Some(v) = self.specifier.as_ref() {
            match v {
                data_source::Specifier::Filename(v) => {
                    struct_ser.serialize_field("filename", v)?;
                }
                data_source::Specifier::InlineBytes(v) => {
                    struct_ser.serialize_field("inlineBytes", pbjson::private::base64::encode(&v).as_str())?;
                }
                data_source::Specifier::InlineString(v) => {
                    struct_ser.serialize_field("inlineString", v)?;
                }
                data_source::Specifier::EnvironmentVariable(v) => {
                    struct_ser.serialize_field("environmentVariable", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filename",
            "inline_bytes",
            "inlineBytes",
            "inline_string",
            "inlineString",
            "environment_variable",
            "environmentVariable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filename,
            InlineBytes,
            InlineString,
            EnvironmentVariable,
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
                            "filename" => Ok(GeneratedField::Filename),
                            "inlineBytes" | "inline_bytes" => Ok(GeneratedField::InlineBytes),
                            "inlineString" | "inline_string" => Ok(GeneratedField::InlineString),
                            "environmentVariable" | "environment_variable" => Ok(GeneratedField::EnvironmentVariable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.DataSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DataSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filename => {
                            if specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            specifier__ = map.next_value::<::std::option::Option<_>>()?.map(data_source::Specifier::Filename);
                        }
                        GeneratedField::InlineBytes => {
                            if specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineBytes"));
                            }
                            specifier__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| data_source::Specifier::InlineBytes(x.0));
                        }
                        GeneratedField::InlineString => {
                            if specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineString"));
                            }
                            specifier__ = map.next_value::<::std::option::Option<_>>()?.map(data_source::Specifier::InlineString);
                        }
                        GeneratedField::EnvironmentVariable => {
                            if specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("environmentVariable"));
                            }
                            specifier__ = map.next_value::<::std::option::Option<_>>()?.map(data_source::Specifier::EnvironmentVariable);
                        }
                    }
                }
                Ok(DataSource {
                    specifier: specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.DataSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DnsResolutionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resolvers.is_empty() {
            len += 1;
        }
        if self.dns_resolver_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.DnsResolutionConfig", len)?;
        if !self.resolvers.is_empty() {
            struct_ser.serialize_field("resolvers", &self.resolvers)?;
        }
        if let Some(v) = self.dns_resolver_options.as_ref() {
            struct_ser.serialize_field("dnsResolverOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsResolutionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resolvers",
            "dns_resolver_options",
            "dnsResolverOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resolvers,
            DnsResolverOptions,
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
                            "resolvers" => Ok(GeneratedField::Resolvers),
                            "dnsResolverOptions" | "dns_resolver_options" => Ok(GeneratedField::DnsResolverOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsResolutionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.DnsResolutionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsResolutionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resolvers__ = None;
                let mut dns_resolver_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Resolvers => {
                            if resolvers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolvers"));
                            }
                            resolvers__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsResolverOptions => {
                            if dns_resolver_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsResolverOptions"));
                            }
                            dns_resolver_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(DnsResolutionConfig {
                    resolvers: resolvers__.unwrap_or_default(),
                    dns_resolver_options: dns_resolver_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.DnsResolutionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DnsResolverOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.use_tcp_for_dns_lookups {
            len += 1;
        }
        if self.no_default_search_domain {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.DnsResolverOptions", len)?;
        if self.use_tcp_for_dns_lookups {
            struct_ser.serialize_field("useTcpForDnsLookups", &self.use_tcp_for_dns_lookups)?;
        }
        if self.no_default_search_domain {
            struct_ser.serialize_field("noDefaultSearchDomain", &self.no_default_search_domain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsResolverOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "use_tcp_for_dns_lookups",
            "useTcpForDnsLookups",
            "no_default_search_domain",
            "noDefaultSearchDomain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UseTcpForDnsLookups,
            NoDefaultSearchDomain,
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
                            "useTcpForDnsLookups" | "use_tcp_for_dns_lookups" => Ok(GeneratedField::UseTcpForDnsLookups),
                            "noDefaultSearchDomain" | "no_default_search_domain" => Ok(GeneratedField::NoDefaultSearchDomain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsResolverOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.DnsResolverOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsResolverOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut use_tcp_for_dns_lookups__ = None;
                let mut no_default_search_domain__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UseTcpForDnsLookups => {
                            if use_tcp_for_dns_lookups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useTcpForDnsLookups"));
                            }
                            use_tcp_for_dns_lookups__ = Some(map.next_value()?);
                        }
                        GeneratedField::NoDefaultSearchDomain => {
                            if no_default_search_domain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noDefaultSearchDomain"));
                            }
                            no_default_search_domain__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DnsResolverOptions {
                    use_tcp_for_dns_lookups: use_tcp_for_dns_lookups__.unwrap_or_default(),
                    no_default_search_domain: no_default_search_domain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.DnsResolverOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EnvoyInternalAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.endpoint_id.is_empty() {
            len += 1;
        }
        if self.address_name_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.EnvoyInternalAddress", len)?;
        if !self.endpoint_id.is_empty() {
            struct_ser.serialize_field("endpointId", &self.endpoint_id)?;
        }
        if let Some(v) = self.address_name_specifier.as_ref() {
            match v {
                envoy_internal_address::AddressNameSpecifier::ServerListenerName(v) => {
                    struct_ser.serialize_field("serverListenerName", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EnvoyInternalAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint_id",
            "endpointId",
            "server_listener_name",
            "serverListenerName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EndpointId,
            ServerListenerName,
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
                            "endpointId" | "endpoint_id" => Ok(GeneratedField::EndpointId),
                            "serverListenerName" | "server_listener_name" => Ok(GeneratedField::ServerListenerName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EnvoyInternalAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.EnvoyInternalAddress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EnvoyInternalAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint_id__ = None;
                let mut address_name_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EndpointId => {
                            if endpoint_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointId"));
                            }
                            endpoint_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServerListenerName => {
                            if address_name_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverListenerName"));
                            }
                            address_name_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(envoy_internal_address::AddressNameSpecifier::ServerListenerName);
                        }
                    }
                }
                Ok(EnvoyInternalAddress {
                    endpoint_id: endpoint_id__.unwrap_or_default(),
                    address_name_specifier: address_name_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.EnvoyInternalAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventServiceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_source_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.EventServiceConfig", len)?;
        if let Some(v) = self.config_source_specifier.as_ref() {
            match v {
                event_service_config::ConfigSourceSpecifier::GrpcService(v) => {
                    struct_ser.serialize_field("grpcService", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventServiceConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventServiceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.EventServiceConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventServiceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if config_source_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            config_source_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(event_service_config::ConfigSourceSpecifier::GrpcService)
;
                        }
                    }
                }
                Ok(EventServiceConfig {
                    config_source_specifier: config_source_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.EventServiceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Extension {
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
        if !self.category.is_empty() {
            len += 1;
        }
        if !self.type_descriptor.is_empty() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if !self.type_urls.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Extension", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.category.is_empty() {
            struct_ser.serialize_field("category", &self.category)?;
        }
        if !self.type_descriptor.is_empty() {
            struct_ser.serialize_field("typeDescriptor", &self.type_descriptor)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if !self.type_urls.is_empty() {
            struct_ser.serialize_field("typeUrls", &self.type_urls)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Extension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "category",
            "type_descriptor",
            "typeDescriptor",
            "version",
            "disabled",
            "type_urls",
            "typeUrls",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Category,
            TypeDescriptor,
            Version,
            Disabled,
            TypeUrls,
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
                            "category" => Ok(GeneratedField::Category),
                            "typeDescriptor" | "type_descriptor" => Ok(GeneratedField::TypeDescriptor),
                            "version" => Ok(GeneratedField::Version),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "typeUrls" | "type_urls" => Ok(GeneratedField::TypeUrls),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Extension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Extension")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Extension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut category__ = None;
                let mut type_descriptor__ = None;
                let mut version__ = None;
                let mut disabled__ = None;
                let mut type_urls__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Category => {
                            if category__.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypeDescriptor => {
                            if type_descriptor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeDescriptor"));
                            }
                            type_descriptor__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypeUrls => {
                            if type_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrls"));
                            }
                            type_urls__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Extension {
                    name: name__.unwrap_or_default(),
                    category: category__.unwrap_or_default(),
                    type_descriptor: type_descriptor__.unwrap_or_default(),
                    version: version__,
                    disabled: disabled__.unwrap_or_default(),
                    type_urls: type_urls__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Extension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtensionConfigSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_source.is_some() {
            len += 1;
        }
        if self.default_config.is_some() {
            len += 1;
        }
        if self.apply_default_config_without_warming {
            len += 1;
        }
        if !self.type_urls.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.ExtensionConfigSource", len)?;
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("configSource", v)?;
        }
        if let Some(v) = self.default_config.as_ref() {
            struct_ser.serialize_field("defaultConfig", v)?;
        }
        if self.apply_default_config_without_warming {
            struct_ser.serialize_field("applyDefaultConfigWithoutWarming", &self.apply_default_config_without_warming)?;
        }
        if !self.type_urls.is_empty() {
            struct_ser.serialize_field("typeUrls", &self.type_urls)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtensionConfigSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_source",
            "configSource",
            "default_config",
            "defaultConfig",
            "apply_default_config_without_warming",
            "applyDefaultConfigWithoutWarming",
            "type_urls",
            "typeUrls",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigSource,
            DefaultConfig,
            ApplyDefaultConfigWithoutWarming,
            TypeUrls,
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
                            "configSource" | "config_source" => Ok(GeneratedField::ConfigSource),
                            "defaultConfig" | "default_config" => Ok(GeneratedField::DefaultConfig),
                            "applyDefaultConfigWithoutWarming" | "apply_default_config_without_warming" => Ok(GeneratedField::ApplyDefaultConfigWithoutWarming),
                            "typeUrls" | "type_urls" => Ok(GeneratedField::TypeUrls),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtensionConfigSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.ExtensionConfigSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtensionConfigSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source__ = None;
                let mut default_config__ = None;
                let mut apply_default_config_without_warming__ = None;
                let mut type_urls__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfigSource => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSource"));
                            }
                            config_source__ = map.next_value()?;
                        }
                        GeneratedField::DefaultConfig => {
                            if default_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultConfig"));
                            }
                            default_config__ = map.next_value()?;
                        }
                        GeneratedField::ApplyDefaultConfigWithoutWarming => {
                            if apply_default_config_without_warming__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applyDefaultConfigWithoutWarming"));
                            }
                            apply_default_config_without_warming__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypeUrls => {
                            if type_urls__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeUrls"));
                            }
                            type_urls__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExtensionConfigSource {
                    config_source: config_source__,
                    default_config: default_config__,
                    apply_default_config_without_warming: apply_default_config_without_warming__.unwrap_or_default(),
                    type_urls: type_urls__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.ExtensionConfigSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtraSourceAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if self.socket_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.ExtraSourceAddress", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if let Some(v) = self.socket_options.as_ref() {
            struct_ser.serialize_field("socketOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtraSourceAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "socket_options",
            "socketOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            SocketOptions,
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
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtraSourceAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.ExtraSourceAddress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtraSourceAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut socket_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::SocketOptions => {
                            if socket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketOptions"));
                            }
                            socket_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExtraSourceAddress {
                    address: address__,
                    socket_options: socket_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.ExtraSourceAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrpcMethodList {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcMethodList", len)?;
        if !self.services.is_empty() {
            struct_ser.serialize_field("services", &self.services)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrpcMethodList {
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
            type Value = GrpcMethodList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcMethodList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GrpcMethodList, V::Error>
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
                Ok(GrpcMethodList {
                    services: services__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcMethodList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_method_list::Service {
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
        if !self.method_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcMethodList.Service", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.method_names.is_empty() {
            struct_ser.serialize_field("methodNames", &self.method_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_method_list::Service {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "method_names",
            "methodNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            MethodNames,
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
                            "methodNames" | "method_names" => Ok(GeneratedField::MethodNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_method_list::Service;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcMethodList.Service")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_method_list::Service, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut method_names__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::MethodNames => {
                            if method_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methodNames"));
                            }
                            method_names__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(grpc_method_list::Service {
                    name: name__.unwrap_or_default(),
                    method_names: method_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcMethodList.Service", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrpcProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http2_protocol_options.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcProtocolOptions", len)?;
        if let Some(v) = self.http2_protocol_options.as_ref() {
            struct_ser.serialize_field("http2ProtocolOptions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrpcProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http2_protocol_options",
            "http2ProtocolOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Http2ProtocolOptions,
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
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrpcProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GrpcProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http2_protocol_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Http2ProtocolOptions => {
                            if http2_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2ProtocolOptions"));
                            }
                            http2_protocol_options__ = map.next_value()?;
                        }
                    }
                }
                Ok(GrpcProtocolOptions {
                    http2_protocol_options: http2_protocol_options__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GrpcService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timeout.is_some() {
            len += 1;
        }
        if !self.initial_metadata.is_empty() {
            len += 1;
        }
        if self.target_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService", len)?;
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if !self.initial_metadata.is_empty() {
            struct_ser.serialize_field("initialMetadata", &self.initial_metadata)?;
        }
        if let Some(v) = self.target_specifier.as_ref() {
            match v {
                grpc_service::TargetSpecifier::EnvoyGrpc(v) => {
                    struct_ser.serialize_field("envoyGrpc", v)?;
                }
                grpc_service::TargetSpecifier::GoogleGrpc(v) => {
                    struct_ser.serialize_field("googleGrpc", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrpcService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timeout",
            "initial_metadata",
            "initialMetadata",
            "envoy_grpc",
            "envoyGrpc",
            "google_grpc",
            "googleGrpc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timeout,
            InitialMetadata,
            EnvoyGrpc,
            GoogleGrpc,
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
                            "timeout" => Ok(GeneratedField::Timeout),
                            "initialMetadata" | "initial_metadata" => Ok(GeneratedField::InitialMetadata),
                            "envoyGrpc" | "envoy_grpc" => Ok(GeneratedField::EnvoyGrpc),
                            "googleGrpc" | "google_grpc" => Ok(GeneratedField::GoogleGrpc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrpcService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GrpcService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timeout__ = None;
                let mut initial_metadata__ = None;
                let mut target_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                        GeneratedField::InitialMetadata => {
                            if initial_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialMetadata"));
                            }
                            initial_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnvoyGrpc => {
                            if target_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("envoyGrpc"));
                            }
                            target_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::TargetSpecifier::EnvoyGrpc)
;
                        }
                        GeneratedField::GoogleGrpc => {
                            if target_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("googleGrpc"));
                            }
                            target_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::TargetSpecifier::GoogleGrpc)
;
                        }
                    }
                }
                Ok(GrpcService {
                    timeout: timeout__,
                    initial_metadata: initial_metadata__.unwrap_or_default(),
                    target_specifier: target_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::EnvoyGrpc {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if !self.authority.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.EnvoyGrpc", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("clusterName", &self.cluster_name)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::EnvoyGrpc {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "authority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            Authority,
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
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "authority" => Ok(GeneratedField::Authority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::EnvoyGrpc;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.EnvoyGrpc")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::EnvoyGrpc, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut authority__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(grpc_service::EnvoyGrpc {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.EnvoyGrpc", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::GoogleGrpc {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target_uri.is_empty() {
            len += 1;
        }
        if self.channel_credentials.is_some() {
            len += 1;
        }
        if !self.call_credentials.is_empty() {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if !self.credentials_factory_name.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.per_stream_buffer_limit_bytes.is_some() {
            len += 1;
        }
        if self.channel_args.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc", len)?;
        if !self.target_uri.is_empty() {
            struct_ser.serialize_field("targetUri", &self.target_uri)?;
        }
        if let Some(v) = self.channel_credentials.as_ref() {
            struct_ser.serialize_field("channelCredentials", v)?;
        }
        if !self.call_credentials.is_empty() {
            struct_ser.serialize_field("callCredentials", &self.call_credentials)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if !self.credentials_factory_name.is_empty() {
            struct_ser.serialize_field("credentialsFactoryName", &self.credentials_factory_name)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.per_stream_buffer_limit_bytes.as_ref() {
            struct_ser.serialize_field("perStreamBufferLimitBytes", v)?;
        }
        if let Some(v) = self.channel_args.as_ref() {
            struct_ser.serialize_field("channelArgs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::GoogleGrpc {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target_uri",
            "targetUri",
            "channel_credentials",
            "channelCredentials",
            "call_credentials",
            "callCredentials",
            "stat_prefix",
            "statPrefix",
            "credentials_factory_name",
            "credentialsFactoryName",
            "config",
            "per_stream_buffer_limit_bytes",
            "perStreamBufferLimitBytes",
            "channel_args",
            "channelArgs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TargetUri,
            ChannelCredentials,
            CallCredentials,
            StatPrefix,
            CredentialsFactoryName,
            Config,
            PerStreamBufferLimitBytes,
            ChannelArgs,
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
                            "targetUri" | "target_uri" => Ok(GeneratedField::TargetUri),
                            "channelCredentials" | "channel_credentials" => Ok(GeneratedField::ChannelCredentials),
                            "callCredentials" | "call_credentials" => Ok(GeneratedField::CallCredentials),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "credentialsFactoryName" | "credentials_factory_name" => Ok(GeneratedField::CredentialsFactoryName),
                            "config" => Ok(GeneratedField::Config),
                            "perStreamBufferLimitBytes" | "per_stream_buffer_limit_bytes" => Ok(GeneratedField::PerStreamBufferLimitBytes),
                            "channelArgs" | "channel_args" => Ok(GeneratedField::ChannelArgs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::GoogleGrpc;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::GoogleGrpc, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target_uri__ = None;
                let mut channel_credentials__ = None;
                let mut call_credentials__ = None;
                let mut stat_prefix__ = None;
                let mut credentials_factory_name__ = None;
                let mut config__ = None;
                let mut per_stream_buffer_limit_bytes__ = None;
                let mut channel_args__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TargetUri => {
                            if target_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetUri"));
                            }
                            target_uri__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChannelCredentials => {
                            if channel_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelCredentials"));
                            }
                            channel_credentials__ = map.next_value()?;
                        }
                        GeneratedField::CallCredentials => {
                            if call_credentials__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callCredentials"));
                            }
                            call_credentials__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::CredentialsFactoryName => {
                            if credentials_factory_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentialsFactoryName"));
                            }
                            credentials_factory_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map.next_value()?;
                        }
                        GeneratedField::PerStreamBufferLimitBytes => {
                            if per_stream_buffer_limit_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("perStreamBufferLimitBytes"));
                            }
                            per_stream_buffer_limit_bytes__ = map.next_value()?;
                        }
                        GeneratedField::ChannelArgs => {
                            if channel_args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelArgs"));
                            }
                            channel_args__ = map.next_value()?;
                        }
                    }
                }
                Ok(grpc_service::GoogleGrpc {
                    target_uri: target_uri__.unwrap_or_default(),
                    channel_credentials: channel_credentials__,
                    call_credentials: call_credentials__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    credentials_factory_name: credentials_factory_name__.unwrap_or_default(),
                    config: config__,
                    per_stream_buffer_limit_bytes: per_stream_buffer_limit_bytes__,
                    channel_args: channel_args__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::CallCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.credential_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials", len)?;
        if let Some(v) = self.credential_specifier.as_ref() {
            match v {
                grpc_service::google_grpc::call_credentials::CredentialSpecifier::AccessToken(v) => {
                    struct_ser.serialize_field("accessToken", v)?;
                }
                grpc_service::google_grpc::call_credentials::CredentialSpecifier::GoogleComputeEngine(v) => {
                    struct_ser.serialize_field("googleComputeEngine", v)?;
                }
                grpc_service::google_grpc::call_credentials::CredentialSpecifier::GoogleRefreshToken(v) => {
                    struct_ser.serialize_field("googleRefreshToken", v)?;
                }
                grpc_service::google_grpc::call_credentials::CredentialSpecifier::ServiceAccountJwtAccess(v) => {
                    struct_ser.serialize_field("serviceAccountJwtAccess", v)?;
                }
                grpc_service::google_grpc::call_credentials::CredentialSpecifier::GoogleIam(v) => {
                    struct_ser.serialize_field("googleIam", v)?;
                }
                grpc_service::google_grpc::call_credentials::CredentialSpecifier::FromPlugin(v) => {
                    struct_ser.serialize_field("fromPlugin", v)?;
                }
                grpc_service::google_grpc::call_credentials::CredentialSpecifier::StsService(v) => {
                    struct_ser.serialize_field("stsService", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::CallCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "access_token",
            "accessToken",
            "google_compute_engine",
            "googleComputeEngine",
            "google_refresh_token",
            "googleRefreshToken",
            "service_account_jwt_access",
            "serviceAccountJwtAccess",
            "google_iam",
            "googleIam",
            "from_plugin",
            "fromPlugin",
            "sts_service",
            "stsService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AccessToken,
            GoogleComputeEngine,
            GoogleRefreshToken,
            ServiceAccountJwtAccess,
            GoogleIam,
            FromPlugin,
            StsService,
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
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "googleComputeEngine" | "google_compute_engine" => Ok(GeneratedField::GoogleComputeEngine),
                            "googleRefreshToken" | "google_refresh_token" => Ok(GeneratedField::GoogleRefreshToken),
                            "serviceAccountJwtAccess" | "service_account_jwt_access" => Ok(GeneratedField::ServiceAccountJwtAccess),
                            "googleIam" | "google_iam" => Ok(GeneratedField::GoogleIam),
                            "fromPlugin" | "from_plugin" => Ok(GeneratedField::FromPlugin),
                            "stsService" | "sts_service" => Ok(GeneratedField::StsService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::CallCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::CallCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credential_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AccessToken => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::CredentialSpecifier::AccessToken);
                        }
                        GeneratedField::GoogleComputeEngine => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("googleComputeEngine"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::CredentialSpecifier::GoogleComputeEngine)
;
                        }
                        GeneratedField::GoogleRefreshToken => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("googleRefreshToken"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::CredentialSpecifier::GoogleRefreshToken);
                        }
                        GeneratedField::ServiceAccountJwtAccess => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceAccountJwtAccess"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::CredentialSpecifier::ServiceAccountJwtAccess)
;
                        }
                        GeneratedField::GoogleIam => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("googleIam"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::CredentialSpecifier::GoogleIam)
;
                        }
                        GeneratedField::FromPlugin => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fromPlugin"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::CredentialSpecifier::FromPlugin)
;
                        }
                        GeneratedField::StsService => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stsService"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::CredentialSpecifier::StsService)
;
                        }
                    }
                }
                Ok(grpc_service::google_grpc::CallCredentials {
                    credential_specifier: credential_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::call_credentials::GoogleIamCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authorization_token.is_empty() {
            len += 1;
        }
        if !self.authority_selector.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.GoogleIAMCredentials", len)?;
        if !self.authorization_token.is_empty() {
            struct_ser.serialize_field("authorizationToken", &self.authorization_token)?;
        }
        if !self.authority_selector.is_empty() {
            struct_ser.serialize_field("authoritySelector", &self.authority_selector)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::call_credentials::GoogleIamCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authorization_token",
            "authorizationToken",
            "authority_selector",
            "authoritySelector",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthorizationToken,
            AuthoritySelector,
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
                            "authorizationToken" | "authorization_token" => Ok(GeneratedField::AuthorizationToken),
                            "authoritySelector" | "authority_selector" => Ok(GeneratedField::AuthoritySelector),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::call_credentials::GoogleIamCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.GoogleIAMCredentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::call_credentials::GoogleIamCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authorization_token__ = None;
                let mut authority_selector__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AuthorizationToken => {
                            if authorization_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationToken"));
                            }
                            authorization_token__ = Some(map.next_value()?);
                        }
                        GeneratedField::AuthoritySelector => {
                            if authority_selector__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authoritySelector"));
                            }
                            authority_selector__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(grpc_service::google_grpc::call_credentials::GoogleIamCredentials {
                    authorization_token: authorization_token__.unwrap_or_default(),
                    authority_selector: authority_selector__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.GoogleIAMCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.MetadataCredentialsFromPlugin", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
            type Value = grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.MetadataCredentialsFromPlugin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin, V::Error>
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
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::call_credentials::metadata_credentials_from_plugin::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(grpc_service::google_grpc::call_credentials::MetadataCredentialsFromPlugin {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.MetadataCredentialsFromPlugin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::call_credentials::ServiceAccountJwtAccessCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.json_key.is_empty() {
            len += 1;
        }
        if self.token_lifetime_seconds != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.ServiceAccountJWTAccessCredentials", len)?;
        if !self.json_key.is_empty() {
            struct_ser.serialize_field("jsonKey", &self.json_key)?;
        }
        if self.token_lifetime_seconds != 0 {
            struct_ser.serialize_field("tokenLifetimeSeconds", ToString::to_string(&self.token_lifetime_seconds).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::call_credentials::ServiceAccountJwtAccessCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "json_key",
            "jsonKey",
            "token_lifetime_seconds",
            "tokenLifetimeSeconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JsonKey,
            TokenLifetimeSeconds,
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
                            "jsonKey" | "json_key" => Ok(GeneratedField::JsonKey),
                            "tokenLifetimeSeconds" | "token_lifetime_seconds" => Ok(GeneratedField::TokenLifetimeSeconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::call_credentials::ServiceAccountJwtAccessCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.ServiceAccountJWTAccessCredentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::call_credentials::ServiceAccountJwtAccessCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut json_key__ = None;
                let mut token_lifetime_seconds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::JsonKey => {
                            if json_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonKey"));
                            }
                            json_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenLifetimeSeconds => {
                            if token_lifetime_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenLifetimeSeconds"));
                            }
                            token_lifetime_seconds__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(grpc_service::google_grpc::call_credentials::ServiceAccountJwtAccessCredentials {
                    json_key: json_key__.unwrap_or_default(),
                    token_lifetime_seconds: token_lifetime_seconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.ServiceAccountJWTAccessCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::call_credentials::StsService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token_exchange_service_uri.is_empty() {
            len += 1;
        }
        if !self.resource.is_empty() {
            len += 1;
        }
        if !self.audience.is_empty() {
            len += 1;
        }
        if !self.scope.is_empty() {
            len += 1;
        }
        if !self.requested_token_type.is_empty() {
            len += 1;
        }
        if !self.subject_token_path.is_empty() {
            len += 1;
        }
        if !self.subject_token_type.is_empty() {
            len += 1;
        }
        if !self.actor_token_path.is_empty() {
            len += 1;
        }
        if !self.actor_token_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.StsService", len)?;
        if !self.token_exchange_service_uri.is_empty() {
            struct_ser.serialize_field("tokenExchangeServiceUri", &self.token_exchange_service_uri)?;
        }
        if !self.resource.is_empty() {
            struct_ser.serialize_field("resource", &self.resource)?;
        }
        if !self.audience.is_empty() {
            struct_ser.serialize_field("audience", &self.audience)?;
        }
        if !self.scope.is_empty() {
            struct_ser.serialize_field("scope", &self.scope)?;
        }
        if !self.requested_token_type.is_empty() {
            struct_ser.serialize_field("requestedTokenType", &self.requested_token_type)?;
        }
        if !self.subject_token_path.is_empty() {
            struct_ser.serialize_field("subjectTokenPath", &self.subject_token_path)?;
        }
        if !self.subject_token_type.is_empty() {
            struct_ser.serialize_field("subjectTokenType", &self.subject_token_type)?;
        }
        if !self.actor_token_path.is_empty() {
            struct_ser.serialize_field("actorTokenPath", &self.actor_token_path)?;
        }
        if !self.actor_token_type.is_empty() {
            struct_ser.serialize_field("actorTokenType", &self.actor_token_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::call_credentials::StsService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "token_exchange_service_uri",
            "tokenExchangeServiceUri",
            "resource",
            "audience",
            "scope",
            "requested_token_type",
            "requestedTokenType",
            "subject_token_path",
            "subjectTokenPath",
            "subject_token_type",
            "subjectTokenType",
            "actor_token_path",
            "actorTokenPath",
            "actor_token_type",
            "actorTokenType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenExchangeServiceUri,
            Resource,
            Audience,
            Scope,
            RequestedTokenType,
            SubjectTokenPath,
            SubjectTokenType,
            ActorTokenPath,
            ActorTokenType,
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
                            "tokenExchangeServiceUri" | "token_exchange_service_uri" => Ok(GeneratedField::TokenExchangeServiceUri),
                            "resource" => Ok(GeneratedField::Resource),
                            "audience" => Ok(GeneratedField::Audience),
                            "scope" => Ok(GeneratedField::Scope),
                            "requestedTokenType" | "requested_token_type" => Ok(GeneratedField::RequestedTokenType),
                            "subjectTokenPath" | "subject_token_path" => Ok(GeneratedField::SubjectTokenPath),
                            "subjectTokenType" | "subject_token_type" => Ok(GeneratedField::SubjectTokenType),
                            "actorTokenPath" | "actor_token_path" => Ok(GeneratedField::ActorTokenPath),
                            "actorTokenType" | "actor_token_type" => Ok(GeneratedField::ActorTokenType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::call_credentials::StsService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.StsService")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::call_credentials::StsService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token_exchange_service_uri__ = None;
                let mut resource__ = None;
                let mut audience__ = None;
                let mut scope__ = None;
                let mut requested_token_type__ = None;
                let mut subject_token_path__ = None;
                let mut subject_token_type__ = None;
                let mut actor_token_path__ = None;
                let mut actor_token_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TokenExchangeServiceUri => {
                            if token_exchange_service_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokenExchangeServiceUri"));
                            }
                            token_exchange_service_uri__ = Some(map.next_value()?);
                        }
                        GeneratedField::Resource => {
                            if resource__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            resource__ = Some(map.next_value()?);
                        }
                        GeneratedField::Audience => {
                            if audience__.is_some() {
                                return Err(serde::de::Error::duplicate_field("audience"));
                            }
                            audience__ = Some(map.next_value()?);
                        }
                        GeneratedField::Scope => {
                            if scope__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            scope__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestedTokenType => {
                            if requested_token_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestedTokenType"));
                            }
                            requested_token_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectTokenPath => {
                            if subject_token_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectTokenPath"));
                            }
                            subject_token_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectTokenType => {
                            if subject_token_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectTokenType"));
                            }
                            subject_token_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::ActorTokenPath => {
                            if actor_token_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actorTokenPath"));
                            }
                            actor_token_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ActorTokenType => {
                            if actor_token_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("actorTokenType"));
                            }
                            actor_token_type__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(grpc_service::google_grpc::call_credentials::StsService {
                    token_exchange_service_uri: token_exchange_service_uri__.unwrap_or_default(),
                    resource: resource__.unwrap_or_default(),
                    audience: audience__.unwrap_or_default(),
                    scope: scope__.unwrap_or_default(),
                    requested_token_type: requested_token_type__.unwrap_or_default(),
                    subject_token_path: subject_token_path__.unwrap_or_default(),
                    subject_token_type: subject_token_type__.unwrap_or_default(),
                    actor_token_path: actor_token_path__.unwrap_or_default(),
                    actor_token_type: actor_token_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.CallCredentials.StsService", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::ChannelArgs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.args.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs", len)?;
        if !self.args.is_empty() {
            struct_ser.serialize_field("args", &self.args)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::ChannelArgs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "args",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Args,
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
                            "args" => Ok(GeneratedField::Args),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::ChannelArgs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::ChannelArgs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut args__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Args => {
                            if args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("args"));
                            }
                            args__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(grpc_service::google_grpc::ChannelArgs {
                    args: args__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::channel_args::Value {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs.Value", len)?;
        if let Some(v) = self.value_specifier.as_ref() {
            match v {
                grpc_service::google_grpc::channel_args::value::ValueSpecifier::StringValue(v) => {
                    struct_ser.serialize_field("stringValue", v)?;
                }
                grpc_service::google_grpc::channel_args::value::ValueSpecifier::IntValue(v) => {
                    struct_ser.serialize_field("intValue", ToString::to_string(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::channel_args::Value {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "string_value",
            "stringValue",
            "int_value",
            "intValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StringValue,
            IntValue,
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
                            "stringValue" | "string_value" => Ok(GeneratedField::StringValue),
                            "intValue" | "int_value" => Ok(GeneratedField::IntValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::channel_args::Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs.Value")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::channel_args::Value, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StringValue => {
                            if value_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringValue"));
                            }
                            value_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::channel_args::value::ValueSpecifier::StringValue);
                        }
                        GeneratedField::IntValue => {
                            if value_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intValue"));
                            }
                            value_specifier__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| grpc_service::google_grpc::channel_args::value::ValueSpecifier::IntValue(x.0));
                        }
                    }
                }
                Ok(grpc_service::google_grpc::channel_args::Value {
                    value_specifier: value_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelArgs.Value", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::ChannelCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.credential_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelCredentials", len)?;
        if let Some(v) = self.credential_specifier.as_ref() {
            match v {
                grpc_service::google_grpc::channel_credentials::CredentialSpecifier::SslCredentials(v) => {
                    struct_ser.serialize_field("sslCredentials", v)?;
                }
                grpc_service::google_grpc::channel_credentials::CredentialSpecifier::GoogleDefault(v) => {
                    struct_ser.serialize_field("googleDefault", v)?;
                }
                grpc_service::google_grpc::channel_credentials::CredentialSpecifier::LocalCredentials(v) => {
                    struct_ser.serialize_field("localCredentials", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::ChannelCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ssl_credentials",
            "sslCredentials",
            "google_default",
            "googleDefault",
            "local_credentials",
            "localCredentials",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SslCredentials,
            GoogleDefault,
            LocalCredentials,
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
                            "sslCredentials" | "ssl_credentials" => Ok(GeneratedField::SslCredentials),
                            "googleDefault" | "google_default" => Ok(GeneratedField::GoogleDefault),
                            "localCredentials" | "local_credentials" => Ok(GeneratedField::LocalCredentials),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::ChannelCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelCredentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::ChannelCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credential_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SslCredentials => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sslCredentials"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::channel_credentials::CredentialSpecifier::SslCredentials)
;
                        }
                        GeneratedField::GoogleDefault => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("googleDefault"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::channel_credentials::CredentialSpecifier::GoogleDefault)
;
                        }
                        GeneratedField::LocalCredentials => {
                            if credential_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localCredentials"));
                            }
                            credential_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_service::google_grpc::channel_credentials::CredentialSpecifier::LocalCredentials)
;
                        }
                    }
                }
                Ok(grpc_service::google_grpc::ChannelCredentials {
                    credential_specifier: credential_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.ChannelCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::GoogleLocalCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.GoogleLocalCredentials", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::GoogleLocalCredentials {
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
            type Value = grpc_service::google_grpc::GoogleLocalCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.GoogleLocalCredentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::GoogleLocalCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(grpc_service::google_grpc::GoogleLocalCredentials {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.GoogleLocalCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_service::google_grpc::SslCredentials {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.root_certs.is_some() {
            len += 1;
        }
        if self.private_key.is_some() {
            len += 1;
        }
        if self.cert_chain.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.SslCredentials", len)?;
        if let Some(v) = self.root_certs.as_ref() {
            struct_ser.serialize_field("rootCerts", v)?;
        }
        if let Some(v) = self.private_key.as_ref() {
            struct_ser.serialize_field("privateKey", v)?;
        }
        if let Some(v) = self.cert_chain.as_ref() {
            struct_ser.serialize_field("certChain", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_service::google_grpc::SslCredentials {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root_certs",
            "rootCerts",
            "private_key",
            "privateKey",
            "cert_chain",
            "certChain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RootCerts,
            PrivateKey,
            CertChain,
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
                            "rootCerts" | "root_certs" => Ok(GeneratedField::RootCerts),
                            "privateKey" | "private_key" => Ok(GeneratedField::PrivateKey),
                            "certChain" | "cert_chain" => Ok(GeneratedField::CertChain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_service::google_grpc::SslCredentials;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.GrpcService.GoogleGrpc.SslCredentials")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_service::google_grpc::SslCredentials, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root_certs__ = None;
                let mut private_key__ = None;
                let mut cert_chain__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RootCerts => {
                            if root_certs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootCerts"));
                            }
                            root_certs__ = map.next_value()?;
                        }
                        GeneratedField::PrivateKey => {
                            if private_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKey"));
                            }
                            private_key__ = map.next_value()?;
                        }
                        GeneratedField::CertChain => {
                            if cert_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certChain"));
                            }
                            cert_chain__ = map.next_value()?;
                        }
                    }
                }
                Ok(grpc_service::google_grpc::SslCredentials {
                    root_certs: root_certs__,
                    private_key: private_key__,
                    cert_chain: cert_chain__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.GrpcService.GoogleGrpc.SslCredentials", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderMap {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HeaderMap", len)?;
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMap {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "headers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Headers,
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
                            "headers" => Ok(GeneratedField::Headers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HeaderMap")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderMap, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HeaderMap {
                    headers: headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HeaderMap", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HeaderValue", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HeaderValue")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                    }
                }
                Ok(HeaderValue {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HeaderValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HeaderValueOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        if self.append.is_some() {
            len += 1;
        }
        if self.append_action != 0 {
            len += 1;
        }
        if self.keep_empty_value {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HeaderValueOption", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.append.as_ref() {
            struct_ser.serialize_field("append", v)?;
        }
        if self.append_action != 0 {
            let v = header_value_option::HeaderAppendAction::from_i32(self.append_action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.append_action)))?;
            struct_ser.serialize_field("appendAction", &v)?;
        }
        if self.keep_empty_value {
            struct_ser.serialize_field("keepEmptyValue", &self.keep_empty_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HeaderValueOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "append",
            "append_action",
            "appendAction",
            "keep_empty_value",
            "keepEmptyValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Append,
            AppendAction,
            KeepEmptyValue,
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
                            "append" => Ok(GeneratedField::Append),
                            "appendAction" | "append_action" => Ok(GeneratedField::AppendAction),
                            "keepEmptyValue" | "keep_empty_value" => Ok(GeneratedField::KeepEmptyValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HeaderValueOption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HeaderValueOption")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HeaderValueOption, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut append__ = None;
                let mut append_action__ = None;
                let mut keep_empty_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Append => {
                            if append__.is_some() {
                                return Err(serde::de::Error::duplicate_field("append"));
                            }
                            append__ = map.next_value()?;
                        }
                        GeneratedField::AppendAction => {
                            if append_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appendAction"));
                            }
                            append_action__ = Some(map.next_value::<header_value_option::HeaderAppendAction>()? as i32);
                        }
                        GeneratedField::KeepEmptyValue => {
                            if keep_empty_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keepEmptyValue"));
                            }
                            keep_empty_value__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HeaderValueOption {
                    header: header__,
                    append: append__,
                    append_action: append_action__.unwrap_or_default(),
                    keep_empty_value: keep_empty_value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HeaderValueOption", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for header_value_option::HeaderAppendAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AppendIfExistsOrAdd => "APPEND_IF_EXISTS_OR_ADD",
            Self::AddIfAbsent => "ADD_IF_ABSENT",
            Self::OverwriteIfExistsOrAdd => "OVERWRITE_IF_EXISTS_OR_ADD",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for header_value_option::HeaderAppendAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "APPEND_IF_EXISTS_OR_ADD",
            "ADD_IF_ABSENT",
            "OVERWRITE_IF_EXISTS_OR_ADD",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = header_value_option::HeaderAppendAction;

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
                    .and_then(header_value_option::HeaderAppendAction::from_i32)
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
                    .and_then(header_value_option::HeaderAppendAction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "APPEND_IF_EXISTS_OR_ADD" => Ok(header_value_option::HeaderAppendAction::AppendIfExistsOrAdd),
                    "ADD_IF_ABSENT" => Ok(header_value_option::HeaderAppendAction::AddIfAbsent),
                    "OVERWRITE_IF_EXISTS_OR_ADD" => Ok(header_value_option::HeaderAppendAction::OverwriteIfExistsOrAdd),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timeout.is_some() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        if self.initial_jitter.is_some() {
            len += 1;
        }
        if self.interval_jitter.is_some() {
            len += 1;
        }
        if self.interval_jitter_percent != 0 {
            len += 1;
        }
        if self.unhealthy_threshold.is_some() {
            len += 1;
        }
        if self.healthy_threshold.is_some() {
            len += 1;
        }
        if self.alt_port.is_some() {
            len += 1;
        }
        if self.reuse_connection.is_some() {
            len += 1;
        }
        if self.no_traffic_interval.is_some() {
            len += 1;
        }
        if self.no_traffic_healthy_interval.is_some() {
            len += 1;
        }
        if self.unhealthy_interval.is_some() {
            len += 1;
        }
        if self.unhealthy_edge_interval.is_some() {
            len += 1;
        }
        if self.healthy_edge_interval.is_some() {
            len += 1;
        }
        if !self.event_log_path.is_empty() {
            len += 1;
        }
        if self.event_service.is_some() {
            len += 1;
        }
        if self.always_log_health_check_failures {
            len += 1;
        }
        if self.tls_options.is_some() {
            len += 1;
        }
        if self.transport_socket_match_criteria.is_some() {
            len += 1;
        }
        if self.health_checker.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck", len)?;
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if let Some(v) = self.initial_jitter.as_ref() {
            struct_ser.serialize_field("initialJitter", v)?;
        }
        if let Some(v) = self.interval_jitter.as_ref() {
            struct_ser.serialize_field("intervalJitter", v)?;
        }
        if self.interval_jitter_percent != 0 {
            struct_ser.serialize_field("intervalJitterPercent", &self.interval_jitter_percent)?;
        }
        if let Some(v) = self.unhealthy_threshold.as_ref() {
            struct_ser.serialize_field("unhealthyThreshold", v)?;
        }
        if let Some(v) = self.healthy_threshold.as_ref() {
            struct_ser.serialize_field("healthyThreshold", v)?;
        }
        if let Some(v) = self.alt_port.as_ref() {
            struct_ser.serialize_field("altPort", v)?;
        }
        if let Some(v) = self.reuse_connection.as_ref() {
            struct_ser.serialize_field("reuseConnection", v)?;
        }
        if let Some(v) = self.no_traffic_interval.as_ref() {
            struct_ser.serialize_field("noTrafficInterval", v)?;
        }
        if let Some(v) = self.no_traffic_healthy_interval.as_ref() {
            struct_ser.serialize_field("noTrafficHealthyInterval", v)?;
        }
        if let Some(v) = self.unhealthy_interval.as_ref() {
            struct_ser.serialize_field("unhealthyInterval", v)?;
        }
        if let Some(v) = self.unhealthy_edge_interval.as_ref() {
            struct_ser.serialize_field("unhealthyEdgeInterval", v)?;
        }
        if let Some(v) = self.healthy_edge_interval.as_ref() {
            struct_ser.serialize_field("healthyEdgeInterval", v)?;
        }
        if !self.event_log_path.is_empty() {
            struct_ser.serialize_field("eventLogPath", &self.event_log_path)?;
        }
        if let Some(v) = self.event_service.as_ref() {
            struct_ser.serialize_field("eventService", v)?;
        }
        if self.always_log_health_check_failures {
            struct_ser.serialize_field("alwaysLogHealthCheckFailures", &self.always_log_health_check_failures)?;
        }
        if let Some(v) = self.tls_options.as_ref() {
            struct_ser.serialize_field("tlsOptions", v)?;
        }
        if let Some(v) = self.transport_socket_match_criteria.as_ref() {
            struct_ser.serialize_field("transportSocketMatchCriteria", v)?;
        }
        if let Some(v) = self.health_checker.as_ref() {
            match v {
                health_check::HealthChecker::HttpHealthCheck(v) => {
                    struct_ser.serialize_field("httpHealthCheck", v)?;
                }
                health_check::HealthChecker::TcpHealthCheck(v) => {
                    struct_ser.serialize_field("tcpHealthCheck", v)?;
                }
                health_check::HealthChecker::GrpcHealthCheck(v) => {
                    struct_ser.serialize_field("grpcHealthCheck", v)?;
                }
                health_check::HealthChecker::CustomHealthCheck(v) => {
                    struct_ser.serialize_field("customHealthCheck", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timeout",
            "interval",
            "initial_jitter",
            "initialJitter",
            "interval_jitter",
            "intervalJitter",
            "interval_jitter_percent",
            "intervalJitterPercent",
            "unhealthy_threshold",
            "unhealthyThreshold",
            "healthy_threshold",
            "healthyThreshold",
            "alt_port",
            "altPort",
            "reuse_connection",
            "reuseConnection",
            "no_traffic_interval",
            "noTrafficInterval",
            "no_traffic_healthy_interval",
            "noTrafficHealthyInterval",
            "unhealthy_interval",
            "unhealthyInterval",
            "unhealthy_edge_interval",
            "unhealthyEdgeInterval",
            "healthy_edge_interval",
            "healthyEdgeInterval",
            "event_log_path",
            "eventLogPath",
            "event_service",
            "eventService",
            "always_log_health_check_failures",
            "alwaysLogHealthCheckFailures",
            "tls_options",
            "tlsOptions",
            "transport_socket_match_criteria",
            "transportSocketMatchCriteria",
            "http_health_check",
            "httpHealthCheck",
            "tcp_health_check",
            "tcpHealthCheck",
            "grpc_health_check",
            "grpcHealthCheck",
            "custom_health_check",
            "customHealthCheck",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timeout,
            Interval,
            InitialJitter,
            IntervalJitter,
            IntervalJitterPercent,
            UnhealthyThreshold,
            HealthyThreshold,
            AltPort,
            ReuseConnection,
            NoTrafficInterval,
            NoTrafficHealthyInterval,
            UnhealthyInterval,
            UnhealthyEdgeInterval,
            HealthyEdgeInterval,
            EventLogPath,
            EventService,
            AlwaysLogHealthCheckFailures,
            TlsOptions,
            TransportSocketMatchCriteria,
            HttpHealthCheck,
            TcpHealthCheck,
            GrpcHealthCheck,
            CustomHealthCheck,
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
                            "timeout" => Ok(GeneratedField::Timeout),
                            "interval" => Ok(GeneratedField::Interval),
                            "initialJitter" | "initial_jitter" => Ok(GeneratedField::InitialJitter),
                            "intervalJitter" | "interval_jitter" => Ok(GeneratedField::IntervalJitter),
                            "intervalJitterPercent" | "interval_jitter_percent" => Ok(GeneratedField::IntervalJitterPercent),
                            "unhealthyThreshold" | "unhealthy_threshold" => Ok(GeneratedField::UnhealthyThreshold),
                            "healthyThreshold" | "healthy_threshold" => Ok(GeneratedField::HealthyThreshold),
                            "altPort" | "alt_port" => Ok(GeneratedField::AltPort),
                            "reuseConnection" | "reuse_connection" => Ok(GeneratedField::ReuseConnection),
                            "noTrafficInterval" | "no_traffic_interval" => Ok(GeneratedField::NoTrafficInterval),
                            "noTrafficHealthyInterval" | "no_traffic_healthy_interval" => Ok(GeneratedField::NoTrafficHealthyInterval),
                            "unhealthyInterval" | "unhealthy_interval" => Ok(GeneratedField::UnhealthyInterval),
                            "unhealthyEdgeInterval" | "unhealthy_edge_interval" => Ok(GeneratedField::UnhealthyEdgeInterval),
                            "healthyEdgeInterval" | "healthy_edge_interval" => Ok(GeneratedField::HealthyEdgeInterval),
                            "eventLogPath" | "event_log_path" => Ok(GeneratedField::EventLogPath),
                            "eventService" | "event_service" => Ok(GeneratedField::EventService),
                            "alwaysLogHealthCheckFailures" | "always_log_health_check_failures" => Ok(GeneratedField::AlwaysLogHealthCheckFailures),
                            "tlsOptions" | "tls_options" => Ok(GeneratedField::TlsOptions),
                            "transportSocketMatchCriteria" | "transport_socket_match_criteria" => Ok(GeneratedField::TransportSocketMatchCriteria),
                            "httpHealthCheck" | "http_health_check" => Ok(GeneratedField::HttpHealthCheck),
                            "tcpHealthCheck" | "tcp_health_check" => Ok(GeneratedField::TcpHealthCheck),
                            "grpcHealthCheck" | "grpc_health_check" => Ok(GeneratedField::GrpcHealthCheck),
                            "customHealthCheck" | "custom_health_check" => Ok(GeneratedField::CustomHealthCheck),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timeout__ = None;
                let mut interval__ = None;
                let mut initial_jitter__ = None;
                let mut interval_jitter__ = None;
                let mut interval_jitter_percent__ = None;
                let mut unhealthy_threshold__ = None;
                let mut healthy_threshold__ = None;
                let mut alt_port__ = None;
                let mut reuse_connection__ = None;
                let mut no_traffic_interval__ = None;
                let mut no_traffic_healthy_interval__ = None;
                let mut unhealthy_interval__ = None;
                let mut unhealthy_edge_interval__ = None;
                let mut healthy_edge_interval__ = None;
                let mut event_log_path__ = None;
                let mut event_service__ = None;
                let mut always_log_health_check_failures__ = None;
                let mut tls_options__ = None;
                let mut transport_socket_match_criteria__ = None;
                let mut health_checker__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::InitialJitter => {
                            if initial_jitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialJitter"));
                            }
                            initial_jitter__ = map.next_value()?;
                        }
                        GeneratedField::IntervalJitter => {
                            if interval_jitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intervalJitter"));
                            }
                            interval_jitter__ = map.next_value()?;
                        }
                        GeneratedField::IntervalJitterPercent => {
                            if interval_jitter_percent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intervalJitterPercent"));
                            }
                            interval_jitter_percent__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UnhealthyThreshold => {
                            if unhealthy_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unhealthyThreshold"));
                            }
                            unhealthy_threshold__ = map.next_value()?;
                        }
                        GeneratedField::HealthyThreshold => {
                            if healthy_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthyThreshold"));
                            }
                            healthy_threshold__ = map.next_value()?;
                        }
                        GeneratedField::AltPort => {
                            if alt_port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("altPort"));
                            }
                            alt_port__ = map.next_value()?;
                        }
                        GeneratedField::ReuseConnection => {
                            if reuse_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reuseConnection"));
                            }
                            reuse_connection__ = map.next_value()?;
                        }
                        GeneratedField::NoTrafficInterval => {
                            if no_traffic_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noTrafficInterval"));
                            }
                            no_traffic_interval__ = map.next_value()?;
                        }
                        GeneratedField::NoTrafficHealthyInterval => {
                            if no_traffic_healthy_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noTrafficHealthyInterval"));
                            }
                            no_traffic_healthy_interval__ = map.next_value()?;
                        }
                        GeneratedField::UnhealthyInterval => {
                            if unhealthy_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unhealthyInterval"));
                            }
                            unhealthy_interval__ = map.next_value()?;
                        }
                        GeneratedField::UnhealthyEdgeInterval => {
                            if unhealthy_edge_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unhealthyEdgeInterval"));
                            }
                            unhealthy_edge_interval__ = map.next_value()?;
                        }
                        GeneratedField::HealthyEdgeInterval => {
                            if healthy_edge_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthyEdgeInterval"));
                            }
                            healthy_edge_interval__ = map.next_value()?;
                        }
                        GeneratedField::EventLogPath => {
                            if event_log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventLogPath"));
                            }
                            event_log_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::EventService => {
                            if event_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventService"));
                            }
                            event_service__ = map.next_value()?;
                        }
                        GeneratedField::AlwaysLogHealthCheckFailures => {
                            if always_log_health_check_failures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alwaysLogHealthCheckFailures"));
                            }
                            always_log_health_check_failures__ = Some(map.next_value()?);
                        }
                        GeneratedField::TlsOptions => {
                            if tls_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsOptions"));
                            }
                            tls_options__ = map.next_value()?;
                        }
                        GeneratedField::TransportSocketMatchCriteria => {
                            if transport_socket_match_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportSocketMatchCriteria"));
                            }
                            transport_socket_match_criteria__ = map.next_value()?;
                        }
                        GeneratedField::HttpHealthCheck => {
                            if health_checker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpHealthCheck"));
                            }
                            health_checker__ = map.next_value::<::std::option::Option<_>>()?.map(health_check::HealthChecker::HttpHealthCheck)
;
                        }
                        GeneratedField::TcpHealthCheck => {
                            if health_checker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tcpHealthCheck"));
                            }
                            health_checker__ = map.next_value::<::std::option::Option<_>>()?.map(health_check::HealthChecker::TcpHealthCheck)
;
                        }
                        GeneratedField::GrpcHealthCheck => {
                            if health_checker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcHealthCheck"));
                            }
                            health_checker__ = map.next_value::<::std::option::Option<_>>()?.map(health_check::HealthChecker::GrpcHealthCheck)
;
                        }
                        GeneratedField::CustomHealthCheck => {
                            if health_checker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customHealthCheck"));
                            }
                            health_checker__ = map.next_value::<::std::option::Option<_>>()?.map(health_check::HealthChecker::CustomHealthCheck)
;
                        }
                    }
                }
                Ok(HealthCheck {
                    timeout: timeout__,
                    interval: interval__,
                    initial_jitter: initial_jitter__,
                    interval_jitter: interval_jitter__,
                    interval_jitter_percent: interval_jitter_percent__.unwrap_or_default(),
                    unhealthy_threshold: unhealthy_threshold__,
                    healthy_threshold: healthy_threshold__,
                    alt_port: alt_port__,
                    reuse_connection: reuse_connection__,
                    no_traffic_interval: no_traffic_interval__,
                    no_traffic_healthy_interval: no_traffic_healthy_interval__,
                    unhealthy_interval: unhealthy_interval__,
                    unhealthy_edge_interval: unhealthy_edge_interval__,
                    healthy_edge_interval: healthy_edge_interval__,
                    event_log_path: event_log_path__.unwrap_or_default(),
                    event_service: event_service__,
                    always_log_health_check_failures: always_log_health_check_failures__.unwrap_or_default(),
                    tls_options: tls_options__,
                    transport_socket_match_criteria: transport_socket_match_criteria__,
                    health_checker: health_checker__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for health_check::CustomHealthCheck {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck.CustomHealthCheck", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                health_check::custom_health_check::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for health_check::CustomHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
            type Value = health_check::CustomHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck.CustomHealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<health_check::CustomHealthCheck, V::Error>
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
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(health_check::custom_health_check::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(health_check::CustomHealthCheck {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck.CustomHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for health_check::GrpcHealthCheck {
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
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.initial_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck.GrpcHealthCheck", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.initial_metadata.is_empty() {
            struct_ser.serialize_field("initialMetadata", &self.initial_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for health_check::GrpcHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "authority",
            "initial_metadata",
            "initialMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            Authority,
            InitialMetadata,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "initialMetadata" | "initial_metadata" => Ok(GeneratedField::InitialMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = health_check::GrpcHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck.GrpcHealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<health_check::GrpcHealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut authority__ = None;
                let mut initial_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::InitialMetadata => {
                            if initial_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialMetadata"));
                            }
                            initial_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(health_check::GrpcHealthCheck {
                    service_name: service_name__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                    initial_metadata: initial_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck.GrpcHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for health_check::HttpHealthCheck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host.is_empty() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if self.send.is_some() {
            len += 1;
        }
        if !self.receive.is_empty() {
            len += 1;
        }
        if self.response_buffer_size.is_some() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_remove.is_empty() {
            len += 1;
        }
        if !self.expected_statuses.is_empty() {
            len += 1;
        }
        if !self.retriable_statuses.is_empty() {
            len += 1;
        }
        if self.codec_client_type != 0 {
            len += 1;
        }
        if self.service_name_matcher.is_some() {
            len += 1;
        }
        if self.method != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck.HttpHealthCheck", len)?;
        if !self.host.is_empty() {
            struct_ser.serialize_field("host", &self.host)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.send.as_ref() {
            struct_ser.serialize_field("send", v)?;
        }
        if !self.receive.is_empty() {
            struct_ser.serialize_field("receive", &self.receive)?;
        }
        if let Some(v) = self.response_buffer_size.as_ref() {
            struct_ser.serialize_field("responseBufferSize", v)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("requestHeadersToAdd", &self.request_headers_to_add)?;
        }
        if !self.request_headers_to_remove.is_empty() {
            struct_ser.serialize_field("requestHeadersToRemove", &self.request_headers_to_remove)?;
        }
        if !self.expected_statuses.is_empty() {
            struct_ser.serialize_field("expectedStatuses", &self.expected_statuses)?;
        }
        if !self.retriable_statuses.is_empty() {
            struct_ser.serialize_field("retriableStatuses", &self.retriable_statuses)?;
        }
        if self.codec_client_type != 0 {
            let v = super::super::super::r#type::v3::CodecClientType::from_i32(self.codec_client_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.codec_client_type)))?;
            struct_ser.serialize_field("codecClientType", &v)?;
        }
        if let Some(v) = self.service_name_matcher.as_ref() {
            struct_ser.serialize_field("serviceNameMatcher", v)?;
        }
        if self.method != 0 {
            let v = RequestMethod::from_i32(self.method)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.method)))?;
            struct_ser.serialize_field("method", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for health_check::HttpHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host",
            "path",
            "send",
            "receive",
            "response_buffer_size",
            "responseBufferSize",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "request_headers_to_remove",
            "requestHeadersToRemove",
            "expected_statuses",
            "expectedStatuses",
            "retriable_statuses",
            "retriableStatuses",
            "codec_client_type",
            "codecClientType",
            "service_name_matcher",
            "serviceNameMatcher",
            "method",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Host,
            Path,
            Send,
            Receive,
            ResponseBufferSize,
            RequestHeadersToAdd,
            RequestHeadersToRemove,
            ExpectedStatuses,
            RetriableStatuses,
            CodecClientType,
            ServiceNameMatcher,
            Method,
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
                            "host" => Ok(GeneratedField::Host),
                            "path" => Ok(GeneratedField::Path),
                            "send" => Ok(GeneratedField::Send),
                            "receive" => Ok(GeneratedField::Receive),
                            "responseBufferSize" | "response_buffer_size" => Ok(GeneratedField::ResponseBufferSize),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "requestHeadersToRemove" | "request_headers_to_remove" => Ok(GeneratedField::RequestHeadersToRemove),
                            "expectedStatuses" | "expected_statuses" => Ok(GeneratedField::ExpectedStatuses),
                            "retriableStatuses" | "retriable_statuses" => Ok(GeneratedField::RetriableStatuses),
                            "codecClientType" | "codec_client_type" => Ok(GeneratedField::CodecClientType),
                            "serviceNameMatcher" | "service_name_matcher" => Ok(GeneratedField::ServiceNameMatcher),
                            "method" => Ok(GeneratedField::Method),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = health_check::HttpHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck.HttpHealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<health_check::HttpHealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host__ = None;
                let mut path__ = None;
                let mut send__ = None;
                let mut receive__ = None;
                let mut response_buffer_size__ = None;
                let mut request_headers_to_add__ = None;
                let mut request_headers_to_remove__ = None;
                let mut expected_statuses__ = None;
                let mut retriable_statuses__ = None;
                let mut codec_client_type__ = None;
                let mut service_name_matcher__ = None;
                let mut method__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = Some(map.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Send => {
                            if send__.is_some() {
                                return Err(serde::de::Error::duplicate_field("send"));
                            }
                            send__ = map.next_value()?;
                        }
                        GeneratedField::Receive => {
                            if receive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receive"));
                            }
                            receive__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseBufferSize => {
                            if response_buffer_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBufferSize"));
                            }
                            response_buffer_size__ = map.next_value()?;
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToRemove => {
                            if request_headers_to_remove__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToRemove"));
                            }
                            request_headers_to_remove__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExpectedStatuses => {
                            if expected_statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expectedStatuses"));
                            }
                            expected_statuses__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetriableStatuses => {
                            if retriable_statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retriableStatuses"));
                            }
                            retriable_statuses__ = Some(map.next_value()?);
                        }
                        GeneratedField::CodecClientType => {
                            if codec_client_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codecClientType"));
                            }
                            codec_client_type__ = Some(map.next_value::<super::super::super::r#type::v3::CodecClientType>()? as i32);
                        }
                        GeneratedField::ServiceNameMatcher => {
                            if service_name_matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceNameMatcher"));
                            }
                            service_name_matcher__ = map.next_value()?;
                        }
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = Some(map.next_value::<RequestMethod>()? as i32);
                        }
                    }
                }
                Ok(health_check::HttpHealthCheck {
                    host: host__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                    send: send__,
                    receive: receive__.unwrap_or_default(),
                    response_buffer_size: response_buffer_size__,
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    request_headers_to_remove: request_headers_to_remove__.unwrap_or_default(),
                    expected_statuses: expected_statuses__.unwrap_or_default(),
                    retriable_statuses: retriable_statuses__.unwrap_or_default(),
                    codec_client_type: codec_client_type__.unwrap_or_default(),
                    service_name_matcher: service_name_matcher__,
                    method: method__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck.HttpHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for health_check::Payload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck.Payload", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                health_check::payload::Payload::Text(v) => {
                    struct_ser.serialize_field("text", v)?;
                }
                health_check::payload::Payload::Binary(v) => {
                    struct_ser.serialize_field("binary", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for health_check::Payload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "text",
            "binary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Text,
            Binary,
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
                            "text" => Ok(GeneratedField::Text),
                            "binary" => Ok(GeneratedField::Binary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = health_check::Payload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck.Payload")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<health_check::Payload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Text => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            payload__ = map.next_value::<::std::option::Option<_>>()?.map(health_check::payload::Payload::Text);
                        }
                        GeneratedField::Binary => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("binary"));
                            }
                            payload__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| health_check::payload::Payload::Binary(x.0));
                        }
                    }
                }
                Ok(health_check::Payload {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck.Payload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for health_check::RedisHealthCheck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck.RedisHealthCheck", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for health_check::RedisHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = health_check::RedisHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck.RedisHealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<health_check::RedisHealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(health_check::RedisHealthCheck {
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck.RedisHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for health_check::TcpHealthCheck {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.send.is_some() {
            len += 1;
        }
        if !self.receive.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck.TcpHealthCheck", len)?;
        if let Some(v) = self.send.as_ref() {
            struct_ser.serialize_field("send", v)?;
        }
        if !self.receive.is_empty() {
            struct_ser.serialize_field("receive", &self.receive)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for health_check::TcpHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "send",
            "receive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Send,
            Receive,
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
                            "send" => Ok(GeneratedField::Send),
                            "receive" => Ok(GeneratedField::Receive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = health_check::TcpHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck.TcpHealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<health_check::TcpHealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut send__ = None;
                let mut receive__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Send => {
                            if send__.is_some() {
                                return Err(serde::de::Error::duplicate_field("send"));
                            }
                            send__ = map.next_value()?;
                        }
                        GeneratedField::Receive => {
                            if receive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receive"));
                            }
                            receive__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(health_check::TcpHealthCheck {
                    send: send__,
                    receive: receive__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck.TcpHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for health_check::TlsOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.alpn_protocols.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthCheck.TlsOptions", len)?;
        if !self.alpn_protocols.is_empty() {
            struct_ser.serialize_field("alpnProtocols", &self.alpn_protocols)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for health_check::TlsOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "alpn_protocols",
            "alpnProtocols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AlpnProtocols,
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
                            "alpnProtocols" | "alpn_protocols" => Ok(GeneratedField::AlpnProtocols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = health_check::TlsOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthCheck.TlsOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<health_check::TlsOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut alpn_protocols__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AlpnProtocols => {
                            if alpn_protocols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alpnProtocols"));
                            }
                            alpn_protocols__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(health_check::TlsOptions {
                    alpn_protocols: alpn_protocols__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthCheck.TlsOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Healthy => "HEALTHY",
            Self::Unhealthy => "UNHEALTHY",
            Self::Draining => "DRAINING",
            Self::Timeout => "TIMEOUT",
            Self::Degraded => "DEGRADED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HealthStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "HEALTHY",
            "UNHEALTHY",
            "DRAINING",
            "TIMEOUT",
            "DEGRADED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthStatus;

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
                    .and_then(HealthStatus::from_i32)
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
                    .and_then(HealthStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(HealthStatus::Unknown),
                    "HEALTHY" => Ok(HealthStatus::Healthy),
                    "UNHEALTHY" => Ok(HealthStatus::Unhealthy),
                    "DRAINING" => Ok(HealthStatus::Draining),
                    "TIMEOUT" => Ok(HealthStatus::Timeout),
                    "DEGRADED" => Ok(HealthStatus::Degraded),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HealthStatusSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.statuses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HealthStatusSet", len)?;
        if !self.statuses.is_empty() {
            let v = self.statuses.iter().cloned().map(|v| {
                HealthStatus::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("statuses", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthStatusSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "statuses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Statuses,
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
                            "statuses" => Ok(GeneratedField::Statuses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthStatusSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HealthStatusSet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthStatusSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut statuses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Statuses => {
                            if statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statuses"));
                            }
                            statuses__ = Some(map.next_value::<Vec<HealthStatus>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(HealthStatusSet {
                    statuses: statuses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HealthStatusSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Http1ProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allow_absolute_url.is_some() {
            len += 1;
        }
        if self.accept_http_10 {
            len += 1;
        }
        if !self.default_host_for_http_10.is_empty() {
            len += 1;
        }
        if self.header_key_format.is_some() {
            len += 1;
        }
        if self.enable_trailers {
            len += 1;
        }
        if self.allow_chunked_length {
            len += 1;
        }
        if self.override_stream_error_on_invalid_http_message.is_some() {
            len += 1;
        }
        if self.send_fully_qualified_url {
            len += 1;
        }
        if self.use_balsa_parser.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Http1ProtocolOptions", len)?;
        if let Some(v) = self.allow_absolute_url.as_ref() {
            struct_ser.serialize_field("allowAbsoluteUrl", v)?;
        }
        if self.accept_http_10 {
            struct_ser.serialize_field("acceptHttp10", &self.accept_http_10)?;
        }
        if !self.default_host_for_http_10.is_empty() {
            struct_ser.serialize_field("defaultHostForHttp10", &self.default_host_for_http_10)?;
        }
        if let Some(v) = self.header_key_format.as_ref() {
            struct_ser.serialize_field("headerKeyFormat", v)?;
        }
        if self.enable_trailers {
            struct_ser.serialize_field("enableTrailers", &self.enable_trailers)?;
        }
        if self.allow_chunked_length {
            struct_ser.serialize_field("allowChunkedLength", &self.allow_chunked_length)?;
        }
        if let Some(v) = self.override_stream_error_on_invalid_http_message.as_ref() {
            struct_ser.serialize_field("overrideStreamErrorOnInvalidHttpMessage", v)?;
        }
        if self.send_fully_qualified_url {
            struct_ser.serialize_field("sendFullyQualifiedUrl", &self.send_fully_qualified_url)?;
        }
        if let Some(v) = self.use_balsa_parser.as_ref() {
            struct_ser.serialize_field("useBalsaParser", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Http1ProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allow_absolute_url",
            "allowAbsoluteUrl",
            "accept_http_10",
            "acceptHttp10",
            "default_host_for_http_10",
            "defaultHostForHttp10",
            "header_key_format",
            "headerKeyFormat",
            "enable_trailers",
            "enableTrailers",
            "allow_chunked_length",
            "allowChunkedLength",
            "override_stream_error_on_invalid_http_message",
            "overrideStreamErrorOnInvalidHttpMessage",
            "send_fully_qualified_url",
            "sendFullyQualifiedUrl",
            "use_balsa_parser",
            "useBalsaParser",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowAbsoluteUrl,
            AcceptHttp10,
            DefaultHostForHttp10,
            HeaderKeyFormat,
            EnableTrailers,
            AllowChunkedLength,
            OverrideStreamErrorOnInvalidHttpMessage,
            SendFullyQualifiedUrl,
            UseBalsaParser,
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
                            "allowAbsoluteUrl" | "allow_absolute_url" => Ok(GeneratedField::AllowAbsoluteUrl),
                            "acceptHttp10" | "accept_http_10" => Ok(GeneratedField::AcceptHttp10),
                            "defaultHostForHttp10" | "default_host_for_http_10" => Ok(GeneratedField::DefaultHostForHttp10),
                            "headerKeyFormat" | "header_key_format" => Ok(GeneratedField::HeaderKeyFormat),
                            "enableTrailers" | "enable_trailers" => Ok(GeneratedField::EnableTrailers),
                            "allowChunkedLength" | "allow_chunked_length" => Ok(GeneratedField::AllowChunkedLength),
                            "overrideStreamErrorOnInvalidHttpMessage" | "override_stream_error_on_invalid_http_message" => Ok(GeneratedField::OverrideStreamErrorOnInvalidHttpMessage),
                            "sendFullyQualifiedUrl" | "send_fully_qualified_url" => Ok(GeneratedField::SendFullyQualifiedUrl),
                            "useBalsaParser" | "use_balsa_parser" => Ok(GeneratedField::UseBalsaParser),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Http1ProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Http1ProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Http1ProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allow_absolute_url__ = None;
                let mut accept_http_10__ = None;
                let mut default_host_for_http_10__ = None;
                let mut header_key_format__ = None;
                let mut enable_trailers__ = None;
                let mut allow_chunked_length__ = None;
                let mut override_stream_error_on_invalid_http_message__ = None;
                let mut send_fully_qualified_url__ = None;
                let mut use_balsa_parser__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowAbsoluteUrl => {
                            if allow_absolute_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowAbsoluteUrl"));
                            }
                            allow_absolute_url__ = map.next_value()?;
                        }
                        GeneratedField::AcceptHttp10 => {
                            if accept_http_10__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acceptHttp10"));
                            }
                            accept_http_10__ = Some(map.next_value()?);
                        }
                        GeneratedField::DefaultHostForHttp10 => {
                            if default_host_for_http_10__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultHostForHttp10"));
                            }
                            default_host_for_http_10__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeaderKeyFormat => {
                            if header_key_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerKeyFormat"));
                            }
                            header_key_format__ = map.next_value()?;
                        }
                        GeneratedField::EnableTrailers => {
                            if enable_trailers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableTrailers"));
                            }
                            enable_trailers__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowChunkedLength => {
                            if allow_chunked_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowChunkedLength"));
                            }
                            allow_chunked_length__ = Some(map.next_value()?);
                        }
                        GeneratedField::OverrideStreamErrorOnInvalidHttpMessage => {
                            if override_stream_error_on_invalid_http_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideStreamErrorOnInvalidHttpMessage"));
                            }
                            override_stream_error_on_invalid_http_message__ = map.next_value()?;
                        }
                        GeneratedField::SendFullyQualifiedUrl => {
                            if send_fully_qualified_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sendFullyQualifiedUrl"));
                            }
                            send_fully_qualified_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseBalsaParser => {
                            if use_balsa_parser__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useBalsaParser"));
                            }
                            use_balsa_parser__ = map.next_value()?;
                        }
                    }
                }
                Ok(Http1ProtocolOptions {
                    allow_absolute_url: allow_absolute_url__,
                    accept_http_10: accept_http_10__.unwrap_or_default(),
                    default_host_for_http_10: default_host_for_http_10__.unwrap_or_default(),
                    header_key_format: header_key_format__,
                    enable_trailers: enable_trailers__.unwrap_or_default(),
                    allow_chunked_length: allow_chunked_length__.unwrap_or_default(),
                    override_stream_error_on_invalid_http_message: override_stream_error_on_invalid_http_message__,
                    send_fully_qualified_url: send_fully_qualified_url__.unwrap_or_default(),
                    use_balsa_parser: use_balsa_parser__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Http1ProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http1_protocol_options::HeaderKeyFormat {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header_format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat", len)?;
        if let Some(v) = self.header_format.as_ref() {
            match v {
                http1_protocol_options::header_key_format::HeaderFormat::ProperCaseWords(v) => {
                    struct_ser.serialize_field("properCaseWords", v)?;
                }
                http1_protocol_options::header_key_format::HeaderFormat::StatefulFormatter(v) => {
                    struct_ser.serialize_field("statefulFormatter", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http1_protocol_options::HeaderKeyFormat {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proper_case_words",
            "properCaseWords",
            "stateful_formatter",
            "statefulFormatter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProperCaseWords,
            StatefulFormatter,
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
                            "properCaseWords" | "proper_case_words" => Ok(GeneratedField::ProperCaseWords),
                            "statefulFormatter" | "stateful_formatter" => Ok(GeneratedField::StatefulFormatter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http1_protocol_options::HeaderKeyFormat;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http1_protocol_options::HeaderKeyFormat, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_format__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProperCaseWords => {
                            if header_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properCaseWords"));
                            }
                            header_format__ = map.next_value::<::std::option::Option<_>>()?.map(http1_protocol_options::header_key_format::HeaderFormat::ProperCaseWords)
;
                        }
                        GeneratedField::StatefulFormatter => {
                            if header_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statefulFormatter"));
                            }
                            header_format__ = map.next_value::<::std::option::Option<_>>()?.map(http1_protocol_options::header_key_format::HeaderFormat::StatefulFormatter)
;
                        }
                    }
                }
                Ok(http1_protocol_options::HeaderKeyFormat {
                    header_format: header_format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http1_protocol_options::header_key_format::ProperCaseWords {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat.ProperCaseWords", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http1_protocol_options::header_key_format::ProperCaseWords {
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
            type Value = http1_protocol_options::header_key_format::ProperCaseWords;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat.ProperCaseWords")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http1_protocol_options::header_key_format::ProperCaseWords, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(http1_protocol_options::header_key_format::ProperCaseWords {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Http1ProtocolOptions.HeaderKeyFormat.ProperCaseWords", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Http2ProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.hpack_table_size.is_some() {
            len += 1;
        }
        if self.max_concurrent_streams.is_some() {
            len += 1;
        }
        if self.initial_stream_window_size.is_some() {
            len += 1;
        }
        if self.initial_connection_window_size.is_some() {
            len += 1;
        }
        if self.allow_connect {
            len += 1;
        }
        if self.allow_metadata {
            len += 1;
        }
        if self.max_outbound_frames.is_some() {
            len += 1;
        }
        if self.max_outbound_control_frames.is_some() {
            len += 1;
        }
        if self.max_consecutive_inbound_frames_with_empty_payload.is_some() {
            len += 1;
        }
        if self.max_inbound_priority_frames_per_stream.is_some() {
            len += 1;
        }
        if self.max_inbound_window_update_frames_per_data_frame_sent.is_some() {
            len += 1;
        }
        if self.stream_error_on_invalid_http_messaging {
            len += 1;
        }
        if self.override_stream_error_on_invalid_http_message.is_some() {
            len += 1;
        }
        if !self.custom_settings_parameters.is_empty() {
            len += 1;
        }
        if self.connection_keepalive.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Http2ProtocolOptions", len)?;
        if let Some(v) = self.hpack_table_size.as_ref() {
            struct_ser.serialize_field("hpackTableSize", v)?;
        }
        if let Some(v) = self.max_concurrent_streams.as_ref() {
            struct_ser.serialize_field("maxConcurrentStreams", v)?;
        }
        if let Some(v) = self.initial_stream_window_size.as_ref() {
            struct_ser.serialize_field("initialStreamWindowSize", v)?;
        }
        if let Some(v) = self.initial_connection_window_size.as_ref() {
            struct_ser.serialize_field("initialConnectionWindowSize", v)?;
        }
        if self.allow_connect {
            struct_ser.serialize_field("allowConnect", &self.allow_connect)?;
        }
        if self.allow_metadata {
            struct_ser.serialize_field("allowMetadata", &self.allow_metadata)?;
        }
        if let Some(v) = self.max_outbound_frames.as_ref() {
            struct_ser.serialize_field("maxOutboundFrames", v)?;
        }
        if let Some(v) = self.max_outbound_control_frames.as_ref() {
            struct_ser.serialize_field("maxOutboundControlFrames", v)?;
        }
        if let Some(v) = self.max_consecutive_inbound_frames_with_empty_payload.as_ref() {
            struct_ser.serialize_field("maxConsecutiveInboundFramesWithEmptyPayload", v)?;
        }
        if let Some(v) = self.max_inbound_priority_frames_per_stream.as_ref() {
            struct_ser.serialize_field("maxInboundPriorityFramesPerStream", v)?;
        }
        if let Some(v) = self.max_inbound_window_update_frames_per_data_frame_sent.as_ref() {
            struct_ser.serialize_field("maxInboundWindowUpdateFramesPerDataFrameSent", v)?;
        }
        if self.stream_error_on_invalid_http_messaging {
            struct_ser.serialize_field("streamErrorOnInvalidHttpMessaging", &self.stream_error_on_invalid_http_messaging)?;
        }
        if let Some(v) = self.override_stream_error_on_invalid_http_message.as_ref() {
            struct_ser.serialize_field("overrideStreamErrorOnInvalidHttpMessage", v)?;
        }
        if !self.custom_settings_parameters.is_empty() {
            struct_ser.serialize_field("customSettingsParameters", &self.custom_settings_parameters)?;
        }
        if let Some(v) = self.connection_keepalive.as_ref() {
            struct_ser.serialize_field("connectionKeepalive", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Http2ProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hpack_table_size",
            "hpackTableSize",
            "max_concurrent_streams",
            "maxConcurrentStreams",
            "initial_stream_window_size",
            "initialStreamWindowSize",
            "initial_connection_window_size",
            "initialConnectionWindowSize",
            "allow_connect",
            "allowConnect",
            "allow_metadata",
            "allowMetadata",
            "max_outbound_frames",
            "maxOutboundFrames",
            "max_outbound_control_frames",
            "maxOutboundControlFrames",
            "max_consecutive_inbound_frames_with_empty_payload",
            "maxConsecutiveInboundFramesWithEmptyPayload",
            "max_inbound_priority_frames_per_stream",
            "maxInboundPriorityFramesPerStream",
            "max_inbound_window_update_frames_per_data_frame_sent",
            "maxInboundWindowUpdateFramesPerDataFrameSent",
            "stream_error_on_invalid_http_messaging",
            "streamErrorOnInvalidHttpMessaging",
            "override_stream_error_on_invalid_http_message",
            "overrideStreamErrorOnInvalidHttpMessage",
            "custom_settings_parameters",
            "customSettingsParameters",
            "connection_keepalive",
            "connectionKeepalive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HpackTableSize,
            MaxConcurrentStreams,
            InitialStreamWindowSize,
            InitialConnectionWindowSize,
            AllowConnect,
            AllowMetadata,
            MaxOutboundFrames,
            MaxOutboundControlFrames,
            MaxConsecutiveInboundFramesWithEmptyPayload,
            MaxInboundPriorityFramesPerStream,
            MaxInboundWindowUpdateFramesPerDataFrameSent,
            StreamErrorOnInvalidHttpMessaging,
            OverrideStreamErrorOnInvalidHttpMessage,
            CustomSettingsParameters,
            ConnectionKeepalive,
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
                            "hpackTableSize" | "hpack_table_size" => Ok(GeneratedField::HpackTableSize),
                            "maxConcurrentStreams" | "max_concurrent_streams" => Ok(GeneratedField::MaxConcurrentStreams),
                            "initialStreamWindowSize" | "initial_stream_window_size" => Ok(GeneratedField::InitialStreamWindowSize),
                            "initialConnectionWindowSize" | "initial_connection_window_size" => Ok(GeneratedField::InitialConnectionWindowSize),
                            "allowConnect" | "allow_connect" => Ok(GeneratedField::AllowConnect),
                            "allowMetadata" | "allow_metadata" => Ok(GeneratedField::AllowMetadata),
                            "maxOutboundFrames" | "max_outbound_frames" => Ok(GeneratedField::MaxOutboundFrames),
                            "maxOutboundControlFrames" | "max_outbound_control_frames" => Ok(GeneratedField::MaxOutboundControlFrames),
                            "maxConsecutiveInboundFramesWithEmptyPayload" | "max_consecutive_inbound_frames_with_empty_payload" => Ok(GeneratedField::MaxConsecutiveInboundFramesWithEmptyPayload),
                            "maxInboundPriorityFramesPerStream" | "max_inbound_priority_frames_per_stream" => Ok(GeneratedField::MaxInboundPriorityFramesPerStream),
                            "maxInboundWindowUpdateFramesPerDataFrameSent" | "max_inbound_window_update_frames_per_data_frame_sent" => Ok(GeneratedField::MaxInboundWindowUpdateFramesPerDataFrameSent),
                            "streamErrorOnInvalidHttpMessaging" | "stream_error_on_invalid_http_messaging" => Ok(GeneratedField::StreamErrorOnInvalidHttpMessaging),
                            "overrideStreamErrorOnInvalidHttpMessage" | "override_stream_error_on_invalid_http_message" => Ok(GeneratedField::OverrideStreamErrorOnInvalidHttpMessage),
                            "customSettingsParameters" | "custom_settings_parameters" => Ok(GeneratedField::CustomSettingsParameters),
                            "connectionKeepalive" | "connection_keepalive" => Ok(GeneratedField::ConnectionKeepalive),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Http2ProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Http2ProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Http2ProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hpack_table_size__ = None;
                let mut max_concurrent_streams__ = None;
                let mut initial_stream_window_size__ = None;
                let mut initial_connection_window_size__ = None;
                let mut allow_connect__ = None;
                let mut allow_metadata__ = None;
                let mut max_outbound_frames__ = None;
                let mut max_outbound_control_frames__ = None;
                let mut max_consecutive_inbound_frames_with_empty_payload__ = None;
                let mut max_inbound_priority_frames_per_stream__ = None;
                let mut max_inbound_window_update_frames_per_data_frame_sent__ = None;
                let mut stream_error_on_invalid_http_messaging__ = None;
                let mut override_stream_error_on_invalid_http_message__ = None;
                let mut custom_settings_parameters__ = None;
                let mut connection_keepalive__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HpackTableSize => {
                            if hpack_table_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hpackTableSize"));
                            }
                            hpack_table_size__ = map.next_value()?;
                        }
                        GeneratedField::MaxConcurrentStreams => {
                            if max_concurrent_streams__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConcurrentStreams"));
                            }
                            max_concurrent_streams__ = map.next_value()?;
                        }
                        GeneratedField::InitialStreamWindowSize => {
                            if initial_stream_window_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStreamWindowSize"));
                            }
                            initial_stream_window_size__ = map.next_value()?;
                        }
                        GeneratedField::InitialConnectionWindowSize => {
                            if initial_connection_window_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialConnectionWindowSize"));
                            }
                            initial_connection_window_size__ = map.next_value()?;
                        }
                        GeneratedField::AllowConnect => {
                            if allow_connect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowConnect"));
                            }
                            allow_connect__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowMetadata => {
                            if allow_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowMetadata"));
                            }
                            allow_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxOutboundFrames => {
                            if max_outbound_frames__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxOutboundFrames"));
                            }
                            max_outbound_frames__ = map.next_value()?;
                        }
                        GeneratedField::MaxOutboundControlFrames => {
                            if max_outbound_control_frames__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxOutboundControlFrames"));
                            }
                            max_outbound_control_frames__ = map.next_value()?;
                        }
                        GeneratedField::MaxConsecutiveInboundFramesWithEmptyPayload => {
                            if max_consecutive_inbound_frames_with_empty_payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConsecutiveInboundFramesWithEmptyPayload"));
                            }
                            max_consecutive_inbound_frames_with_empty_payload__ = map.next_value()?;
                        }
                        GeneratedField::MaxInboundPriorityFramesPerStream => {
                            if max_inbound_priority_frames_per_stream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInboundPriorityFramesPerStream"));
                            }
                            max_inbound_priority_frames_per_stream__ = map.next_value()?;
                        }
                        GeneratedField::MaxInboundWindowUpdateFramesPerDataFrameSent => {
                            if max_inbound_window_update_frames_per_data_frame_sent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInboundWindowUpdateFramesPerDataFrameSent"));
                            }
                            max_inbound_window_update_frames_per_data_frame_sent__ = map.next_value()?;
                        }
                        GeneratedField::StreamErrorOnInvalidHttpMessaging => {
                            if stream_error_on_invalid_http_messaging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamErrorOnInvalidHttpMessaging"));
                            }
                            stream_error_on_invalid_http_messaging__ = Some(map.next_value()?);
                        }
                        GeneratedField::OverrideStreamErrorOnInvalidHttpMessage => {
                            if override_stream_error_on_invalid_http_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideStreamErrorOnInvalidHttpMessage"));
                            }
                            override_stream_error_on_invalid_http_message__ = map.next_value()?;
                        }
                        GeneratedField::CustomSettingsParameters => {
                            if custom_settings_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customSettingsParameters"));
                            }
                            custom_settings_parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionKeepalive => {
                            if connection_keepalive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionKeepalive"));
                            }
                            connection_keepalive__ = map.next_value()?;
                        }
                    }
                }
                Ok(Http2ProtocolOptions {
                    hpack_table_size: hpack_table_size__,
                    max_concurrent_streams: max_concurrent_streams__,
                    initial_stream_window_size: initial_stream_window_size__,
                    initial_connection_window_size: initial_connection_window_size__,
                    allow_connect: allow_connect__.unwrap_or_default(),
                    allow_metadata: allow_metadata__.unwrap_or_default(),
                    max_outbound_frames: max_outbound_frames__,
                    max_outbound_control_frames: max_outbound_control_frames__,
                    max_consecutive_inbound_frames_with_empty_payload: max_consecutive_inbound_frames_with_empty_payload__,
                    max_inbound_priority_frames_per_stream: max_inbound_priority_frames_per_stream__,
                    max_inbound_window_update_frames_per_data_frame_sent: max_inbound_window_update_frames_per_data_frame_sent__,
                    stream_error_on_invalid_http_messaging: stream_error_on_invalid_http_messaging__.unwrap_or_default(),
                    override_stream_error_on_invalid_http_message: override_stream_error_on_invalid_http_message__,
                    custom_settings_parameters: custom_settings_parameters__.unwrap_or_default(),
                    connection_keepalive: connection_keepalive__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Http2ProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http2_protocol_options::SettingsParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.identifier.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Http2ProtocolOptions.SettingsParameter", len)?;
        if let Some(v) = self.identifier.as_ref() {
            struct_ser.serialize_field("identifier", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http2_protocol_options::SettingsParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifier",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
            Value,
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
                            "identifier" => Ok(GeneratedField::Identifier),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http2_protocol_options::SettingsParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Http2ProtocolOptions.SettingsParameter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http2_protocol_options::SettingsParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(http2_protocol_options::SettingsParameter {
                    identifier: identifier__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Http2ProtocolOptions.SettingsParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Http3ProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.quic_protocol_options.is_some() {
            len += 1;
        }
        if self.override_stream_error_on_invalid_http_message.is_some() {
            len += 1;
        }
        if self.allow_extended_connect {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Http3ProtocolOptions", len)?;
        if let Some(v) = self.quic_protocol_options.as_ref() {
            struct_ser.serialize_field("quicProtocolOptions", v)?;
        }
        if let Some(v) = self.override_stream_error_on_invalid_http_message.as_ref() {
            struct_ser.serialize_field("overrideStreamErrorOnInvalidHttpMessage", v)?;
        }
        if self.allow_extended_connect {
            struct_ser.serialize_field("allowExtendedConnect", &self.allow_extended_connect)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Http3ProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quic_protocol_options",
            "quicProtocolOptions",
            "override_stream_error_on_invalid_http_message",
            "overrideStreamErrorOnInvalidHttpMessage",
            "allow_extended_connect",
            "allowExtendedConnect",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QuicProtocolOptions,
            OverrideStreamErrorOnInvalidHttpMessage,
            AllowExtendedConnect,
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
                            "quicProtocolOptions" | "quic_protocol_options" => Ok(GeneratedField::QuicProtocolOptions),
                            "overrideStreamErrorOnInvalidHttpMessage" | "override_stream_error_on_invalid_http_message" => Ok(GeneratedField::OverrideStreamErrorOnInvalidHttpMessage),
                            "allowExtendedConnect" | "allow_extended_connect" => Ok(GeneratedField::AllowExtendedConnect),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Http3ProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Http3ProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Http3ProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quic_protocol_options__ = None;
                let mut override_stream_error_on_invalid_http_message__ = None;
                let mut allow_extended_connect__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::QuicProtocolOptions => {
                            if quic_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quicProtocolOptions"));
                            }
                            quic_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::OverrideStreamErrorOnInvalidHttpMessage => {
                            if override_stream_error_on_invalid_http_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideStreamErrorOnInvalidHttpMessage"));
                            }
                            override_stream_error_on_invalid_http_message__ = map.next_value()?;
                        }
                        GeneratedField::AllowExtendedConnect => {
                            if allow_extended_connect__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowExtendedConnect"));
                            }
                            allow_extended_connect__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Http3ProtocolOptions {
                    quic_protocol_options: quic_protocol_options__,
                    override_stream_error_on_invalid_http_message: override_stream_error_on_invalid_http_message__,
                    allow_extended_connect: allow_extended_connect__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Http3ProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.max_connection_duration.is_some() {
            len += 1;
        }
        if self.max_headers_count.is_some() {
            len += 1;
        }
        if self.max_stream_duration.is_some() {
            len += 1;
        }
        if self.headers_with_underscores_action != 0 {
            len += 1;
        }
        if self.max_requests_per_connection.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HttpProtocolOptions", len)?;
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        if let Some(v) = self.max_connection_duration.as_ref() {
            struct_ser.serialize_field("maxConnectionDuration", v)?;
        }
        if let Some(v) = self.max_headers_count.as_ref() {
            struct_ser.serialize_field("maxHeadersCount", v)?;
        }
        if let Some(v) = self.max_stream_duration.as_ref() {
            struct_ser.serialize_field("maxStreamDuration", v)?;
        }
        if self.headers_with_underscores_action != 0 {
            let v = http_protocol_options::HeadersWithUnderscoresAction::from_i32(self.headers_with_underscores_action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.headers_with_underscores_action)))?;
            struct_ser.serialize_field("headersWithUnderscoresAction", &v)?;
        }
        if let Some(v) = self.max_requests_per_connection.as_ref() {
            struct_ser.serialize_field("maxRequestsPerConnection", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "idle_timeout",
            "idleTimeout",
            "max_connection_duration",
            "maxConnectionDuration",
            "max_headers_count",
            "maxHeadersCount",
            "max_stream_duration",
            "maxStreamDuration",
            "headers_with_underscores_action",
            "headersWithUnderscoresAction",
            "max_requests_per_connection",
            "maxRequestsPerConnection",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IdleTimeout,
            MaxConnectionDuration,
            MaxHeadersCount,
            MaxStreamDuration,
            HeadersWithUnderscoresAction,
            MaxRequestsPerConnection,
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
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "maxConnectionDuration" | "max_connection_duration" => Ok(GeneratedField::MaxConnectionDuration),
                            "maxHeadersCount" | "max_headers_count" => Ok(GeneratedField::MaxHeadersCount),
                            "maxStreamDuration" | "max_stream_duration" => Ok(GeneratedField::MaxStreamDuration),
                            "headersWithUnderscoresAction" | "headers_with_underscores_action" => Ok(GeneratedField::HeadersWithUnderscoresAction),
                            "maxRequestsPerConnection" | "max_requests_per_connection" => Ok(GeneratedField::MaxRequestsPerConnection),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HttpProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut idle_timeout__ = None;
                let mut max_connection_duration__ = None;
                let mut max_headers_count__ = None;
                let mut max_stream_duration__ = None;
                let mut headers_with_underscores_action__ = None;
                let mut max_requests_per_connection__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::MaxConnectionDuration => {
                            if max_connection_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnectionDuration"));
                            }
                            max_connection_duration__ = map.next_value()?;
                        }
                        GeneratedField::MaxHeadersCount => {
                            if max_headers_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxHeadersCount"));
                            }
                            max_headers_count__ = map.next_value()?;
                        }
                        GeneratedField::MaxStreamDuration => {
                            if max_stream_duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxStreamDuration"));
                            }
                            max_stream_duration__ = map.next_value()?;
                        }
                        GeneratedField::HeadersWithUnderscoresAction => {
                            if headers_with_underscores_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersWithUnderscoresAction"));
                            }
                            headers_with_underscores_action__ = Some(map.next_value::<http_protocol_options::HeadersWithUnderscoresAction>()? as i32);
                        }
                        GeneratedField::MaxRequestsPerConnection => {
                            if max_requests_per_connection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestsPerConnection"));
                            }
                            max_requests_per_connection__ = map.next_value()?;
                        }
                    }
                }
                Ok(HttpProtocolOptions {
                    idle_timeout: idle_timeout__,
                    max_connection_duration: max_connection_duration__,
                    max_headers_count: max_headers_count__,
                    max_stream_duration: max_stream_duration__,
                    headers_with_underscores_action: headers_with_underscores_action__.unwrap_or_default(),
                    max_requests_per_connection: max_requests_per_connection__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HttpProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_protocol_options::HeadersWithUnderscoresAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Allow => "ALLOW",
            Self::RejectRequest => "REJECT_REQUEST",
            Self::DropHeader => "DROP_HEADER",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_protocol_options::HeadersWithUnderscoresAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALLOW",
            "REJECT_REQUEST",
            "DROP_HEADER",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_protocol_options::HeadersWithUnderscoresAction;

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
                    .and_then(http_protocol_options::HeadersWithUnderscoresAction::from_i32)
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
                    .and_then(http_protocol_options::HeadersWithUnderscoresAction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ALLOW" => Ok(http_protocol_options::HeadersWithUnderscoresAction::Allow),
                    "REJECT_REQUEST" => Ok(http_protocol_options::HeadersWithUnderscoresAction::RejectRequest),
                    "DROP_HEADER" => Ok(http_protocol_options::HeadersWithUnderscoresAction::DropHeader),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HttpUri {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.uri.is_empty() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.http_upstream_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.HttpUri", len)?;
        if !self.uri.is_empty() {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.http_upstream_type.as_ref() {
            match v {
                http_uri::HttpUpstreamType::Cluster(v) => {
                    struct_ser.serialize_field("cluster", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpUri {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
            "timeout",
            "cluster",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
            Timeout,
            Cluster,
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
                            "uri" => Ok(GeneratedField::Uri),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "cluster" => Ok(GeneratedField::Cluster),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpUri;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.HttpUri")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpUri, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut uri__ = None;
                let mut timeout__ = None;
                let mut http_upstream_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                        GeneratedField::Cluster => {
                            if http_upstream_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            http_upstream_type__ = map.next_value::<::std::option::Option<_>>()?.map(http_uri::HttpUpstreamType::Cluster);
                        }
                    }
                }
                Ok(HttpUri {
                    uri: uri__.unwrap_or_default(),
                    timeout: timeout__,
                    http_upstream_type: http_upstream_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.HttpUri", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeepaliveSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.interval.is_some() {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        if self.interval_jitter.is_some() {
            len += 1;
        }
        if self.connection_idle_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.KeepaliveSettings", len)?;
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        if let Some(v) = self.interval_jitter.as_ref() {
            struct_ser.serialize_field("intervalJitter", v)?;
        }
        if let Some(v) = self.connection_idle_interval.as_ref() {
            struct_ser.serialize_field("connectionIdleInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeepaliveSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "interval",
            "timeout",
            "interval_jitter",
            "intervalJitter",
            "connection_idle_interval",
            "connectionIdleInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Interval,
            Timeout,
            IntervalJitter,
            ConnectionIdleInterval,
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
                            "interval" => Ok(GeneratedField::Interval),
                            "timeout" => Ok(GeneratedField::Timeout),
                            "intervalJitter" | "interval_jitter" => Ok(GeneratedField::IntervalJitter),
                            "connectionIdleInterval" | "connection_idle_interval" => Ok(GeneratedField::ConnectionIdleInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KeepaliveSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.KeepaliveSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeepaliveSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut interval__ = None;
                let mut timeout__ = None;
                let mut interval_jitter__ = None;
                let mut connection_idle_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map.next_value()?;
                        }
                        GeneratedField::IntervalJitter => {
                            if interval_jitter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intervalJitter"));
                            }
                            interval_jitter__ = map.next_value()?;
                        }
                        GeneratedField::ConnectionIdleInterval => {
                            if connection_idle_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionIdleInterval"));
                            }
                            connection_idle_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(KeepaliveSettings {
                    interval: interval__,
                    timeout: timeout__,
                    interval_jitter: interval_jitter__,
                    connection_idle_interval: connection_idle_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.KeepaliveSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Locality {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.region.is_empty() {
            len += 1;
        }
        if !self.zone.is_empty() {
            len += 1;
        }
        if !self.sub_zone.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Locality", len)?;
        if !self.region.is_empty() {
            struct_ser.serialize_field("region", &self.region)?;
        }
        if !self.zone.is_empty() {
            struct_ser.serialize_field("zone", &self.zone)?;
        }
        if !self.sub_zone.is_empty() {
            struct_ser.serialize_field("subZone", &self.sub_zone)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Locality {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "region",
            "zone",
            "sub_zone",
            "subZone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Region,
            Zone,
            SubZone,
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
                            "region" => Ok(GeneratedField::Region),
                            "zone" => Ok(GeneratedField::Zone),
                            "subZone" | "sub_zone" => Ok(GeneratedField::SubZone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Locality;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Locality")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Locality, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut region__ = None;
                let mut zone__ = None;
                let mut sub_zone__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Region => {
                            if region__.is_some() {
                                return Err(serde::de::Error::duplicate_field("region"));
                            }
                            region__ = Some(map.next_value()?);
                        }
                        GeneratedField::Zone => {
                            if zone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zone"));
                            }
                            zone__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubZone => {
                            if sub_zone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subZone"));
                            }
                            sub_zone__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Locality {
                    region: region__.unwrap_or_default(),
                    zone: zone__.unwrap_or_default(),
                    sub_zone: sub_zone__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Locality", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filter_metadata.is_empty() {
            len += 1;
        }
        if !self.typed_filter_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Metadata", len)?;
        if !self.filter_metadata.is_empty() {
            struct_ser.serialize_field("filterMetadata", &self.filter_metadata)?;
        }
        if !self.typed_filter_metadata.is_empty() {
            struct_ser.serialize_field("typedFilterMetadata", &self.typed_filter_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter_metadata",
            "filterMetadata",
            "typed_filter_metadata",
            "typedFilterMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FilterMetadata,
            TypedFilterMetadata,
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
                            "filterMetadata" | "filter_metadata" => Ok(GeneratedField::FilterMetadata),
                            "typedFilterMetadata" | "typed_filter_metadata" => Ok(GeneratedField::TypedFilterMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Metadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Metadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter_metadata__ = None;
                let mut typed_filter_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FilterMetadata => {
                            if filter_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterMetadata"));
                            }
                            filter_metadata__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::TypedFilterMetadata => {
                            if typed_filter_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedFilterMetadata"));
                            }
                            typed_filter_metadata__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(Metadata {
                    filter_metadata: filter_metadata__.unwrap_or_default(),
                    typed_filter_metadata: typed_filter_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Metadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Node {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.cluster.is_empty() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.dynamic_parameters.is_empty() {
            len += 1;
        }
        if self.locality.is_some() {
            len += 1;
        }
        if !self.user_agent_name.is_empty() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        if !self.client_features.is_empty() {
            len += 1;
        }
        if !self.listening_addresses.is_empty() {
            len += 1;
        }
        if self.user_agent_version_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Node", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.dynamic_parameters.is_empty() {
            struct_ser.serialize_field("dynamicParameters", &self.dynamic_parameters)?;
        }
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if !self.user_agent_name.is_empty() {
            struct_ser.serialize_field("userAgentName", &self.user_agent_name)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        if !self.client_features.is_empty() {
            struct_ser.serialize_field("clientFeatures", &self.client_features)?;
        }
        if !self.listening_addresses.is_empty() {
            struct_ser.serialize_field("listeningAddresses", &self.listening_addresses)?;
        }
        if let Some(v) = self.user_agent_version_type.as_ref() {
            match v {
                node::UserAgentVersionType::UserAgentVersion(v) => {
                    struct_ser.serialize_field("userAgentVersion", v)?;
                }
                node::UserAgentVersionType::UserAgentBuildVersion(v) => {
                    struct_ser.serialize_field("userAgentBuildVersion", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Node {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "cluster",
            "metadata",
            "dynamic_parameters",
            "dynamicParameters",
            "locality",
            "user_agent_name",
            "userAgentName",
            "extensions",
            "client_features",
            "clientFeatures",
            "listening_addresses",
            "listeningAddresses",
            "user_agent_version",
            "userAgentVersion",
            "user_agent_build_version",
            "userAgentBuildVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Cluster,
            Metadata,
            DynamicParameters,
            Locality,
            UserAgentName,
            Extensions,
            ClientFeatures,
            ListeningAddresses,
            UserAgentVersion,
            UserAgentBuildVersion,
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
                            "id" => Ok(GeneratedField::Id),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "dynamicParameters" | "dynamic_parameters" => Ok(GeneratedField::DynamicParameters),
                            "locality" => Ok(GeneratedField::Locality),
                            "userAgentName" | "user_agent_name" => Ok(GeneratedField::UserAgentName),
                            "extensions" => Ok(GeneratedField::Extensions),
                            "clientFeatures" | "client_features" => Ok(GeneratedField::ClientFeatures),
                            "listeningAddresses" | "listening_addresses" => Ok(GeneratedField::ListeningAddresses),
                            "userAgentVersion" | "user_agent_version" => Ok(GeneratedField::UserAgentVersion),
                            "userAgentBuildVersion" | "user_agent_build_version" => Ok(GeneratedField::UserAgentBuildVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Node;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Node")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Node, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut cluster__ = None;
                let mut metadata__ = None;
                let mut dynamic_parameters__ = None;
                let mut locality__ = None;
                let mut user_agent_name__ = None;
                let mut extensions__ = None;
                let mut client_features__ = None;
                let mut listening_addresses__ = None;
                let mut user_agent_version_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::DynamicParameters => {
                            if dynamic_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicParameters"));
                            }
                            dynamic_parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map.next_value()?;
                        }
                        GeneratedField::UserAgentName => {
                            if user_agent_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgentName"));
                            }
                            user_agent_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClientFeatures => {
                            if client_features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientFeatures"));
                            }
                            client_features__ = Some(map.next_value()?);
                        }
                        GeneratedField::ListeningAddresses => {
                            if listening_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listeningAddresses"));
                            }
                            listening_addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserAgentVersion => {
                            if user_agent_version_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgentVersion"));
                            }
                            user_agent_version_type__ = map.next_value::<::std::option::Option<_>>()?.map(node::UserAgentVersionType::UserAgentVersion);
                        }
                        GeneratedField::UserAgentBuildVersion => {
                            if user_agent_version_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgentBuildVersion"));
                            }
                            user_agent_version_type__ = map.next_value::<::std::option::Option<_>>()?.map(node::UserAgentVersionType::UserAgentBuildVersion)
;
                        }
                    }
                }
                Ok(Node {
                    id: id__.unwrap_or_default(),
                    cluster: cluster__.unwrap_or_default(),
                    metadata: metadata__,
                    dynamic_parameters: dynamic_parameters__.unwrap_or_default(),
                    locality: locality__,
                    user_agent_name: user_agent_name__.unwrap_or_default(),
                    extensions: extensions__.unwrap_or_default(),
                    client_features: client_features__.unwrap_or_default(),
                    listening_addresses: listening_addresses__.unwrap_or_default(),
                    user_agent_version_type: user_agent_version_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Node", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PathConfigSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.watched_directory.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.PathConfigSource", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.watched_directory.as_ref() {
            struct_ser.serialize_field("watchedDirectory", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PathConfigSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "watched_directory",
            "watchedDirectory",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            WatchedDirectory,
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
                            "path" => Ok(GeneratedField::Path),
                            "watchedDirectory" | "watched_directory" => Ok(GeneratedField::WatchedDirectory),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PathConfigSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.PathConfigSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PathConfigSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut watched_directory__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::WatchedDirectory => {
                            if watched_directory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchedDirectory"));
                            }
                            watched_directory__ = map.next_value()?;
                        }
                    }
                }
                Ok(PathConfigSource {
                    path: path__.unwrap_or_default(),
                    watched_directory: watched_directory__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.PathConfigSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Pipe {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if self.mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.Pipe", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if self.mode != 0 {
            struct_ser.serialize_field("mode", &self.mode)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Pipe {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "mode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            Mode,
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
                            "path" => Ok(GeneratedField::Path),
                            "mode" => Ok(GeneratedField::Mode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Pipe;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.Pipe")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Pipe, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Pipe {
                    path: path__.unwrap_or_default(),
                    mode: mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.Pipe", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProxyProtocolConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version != 0 {
            len += 1;
        }
        if self.pass_through_tlvs.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.ProxyProtocolConfig", len)?;
        if self.version != 0 {
            let v = proxy_protocol_config::Version::from_i32(self.version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.version)))?;
            struct_ser.serialize_field("version", &v)?;
        }
        if let Some(v) = self.pass_through_tlvs.as_ref() {
            struct_ser.serialize_field("passThroughTlvs", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProxyProtocolConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "pass_through_tlvs",
            "passThroughTlvs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
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
                            "version" => Ok(GeneratedField::Version),
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
            type Value = ProxyProtocolConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.ProxyProtocolConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProxyProtocolConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut pass_through_tlvs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value::<proxy_protocol_config::Version>()? as i32);
                        }
                        GeneratedField::PassThroughTlvs => {
                            if pass_through_tlvs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passThroughTlvs"));
                            }
                            pass_through_tlvs__ = map.next_value()?;
                        }
                    }
                }
                Ok(ProxyProtocolConfig {
                    version: version__.unwrap_or_default(),
                    pass_through_tlvs: pass_through_tlvs__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.ProxyProtocolConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for proxy_protocol_config::Version {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::V1 => "V1",
            Self::V2 => "V2",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for proxy_protocol_config::Version {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "V1",
            "V2",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = proxy_protocol_config::Version;

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
                    .and_then(proxy_protocol_config::Version::from_i32)
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
                    .and_then(proxy_protocol_config::Version::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "V1" => Ok(proxy_protocol_config::Version::V1),
                    "V2" => Ok(proxy_protocol_config::Version::V2),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ProxyProtocolPassThroughTlVs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.match_type != 0 {
            len += 1;
        }
        if !self.tlv_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.ProxyProtocolPassThroughTLVs", len)?;
        if self.match_type != 0 {
            let v = proxy_protocol_pass_through_tl_vs::PassTlVsMatchType::from_i32(self.match_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.match_type)))?;
            struct_ser.serialize_field("matchType", &v)?;
        }
        if !self.tlv_type.is_empty() {
            struct_ser.serialize_field("tlvType", &self.tlv_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProxyProtocolPassThroughTlVs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "match_type",
            "matchType",
            "tlv_type",
            "tlvType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MatchType,
            TlvType,
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
                            "matchType" | "match_type" => Ok(GeneratedField::MatchType),
                            "tlvType" | "tlv_type" => Ok(GeneratedField::TlvType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProxyProtocolPassThroughTlVs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.ProxyProtocolPassThroughTLVs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProxyProtocolPassThroughTlVs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut match_type__ = None;
                let mut tlv_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MatchType => {
                            if match_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchType"));
                            }
                            match_type__ = Some(map.next_value::<proxy_protocol_pass_through_tl_vs::PassTlVsMatchType>()? as i32);
                        }
                        GeneratedField::TlvType => {
                            if tlv_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlvType"));
                            }
                            tlv_type__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(ProxyProtocolPassThroughTlVs {
                    match_type: match_type__.unwrap_or_default(),
                    tlv_type: tlv_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.ProxyProtocolPassThroughTLVs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for proxy_protocol_pass_through_tl_vs::PassTlVsMatchType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::IncludeAll => "INCLUDE_ALL",
            Self::Include => "INCLUDE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for proxy_protocol_pass_through_tl_vs::PassTlVsMatchType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INCLUDE_ALL",
            "INCLUDE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = proxy_protocol_pass_through_tl_vs::PassTlVsMatchType;

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
                    .and_then(proxy_protocol_pass_through_tl_vs::PassTlVsMatchType::from_i32)
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
                    .and_then(proxy_protocol_pass_through_tl_vs::PassTlVsMatchType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "INCLUDE_ALL" => Ok(proxy_protocol_pass_through_tl_vs::PassTlVsMatchType::IncludeAll),
                    "INCLUDE" => Ok(proxy_protocol_pass_through_tl_vs::PassTlVsMatchType::Include),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.QueryParameter", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.QueryParameter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                    }
                }
                Ok(QueryParameter {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.QueryParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuicKeepAliveSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_interval.is_some() {
            len += 1;
        }
        if self.initial_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.QuicKeepAliveSettings", len)?;
        if let Some(v) = self.max_interval.as_ref() {
            struct_ser.serialize_field("maxInterval", v)?;
        }
        if let Some(v) = self.initial_interval.as_ref() {
            struct_ser.serialize_field("initialInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuicKeepAliveSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_interval",
            "maxInterval",
            "initial_interval",
            "initialInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxInterval,
            InitialInterval,
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
                            "maxInterval" | "max_interval" => Ok(GeneratedField::MaxInterval),
                            "initialInterval" | "initial_interval" => Ok(GeneratedField::InitialInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuicKeepAliveSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.QuicKeepAliveSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuicKeepAliveSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_interval__ = None;
                let mut initial_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxInterval => {
                            if max_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInterval"));
                            }
                            max_interval__ = map.next_value()?;
                        }
                        GeneratedField::InitialInterval => {
                            if initial_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialInterval"));
                            }
                            initial_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(QuicKeepAliveSettings {
                    max_interval: max_interval__,
                    initial_interval: initial_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.QuicKeepAliveSettings", FIELDS, GeneratedVisitor)
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
        if self.initial_stream_window_size.is_some() {
            len += 1;
        }
        if self.initial_connection_window_size.is_some() {
            len += 1;
        }
        if self.num_timeouts_to_trigger_port_migration.is_some() {
            len += 1;
        }
        if self.connection_keepalive.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.QuicProtocolOptions", len)?;
        if let Some(v) = self.max_concurrent_streams.as_ref() {
            struct_ser.serialize_field("maxConcurrentStreams", v)?;
        }
        if let Some(v) = self.initial_stream_window_size.as_ref() {
            struct_ser.serialize_field("initialStreamWindowSize", v)?;
        }
        if let Some(v) = self.initial_connection_window_size.as_ref() {
            struct_ser.serialize_field("initialConnectionWindowSize", v)?;
        }
        if let Some(v) = self.num_timeouts_to_trigger_port_migration.as_ref() {
            struct_ser.serialize_field("numTimeoutsToTriggerPortMigration", v)?;
        }
        if let Some(v) = self.connection_keepalive.as_ref() {
            struct_ser.serialize_field("connectionKeepalive", v)?;
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
            "initial_stream_window_size",
            "initialStreamWindowSize",
            "initial_connection_window_size",
            "initialConnectionWindowSize",
            "num_timeouts_to_trigger_port_migration",
            "numTimeoutsToTriggerPortMigration",
            "connection_keepalive",
            "connectionKeepalive",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxConcurrentStreams,
            InitialStreamWindowSize,
            InitialConnectionWindowSize,
            NumTimeoutsToTriggerPortMigration,
            ConnectionKeepalive,
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
                            "initialStreamWindowSize" | "initial_stream_window_size" => Ok(GeneratedField::InitialStreamWindowSize),
                            "initialConnectionWindowSize" | "initial_connection_window_size" => Ok(GeneratedField::InitialConnectionWindowSize),
                            "numTimeoutsToTriggerPortMigration" | "num_timeouts_to_trigger_port_migration" => Ok(GeneratedField::NumTimeoutsToTriggerPortMigration),
                            "connectionKeepalive" | "connection_keepalive" => Ok(GeneratedField::ConnectionKeepalive),
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
                formatter.write_str("struct envoy.config.core.v3.QuicProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuicProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_concurrent_streams__ = None;
                let mut initial_stream_window_size__ = None;
                let mut initial_connection_window_size__ = None;
                let mut num_timeouts_to_trigger_port_migration__ = None;
                let mut connection_keepalive__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxConcurrentStreams => {
                            if max_concurrent_streams__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConcurrentStreams"));
                            }
                            max_concurrent_streams__ = map.next_value()?;
                        }
                        GeneratedField::InitialStreamWindowSize => {
                            if initial_stream_window_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialStreamWindowSize"));
                            }
                            initial_stream_window_size__ = map.next_value()?;
                        }
                        GeneratedField::InitialConnectionWindowSize => {
                            if initial_connection_window_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialConnectionWindowSize"));
                            }
                            initial_connection_window_size__ = map.next_value()?;
                        }
                        GeneratedField::NumTimeoutsToTriggerPortMigration => {
                            if num_timeouts_to_trigger_port_migration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numTimeoutsToTriggerPortMigration"));
                            }
                            num_timeouts_to_trigger_port_migration__ = map.next_value()?;
                        }
                        GeneratedField::ConnectionKeepalive => {
                            if connection_keepalive__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionKeepalive"));
                            }
                            connection_keepalive__ = map.next_value()?;
                        }
                    }
                }
                Ok(QuicProtocolOptions {
                    max_concurrent_streams: max_concurrent_streams__,
                    initial_stream_window_size: initial_stream_window_size__,
                    initial_connection_window_size: initial_connection_window_size__,
                    num_timeouts_to_trigger_port_migration: num_timeouts_to_trigger_port_migration__,
                    connection_keepalive: connection_keepalive__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.QuicProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimitSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_tokens.is_some() {
            len += 1;
        }
        if self.fill_rate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RateLimitSettings", len)?;
        if let Some(v) = self.max_tokens.as_ref() {
            struct_ser.serialize_field("maxTokens", v)?;
        }
        if let Some(v) = self.fill_rate.as_ref() {
            struct_ser.serialize_field("fillRate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_tokens",
            "maxTokens",
            "fill_rate",
            "fillRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxTokens,
            FillRate,
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
                            "maxTokens" | "max_tokens" => Ok(GeneratedField::MaxTokens),
                            "fillRate" | "fill_rate" => Ok(GeneratedField::FillRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimitSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RateLimitSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimitSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_tokens__ = None;
                let mut fill_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxTokens => {
                            if max_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTokens"));
                            }
                            max_tokens__ = map.next_value()?;
                        }
                        GeneratedField::FillRate => {
                            if fill_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fillRate"));
                            }
                            fill_rate__ = map.next_value()?;
                        }
                    }
                }
                Ok(RateLimitSettings {
                    max_tokens: max_tokens__,
                    fill_rate: fill_rate__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RateLimitSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoteDataSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http_uri.is_some() {
            len += 1;
        }
        if !self.sha256.is_empty() {
            len += 1;
        }
        if self.retry_policy.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RemoteDataSource", len)?;
        if let Some(v) = self.http_uri.as_ref() {
            struct_ser.serialize_field("httpUri", v)?;
        }
        if !self.sha256.is_empty() {
            struct_ser.serialize_field("sha256", &self.sha256)?;
        }
        if let Some(v) = self.retry_policy.as_ref() {
            struct_ser.serialize_field("retryPolicy", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoteDataSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http_uri",
            "httpUri",
            "sha256",
            "retry_policy",
            "retryPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HttpUri,
            Sha256,
            RetryPolicy,
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
                            "httpUri" | "http_uri" => Ok(GeneratedField::HttpUri),
                            "sha256" => Ok(GeneratedField::Sha256),
                            "retryPolicy" | "retry_policy" => Ok(GeneratedField::RetryPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoteDataSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RemoteDataSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RemoteDataSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http_uri__ = None;
                let mut sha256__ = None;
                let mut retry_policy__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HttpUri => {
                            if http_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpUri"));
                            }
                            http_uri__ = map.next_value()?;
                        }
                        GeneratedField::Sha256 => {
                            if sha256__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sha256"));
                            }
                            sha256__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetryPolicy => {
                            if retry_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryPolicy"));
                            }
                            retry_policy__ = map.next_value()?;
                        }
                    }
                }
                Ok(RemoteDataSource {
                    http_uri: http_uri__,
                    sha256: sha256__.unwrap_or_default(),
                    retry_policy: retry_policy__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RemoteDataSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestMethod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::MethodUnspecified => "METHOD_UNSPECIFIED",
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RequestMethod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "METHOD_UNSPECIFIED",
            "GET",
            "HEAD",
            "POST",
            "PUT",
            "DELETE",
            "CONNECT",
            "OPTIONS",
            "TRACE",
            "PATCH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestMethod;

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
                    .and_then(RequestMethod::from_i32)
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
                    .and_then(RequestMethod::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "METHOD_UNSPECIFIED" => Ok(RequestMethod::MethodUnspecified),
                    "GET" => Ok(RequestMethod::Get),
                    "HEAD" => Ok(RequestMethod::Head),
                    "POST" => Ok(RequestMethod::Post),
                    "PUT" => Ok(RequestMethod::Put),
                    "DELETE" => Ok(RequestMethod::Delete),
                    "CONNECT" => Ok(RequestMethod::Connect),
                    "OPTIONS" => Ok(RequestMethod::Options),
                    "TRACE" => Ok(RequestMethod::Trace),
                    "PATCH" => Ok(RequestMethod::Patch),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RetryPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.retry_back_off.is_some() {
            len += 1;
        }
        if self.num_retries.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RetryPolicy", len)?;
        if let Some(v) = self.retry_back_off.as_ref() {
            struct_ser.serialize_field("retryBackOff", v)?;
        }
        if let Some(v) = self.num_retries.as_ref() {
            struct_ser.serialize_field("numRetries", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RetryPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "retry_back_off",
            "retryBackOff",
            "num_retries",
            "numRetries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RetryBackOff,
            NumRetries,
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
                            "retryBackOff" | "retry_back_off" => Ok(GeneratedField::RetryBackOff),
                            "numRetries" | "num_retries" => Ok(GeneratedField::NumRetries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RetryPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RetryPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RetryPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut retry_back_off__ = None;
                let mut num_retries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RetryBackOff => {
                            if retry_back_off__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryBackOff"));
                            }
                            retry_back_off__ = map.next_value()?;
                        }
                        GeneratedField::NumRetries => {
                            if num_retries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numRetries"));
                            }
                            num_retries__ = map.next_value()?;
                        }
                    }
                }
                Ok(RetryPolicy {
                    retry_back_off: retry_back_off__,
                    num_retries: num_retries__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RetryPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutingPriority {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::High => "HIGH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RoutingPriority {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "HIGH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutingPriority;

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
                    .and_then(RoutingPriority::from_i32)
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
                    .and_then(RoutingPriority::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(RoutingPriority::Default),
                    "HIGH" => Ok(RoutingPriority::High),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimeDouble {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.default_value != 0. {
            len += 1;
        }
        if !self.runtime_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RuntimeDouble", len)?;
        if self.default_value != 0. {
            struct_ser.serialize_field("defaultValue", &self.default_value)?;
        }
        if !self.runtime_key.is_empty() {
            struct_ser.serialize_field("runtimeKey", &self.runtime_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimeDouble {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_value",
            "defaultValue",
            "runtime_key",
            "runtimeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultValue,
            RuntimeKey,
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
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "runtimeKey" | "runtime_key" => Ok(GeneratedField::RuntimeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimeDouble;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RuntimeDouble")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuntimeDouble, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_value__ = None;
                let mut runtime_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RuntimeKey => {
                            if runtime_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKey"));
                            }
                            runtime_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RuntimeDouble {
                    default_value: default_value__.unwrap_or_default(),
                    runtime_key: runtime_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RuntimeDouble", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimeFeatureFlag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.default_value.is_some() {
            len += 1;
        }
        if !self.runtime_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RuntimeFeatureFlag", len)?;
        if let Some(v) = self.default_value.as_ref() {
            struct_ser.serialize_field("defaultValue", v)?;
        }
        if !self.runtime_key.is_empty() {
            struct_ser.serialize_field("runtimeKey", &self.runtime_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimeFeatureFlag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_value",
            "defaultValue",
            "runtime_key",
            "runtimeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultValue,
            RuntimeKey,
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
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "runtimeKey" | "runtime_key" => Ok(GeneratedField::RuntimeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimeFeatureFlag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RuntimeFeatureFlag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuntimeFeatureFlag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_value__ = None;
                let mut runtime_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = map.next_value()?;
                        }
                        GeneratedField::RuntimeKey => {
                            if runtime_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKey"));
                            }
                            runtime_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RuntimeFeatureFlag {
                    default_value: default_value__,
                    runtime_key: runtime_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RuntimeFeatureFlag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimeFractionalPercent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.default_value.is_some() {
            len += 1;
        }
        if !self.runtime_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RuntimeFractionalPercent", len)?;
        if let Some(v) = self.default_value.as_ref() {
            struct_ser.serialize_field("defaultValue", v)?;
        }
        if !self.runtime_key.is_empty() {
            struct_ser.serialize_field("runtimeKey", &self.runtime_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimeFractionalPercent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_value",
            "defaultValue",
            "runtime_key",
            "runtimeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultValue,
            RuntimeKey,
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
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "runtimeKey" | "runtime_key" => Ok(GeneratedField::RuntimeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimeFractionalPercent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RuntimeFractionalPercent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuntimeFractionalPercent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_value__ = None;
                let mut runtime_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = map.next_value()?;
                        }
                        GeneratedField::RuntimeKey => {
                            if runtime_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKey"));
                            }
                            runtime_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RuntimeFractionalPercent {
                    default_value: default_value__,
                    runtime_key: runtime_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RuntimeFractionalPercent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimePercent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.default_value.is_some() {
            len += 1;
        }
        if !self.runtime_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RuntimePercent", len)?;
        if let Some(v) = self.default_value.as_ref() {
            struct_ser.serialize_field("defaultValue", v)?;
        }
        if !self.runtime_key.is_empty() {
            struct_ser.serialize_field("runtimeKey", &self.runtime_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimePercent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_value",
            "defaultValue",
            "runtime_key",
            "runtimeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultValue,
            RuntimeKey,
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
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "runtimeKey" | "runtime_key" => Ok(GeneratedField::RuntimeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimePercent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RuntimePercent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuntimePercent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_value__ = None;
                let mut runtime_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = map.next_value()?;
                        }
                        GeneratedField::RuntimeKey => {
                            if runtime_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKey"));
                            }
                            runtime_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RuntimePercent {
                    default_value: default_value__,
                    runtime_key: runtime_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RuntimePercent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RuntimeUInt32 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.default_value != 0 {
            len += 1;
        }
        if !self.runtime_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.RuntimeUInt32", len)?;
        if self.default_value != 0 {
            struct_ser.serialize_field("defaultValue", &self.default_value)?;
        }
        if !self.runtime_key.is_empty() {
            struct_ser.serialize_field("runtimeKey", &self.runtime_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RuntimeUInt32 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_value",
            "defaultValue",
            "runtime_key",
            "runtimeKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultValue,
            RuntimeKey,
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
                            "defaultValue" | "default_value" => Ok(GeneratedField::DefaultValue),
                            "runtimeKey" | "runtime_key" => Ok(GeneratedField::RuntimeKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RuntimeUInt32;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.RuntimeUInt32")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RuntimeUInt32, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_value__ = None;
                let mut runtime_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefaultValue => {
                            if default_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValue"));
                            }
                            default_value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RuntimeKey => {
                            if runtime_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeKey"));
                            }
                            runtime_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RuntimeUInt32 {
                    default_value: default_value__.unwrap_or_default(),
                    runtime_key: runtime_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.RuntimeUInt32", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SchemeHeaderTransformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transformation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.SchemeHeaderTransformation", len)?;
        if let Some(v) = self.transformation.as_ref() {
            match v {
                scheme_header_transformation::Transformation::SchemeToOverwrite(v) => {
                    struct_ser.serialize_field("schemeToOverwrite", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SchemeHeaderTransformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scheme_to_overwrite",
            "schemeToOverwrite",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SchemeToOverwrite,
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
                            "schemeToOverwrite" | "scheme_to_overwrite" => Ok(GeneratedField::SchemeToOverwrite),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SchemeHeaderTransformation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.SchemeHeaderTransformation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SchemeHeaderTransformation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transformation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SchemeToOverwrite => {
                            if transformation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schemeToOverwrite"));
                            }
                            transformation__ = map.next_value::<::std::option::Option<_>>()?.map(scheme_header_transformation::Transformation::SchemeToOverwrite);
                        }
                    }
                }
                Ok(SchemeHeaderTransformation {
                    transformation: transformation__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.SchemeHeaderTransformation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SelfConfigSource {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transport_api_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.SelfConfigSource", len)?;
        if self.transport_api_version != 0 {
            let v = ApiVersion::from_i32(self.transport_api_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.transport_api_version)))?;
            struct_ser.serialize_field("transportApiVersion", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SelfConfigSource {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transport_api_version",
            "transportApiVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransportApiVersion,
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
                            "transportApiVersion" | "transport_api_version" => Ok(GeneratedField::TransportApiVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SelfConfigSource;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.SelfConfigSource")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SelfConfigSource, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transport_api_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TransportApiVersion => {
                            if transport_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportApiVersion"));
                            }
                            transport_api_version__ = Some(map.next_value::<ApiVersion>()? as i32);
                        }
                    }
                }
                Ok(SelfConfigSource {
                    transport_api_version: transport_api_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.SelfConfigSource", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SocketAddress {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.resolver_name.is_empty() {
            len += 1;
        }
        if self.ipv4_compat {
            len += 1;
        }
        if self.port_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.SocketAddress", len)?;
        if self.protocol != 0 {
            let v = socket_address::Protocol::from_i32(self.protocol)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.resolver_name.is_empty() {
            struct_ser.serialize_field("resolverName", &self.resolver_name)?;
        }
        if self.ipv4_compat {
            struct_ser.serialize_field("ipv4Compat", &self.ipv4_compat)?;
        }
        if let Some(v) = self.port_specifier.as_ref() {
            match v {
                socket_address::PortSpecifier::PortValue(v) => {
                    struct_ser.serialize_field("portValue", v)?;
                }
                socket_address::PortSpecifier::NamedPort(v) => {
                    struct_ser.serialize_field("namedPort", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SocketAddress {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol",
            "address",
            "resolver_name",
            "resolverName",
            "ipv4_compat",
            "ipv4Compat",
            "port_value",
            "portValue",
            "named_port",
            "namedPort",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Protocol,
            Address,
            ResolverName,
            Ipv4Compat,
            PortValue,
            NamedPort,
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
                            "protocol" => Ok(GeneratedField::Protocol),
                            "address" => Ok(GeneratedField::Address),
                            "resolverName" | "resolver_name" => Ok(GeneratedField::ResolverName),
                            "ipv4Compat" | "ipv4_compat" => Ok(GeneratedField::Ipv4Compat),
                            "portValue" | "port_value" => Ok(GeneratedField::PortValue),
                            "namedPort" | "named_port" => Ok(GeneratedField::NamedPort),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SocketAddress;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.SocketAddress")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SocketAddress, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol__ = None;
                let mut address__ = None;
                let mut resolver_name__ = None;
                let mut ipv4_compat__ = None;
                let mut port_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map.next_value::<socket_address::Protocol>()? as i32);
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResolverName => {
                            if resolver_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resolverName"));
                            }
                            resolver_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ipv4Compat => {
                            if ipv4_compat__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4Compat"));
                            }
                            ipv4_compat__ = Some(map.next_value()?);
                        }
                        GeneratedField::PortValue => {
                            if port_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portValue"));
                            }
                            port_specifier__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| socket_address::PortSpecifier::PortValue(x.0));
                        }
                        GeneratedField::NamedPort => {
                            if port_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("namedPort"));
                            }
                            port_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(socket_address::PortSpecifier::NamedPort);
                        }
                    }
                }
                Ok(SocketAddress {
                    protocol: protocol__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    resolver_name: resolver_name__.unwrap_or_default(),
                    ipv4_compat: ipv4_compat__.unwrap_or_default(),
                    port_specifier: port_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.SocketAddress", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for socket_address::Protocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Tcp => "TCP",
            Self::Udp => "UDP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for socket_address::Protocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TCP",
            "UDP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = socket_address::Protocol;

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
                    .and_then(socket_address::Protocol::from_i32)
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
                    .and_then(socket_address::Protocol::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TCP" => Ok(socket_address::Protocol::Tcp),
                    "UDP" => Ok(socket_address::Protocol::Udp),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SocketOption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if self.level != 0 {
            len += 1;
        }
        if self.name != 0 {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.SocketOption", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.level != 0 {
            struct_ser.serialize_field("level", ToString::to_string(&self.level).as_str())?;
        }
        if self.name != 0 {
            struct_ser.serialize_field("name", ToString::to_string(&self.name).as_str())?;
        }
        if self.state != 0 {
            let v = socket_option::SocketState::from_i32(self.state)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.value.as_ref() {
            match v {
                socket_option::Value::IntValue(v) => {
                    struct_ser.serialize_field("intValue", ToString::to_string(&v).as_str())?;
                }
                socket_option::Value::BufValue(v) => {
                    struct_ser.serialize_field("bufValue", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SocketOption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "level",
            "name",
            "state",
            "int_value",
            "intValue",
            "buf_value",
            "bufValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Level,
            Name,
            State,
            IntValue,
            BufValue,
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
                            "description" => Ok(GeneratedField::Description),
                            "level" => Ok(GeneratedField::Level),
                            "name" => Ok(GeneratedField::Name),
                            "state" => Ok(GeneratedField::State),
                            "intValue" | "int_value" => Ok(GeneratedField::IntValue),
                            "bufValue" | "buf_value" => Ok(GeneratedField::BufValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SocketOption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.SocketOption")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SocketOption, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut level__ = None;
                let mut name__ = None;
                let mut state__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::Level => {
                            if level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("level"));
                            }
                            level__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map.next_value::<socket_option::SocketState>()? as i32);
                        }
                        GeneratedField::IntValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| socket_option::Value::IntValue(x.0));
                        }
                        GeneratedField::BufValue => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bufValue"));
                            }
                            value__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| socket_option::Value::BufValue(x.0));
                        }
                    }
                }
                Ok(SocketOption {
                    description: description__.unwrap_or_default(),
                    level: level__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.SocketOption", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for socket_option::SocketState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::StatePrebind => "STATE_PREBIND",
            Self::StateBound => "STATE_BOUND",
            Self::StateListening => "STATE_LISTENING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for socket_option::SocketState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_PREBIND",
            "STATE_BOUND",
            "STATE_LISTENING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = socket_option::SocketState;

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
                    .and_then(socket_option::SocketState::from_i32)
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
                    .and_then(socket_option::SocketState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATE_PREBIND" => Ok(socket_option::SocketState::StatePrebind),
                    "STATE_BOUND" => Ok(socket_option::SocketState::StateBound),
                    "STATE_LISTENING" => Ok(socket_option::SocketState::StateListening),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SocketOptionsOverride {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.socket_options.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.SocketOptionsOverride", len)?;
        if !self.socket_options.is_empty() {
            struct_ser.serialize_field("socketOptions", &self.socket_options)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SocketOptionsOverride {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "socket_options",
            "socketOptions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SocketOptions,
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
                            "socketOptions" | "socket_options" => Ok(GeneratedField::SocketOptions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SocketOptionsOverride;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.SocketOptionsOverride")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SocketOptionsOverride, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut socket_options__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SocketOptions => {
                            if socket_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketOptions"));
                            }
                            socket_options__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SocketOptionsOverride {
                    socket_options: socket_options__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.SocketOptionsOverride", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubstitutionFormatString {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.omit_empty_values {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        if !self.formatters.is_empty() {
            len += 1;
        }
        if self.format.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.SubstitutionFormatString", len)?;
        if self.omit_empty_values {
            struct_ser.serialize_field("omitEmptyValues", &self.omit_empty_values)?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if !self.formatters.is_empty() {
            struct_ser.serialize_field("formatters", &self.formatters)?;
        }
        if let Some(v) = self.format.as_ref() {
            match v {
                substitution_format_string::Format::TextFormat(v) => {
                    struct_ser.serialize_field("textFormat", v)?;
                }
                substitution_format_string::Format::JsonFormat(v) => {
                    struct_ser.serialize_field("jsonFormat", v)?;
                }
                substitution_format_string::Format::TextFormatSource(v) => {
                    struct_ser.serialize_field("textFormatSource", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubstitutionFormatString {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "omit_empty_values",
            "omitEmptyValues",
            "content_type",
            "contentType",
            "formatters",
            "text_format",
            "textFormat",
            "json_format",
            "jsonFormat",
            "text_format_source",
            "textFormatSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OmitEmptyValues,
            ContentType,
            Formatters,
            TextFormat,
            JsonFormat,
            TextFormatSource,
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
                            "omitEmptyValues" | "omit_empty_values" => Ok(GeneratedField::OmitEmptyValues),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "formatters" => Ok(GeneratedField::Formatters),
                            "textFormat" | "text_format" => Ok(GeneratedField::TextFormat),
                            "jsonFormat" | "json_format" => Ok(GeneratedField::JsonFormat),
                            "textFormatSource" | "text_format_source" => Ok(GeneratedField::TextFormatSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubstitutionFormatString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.SubstitutionFormatString")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubstitutionFormatString, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut omit_empty_values__ = None;
                let mut content_type__ = None;
                let mut formatters__ = None;
                let mut format__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OmitEmptyValues => {
                            if omit_empty_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("omitEmptyValues"));
                            }
                            omit_empty_values__ = Some(map.next_value()?);
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Formatters => {
                            if formatters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("formatters"));
                            }
                            formatters__ = Some(map.next_value()?);
                        }
                        GeneratedField::TextFormat => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("textFormat"));
                            }
                            format__ = map.next_value::<::std::option::Option<_>>()?.map(substitution_format_string::Format::TextFormat);
                        }
                        GeneratedField::JsonFormat => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jsonFormat"));
                            }
                            format__ = map.next_value::<::std::option::Option<_>>()?.map(substitution_format_string::Format::JsonFormat)
;
                        }
                        GeneratedField::TextFormatSource => {
                            if format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("textFormatSource"));
                            }
                            format__ = map.next_value::<::std::option::Option<_>>()?.map(substitution_format_string::Format::TextFormatSource)
;
                        }
                    }
                }
                Ok(SubstitutionFormatString {
                    omit_empty_values: omit_empty_values__.unwrap_or_default(),
                    content_type: content_type__.unwrap_or_default(),
                    formatters: formatters__.unwrap_or_default(),
                    format: format__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.SubstitutionFormatString", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TcpKeepalive {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.keepalive_probes.is_some() {
            len += 1;
        }
        if self.keepalive_time.is_some() {
            len += 1;
        }
        if self.keepalive_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.TcpKeepalive", len)?;
        if let Some(v) = self.keepalive_probes.as_ref() {
            struct_ser.serialize_field("keepaliveProbes", v)?;
        }
        if let Some(v) = self.keepalive_time.as_ref() {
            struct_ser.serialize_field("keepaliveTime", v)?;
        }
        if let Some(v) = self.keepalive_interval.as_ref() {
            struct_ser.serialize_field("keepaliveInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TcpKeepalive {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keepalive_probes",
            "keepaliveProbes",
            "keepalive_time",
            "keepaliveTime",
            "keepalive_interval",
            "keepaliveInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeepaliveProbes,
            KeepaliveTime,
            KeepaliveInterval,
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
                            "keepaliveProbes" | "keepalive_probes" => Ok(GeneratedField::KeepaliveProbes),
                            "keepaliveTime" | "keepalive_time" => Ok(GeneratedField::KeepaliveTime),
                            "keepaliveInterval" | "keepalive_interval" => Ok(GeneratedField::KeepaliveInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TcpKeepalive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.TcpKeepalive")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TcpKeepalive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keepalive_probes__ = None;
                let mut keepalive_time__ = None;
                let mut keepalive_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KeepaliveProbes => {
                            if keepalive_probes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keepaliveProbes"));
                            }
                            keepalive_probes__ = map.next_value()?;
                        }
                        GeneratedField::KeepaliveTime => {
                            if keepalive_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keepaliveTime"));
                            }
                            keepalive_time__ = map.next_value()?;
                        }
                        GeneratedField::KeepaliveInterval => {
                            if keepalive_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keepaliveInterval"));
                            }
                            keepalive_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(TcpKeepalive {
                    keepalive_probes: keepalive_probes__,
                    keepalive_time: keepalive_time__,
                    keepalive_interval: keepalive_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.TcpKeepalive", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TcpProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.config.core.v3.TcpProtocolOptions", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TcpProtocolOptions {
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
            type Value = TcpProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.TcpProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TcpProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(TcpProtocolOptions {
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.TcpProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TrafficDirection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Inbound => "INBOUND",
            Self::Outbound => "OUTBOUND",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TrafficDirection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "INBOUND",
            "OUTBOUND",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrafficDirection;

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
                    .and_then(TrafficDirection::from_i32)
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
                    .and_then(TrafficDirection::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNSPECIFIED" => Ok(TrafficDirection::Unspecified),
                    "INBOUND" => Ok(TrafficDirection::Inbound),
                    "OUTBOUND" => Ok(TrafficDirection::Outbound),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TransportSocket {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.TransportSocket", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                transport_socket::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransportSocket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
            type Value = TransportSocket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.TransportSocket")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TransportSocket, V::Error>
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
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(transport_socket::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(TransportSocket {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.TransportSocket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TypedExtensionConfig {
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
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.TypedExtensionConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typedConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TypedExtensionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
            type Value = TypedExtensionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.TypedExtensionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TypedExtensionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut typed_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(TypedExtensionConfig {
                    name: name__.unwrap_or_default(),
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.TypedExtensionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UdpSocketConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_rx_datagram_size.is_some() {
            len += 1;
        }
        if self.prefer_gro.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.UdpSocketConfig", len)?;
        if let Some(v) = self.max_rx_datagram_size.as_ref() {
            struct_ser.serialize_field("maxRxDatagramSize", v)?;
        }
        if let Some(v) = self.prefer_gro.as_ref() {
            struct_ser.serialize_field("preferGro", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UdpSocketConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_rx_datagram_size",
            "maxRxDatagramSize",
            "prefer_gro",
            "preferGro",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxRxDatagramSize,
            PreferGro,
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
                            "maxRxDatagramSize" | "max_rx_datagram_size" => Ok(GeneratedField::MaxRxDatagramSize),
                            "preferGro" | "prefer_gro" => Ok(GeneratedField::PreferGro),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UdpSocketConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.UdpSocketConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UdpSocketConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_rx_datagram_size__ = None;
                let mut prefer_gro__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxRxDatagramSize => {
                            if max_rx_datagram_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRxDatagramSize"));
                            }
                            max_rx_datagram_size__ = map.next_value()?;
                        }
                        GeneratedField::PreferGro => {
                            if prefer_gro__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferGro"));
                            }
                            prefer_gro__ = map.next_value()?;
                        }
                    }
                }
                Ok(UdpSocketConfig {
                    max_rx_datagram_size: max_rx_datagram_size__,
                    prefer_gro: prefer_gro__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.UdpSocketConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamHttpProtocolOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.auto_sni {
            len += 1;
        }
        if self.auto_san_validation {
            len += 1;
        }
        if !self.override_auto_sni_header.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.UpstreamHttpProtocolOptions", len)?;
        if self.auto_sni {
            struct_ser.serialize_field("autoSni", &self.auto_sni)?;
        }
        if self.auto_san_validation {
            struct_ser.serialize_field("autoSanValidation", &self.auto_san_validation)?;
        }
        if !self.override_auto_sni_header.is_empty() {
            struct_ser.serialize_field("overrideAutoSniHeader", &self.override_auto_sni_header)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamHttpProtocolOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auto_sni",
            "autoSni",
            "auto_san_validation",
            "autoSanValidation",
            "override_auto_sni_header",
            "overrideAutoSniHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AutoSni,
            AutoSanValidation,
            OverrideAutoSniHeader,
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
                            "autoSni" | "auto_sni" => Ok(GeneratedField::AutoSni),
                            "autoSanValidation" | "auto_san_validation" => Ok(GeneratedField::AutoSanValidation),
                            "overrideAutoSniHeader" | "override_auto_sni_header" => Ok(GeneratedField::OverrideAutoSniHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamHttpProtocolOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.UpstreamHttpProtocolOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamHttpProtocolOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auto_sni__ = None;
                let mut auto_san_validation__ = None;
                let mut override_auto_sni_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AutoSni => {
                            if auto_sni__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoSni"));
                            }
                            auto_sni__ = Some(map.next_value()?);
                        }
                        GeneratedField::AutoSanValidation => {
                            if auto_san_validation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoSanValidation"));
                            }
                            auto_san_validation__ = Some(map.next_value()?);
                        }
                        GeneratedField::OverrideAutoSniHeader => {
                            if override_auto_sni_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrideAutoSniHeader"));
                            }
                            override_auto_sni_header__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpstreamHttpProtocolOptions {
                    auto_sni: auto_sni__.unwrap_or_default(),
                    auto_san_validation: auto_san_validation__.unwrap_or_default(),
                    override_auto_sni_header: override_auto_sni_header__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.UpstreamHttpProtocolOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for WatchedDirectory {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.core.v3.WatchedDirectory", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WatchedDirectory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
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
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WatchedDirectory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.core.v3.WatchedDirectory")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<WatchedDirectory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(WatchedDirectory {
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.core.v3.WatchedDirectory", FIELDS, GeneratedVisitor)
    }
}
