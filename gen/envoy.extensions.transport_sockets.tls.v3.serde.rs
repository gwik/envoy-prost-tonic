// @generated
impl serde::Serialize for CertificateProviderPluginInstance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if !self.certificate_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance", len)?;
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instanceName", &self.instance_name)?;
        }
        if !self.certificate_name.is_empty() {
            struct_ser.serialize_field("certificateName", &self.certificate_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CertificateProviderPluginInstance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_name",
            "instanceName",
            "certificate_name",
            "certificateName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceName,
            CertificateName,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "certificateName" | "certificate_name" => Ok(GeneratedField::CertificateName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CertificateProviderPluginInstance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CertificateProviderPluginInstance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_name__ = None;
                let mut certificate_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CertificateName => {
                            if certificate_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificateName"));
                            }
                            certificate_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CertificateProviderPluginInstance {
                    instance_name: instance_name__.unwrap_or_default(),
                    certificate_name: certificate_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateProviderPluginInstance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CertificateValidationContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.trusted_ca.is_some() {
            len += 1;
        }
        if self.ca_certificate_provider_instance.is_some() {
            len += 1;
        }
        if self.watched_directory.is_some() {
            len += 1;
        }
        if !self.verify_certificate_spki.is_empty() {
            len += 1;
        }
        if !self.verify_certificate_hash.is_empty() {
            len += 1;
        }
        if !self.match_typed_subject_alt_names.is_empty() {
            len += 1;
        }
        if !self.match_subject_alt_names.is_empty() {
            len += 1;
        }
        if self.require_signed_certificate_timestamp.is_some() {
            len += 1;
        }
        if self.crl.is_some() {
            len += 1;
        }
        if self.allow_expired_certificate {
            len += 1;
        }
        if self.trust_chain_verification != 0 {
            len += 1;
        }
        if self.custom_validator_config.is_some() {
            len += 1;
        }
        if self.only_verify_leaf_cert_crl {
            len += 1;
        }
        if self.max_verify_depth.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext", len)?;
        if let Some(v) = self.trusted_ca.as_ref() {
            struct_ser.serialize_field("trustedCa", v)?;
        }
        if let Some(v) = self.ca_certificate_provider_instance.as_ref() {
            struct_ser.serialize_field("caCertificateProviderInstance", v)?;
        }
        if let Some(v) = self.watched_directory.as_ref() {
            struct_ser.serialize_field("watchedDirectory", v)?;
        }
        if !self.verify_certificate_spki.is_empty() {
            struct_ser.serialize_field("verifyCertificateSpki", &self.verify_certificate_spki)?;
        }
        if !self.verify_certificate_hash.is_empty() {
            struct_ser.serialize_field("verifyCertificateHash", &self.verify_certificate_hash)?;
        }
        if !self.match_typed_subject_alt_names.is_empty() {
            struct_ser.serialize_field("matchTypedSubjectAltNames", &self.match_typed_subject_alt_names)?;
        }
        if !self.match_subject_alt_names.is_empty() {
            struct_ser.serialize_field("matchSubjectAltNames", &self.match_subject_alt_names)?;
        }
        if let Some(v) = self.require_signed_certificate_timestamp.as_ref() {
            struct_ser.serialize_field("requireSignedCertificateTimestamp", v)?;
        }
        if let Some(v) = self.crl.as_ref() {
            struct_ser.serialize_field("crl", v)?;
        }
        if self.allow_expired_certificate {
            struct_ser.serialize_field("allowExpiredCertificate", &self.allow_expired_certificate)?;
        }
        if self.trust_chain_verification != 0 {
            let v = certificate_validation_context::TrustChainVerification::from_i32(self.trust_chain_verification)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.trust_chain_verification)))?;
            struct_ser.serialize_field("trustChainVerification", &v)?;
        }
        if let Some(v) = self.custom_validator_config.as_ref() {
            struct_ser.serialize_field("customValidatorConfig", v)?;
        }
        if self.only_verify_leaf_cert_crl {
            struct_ser.serialize_field("onlyVerifyLeafCertCrl", &self.only_verify_leaf_cert_crl)?;
        }
        if let Some(v) = self.max_verify_depth.as_ref() {
            struct_ser.serialize_field("maxVerifyDepth", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CertificateValidationContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trusted_ca",
            "trustedCa",
            "ca_certificate_provider_instance",
            "caCertificateProviderInstance",
            "watched_directory",
            "watchedDirectory",
            "verify_certificate_spki",
            "verifyCertificateSpki",
            "verify_certificate_hash",
            "verifyCertificateHash",
            "match_typed_subject_alt_names",
            "matchTypedSubjectAltNames",
            "match_subject_alt_names",
            "matchSubjectAltNames",
            "require_signed_certificate_timestamp",
            "requireSignedCertificateTimestamp",
            "crl",
            "allow_expired_certificate",
            "allowExpiredCertificate",
            "trust_chain_verification",
            "trustChainVerification",
            "custom_validator_config",
            "customValidatorConfig",
            "only_verify_leaf_cert_crl",
            "onlyVerifyLeafCertCrl",
            "max_verify_depth",
            "maxVerifyDepth",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrustedCa,
            CaCertificateProviderInstance,
            WatchedDirectory,
            VerifyCertificateSpki,
            VerifyCertificateHash,
            MatchTypedSubjectAltNames,
            MatchSubjectAltNames,
            RequireSignedCertificateTimestamp,
            Crl,
            AllowExpiredCertificate,
            TrustChainVerification,
            CustomValidatorConfig,
            OnlyVerifyLeafCertCrl,
            MaxVerifyDepth,
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
                            "trustedCa" | "trusted_ca" => Ok(GeneratedField::TrustedCa),
                            "caCertificateProviderInstance" | "ca_certificate_provider_instance" => Ok(GeneratedField::CaCertificateProviderInstance),
                            "watchedDirectory" | "watched_directory" => Ok(GeneratedField::WatchedDirectory),
                            "verifyCertificateSpki" | "verify_certificate_spki" => Ok(GeneratedField::VerifyCertificateSpki),
                            "verifyCertificateHash" | "verify_certificate_hash" => Ok(GeneratedField::VerifyCertificateHash),
                            "matchTypedSubjectAltNames" | "match_typed_subject_alt_names" => Ok(GeneratedField::MatchTypedSubjectAltNames),
                            "matchSubjectAltNames" | "match_subject_alt_names" => Ok(GeneratedField::MatchSubjectAltNames),
                            "requireSignedCertificateTimestamp" | "require_signed_certificate_timestamp" => Ok(GeneratedField::RequireSignedCertificateTimestamp),
                            "crl" => Ok(GeneratedField::Crl),
                            "allowExpiredCertificate" | "allow_expired_certificate" => Ok(GeneratedField::AllowExpiredCertificate),
                            "trustChainVerification" | "trust_chain_verification" => Ok(GeneratedField::TrustChainVerification),
                            "customValidatorConfig" | "custom_validator_config" => Ok(GeneratedField::CustomValidatorConfig),
                            "onlyVerifyLeafCertCrl" | "only_verify_leaf_cert_crl" => Ok(GeneratedField::OnlyVerifyLeafCertCrl),
                            "maxVerifyDepth" | "max_verify_depth" => Ok(GeneratedField::MaxVerifyDepth),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CertificateValidationContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CertificateValidationContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trusted_ca__ = None;
                let mut ca_certificate_provider_instance__ = None;
                let mut watched_directory__ = None;
                let mut verify_certificate_spki__ = None;
                let mut verify_certificate_hash__ = None;
                let mut match_typed_subject_alt_names__ = None;
                let mut match_subject_alt_names__ = None;
                let mut require_signed_certificate_timestamp__ = None;
                let mut crl__ = None;
                let mut allow_expired_certificate__ = None;
                let mut trust_chain_verification__ = None;
                let mut custom_validator_config__ = None;
                let mut only_verify_leaf_cert_crl__ = None;
                let mut max_verify_depth__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TrustedCa => {
                            if trusted_ca__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedCa"));
                            }
                            trusted_ca__ = map.next_value()?;
                        }
                        GeneratedField::CaCertificateProviderInstance => {
                            if ca_certificate_provider_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caCertificateProviderInstance"));
                            }
                            ca_certificate_provider_instance__ = map.next_value()?;
                        }
                        GeneratedField::WatchedDirectory => {
                            if watched_directory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchedDirectory"));
                            }
                            watched_directory__ = map.next_value()?;
                        }
                        GeneratedField::VerifyCertificateSpki => {
                            if verify_certificate_spki__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyCertificateSpki"));
                            }
                            verify_certificate_spki__ = Some(map.next_value()?);
                        }
                        GeneratedField::VerifyCertificateHash => {
                            if verify_certificate_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifyCertificateHash"));
                            }
                            verify_certificate_hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::MatchTypedSubjectAltNames => {
                            if match_typed_subject_alt_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchTypedSubjectAltNames"));
                            }
                            match_typed_subject_alt_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::MatchSubjectAltNames => {
                            if match_subject_alt_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchSubjectAltNames"));
                            }
                            match_subject_alt_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequireSignedCertificateTimestamp => {
                            if require_signed_certificate_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requireSignedCertificateTimestamp"));
                            }
                            require_signed_certificate_timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Crl => {
                            if crl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("crl"));
                            }
                            crl__ = map.next_value()?;
                        }
                        GeneratedField::AllowExpiredCertificate => {
                            if allow_expired_certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowExpiredCertificate"));
                            }
                            allow_expired_certificate__ = Some(map.next_value()?);
                        }
                        GeneratedField::TrustChainVerification => {
                            if trust_chain_verification__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustChainVerification"));
                            }
                            trust_chain_verification__ = Some(map.next_value::<certificate_validation_context::TrustChainVerification>()? as i32);
                        }
                        GeneratedField::CustomValidatorConfig => {
                            if custom_validator_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customValidatorConfig"));
                            }
                            custom_validator_config__ = map.next_value()?;
                        }
                        GeneratedField::OnlyVerifyLeafCertCrl => {
                            if only_verify_leaf_cert_crl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("onlyVerifyLeafCertCrl"));
                            }
                            only_verify_leaf_cert_crl__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxVerifyDepth => {
                            if max_verify_depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxVerifyDepth"));
                            }
                            max_verify_depth__ = map.next_value()?;
                        }
                    }
                }
                Ok(CertificateValidationContext {
                    trusted_ca: trusted_ca__,
                    ca_certificate_provider_instance: ca_certificate_provider_instance__,
                    watched_directory: watched_directory__,
                    verify_certificate_spki: verify_certificate_spki__.unwrap_or_default(),
                    verify_certificate_hash: verify_certificate_hash__.unwrap_or_default(),
                    match_typed_subject_alt_names: match_typed_subject_alt_names__.unwrap_or_default(),
                    match_subject_alt_names: match_subject_alt_names__.unwrap_or_default(),
                    require_signed_certificate_timestamp: require_signed_certificate_timestamp__,
                    crl: crl__,
                    allow_expired_certificate: allow_expired_certificate__.unwrap_or_default(),
                    trust_chain_verification: trust_chain_verification__.unwrap_or_default(),
                    custom_validator_config: custom_validator_config__,
                    only_verify_leaf_cert_crl: only_verify_leaf_cert_crl__.unwrap_or_default(),
                    max_verify_depth: max_verify_depth__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CertificateValidationContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for certificate_validation_context::TrustChainVerification {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::VerifyTrustChain => "VERIFY_TRUST_CHAIN",
            Self::AcceptUntrusted => "ACCEPT_UNTRUSTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for certificate_validation_context::TrustChainVerification {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VERIFY_TRUST_CHAIN",
            "ACCEPT_UNTRUSTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = certificate_validation_context::TrustChainVerification;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(certificate_validation_context::TrustChainVerification::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(certificate_validation_context::TrustChainVerification::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VERIFY_TRUST_CHAIN" => Ok(certificate_validation_context::TrustChainVerification::VerifyTrustChain),
                    "ACCEPT_UNTRUSTED" => Ok(certificate_validation_context::TrustChainVerification::AcceptUntrusted),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CommonTlsContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tls_params.is_some() {
            len += 1;
        }
        if !self.tls_certificates.is_empty() {
            len += 1;
        }
        if !self.tls_certificate_sds_secret_configs.is_empty() {
            len += 1;
        }
        if self.tls_certificate_provider_instance.is_some() {
            len += 1;
        }
        if self.tls_certificate_certificate_provider.is_some() {
            len += 1;
        }
        if self.tls_certificate_certificate_provider_instance.is_some() {
            len += 1;
        }
        if !self.alpn_protocols.is_empty() {
            len += 1;
        }
        if self.custom_handshaker.is_some() {
            len += 1;
        }
        if self.key_log.is_some() {
            len += 1;
        }
        if self.validation_context_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext", len)?;
        if let Some(v) = self.tls_params.as_ref() {
            struct_ser.serialize_field("tlsParams", v)?;
        }
        if !self.tls_certificates.is_empty() {
            struct_ser.serialize_field("tlsCertificates", &self.tls_certificates)?;
        }
        if !self.tls_certificate_sds_secret_configs.is_empty() {
            struct_ser.serialize_field("tlsCertificateSdsSecretConfigs", &self.tls_certificate_sds_secret_configs)?;
        }
        if let Some(v) = self.tls_certificate_provider_instance.as_ref() {
            struct_ser.serialize_field("tlsCertificateProviderInstance", v)?;
        }
        if let Some(v) = self.tls_certificate_certificate_provider.as_ref() {
            struct_ser.serialize_field("tlsCertificateCertificateProvider", v)?;
        }
        if let Some(v) = self.tls_certificate_certificate_provider_instance.as_ref() {
            struct_ser.serialize_field("tlsCertificateCertificateProviderInstance", v)?;
        }
        if !self.alpn_protocols.is_empty() {
            struct_ser.serialize_field("alpnProtocols", &self.alpn_protocols)?;
        }
        if let Some(v) = self.custom_handshaker.as_ref() {
            struct_ser.serialize_field("customHandshaker", v)?;
        }
        if let Some(v) = self.key_log.as_ref() {
            struct_ser.serialize_field("keyLog", v)?;
        }
        if let Some(v) = self.validation_context_type.as_ref() {
            match v {
                common_tls_context::ValidationContextType::ValidationContext(v) => {
                    struct_ser.serialize_field("validationContext", v)?;
                }
                common_tls_context::ValidationContextType::ValidationContextSdsSecretConfig(v) => {
                    struct_ser.serialize_field("validationContextSdsSecretConfig", v)?;
                }
                common_tls_context::ValidationContextType::CombinedValidationContext(v) => {
                    struct_ser.serialize_field("combinedValidationContext", v)?;
                }
                common_tls_context::ValidationContextType::ValidationContextCertificateProvider(v) => {
                    struct_ser.serialize_field("validationContextCertificateProvider", v)?;
                }
                common_tls_context::ValidationContextType::ValidationContextCertificateProviderInstance(v) => {
                    struct_ser.serialize_field("validationContextCertificateProviderInstance", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommonTlsContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tls_params",
            "tlsParams",
            "tls_certificates",
            "tlsCertificates",
            "tls_certificate_sds_secret_configs",
            "tlsCertificateSdsSecretConfigs",
            "tls_certificate_provider_instance",
            "tlsCertificateProviderInstance",
            "tls_certificate_certificate_provider",
            "tlsCertificateCertificateProvider",
            "tls_certificate_certificate_provider_instance",
            "tlsCertificateCertificateProviderInstance",
            "alpn_protocols",
            "alpnProtocols",
            "custom_handshaker",
            "customHandshaker",
            "key_log",
            "keyLog",
            "validation_context",
            "validationContext",
            "validation_context_sds_secret_config",
            "validationContextSdsSecretConfig",
            "combined_validation_context",
            "combinedValidationContext",
            "validation_context_certificate_provider",
            "validationContextCertificateProvider",
            "validation_context_certificate_provider_instance",
            "validationContextCertificateProviderInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TlsParams,
            TlsCertificates,
            TlsCertificateSdsSecretConfigs,
            TlsCertificateProviderInstance,
            TlsCertificateCertificateProvider,
            TlsCertificateCertificateProviderInstance,
            AlpnProtocols,
            CustomHandshaker,
            KeyLog,
            ValidationContext,
            ValidationContextSdsSecretConfig,
            CombinedValidationContext,
            ValidationContextCertificateProvider,
            ValidationContextCertificateProviderInstance,
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
                            "tlsParams" | "tls_params" => Ok(GeneratedField::TlsParams),
                            "tlsCertificates" | "tls_certificates" => Ok(GeneratedField::TlsCertificates),
                            "tlsCertificateSdsSecretConfigs" | "tls_certificate_sds_secret_configs" => Ok(GeneratedField::TlsCertificateSdsSecretConfigs),
                            "tlsCertificateProviderInstance" | "tls_certificate_provider_instance" => Ok(GeneratedField::TlsCertificateProviderInstance),
                            "tlsCertificateCertificateProvider" | "tls_certificate_certificate_provider" => Ok(GeneratedField::TlsCertificateCertificateProvider),
                            "tlsCertificateCertificateProviderInstance" | "tls_certificate_certificate_provider_instance" => Ok(GeneratedField::TlsCertificateCertificateProviderInstance),
                            "alpnProtocols" | "alpn_protocols" => Ok(GeneratedField::AlpnProtocols),
                            "customHandshaker" | "custom_handshaker" => Ok(GeneratedField::CustomHandshaker),
                            "keyLog" | "key_log" => Ok(GeneratedField::KeyLog),
                            "validationContext" | "validation_context" => Ok(GeneratedField::ValidationContext),
                            "validationContextSdsSecretConfig" | "validation_context_sds_secret_config" => Ok(GeneratedField::ValidationContextSdsSecretConfig),
                            "combinedValidationContext" | "combined_validation_context" => Ok(GeneratedField::CombinedValidationContext),
                            "validationContextCertificateProvider" | "validation_context_certificate_provider" => Ok(GeneratedField::ValidationContextCertificateProvider),
                            "validationContextCertificateProviderInstance" | "validation_context_certificate_provider_instance" => Ok(GeneratedField::ValidationContextCertificateProviderInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommonTlsContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CommonTlsContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommonTlsContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tls_params__ = None;
                let mut tls_certificates__ = None;
                let mut tls_certificate_sds_secret_configs__ = None;
                let mut tls_certificate_provider_instance__ = None;
                let mut tls_certificate_certificate_provider__ = None;
                let mut tls_certificate_certificate_provider_instance__ = None;
                let mut alpn_protocols__ = None;
                let mut custom_handshaker__ = None;
                let mut key_log__ = None;
                let mut validation_context_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TlsParams => {
                            if tls_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsParams"));
                            }
                            tls_params__ = map.next_value()?;
                        }
                        GeneratedField::TlsCertificates => {
                            if tls_certificates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCertificates"));
                            }
                            tls_certificates__ = Some(map.next_value()?);
                        }
                        GeneratedField::TlsCertificateSdsSecretConfigs => {
                            if tls_certificate_sds_secret_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCertificateSdsSecretConfigs"));
                            }
                            tls_certificate_sds_secret_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::TlsCertificateProviderInstance => {
                            if tls_certificate_provider_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCertificateProviderInstance"));
                            }
                            tls_certificate_provider_instance__ = map.next_value()?;
                        }
                        GeneratedField::TlsCertificateCertificateProvider => {
                            if tls_certificate_certificate_provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCertificateCertificateProvider"));
                            }
                            tls_certificate_certificate_provider__ = map.next_value()?;
                        }
                        GeneratedField::TlsCertificateCertificateProviderInstance => {
                            if tls_certificate_certificate_provider_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCertificateCertificateProviderInstance"));
                            }
                            tls_certificate_certificate_provider_instance__ = map.next_value()?;
                        }
                        GeneratedField::AlpnProtocols => {
                            if alpn_protocols__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alpnProtocols"));
                            }
                            alpn_protocols__ = Some(map.next_value()?);
                        }
                        GeneratedField::CustomHandshaker => {
                            if custom_handshaker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customHandshaker"));
                            }
                            custom_handshaker__ = map.next_value()?;
                        }
                        GeneratedField::KeyLog => {
                            if key_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyLog"));
                            }
                            key_log__ = map.next_value()?;
                        }
                        GeneratedField::ValidationContext => {
                            if validation_context_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContext"));
                            }
                            validation_context_type__ = map.next_value::<::std::option::Option<_>>()?.map(common_tls_context::ValidationContextType::ValidationContext)
