// @generated
impl serde::Serialize for Squash {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster.is_empty() {
            len += 1;
        }
        if self.attachment_template.is_some() {
            len += 1;
        }
        if self.request_timeout.is_some() {
            len += 1;
        }
        if self.attachment_timeout.is_some() {
            len += 1;
        }
        if self.attachment_poll_period.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.squash.v3.Squash", len)?;
        if !self.cluster.is_empty() {
            struct_ser.serialize_field("cluster", &self.cluster)?;
        }
        if let Some(v) = self.attachment_template.as_ref() {
            struct_ser.serialize_field("attachmentTemplate", v)?;
        }
        if let Some(v) = self.request_timeout.as_ref() {
            struct_ser.serialize_field("requestTimeout", v)?;
        }
        if let Some(v) = self.attachment_timeout.as_ref() {
            struct_ser.serialize_field("attachmentTimeout", v)?;
        }
        if let Some(v) = self.attachment_poll_period.as_ref() {
            struct_ser.serialize_field("attachmentPollPeriod", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Squash {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster",
            "attachment_template",
            "attachmentTemplate",
            "request_timeout",
            "requestTimeout",
            "attachment_timeout",
            "attachmentTimeout",
            "attachment_poll_period",
            "attachmentPollPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cluster,
            AttachmentTemplate,
            RequestTimeout,
            AttachmentTimeout,
            AttachmentPollPeriod,
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
                            "cluster" => Ok(GeneratedField::Cluster),
                            "attachmentTemplate" | "attachment_template" => Ok(GeneratedField::AttachmentTemplate),
                            "requestTimeout" | "request_timeout" => Ok(GeneratedField::RequestTimeout),
                            "attachmentTimeout" | "attachment_timeout" => Ok(GeneratedField::AttachmentTimeout),
                            "attachmentPollPeriod" | "attachment_poll_period" => Ok(GeneratedField::AttachmentPollPeriod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Squash;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.squash.v3.Squash")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Squash, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster__ = None;
                let mut attachment_template__ = None;
                let mut request_timeout__ = None;
                let mut attachment_timeout__ = None;
                let mut attachment_poll_period__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cluster => {
                            if cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cluster"));
                            }
                            cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::AttachmentTemplate => {
                            if attachment_template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachmentTemplate"));
                            }
                            attachment_template__ = map.next_value()?;
                        }
                        GeneratedField::RequestTimeout => {
                            if request_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTimeout"));
                            }
                            request_timeout__ = map.next_value()?;
                        }
                        GeneratedField::AttachmentTimeout => {
                            if attachment_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachmentTimeout"));
                            }
                            attachment_timeout__ = map.next_value()?;
                        }
                        GeneratedField::AttachmentPollPeriod => {
                            if attachment_poll_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachmentPollPeriod"));
                            }
                            attachment_poll_period__ = map.next_value()?;
                        }
                    }
                }
                Ok(Squash {
                    cluster: cluster__.unwrap_or_default(),
                    attachment_template: attachment_template__,
                    request_timeout: request_timeout__,
                    attachment_timeout: attachment_timeout__,
                    attachment_poll_period: attachment_poll_period__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.squash.v3.Squash", FIELDS, GeneratedVisitor)
    }
}
