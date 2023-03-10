// @generated
impl serde::Serialize for PostgresProxy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.enable_sql_parsing.is_some() {
            len += 1;
        }
        if self.terminate_ssl {
            len += 1;
        }
        if self.upstream_ssl != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.postgres_proxy.v3alpha.PostgresProxy", len)?;
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.enable_sql_parsing.as_ref() {
            struct_ser.serialize_field("enableSqlParsing", v)?;
        }
        if self.terminate_ssl {
            struct_ser.serialize_field("terminateSsl", &self.terminate_ssl)?;
        }
        if self.upstream_ssl != 0 {
            let v = postgres_proxy::SslMode::from_i32(self.upstream_ssl)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.upstream_ssl)))?;
            struct_ser.serialize_field("upstreamSsl", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PostgresProxy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stat_prefix",
            "statPrefix",
            "enable_sql_parsing",
            "enableSqlParsing",
            "terminate_ssl",
            "terminateSsl",
            "upstream_ssl",
            "upstreamSsl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatPrefix,
            EnableSqlParsing,
            TerminateSsl,
            UpstreamSsl,
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
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "enableSqlParsing" | "enable_sql_parsing" => Ok(GeneratedField::EnableSqlParsing),
                            "terminateSsl" | "terminate_ssl" => Ok(GeneratedField::TerminateSsl),
                            "upstreamSsl" | "upstream_ssl" => Ok(GeneratedField::UpstreamSsl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PostgresProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.postgres_proxy.v3alpha.PostgresProxy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PostgresProxy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stat_prefix__ = None;
                let mut enable_sql_parsing__ = None;
                let mut terminate_ssl__ = None;
                let mut upstream_ssl__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnableSqlParsing => {
                            if enable_sql_parsing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enableSqlParsing"));
                            }
                            enable_sql_parsing__ = map.next_value()?;
                        }
                        GeneratedField::TerminateSsl => {
                            if terminate_ssl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("terminateSsl"));
                            }
                            terminate_ssl__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamSsl => {
                            if upstream_ssl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamSsl"));
                            }
                            upstream_ssl__ = Some(map.next_value::<postgres_proxy::SslMode>()? as i32);
                        }
                    }
                }
                Ok(PostgresProxy {
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    enable_sql_parsing: enable_sql_parsing__,
                    terminate_ssl: terminate_ssl__.unwrap_or_default(),
                    upstream_ssl: upstream_ssl__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.postgres_proxy.v3alpha.PostgresProxy", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for postgres_proxy::SslMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Disable => "DISABLE",
            Self::Require => "REQUIRE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for postgres_proxy::SslMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DISABLE",
            "REQUIRE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = postgres_proxy::SslMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(postgres_proxy::SslMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(postgres_proxy::SslMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DISABLE" => Ok(postgres_proxy::SslMode::Disable),
                    "REQUIRE" => Ok(postgres_proxy::SslMode::Require),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
