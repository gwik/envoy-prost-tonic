// @generated
impl serde::Serialize for RingHash {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.hash_function != 0 {
            len += 1;
        }
        if self.minimum_ring_size.is_some() {
            len += 1;
        }
        if self.maximum_ring_size.is_some() {
            len += 1;
        }
        if self.use_hostname_for_hashing {
            len += 1;
        }
        if self.hash_balance_factor.is_some() {
            len += 1;
        }
        if self.consistent_hashing_lb_config.is_some() {
            len += 1;
        }
        if self.locality_weighted_lb_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.load_balancing_policies.ring_hash.v3.RingHash", len)?;
        if self.hash_function != 0 {
            let v = ring_hash::HashFunction::from_i32(self.hash_function)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.hash_function)))?;
            struct_ser.serialize_field("hashFunction", &v)?;
        }
        if let Some(v) = self.minimum_ring_size.as_ref() {
            struct_ser.serialize_field("minimumRingSize", v)?;
        }
        if let Some(v) = self.maximum_ring_size.as_ref() {
            struct_ser.serialize_field("maximumRingSize", v)?;
        }
        if self.use_hostname_for_hashing {
            struct_ser.serialize_field("useHostnameForHashing", &self.use_hostname_for_hashing)?;
        }
        if let Some(v) = self.hash_balance_factor.as_ref() {
            struct_ser.serialize_field("hashBalanceFactor", v)?;
        }
        if let Some(v) = self.consistent_hashing_lb_config.as_ref() {
            struct_ser.serialize_field("consistentHashingLbConfig", v)?;
        }
        if let Some(v) = self.locality_weighted_lb_config.as_ref() {
            struct_ser.serialize_field("localityWeightedLbConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RingHash {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash_function",
            "hashFunction",
            "minimum_ring_size",
            "minimumRingSize",
            "maximum_ring_size",
            "maximumRingSize",
            "use_hostname_for_hashing",
            "useHostnameForHashing",
            "hash_balance_factor",
            "hashBalanceFactor",
            "consistent_hashing_lb_config",
            "consistentHashingLbConfig",
            "locality_weighted_lb_config",
            "localityWeightedLbConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HashFunction,
            MinimumRingSize,
            MaximumRingSize,
            UseHostnameForHashing,
            HashBalanceFactor,
            ConsistentHashingLbConfig,
            LocalityWeightedLbConfig,
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
                            "hashFunction" | "hash_function" => Ok(GeneratedField::HashFunction),
                            "minimumRingSize" | "minimum_ring_size" => Ok(GeneratedField::MinimumRingSize),
                            "maximumRingSize" | "maximum_ring_size" => Ok(GeneratedField::MaximumRingSize),
                            "useHostnameForHashing" | "use_hostname_for_hashing" => Ok(GeneratedField::UseHostnameForHashing),
                            "hashBalanceFactor" | "hash_balance_factor" => Ok(GeneratedField::HashBalanceFactor),
                            "consistentHashingLbConfig" | "consistent_hashing_lb_config" => Ok(GeneratedField::ConsistentHashingLbConfig),
                            "localityWeightedLbConfig" | "locality_weighted_lb_config" => Ok(GeneratedField::LocalityWeightedLbConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RingHash;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.load_balancing_policies.ring_hash.v3.RingHash")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RingHash, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash_function__ = None;
                let mut minimum_ring_size__ = None;
                let mut maximum_ring_size__ = None;
                let mut use_hostname_for_hashing__ = None;
                let mut hash_balance_factor__ = None;
                let mut consistent_hashing_lb_config__ = None;
                let mut locality_weighted_lb_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HashFunction => {
                            if hash_function__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashFunction"));
                            }
                            hash_function__ = Some(map.next_value::<ring_hash::HashFunction>()? as i32);
                        }
                        GeneratedField::MinimumRingSize => {
                            if minimum_ring_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimumRingSize"));
                            }
                            minimum_ring_size__ = map.next_value()?;
                        }
                        GeneratedField::MaximumRingSize => {
                            if maximum_ring_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximumRingSize"));
                            }
                            maximum_ring_size__ = map.next_value()?;
                        }
                        GeneratedField::UseHostnameForHashing => {
                            if use_hostname_for_hashing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useHostnameForHashing"));
                            }
                            use_hostname_for_hashing__ = Some(map.next_value()?);
                        }
                        GeneratedField::HashBalanceFactor => {
                            if hash_balance_factor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashBalanceFactor"));
                            }
                            hash_balance_factor__ = map.next_value()?;
                        }
                        GeneratedField::ConsistentHashingLbConfig => {
                            if consistent_hashing_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consistentHashingLbConfig"));
                            }
                            consistent_hashing_lb_config__ = map.next_value()?;
                        }
                        GeneratedField::LocalityWeightedLbConfig => {
                            if locality_weighted_lb_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localityWeightedLbConfig"));
                            }
                            locality_weighted_lb_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(RingHash {
                    hash_function: hash_function__.unwrap_or_default(),
                    minimum_ring_size: minimum_ring_size__,
                    maximum_ring_size: maximum_ring_size__,
                    use_hostname_for_hashing: use_hostname_for_hashing__.unwrap_or_default(),
                    hash_balance_factor: hash_balance_factor__,
                    consistent_hashing_lb_config: consistent_hashing_lb_config__,
                    locality_weighted_lb_config: locality_weighted_lb_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.load_balancing_policies.ring_hash.v3.RingHash", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ring_hash::HashFunction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DefaultHash => "DEFAULT_HASH",
            Self::XxHash => "XX_HASH",
            Self::MurmurHash2 => "MURMUR_HASH_2",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ring_hash::HashFunction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT_HASH",
            "XX_HASH",
            "MURMUR_HASH_2",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ring_hash::HashFunction;

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
                    .and_then(ring_hash::HashFunction::from_i32)
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
                    .and_then(ring_hash::HashFunction::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT_HASH" => Ok(ring_hash::HashFunction::DefaultHash),
                    "XX_HASH" => Ok(ring_hash::HashFunction::XxHash),
                    "MURMUR_HASH_2" => Ok(ring_hash::HashFunction::MurmurHash2),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
