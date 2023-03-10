// @generated
impl serde::Serialize for FileBasedKeyValueStoreConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.filename.is_empty() {
            len += 1;
        }
        if self.flush_interval.is_some() {
            len += 1;
        }
        if self.max_entries.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.key_value.file_based.v3.FileBasedKeyValueStoreConfig", len)?;
        if !self.filename.is_empty() {
            struct_ser.serialize_field("filename", &self.filename)?;
        }
        if let Some(v) = self.flush_interval.as_ref() {
            struct_ser.serialize_field("flushInterval", v)?;
        }
        if let Some(v) = self.max_entries.as_ref() {
            struct_ser.serialize_field("maxEntries", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileBasedKeyValueStoreConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filename",
            "flush_interval",
            "flushInterval",
            "max_entries",
            "maxEntries",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filename,
            FlushInterval,
            MaxEntries,
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
                            "filename" => Ok(GeneratedField::Filename),
                            "flushInterval" | "flush_interval" => Ok(GeneratedField::FlushInterval),
                            "maxEntries" | "max_entries" => Ok(GeneratedField::MaxEntries),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileBasedKeyValueStoreConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.key_value.file_based.v3.FileBasedKeyValueStoreConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileBasedKeyValueStoreConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filename__ = None;
                let mut flush_interval__ = None;
                let mut max_entries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = Some(map.next_value()?);
                        }
                        GeneratedField::FlushInterval => {
                            if flush_interval__.is_some() {
                                return Err(serde::de::Error::duplicate_field("flushInterval"));
                            }
                            flush_interval__ = map.next_value()?;
                        }
                        GeneratedField::MaxEntries => {
                            if max_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEntries"));
                            }
                            max_entries__ = map.next_value()?;
                        }
                    }
                }
                Ok(FileBasedKeyValueStoreConfig {
                    filename: filename__.unwrap_or_default(),
                    flush_interval: flush_interval__,
                    max_entries: max_entries__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.key_value.file_based.v3.FileBasedKeyValueStoreConfig", FIELDS, GeneratedVisitor)
    }
}
