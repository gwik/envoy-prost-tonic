// @generated
impl serde::Serialize for BufferBehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.behavior.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior", len)?;
        if let Some(v) = self.behavior.as_ref() {
            match v {
                buffer_behavior::Behavior::StreamWhenPossible(v) => {
                    struct_ser.serialize_field("streamWhenPossible", v)?;
                }
                buffer_behavior::Behavior::Bypass(v) => {
                    struct_ser.serialize_field("bypass", v)?;
                }
                buffer_behavior::Behavior::InjectContentLengthIfNecessary(v) => {
                    struct_ser.serialize_field("injectContentLengthIfNecessary", v)?;
                }
                buffer_behavior::Behavior::FullyBufferAndAlwaysInjectContentLength(v) => {
                    struct_ser.serialize_field("fullyBufferAndAlwaysInjectContentLength", v)?;
                }
                buffer_behavior::Behavior::FullyBuffer(v) => {
                    struct_ser.serialize_field("fullyBuffer", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BufferBehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stream_when_possible",
            "streamWhenPossible",
            "bypass",
            "inject_content_length_if_necessary",
            "injectContentLengthIfNecessary",
            "fully_buffer_and_always_inject_content_length",
            "fullyBufferAndAlwaysInjectContentLength",
            "fully_buffer",
            "fullyBuffer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StreamWhenPossible,
            Bypass,
            InjectContentLengthIfNecessary,
            FullyBufferAndAlwaysInjectContentLength,
            FullyBuffer,
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
                            "streamWhenPossible" | "stream_when_possible" => Ok(GeneratedField::StreamWhenPossible),
                            "bypass" => Ok(GeneratedField::Bypass),
                            "injectContentLengthIfNecessary" | "inject_content_length_if_necessary" => Ok(GeneratedField::InjectContentLengthIfNecessary),
                            "fullyBufferAndAlwaysInjectContentLength" | "fully_buffer_and_always_inject_content_length" => Ok(GeneratedField::FullyBufferAndAlwaysInjectContentLength),
                            "fullyBuffer" | "fully_buffer" => Ok(GeneratedField::FullyBuffer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BufferBehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BufferBehavior, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut behavior__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StreamWhenPossible => {
                            if behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamWhenPossible"));
                            }
                            behavior__ = map.next_value::<::std::option::Option<_>>()?.map(buffer_behavior::Behavior::StreamWhenPossible)
;
                        }
                        GeneratedField::Bypass => {
                            if behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bypass"));
                            }
                            behavior__ = map.next_value::<::std::option::Option<_>>()?.map(buffer_behavior::Behavior::Bypass)
;
                        }
                        GeneratedField::InjectContentLengthIfNecessary => {
                            if behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("injectContentLengthIfNecessary"));
                            }
                            behavior__ = map.next_value::<::std::option::Option<_>>()?.map(buffer_behavior::Behavior::InjectContentLengthIfNecessary)
;
                        }
                        GeneratedField::FullyBufferAndAlwaysInjectContentLength => {
                            if behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullyBufferAndAlwaysInjectContentLength"));
                            }
                            behavior__ = map.next_value::<::std::option::Option<_>>()?.map(buffer_behavior::Behavior::FullyBufferAndAlwaysInjectContentLength)
;
                        }
                        GeneratedField::FullyBuffer => {
                            if behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullyBuffer"));
                            }
                            behavior__ = map.next_value::<::std::option::Option<_>>()?.map(buffer_behavior::Behavior::FullyBuffer)
