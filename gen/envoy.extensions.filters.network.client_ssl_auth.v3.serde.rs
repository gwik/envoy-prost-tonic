// @generated
impl serde::Serialize for ClientSslAuth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.auth_api_cluster.is_empty() {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.refresh_delay.is_some() {
            len += 1;
        }
        if !self.ip_white_list.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.network.client_ssl_auth.v3.ClientSSLAuth", len)?;
        if !self.auth_api_cluster.is_empty() {
            struct_ser.serialize_field("authApiCluster", &self.auth_api_cluster)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.refresh_delay.as_ref() {
            struct_ser.serialize_field("refreshDelay", v)?;
        }
        if !self.ip_white_list.is_empty() {
            struct_ser.serialize_field("ipWhiteList", &self.ip_white_list)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientSslAuth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "auth_api_cluster",
            "authApiCluster",
            "stat_prefix",
            "statPrefix",
            "refresh_delay",
            "refreshDelay",
            "ip_white_list",
            "ipWhiteList",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AuthApiCluster,
            StatPrefix,
            RefreshDelay,
            IpWhiteList,
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
                            "authApiCluster" | "auth_api_cluster" => Ok(GeneratedField::AuthApiCluster),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "refreshDelay" | "refresh_delay" => Ok(GeneratedField::RefreshDelay),
                            "ipWhiteList" | "ip_white_list" => Ok(GeneratedField::IpWhiteList),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientSslAuth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.network.client_ssl_auth.v3.ClientSSLAuth")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientSslAuth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut auth_api_cluster__ = None;
                let mut stat_prefix__ = None;
                let mut refresh_delay__ = None;
                let mut ip_white_list__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AuthApiCluster => {
                            if auth_api_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authApiCluster"));
                            }
                            auth_api_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::RefreshDelay => {
                            if refresh_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("refreshDelay"));
                            }
                            refresh_delay__ = map.next_value()?;
                        }
                        GeneratedField::IpWhiteList => {
                            if ip_white_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ipWhiteList"));
                            }
                            ip_white_list__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientSslAuth {
                    auth_api_cluster: auth_api_cluster__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    refresh_delay: refresh_delay__,
                    ip_white_list: ip_white_list__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.network.client_ssl_auth.v3.ClientSSLAuth", FIELDS, GeneratedVisitor)
    }
}
