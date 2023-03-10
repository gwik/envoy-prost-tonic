// @generated
impl serde::Serialize for FixedServerPreferredAddressConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ipv4_type.is_some() {
            len += 1;
        }
        if self.ipv6_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.quic.server_preferred_address.v3.FixedServerPreferredAddressConfig", len)?;
        if let Some(v) = self.ipv4_type.as_ref() {
            match v {
                fixed_server_preferred_address_config::Ipv4Type::Ipv4Address(v) => {
                    struct_ser.serialize_field("ipv4Address", v)?;
                }
            }
        }
        if let Some(v) = self.ipv6_type.as_ref() {
            match v {
                fixed_server_preferred_address_config::Ipv6Type::Ipv6Address(v) => {
                    struct_ser.serialize_field("ipv6Address", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixedServerPreferredAddressConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ipv4_address",
            "ipv4Address",
            "ipv6_address",
            "ipv6Address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ipv4Address,
            Ipv6Address,
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
                            "ipv4Address" | "ipv4_address" => Ok(GeneratedField::Ipv4Address),
                            "ipv6Address" | "ipv6_address" => Ok(GeneratedField::Ipv6Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FixedServerPreferredAddressConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.quic.server_preferred_address.v3.FixedServerPreferredAddressConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FixedServerPreferredAddressConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ipv4_type__ = None;
                let mut ipv6_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ipv4Address => {
                            if ipv4_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv4Address"));
                            }
                            ipv4_type__ = map.next_value::<::std::option::Option<_>>()?.map(fixed_server_preferred_address_config::Ipv4Type::Ipv4Address);
                        }
                        GeneratedField::Ipv6Address => {
                            if ipv6_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipv6Address"));
                            }
                            ipv6_type__ = map.next_value::<::std::option::Option<_>>()?.map(fixed_server_preferred_address_config::Ipv6Type::Ipv6Address);
                        }
                    }
                }
                Ok(FixedServerPreferredAddressConfig {
                    ipv4_type: ipv4_type__,
                    ipv6_type: ipv6_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.quic.server_preferred_address.v3.FixedServerPreferredAddressConfig", FIELDS, GeneratedVisitor)
    }
}
