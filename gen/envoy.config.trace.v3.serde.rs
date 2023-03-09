// @generated
impl serde::Serialize for ClientConfig {
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
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if self.max_cache_size.is_some() {
            len += 1;
        }
        if self.backend_token_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.ClientConfig", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instanceName", &self.instance_name)?;
        }
        if let Some(v) = self.max_cache_size.as_ref() {
            struct_ser.serialize_field("maxCacheSize", v)?;
        }
        if let Some(v) = self.backend_token_specifier.as_ref() {
            match v {
                client_config::BackendTokenSpecifier::BackendToken(v) => {
                    struct_ser.serialize_field("backendToken", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "instance_name",
            "instanceName",
            "max_cache_size",
            "maxCacheSize",
            "backend_token",
            "backendToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            InstanceName,
            MaxCacheSize,
            BackendToken,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "maxCacheSize" | "max_cache_size" => Ok(GeneratedField::MaxCacheSize),
                            "backendToken" | "backend_token" => Ok(GeneratedField::BackendToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.ClientConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut instance_name__ = None;
                let mut max_cache_size__ = None;
                let mut backend_token_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxCacheSize => {
                            if max_cache_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCacheSize"));
                            }
                            max_cache_size__ = map.next_value()?;
                        }
                        GeneratedField::BackendToken => {
                            if backend_token_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backendToken"));
                            }
                            backend_token_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(client_config::BackendTokenSpecifier::BackendToken);
                        }
                    }
                }
                Ok(ClientConfig {
                    service_name: service_name__.unwrap_or_default(),
                    instance_name: instance_name__.unwrap_or_default(),
                    max_cache_size: max_cache_size__,
                    backend_token_specifier: backend_token_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.ClientConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatadogConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collector_cluster.is_empty() {
            len += 1;
        }
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.collector_hostname.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.DatadogConfig", len)?;
        if !self.collector_cluster.is_empty() {
            struct_ser.serialize_field("collectorCluster", &self.collector_cluster)?;
        }
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.collector_hostname.is_empty() {
            struct_ser.serialize_field("collectorHostname", &self.collector_hostname)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatadogConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collector_cluster",
            "collectorCluster",
            "service_name",
            "serviceName",
            "collector_hostname",
            "collectorHostname",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectorCluster,
            ServiceName,
            CollectorHostname,
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
                            "collectorCluster" | "collector_cluster" => Ok(GeneratedField::CollectorCluster),
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "collectorHostname" | "collector_hostname" => Ok(GeneratedField::CollectorHostname),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatadogConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.DatadogConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DatadogConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collector_cluster__ = None;
                let mut service_name__ = None;
                let mut collector_hostname__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CollectorCluster => {
                            if collector_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorCluster"));
                            }
                            collector_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CollectorHostname => {
                            if collector_hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorHostname"));
                            }
                            collector_hostname__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(DatadogConfig {
                    collector_cluster: collector_cluster__.unwrap_or_default(),
                    service_name: service_name__.unwrap_or_default(),
                    collector_hostname: collector_hostname__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.DatadogConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicOtConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.library.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.DynamicOtConfig", len)?;
        if !self.library.is_empty() {
            struct_ser.serialize_field("library", &self.library)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicOtConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "library",
            "config",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Library,
            Config,
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
                            "library" => Ok(GeneratedField::Library),
                            "config" => Ok(GeneratedField::Config),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DynamicOtConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.DynamicOtConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DynamicOtConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut library__ = None;
                let mut config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Library => {
                            if library__.is_some() {
                                return Err(serde::de::Error::duplicate_field("library"));
                            }
                            library__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map.next_value()?;
                        }
                    }
                }
                Ok(DynamicOtConfig {
                    library: library__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.DynamicOtConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LightstepConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collector_cluster.is_empty() {
            len += 1;
        }
        if !self.access_token_file.is_empty() {
            len += 1;
        }
        if self.access_token.is_some() {
            len += 1;
        }
        if !self.propagation_modes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.LightstepConfig", len)?;
        if !self.collector_cluster.is_empty() {
            struct_ser.serialize_field("collectorCluster", &self.collector_cluster)?;
        }
        if !self.access_token_file.is_empty() {
            struct_ser.serialize_field("accessTokenFile", &self.access_token_file)?;
        }
        if let Some(v) = self.access_token.as_ref() {
            struct_ser.serialize_field("accessToken", v)?;
        }
        if !self.propagation_modes.is_empty() {
            let v = self.propagation_modes.iter().cloned().map(|v| {
                lightstep_config::PropagationMode::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("propagationModes", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LightstepConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collector_cluster",
            "collectorCluster",
            "access_token_file",
            "accessTokenFile",
            "access_token",
            "accessToken",
            "propagation_modes",
            "propagationModes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectorCluster,
            AccessTokenFile,
            AccessToken,
            PropagationModes,
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
                            "collectorCluster" | "collector_cluster" => Ok(GeneratedField::CollectorCluster),
                            "accessTokenFile" | "access_token_file" => Ok(GeneratedField::AccessTokenFile),
                            "accessToken" | "access_token" => Ok(GeneratedField::AccessToken),
                            "propagationModes" | "propagation_modes" => Ok(GeneratedField::PropagationModes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LightstepConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.LightstepConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LightstepConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collector_cluster__ = None;
                let mut access_token_file__ = None;
                let mut access_token__ = None;
                let mut propagation_modes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CollectorCluster => {
                            if collector_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorCluster"));
                            }
                            collector_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::AccessTokenFile => {
                            if access_token_file__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessTokenFile"));
                            }
                            access_token_file__ = Some(map.next_value()?);
                        }
                        GeneratedField::AccessToken => {
                            if access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessToken"));
                            }
                            access_token__ = map.next_value()?;
                        }
                        GeneratedField::PropagationModes => {
                            if propagation_modes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("propagationModes"));
                            }
                            propagation_modes__ = Some(map.next_value::<Vec<lightstep_config::PropagationMode>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(LightstepConfig {
                    collector_cluster: collector_cluster__.unwrap_or_default(),
                    access_token_file: access_token_file__.unwrap_or_default(),
                    access_token: access_token__,
                    propagation_modes: propagation_modes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.LightstepConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for lightstep_config::PropagationMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Envoy => "ENVOY",
            Self::Lightstep => "LIGHTSTEP",
            Self::B3 => "B3",
            Self::TraceContext => "TRACE_CONTEXT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for lightstep_config::PropagationMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ENVOY",
            "LIGHTSTEP",
            "B3",
            "TRACE_CONTEXT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = lightstep_config::PropagationMode;

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
                    .and_then(lightstep_config::PropagationMode::from_i32)
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
                    .and_then(lightstep_config::PropagationMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ENVOY" => Ok(lightstep_config::PropagationMode::Envoy),
                    "LIGHTSTEP" => Ok(lightstep_config::PropagationMode::Lightstep),
                    "B3" => Ok(lightstep_config::PropagationMode::B3),
                    "TRACE_CONTEXT" => Ok(lightstep_config::PropagationMode::TraceContext),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OpenCensusConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trace_config.is_some() {
            len += 1;
        }
        if self.stdout_exporter_enabled {
            len += 1;
        }
        if self.stackdriver_exporter_enabled {
            len += 1;
        }
        if !self.stackdriver_project_id.is_empty() {
            len += 1;
        }
        if !self.stackdriver_address.is_empty() {
            len += 1;
        }
        if self.stackdriver_grpc_service.is_some() {
            len += 1;
        }
        if self.zipkin_exporter_enabled {
            len += 1;
        }
        if !self.zipkin_url.is_empty() {
            len += 1;
        }
        if self.ocagent_exporter_enabled {
            len += 1;
        }
        if !self.ocagent_address.is_empty() {
            len += 1;
        }
        if self.ocagent_grpc_service.is_some() {
            len += 1;
        }
        if !self.incoming_trace_context.is_empty() {
            len += 1;
        }
        if !self.outgoing_trace_context.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.OpenCensusConfig", len)?;
        if let Some(v) = self.trace_config.as_ref() {
            struct_ser.serialize_field("traceConfig", v)?;
        }
        if self.stdout_exporter_enabled {
            struct_ser.serialize_field("stdoutExporterEnabled", &self.stdout_exporter_enabled)?;
        }
        if self.stackdriver_exporter_enabled {
            struct_ser.serialize_field("stackdriverExporterEnabled", &self.stackdriver_exporter_enabled)?;
        }
        if !self.stackdriver_project_id.is_empty() {
            struct_ser.serialize_field("stackdriverProjectId", &self.stackdriver_project_id)?;
        }
        if !self.stackdriver_address.is_empty() {
            struct_ser.serialize_field("stackdriverAddress", &self.stackdriver_address)?;
        }
        if let Some(v) = self.stackdriver_grpc_service.as_ref() {
            struct_ser.serialize_field("stackdriverGrpcService", v)?;
        }
        if self.zipkin_exporter_enabled {
            struct_ser.serialize_field("zipkinExporterEnabled", &self.zipkin_exporter_enabled)?;
        }
        if !self.zipkin_url.is_empty() {
            struct_ser.serialize_field("zipkinUrl", &self.zipkin_url)?;
        }
        if self.ocagent_exporter_enabled {
            struct_ser.serialize_field("ocagentExporterEnabled", &self.ocagent_exporter_enabled)?;
        }
        if !self.ocagent_address.is_empty() {
            struct_ser.serialize_field("ocagentAddress", &self.ocagent_address)?;
        }
        if let Some(v) = self.ocagent_grpc_service.as_ref() {
            struct_ser.serialize_field("ocagentGrpcService", v)?;
        }
        if !self.incoming_trace_context.is_empty() {
            let v = self.incoming_trace_context.iter().cloned().map(|v| {
                open_census_config::TraceContext::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("incomingTraceContext", &v)?;
        }
        if !self.outgoing_trace_context.is_empty() {
            let v = self.outgoing_trace_context.iter().cloned().map(|v| {
                open_census_config::TraceContext::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("outgoingTraceContext", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OpenCensusConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trace_config",
            "traceConfig",
            "stdout_exporter_enabled",
            "stdoutExporterEnabled",
            "stackdriver_exporter_enabled",
            "stackdriverExporterEnabled",
            "stackdriver_project_id",
            "stackdriverProjectId",
            "stackdriver_address",
            "stackdriverAddress",
            "stackdriver_grpc_service",
            "stackdriverGrpcService",
            "zipkin_exporter_enabled",
            "zipkinExporterEnabled",
            "zipkin_url",
            "zipkinUrl",
            "ocagent_exporter_enabled",
            "ocagentExporterEnabled",
            "ocagent_address",
            "ocagentAddress",
            "ocagent_grpc_service",
            "ocagentGrpcService",
            "incoming_trace_context",
            "incomingTraceContext",
            "outgoing_trace_context",
            "outgoingTraceContext",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TraceConfig,
            StdoutExporterEnabled,
            StackdriverExporterEnabled,
            StackdriverProjectId,
            StackdriverAddress,
            StackdriverGrpcService,
            ZipkinExporterEnabled,
            ZipkinUrl,
            OcagentExporterEnabled,
            OcagentAddress,
            OcagentGrpcService,
            IncomingTraceContext,
            OutgoingTraceContext,
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
                            "traceConfig" | "trace_config" => Ok(GeneratedField::TraceConfig),
                            "stdoutExporterEnabled" | "stdout_exporter_enabled" => Ok(GeneratedField::StdoutExporterEnabled),
                            "stackdriverExporterEnabled" | "stackdriver_exporter_enabled" => Ok(GeneratedField::StackdriverExporterEnabled),
                            "stackdriverProjectId" | "stackdriver_project_id" => Ok(GeneratedField::StackdriverProjectId),
                            "stackdriverAddress" | "stackdriver_address" => Ok(GeneratedField::StackdriverAddress),
                            "stackdriverGrpcService" | "stackdriver_grpc_service" => Ok(GeneratedField::StackdriverGrpcService),
                            "zipkinExporterEnabled" | "zipkin_exporter_enabled" => Ok(GeneratedField::ZipkinExporterEnabled),
                            "zipkinUrl" | "zipkin_url" => Ok(GeneratedField::ZipkinUrl),
                            "ocagentExporterEnabled" | "ocagent_exporter_enabled" => Ok(GeneratedField::OcagentExporterEnabled),
                            "ocagentAddress" | "ocagent_address" => Ok(GeneratedField::OcagentAddress),
                            "ocagentGrpcService" | "ocagent_grpc_service" => Ok(GeneratedField::OcagentGrpcService),
                            "incomingTraceContext" | "incoming_trace_context" => Ok(GeneratedField::IncomingTraceContext),
                            "outgoingTraceContext" | "outgoing_trace_context" => Ok(GeneratedField::OutgoingTraceContext),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OpenCensusConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.OpenCensusConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OpenCensusConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trace_config__ = None;
                let mut stdout_exporter_enabled__ = None;
                let mut stackdriver_exporter_enabled__ = None;
                let mut stackdriver_project_id__ = None;
                let mut stackdriver_address__ = None;
                let mut stackdriver_grpc_service__ = None;
                let mut zipkin_exporter_enabled__ = None;
                let mut zipkin_url__ = None;
                let mut ocagent_exporter_enabled__ = None;
                let mut ocagent_address__ = None;
                let mut ocagent_grpc_service__ = None;
                let mut incoming_trace_context__ = None;
                let mut outgoing_trace_context__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TraceConfig => {
                            if trace_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceConfig"));
                            }
                            trace_config__ = map.next_value()?;
                        }
                        GeneratedField::StdoutExporterEnabled => {
                            if stdout_exporter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stdoutExporterEnabled"));
                            }
                            stdout_exporter_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::StackdriverExporterEnabled => {
                            if stackdriver_exporter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackdriverExporterEnabled"));
                            }
                            stackdriver_exporter_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::StackdriverProjectId => {
                            if stackdriver_project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackdriverProjectId"));
                            }
                            stackdriver_project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::StackdriverAddress => {
                            if stackdriver_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackdriverAddress"));
                            }
                            stackdriver_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::StackdriverGrpcService => {
                            if stackdriver_grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackdriverGrpcService"));
                            }
                            stackdriver_grpc_service__ = map.next_value()?;
                        }
                        GeneratedField::ZipkinExporterEnabled => {
                            if zipkin_exporter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zipkinExporterEnabled"));
                            }
                            zipkin_exporter_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::ZipkinUrl => {
                            if zipkin_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("zipkinUrl"));
                            }
                            zipkin_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::OcagentExporterEnabled => {
                            if ocagent_exporter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ocagentExporterEnabled"));
                            }
                            ocagent_exporter_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::OcagentAddress => {
                            if ocagent_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ocagentAddress"));
                            }
                            ocagent_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::OcagentGrpcService => {
                            if ocagent_grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ocagentGrpcService"));
                            }
                            ocagent_grpc_service__ = map.next_value()?;
                        }
                        GeneratedField::IncomingTraceContext => {
                            if incoming_trace_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("incomingTraceContext"));
                            }
                            incoming_trace_context__ = Some(map.next_value::<Vec<open_census_config::TraceContext>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::OutgoingTraceContext => {
                            if outgoing_trace_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outgoingTraceContext"));
                            }
                            outgoing_trace_context__ = Some(map.next_value::<Vec<open_census_config::TraceContext>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(OpenCensusConfig {
                    trace_config: trace_config__,
                    stdout_exporter_enabled: stdout_exporter_enabled__.unwrap_or_default(),
                    stackdriver_exporter_enabled: stackdriver_exporter_enabled__.unwrap_or_default(),
                    stackdriver_project_id: stackdriver_project_id__.unwrap_or_default(),
                    stackdriver_address: stackdriver_address__.unwrap_or_default(),
                    stackdriver_grpc_service: stackdriver_grpc_service__,
                    zipkin_exporter_enabled: zipkin_exporter_enabled__.unwrap_or_default(),
                    zipkin_url: zipkin_url__.unwrap_or_default(),
                    ocagent_exporter_enabled: ocagent_exporter_enabled__.unwrap_or_default(),
                    ocagent_address: ocagent_address__.unwrap_or_default(),
                    ocagent_grpc_service: ocagent_grpc_service__,
                    incoming_trace_context: incoming_trace_context__.unwrap_or_default(),
                    outgoing_trace_context: outgoing_trace_context__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.OpenCensusConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for open_census_config::TraceContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::TraceContext => "TRACE_CONTEXT",
            Self::GrpcTraceBin => "GRPC_TRACE_BIN",
            Self::CloudTraceContext => "CLOUD_TRACE_CONTEXT",
            Self::B3 => "B3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for open_census_config::TraceContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "TRACE_CONTEXT",
            "GRPC_TRACE_BIN",
            "CLOUD_TRACE_CONTEXT",
            "B3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = open_census_config::TraceContext;

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
                    .and_then(open_census_config::TraceContext::from_i32)
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
                    .and_then(open_census_config::TraceContext::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(open_census_config::TraceContext::None),
                    "TRACE_CONTEXT" => Ok(open_census_config::TraceContext::TraceContext),
                    "GRPC_TRACE_BIN" => Ok(open_census_config::TraceContext::GrpcTraceBin),
                    "CLOUD_TRACE_CONTEXT" => Ok(open_census_config::TraceContext::CloudTraceContext),
                    "B3" => Ok(open_census_config::TraceContext::B3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OpenTelemetryConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        if !self.service_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.OpenTelemetryConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OpenTelemetryConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
            "service_name",
            "serviceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
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
            type Value = OpenTelemetryConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.OpenTelemetryConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OpenTelemetryConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                let mut service_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OpenTelemetryConfig {
                    grpc_service: grpc_service__,
                    service_name: service_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.OpenTelemetryConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SkyWalkingConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        if self.client_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.SkyWalkingConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        if let Some(v) = self.client_config.as_ref() {
            struct_ser.serialize_field("clientConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SkyWalkingConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
            "client_config",
            "clientConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
            ClientConfig,
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
                            "clientConfig" | "client_config" => Ok(GeneratedField::ClientConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SkyWalkingConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.SkyWalkingConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SkyWalkingConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                let mut client_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                        GeneratedField::ClientConfig => {
                            if client_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientConfig"));
                            }
                            client_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(SkyWalkingConfig {
                    grpc_service: grpc_service__,
                    client_config: client_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.SkyWalkingConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TraceServiceConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.TraceServiceConfig", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TraceServiceConfig {
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
            type Value = TraceServiceConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.TraceServiceConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TraceServiceConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                    }
                }
                Ok(TraceServiceConfig {
                    grpc_service: grpc_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.TraceServiceConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Tracing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.http.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.Tracing", len)?;
        if let Some(v) = self.http.as_ref() {
            struct_ser.serialize_field("http", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Tracing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "http",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Http,
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
                            "http" => Ok(GeneratedField::Http),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Tracing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.Tracing")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Tracing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut http__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Http => {
                            if http__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http"));
                            }
                            http__ = map.next_value()?;
                        }
                    }
                }
                Ok(Tracing {
                    http: http__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.Tracing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tracing::Http {
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.Tracing.Http", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                tracing::http::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tracing::Http {
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
            type Value = tracing::Http;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.Tracing.Http")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tracing::Http, V::Error>
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
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(tracing::http::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(tracing::Http {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.Tracing.Http", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for XRayConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.daemon_endpoint.is_some() {
            len += 1;
        }
        if !self.segment_name.is_empty() {
            len += 1;
        }
        if self.sampling_rule_manifest.is_some() {
            len += 1;
        }
        if self.segment_fields.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.XRayConfig", len)?;
        if let Some(v) = self.daemon_endpoint.as_ref() {
            struct_ser.serialize_field("daemonEndpoint", v)?;
        }
        if !self.segment_name.is_empty() {
            struct_ser.serialize_field("segmentName", &self.segment_name)?;
        }
        if let Some(v) = self.sampling_rule_manifest.as_ref() {
            struct_ser.serialize_field("samplingRuleManifest", v)?;
        }
        if let Some(v) = self.segment_fields.as_ref() {
            struct_ser.serialize_field("segmentFields", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for XRayConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "daemon_endpoint",
            "daemonEndpoint",
            "segment_name",
            "segmentName",
            "sampling_rule_manifest",
            "samplingRuleManifest",
            "segment_fields",
            "segmentFields",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DaemonEndpoint,
            SegmentName,
            SamplingRuleManifest,
            SegmentFields,
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
                            "daemonEndpoint" | "daemon_endpoint" => Ok(GeneratedField::DaemonEndpoint),
                            "segmentName" | "segment_name" => Ok(GeneratedField::SegmentName),
                            "samplingRuleManifest" | "sampling_rule_manifest" => Ok(GeneratedField::SamplingRuleManifest),
                            "segmentFields" | "segment_fields" => Ok(GeneratedField::SegmentFields),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = XRayConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.XRayConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<XRayConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut daemon_endpoint__ = None;
                let mut segment_name__ = None;
                let mut sampling_rule_manifest__ = None;
                let mut segment_fields__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DaemonEndpoint => {
                            if daemon_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daemonEndpoint"));
                            }
                            daemon_endpoint__ = map.next_value()?;
                        }
                        GeneratedField::SegmentName => {
                            if segment_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("segmentName"));
                            }
                            segment_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::SamplingRuleManifest => {
                            if sampling_rule_manifest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRuleManifest"));
                            }
                            sampling_rule_manifest__ = map.next_value()?;
                        }
                        GeneratedField::SegmentFields => {
                            if segment_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("segmentFields"));
                            }
                            segment_fields__ = map.next_value()?;
                        }
                    }
                }
                Ok(XRayConfig {
                    daemon_endpoint: daemon_endpoint__,
                    segment_name: segment_name__.unwrap_or_default(),
                    sampling_rule_manifest: sampling_rule_manifest__,
                    segment_fields: segment_fields__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.XRayConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for x_ray_config::SegmentFields {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.origin.is_empty() {
            len += 1;
        }
        if self.aws.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.XRayConfig.SegmentFields", len)?;
        if !self.origin.is_empty() {
            struct_ser.serialize_field("origin", &self.origin)?;
        }
        if let Some(v) = self.aws.as_ref() {
            struct_ser.serialize_field("aws", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for x_ray_config::SegmentFields {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "origin",
            "aws",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Origin,
            Aws,
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
                            "origin" => Ok(GeneratedField::Origin),
                            "aws" => Ok(GeneratedField::Aws),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = x_ray_config::SegmentFields;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.XRayConfig.SegmentFields")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<x_ray_config::SegmentFields, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut origin__ = None;
                let mut aws__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Origin => {
                            if origin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("origin"));
                            }
                            origin__ = Some(map.next_value()?);
                        }
                        GeneratedField::Aws => {
                            if aws__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aws"));
                            }
                            aws__ = map.next_value()?;
                        }
                    }
                }
                Ok(x_ray_config::SegmentFields {
                    origin: origin__.unwrap_or_default(),
                    aws: aws__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.XRayConfig.SegmentFields", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ZipkinConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collector_cluster.is_empty() {
            len += 1;
        }
        if !self.collector_endpoint.is_empty() {
            len += 1;
        }
        if self.trace_id_128bit {
            len += 1;
        }
        if self.shared_span_context.is_some() {
            len += 1;
        }
        if self.collector_endpoint_version != 0 {
            len += 1;
        }
        if !self.collector_hostname.is_empty() {
            len += 1;
        }
        if self.split_spans_for_request {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v3.ZipkinConfig", len)?;
        if !self.collector_cluster.is_empty() {
            struct_ser.serialize_field("collectorCluster", &self.collector_cluster)?;
        }
        if !self.collector_endpoint.is_empty() {
            struct_ser.serialize_field("collectorEndpoint", &self.collector_endpoint)?;
        }
        if self.trace_id_128bit {
            struct_ser.serialize_field("traceId128bit", &self.trace_id_128bit)?;
        }
        if let Some(v) = self.shared_span_context.as_ref() {
            struct_ser.serialize_field("sharedSpanContext", v)?;
        }
        if self.collector_endpoint_version != 0 {
            let v = zipkin_config::CollectorEndpointVersion::from_i32(self.collector_endpoint_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.collector_endpoint_version)))?;
            struct_ser.serialize_field("collectorEndpointVersion", &v)?;
        }
        if !self.collector_hostname.is_empty() {
            struct_ser.serialize_field("collectorHostname", &self.collector_hostname)?;
        }
        if self.split_spans_for_request {
            struct_ser.serialize_field("splitSpansForRequest", &self.split_spans_for_request)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZipkinConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collector_cluster",
            "collectorCluster",
            "collector_endpoint",
            "collectorEndpoint",
            "trace_id_128bit",
            "traceId128bit",
            "shared_span_context",
            "sharedSpanContext",
            "collector_endpoint_version",
            "collectorEndpointVersion",
            "collector_hostname",
            "collectorHostname",
            "split_spans_for_request",
            "splitSpansForRequest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectorCluster,
            CollectorEndpoint,
            TraceId128bit,
            SharedSpanContext,
            CollectorEndpointVersion,
            CollectorHostname,
            SplitSpansForRequest,
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
                            "collectorCluster" | "collector_cluster" => Ok(GeneratedField::CollectorCluster),
                            "collectorEndpoint" | "collector_endpoint" => Ok(GeneratedField::CollectorEndpoint),
                            "traceId128bit" | "trace_id_128bit" => Ok(GeneratedField::TraceId128bit),
                            "sharedSpanContext" | "shared_span_context" => Ok(GeneratedField::SharedSpanContext),
                            "collectorEndpointVersion" | "collector_endpoint_version" => Ok(GeneratedField::CollectorEndpointVersion),
                            "collectorHostname" | "collector_hostname" => Ok(GeneratedField::CollectorHostname),
                            "splitSpansForRequest" | "split_spans_for_request" => Ok(GeneratedField::SplitSpansForRequest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZipkinConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v3.ZipkinConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ZipkinConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collector_cluster__ = None;
                let mut collector_endpoint__ = None;
                let mut trace_id_128bit__ = None;
                let mut shared_span_context__ = None;
                let mut collector_endpoint_version__ = None;
                let mut collector_hostname__ = None;
                let mut split_spans_for_request__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CollectorCluster => {
                            if collector_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorCluster"));
                            }
                            collector_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::CollectorEndpoint => {
                            if collector_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorEndpoint"));
                            }
                            collector_endpoint__ = Some(map.next_value()?);
                        }
                        GeneratedField::TraceId128bit => {
                            if trace_id_128bit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId128bit"));
                            }
                            trace_id_128bit__ = Some(map.next_value()?);
                        }
                        GeneratedField::SharedSpanContext => {
                            if shared_span_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharedSpanContext"));
                            }
                            shared_span_context__ = map.next_value()?;
                        }
                        GeneratedField::CollectorEndpointVersion => {
                            if collector_endpoint_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorEndpointVersion"));
                            }
                            collector_endpoint_version__ = Some(map.next_value::<zipkin_config::CollectorEndpointVersion>()? as i32);
                        }
                        GeneratedField::CollectorHostname => {
                            if collector_hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectorHostname"));
                            }
                            collector_hostname__ = Some(map.next_value()?);
                        }
                        GeneratedField::SplitSpansForRequest => {
                            if split_spans_for_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("splitSpansForRequest"));
                            }
                            split_spans_for_request__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ZipkinConfig {
                    collector_cluster: collector_cluster__.unwrap_or_default(),
                    collector_endpoint: collector_endpoint__.unwrap_or_default(),
                    trace_id_128bit: trace_id_128bit__.unwrap_or_default(),
                    shared_span_context: shared_span_context__,
                    collector_endpoint_version: collector_endpoint_version__.unwrap_or_default(),
                    collector_hostname: collector_hostname__.unwrap_or_default(),
                    split_spans_for_request: split_spans_for_request__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v3.ZipkinConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for zipkin_config::CollectorEndpointVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DeprecatedAndUnavailableDoNotUse => "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE",
            Self::HttpJson => "HTTP_JSON",
            Self::HttpProto => "HTTP_PROTO",
            Self::Grpc => "GRPC",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for zipkin_config::CollectorEndpointVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE",
            "HTTP_JSON",
            "HTTP_PROTO",
            "GRPC",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = zipkin_config::CollectorEndpointVersion;

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
                    .and_then(zipkin_config::CollectorEndpointVersion::from_i32)
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
                    .and_then(zipkin_config::CollectorEndpointVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEPRECATED_AND_UNAVAILABLE_DO_NOT_USE" => Ok(zipkin_config::CollectorEndpointVersion::DeprecatedAndUnavailableDoNotUse),
                    "HTTP_JSON" => Ok(zipkin_config::CollectorEndpointVersion::HttpJson),
                    "HTTP_PROTO" => Ok(zipkin_config::CollectorEndpointVersion::HttpProto),
                    "GRPC" => Ok(zipkin_config::CollectorEndpointVersion::Grpc),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
