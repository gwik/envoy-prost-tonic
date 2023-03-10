// @generated
impl serde::Serialize for AdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.AdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AdsDummy {
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
            type Value = AdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.AdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(AdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.AdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Capability {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.health_check_protocols.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.Capability", len)?;
        if !self.health_check_protocols.is_empty() {
            let v = self.health_check_protocols.iter().cloned().map(|v| {
                capability::Protocol::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("healthCheckProtocols", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Capability {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_check_protocols",
            "healthCheckProtocols",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthCheckProtocols,
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
                            "healthCheckProtocols" | "health_check_protocols" => Ok(GeneratedField::HealthCheckProtocols),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Capability;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.Capability")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Capability, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut health_check_protocols__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HealthCheckProtocols => {
                            if health_check_protocols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckProtocols"));
                            }
                            health_check_protocols__ = Some(map.next_value::<Vec<capability::Protocol>>()?.into_iter().map(|x| x as i32).collect());
                        }
                    }
                }
                Ok(Capability {
                    health_check_protocols: health_check_protocols__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.Capability", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for capability::Protocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Http => "HTTP",
            Self::Tcp => "TCP",
            Self::Redis => "REDIS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for capability::Protocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HTTP",
            "TCP",
            "REDIS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = capability::Protocol;

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
                    .and_then(capability::Protocol::from_i32)
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
                    .and_then(capability::Protocol::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HTTP" => Ok(capability::Protocol::Http),
                    "TCP" => Ok(capability::Protocol::Tcp),
                    "REDIS" => Ok(capability::Protocol::Redis),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterHealthCheck {
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
        if !self.health_checks.is_empty() {
            len += 1;
        }
        if !self.locality_endpoints.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.ClusterHealthCheck", len)?;
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("clusterName", &self.cluster_name)?;
        }
        if !self.health_checks.is_empty() {
            struct_ser.serialize_field("healthChecks", &self.health_checks)?;
        }
        if !self.locality_endpoints.is_empty() {
            struct_ser.serialize_field("localityEndpoints", &self.locality_endpoints)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterHealthCheck {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_name",
            "clusterName",
            "health_checks",
            "healthChecks",
            "locality_endpoints",
            "localityEndpoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterName,
            HealthChecks,
            LocalityEndpoints,
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
                            "healthChecks" | "health_checks" => Ok(GeneratedField::HealthChecks),
                            "localityEndpoints" | "locality_endpoints" => Ok(GeneratedField::LocalityEndpoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterHealthCheck;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.ClusterHealthCheck")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterHealthCheck, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_name__ = None;
                let mut health_checks__ = None;
                let mut locality_endpoints__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::HealthChecks => {
                            if health_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthChecks"));
                            }
                            health_checks__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalityEndpoints => {
                            if locality_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityEndpoints"));
                            }
                            locality_endpoints__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClusterHealthCheck {
                    cluster_name: cluster_name__.unwrap_or_default(),
                    health_checks: health_checks__.unwrap_or_default(),
                    locality_endpoints: locality_endpoints__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.ClusterHealthCheck", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointHealth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint.is_some() {
            len += 1;
        }
        if self.health_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.EndpointHealth", len)?;
        if let Some(v) = self.endpoint.as_ref() {
            struct_ser.serialize_field("endpoint", v)?;
        }
        if self.health_status != 0 {
            let v = super::super::super::api::v2::core::HealthStatus::from_i32(self.health_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.health_status)))?;
            struct_ser.serialize_field("healthStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointHealth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint",
            "health_status",
            "healthStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoint,
            HealthStatus,
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
                            "endpoint" => Ok(GeneratedField::Endpoint),
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointHealth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.EndpointHealth")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EndpointHealth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint__ = None;
                let mut health_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Endpoint => {
                            if endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            endpoint__ = map.next_value()?;
                        }
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = Some(map.next_value::<super::super::super::api::v2::core::HealthStatus>()? as i32);
                        }
                    }
                }
                Ok(EndpointHealth {
                    endpoint: endpoint__,
                    health_status: health_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.EndpointHealth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointHealthResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.endpoints_health.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.EndpointHealthResponse", len)?;
        if !self.endpoints_health.is_empty() {
            struct_ser.serialize_field("endpointsHealth", &self.endpoints_health)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointHealthResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoints_health",
            "endpointsHealth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EndpointsHealth,
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
                            "endpointsHealth" | "endpoints_health" => Ok(GeneratedField::EndpointsHealth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointHealthResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.EndpointHealthResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EndpointHealthResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoints_health__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EndpointsHealth => {
                            if endpoints_health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointsHealth"));
                            }
                            endpoints_health__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EndpointHealthResponse {
                    endpoints_health: endpoints_health__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.EndpointHealthResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.node.is_some() {
            len += 1;
        }
        if self.capability.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.HealthCheckRequest", len)?;
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        if let Some(v) = self.capability.as_ref() {
            struct_ser.serialize_field("capability", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "capability",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            Capability,
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
                            "node" => Ok(GeneratedField::Node),
                            "capability" => Ok(GeneratedField::Capability),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.HealthCheckRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut capability__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                        GeneratedField::Capability => {
                            if capability__.is_some() {
                                return Err(serde::de::Error::duplicate_field("capability"));
                            }
                            capability__ = map.next_value()?;
                        }
                    }
                }
                Ok(HealthCheckRequest {
                    node: node__,
                    capability: capability__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.HealthCheckRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckRequestOrEndpointHealthResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.HealthCheckRequestOrEndpointHealthResponse", len)?;
        if let Some(v) = self.request_type.as_ref() {
            match v {
                health_check_request_or_endpoint_health_response::RequestType::HealthCheckRequest(v) => {
                    struct_ser.serialize_field("healthCheckRequest", v)?;
                }
                health_check_request_or_endpoint_health_response::RequestType::EndpointHealthResponse(v) => {
                    struct_ser.serialize_field("endpointHealthResponse", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckRequestOrEndpointHealthResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_check_request",
            "healthCheckRequest",
            "endpoint_health_response",
            "endpointHealthResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthCheckRequest,
            EndpointHealthResponse,
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
                            "healthCheckRequest" | "health_check_request" => Ok(GeneratedField::HealthCheckRequest),
                            "endpointHealthResponse" | "endpoint_health_response" => Ok(GeneratedField::EndpointHealthResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckRequestOrEndpointHealthResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.HealthCheckRequestOrEndpointHealthResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckRequestOrEndpointHealthResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HealthCheckRequest => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckRequest"));
                            }
                            request_type__ = map.next_value::<::std::option::Option<_>>()?.map(health_check_request_or_endpoint_health_response::RequestType::HealthCheckRequest)
;
                        }
                        GeneratedField::EndpointHealthResponse => {
                            if request_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointHealthResponse"));
                            }
                            request_type__ = map.next_value::<::std::option::Option<_>>()?.map(health_check_request_or_endpoint_health_response::RequestType::EndpointHealthResponse)
;
                        }
                    }
                }
                Ok(HealthCheckRequestOrEndpointHealthResponse {
                    request_type: request_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.HealthCheckRequestOrEndpointHealthResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckSpecifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_health_checks.is_empty() {
            len += 1;
        }
        if self.interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.HealthCheckSpecifier", len)?;
        if !self.cluster_health_checks.is_empty() {
            struct_ser.serialize_field("clusterHealthChecks", &self.cluster_health_checks)?;
        }
        if let Some(v) = self.interval.as_ref() {
            struct_ser.serialize_field("interval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckSpecifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_health_checks",
            "clusterHealthChecks",
            "interval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterHealthChecks,
            Interval,
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
                            "clusterHealthChecks" | "cluster_health_checks" => Ok(GeneratedField::ClusterHealthChecks),
                            "interval" => Ok(GeneratedField::Interval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckSpecifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.HealthCheckSpecifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckSpecifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_health_checks__ = None;
                let mut interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterHealthChecks => {
                            if cluster_health_checks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterHealthChecks"));
                            }
                            cluster_health_checks__ = Some(map.next_value()?);
                        }
                        GeneratedField::Interval => {
                            if interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(HealthCheckSpecifier {
                    cluster_health_checks: cluster_health_checks__.unwrap_or_default(),
                    interval: interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.HealthCheckSpecifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LocalityEndpoints {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.locality.is_some() {
            len += 1;
        }
        if !self.endpoints.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.LocalityEndpoints", len)?;
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        if !self.endpoints.is_empty() {
            struct_ser.serialize_field("endpoints", &self.endpoints)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LocalityEndpoints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "locality",
            "endpoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Locality,
            Endpoints,
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
                            "locality" => Ok(GeneratedField::Locality),
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LocalityEndpoints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.LocalityEndpoints")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LocalityEndpoints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut locality__ = None;
                let mut endpoints__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map.next_value()?;
                        }
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LocalityEndpoints {
                    locality: locality__,
                    endpoints: endpoints__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.LocalityEndpoints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RtdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.RtdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RtdsDummy {
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
            type Value = RtdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.RtdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RtdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RtdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.RtdsDummy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Runtime {
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
        if self.layer.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.Runtime", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.layer.as_ref() {
            struct_ser.serialize_field("layer", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Runtime {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "layer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Layer,
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
                            "layer" => Ok(GeneratedField::Layer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Runtime;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.Runtime")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Runtime, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut layer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Layer => {
                            if layer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("layer"));
                            }
                            layer__ = map.next_value()?;
                        }
                    }
                }
                Ok(Runtime {
                    name: name__.unwrap_or_default(),
                    layer: layer__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.Runtime", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SdsDummy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.service.discovery.v2.SdsDummy", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SdsDummy {
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
            type Value = SdsDummy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.service.discovery.v2.SdsDummy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SdsDummy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SdsDummy {
                })
            }
        }
        deserializer.deserialize_struct("envoy.service.discovery.v2.SdsDummy", FIELDS, GeneratedVisitor)
    }
}
