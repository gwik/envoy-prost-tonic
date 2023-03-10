// @generated
impl serde::Serialize for XRayConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.daemon_endpoint.is_some() {
            len += 1;
        }
        if !self.segment_name.is_empty() {
            len += 1;
        }
        if self.sampling_rule_manifest.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.trace.v2alpha.XRayConfig", len)?;
        if let Some(v) = self.daemon_endpoint.as_ref() {
            struct_ser.serialize_field("daemonEndpoint", v)?;
        }
        if !self.segment_name.is_empty() {
            struct_ser.serialize_field("segmentName", &self.segment_name)?;
        }
        if let Some(v) = self.sampling_rule_manifest.as_ref() {
            struct_ser.serialize_field("samplingRuleManifest", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for XRayConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "daemon_endpoint",
            "daemonEndpoint",
            "segment_name",
            "segmentName",
            "sampling_rule_manifest",
            "samplingRuleManifest",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DaemonEndpoint,
            SegmentName,
            SamplingRuleManifest,
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
                            "daemonEndpoint" | "daemon_endpoint" => Ok(GeneratedField::DaemonEndpoint),
                            "segmentName" | "segment_name" => Ok(GeneratedField::SegmentName),
                            "samplingRuleManifest" | "sampling_rule_manifest" => Ok(GeneratedField::SamplingRuleManifest),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = XRayConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.trace.v2alpha.XRayConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<XRayConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut daemon_endpoint__ = None;
                let mut segment_name__ = None;
                let mut sampling_rule_manifest__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DaemonEndpoint => {
                            if daemon_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daemonEndpoint"));
                            }
                            daemon_endpoint__ = map.next_value()?;
                        }
                        GeneratedField::SegmentName => {
                            if segment_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("segmentName"));
                            }
                            segment_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::SamplingRuleManifest => {
                            if sampling_rule_manifest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("samplingRuleManifest"));
                            }
                            sampling_rule_manifest__ = map.next_value()?;
                        }
                    }
                }
                Ok(XRayConfig {
                    daemon_endpoint: daemon_endpoint__,
                    segment_name: segment_name__.unwrap_or_default(),
                    sampling_rule_manifest: sampling_rule_manifest__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.trace.v2alpha.XRayConfig", FIELDS, GeneratedVisitor)
    }
}
