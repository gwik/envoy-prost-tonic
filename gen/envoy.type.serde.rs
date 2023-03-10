// @generated
impl serde::Serialize for CodecClientType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Http1 => "HTTP1",
            Self::Http2 => "HTTP2",
            Self::Http3 => "HTTP3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for CodecClientType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HTTP1",
            "HTTP2",
            "HTTP3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CodecClientType;

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
                    .and_then(CodecClientType::from_i32)
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
                    .and_then(CodecClientType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HTTP1" => Ok(CodecClientType::Http1),
                    "HTTP2" => Ok(CodecClientType::Http2),
                    "HTTP3" => Ok(CodecClientType::Http3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DoubleRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0. {
            len += 1;
        }
        if self.end != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.DoubleRange", len)?;
        if self.start != 0. {
            struct_ser.serialize_field("start", &self.start)?;
        }
        if self.end != 0. {
            struct_ser.serialize_field("end", &self.end)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DoubleRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DoubleRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.DoubleRange")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DoubleRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DoubleRange {
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.DoubleRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FractionalPercent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.numerator != 0 {
            len += 1;
        }
        if self.denominator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.FractionalPercent", len)?;
        if self.numerator != 0 {
            struct_ser.serialize_field("numerator", &self.numerator)?;
        }
        if self.denominator != 0 {
            let v = fractional_percent::DenominatorType::from_i32(self.denominator)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.denominator)))?;
            struct_ser.serialize_field("denominator", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FractionalPercent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "numerator",
            "denominator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Numerator,
            Denominator,
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
                            "numerator" => Ok(GeneratedField::Numerator),
                            "denominator" => Ok(GeneratedField::Denominator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FractionalPercent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.FractionalPercent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FractionalPercent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut numerator__ = None;
                let mut denominator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Numerator => {
                            if numerator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numerator"));
                            }
                            numerator__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Denominator => {
                            if denominator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denominator"));
                            }
                            denominator__ = Some(map.next_value::<fractional_percent::DenominatorType>()? as i32);
                        }
                    }
                }
                Ok(FractionalPercent {
                    numerator: numerator__.unwrap_or_default(),
                    denominator: denominator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.FractionalPercent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for fractional_percent::DenominatorType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Hundred => "HUNDRED",
            Self::TenThousand => "TEN_THOUSAND",
            Self::Million => "MILLION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for fractional_percent::DenominatorType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "HUNDRED",
            "TEN_THOUSAND",
            "MILLION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = fractional_percent::DenominatorType;

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
                    .and_then(fractional_percent::DenominatorType::from_i32)
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
                    .and_then(fractional_percent::DenominatorType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "HUNDRED" => Ok(fractional_percent::DenominatorType::Hundred),
                    "TEN_THOUSAND" => Ok(fractional_percent::DenominatorType::TenThousand),
                    "MILLION" => Ok(fractional_percent::DenominatorType::Million),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HashPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.policy_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.HashPolicy", len)?;
        if let Some(v) = self.policy_specifier.as_ref() {
            match v {
                hash_policy::PolicySpecifier::SourceIp(v) => {
                    struct_ser.serialize_field("sourceIp", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HashPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "source_ip",
            "sourceIp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SourceIp,
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
                            "sourceIp" | "source_ip" => Ok(GeneratedField::SourceIp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HashPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.HashPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HashPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut policy_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SourceIp => {
                            if policy_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceIp"));
                            }
                            policy_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(hash_policy::PolicySpecifier::SourceIp)
;
                        }
                    }
                }
                Ok(HashPolicy {
                    policy_specifier: policy_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.HashPolicy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for hash_policy::SourceIp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.r#type.HashPolicy.SourceIp", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for hash_policy::SourceIp {
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
            type Value = hash_policy::SourceIp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.HashPolicy.SourceIp")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<hash_policy::SourceIp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(hash_policy::SourceIp {
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.HashPolicy.SourceIp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.HttpStatus", len)?;
        if self.code != 0 {
            let v = StatusCode::from_i32(self.code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.HttpStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map.next_value::<StatusCode>()? as i32);
                        }
                    }
                }
                Ok(HttpStatus {
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.HttpStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Int32Range {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.end != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.Int32Range", len)?;
        if self.start != 0 {
            struct_ser.serialize_field("start", &self.start)?;
        }
        if self.end != 0 {
            struct_ser.serialize_field("end", &self.end)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Int32Range {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Int32Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.Int32Range")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Int32Range, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Int32Range {
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.Int32Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Int64Range {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.start != 0 {
            len += 1;
        }
        if self.end != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.Int64Range", len)?;
        if self.start != 0 {
            struct_ser.serialize_field("start", ToString::to_string(&self.start).as_str())?;
        }
        if self.end != 0 {
            struct_ser.serialize_field("end", ToString::to_string(&self.end).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Int64Range {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "start",
            "end",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Start,
            End,
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
                            "start" => Ok(GeneratedField::Start),
                            "end" => Ok(GeneratedField::End),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Int64Range;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.Int64Range")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Int64Range, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut start__ = None;
                let mut end__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Start => {
                            if start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            start__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::End => {
                            if end__.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            end__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Int64Range {
                    start: start__.unwrap_or_default(),
                    end: end__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.Int64Range", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Percent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.Percent", len)?;
        if self.value != 0. {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Percent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = Percent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.Percent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Percent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Percent {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.Percent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SemanticVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.major_number != 0 {
            len += 1;
        }
        if self.minor_number != 0 {
            len += 1;
        }
        if self.patch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.SemanticVersion", len)?;
        if self.major_number != 0 {
            struct_ser.serialize_field("majorNumber", &self.major_number)?;
        }
        if self.minor_number != 0 {
            struct_ser.serialize_field("minorNumber", &self.minor_number)?;
        }
        if self.patch != 0 {
            struct_ser.serialize_field("patch", &self.patch)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SemanticVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "major_number",
            "majorNumber",
            "minor_number",
            "minorNumber",
            "patch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MajorNumber,
            MinorNumber,
            Patch,
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
                            "majorNumber" | "major_number" => Ok(GeneratedField::MajorNumber),
                            "minorNumber" | "minor_number" => Ok(GeneratedField::MinorNumber),
                            "patch" => Ok(GeneratedField::Patch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SemanticVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.SemanticVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SemanticVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut major_number__ = None;
                let mut minor_number__ = None;
                let mut patch__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MajorNumber => {
                            if major_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("majorNumber"));
                            }
                            major_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinorNumber => {
                            if minor_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minorNumber"));
                            }
                            minor_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Patch => {
                            if patch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("patch"));
                            }
                            patch__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SemanticVersion {
                    major_number: major_number__.unwrap_or_default(),
                    minor_number: minor_number__.unwrap_or_default(),
                    patch: patch__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.SemanticVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StatusCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Empty => "Empty",
            Self::Continue => "Continue",
            Self::Ok => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NonAuthoritativeInformation => "NonAuthoritativeInformation",
            Self::NoContent => "NoContent",
            Self::ResetContent => "ResetContent",
            Self::PartialContent => "PartialContent",
            Self::MultiStatus => "MultiStatus",
            Self::AlreadyReported => "AlreadyReported",
            Self::ImUsed => "IMUsed",
            Self::MultipleChoices => "MultipleChoices",
            Self::MovedPermanently => "MovedPermanently",
            Self::Found => "Found",
            Self::SeeOther => "SeeOther",
            Self::NotModified => "NotModified",
            Self::UseProxy => "UseProxy",
            Self::TemporaryRedirect => "TemporaryRedirect",
            Self::PermanentRedirect => "PermanentRedirect",
            Self::BadRequest => "BadRequest",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "PaymentRequired",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "NotFound",
            Self::MethodNotAllowed => "MethodNotAllowed",
            Self::NotAcceptable => "NotAcceptable",
            Self::ProxyAuthenticationRequired => "ProxyAuthenticationRequired",
            Self::RequestTimeout => "RequestTimeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "LengthRequired",
            Self::PreconditionFailed => "PreconditionFailed",
            Self::PayloadTooLarge => "PayloadTooLarge",
            Self::UriTooLong => "URITooLong",
            Self::UnsupportedMediaType => "UnsupportedMediaType",
            Self::RangeNotSatisfiable => "RangeNotSatisfiable",
            Self::ExpectationFailed => "ExpectationFailed",
            Self::MisdirectedRequest => "MisdirectedRequest",
            Self::UnprocessableEntity => "UnprocessableEntity",
            Self::Locked => "Locked",
            Self::FailedDependency => "FailedDependency",
            Self::UpgradeRequired => "UpgradeRequired",
            Self::PreconditionRequired => "PreconditionRequired",
            Self::TooManyRequests => "TooManyRequests",
            Self::RequestHeaderFieldsTooLarge => "RequestHeaderFieldsTooLarge",
            Self::InternalServerError => "InternalServerError",
            Self::NotImplemented => "NotImplemented",
            Self::BadGateway => "BadGateway",
            Self::ServiceUnavailable => "ServiceUnavailable",
            Self::GatewayTimeout => "GatewayTimeout",
            Self::HttpVersionNotSupported => "HTTPVersionNotSupported",
            Self::VariantAlsoNegotiates => "VariantAlsoNegotiates",
            Self::InsufficientStorage => "InsufficientStorage",
            Self::LoopDetected => "LoopDetected",
            Self::NotExtended => "NotExtended",
            Self::NetworkAuthenticationRequired => "NetworkAuthenticationRequired",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StatusCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Empty",
            "Continue",
            "OK",
            "Created",
            "Accepted",
            "NonAuthoritativeInformation",
            "NoContent",
            "ResetContent",
            "PartialContent",
            "MultiStatus",
            "AlreadyReported",
            "IMUsed",
            "MultipleChoices",
            "MovedPermanently",
            "Found",
            "SeeOther",
            "NotModified",
            "UseProxy",
            "TemporaryRedirect",
            "PermanentRedirect",
            "BadRequest",
            "Unauthorized",
            "PaymentRequired",
            "Forbidden",
            "NotFound",
            "MethodNotAllowed",
            "NotAcceptable",
            "ProxyAuthenticationRequired",
            "RequestTimeout",
            "Conflict",
            "Gone",
            "LengthRequired",
            "PreconditionFailed",
            "PayloadTooLarge",
            "URITooLong",
            "UnsupportedMediaType",
            "RangeNotSatisfiable",
            "ExpectationFailed",
            "MisdirectedRequest",
            "UnprocessableEntity",
            "Locked",
            "FailedDependency",
            "UpgradeRequired",
            "PreconditionRequired",
            "TooManyRequests",
            "RequestHeaderFieldsTooLarge",
            "InternalServerError",
            "NotImplemented",
            "BadGateway",
            "ServiceUnavailable",
            "GatewayTimeout",
            "HTTPVersionNotSupported",
            "VariantAlsoNegotiates",
            "InsufficientStorage",
            "LoopDetected",
            "NotExtended",
            "NetworkAuthenticationRequired",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StatusCode;

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
                    .and_then(StatusCode::from_i32)
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
                    .and_then(StatusCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Empty" => Ok(StatusCode::Empty),
                    "Continue" => Ok(StatusCode::Continue),
                    "OK" => Ok(StatusCode::Ok),
                    "Created" => Ok(StatusCode::Created),
                    "Accepted" => Ok(StatusCode::Accepted),
                    "NonAuthoritativeInformation" => Ok(StatusCode::NonAuthoritativeInformation),
                    "NoContent" => Ok(StatusCode::NoContent),
                    "ResetContent" => Ok(StatusCode::ResetContent),
                    "PartialContent" => Ok(StatusCode::PartialContent),
                    "MultiStatus" => Ok(StatusCode::MultiStatus),
                    "AlreadyReported" => Ok(StatusCode::AlreadyReported),
                    "IMUsed" => Ok(StatusCode::ImUsed),
                    "MultipleChoices" => Ok(StatusCode::MultipleChoices),
                    "MovedPermanently" => Ok(StatusCode::MovedPermanently),
                    "Found" => Ok(StatusCode::Found),
                    "SeeOther" => Ok(StatusCode::SeeOther),
                    "NotModified" => Ok(StatusCode::NotModified),
                    "UseProxy" => Ok(StatusCode::UseProxy),
                    "TemporaryRedirect" => Ok(StatusCode::TemporaryRedirect),
                    "PermanentRedirect" => Ok(StatusCode::PermanentRedirect),
                    "BadRequest" => Ok(StatusCode::BadRequest),
                    "Unauthorized" => Ok(StatusCode::Unauthorized),
                    "PaymentRequired" => Ok(StatusCode::PaymentRequired),
                    "Forbidden" => Ok(StatusCode::Forbidden),
                    "NotFound" => Ok(StatusCode::NotFound),
                    "MethodNotAllowed" => Ok(StatusCode::MethodNotAllowed),
                    "NotAcceptable" => Ok(StatusCode::NotAcceptable),
                    "ProxyAuthenticationRequired" => Ok(StatusCode::ProxyAuthenticationRequired),
                    "RequestTimeout" => Ok(StatusCode::RequestTimeout),
                    "Conflict" => Ok(StatusCode::Conflict),
                    "Gone" => Ok(StatusCode::Gone),
                    "LengthRequired" => Ok(StatusCode::LengthRequired),
                    "PreconditionFailed" => Ok(StatusCode::PreconditionFailed),
                    "PayloadTooLarge" => Ok(StatusCode::PayloadTooLarge),
                    "URITooLong" => Ok(StatusCode::UriTooLong),
                    "UnsupportedMediaType" => Ok(StatusCode::UnsupportedMediaType),
                    "RangeNotSatisfiable" => Ok(StatusCode::RangeNotSatisfiable),
                    "ExpectationFailed" => Ok(StatusCode::ExpectationFailed),
                    "MisdirectedRequest" => Ok(StatusCode::MisdirectedRequest),
                    "UnprocessableEntity" => Ok(StatusCode::UnprocessableEntity),
                    "Locked" => Ok(StatusCode::Locked),
                    "FailedDependency" => Ok(StatusCode::FailedDependency),
                    "UpgradeRequired" => Ok(StatusCode::UpgradeRequired),
                    "PreconditionRequired" => Ok(StatusCode::PreconditionRequired),
                    "TooManyRequests" => Ok(StatusCode::TooManyRequests),
                    "RequestHeaderFieldsTooLarge" => Ok(StatusCode::RequestHeaderFieldsTooLarge),
                    "InternalServerError" => Ok(StatusCode::InternalServerError),
                    "NotImplemented" => Ok(StatusCode::NotImplemented),
                    "BadGateway" => Ok(StatusCode::BadGateway),
                    "ServiceUnavailable" => Ok(StatusCode::ServiceUnavailable),
                    "GatewayTimeout" => Ok(StatusCode::GatewayTimeout),
                    "HTTPVersionNotSupported" => Ok(StatusCode::HttpVersionNotSupported),
                    "VariantAlsoNegotiates" => Ok(StatusCode::VariantAlsoNegotiates),
                    "InsufficientStorage" => Ok(StatusCode::InsufficientStorage),
                    "LoopDetected" => Ok(StatusCode::LoopDetected),
                    "NotExtended" => Ok(StatusCode::NotExtended),
                    "NetworkAuthenticationRequired" => Ok(StatusCode::NetworkAuthenticationRequired),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TokenBucket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_tokens != 0 {
            len += 1;
        }
        if self.tokens_per_fill.is_some() {
            len += 1;
        }
        if self.fill_interval.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.r#type.TokenBucket", len)?;
        if self.max_tokens != 0 {
            struct_ser.serialize_field("maxTokens", &self.max_tokens)?;
        }
        if let Some(v) = self.tokens_per_fill.as_ref() {
            struct_ser.serialize_field("tokensPerFill", v)?;
        }
        if let Some(v) = self.fill_interval.as_ref() {
            struct_ser.serialize_field("fillInterval", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenBucket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_tokens",
            "maxTokens",
            "tokens_per_fill",
            "tokensPerFill",
            "fill_interval",
            "fillInterval",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxTokens,
            TokensPerFill,
            FillInterval,
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
                            "tokensPerFill" | "tokens_per_fill" => Ok(GeneratedField::TokensPerFill),
                            "fillInterval" | "fill_interval" => Ok(GeneratedField::FillInterval),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenBucket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.r#type.TokenBucket")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TokenBucket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_tokens__ = None;
                let mut tokens_per_fill__ = None;
                let mut fill_interval__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxTokens => {
                            if max_tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxTokens"));
                            }
                            max_tokens__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TokensPerFill => {
                            if tokens_per_fill__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokensPerFill"));
                            }
                            tokens_per_fill__ = map.next_value()?;
                        }
                        GeneratedField::FillInterval => {
                            if fill_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fillInterval"));
                            }
                            fill_interval__ = map.next_value()?;
                        }
                    }
                }
                Ok(TokenBucket {
                    max_tokens: max_tokens__.unwrap_or_default(),
                    tokens_per_fill: tokens_per_fill__,
                    fill_interval: fill_interval__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.r#type.TokenBucket", FIELDS, GeneratedVisitor)
    }
}