;
                        }
                    }
                }
                Ok(BufferBehavior {
                    behavior: behavior__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for buffer_behavior::Bypass {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.Bypass", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for buffer_behavior::Bypass {
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
            type Value = buffer_behavior::Bypass;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.Bypass")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<buffer_behavior::Bypass, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(buffer_behavior::Bypass {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.Bypass", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for buffer_behavior::FullyBuffer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.FullyBuffer", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for buffer_behavior::FullyBuffer {
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
            type Value = buffer_behavior::FullyBuffer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.FullyBuffer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<buffer_behavior::FullyBuffer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(buffer_behavior::FullyBuffer {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.FullyBuffer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for buffer_behavior::FullyBufferAndAlwaysInjectContentLength {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.FullyBufferAndAlwaysInjectContentLength", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for buffer_behavior::FullyBufferAndAlwaysInjectContentLength {
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
            type Value = buffer_behavior::FullyBufferAndAlwaysInjectContentLength;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.FullyBufferAndAlwaysInjectContentLength")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<buffer_behavior::FullyBufferAndAlwaysInjectContentLength, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(buffer_behavior::FullyBufferAndAlwaysInjectContentLength {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.FullyBufferAndAlwaysInjectContentLength", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for buffer_behavior::InjectContentLengthIfNecessary {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.InjectContentLengthIfNecessary", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for buffer_behavior::InjectContentLengthIfNecessary {
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
            type Value = buffer_behavior::InjectContentLengthIfNecessary;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.InjectContentLengthIfNecessary")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<buffer_behavior::InjectContentLengthIfNecessary, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(buffer_behavior::InjectContentLengthIfNecessary {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.InjectContentLengthIfNecessary", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for buffer_behavior::StreamWhenPossible {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.StreamWhenPossible", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for buffer_behavior::StreamWhenPossible {
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
            type Value = buffer_behavior::StreamWhenPossible;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.StreamWhenPossible")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<buffer_behavior::StreamWhenPossible, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(buffer_behavior::StreamWhenPossible {
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.BufferBehavior.StreamWhenPossible", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileSystemBufferFilterConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.manager_config.is_some() {
            len += 1;
        }
        if self.storage_buffer_path.is_some() {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.FileSystemBufferFilterConfig", len)?;
        if let Some(v) = self.manager_config.as_ref() {
            struct_ser.serialize_field("managerConfig", v)?;
        }
        if let Some(v) = self.storage_buffer_path.as_ref() {
            struct_ser.serialize_field("storageBufferPath", v)?;
        }
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemBufferFilterConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "manager_config",
            "managerConfig",
            "storage_buffer_path",
            "storageBufferPath",
            "request",
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ManagerConfig,
            StorageBufferPath,
            Request,
            Response,
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
                            "managerConfig" | "manager_config" => Ok(GeneratedField::ManagerConfig),
                            "storageBufferPath" | "storage_buffer_path" => Ok(GeneratedField::StorageBufferPath),
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemBufferFilterConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.FileSystemBufferFilterConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemBufferFilterConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut manager_config__ = None;
                let mut storage_buffer_path__ = None;
                let mut request__ = None;
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ManagerConfig => {
                            if manager_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("managerConfig"));
                            }
                            manager_config__ = map.next_value()?;
                        }
                        GeneratedField::StorageBufferPath => {
                            if storage_buffer_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageBufferPath"));
                            }
                            storage_buffer_path__ = map.next_value()?;
                        }
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map.next_value()?;
                        }
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map.next_value()?;
                        }
                    }
                }
                Ok(FileSystemBufferFilterConfig {
                    manager_config: manager_config__,
                    storage_buffer_path: storage_buffer_path__,
                    request: request__,
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.FileSystemBufferFilterConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.behavior.is_some() {
            len += 1;
        }
        if self.memory_buffer_bytes_limit.is_some() {
            len += 1;
        }
        if self.storage_buffer_bytes_limit.is_some() {
            len += 1;
        }
        if self.storage_buffer_queue_high_watermark_bytes.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.StreamConfig", len)?;
        if let Some(v) = self.behavior.as_ref() {
            struct_ser.serialize_field("behavior", v)?;
        }
        if let Some(v) = self.memory_buffer_bytes_limit.as_ref() {
            struct_ser.serialize_field("memoryBufferBytesLimit", v)?;
        }
        if let Some(v) = self.storage_buffer_bytes_limit.as_ref() {
            struct_ser.serialize_field("storageBufferBytesLimit", v)?;
        }
        if let Some(v) = self.storage_buffer_queue_high_watermark_bytes.as_ref() {
            struct_ser.serialize_field("storageBufferQueueHighWatermarkBytes", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "behavior",
            "memory_buffer_bytes_limit",
            "memoryBufferBytesLimit",
            "storage_buffer_bytes_limit",
            "storageBufferBytesLimit",
            "storage_buffer_queue_high_watermark_bytes",
            "storageBufferQueueHighWatermarkBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Behavior,
            MemoryBufferBytesLimit,
            StorageBufferBytesLimit,
            StorageBufferQueueHighWatermarkBytes,
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
                            "behavior" => Ok(GeneratedField::Behavior),
                            "memoryBufferBytesLimit" | "memory_buffer_bytes_limit" => Ok(GeneratedField::MemoryBufferBytesLimit),
                            "storageBufferBytesLimit" | "storage_buffer_bytes_limit" => Ok(GeneratedField::StorageBufferBytesLimit),
                            "storageBufferQueueHighWatermarkBytes" | "storage_buffer_queue_high_watermark_bytes" => Ok(GeneratedField::StorageBufferQueueHighWatermarkBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.file_system_buffer.v3.StreamConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<StreamConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut behavior__ = None;
                let mut memory_buffer_bytes_limit__ = None;
                let mut storage_buffer_bytes_limit__ = None;
                let mut storage_buffer_queue_high_watermark_bytes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Behavior => {
                            if behavior__.is_some() {
                                return Err(serde::de::Error::duplicate_field("behavior"));
                            }
                            behavior__ = map.next_value()?;
                        }
                        GeneratedField::MemoryBufferBytesLimit => {
                            if memory_buffer_bytes_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryBufferBytesLimit"));
                            }
                            memory_buffer_bytes_limit__ = map.next_value()?;
                        }
                        GeneratedField::StorageBufferBytesLimit => {
                            if storage_buffer_bytes_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageBufferBytesLimit"));
                            }
                            storage_buffer_bytes_limit__ = map.next_value()?;
                        }
                        GeneratedField::StorageBufferQueueHighWatermarkBytes => {
                            if storage_buffer_queue_high_watermark_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageBufferQueueHighWatermarkBytes"));
                            }
                            storage_buffer_queue_high_watermark_bytes__ = map.next_value()?;
                        }
                    }
                }
                Ok(StreamConfig {
                    behavior: behavior__,
                    memory_buffer_bytes_limit: memory_buffer_bytes_limit__,
                    storage_buffer_bytes_limit: storage_buffer_bytes_limit__,
                    storage_buffer_queue_high_watermark_bytes: storage_buffer_queue_high_watermark_bytes__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.file_system_buffer.v3.StreamConfig", FIELDS, GeneratedVisitor)
    }
}
