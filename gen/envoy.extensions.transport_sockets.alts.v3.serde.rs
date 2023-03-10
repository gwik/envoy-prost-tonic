// @generated
impl serde::Serialize for Alts {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.handshaker_service.is_empty() {
            len += 1;
        }
        if !self.peer_service_accounts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.transport_sockets.alts.v3.Alts", len)?;
        if !self.handshaker_service.is_empty() {
            struct_ser.serialize_field("handshakerService", &self.handshaker_service)?;
        }
        if !self.peer_service_accounts.is_empty() {
            struct_ser.serialize_field("peerServiceAccounts", &self.peer_service_accounts)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Alts {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "handshaker_service",
            "handshakerService",
            "peer_service_accounts",
            "peerServiceAccounts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HandshakerService,
            PeerServiceAccounts,
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
                            "handshakerService" | "handshaker_service" => Ok(GeneratedField::HandshakerService),
                            "peerServiceAccounts" | "peer_service_accounts" => Ok(GeneratedField::PeerServiceAccounts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Alts;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.transport_sockets.alts.v3.Alts")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Alts, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut handshaker_service__ = None;
                let mut peer_service_accounts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HandshakerService => {
                            if handshaker_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("handshakerService"));
                            }
                            handshaker_service__ = Some(map.next_value()?);
                        }
                        GeneratedField::PeerServiceAccounts => {
                            if peer_service_accounts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerServiceAccounts"));
                            }
                            peer_service_accounts__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Alts {
                    handshaker_service: handshaker_service__.unwrap_or_default(),
                    peer_service_accounts: peer_service_accounts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.transport_sockets.alts.v3.Alts", FIELDS, GeneratedVisitor)
    }
}
