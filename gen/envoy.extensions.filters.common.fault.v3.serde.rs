// @generated
impl serde::Serialize for FaultDelay {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.percentage.is_some() {
            len += 1;
        }
        if self.fault_delay_secifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.fault.v3.FaultDelay", len)?;
        if let Some(v) = self.percentage.as_ref() {
            struct_ser.serialize_field("percentage", v)?;
        }
        if let Some(v) = self.fault_delay_secifier.as_ref() {
            match v {
                fault_delay::FaultDelaySecifier::FixedDelay(v) => {
                    struct_ser.serialize_field("fixedDelay", v)?;
                }
                fault_delay::FaultDelaySecifier::HeaderDelay(v) => {
                    struct_ser.serialize_field("headerDelay", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FaultDelay {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "percentage",
            "fixed_delay",
            "fixedDelay",
            "header_delay",
            "headerDelay",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Percentage,
            FixedDelay,
            HeaderDelay,
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
                            "percentage" => Ok(GeneratedField::Percentage),
                            "fixedDelay" | "fixed_delay" => Ok(GeneratedField::FixedDelay),
                            "headerDelay" | "header_delay" => Ok(GeneratedField::HeaderDelay),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FaultDelay;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.fault.v3.FaultDelay")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FaultDelay, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut percentage__ = None;
                let mut fault_delay_secifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Percentage => {
                            if percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percentage"));
                            }
                            percentage__ = map.next_value()?;
                        }
                        GeneratedField::FixedDelay => {
                            if fault_delay_secifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDelay"));
                            }
                            fault_delay_secifier__ = map.next_value::<::std::option::Option<_>>()?.map(fault_delay::FaultDelaySecifier::FixedDelay)
;
                        }
                        GeneratedField::HeaderDelay => {
                            if fault_delay_secifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerDelay"));
                            }
                            fault_delay_secifier__ = map.next_value::<::std::option::Option<_>>()?.map(fault_delay::FaultDelaySecifier::HeaderDelay)
;
                        }
                    }
                }
                Ok(FaultDelay {
                    percentage: percentage__,
                    fault_delay_secifier: fault_delay_secifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.fault.v3.FaultDelay", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for fault_delay::FaultDelayType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Fixed => "FIXED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for fault_delay::FaultDelayType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FIXED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = fault_delay::FaultDelayType;

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
                    .and_then(fault_delay::FaultDelayType::from_i32)
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
                    .and_then(fault_delay::FaultDelayType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FIXED" => Ok(fault_delay::FaultDelayType::Fixed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for fault_delay::HeaderDelay {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.fault.v3.FaultDelay.HeaderDelay", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for fault_delay::HeaderDelay {
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
            type Value = fault_delay::HeaderDelay;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.fault.v3.FaultDelay.HeaderDelay")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<fault_delay::HeaderDelay, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(fault_delay::HeaderDelay {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.fault.v3.FaultDelay.HeaderDelay", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FaultRateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.percentage.is_some() {
            len += 1;
        }
        if self.limit_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.fault.v3.FaultRateLimit", len)?;
        if let Some(v) = self.percentage.as_ref() {
            struct_ser.serialize_field("percentage", v)?;
        }
        if let Some(v) = self.limit_type.as_ref() {
            match v {
                fault_rate_limit::LimitType::FixedLimit(v) => {
                    struct_ser.serialize_field("fixedLimit", v)?;
                }
                fault_rate_limit::LimitType::HeaderLimit(v) => {
                    struct_ser.serialize_field("headerLimit", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FaultRateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "percentage",
            "fixed_limit",
            "fixedLimit",
            "header_limit",
            "headerLimit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Percentage,
            FixedLimit,
            HeaderLimit,
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
                            "percentage" => Ok(GeneratedField::Percentage),
                            "fixedLimit" | "fixed_limit" => Ok(GeneratedField::FixedLimit),
                            "headerLimit" | "header_limit" => Ok(GeneratedField::HeaderLimit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FaultRateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.fault.v3.FaultRateLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FaultRateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut percentage__ = None;
                let mut limit_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Percentage => {
                            if percentage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("percentage"));
                            }
                            percentage__ = map.next_value()?;
                        }
                        GeneratedField::FixedLimit => {
                            if limit_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedLimit"));
                            }
                            limit_type__ = map.next_value::<::std::option::Option<_>>()?.map(fault_rate_limit::LimitType::FixedLimit)
;
                        }
                        GeneratedField::HeaderLimit => {
                            if limit_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerLimit"));
                            }
                            limit_type__ = map.next_value::<::std::option::Option<_>>()?.map(fault_rate_limit::LimitType::HeaderLimit)
;
                        }
                    }
                }
                Ok(FaultRateLimit {
                    percentage: percentage__,
                    limit_type: limit_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.fault.v3.FaultRateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for fault_rate_limit::FixedLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.limit_kbps != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.fault.v3.FaultRateLimit.FixedLimit", len)?;
        if self.limit_kbps != 0 {
            struct_ser.serialize_field("limitKbps", ToString::to_string(&self.limit_kbps).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for fault_rate_limit::FixedLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "limit_kbps",
            "limitKbps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LimitKbps,
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
                            "limitKbps" | "limit_kbps" => Ok(GeneratedField::LimitKbps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = fault_rate_limit::FixedLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.fault.v3.FaultRateLimit.FixedLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<fault_rate_limit::FixedLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut limit_kbps__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LimitKbps => {
                            if limit_kbps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limitKbps"));
                            }
                            limit_kbps__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(fault_rate_limit::FixedLimit {
                    limit_kbps: limit_kbps__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.fault.v3.FaultRateLimit.FixedLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for fault_rate_limit::HeaderLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.common.fault.v3.FaultRateLimit.HeaderLimit", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for fault_rate_limit::HeaderLimit {
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
            type Value = fault_rate_limit::HeaderLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.common.fault.v3.FaultRateLimit.HeaderLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<fault_rate_limit::HeaderLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(fault_rate_limit::HeaderLimit {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.common.fault.v3.FaultRateLimit.HeaderLimit", FIELDS, GeneratedVisitor)
    }
}
