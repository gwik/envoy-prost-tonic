// @generated
impl serde::Serialize for ConnectionLimit {
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
        if self.max_connections.is_some() {
            len += 1;
        }
        if self.delay.is_some() {
            len += 1;
        }
        if self.runtime_enabled.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.connection_limit.v3.ConnectionLimit", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.max_connections.as_ref() {
            struct_ser.serialize_field("maxConnections", v)?;
        }
        if let Some(v) = self.delay.as_ref() {
            struct_ser.serialize_field("delay", v)?;
        }
        if let Some(v) = self.runtime_enabled.as_ref() {
            struct_ser.serialize_field("runtimeEnabled", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "max_connections",
            "maxConnections",
            "delay",
            "runtime_enabled",
            "runtimeEnabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            MaxConnections,
            Delay,
            RuntimeEnabled,
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
                            "maxConnections" | "max_connections" => Ok(GeneratedField::MaxConnections),
                            "delay" => Ok(GeneratedField::Delay),
                            "runtimeEnabled" | "runtime_enabled" => Ok(GeneratedField::RuntimeEnabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConnectionLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.connection_limit.v3.ConnectionLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConnectionLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut max_connections__ = None;
                let mut delay__ = None;
                let mut runtime_enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxConnections => {
                            if max_connections__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxConnections"));
                            }
                            max_connections__ = map.next_value()?;
                        }
                        GeneratedField::Delay => {
                            if delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            delay__ = map.next_value()?;
                        }
                        GeneratedField::RuntimeEnabled => {
                            if runtime_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeEnabled"));
                            }
                            runtime_enabled__ = map.next_value()?;
                        }
                    }
                }
                Ok(ConnectionLimit {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    max_connections: max_connections__,
                    delay: delay__,
                    runtime_enabled: runtime_enabled__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.connection_limit.v3.ConnectionLimit", FIELDS, GeneratedVisitor)
    }
}
