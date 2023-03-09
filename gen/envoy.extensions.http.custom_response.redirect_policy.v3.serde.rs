// @generated
impl serde::Serialize for RedirectPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status_code.is_some() {
            len += 1;
        }
        if !self.response_headers_to_add.is_empty() {
            len += 1;
        }
        if !self.request_headers_to_add.is_empty() {
            len += 1;
        }
        if self.modify_request_headers_action.is_some() {
            len += 1;
        }
        if self.redirect_action_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.http.custom_response.redirect_policy.v3.RedirectPolicy", len)?;
        if let Some(v) = self.status_code.as_ref() {
            struct_ser.serialize_field("statusCode", v)?;
        }
        if !self.response_headers_to_add.is_empty() {
            struct_ser.serialize_field("responseHeadersToAdd", &self.response_headers_to_add)?;
        }
        if !self.request_headers_to_add.is_empty() {
            struct_ser.serialize_field("requestHeadersToAdd", &self.request_headers_to_add)?;
        }
        if let Some(v) = self.modify_request_headers_action.as_ref() {
            struct_ser.serialize_field("modifyRequestHeadersAction", v)?;
        }
        if let Some(v) = self.redirect_action_specifier.as_ref() {
            match v {
                redirect_policy::RedirectActionSpecifier::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
                redirect_policy::RedirectActionSpecifier::RedirectAction(v) => {
                    struct_ser.serialize_field("redirectAction", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedirectPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status_code",
            "statusCode",
            "response_headers_to_add",
            "responseHeadersToAdd",
            "request_headers_to_add",
            "requestHeadersToAdd",
            "modify_request_headers_action",
            "modifyRequestHeadersAction",
            "uri",
            "redirect_action",
            "redirectAction",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatusCode,
            ResponseHeadersToAdd,
            RequestHeadersToAdd,
            ModifyRequestHeadersAction,
            Uri,
            RedirectAction,
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
                            "statusCode" | "status_code" => Ok(GeneratedField::StatusCode),
                            "responseHeadersToAdd" | "response_headers_to_add" => Ok(GeneratedField::ResponseHeadersToAdd),
                            "requestHeadersToAdd" | "request_headers_to_add" => Ok(GeneratedField::RequestHeadersToAdd),
                            "modifyRequestHeadersAction" | "modify_request_headers_action" => Ok(GeneratedField::ModifyRequestHeadersAction),
                            "uri" => Ok(GeneratedField::Uri),
                            "redirectAction" | "redirect_action" => Ok(GeneratedField::RedirectAction),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedirectPolicy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.http.custom_response.redirect_policy.v3.RedirectPolicy")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RedirectPolicy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status_code__ = None;
                let mut response_headers_to_add__ = None;
                let mut request_headers_to_add__ = None;
                let mut modify_request_headers_action__ = None;
                let mut redirect_action_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatusCode => {
                            if status_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusCode"));
                            }
                            status_code__ = map.next_value()?;
                        }
                        GeneratedField::ResponseHeadersToAdd => {
                            if response_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersToAdd"));
                            }
                            response_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersToAdd => {
                            if request_headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersToAdd"));
                            }
                            request_headers_to_add__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModifyRequestHeadersAction => {
                            if modify_request_headers_action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifyRequestHeadersAction"));
                            }
                            modify_request_headers_action__ = map.next_value()?;
                        }
                        GeneratedField::Uri => {
                            if redirect_action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            redirect_action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(redirect_policy::RedirectActionSpecifier::Uri);
                        }
                        GeneratedField::RedirectAction => {
                            if redirect_action_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("redirectAction"));
                            }
                            redirect_action_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(redirect_policy::RedirectActionSpecifier::RedirectAction)
;
                        }
                    }
                }
                Ok(RedirectPolicy {
                    status_code: status_code__,
                    response_headers_to_add: response_headers_to_add__.unwrap_or_default(),
                    request_headers_to_add: request_headers_to_add__.unwrap_or_default(),
                    modify_request_headers_action: modify_request_headers_action__,
                    redirect_action_specifier: redirect_action_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.http.custom_response.redirect_policy.v3.RedirectPolicy", FIELDS, GeneratedVisitor)
    }
}
