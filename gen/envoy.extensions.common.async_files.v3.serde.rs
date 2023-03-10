// @generated
impl serde::Serialize for AsyncFileManagerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.manager_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.common.async_files.v3.AsyncFileManagerConfig", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.manager_type.as_ref() {
            match v {
                async_file_manager_config::ManagerType::ThreadPool(v) => {
                    struct_ser.serialize_field("threadPool", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AsyncFileManagerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "thread_pool",
            "threadPool",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ThreadPool,
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
                            "id" => Ok(GeneratedField::Id),
                            "threadPool" | "thread_pool" => Ok(GeneratedField::ThreadPool),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AsyncFileManagerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.common.async_files.v3.AsyncFileManagerConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AsyncFileManagerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut manager_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ThreadPool => {
                            if manager_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threadPool"));
                            }
                            manager_type__ = map.next_value::<::std::option::Option<_>>()?.map(async_file_manager_config::ManagerType::ThreadPool)
;
                        }
                    }
                }
                Ok(AsyncFileManagerConfig {
                    id: id__.unwrap_or_default(),
                    manager_type: manager_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.common.async_files.v3.AsyncFileManagerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for async_file_manager_config::ThreadPool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.thread_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.common.async_files.v3.AsyncFileManagerConfig.ThreadPool", len)?;
        if self.thread_count != 0 {
            struct_ser.serialize_field("threadCount", &self.thread_count)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for async_file_manager_config::ThreadPool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "thread_count",
            "threadCount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ThreadCount,
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
                            "threadCount" | "thread_count" => Ok(GeneratedField::ThreadCount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = async_file_manager_config::ThreadPool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.common.async_files.v3.AsyncFileManagerConfig.ThreadPool")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<async_file_manager_config::ThreadPool, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut thread_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ThreadCount => {
                            if thread_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("threadCount"));
                            }
                            thread_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(async_file_manager_config::ThreadPool {
                    thread_count: thread_count__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.common.async_files.v3.AsyncFileManagerConfig.ThreadPool", FIELDS, GeneratedVisitor)
    }
}
