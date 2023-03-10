// @generated
impl serde::Serialize for Thrift {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.method_name.is_empty() {
            len += 1;
        }
        if self.transport != 0 {
            len += 1;
        }
        if self.protocol != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.health_checkers.thrift.v3.Thrift", len)?;
        if !self.method_name.is_empty() {
            struct_ser.serialize_field("methodName", &self.method_name)?;
        }
        if self.transport != 0 {
            let v = super::super::super::filters::network::thrift_proxy::v3::TransportType::from_i32(self.transport)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.transport)))?;
            struct_ser.serialize_field("transport", &v)?;
        }
        if self.protocol != 0 {
            let v = super::super::super::filters::network::thrift_proxy::v3::ProtocolType::from_i32(self.protocol)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol)))?;
            struct_ser.serialize_field("protocol", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Thrift {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method_name",
            "methodName",
            "transport",
            "protocol",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MethodName,
            Transport,
            Protocol,
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
                            "methodName" | "method_name" => Ok(GeneratedField::MethodName),
                            "transport" => Ok(GeneratedField::Transport),
                            "protocol" => Ok(GeneratedField::Protocol),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Thrift;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.health_checkers.thrift.v3.Thrift")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Thrift, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut method_name__ = None;
                let mut transport__ = None;
                let mut protocol__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MethodName => {
                            if method_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("methodName"));
                            }
                            method_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Transport => {
                            if transport__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transport"));
                            }
                            transport__ = Some(map.next_value::<super::super::super::filters::network::thrift_proxy::v3::TransportType>()? as i32);
                        }
                        GeneratedField::Protocol => {
                            if protocol__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocol"));
                            }
                            protocol__ = Some(map.next_value::<super::super::super::filters::network::thrift_proxy::v3::ProtocolType>()? as i32);
                        }
                    }
                }
                Ok(Thrift {
                    method_name: method_name__.unwrap_or_default(),
                    transport: transport__.unwrap_or_default(),
                    protocol: protocol__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.health_checkers.thrift.v3.Thrift", FIELDS, GeneratedVisitor)
    }
}
