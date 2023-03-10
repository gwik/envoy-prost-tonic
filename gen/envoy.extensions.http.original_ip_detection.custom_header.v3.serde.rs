// @generated
impl serde::Serialize for CustomHeaderConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.header_name.is_empty() {
            len += 1;
        }
        if self.allow_extension_to_set_address_as_trusted {
            len += 1;
        }
        if self.reject_with_status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.original_ip_detection.custom_header.v3.CustomHeaderConfig", len)?;
        if !self.header_name.is_empty() {
            struct_ser.serialize_field("headerName", &self.header_name)?;
        }
        if self.allow_extension_to_set_address_as_trusted {
            struct_ser.serialize_field("allowExtensionToSetAddressAsTrusted", &self.allow_extension_to_set_address_as_trusted)?;
        }
        if let Some(v) = self.reject_with_status.as_ref() {
            struct_ser.serialize_field("rejectWithStatus", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CustomHeaderConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_name",
            "headerName",
            "allow_extension_to_set_address_as_trusted",
            "allowExtensionToSetAddressAsTrusted",
            "reject_with_status",
            "rejectWithStatus",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderName,
            AllowExtensionToSetAddressAsTrusted,
            RejectWithStatus,
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
                            "headerName" | "header_name" => Ok(GeneratedField::HeaderName),
                            "allowExtensionToSetAddressAsTrusted" | "allow_extension_to_set_address_as_trusted" => Ok(GeneratedField::AllowExtensionToSetAddressAsTrusted),
                            "rejectWithStatus" | "reject_with_status" => Ok(GeneratedField::RejectWithStatus),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CustomHeaderConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.original_ip_detection.custom_header.v3.CustomHeaderConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CustomHeaderConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header_name__ = None;
                let mut allow_extension_to_set_address_as_trusted__ = None;
                let mut reject_with_status__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HeaderName => {
                            if header_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerName"));
                            }
                            header_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowExtensionToSetAddressAsTrusted => {
                            if allow_extension_to_set_address_as_trusted__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowExtensionToSetAddressAsTrusted"));
                            }
                            allow_extension_to_set_address_as_trusted__ = Some(map.next_value()?);
                        }
                        GeneratedField::RejectWithStatus => {
                            if reject_with_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectWithStatus"));
                            }
                            reject_with_status__ = map.next_value()?;
                        }
                    }
                }
                Ok(CustomHeaderConfig {
                    header_name: header_name__.unwrap_or_default(),
                    allow_extension_to_set_address_as_trusted: allow_extension_to_set_address_as_trusted__.unwrap_or_default(),
                    reject_with_status: reject_with_status__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.original_ip_detection.custom_header.v3.CustomHeaderConfig", FIELDS, GeneratedVisitor)
    }
}
