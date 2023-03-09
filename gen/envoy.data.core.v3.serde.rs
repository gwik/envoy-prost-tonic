// @generated
impl serde::Serialize for DegradedHealthyHost {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.data.core.v3.DegradedHealthyHost", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DegradedHealthyHost {
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
            type Value = DegradedHealthyHost;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.core.v3.DegradedHealthyHost")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DegradedHealthyHost, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DegradedHealthyHost {
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.core.v3.DegradedHealthyHost", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckAddHealthy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.first_check {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.core.v3.HealthCheckAddHealthy", len)?;
        if self.first_check {
            struct_ser.serialize_field("firstCheck", &self.first_check)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckAddHealthy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "first_check",
            "firstCheck",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FirstCheck,
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
                            "firstCheck" | "first_check" => Ok(GeneratedField::FirstCheck),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckAddHealthy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.core.v3.HealthCheckAddHealthy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckAddHealthy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut first_check__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FirstCheck => {
                            if first_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstCheck"));
                            }
                            first_check__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HealthCheckAddHealthy {
                    first_check: first_check__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.core.v3.HealthCheckAddHealthy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckEjectUnhealthy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failure_type != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.core.v3.HealthCheckEjectUnhealthy", len)?;
        if self.failure_type != 0 {
            let v = HealthCheckFailureType::from_i32(self.failure_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.failure_type)))?;
            struct_ser.serialize_field("failureType", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckEjectUnhealthy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failure_type",
            "failureType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailureType,
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
                            "failureType" | "failure_type" => Ok(GeneratedField::FailureType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckEjectUnhealthy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.core.v3.HealthCheckEjectUnhealthy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckEjectUnhealthy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failure_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FailureType => {
                            if failure_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureType"));
                            }
                            failure_type__ = Some(map.next_value::<HealthCheckFailureType>()? as i32);
                        }
                    }
                }
                Ok(HealthCheckEjectUnhealthy {
                    failure_type: failure_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.core.v3.HealthCheckEjectUnhealthy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.health_checker_type != 0 {
            len += 1;
        }
        if self.host.is_some() {
            len += 1;
        }
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.core.v3.HealthCheckEvent", len)?;
        if self.health_checker_type != 0 {
            let v = HealthCheckerType::from_i32(self.health_checker_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.health_checker_type)))?;
            struct_ser.serialize_field("healthCheckerType", &v)?;
        }
        if let Some(v) = self.host.as_ref() {
            struct_ser.serialize_field("host", v)?;
        }
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("clusterName", &self.cluster_name)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.event.as_ref() {
            match v {
                health_check_event::Event::EjectUnhealthyEvent(v) => {
                    struct_ser.serialize_field("ejectUnhealthyEvent", v)?;
                }
                health_check_event::Event::AddHealthyEvent(v) => {
                    struct_ser.serialize_field("addHealthyEvent", v)?;
                }
                health_check_event::Event::HealthCheckFailureEvent(v) => {
                    struct_ser.serialize_field("healthCheckFailureEvent", v)?;
                }
                health_check_event::Event::DegradedHealthyHost(v) => {
                    struct_ser.serialize_field("degradedHealthyHost", v)?;
                }
                health_check_event::Event::NoLongerDegradedHost(v) => {
                    struct_ser.serialize_field("noLongerDegradedHost", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "health_checker_type",
            "healthCheckerType",
            "host",
            "cluster_name",
            "clusterName",
            "timestamp",
            "eject_unhealthy_event",
            "ejectUnhealthyEvent",
            "add_healthy_event",
            "addHealthyEvent",
            "health_check_failure_event",
            "healthCheckFailureEvent",
            "degraded_healthy_host",
            "degradedHealthyHost",
            "no_longer_degraded_host",
            "noLongerDegradedHost",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HealthCheckerType,
            Host,
            ClusterName,
            Timestamp,
            EjectUnhealthyEvent,
            AddHealthyEvent,
            HealthCheckFailureEvent,
            DegradedHealthyHost,
            NoLongerDegradedHost,
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
                            "healthCheckerType" | "health_checker_type" => Ok(GeneratedField::HealthCheckerType),
                            "host" => Ok(GeneratedField::Host),
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "ejectUnhealthyEvent" | "eject_unhealthy_event" => Ok(GeneratedField::EjectUnhealthyEvent),
                            "addHealthyEvent" | "add_healthy_event" => Ok(GeneratedField::AddHealthyEvent),
                            "healthCheckFailureEvent" | "health_check_failure_event" => Ok(GeneratedField::HealthCheckFailureEvent),
                            "degradedHealthyHost" | "degraded_healthy_host" => Ok(GeneratedField::DegradedHealthyHost),
                            "noLongerDegradedHost" | "no_longer_degraded_host" => Ok(GeneratedField::NoLongerDegradedHost),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.core.v3.HealthCheckEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut health_checker_type__ = None;
                let mut host__ = None;
                let mut cluster_name__ = None;
                let mut timestamp__ = None;
                let mut event__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HealthCheckerType => {
                            if health_checker_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckerType"));
                            }
                            health_checker_type__ = Some(map.next_value::<HealthCheckerType>()? as i32);
                        }
                        GeneratedField::Host => {
                            if host__.is_some() {
                                return Err(serde::de::Error::duplicate_field("host"));
                            }
                            host__ = map.next_value()?;
                        }
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::EjectUnhealthyEvent => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ejectUnhealthyEvent"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(health_check_event::Event::EjectUnhealthyEvent)
;
                        }
                        GeneratedField::AddHealthyEvent => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addHealthyEvent"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(health_check_event::Event::AddHealthyEvent)
;
                        }
                        GeneratedField::HealthCheckFailureEvent => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthCheckFailureEvent"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(health_check_event::Event::HealthCheckFailureEvent)
;
                        }
                        GeneratedField::DegradedHealthyHost => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("degradedHealthyHost"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(health_check_event::Event::DegradedHealthyHost)
;
                        }
                        GeneratedField::NoLongerDegradedHost => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noLongerDegradedHost"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(health_check_event::Event::NoLongerDegradedHost)
