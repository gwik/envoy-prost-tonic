// @generated
impl serde::Serialize for AwsIamConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service_name.is_empty() {
            len += 1;
        }
        if !self.region.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.grpc_credential.v2alpha.AwsIamConfig", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.region.is_empty() {
            struct_ser.serialize_field("region", &self.region)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AwsIamConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "region",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            Region,
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
                            "serviceName" | "service_name" => Ok(GeneratedField::ServiceName),
                            "region" => Ok(GeneratedField::Region),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AwsIamConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.grpc_credential.v2alpha.AwsIamConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AwsIamConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut region__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServiceName => {
                            if service_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceName"));
                            }
                            service_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Region => {
                            if region__.is_some() {
                                return Err(serde::de::Error::duplicate_field("region"));
                            }
                            region__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AwsIamConfig {
                    service_name: service_name__.unwrap_or_default(),
                    region: region__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.grpc_credential.v2alpha.AwsIamConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileBasedMetadataConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.secret_data.is_some() {
            len += 1;
        }
        if !self.header_key.is_empty() {
            len += 1;
        }
        if !self.header_prefix.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.grpc_credential.v2alpha.FileBasedMetadataConfig", len)?;
        if let Some(v) = self.secret_data.as_ref() {
            struct_ser.serialize_field("secretData", v)?;
        }
        if !self.header_key.is_empty() {
            struct_ser.serialize_field("headerKey", &self.header_key)?;
        }
        if !self.header_prefix.is_empty() {
            struct_ser.serialize_field("headerPrefix", &self.header_prefix)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileBasedMetadataConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "secret_data",
            "secretData",
            "header_key",
            "headerKey",
            "header_prefix",
            "headerPrefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SecretData,
            HeaderKey,
            HeaderPrefix,
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
                            "secretData" | "secret_data" => Ok(GeneratedField::SecretData),
                            "headerKey" | "header_key" => Ok(GeneratedField::HeaderKey),
                            "headerPrefix" | "header_prefix" => Ok(GeneratedField::HeaderPrefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileBasedMetadataConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.grpc_credential.v2alpha.FileBasedMetadataConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileBasedMetadataConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut secret_data__ = None;
                let mut header_key__ = None;
                let mut header_prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SecretData => {
                            if secret_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secretData"));
                            }
                            secret_data__ = map.next_value()?;
                        }
                        GeneratedField::HeaderKey => {
                            if header_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerKey"));
                            }
                            header_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeaderPrefix => {
                            if header_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerPrefix"));
                            }
                            header_prefix__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FileBasedMetadataConfig {
                    secret_data: secret_data__,
                    header_key: header_key__.unwrap_or_default(),
                    header_prefix: header_prefix__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.grpc_credential.v2alpha.FileBasedMetadataConfig", FIELDS, GeneratedVisitor)
    }
}
