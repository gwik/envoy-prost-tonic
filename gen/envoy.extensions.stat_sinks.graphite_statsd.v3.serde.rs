// @generated
impl serde::Serialize for GraphiteStatsdSink {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prefix.is_empty() {
            len += 1;
        }
        if self.max_bytes_per_datagram.is_some() {
            len += 1;
        }
        if self.statsd_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.stat_sinks.graphite_statsd.v3.GraphiteStatsdSink", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        if let Some(v) = self.max_bytes_per_datagram.as_ref() {
            struct_ser.serialize_field("maxBytesPerDatagram", v)?;
        }
        if let Some(v) = self.statsd_specifier.as_ref() {
            match v {
                graphite_statsd_sink::StatsdSpecifier::Address(v) => {
                    struct_ser.serialize_field("address", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GraphiteStatsdSink {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prefix",
            "max_bytes_per_datagram",
            "maxBytesPerDatagram",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            MaxBytesPerDatagram,
            Address,
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
                            "prefix" => Ok(GeneratedField::Prefix),
                            "maxBytesPerDatagram" | "max_bytes_per_datagram" => Ok(GeneratedField::MaxBytesPerDatagram),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GraphiteStatsdSink;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.stat_sinks.graphite_statsd.v3.GraphiteStatsdSink")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GraphiteStatsdSink, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                let mut max_bytes_per_datagram__ = None;
                let mut statsd_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxBytesPerDatagram => {
                            if max_bytes_per_datagram__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBytesPerDatagram"));
                            }
                            max_bytes_per_datagram__ = map.next_value()?;
                        }
                        GeneratedField::Address => {
                            if statsd_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            statsd_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(graphite_statsd_sink::StatsdSpecifier::Address)
;
                        }
                    }
                }
                Ok(GraphiteStatsdSink {
                    prefix: prefix__.unwrap_or_default(),
                    max_bytes_per_datagram: max_bytes_per_datagram__,
                    statsd_specifier: statsd_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.stat_sinks.graphite_statsd.v3.GraphiteStatsdSink", FIELDS, GeneratedVisitor)
    }
}
