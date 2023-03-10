// @generated
impl serde::Serialize for DnsCacheConfig {
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
        if self.dns_lookup_family != 0 {
            len += 1;
        }
        if self.dns_refresh_rate.is_some() {
            len += 1;
        }
        if self.host_ttl.is_some() {
            len += 1;
        }
        if self.max_hosts.is_some() {
            len += 1;
        }
        if self.dns_failure_refresh_rate.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.common.dynamic_forward_proxy.v2alpha.DnsCacheConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.dns_lookup_family != 0 {
            let v = super::super::super::super::api::v2::cluster_::DnsLookupFamily::from_i32(self.dns_lookup_family)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.dns_lookup_family)))?;
            struct_ser.serialize_field("dnsLookupFamily", &v)?;
        }
        if let Some(v) = self.dns_refresh_rate.as_ref() {
            struct_ser.serialize_field("dnsRefreshRate", v)?;
        }
        if let Some(v) = self.host_ttl.as_ref() {
            struct_ser.serialize_field("hostTtl", v)?;
        }
        if let Some(v) = self.max_hosts.as_ref() {
            struct_ser.serialize_field("maxHosts", v)?;
        }
        if let Some(v) = self.dns_failure_refresh_rate.as_ref() {
            struct_ser.serialize_field("dnsFailureRefreshRate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DnsCacheConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "dns_lookup_family",
            "dnsLookupFamily",
            "dns_refresh_rate",
            "dnsRefreshRate",
            "host_ttl",
            "hostTtl",
            "max_hosts",
            "maxHosts",
            "dns_failure_refresh_rate",
            "dnsFailureRefreshRate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DnsLookupFamily,
            DnsRefreshRate,
            HostTtl,
            MaxHosts,
            DnsFailureRefreshRate,
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
                            "dnsLookupFamily" | "dns_lookup_family" => Ok(GeneratedField::DnsLookupFamily),
                            "dnsRefreshRate" | "dns_refresh_rate" => Ok(GeneratedField::DnsRefreshRate),
                            "hostTtl" | "host_ttl" => Ok(GeneratedField::HostTtl),
                            "maxHosts" | "max_hosts" => Ok(GeneratedField::MaxHosts),
                            "dnsFailureRefreshRate" | "dns_failure_refresh_rate" => Ok(GeneratedField::DnsFailureRefreshRate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DnsCacheConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.common.dynamic_forward_proxy.v2alpha.DnsCacheConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DnsCacheConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut dns_lookup_family__ = None;
                let mut dns_refresh_rate__ = None;
                let mut host_ttl__ = None;
                let mut max_hosts__ = None;
                let mut dns_failure_refresh_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsLookupFamily => {
                            if dns_lookup_family__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsLookupFamily"));
                            }
                            dns_lookup_family__ = Some(map.next_value::<super::super::super::super::api::v2::cluster_::DnsLookupFamily>()? as i32);
                        }
                        GeneratedField::DnsRefreshRate => {
                            if dns_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsRefreshRate"));
                            }
                            dns_refresh_rate__ = map.next_value()?;
                        }
                        GeneratedField::HostTtl => {
                            if host_ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostTtl"));
                            }
                            host_ttl__ = map.next_value()?;
                        }
                        GeneratedField::MaxHosts => {
                            if max_hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxHosts"));
                            }
                            max_hosts__ = map.next_value()?;
                        }
                        GeneratedField::DnsFailureRefreshRate => {
                            if dns_failure_refresh_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsFailureRefreshRate"));
                            }
                            dns_failure_refresh_rate__ = map.next_value()?;
                        }
                    }
                }
                Ok(DnsCacheConfig {
                    name: name__.unwrap_or_default(),
                    dns_lookup_family: dns_lookup_family__.unwrap_or_default(),
                    dns_refresh_rate: dns_refresh_rate__,
                    host_ttl: host_ttl__,
                    max_hosts: max_hosts__,
                    dns_failure_refresh_rate: dns_failure_refresh_rate__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.common.dynamic_forward_proxy.v2alpha.DnsCacheConfig", FIELDS, GeneratedVisitor)
    }
}