;
                        }
                        GeneratedField::ValidationContextSdsSecretConfig => {
                            if validation_context_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContextSdsSecretConfig"));
                            }
                            validation_context_type__ = map.next_value::<::std::option::Option<_>>()?.map(common_tls_context::ValidationContextType::ValidationContextSdsSecretConfig)
;
                        }
                        GeneratedField::CombinedValidationContext => {
                            if validation_context_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("combinedValidationContext"));
                            }
                            validation_context_type__ = map.next_value::<::std::option::Option<_>>()?.map(common_tls_context::ValidationContextType::CombinedValidationContext)
;
                        }
                        GeneratedField::ValidationContextCertificateProvider => {
                            if validation_context_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContextCertificateProvider"));
                            }
                            validation_context_type__ = map.next_value::<::std::option::Option<_>>()?.map(common_tls_context::ValidationContextType::ValidationContextCertificateProvider)
;
                        }
                        GeneratedField::ValidationContextCertificateProviderInstance => {
                            if validation_context_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContextCertificateProviderInstance"));
                            }
                            validation_context_type__ = map.next_value::<::std::option::Option<_>>()?.map(common_tls_context::ValidationContextType::ValidationContextCertificateProviderInstance)
;
                        }
                    }
                }
                Ok(CommonTlsContext {
                    tls_params: tls_params__,
                    tls_certificates: tls_certificates__.unwrap_or_default(),
                    tls_certificate_sds_secret_configs: tls_certificate_sds_secret_configs__.unwrap_or_default(),
                    tls_certificate_provider_instance: tls_certificate_provider_instance__,
                    tls_certificate_certificate_provider: tls_certificate_certificate_provider__,
                    tls_certificate_certificate_provider_instance: tls_certificate_certificate_provider_instance__,
                    alpn_protocols: alpn_protocols__.unwrap_or_default(),
                    custom_handshaker: custom_handshaker__,
                    key_log: key_log__,
                    validation_context_type: validation_context_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for common_tls_context::CertificateProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                common_tls_context::certificate_provider::Config::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for common_tls_context::CertificateProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TypedConfig,
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
                            "name" => Ok(GeneratedField::Name),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = common_tls_context::CertificateProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<common_tls_context::CertificateProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config__ = map.next_value::<::std::option::Option<_>>()?.map(common_tls_context::certificate_provider::Config::TypedConfig)
;
                        }
                    }
                }
                Ok(common_tls_context::CertificateProvider {
                    name: name__.unwrap_or_default(),
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for common_tls_context::CertificateProviderInstance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.instance_name.is_empty() {
            len += 1;
        }
        if !self.certificate_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance", len)?;
        if !self.instance_name.is_empty() {
            struct_ser.serialize_field("instanceName", &self.instance_name)?;
        }
        if !self.certificate_name.is_empty() {
            struct_ser.serialize_field("certificateName", &self.certificate_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for common_tls_context::CertificateProviderInstance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "instance_name",
            "instanceName",
            "certificate_name",
            "certificateName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InstanceName,
            CertificateName,
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
                            "instanceName" | "instance_name" => Ok(GeneratedField::InstanceName),
                            "certificateName" | "certificate_name" => Ok(GeneratedField::CertificateName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = common_tls_context::CertificateProviderInstance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<common_tls_context::CertificateProviderInstance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut instance_name__ = None;
                let mut certificate_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InstanceName => {
                            if instance_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceName"));
                            }
                            instance_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CertificateName => {
                            if certificate_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificateName"));
                            }
                            certificate_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(common_tls_context::CertificateProviderInstance {
                    instance_name: instance_name__.unwrap_or_default(),
                    certificate_name: certificate_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for common_tls_context::CombinedCertificateValidationContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.default_validation_context.is_some() {
            len += 1;
        }
        if self.validation_context_sds_secret_config.is_some() {
            len += 1;
        }
        if self.validation_context_certificate_provider.is_some() {
            len += 1;
        }
        if self.validation_context_certificate_provider_instance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CombinedCertificateValidationContext", len)?;
        if let Some(v) = self.default_validation_context.as_ref() {
            struct_ser.serialize_field("defaultValidationContext", v)?;
        }
        if let Some(v) = self.validation_context_sds_secret_config.as_ref() {
            struct_ser.serialize_field("validationContextSdsSecretConfig", v)?;
        }
        if let Some(v) = self.validation_context_certificate_provider.as_ref() {
            struct_ser.serialize_field("validationContextCertificateProvider", v)?;
        }
        if let Some(v) = self.validation_context_certificate_provider_instance.as_ref() {
            struct_ser.serialize_field("validationContextCertificateProviderInstance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for common_tls_context::CombinedCertificateValidationContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "default_validation_context",
            "defaultValidationContext",
            "validation_context_sds_secret_config",
            "validationContextSdsSecretConfig",
            "validation_context_certificate_provider",
            "validationContextCertificateProvider",
            "validation_context_certificate_provider_instance",
            "validationContextCertificateProviderInstance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DefaultValidationContext,
            ValidationContextSdsSecretConfig,
            ValidationContextCertificateProvider,
            ValidationContextCertificateProviderInstance,
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
                            "defaultValidationContext" | "default_validation_context" => Ok(GeneratedField::DefaultValidationContext),
                            "validationContextSdsSecretConfig" | "validation_context_sds_secret_config" => Ok(GeneratedField::ValidationContextSdsSecretConfig),
                            "validationContextCertificateProvider" | "validation_context_certificate_provider" => Ok(GeneratedField::ValidationContextCertificateProvider),
                            "validationContextCertificateProviderInstance" | "validation_context_certificate_provider_instance" => Ok(GeneratedField::ValidationContextCertificateProviderInstance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = common_tls_context::CombinedCertificateValidationContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CombinedCertificateValidationContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<common_tls_context::CombinedCertificateValidationContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut default_validation_context__ = None;
                let mut validation_context_sds_secret_config__ = None;
                let mut validation_context_certificate_provider__ = None;
                let mut validation_context_certificate_provider_instance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DefaultValidationContext => {
                            if default_validation_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValidationContext"));
                            }
                            default_validation_context__ = map.next_value()?;
                        }
                        GeneratedField::ValidationContextSdsSecretConfig => {
                            if validation_context_sds_secret_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContextSdsSecretConfig"));
                            }
                            validation_context_sds_secret_config__ = map.next_value()?;
                        }
                        GeneratedField::ValidationContextCertificateProvider => {
                            if validation_context_certificate_provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContextCertificateProvider"));
                            }
                            validation_context_certificate_provider__ = map.next_value()?;
                        }
                        GeneratedField::ValidationContextCertificateProviderInstance => {
                            if validation_context_certificate_provider_instance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContextCertificateProviderInstance"));
                            }
                            validation_context_certificate_provider_instance__ = map.next_value()?;
                        }
                    }
                }
                Ok(common_tls_context::CombinedCertificateValidationContext {
                    default_validation_context: default_validation_context__,
                    validation_context_sds_secret_config: validation_context_sds_secret_config__,
                    validation_context_certificate_provider: validation_context_certificate_provider__,
                    validation_context_certificate_provider_instance: validation_context_certificate_provider_instance__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.CommonTlsContext.CombinedCertificateValidationContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DownstreamTlsContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_tls_context.is_some() {
            len += 1;
        }
        if self.require_client_certificate.is_some() {
            len += 1;
        }
        if self.require_sni.is_some() {
            len += 1;
        }
        if self.session_timeout.is_some() {
            len += 1;
        }
        if self.ocsp_staple_policy != 0 {
            len += 1;
        }
        if self.full_scan_certs_on_sni_mismatch.is_some() {
            len += 1;
        }
        if self.session_ticket_keys_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext", len)?;
        if let Some(v) = self.common_tls_context.as_ref() {
            struct_ser.serialize_field("commonTlsContext", v)?;
        }
        if let Some(v) = self.require_client_certificate.as_ref() {
            struct_ser.serialize_field("requireClientCertificate", v)?;
        }
        if let Some(v) = self.require_sni.as_ref() {
            struct_ser.serialize_field("requireSni", v)?;
        }
        if let Some(v) = self.session_timeout.as_ref() {
            struct_ser.serialize_field("sessionTimeout", v)?;
        }
        if self.ocsp_staple_policy != 0 {
            let v = downstream_tls_context::OcspStaplePolicy::from_i32(self.ocsp_staple_policy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.ocsp_staple_policy)))?;
            struct_ser.serialize_field("ocspStaplePolicy", &v)?;
        }
        if let Some(v) = self.full_scan_certs_on_sni_mismatch.as_ref() {
            struct_ser.serialize_field("fullScanCertsOnSniMismatch", v)?;
        }
        if let Some(v) = self.session_ticket_keys_type.as_ref() {
            match v {
                downstream_tls_context::SessionTicketKeysType::SessionTicketKeys(v) => {
                    struct_ser.serialize_field("sessionTicketKeys", v)?;
                }
                downstream_tls_context::SessionTicketKeysType::SessionTicketKeysSdsSecretConfig(v) => {
                    struct_ser.serialize_field("sessionTicketKeysSdsSecretConfig", v)?;
                }
                downstream_tls_context::SessionTicketKeysType::DisableStatelessSessionResumption(v) => {
                    struct_ser.serialize_field("disableStatelessSessionResumption", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DownstreamTlsContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_tls_context",
            "commonTlsContext",
            "require_client_certificate",
            "requireClientCertificate",
            "require_sni",
            "requireSni",
            "session_timeout",
            "sessionTimeout",
            "ocsp_staple_policy",
            "ocspStaplePolicy",
            "full_scan_certs_on_sni_mismatch",
            "fullScanCertsOnSniMismatch",
            "session_ticket_keys",
            "sessionTicketKeys",
            "session_ticket_keys_sds_secret_config",
            "sessionTicketKeysSdsSecretConfig",
            "disable_stateless_session_resumption",
            "disableStatelessSessionResumption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonTlsContext,
            RequireClientCertificate,
            RequireSni,
            SessionTimeout,
            OcspStaplePolicy,
            FullScanCertsOnSniMismatch,
            SessionTicketKeys,
            SessionTicketKeysSdsSecretConfig,
            DisableStatelessSessionResumption,
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
                            "commonTlsContext" | "common_tls_context" => Ok(GeneratedField::CommonTlsContext),
                            "requireClientCertificate" | "require_client_certificate" => Ok(GeneratedField::RequireClientCertificate),
                            "requireSni" | "require_sni" => Ok(GeneratedField::RequireSni),
                            "sessionTimeout" | "session_timeout" => Ok(GeneratedField::SessionTimeout),
                            "ocspStaplePolicy" | "ocsp_staple_policy" => Ok(GeneratedField::OcspStaplePolicy),
                            "fullScanCertsOnSniMismatch" | "full_scan_certs_on_sni_mismatch" => Ok(GeneratedField::FullScanCertsOnSniMismatch),
                            "sessionTicketKeys" | "session_ticket_keys" => Ok(GeneratedField::SessionTicketKeys),
                            "sessionTicketKeysSdsSecretConfig" | "session_ticket_keys_sds_secret_config" => Ok(GeneratedField::SessionTicketKeysSdsSecretConfig),
                            "disableStatelessSessionResumption" | "disable_stateless_session_resumption" => Ok(GeneratedField::DisableStatelessSessionResumption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DownstreamTlsContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DownstreamTlsContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_tls_context__ = None;
                let mut require_client_certificate__ = None;
                let mut require_sni__ = None;
                let mut session_timeout__ = None;
                let mut ocsp_staple_policy__ = None;
                let mut full_scan_certs_on_sni_mismatch__ = None;
                let mut session_ticket_keys_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonTlsContext => {
                            if common_tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonTlsContext"));
                            }
                            common_tls_context__ = map.next_value()?;
                        }
                        GeneratedField::RequireClientCertificate => {
                            if require_client_certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requireClientCertificate"));
                            }
                            require_client_certificate__ = map.next_value()?;
                        }
                        GeneratedField::RequireSni => {
                            if require_sni__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requireSni"));
                            }
                            require_sni__ = map.next_value()?;
                        }
                        GeneratedField::SessionTimeout => {
                            if session_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionTimeout"));
                            }
                            session_timeout__ = map.next_value()?;
                        }
                        GeneratedField::OcspStaplePolicy => {
                            if ocsp_staple_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ocspStaplePolicy"));
                            }
                            ocsp_staple_policy__ = Some(map.next_value::<downstream_tls_context::OcspStaplePolicy>()? as i32);
                        }
                        GeneratedField::FullScanCertsOnSniMismatch => {
                            if full_scan_certs_on_sni_mismatch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullScanCertsOnSniMismatch"));
                            }
                            full_scan_certs_on_sni_mismatch__ = map.next_value()?;
                        }
                        GeneratedField::SessionTicketKeys => {
                            if session_ticket_keys_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionTicketKeys"));
                            }
                            session_ticket_keys_type__ = map.next_value::<::std::option::Option<_>>()?.map(downstream_tls_context::SessionTicketKeysType::SessionTicketKeys)
;
                        }
                        GeneratedField::SessionTicketKeysSdsSecretConfig => {
                            if session_ticket_keys_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionTicketKeysSdsSecretConfig"));
                            }
                            session_ticket_keys_type__ = map.next_value::<::std::option::Option<_>>()?.map(downstream_tls_context::SessionTicketKeysType::SessionTicketKeysSdsSecretConfig)
;
                        }
                        GeneratedField::DisableStatelessSessionResumption => {
                            if session_ticket_keys_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableStatelessSessionResumption"));
                            }
                            session_ticket_keys_type__ = map.next_value::<::std::option::Option<_>>()?.map(downstream_tls_context::SessionTicketKeysType::DisableStatelessSessionResumption);
                        }
                    }
                }
                Ok(DownstreamTlsContext {
                    common_tls_context: common_tls_context__,
                    require_client_certificate: require_client_certificate__,
                    require_sni: require_sni__,
                    session_timeout: session_timeout__,
                    ocsp_staple_policy: ocsp_staple_policy__.unwrap_or_default(),
                    full_scan_certs_on_sni_mismatch: full_scan_certs_on_sni_mismatch__,
                    session_ticket_keys_type: session_ticket_keys_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.DownstreamTlsContext", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for downstream_tls_context::OcspStaplePolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::LenientStapling => "LENIENT_STAPLING",
            Self::StrictStapling => "STRICT_STAPLING",
            Self::MustStaple => "MUST_STAPLE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for downstream_tls_context::OcspStaplePolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LENIENT_STAPLING",
            "STRICT_STAPLING",
            "MUST_STAPLE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = downstream_tls_context::OcspStaplePolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(downstream_tls_context::OcspStaplePolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(downstream_tls_context::OcspStaplePolicy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LENIENT_STAPLING" => Ok(downstream_tls_context::OcspStaplePolicy::LenientStapling),
                    "STRICT_STAPLING" => Ok(downstream_tls_context::OcspStaplePolicy::StrictStapling),
                    "MUST_STAPLE" => Ok(downstream_tls_context::OcspStaplePolicy::MustStaple),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GenericSecret {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.secret.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.GenericSecret", len)?;
        if let Some(v) = self.secret.as_ref() {
            struct_ser.serialize_field("secret", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenericSecret {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "secret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Secret,
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
                            "secret" => Ok(GeneratedField::Secret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenericSecret;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.GenericSecret")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GenericSecret, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut secret__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = map.next_value()?;
                        }
                    }
                }
                Ok(GenericSecret {
                    secret: secret__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.GenericSecret", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PrivateKeyProvider {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.provider_name.is_empty() {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider", len)?;
        if !self.provider_name.is_empty() {
            struct_ser.serialize_field("providerName", &self.provider_name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                private_key_provider::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PrivateKeyProvider {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "provider_name",
            "providerName",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProviderName,
            TypedConfig,
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
                            "providerName" | "provider_name" => Ok(GeneratedField::ProviderName),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PrivateKeyProvider;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PrivateKeyProvider, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut provider_name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProviderName => {
                            if provider_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("providerName"));
                            }
                            provider_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(private_key_provider::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(PrivateKeyProvider {
                    provider_name: provider_name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.PrivateKeyProvider", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SpiffeCertValidatorConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trust_domains.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.SPIFFECertValidatorConfig", len)?;
        if !self.trust_domains.is_empty() {
            struct_ser.serialize_field("trustDomains", &self.trust_domains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SpiffeCertValidatorConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trust_domains",
            "trustDomains",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TrustDomains,
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
                            "trustDomains" | "trust_domains" => Ok(GeneratedField::TrustDomains),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SpiffeCertValidatorConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.SPIFFECertValidatorConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SpiffeCertValidatorConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trust_domains__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TrustDomains => {
                            if trust_domains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustDomains"));
                            }
                            trust_domains__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SpiffeCertValidatorConfig {
                    trust_domains: trust_domains__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.SPIFFECertValidatorConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for spiffe_cert_validator_config::TrustDomain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.trust_bundle.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.SPIFFECertValidatorConfig.TrustDomain", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.trust_bundle.as_ref() {
            struct_ser.serialize_field("trustBundle", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for spiffe_cert_validator_config::TrustDomain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "trust_bundle",
            "trustBundle",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TrustBundle,
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
                            "name" => Ok(GeneratedField::Name),
                            "trustBundle" | "trust_bundle" => Ok(GeneratedField::TrustBundle),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = spiffe_cert_validator_config::TrustDomain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.SPIFFECertValidatorConfig.TrustDomain")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<spiffe_cert_validator_config::TrustDomain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut trust_bundle__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TrustBundle => {
                            if trust_bundle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustBundle"));
                            }
                            trust_bundle__ = map.next_value()?;
                        }
                    }
                }
                Ok(spiffe_cert_validator_config::TrustDomain {
                    name: name__.unwrap_or_default(),
                    trust_bundle: trust_bundle__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.SPIFFECertValidatorConfig.TrustDomain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SdsSecretConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.sds_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.sds_config.as_ref() {
            struct_ser.serialize_field("sdsConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SdsSecretConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "sds_config",
            "sdsConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            SdsConfig,
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
                            "name" => Ok(GeneratedField::Name),
                            "sdsConfig" | "sds_config" => Ok(GeneratedField::SdsConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SdsSecretConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SdsSecretConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut sds_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::SdsConfig => {
                            if sds_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sdsConfig"));
                            }
                            sds_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(SdsSecretConfig {
                    name: name__.unwrap_or_default(),
                    sds_config: sds_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.SdsSecretConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Secret {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.Secret", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            match v {
                secret::Type::TlsCertificate(v) => {
                    struct_ser.serialize_field("tlsCertificate", v)?;
                }
                secret::Type::SessionTicketKeys(v) => {
                    struct_ser.serialize_field("sessionTicketKeys", v)?;
                }
                secret::Type::ValidationContext(v) => {
                    struct_ser.serialize_field("validationContext", v)?;
                }
                secret::Type::GenericSecret(v) => {
                    struct_ser.serialize_field("genericSecret", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Secret {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "tls_certificate",
            "tlsCertificate",
            "session_ticket_keys",
            "sessionTicketKeys",
            "validation_context",
            "validationContext",
            "generic_secret",
            "genericSecret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TlsCertificate,
            SessionTicketKeys,
            ValidationContext,
            GenericSecret,
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
                            "name" => Ok(GeneratedField::Name),
                            "tlsCertificate" | "tls_certificate" => Ok(GeneratedField::TlsCertificate),
                            "sessionTicketKeys" | "session_ticket_keys" => Ok(GeneratedField::SessionTicketKeys),
                            "validationContext" | "validation_context" => Ok(GeneratedField::ValidationContext),
                            "genericSecret" | "generic_secret" => Ok(GeneratedField::GenericSecret),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Secret;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.Secret")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Secret, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut r#type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TlsCertificate => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCertificate"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<_>>()?.map(secret::Type::TlsCertificate)
;
                        }
                        GeneratedField::SessionTicketKeys => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sessionTicketKeys"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<_>>()?.map(secret::Type::SessionTicketKeys)
;
                        }
                        GeneratedField::ValidationContext => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validationContext"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<_>>()?.map(secret::Type::ValidationContext)
;
                        }
                        GeneratedField::GenericSecret => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genericSecret"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<_>>()?.map(secret::Type::GenericSecret)
;
                        }
                    }
                }
                Ok(Secret {
                    name: name__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.Secret", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectAltNameMatcher {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.san_type != 0 {
            len += 1;
        }
        if self.matcher.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher", len)?;
        if self.san_type != 0 {
            let v = subject_alt_name_matcher::SanType::from_i32(self.san_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.san_type)))?;
            struct_ser.serialize_field("sanType", &v)?;
        }
        if let Some(v) = self.matcher.as_ref() {
            struct_ser.serialize_field("matcher", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectAltNameMatcher {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "san_type",
            "sanType",
            "matcher",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SanType,
            Matcher,
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
                            "sanType" | "san_type" => Ok(GeneratedField::SanType),
                            "matcher" => Ok(GeneratedField::Matcher),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectAltNameMatcher;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubjectAltNameMatcher, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut san_type__ = None;
                let mut matcher__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SanType => {
                            if san_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sanType"));
                            }
                            san_type__ = Some(map.next_value::<subject_alt_name_matcher::SanType>()? as i32);
                        }
                        GeneratedField::Matcher => {
                            if matcher__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matcher"));
                            }
                            matcher__ = map.next_value()?;
                        }
                    }
                }
                Ok(SubjectAltNameMatcher {
                    san_type: san_type__.unwrap_or_default(),
                    matcher: matcher__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.SubjectAltNameMatcher", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for subject_alt_name_matcher::SanType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SAN_TYPE_UNSPECIFIED",
            Self::Email => "EMAIL",
            Self::Dns => "DNS",
            Self::Uri => "URI",
            Self::IpAddress => "IP_ADDRESS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for subject_alt_name_matcher::SanType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SAN_TYPE_UNSPECIFIED",
            "EMAIL",
            "DNS",
            "URI",
            "IP_ADDRESS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = subject_alt_name_matcher::SanType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(subject_alt_name_matcher::SanType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(subject_alt_name_matcher::SanType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SAN_TYPE_UNSPECIFIED" => Ok(subject_alt_name_matcher::SanType::Unspecified),
                    "EMAIL" => Ok(subject_alt_name_matcher::SanType::Email),
                    "DNS" => Ok(subject_alt_name_matcher::SanType::Dns),
                    "URI" => Ok(subject_alt_name_matcher::SanType::Uri),
                    "IP_ADDRESS" => Ok(subject_alt_name_matcher::SanType::IpAddress),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TlsCertificate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.certificate_chain.is_some() {
            len += 1;
        }
        if self.private_key.is_some() {
            len += 1;
        }
        if self.pkcs12.is_some() {
            len += 1;
        }
        if self.watched_directory.is_some() {
            len += 1;
        }
        if self.private_key_provider.is_some() {
            len += 1;
        }
        if self.password.is_some() {
            len += 1;
        }
        if self.ocsp_staple.is_some() {
            len += 1;
        }
        if !self.signed_certificate_timestamp.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsCertificate", len)?;
        if let Some(v) = self.certificate_chain.as_ref() {
            struct_ser.serialize_field("certificateChain", v)?;
        }
        if let Some(v) = self.private_key.as_ref() {
            struct_ser.serialize_field("privateKey", v)?;
        }
        if let Some(v) = self.pkcs12.as_ref() {
            struct_ser.serialize_field("pkcs12", v)?;
        }
        if let Some(v) = self.watched_directory.as_ref() {
            struct_ser.serialize_field("watchedDirectory", v)?;
        }
        if let Some(v) = self.private_key_provider.as_ref() {
            struct_ser.serialize_field("privateKeyProvider", v)?;
        }
        if let Some(v) = self.password.as_ref() {
            struct_ser.serialize_field("password", v)?;
        }
        if let Some(v) = self.ocsp_staple.as_ref() {
            struct_ser.serialize_field("ocspStaple", v)?;
        }
        if !self.signed_certificate_timestamp.is_empty() {
            struct_ser.serialize_field("signedCertificateTimestamp", &self.signed_certificate_timestamp)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsCertificate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "certificate_chain",
            "certificateChain",
            "private_key",
            "privateKey",
            "pkcs12",
            "watched_directory",
            "watchedDirectory",
            "private_key_provider",
            "privateKeyProvider",
            "password",
            "ocsp_staple",
            "ocspStaple",
            "signed_certificate_timestamp",
            "signedCertificateTimestamp",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CertificateChain,
            PrivateKey,
            Pkcs12,
            WatchedDirectory,
            PrivateKeyProvider,
            Password,
            OcspStaple,
            SignedCertificateTimestamp,
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
                            "certificateChain" | "certificate_chain" => Ok(GeneratedField::CertificateChain),
                            "privateKey" | "private_key" => Ok(GeneratedField::PrivateKey),
                            "pkcs12" => Ok(GeneratedField::Pkcs12),
                            "watchedDirectory" | "watched_directory" => Ok(GeneratedField::WatchedDirectory),
                            "privateKeyProvider" | "private_key_provider" => Ok(GeneratedField::PrivateKeyProvider),
                            "password" => Ok(GeneratedField::Password),
                            "ocspStaple" | "ocsp_staple" => Ok(GeneratedField::OcspStaple),
                            "signedCertificateTimestamp" | "signed_certificate_timestamp" => Ok(GeneratedField::SignedCertificateTimestamp),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsCertificate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.TlsCertificate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TlsCertificate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut certificate_chain__ = None;
                let mut private_key__ = None;
                let mut pkcs12__ = None;
                let mut watched_directory__ = None;
                let mut private_key_provider__ = None;
                let mut password__ = None;
                let mut ocsp_staple__ = None;
                let mut signed_certificate_timestamp__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CertificateChain => {
                            if certificate_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificateChain"));
                            }
                            certificate_chain__ = map.next_value()?;
                        }
                        GeneratedField::PrivateKey => {
                            if private_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKey"));
                            }
                            private_key__ = map.next_value()?;
                        }
                        GeneratedField::Pkcs12 => {
                            if pkcs12__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pkcs12"));
                            }
                            pkcs12__ = map.next_value()?;
                        }
                        GeneratedField::WatchedDirectory => {
                            if watched_directory__.is_some() {
                                return Err(serde::de::Error::duplicate_field("watchedDirectory"));
                            }
                            watched_directory__ = map.next_value()?;
                        }
                        GeneratedField::PrivateKeyProvider => {
                            if private_key_provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKeyProvider"));
                            }
                            private_key_provider__ = map.next_value()?;
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = map.next_value()?;
                        }
                        GeneratedField::OcspStaple => {
                            if ocsp_staple__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ocspStaple"));
                            }
                            ocsp_staple__ = map.next_value()?;
                        }
                        GeneratedField::SignedCertificateTimestamp => {
                            if signed_certificate_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedCertificateTimestamp"));
                            }
                            signed_certificate_timestamp__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TlsCertificate {
                    certificate_chain: certificate_chain__,
                    private_key: private_key__,
                    pkcs12: pkcs12__,
                    watched_directory: watched_directory__,
                    private_key_provider: private_key_provider__,
                    password: password__,
                    ocsp_staple: ocsp_staple__,
                    signed_certificate_timestamp: signed_certificate_timestamp__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsCertificate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TlsKeyLog {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.local_address_range.is_empty() {
            len += 1;
        }
        if !self.remote_address_range.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsKeyLog", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.local_address_range.is_empty() {
            struct_ser.serialize_field("localAddressRange", &self.local_address_range)?;
        }
        if !self.remote_address_range.is_empty() {
            struct_ser.serialize_field("remoteAddressRange", &self.remote_address_range)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsKeyLog {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "local_address_range",
            "localAddressRange",
            "remote_address_range",
            "remoteAddressRange",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            LocalAddressRange,
            RemoteAddressRange,
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
                            "path" => Ok(GeneratedField::Path),
                            "localAddressRange" | "local_address_range" => Ok(GeneratedField::LocalAddressRange),
                            "remoteAddressRange" | "remote_address_range" => Ok(GeneratedField::RemoteAddressRange),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsKeyLog;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.TlsKeyLog")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TlsKeyLog, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut local_address_range__ = None;
                let mut remote_address_range__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalAddressRange => {
                            if local_address_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localAddressRange"));
                            }
                            local_address_range__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoteAddressRange => {
                            if remote_address_range__.is_some() {
                                return Err(serde::de::Error::duplicate_field("remoteAddressRange"));
                            }
                            remote_address_range__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TlsKeyLog {
                    path: path__.unwrap_or_default(),
                    local_address_range: local_address_range__.unwrap_or_default(),
                    remote_address_range: remote_address_range__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsKeyLog", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TlsParameters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tls_minimum_protocol_version != 0 {
            len += 1;
        }
        if self.tls_maximum_protocol_version != 0 {
            len += 1;
        }
        if !self.cipher_suites.is_empty() {
            len += 1;
        }
        if !self.ecdh_curves.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsParameters", len)?;
        if self.tls_minimum_protocol_version != 0 {
            let v = tls_parameters::TlsProtocol::from_i32(self.tls_minimum_protocol_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.tls_minimum_protocol_version)))?;
            struct_ser.serialize_field("tlsMinimumProtocolVersion", &v)?;
        }
        if self.tls_maximum_protocol_version != 0 {
            let v = tls_parameters::TlsProtocol::from_i32(self.tls_maximum_protocol_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.tls_maximum_protocol_version)))?;
            struct_ser.serialize_field("tlsMaximumProtocolVersion", &v)?;
        }
        if !self.cipher_suites.is_empty() {
            struct_ser.serialize_field("cipherSuites", &self.cipher_suites)?;
        }
        if !self.ecdh_curves.is_empty() {
            struct_ser.serialize_field("ecdhCurves", &self.ecdh_curves)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsParameters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tls_minimum_protocol_version",
            "tlsMinimumProtocolVersion",
            "tls_maximum_protocol_version",
            "tlsMaximumProtocolVersion",
            "cipher_suites",
            "cipherSuites",
            "ecdh_curves",
            "ecdhCurves",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TlsMinimumProtocolVersion,
            TlsMaximumProtocolVersion,
            CipherSuites,
            EcdhCurves,
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
                            "tlsMinimumProtocolVersion" | "tls_minimum_protocol_version" => Ok(GeneratedField::TlsMinimumProtocolVersion),
                            "tlsMaximumProtocolVersion" | "tls_maximum_protocol_version" => Ok(GeneratedField::TlsMaximumProtocolVersion),
                            "cipherSuites" | "cipher_suites" => Ok(GeneratedField::CipherSuites),
                            "ecdhCurves" | "ecdh_curves" => Ok(GeneratedField::EcdhCurves),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsParameters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.TlsParameters")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TlsParameters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tls_minimum_protocol_version__ = None;
                let mut tls_maximum_protocol_version__ = None;
                let mut cipher_suites__ = None;
                let mut ecdh_curves__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TlsMinimumProtocolVersion => {
                            if tls_minimum_protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsMinimumProtocolVersion"));
                            }
                            tls_minimum_protocol_version__ = Some(map.next_value::<tls_parameters::TlsProtocol>()? as i32);
                        }
                        GeneratedField::TlsMaximumProtocolVersion => {
                            if tls_maximum_protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsMaximumProtocolVersion"));
                            }
                            tls_maximum_protocol_version__ = Some(map.next_value::<tls_parameters::TlsProtocol>()? as i32);
                        }
                        GeneratedField::CipherSuites => {
                            if cipher_suites__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cipherSuites"));
                            }
                            cipher_suites__ = Some(map.next_value()?);
                        }
                        GeneratedField::EcdhCurves => {
                            if ecdh_curves__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdhCurves"));
                            }
                            ecdh_curves__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TlsParameters {
                    tls_minimum_protocol_version: tls_minimum_protocol_version__.unwrap_or_default(),
                    tls_maximum_protocol_version: tls_maximum_protocol_version__.unwrap_or_default(),
                    cipher_suites: cipher_suites__.unwrap_or_default(),
                    ecdh_curves: ecdh_curves__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsParameters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tls_parameters::TlsProtocol {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::TlsAuto => "TLS_AUTO",
            Self::TlSv10 => "TLSv1_0",
            Self::TlSv11 => "TLSv1_1",
            Self::TlSv12 => "TLSv1_2",
            Self::TlSv13 => "TLSv1_3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for tls_parameters::TlsProtocol {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TLS_AUTO",
            "TLSv1_0",
            "TLSv1_1",
            "TLSv1_2",
            "TLSv1_3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tls_parameters::TlsProtocol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(tls_parameters::TlsProtocol::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(tls_parameters::TlsProtocol::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TLS_AUTO" => Ok(tls_parameters::TlsProtocol::TlsAuto),
                    "TLSv1_0" => Ok(tls_parameters::TlsProtocol::TlSv10),
                    "TLSv1_1" => Ok(tls_parameters::TlsProtocol::TlSv11),
                    "TLSv1_2" => Ok(tls_parameters::TlsProtocol::TlSv12),
                    "TLSv1_3" => Ok(tls_parameters::TlsProtocol::TlSv13),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TlsSessionTicketKeys {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.keys.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys", len)?;
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsSessionTicketKeys {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Keys,
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
                            "keys" => Ok(GeneratedField::Keys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsSessionTicketKeys;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TlsSessionTicketKeys, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut keys__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TlsSessionTicketKeys {
                    keys: keys__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.TlsSessionTicketKeys", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpstreamTlsContext {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_tls_context.is_some() {
            len += 1;
        }
        if !self.sni.is_empty() {
            len += 1;
        }
        if self.allow_renegotiation {
            len += 1;
        }
        if self.max_session_keys.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.tls.v3.UpstreamTlsContext", len)?;
        if let Some(v) = self.common_tls_context.as_ref() {
            struct_ser.serialize_field("commonTlsContext", v)?;
        }
        if !self.sni.is_empty() {
            struct_ser.serialize_field("sni", &self.sni)?;
        }
        if self.allow_renegotiation {
            struct_ser.serialize_field("allowRenegotiation", &self.allow_renegotiation)?;
        }
        if let Some(v) = self.max_session_keys.as_ref() {
            struct_ser.serialize_field("maxSessionKeys", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpstreamTlsContext {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_tls_context",
            "commonTlsContext",
            "sni",
            "allow_renegotiation",
            "allowRenegotiation",
            "max_session_keys",
            "maxSessionKeys",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonTlsContext,
            Sni,
            AllowRenegotiation,
            MaxSessionKeys,
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
                            "commonTlsContext" | "common_tls_context" => Ok(GeneratedField::CommonTlsContext),
                            "sni" => Ok(GeneratedField::Sni),
                            "allowRenegotiation" | "allow_renegotiation" => Ok(GeneratedField::AllowRenegotiation),
                            "maxSessionKeys" | "max_session_keys" => Ok(GeneratedField::MaxSessionKeys),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpstreamTlsContext;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.tls.v3.UpstreamTlsContext")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpstreamTlsContext, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_tls_context__ = None;
                let mut sni__ = None;
                let mut allow_renegotiation__ = None;
                let mut max_session_keys__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonTlsContext => {
                            if common_tls_context__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonTlsContext"));
                            }
                            common_tls_context__ = map.next_value()?;
                        }
                        GeneratedField::Sni => {
                            if sni__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sni"));
                            }
                            sni__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowRenegotiation => {
                            if allow_renegotiation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowRenegotiation"));
                            }
                            allow_renegotiation__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxSessionKeys => {
                            if max_session_keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSessionKeys"));
                            }
                            max_session_keys__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpstreamTlsContext {
                    common_tls_context: common_tls_context__,
                    sni: sni__.unwrap_or_default(),
                    allow_renegotiation: allow_renegotiation__.unwrap_or_default(),
                    max_session_keys: max_session_keys__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.tls.v3.UpstreamTlsContext", FIELDS, GeneratedVisitor)
    }
}
