// @generated
impl serde::Serialize for AwsRequestSigning {
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
        if !self.host_rewrite.is_empty() {
            len += 1;
        }
        if self.use_unsigned_payload {
            len += 1;
        }
        if !self.match_excluded_headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.aws_request_signing.v3.AwsRequestSigning", len)?;
        if !self.service_name.is_empty() {
            struct_ser.serialize_field("serviceName", &self.service_name)?;
        }
        if !self.region.is_empty() {
            struct_ser.serialize_field("region", &self.region)?;
        }
        if !self.host_rewrite.is_empty() {
            struct_ser.serialize_field("hostRewrite", &self.host_rewrite)?;
        }
        if self.use_unsigned_payload {
            struct_ser.serialize_field("useUnsignedPayload", &self.use_unsigned_payload)?;
        }
        if !self.match_excluded_headers.is_empty() {
            struct_ser.serialize_field("matchExcludedHeaders", &self.match_excluded_headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AwsRequestSigning {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service_name",
            "serviceName",
            "region",
            "host_rewrite",
            "hostRewrite",
            "use_unsigned_payload",
            "useUnsignedPayload",
            "match_excluded_headers",
            "matchExcludedHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServiceName,
            Region,
            HostRewrite,
            UseUnsignedPayload,
            MatchExcludedHeaders,
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
                            "hostRewrite" | "host_rewrite" => Ok(GeneratedField::HostRewrite),
                            "useUnsignedPayload" | "use_unsigned_payload" => Ok(GeneratedField::UseUnsignedPayload),
                            "matchExcludedHeaders" | "match_excluded_headers" => Ok(GeneratedField::MatchExcludedHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AwsRequestSigning;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.aws_request_signing.v3.AwsRequestSigning")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AwsRequestSigning, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service_name__ = None;
                let mut region__ = None;
                let mut host_rewrite__ = None;
                let mut use_unsigned_payload__ = None;
                let mut match_excluded_headers__ = None;
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
                        GeneratedField::HostRewrite => {
                            if host_rewrite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostRewrite"));
                            }
                            host_rewrite__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseUnsignedPayload => {
                            if use_unsigned_payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useUnsignedPayload"));
                            }
                            use_unsigned_payload__ = Some(map.next_value()?);
                        }
                        GeneratedField::MatchExcludedHeaders => {
                            if match_excluded_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchExcludedHeaders"));
                            }
                            match_excluded_headers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AwsRequestSigning {
                    service_name: service_name__.unwrap_or_default(),
                    region: region__.unwrap_or_default(),
                    host_rewrite: host_rewrite__.unwrap_or_default(),
                    use_unsigned_payload: use_unsigned_payload__.unwrap_or_default(),
                    match_excluded_headers: match_excluded_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.aws_request_signing.v3.AwsRequestSigning", FIELDS, GeneratedVisitor)
    }
}
