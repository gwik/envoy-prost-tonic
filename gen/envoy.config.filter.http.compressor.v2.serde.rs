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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.compressor.v2.Compressor", len)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContentLength,
            ContentType,
            DisableOnEtagHeader,
            RemoveAcceptEncodingHeader,
            RuntimeEnabled,
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
                formatter.write_str("struct envoy.config.filter.http.compressor.v2.Compressor")
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
                    }
                }
                Ok(Compressor {
                    content_length: content_length__,
                    content_type: content_type__.unwrap_or_default(),
                    disable_on_etag_header: disable_on_etag_header__.unwrap_or_default(),
                    remove_accept_encoding_header: remove_accept_encoding_header__.unwrap_or_default(),
                    runtime_enabled: runtime_enabled__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.compressor.v2.Compressor", FIELDS, GeneratedVisitor)
    }
}
