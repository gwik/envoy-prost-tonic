// @generated
impl serde::Serialize for ExtAuthz {
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
        if self.grpc_service.is_some() {
            len += 1;
        }
        if self.failure_mode_allow {
            len += 1;
        }
        if self.include_peer_certificate {
            len += 1;
        }
        if self.transport_api_version != 0 {
            len += 1;
        }
        if self.filter_enabled_metadata.is_some() {
            len += 1;
        }
        if !self.bootstrap_metadata_labels_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.ext_authz.v3.ExtAuthz", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        if self.failure_mode_allow {
            struct_ser.serialize_field("failureModeAllow", &self.failure_mode_allow)?;
        }
        if self.include_peer_certificate {
            struct_ser.serialize_field("includePeerCertificate", &self.include_peer_certificate)?;
        }
        if self.transport_api_version != 0 {
            let v = super::super::super::super::super::config::core::v3::ApiVersion::from_i32(self.transport_api_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.transport_api_version)))?;
            struct_ser.serialize_field("transportApiVersion", &v)?;
        }
        if let Some(v) = self.filter_enabled_metadata.as_ref() {
            struct_ser.serialize_field("filterEnabledMetadata", v)?;
        }
        if !self.bootstrap_metadata_labels_key.is_empty() {
            struct_ser.serialize_field("bootstrapMetadataLabelsKey", &self.bootstrap_metadata_labels_key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtAuthz {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "grpc_service",
            "grpcService",
            "failure_mode_allow",
            "failureModeAllow",
            "include_peer_certificate",
            "includePeerCertificate",
            "transport_api_version",
            "transportApiVersion",
            "filter_enabled_metadata",
            "filterEnabledMetadata",
            "bootstrap_metadata_labels_key",
            "bootstrapMetadataLabelsKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            GrpcService,
            FailureModeAllow,
            IncludePeerCertificate,
            TransportApiVersion,
            FilterEnabledMetadata,
            BootstrapMetadataLabelsKey,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "failureModeAllow" | "failure_mode_allow" => Ok(GeneratedField::FailureModeAllow),
                            "includePeerCertificate" | "include_peer_certificate" => Ok(GeneratedField::IncludePeerCertificate),
                            "transportApiVersion" | "transport_api_version" => Ok(GeneratedField::TransportApiVersion),
                            "filterEnabledMetadata" | "filter_enabled_metadata" => Ok(GeneratedField::FilterEnabledMetadata),
                            "bootstrapMetadataLabelsKey" | "bootstrap_metadata_labels_key" => Ok(GeneratedField::BootstrapMetadataLabelsKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtAuthz;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.ext_authz.v3.ExtAuthz")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtAuthz, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut grpc_service__ = None;
                let mut failure_mode_allow__ = None;
                let mut include_peer_certificate__ = None;
                let mut transport_api_version__ = None;
                let mut filter_enabled_metadata__ = None;
                let mut bootstrap_metadata_labels_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                        GeneratedField::FailureModeAllow => {
                            if failure_mode_allow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureModeAllow"));
                            }
                            failure_mode_allow__ = Some(map.next_value()?);
                        }
                        GeneratedField::IncludePeerCertificate => {
                            if include_peer_certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includePeerCertificate"));
                            }
                            include_peer_certificate__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransportApiVersion => {
                            if transport_api_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transportApiVersion"));
                            }
                            transport_api_version__ = Some(map.next_value::<super::super::super::super::super::config::core::v3::ApiVersion>()? as i32);
                        }
                        GeneratedField::FilterEnabledMetadata => {
                            if filter_enabled_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabledMetadata"));
                            }
                            filter_enabled_metadata__ = map.next_value()?;
                        }
                        GeneratedField::BootstrapMetadataLabelsKey => {
                            if bootstrap_metadata_labels_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootstrapMetadataLabelsKey"));
                            }
                            bootstrap_metadata_labels_key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExtAuthz {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    grpc_service: grpc_service__,
                    failure_mode_allow: failure_mode_allow__.unwrap_or_default(),
                    include_peer_certificate: include_peer_certificate__.unwrap_or_default(),
                    transport_api_version: transport_api_version__.unwrap_or_default(),
                    filter_enabled_metadata: filter_enabled_metadata__,
                    bootstrap_metadata_labels_key: bootstrap_metadata_labels_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.ext_authz.v3.ExtAuthz", FIELDS, GeneratedVisitor)
    }
}
