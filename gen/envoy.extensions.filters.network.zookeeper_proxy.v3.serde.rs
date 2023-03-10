// @generated
impl serde::Serialize for ZooKeeperProxy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        if self.max_packet_bytes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.zookeeper_proxy.v3.ZooKeeperProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("accessLog", &self.access_log)?;
        }
        if let Some(v) = self.max_packet_bytes.as_ref() {
            struct_ser.serialize_field("maxPacketBytes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ZooKeeperProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "access_log",
            "accessLog",
            "max_packet_bytes",
            "maxPacketBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            AccessLog,
            MaxPacketBytes,
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
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "maxPacketBytes" | "max_packet_bytes" => Ok(GeneratedField::MaxPacketBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ZooKeeperProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.zookeeper_proxy.v3.ZooKeeperProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ZooKeeperProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut access_log__ = None;
                let mut max_packet_bytes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxPacketBytes => {
                            if max_packet_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPacketBytes"));
                            }
                            max_packet_bytes__ = map.next_value()?;
                        }
                    }
                }
                Ok(ZooKeeperProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    access_log: access_log__.unwrap_or_default(),
                    max_packet_bytes: max_packet_bytes__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.zookeeper_proxy.v3.ZooKeeperProxy", FIELDS, GeneratedVisitor)
    }
}
