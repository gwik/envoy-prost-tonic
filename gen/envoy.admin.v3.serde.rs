// @generated
impl serde::Serialize for BootstrapConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bootstrap.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.BootstrapConfigDump", len)?;
        if let Some(v) = self.bootstrap.as_ref() {
            struct_ser.serialize_field("bootstrap", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BootstrapConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bootstrap",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bootstrap,
            LastUpdated,
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
                            "bootstrap" => Ok(GeneratedField::Bootstrap),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BootstrapConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.BootstrapConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BootstrapConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bootstrap__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Bootstrap => {
                            if bootstrap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootstrap"));
                            }
                            bootstrap__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                    }
                }
                Ok(BootstrapConfigDump {
                    bootstrap: bootstrap__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.BootstrapConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Certificate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ca_cert.is_empty() {
            len += 1;
        }
        if !self.cert_chain.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.Certificate", len)?;
        if !self.ca_cert.is_empty() {
            struct_ser.serialize_field("caCert", &self.ca_cert)?;
        }
        if !self.cert_chain.is_empty() {
            struct_ser.serialize_field("certChain", &self.cert_chain)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Certificate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ca_cert",
            "caCert",
            "cert_chain",
            "certChain",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CaCert,
            CertChain,
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
                            "caCert" | "ca_cert" => Ok(GeneratedField::CaCert),
                            "certChain" | "cert_chain" => Ok(GeneratedField::CertChain),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Certificate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.Certificate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Certificate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ca_cert__ = None;
                let mut cert_chain__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CaCert => {
                            if ca_cert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caCert"));
                            }
                            ca_cert__ = Some(map.next_value()?);
                        }
                        GeneratedField::CertChain => {
                            if cert_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certChain"));
                            }
                            cert_chain__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Certificate {
                    ca_cert: ca_cert__.unwrap_or_default(),
                    cert_chain: cert_chain__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.Certificate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CertificateDetails {
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
        if !self.serial_number.is_empty() {
            len += 1;
        }
        if !self.subject_alt_names.is_empty() {
            len += 1;
        }
        if self.days_until_expiration != 0 {
            len += 1;
        }
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.expiration_time.is_some() {
            len += 1;
        }
        if self.ocsp_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.CertificateDetails", len)?;
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.serial_number.is_empty() {
            struct_ser.serialize_field("serialNumber", &self.serial_number)?;
        }
        if !self.subject_alt_names.is_empty() {
            struct_ser.serialize_field("subjectAltNames", &self.subject_alt_names)?;
        }
        if self.days_until_expiration != 0 {
            struct_ser.serialize_field("daysUntilExpiration", ToString::to_string(&self.days_until_expiration).as_str())?;
        }
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.expiration_time.as_ref() {
            struct_ser.serialize_field("expirationTime", v)?;
        }
        if let Some(v) = self.ocsp_details.as_ref() {
            struct_ser.serialize_field("ocspDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CertificateDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "serial_number",
            "serialNumber",
            "subject_alt_names",
            "subjectAltNames",
            "days_until_expiration",
            "daysUntilExpiration",
            "valid_from",
            "validFrom",
            "expiration_time",
            "expirationTime",
            "ocsp_details",
            "ocspDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            SerialNumber,
            SubjectAltNames,
            DaysUntilExpiration,
            ValidFrom,
            ExpirationTime,
            OcspDetails,
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
                            "serialNumber" | "serial_number" => Ok(GeneratedField::SerialNumber),
                            "subjectAltNames" | "subject_alt_names" => Ok(GeneratedField::SubjectAltNames),
                            "daysUntilExpiration" | "days_until_expiration" => Ok(GeneratedField::DaysUntilExpiration),
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "expirationTime" | "expiration_time" => Ok(GeneratedField::ExpirationTime),
                            "ocspDetails" | "ocsp_details" => Ok(GeneratedField::OcspDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CertificateDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.CertificateDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CertificateDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut serial_number__ = None;
                let mut subject_alt_names__ = None;
                let mut days_until_expiration__ = None;
                let mut valid_from__ = None;
                let mut expiration_time__ = None;
                let mut ocsp_details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::SerialNumber => {
                            if serial_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serialNumber"));
                            }
                            serial_number__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectAltNames => {
                            if subject_alt_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectAltNames"));
                            }
                            subject_alt_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::DaysUntilExpiration => {
                            if days_until_expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daysUntilExpiration"));
                            }
                            days_until_expiration__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map.next_value()?;
                        }
                        GeneratedField::ExpirationTime => {
                            if expiration_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationTime"));
                            }
                            expiration_time__ = map.next_value()?;
                        }
                        GeneratedField::OcspDetails => {
                            if ocsp_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ocspDetails"));
                            }
                            ocsp_details__ = map.next_value()?;
                        }
                    }
                }
                Ok(CertificateDetails {
                    path: path__.unwrap_or_default(),
                    serial_number: serial_number__.unwrap_or_default(),
                    subject_alt_names: subject_alt_names__.unwrap_or_default(),
                    days_until_expiration: days_until_expiration__.unwrap_or_default(),
                    valid_from: valid_from__,
                    expiration_time: expiration_time__,
                    ocsp_details: ocsp_details__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.CertificateDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for certificate_details::OcspDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.valid_from.is_some() {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.CertificateDetails.OcspDetails", len)?;
        if let Some(v) = self.valid_from.as_ref() {
            struct_ser.serialize_field("validFrom", v)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for certificate_details::OcspDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "valid_from",
            "validFrom",
            "expiration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidFrom,
            Expiration,
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
                            "validFrom" | "valid_from" => Ok(GeneratedField::ValidFrom),
                            "expiration" => Ok(GeneratedField::Expiration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = certificate_details::OcspDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.CertificateDetails.OcspDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<certificate_details::OcspDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut valid_from__ = None;
                let mut expiration__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidFrom => {
                            if valid_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validFrom"));
                            }
                            valid_from__ = map.next_value()?;
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map.next_value()?;
                        }
                    }
                }
                Ok(certificate_details::OcspDetails {
                    valid_from: valid_from__,
                    expiration: expiration__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.CertificateDetails.OcspDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Certificates {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.certificates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.Certificates", len)?;
        if !self.certificates.is_empty() {
            struct_ser.serialize_field("certificates", &self.certificates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Certificates {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "certificates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Certificates,
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
                            "certificates" => Ok(GeneratedField::Certificates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Certificates;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.Certificates")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Certificates, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut certificates__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Certificates => {
                            if certificates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificates"));
                            }
                            certificates__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Certificates {
                    certificates: certificates__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.Certificates", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientResourceStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Requested => "REQUESTED",
            Self::DoesNotExist => "DOES_NOT_EXIST",
            Self::Acked => "ACKED",
            Self::Nacked => "NACKED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ClientResourceStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "REQUESTED",
            "DOES_NOT_EXIST",
            "ACKED",
            "NACKED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientResourceStatus;

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
                    .and_then(ClientResourceStatus::from_i32)
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
                    .and_then(ClientResourceStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(ClientResourceStatus::Unknown),
                    "REQUESTED" => Ok(ClientResourceStatus::Requested),
                    "DOES_NOT_EXIST" => Ok(ClientResourceStatus::DoesNotExist),
                    "ACKED" => Ok(ClientResourceStatus::Acked),
                    "NACKED" => Ok(ClientResourceStatus::Nacked),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterStatus {
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
        if self.added_via_api {
            len += 1;
        }
        if self.success_rate_ejection_threshold.is_some() {
            len += 1;
        }
        if !self.host_statuses.is_empty() {
            len += 1;
        }
        if self.local_origin_success_rate_ejection_threshold.is_some() {
            len += 1;
        }
        if self.circuit_breakers.is_some() {
            len += 1;
        }
        if !self.observability_name.is_empty() {
            len += 1;
        }
        if !self.eds_service_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ClusterStatus", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.added_via_api {
            struct_ser.serialize_field("addedViaApi", &self.added_via_api)?;
        }
        if let Some(v) = self.success_rate_ejection_threshold.as_ref() {
            struct_ser.serialize_field("successRateEjectionThreshold", v)?;
        }
        if !self.host_statuses.is_empty() {
            struct_ser.serialize_field("hostStatuses", &self.host_statuses)?;
        }
        if let Some(v) = self.local_origin_success_rate_ejection_threshold.as_ref() {
            struct_ser.serialize_field("localOriginSuccessRateEjectionThreshold", v)?;
        }
        if let Some(v) = self.circuit_breakers.as_ref() {
            struct_ser.serialize_field("circuitBreakers", v)?;
        }
        if !self.observability_name.is_empty() {
            struct_ser.serialize_field("observabilityName", &self.observability_name)?;
        }
        if !self.eds_service_name.is_empty() {
            struct_ser.serialize_field("edsServiceName", &self.eds_service_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "added_via_api",
            "addedViaApi",
            "success_rate_ejection_threshold",
            "successRateEjectionThreshold",
            "host_statuses",
            "hostStatuses",
            "local_origin_success_rate_ejection_threshold",
            "localOriginSuccessRateEjectionThreshold",
            "circuit_breakers",
            "circuitBreakers",
            "observability_name",
            "observabilityName",
            "eds_service_name",
            "edsServiceName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            AddedViaApi,
            SuccessRateEjectionThreshold,
            HostStatuses,
            LocalOriginSuccessRateEjectionThreshold,
            CircuitBreakers,
            ObservabilityName,
            EdsServiceName,
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
                            "addedViaApi" | "added_via_api" => Ok(GeneratedField::AddedViaApi),
                            "successRateEjectionThreshold" | "success_rate_ejection_threshold" => Ok(GeneratedField::SuccessRateEjectionThreshold),
                            "hostStatuses" | "host_statuses" => Ok(GeneratedField::HostStatuses),
                            "localOriginSuccessRateEjectionThreshold" | "local_origin_success_rate_ejection_threshold" => Ok(GeneratedField::LocalOriginSuccessRateEjectionThreshold),
                            "circuitBreakers" | "circuit_breakers" => Ok(GeneratedField::CircuitBreakers),
                            "observabilityName" | "observability_name" => Ok(GeneratedField::ObservabilityName),
                            "edsServiceName" | "eds_service_name" => Ok(GeneratedField::EdsServiceName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ClusterStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClusterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut added_via_api__ = None;
                let mut success_rate_ejection_threshold__ = None;
                let mut host_statuses__ = None;
                let mut local_origin_success_rate_ejection_threshold__ = None;
                let mut circuit_breakers__ = None;
                let mut observability_name__ = None;
                let mut eds_service_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::AddedViaApi => {
                            if added_via_api__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addedViaApi"));
                            }
                            added_via_api__ = Some(map.next_value()?);
                        }
                        GeneratedField::SuccessRateEjectionThreshold => {
                            if success_rate_ejection_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRateEjectionThreshold"));
                            }
                            success_rate_ejection_threshold__ = map.next_value()?;
                        }
                        GeneratedField::HostStatuses => {
                            if host_statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostStatuses"));
                            }
                            host_statuses__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalOriginSuccessRateEjectionThreshold => {
                            if local_origin_success_rate_ejection_threshold__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localOriginSuccessRateEjectionThreshold"));
                            }
                            local_origin_success_rate_ejection_threshold__ = map.next_value()?;
                        }
                        GeneratedField::CircuitBreakers => {
                            if circuit_breakers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("circuitBreakers"));
                            }
                            circuit_breakers__ = map.next_value()?;
                        }
                        GeneratedField::ObservabilityName => {
                            if observability_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("observabilityName"));
                            }
                            observability_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::EdsServiceName => {
                            if eds_service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edsServiceName"));
                            }
                            eds_service_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClusterStatus {
                    name: name__.unwrap_or_default(),
                    added_via_api: added_via_api__.unwrap_or_default(),
                    success_rate_ejection_threshold: success_rate_ejection_threshold__,
                    host_statuses: host_statuses__.unwrap_or_default(),
                    local_origin_success_rate_ejection_threshold: local_origin_success_rate_ejection_threshold__,
                    circuit_breakers: circuit_breakers__,
                    observability_name: observability_name__.unwrap_or_default(),
                    eds_service_name: eds_service_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ClusterStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Clusters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_statuses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.Clusters", len)?;
        if !self.cluster_statuses.is_empty() {
            struct_ser.serialize_field("clusterStatuses", &self.cluster_statuses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Clusters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_statuses",
            "clusterStatuses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterStatuses,
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
                            "clusterStatuses" | "cluster_statuses" => Ok(GeneratedField::ClusterStatuses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Clusters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.Clusters")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Clusters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_statuses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClusterStatuses => {
                            if cluster_statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterStatuses"));
                            }
                            cluster_statuses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Clusters {
                    cluster_statuses: cluster_statuses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.Clusters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClustersConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.static_clusters.is_empty() {
            len += 1;
        }
        if !self.dynamic_active_clusters.is_empty() {
            len += 1;
        }
        if !self.dynamic_warming_clusters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ClustersConfigDump", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if !self.static_clusters.is_empty() {
            struct_ser.serialize_field("staticClusters", &self.static_clusters)?;
        }
        if !self.dynamic_active_clusters.is_empty() {
            struct_ser.serialize_field("dynamicActiveClusters", &self.dynamic_active_clusters)?;
        }
        if !self.dynamic_warming_clusters.is_empty() {
            struct_ser.serialize_field("dynamicWarmingClusters", &self.dynamic_warming_clusters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClustersConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "static_clusters",
            "staticClusters",
            "dynamic_active_clusters",
            "dynamicActiveClusters",
            "dynamic_warming_clusters",
            "dynamicWarmingClusters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            StaticClusters,
            DynamicActiveClusters,
            DynamicWarmingClusters,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "staticClusters" | "static_clusters" => Ok(GeneratedField::StaticClusters),
                            "dynamicActiveClusters" | "dynamic_active_clusters" => Ok(GeneratedField::DynamicActiveClusters),
                            "dynamicWarmingClusters" | "dynamic_warming_clusters" => Ok(GeneratedField::DynamicWarmingClusters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClustersConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ClustersConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClustersConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut static_clusters__ = None;
                let mut dynamic_active_clusters__ = None;
                let mut dynamic_warming_clusters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::StaticClusters => {
                            if static_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticClusters"));
                            }
                            static_clusters__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicActiveClusters => {
                            if dynamic_active_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicActiveClusters"));
                            }
                            dynamic_active_clusters__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicWarmingClusters => {
                            if dynamic_warming_clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicWarmingClusters"));
                            }
                            dynamic_warming_clusters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClustersConfigDump {
                    version_info: version_info__.unwrap_or_default(),
                    static_clusters: static_clusters__.unwrap_or_default(),
                    dynamic_active_clusters: dynamic_active_clusters__.unwrap_or_default(),
                    dynamic_warming_clusters: dynamic_warming_clusters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ClustersConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for clusters_config_dump::DynamicCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.cluster.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ClustersConfigDump.DynamicCluster", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.cluster.as_ref() {
            struct_ser.serialize_field("cluster", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for clusters_config_dump::DynamicCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "cluster",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Cluster,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "cluster" => Ok(GeneratedField::Cluster),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = clusters_config_dump::DynamicCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ClustersConfigDump.DynamicCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<clusters_config_dump::DynamicCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut cluster__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(clusters_config_dump::DynamicCluster {
                    version_info: version_info__.unwrap_or_default(),
                    cluster: cluster__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ClustersConfigDump.DynamicCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for clusters_config_dump::StaticCluster {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cluster.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ClustersConfigDump.StaticCluster", len)?;
        if let Some(v) = self.cluster.as_ref() {
            struct_ser.serialize_field("cluster", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for clusters_config_dump::StaticCluster {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            LastUpdated,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = clusters_config_dump::StaticCluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ClustersConfigDump.StaticCluster")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<clusters_config_dump::StaticCluster, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                    }
                }
                Ok(clusters_config_dump::StaticCluster {
                    cluster: cluster__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ClustersConfigDump.StaticCluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommandLineOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.base_id != 0 {
            len += 1;
        }
        if self.use_dynamic_base_id {
            len += 1;
        }
        if !self.base_id_path.is_empty() {
            len += 1;
        }
        if self.concurrency != 0 {
            len += 1;
        }
        if !self.config_path.is_empty() {
            len += 1;
        }
        if !self.config_yaml.is_empty() {
            len += 1;
        }
        if self.allow_unknown_static_fields {
            len += 1;
        }
        if self.reject_unknown_dynamic_fields {
            len += 1;
        }
        if self.ignore_unknown_dynamic_fields {
            len += 1;
        }
        if !self.admin_address_path.is_empty() {
            len += 1;
        }
        if self.local_address_ip_version != 0 {
            len += 1;
        }
        if !self.log_level.is_empty() {
            len += 1;
        }
        if !self.component_log_level.is_empty() {
            len += 1;
        }
        if !self.log_format.is_empty() {
            len += 1;
        }
        if self.log_format_escaped {
            len += 1;
        }
        if !self.log_path.is_empty() {
            len += 1;
        }
        if !self.service_cluster.is_empty() {
            len += 1;
        }
        if !self.service_node.is_empty() {
            len += 1;
        }
        if !self.service_zone.is_empty() {
            len += 1;
        }
        if self.file_flush_interval.is_some() {
            len += 1;
        }
        if self.drain_time.is_some() {
            len += 1;
        }
        if self.drain_strategy != 0 {
            len += 1;
        }
        if self.parent_shutdown_time.is_some() {
            len += 1;
        }
        if self.mode != 0 {
            len += 1;
        }
        if self.disable_hot_restart {
            len += 1;
        }
        if self.enable_mutex_tracing {
            len += 1;
        }
        if self.restart_epoch != 0 {
            len += 1;
        }
        if self.cpuset_threads {
            len += 1;
        }
        if !self.disabled_extensions.is_empty() {
            len += 1;
        }
        if self.enable_fine_grain_logging {
            len += 1;
        }
        if !self.socket_path.is_empty() {
            len += 1;
        }
        if self.socket_mode != 0 {
            len += 1;
        }
        if self.enable_core_dump {
            len += 1;
        }
        if !self.stats_tag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.CommandLineOptions", len)?;
        if self.base_id != 0 {
            struct_ser.serialize_field("baseId", ToString::to_string(&self.base_id).as_str())?;
        }
        if self.use_dynamic_base_id {
            struct_ser.serialize_field("useDynamicBaseId", &self.use_dynamic_base_id)?;
        }
        if !self.base_id_path.is_empty() {
            struct_ser.serialize_field("baseIdPath", &self.base_id_path)?;
        }
        if self.concurrency != 0 {
            struct_ser.serialize_field("concurrency", &self.concurrency)?;
        }
        if !self.config_path.is_empty() {
            struct_ser.serialize_field("configPath", &self.config_path)?;
        }
        if !self.config_yaml.is_empty() {
            struct_ser.serialize_field("configYaml", &self.config_yaml)?;
        }
        if self.allow_unknown_static_fields {
            struct_ser.serialize_field("allowUnknownStaticFields", &self.allow_unknown_static_fields)?;
        }
        if self.reject_unknown_dynamic_fields {
            struct_ser.serialize_field("rejectUnknownDynamicFields", &self.reject_unknown_dynamic_fields)?;
        }
        if self.ignore_unknown_dynamic_fields {
            struct_ser.serialize_field("ignoreUnknownDynamicFields", &self.ignore_unknown_dynamic_fields)?;
        }
        if !self.admin_address_path.is_empty() {
            struct_ser.serialize_field("adminAddressPath", &self.admin_address_path)?;
        }
        if self.local_address_ip_version != 0 {
            let v = command_line_options::IpVersion::from_i32(self.local_address_ip_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.local_address_ip_version)))?;
            struct_ser.serialize_field("localAddressIpVersion", &v)?;
        }
        if !self.log_level.is_empty() {
            struct_ser.serialize_field("logLevel", &self.log_level)?;
        }
        if !self.component_log_level.is_empty() {
            struct_ser.serialize_field("componentLogLevel", &self.component_log_level)?;
        }
        if !self.log_format.is_empty() {
            struct_ser.serialize_field("logFormat", &self.log_format)?;
        }
        if self.log_format_escaped {
            struct_ser.serialize_field("logFormatEscaped", &self.log_format_escaped)?;
        }
        if !self.log_path.is_empty() {
            struct_ser.serialize_field("logPath", &self.log_path)?;
        }
        if !self.service_cluster.is_empty() {
            struct_ser.serialize_field("serviceCluster", &self.service_cluster)?;
        }
        if !self.service_node.is_empty() {
            struct_ser.serialize_field("serviceNode", &self.service_node)?;
        }
        if !self.service_zone.is_empty() {
            struct_ser.serialize_field("serviceZone", &self.service_zone)?;
        }
        if let Some(v) = self.file_flush_interval.as_ref() {
            struct_ser.serialize_field("fileFlushInterval", v)?;
        }
        if let Some(v) = self.drain_time.as_ref() {
            struct_ser.serialize_field("drainTime", v)?;
        }
        if self.drain_strategy != 0 {
            let v = command_line_options::DrainStrategy::from_i32(self.drain_strategy)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.drain_strategy)))?;
            struct_ser.serialize_field("drainStrategy", &v)?;
        }
        if let Some(v) = self.parent_shutdown_time.as_ref() {
            struct_ser.serialize_field("parentShutdownTime", v)?;
        }
        if self.mode != 0 {
            let v = command_line_options::Mode::from_i32(self.mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.mode)))?;
            struct_ser.serialize_field("mode", &v)?;
        }
        if self.disable_hot_restart {
            struct_ser.serialize_field("disableHotRestart", &self.disable_hot_restart)?;
        }
        if self.enable_mutex_tracing {
            struct_ser.serialize_field("enableMutexTracing", &self.enable_mutex_tracing)?;
        }
        if self.restart_epoch != 0 {
            struct_ser.serialize_field("restartEpoch", &self.restart_epoch)?;
        }
        if self.cpuset_threads {
            struct_ser.serialize_field("cpusetThreads", &self.cpuset_threads)?;
        }
        if !self.disabled_extensions.is_empty() {
            struct_ser.serialize_field("disabledExtensions", &self.disabled_extensions)?;
        }
        if self.enable_fine_grain_logging {
            struct_ser.serialize_field("enableFineGrainLogging", &self.enable_fine_grain_logging)?;
        }
        if !self.socket_path.is_empty() {
            struct_ser.serialize_field("socketPath", &self.socket_path)?;
        }
        if self.socket_mode != 0 {
            struct_ser.serialize_field("socketMode", &self.socket_mode)?;
        }
        if self.enable_core_dump {
            struct_ser.serialize_field("enableCoreDump", &self.enable_core_dump)?;
        }
        if !self.stats_tag.is_empty() {
            struct_ser.serialize_field("statsTag", &self.stats_tag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommandLineOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_id",
            "baseId",
            "use_dynamic_base_id",
            "useDynamicBaseId",
            "base_id_path",
            "baseIdPath",
            "concurrency",
            "config_path",
            "configPath",
            "config_yaml",
            "configYaml",
            "allow_unknown_static_fields",
            "allowUnknownStaticFields",
            "reject_unknown_dynamic_fields",
            "rejectUnknownDynamicFields",
            "ignore_unknown_dynamic_fields",
            "ignoreUnknownDynamicFields",
            "admin_address_path",
            "adminAddressPath",
            "local_address_ip_version",
            "localAddressIpVersion",
            "log_level",
            "logLevel",
            "component_log_level",
            "componentLogLevel",
            "log_format",
            "logFormat",
            "log_format_escaped",
            "logFormatEscaped",
            "log_path",
            "logPath",
            "service_cluster",
            "serviceCluster",
            "service_node",
            "serviceNode",
            "service_zone",
            "serviceZone",
            "file_flush_interval",
            "fileFlushInterval",
            "drain_time",
            "drainTime",
            "drain_strategy",
            "drainStrategy",
            "parent_shutdown_time",
            "parentShutdownTime",
            "mode",
            "disable_hot_restart",
            "disableHotRestart",
            "enable_mutex_tracing",
            "enableMutexTracing",
            "restart_epoch",
            "restartEpoch",
            "cpuset_threads",
            "cpusetThreads",
            "disabled_extensions",
            "disabledExtensions",
            "enable_fine_grain_logging",
            "enableFineGrainLogging",
            "socket_path",
            "socketPath",
            "socket_mode",
            "socketMode",
            "enable_core_dump",
            "enableCoreDump",
            "stats_tag",
            "statsTag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseId,
            UseDynamicBaseId,
            BaseIdPath,
            Concurrency,
            ConfigPath,
            ConfigYaml,
            AllowUnknownStaticFields,
            RejectUnknownDynamicFields,
            IgnoreUnknownDynamicFields,
            AdminAddressPath,
            LocalAddressIpVersion,
            LogLevel,
            ComponentLogLevel,
            LogFormat,
            LogFormatEscaped,
            LogPath,
            ServiceCluster,
            ServiceNode,
            ServiceZone,
            FileFlushInterval,
            DrainTime,
            DrainStrategy,
            ParentShutdownTime,
            Mode,
            DisableHotRestart,
            EnableMutexTracing,
            RestartEpoch,
            CpusetThreads,
            DisabledExtensions,
            EnableFineGrainLogging,
            SocketPath,
            SocketMode,
            EnableCoreDump,
            StatsTag,
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
                            "baseId" | "base_id" => Ok(GeneratedField::BaseId),
                            "useDynamicBaseId" | "use_dynamic_base_id" => Ok(GeneratedField::UseDynamicBaseId),
                            "baseIdPath" | "base_id_path" => Ok(GeneratedField::BaseIdPath),
                            "concurrency" => Ok(GeneratedField::Concurrency),
                            "configPath" | "config_path" => Ok(GeneratedField::ConfigPath),
                            "configYaml" | "config_yaml" => Ok(GeneratedField::ConfigYaml),
                            "allowUnknownStaticFields" | "allow_unknown_static_fields" => Ok(GeneratedField::AllowUnknownStaticFields),
                            "rejectUnknownDynamicFields" | "reject_unknown_dynamic_fields" => Ok(GeneratedField::RejectUnknownDynamicFields),
                            "ignoreUnknownDynamicFields" | "ignore_unknown_dynamic_fields" => Ok(GeneratedField::IgnoreUnknownDynamicFields),
                            "adminAddressPath" | "admin_address_path" => Ok(GeneratedField::AdminAddressPath),
                            "localAddressIpVersion" | "local_address_ip_version" => Ok(GeneratedField::LocalAddressIpVersion),
                            "logLevel" | "log_level" => Ok(GeneratedField::LogLevel),
                            "componentLogLevel" | "component_log_level" => Ok(GeneratedField::ComponentLogLevel),
                            "logFormat" | "log_format" => Ok(GeneratedField::LogFormat),
                            "logFormatEscaped" | "log_format_escaped" => Ok(GeneratedField::LogFormatEscaped),
                            "logPath" | "log_path" => Ok(GeneratedField::LogPath),
                            "serviceCluster" | "service_cluster" => Ok(GeneratedField::ServiceCluster),
                            "serviceNode" | "service_node" => Ok(GeneratedField::ServiceNode),
                            "serviceZone" | "service_zone" => Ok(GeneratedField::ServiceZone),
                            "fileFlushInterval" | "file_flush_interval" => Ok(GeneratedField::FileFlushInterval),
                            "drainTime" | "drain_time" => Ok(GeneratedField::DrainTime),
                            "drainStrategy" | "drain_strategy" => Ok(GeneratedField::DrainStrategy),
                            "parentShutdownTime" | "parent_shutdown_time" => Ok(GeneratedField::ParentShutdownTime),
                            "mode" => Ok(GeneratedField::Mode),
                            "disableHotRestart" | "disable_hot_restart" => Ok(GeneratedField::DisableHotRestart),
                            "enableMutexTracing" | "enable_mutex_tracing" => Ok(GeneratedField::EnableMutexTracing),
                            "restartEpoch" | "restart_epoch" => Ok(GeneratedField::RestartEpoch),
                            "cpusetThreads" | "cpuset_threads" => Ok(GeneratedField::CpusetThreads),
                            "disabledExtensions" | "disabled_extensions" => Ok(GeneratedField::DisabledExtensions),
                            "enableFineGrainLogging" | "enable_fine_grain_logging" => Ok(GeneratedField::EnableFineGrainLogging),
                            "socketPath" | "socket_path" => Ok(GeneratedField::SocketPath),
                            "socketMode" | "socket_mode" => Ok(GeneratedField::SocketMode),
                            "enableCoreDump" | "enable_core_dump" => Ok(GeneratedField::EnableCoreDump),
                            "statsTag" | "stats_tag" => Ok(GeneratedField::StatsTag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommandLineOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.CommandLineOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommandLineOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut base_id__ = None;
                let mut use_dynamic_base_id__ = None;
                let mut base_id_path__ = None;
                let mut concurrency__ = None;
                let mut config_path__ = None;
                let mut config_yaml__ = None;
                let mut allow_unknown_static_fields__ = None;
                let mut reject_unknown_dynamic_fields__ = None;
                let mut ignore_unknown_dynamic_fields__ = None;
                let mut admin_address_path__ = None;
                let mut local_address_ip_version__ = None;
                let mut log_level__ = None;
                let mut component_log_level__ = None;
                let mut log_format__ = None;
                let mut log_format_escaped__ = None;
                let mut log_path__ = None;
                let mut service_cluster__ = None;
                let mut service_node__ = None;
                let mut service_zone__ = None;
                let mut file_flush_interval__ = None;
                let mut drain_time__ = None;
                let mut drain_strategy__ = None;
                let mut parent_shutdown_time__ = None;
                let mut mode__ = None;
                let mut disable_hot_restart__ = None;
                let mut enable_mutex_tracing__ = None;
                let mut restart_epoch__ = None;
                let mut cpuset_threads__ = None;
                let mut disabled_extensions__ = None;
                let mut enable_fine_grain_logging__ = None;
                let mut socket_path__ = None;
                let mut socket_mode__ = None;
                let mut enable_core_dump__ = None;
                let mut stats_tag__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BaseId => {
                            if base_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseId"));
                            }
                            base_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UseDynamicBaseId => {
                            if use_dynamic_base_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useDynamicBaseId"));
                            }
                            use_dynamic_base_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::BaseIdPath => {
                            if base_id_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseIdPath"));
                            }
                            base_id_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Concurrency => {
                            if concurrency__.is_some() {
                                return Err(serde::de::Error::duplicate_field("concurrency"));
                            }
                            concurrency__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ConfigPath => {
                            if config_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configPath"));
                            }
                            config_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConfigYaml => {
                            if config_yaml__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configYaml"));
                            }
                            config_yaml__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowUnknownStaticFields => {
                            if allow_unknown_static_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowUnknownStaticFields"));
                            }
                            allow_unknown_static_fields__ = Some(map.next_value()?);
                        }
                        GeneratedField::RejectUnknownDynamicFields => {
                            if reject_unknown_dynamic_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectUnknownDynamicFields"));
                            }
                            reject_unknown_dynamic_fields__ = Some(map.next_value()?);
                        }
                        GeneratedField::IgnoreUnknownDynamicFields => {
                            if ignore_unknown_dynamic_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreUnknownDynamicFields"));
                            }
                            ignore_unknown_dynamic_fields__ = Some(map.next_value()?);
                        }
                        GeneratedField::AdminAddressPath => {
                            if admin_address_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminAddressPath"));
                            }
                            admin_address_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalAddressIpVersion => {
                            if local_address_ip_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localAddressIpVersion"));
                            }
                            local_address_ip_version__ = Some(map.next_value::<command_line_options::IpVersion>()? as i32);
                        }
                        GeneratedField::LogLevel => {
                            if log_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logLevel"));
                            }
                            log_level__ = Some(map.next_value()?);
                        }
                        GeneratedField::ComponentLogLevel => {
                            if component_log_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("componentLogLevel"));
                            }
                            component_log_level__ = Some(map.next_value()?);
                        }
                        GeneratedField::LogFormat => {
                            if log_format__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logFormat"));
                            }
                            log_format__ = Some(map.next_value()?);
                        }
                        GeneratedField::LogFormatEscaped => {
                            if log_format_escaped__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logFormatEscaped"));
                            }
                            log_format_escaped__ = Some(map.next_value()?);
                        }
                        GeneratedField::LogPath => {
                            if log_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("logPath"));
                            }
                            log_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServiceCluster => {
                            if service_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceCluster"));
                            }
                            service_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServiceNode => {
                            if service_node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceNode"));
                            }
                            service_node__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServiceZone => {
                            if service_zone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceZone"));
                            }
                            service_zone__ = Some(map.next_value()?);
                        }
                        GeneratedField::FileFlushInterval => {
                            if file_flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileFlushInterval"));
                            }
                            file_flush_interval__ = map.next_value()?;
                        }
                        GeneratedField::DrainTime => {
                            if drain_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainTime"));
                            }
                            drain_time__ = map.next_value()?;
                        }
                        GeneratedField::DrainStrategy => {
                            if drain_strategy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainStrategy"));
                            }
                            drain_strategy__ = Some(map.next_value::<command_line_options::DrainStrategy>()? as i32);
                        }
                        GeneratedField::ParentShutdownTime => {
                            if parent_shutdown_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentShutdownTime"));
                            }
                            parent_shutdown_time__ = map.next_value()?;
                        }
                        GeneratedField::Mode => {
                            if mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            mode__ = Some(map.next_value::<command_line_options::Mode>()? as i32);
                        }
                        GeneratedField::DisableHotRestart => {
                            if disable_hot_restart__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableHotRestart"));
                            }
                            disable_hot_restart__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnableMutexTracing => {
                            if enable_mutex_tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableMutexTracing"));
                            }
                            enable_mutex_tracing__ = Some(map.next_value()?);
                        }
                        GeneratedField::RestartEpoch => {
                            if restart_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restartEpoch"));
                            }
                            restart_epoch__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CpusetThreads => {
                            if cpuset_threads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpusetThreads"));
                            }
                            cpuset_threads__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisabledExtensions => {
                            if disabled_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabledExtensions"));
                            }
                            disabled_extensions__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnableFineGrainLogging => {
                            if enable_fine_grain_logging__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableFineGrainLogging"));
                            }
                            enable_fine_grain_logging__ = Some(map.next_value()?);
                        }
                        GeneratedField::SocketPath => {
                            if socket_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketPath"));
                            }
                            socket_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::SocketMode => {
                            if socket_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("socketMode"));
                            }
                            socket_mode__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EnableCoreDump => {
                            if enable_core_dump__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableCoreDump"));
                            }
                            enable_core_dump__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatsTag => {
                            if stats_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statsTag"));
                            }
                            stats_tag__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CommandLineOptions {
                    base_id: base_id__.unwrap_or_default(),
                    use_dynamic_base_id: use_dynamic_base_id__.unwrap_or_default(),
                    base_id_path: base_id_path__.unwrap_or_default(),
                    concurrency: concurrency__.unwrap_or_default(),
                    config_path: config_path__.unwrap_or_default(),
                    config_yaml: config_yaml__.unwrap_or_default(),
                    allow_unknown_static_fields: allow_unknown_static_fields__.unwrap_or_default(),
                    reject_unknown_dynamic_fields: reject_unknown_dynamic_fields__.unwrap_or_default(),
                    ignore_unknown_dynamic_fields: ignore_unknown_dynamic_fields__.unwrap_or_default(),
                    admin_address_path: admin_address_path__.unwrap_or_default(),
                    local_address_ip_version: local_address_ip_version__.unwrap_or_default(),
                    log_level: log_level__.unwrap_or_default(),
                    component_log_level: component_log_level__.unwrap_or_default(),
                    log_format: log_format__.unwrap_or_default(),
                    log_format_escaped: log_format_escaped__.unwrap_or_default(),
                    log_path: log_path__.unwrap_or_default(),
                    service_cluster: service_cluster__.unwrap_or_default(),
                    service_node: service_node__.unwrap_or_default(),
                    service_zone: service_zone__.unwrap_or_default(),
                    file_flush_interval: file_flush_interval__,
                    drain_time: drain_time__,
                    drain_strategy: drain_strategy__.unwrap_or_default(),
                    parent_shutdown_time: parent_shutdown_time__,
                    mode: mode__.unwrap_or_default(),
                    disable_hot_restart: disable_hot_restart__.unwrap_or_default(),
                    enable_mutex_tracing: enable_mutex_tracing__.unwrap_or_default(),
                    restart_epoch: restart_epoch__.unwrap_or_default(),
                    cpuset_threads: cpuset_threads__.unwrap_or_default(),
                    disabled_extensions: disabled_extensions__.unwrap_or_default(),
                    enable_fine_grain_logging: enable_fine_grain_logging__.unwrap_or_default(),
                    socket_path: socket_path__.unwrap_or_default(),
                    socket_mode: socket_mode__.unwrap_or_default(),
                    enable_core_dump: enable_core_dump__.unwrap_or_default(),
                    stats_tag: stats_tag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.CommandLineOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for command_line_options::DrainStrategy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Gradual => "Gradual",
            Self::Immediate => "Immediate",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for command_line_options::DrainStrategy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Gradual",
            "Immediate",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = command_line_options::DrainStrategy;

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
                    .and_then(command_line_options::DrainStrategy::from_i32)
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
                    .and_then(command_line_options::DrainStrategy::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Gradual" => Ok(command_line_options::DrainStrategy::Gradual),
                    "Immediate" => Ok(command_line_options::DrainStrategy::Immediate),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for command_line_options::IpVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::V4 => "v4",
            Self::V6 => "v6",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for command_line_options::IpVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "v4",
            "v6",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = command_line_options::IpVersion;

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
                    .and_then(command_line_options::IpVersion::from_i32)
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
                    .and_then(command_line_options::IpVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "v4" => Ok(command_line_options::IpVersion::V4),
                    "v6" => Ok(command_line_options::IpVersion::V6),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for command_line_options::Mode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Serve => "Serve",
            Self::Validate => "Validate",
            Self::InitOnly => "InitOnly",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for command_line_options::Mode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Serve",
            "Validate",
            "InitOnly",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = command_line_options::Mode;

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
                    .and_then(command_line_options::Mode::from_i32)
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
                    .and_then(command_line_options::Mode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Serve" => Ok(command_line_options::Mode::Serve),
                    "Validate" => Ok(command_line_options::Mode::Validate),
                    "InitOnly" => Ok(command_line_options::Mode::InitOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ConfigDump", len)?;
        if !self.configs.is_empty() {
            struct_ser.serialize_field("configs", &self.configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "configs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Configs,
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
                            "configs" => Ok(GeneratedField::Configs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut configs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Configs => {
                            if configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configs"));
                            }
                            configs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ConfigDump {
                    configs: configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EcdsConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.ecds_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EcdsConfigDump", len)?;
        if !self.ecds_filters.is_empty() {
            struct_ser.serialize_field("ecdsFilters", &self.ecds_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EcdsConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ecds_filters",
            "ecdsFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EcdsFilters,
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
                            "ecdsFilters" | "ecds_filters" => Ok(GeneratedField::EcdsFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EcdsConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EcdsConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EcdsConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ecds_filters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EcdsFilters => {
                            if ecds_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdsFilters"));
                            }
                            ecds_filters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EcdsConfigDump {
                    ecds_filters: ecds_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EcdsConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ecds_config_dump::EcdsFilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.ecds_filter.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.ecds_filter.as_ref() {
            struct_ser.serialize_field("ecdsFilter", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ecds_config_dump::EcdsFilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "ecds_filter",
            "ecdsFilter",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            EcdsFilter,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "ecdsFilter" | "ecds_filter" => Ok(GeneratedField::EcdsFilter),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ecds_config_dump::EcdsFilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ecds_config_dump::EcdsFilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut ecds_filter__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::EcdsFilter => {
                            if ecds_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ecdsFilter"));
                            }
                            ecds_filter__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(ecds_config_dump::EcdsFilterConfig {
                    version_info: version_info__.unwrap_or_default(),
                    ecds_filter: ecds_filter__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EcdsConfigDump.EcdsFilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EndpointsConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.static_endpoint_configs.is_empty() {
            len += 1;
        }
        if !self.dynamic_endpoint_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EndpointsConfigDump", len)?;
        if !self.static_endpoint_configs.is_empty() {
            struct_ser.serialize_field("staticEndpointConfigs", &self.static_endpoint_configs)?;
        }
        if !self.dynamic_endpoint_configs.is_empty() {
            struct_ser.serialize_field("dynamicEndpointConfigs", &self.dynamic_endpoint_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EndpointsConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "static_endpoint_configs",
            "staticEndpointConfigs",
            "dynamic_endpoint_configs",
            "dynamicEndpointConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaticEndpointConfigs,
            DynamicEndpointConfigs,
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
                            "staticEndpointConfigs" | "static_endpoint_configs" => Ok(GeneratedField::StaticEndpointConfigs),
                            "dynamicEndpointConfigs" | "dynamic_endpoint_configs" => Ok(GeneratedField::DynamicEndpointConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EndpointsConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EndpointsConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EndpointsConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut static_endpoint_configs__ = None;
                let mut dynamic_endpoint_configs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StaticEndpointConfigs => {
                            if static_endpoint_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticEndpointConfigs"));
                            }
                            static_endpoint_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicEndpointConfigs => {
                            if dynamic_endpoint_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicEndpointConfigs"));
                            }
                            dynamic_endpoint_configs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EndpointsConfigDump {
                    static_endpoint_configs: static_endpoint_configs__.unwrap_or_default(),
                    dynamic_endpoint_configs: dynamic_endpoint_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EndpointsConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for endpoints_config_dump::DynamicEndpointConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.endpoint_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.endpoint_config.as_ref() {
            struct_ser.serialize_field("endpointConfig", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for endpoints_config_dump::DynamicEndpointConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "endpoint_config",
            "endpointConfig",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            EndpointConfig,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "endpointConfig" | "endpoint_config" => Ok(GeneratedField::EndpointConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = endpoints_config_dump::DynamicEndpointConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<endpoints_config_dump::DynamicEndpointConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut endpoint_config__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::EndpointConfig => {
                            if endpoint_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointConfig"));
                            }
                            endpoint_config__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(endpoints_config_dump::DynamicEndpointConfig {
                    version_info: version_info__.unwrap_or_default(),
                    endpoint_config: endpoint_config__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EndpointsConfigDump.DynamicEndpointConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for endpoints_config_dump::StaticEndpointConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.endpoint_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig", len)?;
        if let Some(v) = self.endpoint_config.as_ref() {
            struct_ser.serialize_field("endpointConfig", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for endpoints_config_dump::StaticEndpointConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoint_config",
            "endpointConfig",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EndpointConfig,
            LastUpdated,
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
                            "endpointConfig" | "endpoint_config" => Ok(GeneratedField::EndpointConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = endpoints_config_dump::StaticEndpointConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<endpoints_config_dump::StaticEndpointConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoint_config__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EndpointConfig => {
                            if endpoint_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointConfig"));
                            }
                            endpoint_config__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                    }
                }
                Ok(endpoints_config_dump::StaticEndpointConfig {
                    endpoint_config: endpoint_config__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.EndpointsConfigDump.StaticEndpointConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostHealthStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failed_active_health_check {
            len += 1;
        }
        if self.failed_outlier_check {
            len += 1;
        }
        if self.failed_active_degraded_check {
            len += 1;
        }
        if self.pending_dynamic_removal {
            len += 1;
        }
        if self.pending_active_hc {
            len += 1;
        }
        if self.excluded_via_immediate_hc_fail {
            len += 1;
        }
        if self.active_hc_timeout {
            len += 1;
        }
        if self.eds_health_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.HostHealthStatus", len)?;
        if self.failed_active_health_check {
            struct_ser.serialize_field("failedActiveHealthCheck", &self.failed_active_health_check)?;
        }
        if self.failed_outlier_check {
            struct_ser.serialize_field("failedOutlierCheck", &self.failed_outlier_check)?;
        }
        if self.failed_active_degraded_check {
            struct_ser.serialize_field("failedActiveDegradedCheck", &self.failed_active_degraded_check)?;
        }
        if self.pending_dynamic_removal {
            struct_ser.serialize_field("pendingDynamicRemoval", &self.pending_dynamic_removal)?;
        }
        if self.pending_active_hc {
            struct_ser.serialize_field("pendingActiveHc", &self.pending_active_hc)?;
        }
        if self.excluded_via_immediate_hc_fail {
            struct_ser.serialize_field("excludedViaImmediateHcFail", &self.excluded_via_immediate_hc_fail)?;
        }
        if self.active_hc_timeout {
            struct_ser.serialize_field("activeHcTimeout", &self.active_hc_timeout)?;
        }
        if self.eds_health_status != 0 {
            let v = super::super::config::core::v3::HealthStatus::from_i32(self.eds_health_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.eds_health_status)))?;
            struct_ser.serialize_field("edsHealthStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostHealthStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failed_active_health_check",
            "failedActiveHealthCheck",
            "failed_outlier_check",
            "failedOutlierCheck",
            "failed_active_degraded_check",
            "failedActiveDegradedCheck",
            "pending_dynamic_removal",
            "pendingDynamicRemoval",
            "pending_active_hc",
            "pendingActiveHc",
            "excluded_via_immediate_hc_fail",
            "excludedViaImmediateHcFail",
            "active_hc_timeout",
            "activeHcTimeout",
            "eds_health_status",
            "edsHealthStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailedActiveHealthCheck,
            FailedOutlierCheck,
            FailedActiveDegradedCheck,
            PendingDynamicRemoval,
            PendingActiveHc,
            ExcludedViaImmediateHcFail,
            ActiveHcTimeout,
            EdsHealthStatus,
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
                            "failedActiveHealthCheck" | "failed_active_health_check" => Ok(GeneratedField::FailedActiveHealthCheck),
                            "failedOutlierCheck" | "failed_outlier_check" => Ok(GeneratedField::FailedOutlierCheck),
                            "failedActiveDegradedCheck" | "failed_active_degraded_check" => Ok(GeneratedField::FailedActiveDegradedCheck),
                            "pendingDynamicRemoval" | "pending_dynamic_removal" => Ok(GeneratedField::PendingDynamicRemoval),
                            "pendingActiveHc" | "pending_active_hc" => Ok(GeneratedField::PendingActiveHc),
                            "excludedViaImmediateHcFail" | "excluded_via_immediate_hc_fail" => Ok(GeneratedField::ExcludedViaImmediateHcFail),
                            "activeHcTimeout" | "active_hc_timeout" => Ok(GeneratedField::ActiveHcTimeout),
                            "edsHealthStatus" | "eds_health_status" => Ok(GeneratedField::EdsHealthStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostHealthStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.HostHealthStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HostHealthStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failed_active_health_check__ = None;
                let mut failed_outlier_check__ = None;
                let mut failed_active_degraded_check__ = None;
                let mut pending_dynamic_removal__ = None;
                let mut pending_active_hc__ = None;
                let mut excluded_via_immediate_hc_fail__ = None;
                let mut active_hc_timeout__ = None;
                let mut eds_health_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FailedActiveHealthCheck => {
                            if failed_active_health_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedActiveHealthCheck"));
                            }
                            failed_active_health_check__ = Some(map.next_value()?);
                        }
                        GeneratedField::FailedOutlierCheck => {
                            if failed_outlier_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedOutlierCheck"));
                            }
                            failed_outlier_check__ = Some(map.next_value()?);
                        }
                        GeneratedField::FailedActiveDegradedCheck => {
                            if failed_active_degraded_check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedActiveDegradedCheck"));
                            }
                            failed_active_degraded_check__ = Some(map.next_value()?);
                        }
                        GeneratedField::PendingDynamicRemoval => {
                            if pending_dynamic_removal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingDynamicRemoval"));
                            }
                            pending_dynamic_removal__ = Some(map.next_value()?);
                        }
                        GeneratedField::PendingActiveHc => {
                            if pending_active_hc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pendingActiveHc"));
                            }
                            pending_active_hc__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExcludedViaImmediateHcFail => {
                            if excluded_via_immediate_hc_fail__.is_some() {
                                return Err(serde::de::Error::duplicate_field("excludedViaImmediateHcFail"));
                            }
                            excluded_via_immediate_hc_fail__ = Some(map.next_value()?);
                        }
                        GeneratedField::ActiveHcTimeout => {
                            if active_hc_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeHcTimeout"));
                            }
                            active_hc_timeout__ = Some(map.next_value()?);
                        }
                        GeneratedField::EdsHealthStatus => {
                            if eds_health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("edsHealthStatus"));
                            }
                            eds_health_status__ = Some(map.next_value::<super::super::config::core::v3::HealthStatus>()? as i32);
                        }
                    }
                }
                Ok(HostHealthStatus {
                    failed_active_health_check: failed_active_health_check__.unwrap_or_default(),
                    failed_outlier_check: failed_outlier_check__.unwrap_or_default(),
                    failed_active_degraded_check: failed_active_degraded_check__.unwrap_or_default(),
                    pending_dynamic_removal: pending_dynamic_removal__.unwrap_or_default(),
                    pending_active_hc: pending_active_hc__.unwrap_or_default(),
                    excluded_via_immediate_hc_fail: excluded_via_immediate_hc_fail__.unwrap_or_default(),
                    active_hc_timeout: active_hc_timeout__.unwrap_or_default(),
                    eds_health_status: eds_health_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.HostHealthStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HostStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.address.is_some() {
            len += 1;
        }
        if !self.stats.is_empty() {
            len += 1;
        }
        if self.health_status.is_some() {
            len += 1;
        }
        if self.success_rate.is_some() {
            len += 1;
        }
        if self.weight != 0 {
            len += 1;
        }
        if !self.hostname.is_empty() {
            len += 1;
        }
        if self.priority != 0 {
            len += 1;
        }
        if self.local_origin_success_rate.is_some() {
            len += 1;
        }
        if self.locality.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.HostStatus", len)?;
        if let Some(v) = self.address.as_ref() {
            struct_ser.serialize_field("address", v)?;
        }
        if !self.stats.is_empty() {
            struct_ser.serialize_field("stats", &self.stats)?;
        }
        if let Some(v) = self.health_status.as_ref() {
            struct_ser.serialize_field("healthStatus", v)?;
        }
        if let Some(v) = self.success_rate.as_ref() {
            struct_ser.serialize_field("successRate", v)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if !self.hostname.is_empty() {
            struct_ser.serialize_field("hostname", &self.hostname)?;
        }
        if self.priority != 0 {
            struct_ser.serialize_field("priority", &self.priority)?;
        }
        if let Some(v) = self.local_origin_success_rate.as_ref() {
            struct_ser.serialize_field("localOriginSuccessRate", v)?;
        }
        if let Some(v) = self.locality.as_ref() {
            struct_ser.serialize_field("locality", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "stats",
            "health_status",
            "healthStatus",
            "success_rate",
            "successRate",
            "weight",
            "hostname",
            "priority",
            "local_origin_success_rate",
            "localOriginSuccessRate",
            "locality",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Stats,
            HealthStatus,
            SuccessRate,
            Weight,
            Hostname,
            Priority,
            LocalOriginSuccessRate,
            Locality,
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
                            "address" => Ok(GeneratedField::Address),
                            "stats" => Ok(GeneratedField::Stats),
                            "healthStatus" | "health_status" => Ok(GeneratedField::HealthStatus),
                            "successRate" | "success_rate" => Ok(GeneratedField::SuccessRate),
                            "weight" => Ok(GeneratedField::Weight),
                            "hostname" => Ok(GeneratedField::Hostname),
                            "priority" => Ok(GeneratedField::Priority),
                            "localOriginSuccessRate" | "local_origin_success_rate" => Ok(GeneratedField::LocalOriginSuccessRate),
                            "locality" => Ok(GeneratedField::Locality),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.HostStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HostStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut stats__ = None;
                let mut health_status__ = None;
                let mut success_rate__ = None;
                let mut weight__ = None;
                let mut hostname__ = None;
                let mut priority__ = None;
                let mut local_origin_success_rate__ = None;
                let mut locality__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = map.next_value()?;
                        }
                        GeneratedField::Stats => {
                            if stats__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            stats__ = Some(map.next_value()?);
                        }
                        GeneratedField::HealthStatus => {
                            if health_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("healthStatus"));
                            }
                            health_status__ = map.next_value()?;
                        }
                        GeneratedField::SuccessRate => {
                            if success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("successRate"));
                            }
                            success_rate__ = map.next_value()?;
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hostname => {
                            if hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostname"));
                            }
                            hostname__ = Some(map.next_value()?);
                        }
                        GeneratedField::Priority => {
                            if priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            priority__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LocalOriginSuccessRate => {
                            if local_origin_success_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localOriginSuccessRate"));
                            }
                            local_origin_success_rate__ = map.next_value()?;
                        }
                        GeneratedField::Locality => {
                            if locality__.is_some() {
                                return Err(serde::de::Error::duplicate_field("locality"));
                            }
                            locality__ = map.next_value()?;
                        }
                    }
                }
                Ok(HostStatus {
                    address: address__,
                    stats: stats__.unwrap_or_default(),
                    health_status: health_status__,
                    success_rate: success_rate__,
                    weight: weight__.unwrap_or_default(),
                    hostname: hostname__.unwrap_or_default(),
                    priority: priority__.unwrap_or_default(),
                    local_origin_success_rate: local_origin_success_rate__,
                    locality: locality__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.HostStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenerStatus {
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
        if self.local_address.is_some() {
            len += 1;
        }
        if !self.additional_local_addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenerStatus", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.local_address.as_ref() {
            struct_ser.serialize_field("localAddress", v)?;
        }
        if !self.additional_local_addresses.is_empty() {
            struct_ser.serialize_field("additionalLocalAddresses", &self.additional_local_addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenerStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "local_address",
            "localAddress",
            "additional_local_addresses",
            "additionalLocalAddresses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            LocalAddress,
            AdditionalLocalAddresses,
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
                            "localAddress" | "local_address" => Ok(GeneratedField::LocalAddress),
                            "additionalLocalAddresses" | "additional_local_addresses" => Ok(GeneratedField::AdditionalLocalAddresses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListenerStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenerStatus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListenerStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut local_address__ = None;
                let mut additional_local_addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalAddress => {
                            if local_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localAddress"));
                            }
                            local_address__ = map.next_value()?;
                        }
                        GeneratedField::AdditionalLocalAddresses => {
                            if additional_local_addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("additionalLocalAddresses"));
                            }
                            additional_local_addresses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListenerStatus {
                    name: name__.unwrap_or_default(),
                    local_address: local_address__,
                    additional_local_addresses: additional_local_addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenerStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Listeners {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.listener_statuses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.Listeners", len)?;
        if !self.listener_statuses.is_empty() {
            struct_ser.serialize_field("listenerStatuses", &self.listener_statuses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Listeners {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "listener_statuses",
            "listenerStatuses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ListenerStatuses,
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
                            "listenerStatuses" | "listener_statuses" => Ok(GeneratedField::ListenerStatuses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Listeners;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.Listeners")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Listeners, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut listener_statuses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ListenerStatuses => {
                            if listener_statuses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listenerStatuses"));
                            }
                            listener_statuses__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Listeners {
                    listener_statuses: listener_statuses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.Listeners", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListenersConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.static_listeners.is_empty() {
            len += 1;
        }
        if !self.dynamic_listeners.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if !self.static_listeners.is_empty() {
            struct_ser.serialize_field("staticListeners", &self.static_listeners)?;
        }
        if !self.dynamic_listeners.is_empty() {
            struct_ser.serialize_field("dynamicListeners", &self.dynamic_listeners)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListenersConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "static_listeners",
            "staticListeners",
            "dynamic_listeners",
            "dynamicListeners",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            StaticListeners,
            DynamicListeners,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "staticListeners" | "static_listeners" => Ok(GeneratedField::StaticListeners),
                            "dynamicListeners" | "dynamic_listeners" => Ok(GeneratedField::DynamicListeners),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListenersConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListenersConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut static_listeners__ = None;
                let mut dynamic_listeners__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::StaticListeners => {
                            if static_listeners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticListeners"));
                            }
                            static_listeners__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicListeners => {
                            if dynamic_listeners__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicListeners"));
                            }
                            dynamic_listeners__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ListenersConfigDump {
                    version_info: version_info__.unwrap_or_default(),
                    static_listeners: static_listeners__.unwrap_or_default(),
                    dynamic_listeners: dynamic_listeners__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listeners_config_dump::DynamicListener {
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
        if self.active_state.is_some() {
            len += 1;
        }
        if self.warming_state.is_some() {
            len += 1;
        }
        if self.draining_state.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListener", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.active_state.as_ref() {
            struct_ser.serialize_field("activeState", v)?;
        }
        if let Some(v) = self.warming_state.as_ref() {
            struct_ser.serialize_field("warmingState", v)?;
        }
        if let Some(v) = self.draining_state.as_ref() {
            struct_ser.serialize_field("drainingState", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listeners_config_dump::DynamicListener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "active_state",
            "activeState",
            "warming_state",
            "warmingState",
            "draining_state",
            "drainingState",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ActiveState,
            WarmingState,
            DrainingState,
            ErrorState,
            ClientStatus,
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
                            "activeState" | "active_state" => Ok(GeneratedField::ActiveState),
                            "warmingState" | "warming_state" => Ok(GeneratedField::WarmingState),
                            "drainingState" | "draining_state" => Ok(GeneratedField::DrainingState),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listeners_config_dump::DynamicListener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump.DynamicListener")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<listeners_config_dump::DynamicListener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut active_state__ = None;
                let mut warming_state__ = None;
                let mut draining_state__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ActiveState => {
                            if active_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("activeState"));
                            }
                            active_state__ = map.next_value()?;
                        }
                        GeneratedField::WarmingState => {
                            if warming_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("warmingState"));
                            }
                            warming_state__ = map.next_value()?;
                        }
                        GeneratedField::DrainingState => {
                            if draining_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainingState"));
                            }
                            draining_state__ = map.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(listeners_config_dump::DynamicListener {
                    name: name__.unwrap_or_default(),
                    active_state: active_state__,
                    warming_state: warming_state__,
                    draining_state: draining_state__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListener", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listeners_config_dump::DynamicListenerState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.listener.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListenerState", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.listener.as_ref() {
            struct_ser.serialize_field("listener", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listeners_config_dump::DynamicListenerState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "listener",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            Listener,
            LastUpdated,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "listener" => Ok(GeneratedField::Listener),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listeners_config_dump::DynamicListenerState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump.DynamicListenerState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<listeners_config_dump::DynamicListenerState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut listener__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::Listener => {
                            if listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listener"));
                            }
                            listener__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                    }
                }
                Ok(listeners_config_dump::DynamicListenerState {
                    version_info: version_info__.unwrap_or_default(),
                    listener: listener__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump.DynamicListenerState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for listeners_config_dump::StaticListener {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.listener.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ListenersConfigDump.StaticListener", len)?;
        if let Some(v) = self.listener.as_ref() {
            struct_ser.serialize_field("listener", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for listeners_config_dump::StaticListener {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "listener",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Listener,
            LastUpdated,
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
                            "listener" => Ok(GeneratedField::Listener),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = listeners_config_dump::StaticListener;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ListenersConfigDump.StaticListener")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<listeners_config_dump::StaticListener, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut listener__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Listener => {
                            if listener__.is_some() {
                                return Err(serde::de::Error::duplicate_field("listener"));
                            }
                            listener__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                    }
                }
                Ok(listeners_config_dump::StaticListener {
                    listener: listener__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ListenersConfigDump.StaticListener", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Memory {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allocated != 0 {
            len += 1;
        }
        if self.heap_size != 0 {
            len += 1;
        }
        if self.pageheap_unmapped != 0 {
            len += 1;
        }
        if self.pageheap_free != 0 {
            len += 1;
        }
        if self.total_thread_cache != 0 {
            len += 1;
        }
        if self.total_physical_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.Memory", len)?;
        if self.allocated != 0 {
            struct_ser.serialize_field("allocated", ToString::to_string(&self.allocated).as_str())?;
        }
        if self.heap_size != 0 {
            struct_ser.serialize_field("heapSize", ToString::to_string(&self.heap_size).as_str())?;
        }
        if self.pageheap_unmapped != 0 {
            struct_ser.serialize_field("pageheapUnmapped", ToString::to_string(&self.pageheap_unmapped).as_str())?;
        }
        if self.pageheap_free != 0 {
            struct_ser.serialize_field("pageheapFree", ToString::to_string(&self.pageheap_free).as_str())?;
        }
        if self.total_thread_cache != 0 {
            struct_ser.serialize_field("totalThreadCache", ToString::to_string(&self.total_thread_cache).as_str())?;
        }
        if self.total_physical_bytes != 0 {
            struct_ser.serialize_field("totalPhysicalBytes", ToString::to_string(&self.total_physical_bytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Memory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allocated",
            "heap_size",
            "heapSize",
            "pageheap_unmapped",
            "pageheapUnmapped",
            "pageheap_free",
            "pageheapFree",
            "total_thread_cache",
            "totalThreadCache",
            "total_physical_bytes",
            "totalPhysicalBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Allocated,
            HeapSize,
            PageheapUnmapped,
            PageheapFree,
            TotalThreadCache,
            TotalPhysicalBytes,
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
                            "allocated" => Ok(GeneratedField::Allocated),
                            "heapSize" | "heap_size" => Ok(GeneratedField::HeapSize),
                            "pageheapUnmapped" | "pageheap_unmapped" => Ok(GeneratedField::PageheapUnmapped),
                            "pageheapFree" | "pageheap_free" => Ok(GeneratedField::PageheapFree),
                            "totalThreadCache" | "total_thread_cache" => Ok(GeneratedField::TotalThreadCache),
                            "totalPhysicalBytes" | "total_physical_bytes" => Ok(GeneratedField::TotalPhysicalBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Memory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.Memory")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Memory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allocated__ = None;
                let mut heap_size__ = None;
                let mut pageheap_unmapped__ = None;
                let mut pageheap_free__ = None;
                let mut total_thread_cache__ = None;
                let mut total_physical_bytes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Allocated => {
                            if allocated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocated"));
                            }
                            allocated__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HeapSize => {
                            if heap_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("heapSize"));
                            }
                            heap_size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageheapUnmapped => {
                            if pageheap_unmapped__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageheapUnmapped"));
                            }
                            pageheap_unmapped__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageheapFree => {
                            if pageheap_free__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageheapFree"));
                            }
                            pageheap_free__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalThreadCache => {
                            if total_thread_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalThreadCache"));
                            }
                            total_thread_cache__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalPhysicalBytes => {
                            if total_physical_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalPhysicalBytes"));
                            }
                            total_physical_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Memory {
                    allocated: allocated__.unwrap_or_default(),
                    heap_size: heap_size__.unwrap_or_default(),
                    pageheap_unmapped: pageheap_unmapped__.unwrap_or_default(),
                    pageheap_free: pageheap_free__.unwrap_or_default(),
                    total_thread_cache: total_thread_cache__.unwrap_or_default(),
                    total_physical_bytes: total_physical_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.Memory", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MutexStats {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.num_contentions != 0 {
            len += 1;
        }
        if self.current_wait_cycles != 0 {
            len += 1;
        }
        if self.lifetime_wait_cycles != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.MutexStats", len)?;
        if self.num_contentions != 0 {
            struct_ser.serialize_field("numContentions", ToString::to_string(&self.num_contentions).as_str())?;
        }
        if self.current_wait_cycles != 0 {
            struct_ser.serialize_field("currentWaitCycles", ToString::to_string(&self.current_wait_cycles).as_str())?;
        }
        if self.lifetime_wait_cycles != 0 {
            struct_ser.serialize_field("lifetimeWaitCycles", ToString::to_string(&self.lifetime_wait_cycles).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MutexStats {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "num_contentions",
            "numContentions",
            "current_wait_cycles",
            "currentWaitCycles",
            "lifetime_wait_cycles",
            "lifetimeWaitCycles",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NumContentions,
            CurrentWaitCycles,
            LifetimeWaitCycles,
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
                            "numContentions" | "num_contentions" => Ok(GeneratedField::NumContentions),
                            "currentWaitCycles" | "current_wait_cycles" => Ok(GeneratedField::CurrentWaitCycles),
                            "lifetimeWaitCycles" | "lifetime_wait_cycles" => Ok(GeneratedField::LifetimeWaitCycles),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MutexStats;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.MutexStats")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MutexStats, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut num_contentions__ = None;
                let mut current_wait_cycles__ = None;
                let mut lifetime_wait_cycles__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NumContentions => {
                            if num_contentions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numContentions"));
                            }
                            num_contentions__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CurrentWaitCycles => {
                            if current_wait_cycles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentWaitCycles"));
                            }
                            current_wait_cycles__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LifetimeWaitCycles => {
                            if lifetime_wait_cycles__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifetimeWaitCycles"));
                            }
                            lifetime_wait_cycles__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MutexStats {
                    num_contentions: num_contentions__.unwrap_or_default(),
                    current_wait_cycles: current_wait_cycles__.unwrap_or_default(),
                    lifetime_wait_cycles: lifetime_wait_cycles__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.MutexStats", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RoutesConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.static_route_configs.is_empty() {
            len += 1;
        }
        if !self.dynamic_route_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.RoutesConfigDump", len)?;
        if !self.static_route_configs.is_empty() {
            struct_ser.serialize_field("staticRouteConfigs", &self.static_route_configs)?;
        }
        if !self.dynamic_route_configs.is_empty() {
            struct_ser.serialize_field("dynamicRouteConfigs", &self.dynamic_route_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RoutesConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "static_route_configs",
            "staticRouteConfigs",
            "dynamic_route_configs",
            "dynamicRouteConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaticRouteConfigs,
            DynamicRouteConfigs,
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
                            "staticRouteConfigs" | "static_route_configs" => Ok(GeneratedField::StaticRouteConfigs),
                            "dynamicRouteConfigs" | "dynamic_route_configs" => Ok(GeneratedField::DynamicRouteConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RoutesConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.RoutesConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RoutesConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut static_route_configs__ = None;
                let mut dynamic_route_configs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StaticRouteConfigs => {
                            if static_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticRouteConfigs"));
                            }
                            static_route_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicRouteConfigs => {
                            if dynamic_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicRouteConfigs"));
                            }
                            dynamic_route_configs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RoutesConfigDump {
                    static_route_configs: static_route_configs__.unwrap_or_default(),
                    dynamic_route_configs: dynamic_route_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.RoutesConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for routes_config_dump::DynamicRouteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.route_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig", len)?;
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.route_config.as_ref() {
            struct_ser.serialize_field("routeConfig", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for routes_config_dump::DynamicRouteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version_info",
            "versionInfo",
            "route_config",
            "routeConfig",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersionInfo,
            RouteConfig,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = routes_config_dump::DynamicRouteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<routes_config_dump::DynamicRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version_info__ = None;
                let mut route_config__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::RouteConfig => {
                            if route_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_config__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(routes_config_dump::DynamicRouteConfig {
                    version_info: version_info__.unwrap_or_default(),
                    route_config: route_config__,
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.RoutesConfigDump.DynamicRouteConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for routes_config_dump::StaticRouteConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.route_config.is_some() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.RoutesConfigDump.StaticRouteConfig", len)?;
        if let Some(v) = self.route_config.as_ref() {
            struct_ser.serialize_field("routeConfig", v)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for routes_config_dump::StaticRouteConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "route_config",
            "routeConfig",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RouteConfig,
            LastUpdated,
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
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = routes_config_dump::StaticRouteConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.RoutesConfigDump.StaticRouteConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<routes_config_dump::StaticRouteConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut route_config__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RouteConfig => {
                            if route_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_config__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                    }
                }
                Ok(routes_config_dump::StaticRouteConfig {
                    route_config: route_config__,
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.RoutesConfigDump.StaticRouteConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRoutesConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.inline_scoped_route_configs.is_empty() {
            len += 1;
        }
        if !self.dynamic_scoped_route_configs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ScopedRoutesConfigDump", len)?;
        if !self.inline_scoped_route_configs.is_empty() {
            struct_ser.serialize_field("inlineScopedRouteConfigs", &self.inline_scoped_route_configs)?;
        }
        if !self.dynamic_scoped_route_configs.is_empty() {
            struct_ser.serialize_field("dynamicScopedRouteConfigs", &self.dynamic_scoped_route_configs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRoutesConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "inline_scoped_route_configs",
            "inlineScopedRouteConfigs",
            "dynamic_scoped_route_configs",
            "dynamicScopedRouteConfigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InlineScopedRouteConfigs,
            DynamicScopedRouteConfigs,
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
                            "inlineScopedRouteConfigs" | "inline_scoped_route_configs" => Ok(GeneratedField::InlineScopedRouteConfigs),
                            "dynamicScopedRouteConfigs" | "dynamic_scoped_route_configs" => Ok(GeneratedField::DynamicScopedRouteConfigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRoutesConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ScopedRoutesConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScopedRoutesConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut inline_scoped_route_configs__ = None;
                let mut dynamic_scoped_route_configs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::InlineScopedRouteConfigs => {
                            if inline_scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inlineScopedRouteConfigs"));
                            }
                            inline_scoped_route_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicScopedRouteConfigs => {
                            if dynamic_scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicScopedRouteConfigs"));
                            }
                            dynamic_scoped_route_configs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ScopedRoutesConfigDump {
                    inline_scoped_route_configs: inline_scoped_route_configs__.unwrap_or_default(),
                    dynamic_scoped_route_configs: dynamic_scoped_route_configs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ScopedRoutesConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes_config_dump::DynamicScopedRouteConfigs {
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
        if !self.version_info.is_empty() {
            len += 1;
        }
        if !self.scoped_route_configs.is_empty() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if !self.scoped_route_configs.is_empty() {
            struct_ser.serialize_field("scopedRouteConfigs", &self.scoped_route_configs)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes_config_dump::DynamicScopedRouteConfigs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version_info",
            "versionInfo",
            "scoped_route_configs",
            "scopedRouteConfigs",
            "last_updated",
            "lastUpdated",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            VersionInfo,
            ScopedRouteConfigs,
            LastUpdated,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "scopedRouteConfigs" | "scoped_route_configs" => Ok(GeneratedField::ScopedRouteConfigs),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes_config_dump::DynamicScopedRouteConfigs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_routes_config_dump::DynamicScopedRouteConfigs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version_info__ = None;
                let mut scoped_route_configs__ = None;
                let mut last_updated__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::ScopedRouteConfigs => {
                            if scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigs"));
                            }
                            scoped_route_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(scoped_routes_config_dump::DynamicScopedRouteConfigs {
                    name: name__.unwrap_or_default(),
                    version_info: version_info__.unwrap_or_default(),
                    scoped_route_configs: scoped_route_configs__.unwrap_or_default(),
                    last_updated: last_updated__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.DynamicScopedRouteConfigs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes_config_dump::InlineScopedRouteConfigs {
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
        if !self.scoped_route_configs.is_empty() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.scoped_route_configs.is_empty() {
            struct_ser.serialize_field("scopedRouteConfigs", &self.scoped_route_configs)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes_config_dump::InlineScopedRouteConfigs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "scoped_route_configs",
            "scopedRouteConfigs",
            "last_updated",
            "lastUpdated",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ScopedRouteConfigs,
            LastUpdated,
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
                            "scopedRouteConfigs" | "scoped_route_configs" => Ok(GeneratedField::ScopedRouteConfigs),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes_config_dump::InlineScopedRouteConfigs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_routes_config_dump::InlineScopedRouteConfigs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut scoped_route_configs__ = None;
                let mut last_updated__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ScopedRouteConfigs => {
                            if scoped_route_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigs"));
                            }
                            scoped_route_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                    }
                }
                Ok(scoped_routes_config_dump::InlineScopedRouteConfigs {
                    name: name__.unwrap_or_default(),
                    scoped_route_configs: scoped_route_configs__.unwrap_or_default(),
                    last_updated: last_updated__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ScopedRoutesConfigDump.InlineScopedRouteConfigs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SecretsConfigDump {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.static_secrets.is_empty() {
            len += 1;
        }
        if !self.dynamic_active_secrets.is_empty() {
            len += 1;
        }
        if !self.dynamic_warming_secrets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.SecretsConfigDump", len)?;
        if !self.static_secrets.is_empty() {
            struct_ser.serialize_field("staticSecrets", &self.static_secrets)?;
        }
        if !self.dynamic_active_secrets.is_empty() {
            struct_ser.serialize_field("dynamicActiveSecrets", &self.dynamic_active_secrets)?;
        }
        if !self.dynamic_warming_secrets.is_empty() {
            struct_ser.serialize_field("dynamicWarmingSecrets", &self.dynamic_warming_secrets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SecretsConfigDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "static_secrets",
            "staticSecrets",
            "dynamic_active_secrets",
            "dynamicActiveSecrets",
            "dynamic_warming_secrets",
            "dynamicWarmingSecrets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StaticSecrets,
            DynamicActiveSecrets,
            DynamicWarmingSecrets,
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
                            "staticSecrets" | "static_secrets" => Ok(GeneratedField::StaticSecrets),
                            "dynamicActiveSecrets" | "dynamic_active_secrets" => Ok(GeneratedField::DynamicActiveSecrets),
                            "dynamicWarmingSecrets" | "dynamic_warming_secrets" => Ok(GeneratedField::DynamicWarmingSecrets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SecretsConfigDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.SecretsConfigDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SecretsConfigDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut static_secrets__ = None;
                let mut dynamic_active_secrets__ = None;
                let mut dynamic_warming_secrets__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StaticSecrets => {
                            if static_secrets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("staticSecrets"));
                            }
                            static_secrets__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicActiveSecrets => {
                            if dynamic_active_secrets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicActiveSecrets"));
                            }
                            dynamic_active_secrets__ = Some(map.next_value()?);
                        }
                        GeneratedField::DynamicWarmingSecrets => {
                            if dynamic_warming_secrets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dynamicWarmingSecrets"));
                            }
                            dynamic_warming_secrets__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SecretsConfigDump {
                    static_secrets: static_secrets__.unwrap_or_default(),
                    dynamic_active_secrets: dynamic_active_secrets__.unwrap_or_default(),
                    dynamic_warming_secrets: dynamic_warming_secrets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.SecretsConfigDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for secrets_config_dump::DynamicSecret {
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
        if !self.version_info.is_empty() {
            len += 1;
        }
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.secret.is_some() {
            len += 1;
        }
        if self.error_state.is_some() {
            len += 1;
        }
        if self.client_status != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.SecretsConfigDump.DynamicSecret", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if let Some(v) = self.secret.as_ref() {
            struct_ser.serialize_field("secret", v)?;
        }
        if let Some(v) = self.error_state.as_ref() {
            struct_ser.serialize_field("errorState", v)?;
        }
        if self.client_status != 0 {
            let v = ClientResourceStatus::from_i32(self.client_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.client_status)))?;
            struct_ser.serialize_field("clientStatus", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for secrets_config_dump::DynamicSecret {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version_info",
            "versionInfo",
            "last_updated",
            "lastUpdated",
            "secret",
            "error_state",
            "errorState",
            "client_status",
            "clientStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            VersionInfo,
            LastUpdated,
            Secret,
            ErrorState,
            ClientStatus,
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
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
                            "secret" => Ok(GeneratedField::Secret),
                            "errorState" | "error_state" => Ok(GeneratedField::ErrorState),
                            "clientStatus" | "client_status" => Ok(GeneratedField::ClientStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = secrets_config_dump::DynamicSecret;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.SecretsConfigDump.DynamicSecret")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<secrets_config_dump::DynamicSecret, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version_info__ = None;
                let mut last_updated__ = None;
                let mut secret__ = None;
                let mut error_state__ = None;
                let mut client_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = map.next_value()?;
                        }
                        GeneratedField::ErrorState => {
                            if error_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorState"));
                            }
                            error_state__ = map.next_value()?;
                        }
                        GeneratedField::ClientStatus => {
                            if client_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientStatus"));
                            }
                            client_status__ = Some(map.next_value::<ClientResourceStatus>()? as i32);
                        }
                    }
                }
                Ok(secrets_config_dump::DynamicSecret {
                    name: name__.unwrap_or_default(),
                    version_info: version_info__.unwrap_or_default(),
                    last_updated: last_updated__,
                    secret: secret__,
                    error_state: error_state__,
                    client_status: client_status__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.SecretsConfigDump.DynamicSecret", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for secrets_config_dump::StaticSecret {
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
        if self.last_updated.is_some() {
            len += 1;
        }
        if self.secret.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.SecretsConfigDump.StaticSecret", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.last_updated.as_ref() {
            struct_ser.serialize_field("lastUpdated", v)?;
        }
        if let Some(v) = self.secret.as_ref() {
            struct_ser.serialize_field("secret", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for secrets_config_dump::StaticSecret {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "last_updated",
            "lastUpdated",
            "secret",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            LastUpdated,
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
                            "name" => Ok(GeneratedField::Name),
                            "lastUpdated" | "last_updated" => Ok(GeneratedField::LastUpdated),
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
            type Value = secrets_config_dump::StaticSecret;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.SecretsConfigDump.StaticSecret")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<secrets_config_dump::StaticSecret, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut last_updated__ = None;
                let mut secret__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastUpdated => {
                            if last_updated__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdated"));
                            }
                            last_updated__ = map.next_value()?;
                        }
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = map.next_value()?;
                        }
                    }
                }
                Ok(secrets_config_dump::StaticSecret {
                    name: name__.unwrap_or_default(),
                    last_updated: last_updated__,
                    secret: secret__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.SecretsConfigDump.StaticSecret", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.uptime_current_epoch.is_some() {
            len += 1;
        }
        if self.uptime_all_epochs.is_some() {
            len += 1;
        }
        if !self.hot_restart_version.is_empty() {
            len += 1;
        }
        if self.command_line_options.is_some() {
            len += 1;
        }
        if self.node.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.ServerInfo", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if self.state != 0 {
            let v = server_info::State::from_i32(self.state)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.uptime_current_epoch.as_ref() {
            struct_ser.serialize_field("uptimeCurrentEpoch", v)?;
        }
        if let Some(v) = self.uptime_all_epochs.as_ref() {
            struct_ser.serialize_field("uptimeAllEpochs", v)?;
        }
        if !self.hot_restart_version.is_empty() {
            struct_ser.serialize_field("hotRestartVersion", &self.hot_restart_version)?;
        }
        if let Some(v) = self.command_line_options.as_ref() {
            struct_ser.serialize_field("commandLineOptions", v)?;
        }
        if let Some(v) = self.node.as_ref() {
            struct_ser.serialize_field("node", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "state",
            "uptime_current_epoch",
            "uptimeCurrentEpoch",
            "uptime_all_epochs",
            "uptimeAllEpochs",
            "hot_restart_version",
            "hotRestartVersion",
            "command_line_options",
            "commandLineOptions",
            "node",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            State,
            UptimeCurrentEpoch,
            UptimeAllEpochs,
            HotRestartVersion,
            CommandLineOptions,
            Node,
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
                            "version" => Ok(GeneratedField::Version),
                            "state" => Ok(GeneratedField::State),
                            "uptimeCurrentEpoch" | "uptime_current_epoch" => Ok(GeneratedField::UptimeCurrentEpoch),
                            "uptimeAllEpochs" | "uptime_all_epochs" => Ok(GeneratedField::UptimeAllEpochs),
                            "hotRestartVersion" | "hot_restart_version" => Ok(GeneratedField::HotRestartVersion),
                            "commandLineOptions" | "command_line_options" => Ok(GeneratedField::CommandLineOptions),
                            "node" => Ok(GeneratedField::Node),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.ServerInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut state__ = None;
                let mut uptime_current_epoch__ = None;
                let mut uptime_all_epochs__ = None;
                let mut hot_restart_version__ = None;
                let mut command_line_options__ = None;
                let mut node__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map.next_value::<server_info::State>()? as i32);
                        }
                        GeneratedField::UptimeCurrentEpoch => {
                            if uptime_current_epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uptimeCurrentEpoch"));
                            }
                            uptime_current_epoch__ = map.next_value()?;
                        }
                        GeneratedField::UptimeAllEpochs => {
                            if uptime_all_epochs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uptimeAllEpochs"));
                            }
                            uptime_all_epochs__ = map.next_value()?;
                        }
                        GeneratedField::HotRestartVersion => {
                            if hot_restart_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hotRestartVersion"));
                            }
                            hot_restart_version__ = Some(map.next_value()?);
                        }
                        GeneratedField::CommandLineOptions => {
                            if command_line_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commandLineOptions"));
                            }
                            command_line_options__ = map.next_value()?;
                        }
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = map.next_value()?;
                        }
                    }
                }
                Ok(ServerInfo {
                    version: version__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    uptime_current_epoch: uptime_current_epoch__,
                    uptime_all_epochs: uptime_all_epochs__,
                    hot_restart_version: hot_restart_version__.unwrap_or_default(),
                    command_line_options: command_line_options__,
                    node: node__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.ServerInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for server_info::State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Live => "LIVE",
            Self::Draining => "DRAINING",
            Self::PreInitializing => "PRE_INITIALIZING",
            Self::Initializing => "INITIALIZING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for server_info::State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "LIVE",
            "DRAINING",
            "PRE_INITIALIZING",
            "INITIALIZING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = server_info::State;

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
                    .and_then(server_info::State::from_i32)
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
                    .and_then(server_info::State::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "LIVE" => Ok(server_info::State::Live),
                    "DRAINING" => Ok(server_info::State::Draining),
                    "PRE_INITIALIZING" => Ok(server_info::State::PreInitializing),
                    "INITIALIZING" => Ok(server_info::State::Initializing),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SimpleMetric {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.value != 0 {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.SimpleMetric", len)?;
        if self.r#type != 0 {
            let v = simple_metric::Type::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.value != 0 {
            struct_ser.serialize_field("value", ToString::to_string(&self.value).as_str())?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SimpleMetric {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "value",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Value,
            Name,
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
                            "type" => Ok(GeneratedField::Type),
                            "value" => Ok(GeneratedField::Value),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SimpleMetric;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.SimpleMetric")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SimpleMetric, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut value__ = None;
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<simple_metric::Type>()? as i32);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(SimpleMetric {
                    r#type: r#type__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.SimpleMetric", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for simple_metric::Type {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Counter => "COUNTER",
            Self::Gauge => "GAUGE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for simple_metric::Type {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "COUNTER",
            "GAUGE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = simple_metric::Type;

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
                    .and_then(simple_metric::Type::from_i32)
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
                    .and_then(simple_metric::Type::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "COUNTER" => Ok(simple_metric::Type::Counter),
                    "GAUGE" => Ok(simple_metric::Type::Gauge),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SubjectAlternateName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.SubjectAlternateName", len)?;
        if let Some(v) = self.name.as_ref() {
            match v {
                subject_alternate_name::Name::Dns(v) => {
                    struct_ser.serialize_field("dns", v)?;
                }
                subject_alternate_name::Name::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
                subject_alternate_name::Name::IpAddress(v) => {
                    struct_ser.serialize_field("ipAddress", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SubjectAlternateName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "dns",
            "uri",
            "ip_address",
            "ipAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dns,
            Uri,
            IpAddress,
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
                            "dns" => Ok(GeneratedField::Dns),
                            "uri" => Ok(GeneratedField::Uri),
                            "ipAddress" | "ip_address" => Ok(GeneratedField::IpAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SubjectAlternateName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.SubjectAlternateName")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SubjectAlternateName, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Dns => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dns"));
                            }
                            name__ = map.next_value::<::std::option::Option<_>>()?.map(subject_alternate_name::Name::Dns);
                        }
                        GeneratedField::Uri => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            name__ = map.next_value::<::std::option::Option<_>>()?.map(subject_alternate_name::Name::Uri);
                        }
                        GeneratedField::IpAddress => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipAddress"));
                            }
                            name__ = map.next_value::<::std::option::Option<_>>()?.map(subject_alternate_name::Name::IpAddress);
                        }
                    }
                }
                Ok(SubjectAlternateName {
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.SubjectAlternateName", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TapRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.config_id.is_empty() {
            len += 1;
        }
        if self.tap_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.TapRequest", len)?;
        if !self.config_id.is_empty() {
            struct_ser.serialize_field("configId", &self.config_id)?;
        }
        if let Some(v) = self.tap_config.as_ref() {
            struct_ser.serialize_field("tapConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TapRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_id",
            "configId",
            "tap_config",
            "tapConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigId,
            TapConfig,
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
                            "configId" | "config_id" => Ok(GeneratedField::ConfigId),
                            "tapConfig" | "tap_config" => Ok(GeneratedField::TapConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TapRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.TapRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TapRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_id__ = None;
                let mut tap_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfigId => {
                            if config_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configId"));
                            }
                            config_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::TapConfig => {
                            if tap_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tapConfig"));
                            }
                            tap_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(TapRequest {
                    config_id: config_id__.unwrap_or_default(),
                    tap_config: tap_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.TapRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UnreadyTargetsDumps {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.unready_targets_dumps.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.UnreadyTargetsDumps", len)?;
        if !self.unready_targets_dumps.is_empty() {
            struct_ser.serialize_field("unreadyTargetsDumps", &self.unready_targets_dumps)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnreadyTargetsDumps {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unready_targets_dumps",
            "unreadyTargetsDumps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnreadyTargetsDumps,
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
                            "unreadyTargetsDumps" | "unready_targets_dumps" => Ok(GeneratedField::UnreadyTargetsDumps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnreadyTargetsDumps;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.UnreadyTargetsDumps")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UnreadyTargetsDumps, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unready_targets_dumps__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UnreadyTargetsDumps => {
                            if unready_targets_dumps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unreadyTargetsDumps"));
                            }
                            unready_targets_dumps__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UnreadyTargetsDumps {
                    unready_targets_dumps: unready_targets_dumps__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.UnreadyTargetsDumps", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for unready_targets_dumps::UnreadyTargetsDump {
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
        if !self.target_names.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.UnreadyTargetsDumps.UnreadyTargetsDump", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.target_names.is_empty() {
            struct_ser.serialize_field("targetNames", &self.target_names)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for unready_targets_dumps::UnreadyTargetsDump {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "target_names",
            "targetNames",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            TargetNames,
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
                            "targetNames" | "target_names" => Ok(GeneratedField::TargetNames),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = unready_targets_dumps::UnreadyTargetsDump;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.UnreadyTargetsDumps.UnreadyTargetsDump")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<unready_targets_dumps::UnreadyTargetsDump, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut target_names__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::TargetNames => {
                            if target_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetNames"));
                            }
                            target_names__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(unready_targets_dumps::UnreadyTargetsDump {
                    name: name__.unwrap_or_default(),
                    target_names: target_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.UnreadyTargetsDumps.UnreadyTargetsDump", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateFailureState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failed_configuration.is_some() {
            len += 1;
        }
        if self.last_update_attempt.is_some() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        if !self.version_info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.admin.v3.UpdateFailureState", len)?;
        if let Some(v) = self.failed_configuration.as_ref() {
            struct_ser.serialize_field("failedConfiguration", v)?;
        }
        if let Some(v) = self.last_update_attempt.as_ref() {
            struct_ser.serialize_field("lastUpdateAttempt", v)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        if !self.version_info.is_empty() {
            struct_ser.serialize_field("versionInfo", &self.version_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateFailureState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failed_configuration",
            "failedConfiguration",
            "last_update_attempt",
            "lastUpdateAttempt",
            "details",
            "version_info",
            "versionInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailedConfiguration,
            LastUpdateAttempt,
            Details,
            VersionInfo,
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
                            "failedConfiguration" | "failed_configuration" => Ok(GeneratedField::FailedConfiguration),
                            "lastUpdateAttempt" | "last_update_attempt" => Ok(GeneratedField::LastUpdateAttempt),
                            "details" => Ok(GeneratedField::Details),
                            "versionInfo" | "version_info" => Ok(GeneratedField::VersionInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateFailureState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.admin.v3.UpdateFailureState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateFailureState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failed_configuration__ = None;
                let mut last_update_attempt__ = None;
                let mut details__ = None;
                let mut version_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FailedConfiguration => {
                            if failed_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedConfiguration"));
                            }
                            failed_configuration__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdateAttempt => {
                            if last_update_attempt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdateAttempt"));
                            }
                            last_update_attempt__ = map.next_value()?;
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(map.next_value()?);
                        }
                        GeneratedField::VersionInfo => {
                            if version_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versionInfo"));
                            }
                            version_info__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateFailureState {
                    failed_configuration: failed_configuration__,
                    last_update_attempt: last_update_attempt__,
                    details: details__.unwrap_or_default(),
                    version_info: version_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.admin.v3.UpdateFailureState", FIELDS, GeneratedVisitor)
    }
}
