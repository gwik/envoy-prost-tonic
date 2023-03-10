// @generated
impl serde::Serialize for Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Eject => "EJECT",
            Self::Uneject => "UNEJECT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "EJECT",
            "UNEJECT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Action;

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
                    .and_then(Action::from_i32)
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
                    .and_then(Action::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "EJECT" => Ok(Action::Eject),
                    "UNEJECT" => Ok(Action::Uneject),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for OutlierDetectionEvent {
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
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.secs_since_last_action.is_some() {
            len += 1;
        }
        if !self.cluster_name.is_empty() {
            len += 1;
        }
        if !self.upstream_url.is_empty() {
            len += 1;
        }
        if self.action != 0 {
            len += 1;
        }
        if self.num_ejections != 0 {
            len += 1;
        }
        if self.enforced {
            len += 1;
        }
        if self.event.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.cluster.v2alpha.OutlierDetectionEvent", len)?;
        if self.r#type != 0 {
            let v = OutlierEjectionType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.secs_since_last_action.as_ref() {
            struct_ser.serialize_field("secsSinceLastAction", v)?;
        }
        if !self.cluster_name.is_empty() {
            struct_ser.serialize_field("clusterName", &self.cluster_name)?;
        }
        if !self.upstream_url.is_empty() {
            struct_ser.serialize_field("upstreamUrl", &self.upstream_url)?;
        }
        if self.action != 0 {
            let v = Action::from_i32(self.action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if self.num_ejections != 0 {
            struct_ser.serialize_field("numEjections", &self.num_ejections)?;
        }
        if self.enforced {
            struct_ser.serialize_field("enforced", &self.enforced)?;
        }
        if let Some(v) = self.event.as_ref() {
            match v {
                outlier_detection_event::Event::EjectSuccessRateEvent(v) => {
                    struct_ser.serialize_field("ejectSuccessRateEvent", v)?;
                }
                outlier_detection_event::Event::EjectConsecutiveEvent(v) => {
                    struct_ser.serialize_field("ejectConsecutiveEvent", v)?;
                }
                outlier_detection_event::Event::EjectFailurePercentageEvent(v) => {
                    struct_ser.serialize_field("ejectFailurePercentageEvent", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutlierDetectionEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "timestamp",
            "secs_since_last_action",
            "secsSinceLastAction",
            "cluster_name",
            "clusterName",
            "upstream_url",
            "upstreamUrl",
            "action",
            "num_ejections",
            "numEjections",
            "enforced",
            "eject_success_rate_event",
            "ejectSuccessRateEvent",
            "eject_consecutive_event",
            "ejectConsecutiveEvent",
            "eject_failure_percentage_event",
            "ejectFailurePercentageEvent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Timestamp,
            SecsSinceLastAction,
            ClusterName,
            UpstreamUrl,
            Action,
            NumEjections,
            Enforced,
            EjectSuccessRateEvent,
            EjectConsecutiveEvent,
            EjectFailurePercentageEvent,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "secsSinceLastAction" | "secs_since_last_action" => Ok(GeneratedField::SecsSinceLastAction),
                            "clusterName" | "cluster_name" => Ok(GeneratedField::ClusterName),
                            "upstreamUrl" | "upstream_url" => Ok(GeneratedField::UpstreamUrl),
                            "action" => Ok(GeneratedField::Action),
                            "numEjections" | "num_ejections" => Ok(GeneratedField::NumEjections),
                            "enforced" => Ok(GeneratedField::Enforced),
                            "ejectSuccessRateEvent" | "eject_success_rate_event" => Ok(GeneratedField::EjectSuccessRateEvent),
                            "ejectConsecutiveEvent" | "eject_consecutive_event" => Ok(GeneratedField::EjectConsecutiveEvent),
                            "ejectFailurePercentageEvent" | "eject_failure_percentage_event" => Ok(GeneratedField::EjectFailurePercentageEvent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutlierDetectionEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.cluster.v2alpha.OutlierDetectionEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OutlierDetectionEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut timestamp__ = None;
                let mut secs_since_last_action__ = None;
                let mut cluster_name__ = None;
                let mut upstream_url__ = None;
                let mut action__ = None;
                let mut num_ejections__ = None;
                let mut enforced__ = None;
                let mut event__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<OutlierEjectionType>()? as i32);
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::SecsSinceLastAction => {
                            if secs_since_last_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secsSinceLastAction"));
                            }
                            secs_since_last_action__ = map.next_value()?;
                        }
                        GeneratedField::ClusterName => {
                            if cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterName"));
                            }
                            cluster_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamUrl => {
                            if upstream_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamUrl"));
                            }
                            upstream_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map.next_value::<Action>()? as i32);
                        }
                        GeneratedField::NumEjections => {
                            if num_ejections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numEjections"));
                            }
                            num_ejections__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Enforced => {
                            if enforced__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enforced"));
                            }
                            enforced__ = Some(map.next_value()?);
                        }
                        GeneratedField::EjectSuccessRateEvent => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ejectSuccessRateEvent"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(outlier_detection_event::Event::EjectSuccessRateEvent)
;
                        }
                        GeneratedField::EjectConsecutiveEvent => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ejectConsecutiveEvent"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(outlier_detection_event::Event::EjectConsecutiveEvent)
;
                        }
                        GeneratedField::EjectFailurePercentageEvent => {
                            if event__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ejectFailurePercentageEvent"));
                            }
                            event__ = map.next_value::<::std::option::Option<_>>()?.map(outlier_detection_event::Event::EjectFailurePercentageEvent)
;
                        }
                    }
                }
                Ok(OutlierDetectionEvent {
                    r#type: r#type__.unwrap_or_default(),
                    timestamp: timestamp__,
                    secs_since_last_action: secs_since_last_action__,
                    cluster_name: cluster_name__.unwrap_or_default(),
                    upstream_url: upstream_url__.unwrap_or_default(),
                    action: action__.unwrap_or_default(),
                    num_ejections: num_ejections__.unwrap_or_default(),
                    enforced: enforced__.unwrap_or_default(),
                    event: event__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.cluster.v2alpha.OutlierDetectionEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutlierEjectConsecutive {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.data.cluster.v2alpha.OutlierEjectConsecutive", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutlierEjectConsecutive {
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
            type Value = OutlierEjectConsecutive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.cluster.v2alpha.OutlierEjectConsecutive")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OutlierEjectConsecutive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(OutlierEjectConsecutive {
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.cluster.v2alpha.OutlierEjectConsecutive", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutlierEjectFailurePercentage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_success_rate != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.cluster.v2alpha.OutlierEjectFailurePercentage", len)?;
        if self.host_success_rate != 0 {
            struct_ser.serialize_field("hostSuccessRate", &self.host_success_rate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutlierEjectFailurePercentage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_success_rate",
            "hostSuccessRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostSuccessRate,
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
                            "hostSuccessRate" | "host_success_rate" => Ok(GeneratedField::HostSuccessRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutlierEjectFailurePercentage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.cluster.v2alpha.OutlierEjectFailurePercentage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OutlierEjectFailurePercentage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_success_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostSuccessRate => {
                            if host_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostSuccessRate"));
                            }
                            host_success_rate__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(OutlierEjectFailurePercentage {
                    host_success_rate: host_success_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.cluster.v2alpha.OutlierEjectFailurePercentage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutlierEjectSuccessRate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_success_rate != 0 {
            len += 1;
        }
        if self.cluster_average_success_rate != 0 {
            len += 1;
        }
        if self.cluster_success_rate_ejection_threshold != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.cluster.v2alpha.OutlierEjectSuccessRate", len)?;
        if self.host_success_rate != 0 {
            struct_ser.serialize_field("hostSuccessRate", &self.host_success_rate)?;
        }
        if self.cluster_average_success_rate != 0 {
            struct_ser.serialize_field("clusterAverageSuccessRate", &self.cluster_average_success_rate)?;
        }
        if self.cluster_success_rate_ejection_threshold != 0 {
            struct_ser.serialize_field("clusterSuccessRateEjectionThreshold", &self.cluster_success_rate_ejection_threshold)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutlierEjectSuccessRate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_success_rate",
            "hostSuccessRate",
            "cluster_average_success_rate",
            "clusterAverageSuccessRate",
            "cluster_success_rate_ejection_threshold",
            "clusterSuccessRateEjectionThreshold",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostSuccessRate,
            ClusterAverageSuccessRate,
            ClusterSuccessRateEjectionThreshold,
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
                            "hostSuccessRate" | "host_success_rate" => Ok(GeneratedField::HostSuccessRate),
                            "clusterAverageSuccessRate" | "cluster_average_success_rate" => Ok(GeneratedField::ClusterAverageSuccessRate),
                            "clusterSuccessRateEjectionThreshold" | "cluster_success_rate_ejection_threshold" => Ok(GeneratedField::ClusterSuccessRateEjectionThreshold),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutlierEjectSuccessRate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.cluster.v2alpha.OutlierEjectSuccessRate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OutlierEjectSuccessRate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut host_success_rate__ = None;
                let mut cluster_average_success_rate__ = None;
                let mut cluster_success_rate_ejection_threshold__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostSuccessRate => {
                            if host_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostSuccessRate"));
                            }
                            host_success_rate__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClusterAverageSuccessRate => {
                            if cluster_average_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterAverageSuccessRate"));
                            }
                            cluster_average_success_rate__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClusterSuccessRateEjectionThreshold => {
                            if cluster_success_rate_ejection_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterSuccessRateEjectionThreshold"));
                            }
                            cluster_success_rate_ejection_threshold__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(OutlierEjectSuccessRate {
                    host_success_rate: host_success_rate__.unwrap_or_default(),
                    cluster_average_success_rate: cluster_average_success_rate__.unwrap_or_default(),
                    cluster_success_rate_ejection_threshold: cluster_success_rate_ejection_threshold__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.cluster.v2alpha.OutlierEjectSuccessRate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutlierEjectionType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Consecutive5xx => "CONSECUTIVE_5XX",
            Self::ConsecutiveGatewayFailure => "CONSECUTIVE_GATEWAY_FAILURE",
            Self::SuccessRate => "SUCCESS_RATE",
            Self::ConsecutiveLocalOriginFailure => "CONSECUTIVE_LOCAL_ORIGIN_FAILURE",
            Self::SuccessRateLocalOrigin => "SUCCESS_RATE_LOCAL_ORIGIN",
            Self::FailurePercentage => "FAILURE_PERCENTAGE",
            Self::FailurePercentageLocalOrigin => "FAILURE_PERCENTAGE_LOCAL_ORIGIN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for OutlierEjectionType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONSECUTIVE_5XX",
            "CONSECUTIVE_GATEWAY_FAILURE",
            "SUCCESS_RATE",
            "CONSECUTIVE_LOCAL_ORIGIN_FAILURE",
            "SUCCESS_RATE_LOCAL_ORIGIN",
            "FAILURE_PERCENTAGE",
            "FAILURE_PERCENTAGE_LOCAL_ORIGIN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutlierEjectionType;

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
                    .and_then(OutlierEjectionType::from_i32)
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
                    .and_then(OutlierEjectionType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONSECUTIVE_5XX" => Ok(OutlierEjectionType::Consecutive5xx),
                    "CONSECUTIVE_GATEWAY_FAILURE" => Ok(OutlierEjectionType::ConsecutiveGatewayFailure),
                    "SUCCESS_RATE" => Ok(OutlierEjectionType::SuccessRate),
                    "CONSECUTIVE_LOCAL_ORIGIN_FAILURE" => Ok(OutlierEjectionType::ConsecutiveLocalOriginFailure),
                    "SUCCESS_RATE_LOCAL_ORIGIN" => Ok(OutlierEjectionType::SuccessRateLocalOrigin),
                    "FAILURE_PERCENTAGE" => Ok(OutlierEjectionType::FailurePercentage),
                    "FAILURE_PERCENTAGE_LOCAL_ORIGIN" => Ok(OutlierEjectionType::FailurePercentageLocalOrigin),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
