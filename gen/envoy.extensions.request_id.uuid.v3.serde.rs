// @generated
impl serde::Serialize for UuidRequestIdConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pack_trace_reason.is_some() {
            len += 1;
        }
        if self.use_request_id_for_trace_sampling.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.request_id.uuid.v3.UuidRequestIdConfig", len)?;
        if let Some(v) = self.pack_trace_reason.as_ref() {
            struct_ser.serialize_field("packTraceReason", v)?;
        }
        if let Some(v) = self.use_request_id_for_trace_sampling.as_ref() {
            struct_ser.serialize_field("useRequestIdForTraceSampling", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UuidRequestIdConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pack_trace_reason",
            "packTraceReason",
            "use_request_id_for_trace_sampling",
            "useRequestIdForTraceSampling",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PackTraceReason,
            UseRequestIdForTraceSampling,
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
                            "packTraceReason" | "pack_trace_reason" => Ok(GeneratedField::PackTraceReason),
                            "useRequestIdForTraceSampling" | "use_request_id_for_trace_sampling" => Ok(GeneratedField::UseRequestIdForTraceSampling),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UuidRequestIdConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.request_id.uuid.v3.UuidRequestIdConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UuidRequestIdConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pack_trace_reason__ = None;
                let mut use_request_id_for_trace_sampling__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PackTraceReason => {
                            if pack_trace_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packTraceReason"));
                            }
                            pack_trace_reason__ = map.next_value()?;
                        }
                        GeneratedField::UseRequestIdForTraceSampling => {
                            if use_request_id_for_trace_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useRequestIdForTraceSampling"));
                            }
                            use_request_id_for_trace_sampling__ = map.next_value()?;
                        }
                    }
                }
                Ok(UuidRequestIdConfig {
                    pack_trace_reason: pack_trace_reason__,
                    use_request_id_for_trace_sampling: use_request_id_for_trace_sampling__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.request_id.uuid.v3.UuidRequestIdConfig", FIELDS, GeneratedVisitor)
    }
}
