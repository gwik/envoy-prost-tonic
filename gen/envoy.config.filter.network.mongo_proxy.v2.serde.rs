// @generated
impl serde::Serialize for MongoProxy {
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
        if self.delay.is_some() {
            len += 1;
        }
        if self.emit_dynamic_metadata {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.mongo_proxy.v2.MongoProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("accessLog", &self.access_log)?;
        }
        if let Some(v) = self.delay.as_ref() {
            struct_ser.serialize_field("delay", v)?;
        }
        if self.emit_dynamic_metadata {
            struct_ser.serialize_field("emitDynamicMetadata", &self.emit_dynamic_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MongoProxy {
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
            "delay",
            "emit_dynamic_metadata",
            "emitDynamicMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            AccessLog,
            Delay,
            EmitDynamicMetadata,
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
                            "delay" => Ok(GeneratedField::Delay),
                            "emitDynamicMetadata" | "emit_dynamic_metadata" => Ok(GeneratedField::EmitDynamicMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MongoProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.mongo_proxy.v2.MongoProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MongoProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut access_log__ = None;
                let mut delay__ = None;
                let mut emit_dynamic_metadata__ = None;
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
                        GeneratedField::Delay => {
                            if delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delay"));
                            }
                            delay__ = map.next_value()?;
                        }
                        GeneratedField::EmitDynamicMetadata => {
                            if emit_dynamic_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emitDynamicMetadata"));
                            }
                            emit_dynamic_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MongoProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    access_log: access_log__.unwrap_or_default(),
                    delay: delay__,
                    emit_dynamic_metadata: emit_dynamic_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.mongo_proxy.v2.MongoProxy", FIELDS, GeneratedVisitor)
    }
}
