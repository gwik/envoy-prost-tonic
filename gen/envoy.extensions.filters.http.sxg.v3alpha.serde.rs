// @generated
impl serde::Serialize for Sxg {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.certificate.is_some() {
            len += 1;
        }
        if self.private_key.is_some() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        if self.mi_record_size != 0 {
            len += 1;
        }
        if !self.cbor_url.is_empty() {
            len += 1;
        }
        if !self.validity_url.is_empty() {
            len += 1;
        }
        if !self.client_can_accept_sxg_header.is_empty() {
            len += 1;
        }
        if !self.should_encode_sxg_header.is_empty() {
            len += 1;
        }
        if !self.header_prefix_filters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.sxg.v3alpha.SXG", len)?;
        if let Some(v) = self.certificate.as_ref() {
            struct_ser.serialize_field("certificate", v)?;
        }
        if let Some(v) = self.private_key.as_ref() {
            struct_ser.serialize_field("privateKey", v)?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        if self.mi_record_size != 0 {
            struct_ser.serialize_field("miRecordSize", ToString::to_string(&self.mi_record_size).as_str())?;
        }
        if !self.cbor_url.is_empty() {
            struct_ser.serialize_field("cborUrl", &self.cbor_url)?;
        }
        if !self.validity_url.is_empty() {
            struct_ser.serialize_field("validityUrl", &self.validity_url)?;
        }
        if !self.client_can_accept_sxg_header.is_empty() {
            struct_ser.serialize_field("clientCanAcceptSxgHeader", &self.client_can_accept_sxg_header)?;
        }
        if !self.should_encode_sxg_header.is_empty() {
            struct_ser.serialize_field("shouldEncodeSxgHeader", &self.should_encode_sxg_header)?;
        }
        if !self.header_prefix_filters.is_empty() {
            struct_ser.serialize_field("headerPrefixFilters", &self.header_prefix_filters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Sxg {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "certificate",
            "private_key",
            "privateKey",
            "duration",
            "mi_record_size",
            "miRecordSize",
            "cbor_url",
            "cborUrl",
            "validity_url",
            "validityUrl",
            "client_can_accept_sxg_header",
            "clientCanAcceptSxgHeader",
            "should_encode_sxg_header",
            "shouldEncodeSxgHeader",
            "header_prefix_filters",
            "headerPrefixFilters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Certificate,
            PrivateKey,
            Duration,
            MiRecordSize,
            CborUrl,
            ValidityUrl,
            ClientCanAcceptSxgHeader,
            ShouldEncodeSxgHeader,
            HeaderPrefixFilters,
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
                            "certificate" => Ok(GeneratedField::Certificate),
                            "privateKey" | "private_key" => Ok(GeneratedField::PrivateKey),
                            "duration" => Ok(GeneratedField::Duration),
                            "miRecordSize" | "mi_record_size" => Ok(GeneratedField::MiRecordSize),
                            "cborUrl" | "cbor_url" => Ok(GeneratedField::CborUrl),
                            "validityUrl" | "validity_url" => Ok(GeneratedField::ValidityUrl),
                            "clientCanAcceptSxgHeader" | "client_can_accept_sxg_header" => Ok(GeneratedField::ClientCanAcceptSxgHeader),
                            "shouldEncodeSxgHeader" | "should_encode_sxg_header" => Ok(GeneratedField::ShouldEncodeSxgHeader),
                            "headerPrefixFilters" | "header_prefix_filters" => Ok(GeneratedField::HeaderPrefixFilters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Sxg;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.sxg.v3alpha.SXG")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Sxg, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut certificate__ = None;
                let mut private_key__ = None;
                let mut duration__ = None;
                let mut mi_record_size__ = None;
                let mut cbor_url__ = None;
                let mut validity_url__ = None;
                let mut client_can_accept_sxg_header__ = None;
                let mut should_encode_sxg_header__ = None;
                let mut header_prefix_filters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Certificate => {
                            if certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("certificate"));
                            }
                            certificate__ = map.next_value()?;
                        }
                        GeneratedField::PrivateKey => {
                            if private_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateKey"));
                            }
                            private_key__ = map.next_value()?;
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map.next_value()?;
                        }
                        GeneratedField::MiRecordSize => {
                            if mi_record_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("miRecordSize"));
                            }
                            mi_record_size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CborUrl => {
                            if cbor_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cborUrl"));
                            }
                            cbor_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidityUrl => {
                            if validity_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validityUrl"));
                            }
                            validity_url__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClientCanAcceptSxgHeader => {
                            if client_can_accept_sxg_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientCanAcceptSxgHeader"));
                            }
                            client_can_accept_sxg_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::ShouldEncodeSxgHeader => {
                            if should_encode_sxg_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shouldEncodeSxgHeader"));
                            }
                            should_encode_sxg_header__ = Some(map.next_value()?);
                        }
                        GeneratedField::HeaderPrefixFilters => {
                            if header_prefix_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerPrefixFilters"));
                            }
                            header_prefix_filters__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Sxg {
                    certificate: certificate__,
                    private_key: private_key__,
                    duration: duration__,
                    mi_record_size: mi_record_size__.unwrap_or_default(),
                    cbor_url: cbor_url__.unwrap_or_default(),
                    validity_url: validity_url__.unwrap_or_default(),
                    client_can_accept_sxg_header: client_can_accept_sxg_header__.unwrap_or_default(),
                    should_encode_sxg_header: should_encode_sxg_header__.unwrap_or_default(),
                    header_prefix_filters: header_prefix_filters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.sxg.v3alpha.SXG", FIELDS, GeneratedVisitor)
    }
}
