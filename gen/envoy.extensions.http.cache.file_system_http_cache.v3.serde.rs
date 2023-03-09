// @generated
impl serde::Serialize for FileSystemHttpCacheConfig {
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
        if !self.cache_path.is_empty() {
            len += 1;
        }
        if self.max_cache_size_bytes.is_some() {
            len += 1;
        }
        if self.max_individual_cache_entry_size_bytes.is_some() {
            len += 1;
        }
        if self.max_cache_entry_count.is_some() {
            len += 1;
        }
        if self.cache_subdivisions != 0 {
            len += 1;
        }
        if self.evict_fraction != 0. {
            len += 1;
        }
        if self.max_eviction_period.is_some() {
            len += 1;
        }
        if self.min_eviction_period.is_some() {
            len += 1;
        }
        if self.create_cache_path {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.cache.file_system_http_cache.v3.FileSystemHttpCacheConfig", len)?;
        if let Some(v) = self.manager_config.as_ref() {
            struct_ser.serialize_field("managerConfig", v)?;
        }
        if !self.cache_path.is_empty() {
            struct_ser.serialize_field("cachePath", &self.cache_path)?;
        }
        if let Some(v) = self.max_cache_size_bytes.as_ref() {
            struct_ser.serialize_field("maxCacheSizeBytes", v)?;
        }
        if let Some(v) = self.max_individual_cache_entry_size_bytes.as_ref() {
            struct_ser.serialize_field("maxIndividualCacheEntrySizeBytes", v)?;
        }
        if let Some(v) = self.max_cache_entry_count.as_ref() {
            struct_ser.serialize_field("maxCacheEntryCount", v)?;
        }
        if self.cache_subdivisions != 0 {
            struct_ser.serialize_field("cacheSubdivisions", &self.cache_subdivisions)?;
        }
        if self.evict_fraction != 0. {
            struct_ser.serialize_field("evictFraction", &self.evict_fraction)?;
        }
        if let Some(v) = self.max_eviction_period.as_ref() {
            struct_ser.serialize_field("maxEvictionPeriod", v)?;
        }
        if let Some(v) = self.min_eviction_period.as_ref() {
            struct_ser.serialize_field("minEvictionPeriod", v)?;
        }
        if self.create_cache_path {
            struct_ser.serialize_field("createCachePath", &self.create_cache_path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileSystemHttpCacheConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "manager_config",
            "managerConfig",
            "cache_path",
            "cachePath",
            "max_cache_size_bytes",
            "maxCacheSizeBytes",
            "max_individual_cache_entry_size_bytes",
            "maxIndividualCacheEntrySizeBytes",
            "max_cache_entry_count",
            "maxCacheEntryCount",
            "cache_subdivisions",
            "cacheSubdivisions",
            "evict_fraction",
            "evictFraction",
            "max_eviction_period",
            "maxEvictionPeriod",
            "min_eviction_period",
            "minEvictionPeriod",
            "create_cache_path",
            "createCachePath",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ManagerConfig,
            CachePath,
            MaxCacheSizeBytes,
            MaxIndividualCacheEntrySizeBytes,
            MaxCacheEntryCount,
            CacheSubdivisions,
            EvictFraction,
            MaxEvictionPeriod,
            MinEvictionPeriod,
            CreateCachePath,
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
                            "cachePath" | "cache_path" => Ok(GeneratedField::CachePath),
                            "maxCacheSizeBytes" | "max_cache_size_bytes" => Ok(GeneratedField::MaxCacheSizeBytes),
                            "maxIndividualCacheEntrySizeBytes" | "max_individual_cache_entry_size_bytes" => Ok(GeneratedField::MaxIndividualCacheEntrySizeBytes),
                            "maxCacheEntryCount" | "max_cache_entry_count" => Ok(GeneratedField::MaxCacheEntryCount),
                            "cacheSubdivisions" | "cache_subdivisions" => Ok(GeneratedField::CacheSubdivisions),
                            "evictFraction" | "evict_fraction" => Ok(GeneratedField::EvictFraction),
                            "maxEvictionPeriod" | "max_eviction_period" => Ok(GeneratedField::MaxEvictionPeriod),
                            "minEvictionPeriod" | "min_eviction_period" => Ok(GeneratedField::MinEvictionPeriod),
                            "createCachePath" | "create_cache_path" => Ok(GeneratedField::CreateCachePath),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileSystemHttpCacheConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.cache.file_system_http_cache.v3.FileSystemHttpCacheConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileSystemHttpCacheConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut manager_config__ = None;
                let mut cache_path__ = None;
                let mut max_cache_size_bytes__ = None;
                let mut max_individual_cache_entry_size_bytes__ = None;
                let mut max_cache_entry_count__ = None;
                let mut cache_subdivisions__ = None;
                let mut evict_fraction__ = None;
                let mut max_eviction_period__ = None;
                let mut min_eviction_period__ = None;
                let mut create_cache_path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ManagerConfig => {
                            if manager_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("managerConfig"));
                            }
                            manager_config__ = map.next_value()?;
                        }
                        GeneratedField::CachePath => {
                            if cache_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cachePath"));
                            }
                            cache_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxCacheSizeBytes => {
                            if max_cache_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCacheSizeBytes"));
                            }
                            max_cache_size_bytes__ = map.next_value()?;
                        }
                        GeneratedField::MaxIndividualCacheEntrySizeBytes => {
                            if max_individual_cache_entry_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxIndividualCacheEntrySizeBytes"));
                            }
                            max_individual_cache_entry_size_bytes__ = map.next_value()?;
                        }
                        GeneratedField::MaxCacheEntryCount => {
                            if max_cache_entry_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxCacheEntryCount"));
                            }
                            max_cache_entry_count__ = map.next_value()?;
                        }
                        GeneratedField::CacheSubdivisions => {
                            if cache_subdivisions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cacheSubdivisions"));
                            }
                            cache_subdivisions__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EvictFraction => {
                            if evict_fraction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evictFraction"));
                            }
                            evict_fraction__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MaxEvictionPeriod => {
                            if max_eviction_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEvictionPeriod"));
                            }
                            max_eviction_period__ = map.next_value()?;
                        }
                        GeneratedField::MinEvictionPeriod => {
                            if min_eviction_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minEvictionPeriod"));
                            }
                            min_eviction_period__ = map.next_value()?;
                        }
                        GeneratedField::CreateCachePath => {
                            if create_cache_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createCachePath"));
                            }
                            create_cache_path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(FileSystemHttpCacheConfig {
                    manager_config: manager_config__,
                    cache_path: cache_path__.unwrap_or_default(),
                    max_cache_size_bytes: max_cache_size_bytes__,
                    max_individual_cache_entry_size_bytes: max_individual_cache_entry_size_bytes__,
                    max_cache_entry_count: max_cache_entry_count__,
                    cache_subdivisions: cache_subdivisions__.unwrap_or_default(),
                    evict_fraction: evict_fraction__.unwrap_or_default(),
                    max_eviction_period: max_eviction_period__,
                    min_eviction_period: min_eviction_period__,
                    create_cache_path: create_cache_path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.cache.file_system_http_cache.v3.FileSystemHttpCacheConfig", FIELDS, GeneratedVisitor)
    }
}
