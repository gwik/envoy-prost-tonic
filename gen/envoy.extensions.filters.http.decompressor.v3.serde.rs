// @generated
impl serde::Serialize for Decompressor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.decompressor_library.is_some() {
            len += 1;
        }
        if self.request_direction_config.is_some() {
            len += 1;
        }
        if self.response_direction_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor", len)?;
        if let Some(v) = self.decompressor_library.as_ref() {
            struct_ser.serialize_field("decompressorLibrary", v)?;
        }
        if let Some(v) = self.request_direction_config.as_ref() {
            struct_ser.serialize_field("requestDirectionConfig", v)?;
        }
        if let Some(v) = self.response_direction_config.as_ref() {
            struct_ser.serialize_field("responseDirectionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Decompressor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "decompressor_library",
            "decompressorLibrary",
            "request_direction_config",
            "requestDirectionConfig",
            "response_direction_config",
            "responseDirectionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DecompressorLibrary,
            RequestDirectionConfig,
            ResponseDirectionConfig,
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
                            "decompressorLibrary" | "decompressor_library" => Ok(GeneratedField::DecompressorLibrary),
                            "requestDirectionConfig" | "request_direction_config" => Ok(GeneratedField::RequestDirectionConfig),
                            "responseDirectionConfig" | "response_direction_config" => Ok(GeneratedField::ResponseDirectionConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Decompressor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.decompressor.v3.Decompressor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Decompressor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut decompressor_library__ = None;
                let mut request_direction_config__ = None;
                let mut response_direction_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DecompressorLibrary => {
                            if decompressor_library__.is_some() {
                                return Err(serde::de::Error::duplicate_field("decompressorLibrary"));
                            }
                            decompressor_library__ = map.next_value()?;
                        }
                        GeneratedField::RequestDirectionConfig => {
                            if request_direction_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestDirectionConfig"));
                            }
                            request_direction_config__ = map.next_value()?;
                        }
                        GeneratedField::ResponseDirectionConfig => {
                            if response_direction_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseDirectionConfig"));
                            }
                            response_direction_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(Decompressor {
                    decompressor_library: decompressor_library__,
                    request_direction_config: request_direction_config__,
                    response_direction_config: response_direction_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for decompressor::CommonDirectionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled.is_some() {
            len += 1;
        }
        if self.ignore_no_transform_header {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor.CommonDirectionConfig", len)?;
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        if self.ignore_no_transform_header {
            struct_ser.serialize_field("ignoreNoTransformHeader", &self.ignore_no_transform_header)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for decompressor::CommonDirectionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
            "ignore_no_transform_header",
            "ignoreNoTransformHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            IgnoreNoTransformHeader,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            "ignoreNoTransformHeader" | "ignore_no_transform_header" => Ok(GeneratedField::IgnoreNoTransformHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = decompressor::CommonDirectionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.decompressor.v3.Decompressor.CommonDirectionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<decompressor::CommonDirectionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut ignore_no_transform_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map.next_value()?;
                        }
                        GeneratedField::IgnoreNoTransformHeader => {
                            if ignore_no_transform_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreNoTransformHeader"));
                            }
                            ignore_no_transform_header__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(decompressor::CommonDirectionConfig {
                    enabled: enabled__,
                    ignore_no_transform_header: ignore_no_transform_header__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor.CommonDirectionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for decompressor::RequestDirectionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_config.is_some() {
            len += 1;
        }
        if self.advertise_accept_encoding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor.RequestDirectionConfig", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        if let Some(v) = self.advertise_accept_encoding.as_ref() {
            struct_ser.serialize_field("advertiseAcceptEncoding", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for decompressor::RequestDirectionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_config",
            "commonConfig",
            "advertise_accept_encoding",
            "advertiseAcceptEncoding",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonConfig,
            AdvertiseAcceptEncoding,
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
                            "commonConfig" | "common_config" => Ok(GeneratedField::CommonConfig),
                            "advertiseAcceptEncoding" | "advertise_accept_encoding" => Ok(GeneratedField::AdvertiseAcceptEncoding),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = decompressor::RequestDirectionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.decompressor.v3.Decompressor.RequestDirectionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<decompressor::RequestDirectionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_config__ = None;
                let mut advertise_accept_encoding__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonConfig => {
                            if common_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonConfig"));
                            }
                            common_config__ = map.next_value()?;
                        }
                        GeneratedField::AdvertiseAcceptEncoding => {
                            if advertise_accept_encoding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("advertiseAcceptEncoding"));
                            }
                            advertise_accept_encoding__ = map.next_value()?;
                        }
                    }
                }
                Ok(decompressor::RequestDirectionConfig {
                    common_config: common_config__,
                    advertise_accept_encoding: advertise_accept_encoding__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor.RequestDirectionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for decompressor::ResponseDirectionConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor.ResponseDirectionConfig", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for decompressor::ResponseDirectionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_config",
            "commonConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonConfig,
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
                            "commonConfig" | "common_config" => Ok(GeneratedField::CommonConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = decompressor::ResponseDirectionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.decompressor.v3.Decompressor.ResponseDirectionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<decompressor::ResponseDirectionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonConfig => {
                            if common_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonConfig"));
                            }
                            common_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(decompressor::ResponseDirectionConfig {
                    common_config: common_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.decompressor.v3.Decompressor.ResponseDirectionConfig", FIELDS, GeneratedVisitor)
    }
}
