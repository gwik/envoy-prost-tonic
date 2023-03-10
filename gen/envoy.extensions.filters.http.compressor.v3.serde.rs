// @generated
impl serde::Serialize for Compressor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.content_length.is_some() {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        if self.disable_on_etag_header {
            len += 1;
        }
        if self.remove_accept_encoding_header {
            len += 1;
        }
        if self.runtime_enabled.is_some() {
            len += 1;
        }
        if self.compressor_library.is_some() {
            len += 1;
        }
        if self.request_direction_config.is_some() {
            len += 1;
        }
        if self.response_direction_config.is_some() {
            len += 1;
        }
        if self.choose_first {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor", len)?;
        if let Some(v) = self.content_length.as_ref() {
            struct_ser.serialize_field("contentLength", v)?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        if self.disable_on_etag_header {
            struct_ser.serialize_field("disableOnEtagHeader", &self.disable_on_etag_header)?;
        }
        if self.remove_accept_encoding_header {
            struct_ser.serialize_field("removeAcceptEncodingHeader", &self.remove_accept_encoding_header)?;
        }
        if let Some(v) = self.runtime_enabled.as_ref() {
            struct_ser.serialize_field("runtimeEnabled", v)?;
        }
        if let Some(v) = self.compressor_library.as_ref() {
            struct_ser.serialize_field("compressorLibrary", v)?;
        }
        if let Some(v) = self.request_direction_config.as_ref() {
            struct_ser.serialize_field("requestDirectionConfig", v)?;
        }
        if let Some(v) = self.response_direction_config.as_ref() {
            struct_ser.serialize_field("responseDirectionConfig", v)?;
        }
        if self.choose_first {
            struct_ser.serialize_field("chooseFirst", &self.choose_first)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Compressor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "content_length",
            "contentLength",
            "content_type",
            "contentType",
            "disable_on_etag_header",
            "disableOnEtagHeader",
            "remove_accept_encoding_header",
            "removeAcceptEncodingHeader",
            "runtime_enabled",
            "runtimeEnabled",
            "compressor_library",
            "compressorLibrary",
            "request_direction_config",
            "requestDirectionConfig",
            "response_direction_config",
            "responseDirectionConfig",
            "choose_first",
            "chooseFirst",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContentLength,
            ContentType,
            DisableOnEtagHeader,
            RemoveAcceptEncodingHeader,
            RuntimeEnabled,
            CompressorLibrary,
            RequestDirectionConfig,
            ResponseDirectionConfig,
            ChooseFirst,
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
                            "contentLength" | "content_length" => Ok(GeneratedField::ContentLength),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            "disableOnEtagHeader" | "disable_on_etag_header" => Ok(GeneratedField::DisableOnEtagHeader),
                            "removeAcceptEncodingHeader" | "remove_accept_encoding_header" => Ok(GeneratedField::RemoveAcceptEncodingHeader),
                            "runtimeEnabled" | "runtime_enabled" => Ok(GeneratedField::RuntimeEnabled),
                            "compressorLibrary" | "compressor_library" => Ok(GeneratedField::CompressorLibrary),
                            "requestDirectionConfig" | "request_direction_config" => Ok(GeneratedField::RequestDirectionConfig),
                            "responseDirectionConfig" | "response_direction_config" => Ok(GeneratedField::ResponseDirectionConfig),
                            "chooseFirst" | "choose_first" => Ok(GeneratedField::ChooseFirst),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Compressor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.compressor.v3.Compressor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Compressor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut content_length__ = None;
                let mut content_type__ = None;
                let mut disable_on_etag_header__ = None;
                let mut remove_accept_encoding_header__ = None;
                let mut runtime_enabled__ = None;
                let mut compressor_library__ = None;
                let mut request_direction_config__ = None;
                let mut response_direction_config__ = None;
                let mut choose_first__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContentLength => {
                            if content_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentLength"));
                            }
                            content_length__ = map.next_value()?;
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisableOnEtagHeader => {
                            if disable_on_etag_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableOnEtagHeader"));
                            }
                            disable_on_etag_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoveAcceptEncodingHeader => {
                            if remove_accept_encoding_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeAcceptEncodingHeader"));
                            }
                            remove_accept_encoding_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::RuntimeEnabled => {
                            if runtime_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runtimeEnabled"));
                            }
                            runtime_enabled__ = map.next_value()?;
                        }
                        GeneratedField::CompressorLibrary => {
                            if compressor_library__.is_some() {
                                return Err(serde::de::Error::duplicate_field("compressorLibrary"));
                            }
                            compressor_library__ = map.next_value()?;
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
                        GeneratedField::ChooseFirst => {
                            if choose_first__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chooseFirst"));
                            }
                            choose_first__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Compressor {
                    content_length: content_length__,
                    content_type: content_type__.unwrap_or_default(),
                    disable_on_etag_header: disable_on_etag_header__.unwrap_or_default(),
                    remove_accept_encoding_header: remove_accept_encoding_header__.unwrap_or_default(),
                    runtime_enabled: runtime_enabled__,
                    compressor_library: compressor_library__,
                    request_direction_config: request_direction_config__,
                    response_direction_config: response_direction_config__,
                    choose_first: choose_first__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for compressor::CommonDirectionConfig {
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
        if self.min_content_length.is_some() {
            len += 1;
        }
        if !self.content_type.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor.CommonDirectionConfig", len)?;
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        if let Some(v) = self.min_content_length.as_ref() {
            struct_ser.serialize_field("minContentLength", v)?;
        }
        if !self.content_type.is_empty() {
            struct_ser.serialize_field("contentType", &self.content_type)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for compressor::CommonDirectionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
            "min_content_length",
            "minContentLength",
            "content_type",
            "contentType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
            MinContentLength,
            ContentType,
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
                            "minContentLength" | "min_content_length" => Ok(GeneratedField::MinContentLength),
                            "contentType" | "content_type" => Ok(GeneratedField::ContentType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = compressor::CommonDirectionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.compressor.v3.Compressor.CommonDirectionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<compressor::CommonDirectionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                let mut min_content_length__ = None;
                let mut content_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map.next_value()?;
                        }
                        GeneratedField::MinContentLength => {
                            if min_content_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minContentLength"));
                            }
                            min_content_length__ = map.next_value()?;
                        }
                        GeneratedField::ContentType => {
                            if content_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentType"));
                            }
                            content_type__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(compressor::CommonDirectionConfig {
                    enabled: enabled__,
                    min_content_length: min_content_length__,
                    content_type: content_type__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor.CommonDirectionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for compressor::RequestDirectionConfig {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor.RequestDirectionConfig", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for compressor::RequestDirectionConfig {
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
            type Value = compressor::RequestDirectionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.compressor.v3.Compressor.RequestDirectionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<compressor::RequestDirectionConfig, V::Error>
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
                Ok(compressor::RequestDirectionConfig {
                    common_config: common_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor.RequestDirectionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for compressor::ResponseDirectionConfig {
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
        if self.disable_on_etag_header {
            len += 1;
        }
        if self.remove_accept_encoding_header {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor.ResponseDirectionConfig", len)?;
        if let Some(v) = self.common_config.as_ref() {
            struct_ser.serialize_field("commonConfig", v)?;
        }
        if self.disable_on_etag_header {
            struct_ser.serialize_field("disableOnEtagHeader", &self.disable_on_etag_header)?;
        }
        if self.remove_accept_encoding_header {
            struct_ser.serialize_field("removeAcceptEncodingHeader", &self.remove_accept_encoding_header)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for compressor::ResponseDirectionConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_config",
            "commonConfig",
            "disable_on_etag_header",
            "disableOnEtagHeader",
            "remove_accept_encoding_header",
            "removeAcceptEncodingHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonConfig,
            DisableOnEtagHeader,
            RemoveAcceptEncodingHeader,
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
                            "disableOnEtagHeader" | "disable_on_etag_header" => Ok(GeneratedField::DisableOnEtagHeader),
                            "removeAcceptEncodingHeader" | "remove_accept_encoding_header" => Ok(GeneratedField::RemoveAcceptEncodingHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = compressor::ResponseDirectionConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.compressor.v3.Compressor.ResponseDirectionConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<compressor::ResponseDirectionConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_config__ = None;
                let mut disable_on_etag_header__ = None;
                let mut remove_accept_encoding_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonConfig => {
                            if common_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonConfig"));
                            }
                            common_config__ = map.next_value()?;
                        }
                        GeneratedField::DisableOnEtagHeader => {
                            if disable_on_etag_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableOnEtagHeader"));
                            }
                            disable_on_etag_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoveAcceptEncodingHeader => {
                            if remove_accept_encoding_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeAcceptEncodingHeader"));
                            }
                            remove_accept_encoding_header__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(compressor::ResponseDirectionConfig {
                    common_config: common_config__,
                    disable_on_etag_header: disable_on_etag_header__.unwrap_or_default(),
                    remove_accept_encoding_header: remove_accept_encoding_header__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.compressor.v3.Compressor.ResponseDirectionConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompressorOverrides {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response_direction_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.compressor.v3.CompressorOverrides", len)?;
        if let Some(v) = self.response_direction_config.as_ref() {
            struct_ser.serialize_field("responseDirectionConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompressorOverrides {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response_direction_config",
            "responseDirectionConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = CompressorOverrides;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.compressor.v3.CompressorOverrides")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CompressorOverrides, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response_direction_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResponseDirectionConfig => {
                            if response_direction_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseDirectionConfig"));
                            }
                            response_direction_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(CompressorOverrides {
                    response_direction_config: response_direction_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.compressor.v3.CompressorOverrides", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CompressorPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.compressor.v3.CompressorPerRoute", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                compressor_per_route::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                compressor_per_route::Override::Overrides(v) => {
                    struct_ser.serialize_field("overrides", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CompressorPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "overrides",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            Overrides,
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
                            "disabled" => Ok(GeneratedField::Disabled),
                            "overrides" => Ok(GeneratedField::Overrides),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CompressorPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.compressor.v3.CompressorPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CompressorPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#override__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(compressor_per_route::Override::Disabled);
                        }
                        GeneratedField::Overrides => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(compressor_per_route::Override::Overrides)
;
                        }
                    }
                }
                Ok(CompressorPerRoute {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.compressor.v3.CompressorPerRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseDirectionOverrides {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.compressor.v3.ResponseDirectionOverrides", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseDirectionOverrides {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseDirectionOverrides;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.compressor.v3.ResponseDirectionOverrides")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResponseDirectionOverrides, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ResponseDirectionOverrides {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.compressor.v3.ResponseDirectionOverrides", FIELDS, GeneratedVisitor)
    }
}
