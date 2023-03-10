// @generated
impl serde::Serialize for Config {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.upgrade_protobuf_to_grpc {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_http1_bridge.v3.Config", len)?;
        if self.upgrade_protobuf_to_grpc {
            struct_ser.serialize_field("upgradeProtobufToGrpc", &self.upgrade_protobuf_to_grpc)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Config {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upgrade_protobuf_to_grpc",
            "upgradeProtobufToGrpc",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpgradeProtobufToGrpc,
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
                            "upgradeProtobufToGrpc" | "upgrade_protobuf_to_grpc" => Ok(GeneratedField::UpgradeProtobufToGrpc),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_http1_bridge.v3.Config")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Config, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upgrade_protobuf_to_grpc__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UpgradeProtobufToGrpc => {
                            if upgrade_protobuf_to_grpc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeProtobufToGrpc"));
                            }
                            upgrade_protobuf_to_grpc__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Config {
                    upgrade_protobuf_to_grpc: upgrade_protobuf_to_grpc__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_http1_bridge.v3.Config", FIELDS, GeneratedVisitor)
    }
}
