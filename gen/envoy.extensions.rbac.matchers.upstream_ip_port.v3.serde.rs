// @generated
impl serde::Serialize for UpstreamIpPortMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.upstream_ip.is_some() {
            len += 1;
        }
        if self.upstream_port_range.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.rbac.matchers.upstream_ip_port.v3.UpstreamIpPortMatcher", len)?;
        if let Some(v) = self.upstream_ip.as_ref() {
            struct_ser.serialize_field("upstreamIp", v)?;
        }
        if let Some(v) = self.upstream_port_range.as_ref() {
            struct_ser.serialize_field("upstreamPortRange", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamIpPortMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upstream_ip",
            "upstreamIp",
            "upstream_port_range",
            "upstreamPortRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpstreamIp,
            UpstreamPortRange,
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
                            "upstreamIp" | "upstream_ip" => Ok(GeneratedField::UpstreamIp),
                            "upstreamPortRange" | "upstream_port_range" => Ok(GeneratedField::UpstreamPortRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamIpPortMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.rbac.matchers.upstream_ip_port.v3.UpstreamIpPortMatcher")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamIpPortMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upstream_ip__ = None;
                let mut upstream_port_range__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UpstreamIp => {
                            if upstream_ip__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamIp"));
                            }
                            upstream_ip__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamPortRange => {
                            if upstream_port_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamPortRange"));
                            }
                            upstream_port_range__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpstreamIpPortMatcher {
                    upstream_ip: upstream_ip__,
                    upstream_port_range: upstream_port_range__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.rbac.matchers.upstream_ip_port.v3.UpstreamIpPortMatcher", FIELDS, GeneratedVisitor)
    }
}