;
                        }
                    }
                }
                Ok(HealthCheckEvent {
                    health_checker_type: health_checker_type__.unwrap_or_default(),
                    host: host__,
                    cluster_name: cluster_name__.unwrap_or_default(),
                    timestamp: timestamp__,
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.core.v3.HealthCheckEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failure_type != 0 {
            len += 1;
        }
        if self.first_check {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.core.v3.HealthCheckFailure", len)?;
        if self.failure_type != 0 {
            let v = HealthCheckFailureType::from_i32(self.failure_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.failure_type)))?;
            struct_ser.serialize_field("failureType", &v)?;
        }
        if self.first_check {
            struct_ser.serialize_field("firstCheck", &self.first_check)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failure_type",
            "failureType",
            "first_check",
            "firstCheck",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailureType,
            FirstCheck,
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
                            "failureType" | "failure_type" => Ok(GeneratedField::FailureType),
                            "firstCheck" | "first_check" => Ok(GeneratedField::FirstCheck),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.core.v3.HealthCheckFailure")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HealthCheckFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failure_type__ = None;
                let mut first_check__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FailureType => {
                            if failure_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureType"));
                            }
                            failure_type__ = Some(map.next_value::<HealthCheckFailureType>()? as i32);
                        }
                        GeneratedField::FirstCheck => {
                            if first_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstCheck"));
                            }
                            first_check__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HealthCheckFailure {
                    failure_type: failure_type__.unwrap_or_default(),
                    first_check: first_check__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.core.v3.HealthCheckFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckFailureType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Active => "ACTIVE",
            Self::Passive => "PASSIVE",
            Self::Network => "NETWORK",
            Self::NetworkTimeout => "NETWORK_TIMEOUT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckFailureType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTIVE",
            "PASSIVE",
            "NETWORK",
            "NETWORK_TIMEOUT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckFailureType;

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
                    .and_then(HealthCheckFailureType::from_i32)
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
                    .and_then(HealthCheckFailureType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACTIVE" => Ok(HealthCheckFailureType::Active),
                    "PASSIVE" => Ok(HealthCheckFailureType::Passive),
                    "NETWORK" => Ok(HealthCheckFailureType::Network),
                    "NETWORK_TIMEOUT" => Ok(HealthCheckFailureType::NetworkTimeout),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HealthCheckerType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Http => "HTTP",
            Self::Tcp => "TCP",
            Self::Grpc => "GRPC",
            Self::Redis => "REDIS",
            Self::Thrift => "THRIFT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckerType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HTTP",
            "TCP",
            "GRPC",
            "REDIS",
            "THRIFT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HealthCheckerType;

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
                    .and_then(HealthCheckerType::from_i32)
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
                    .and_then(HealthCheckerType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HTTP" => Ok(HealthCheckerType::Http),
                    "TCP" => Ok(HealthCheckerType::Tcp),
                    "GRPC" => Ok(HealthCheckerType::Grpc),
                    "REDIS" => Ok(HealthCheckerType::Redis),
                    "THRIFT" => Ok(HealthCheckerType::Thrift),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for NoLongerDegradedHost {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.data.core.v3.NoLongerDegradedHost", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NoLongerDegradedHost {
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
            type Value = NoLongerDegradedHost;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.core.v3.NoLongerDegradedHost")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NoLongerDegradedHost, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(NoLongerDegradedHost {
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.core.v3.NoLongerDegradedHost", FIELDS, GeneratedVisitor)
    }
}
