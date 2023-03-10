// @generated
impl serde::Serialize for AuthorizationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed_headers.is_some() {
            len += 1;
        }
        if !self.headers_to_add.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ext_authz.v2.AuthorizationRequest", len)?;
        if let Some(v) = self.allowed_headers.as_ref() {
            struct_ser.serialize_field("allowedHeaders", v)?;
        }
        if !self.headers_to_add.is_empty() {
            struct_ser.serialize_field("headersToAdd", &self.headers_to_add)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_headers",
            "allowedHeaders",
            "headers_to_add",
            "headersToAdd",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedHeaders,
            HeadersToAdd,
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
                            "allowedHeaders" | "allowed_headers" => Ok(GeneratedField::AllowedHeaders),
                            "headersToAdd" | "headers_to_add" => Ok(GeneratedField::HeadersToAdd),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ext_authz.v2.AuthorizationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AuthorizationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_headers__ = None;
                let mut headers_to_add__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowedHeaders => {
                            if allowed_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedHeaders"));
                            }
                            allowed_headers__ = map.next_value()?;
                        }
                        GeneratedField::HeadersToAdd => {
                            if headers_to_add__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headersToAdd"));
                            }
                            headers_to_add__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AuthorizationRequest {
                    allowed_headers: allowed_headers__,
                    headers_to_add: headers_to_add__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ext_authz.v2.AuthorizationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AuthorizationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allowed_upstream_headers.is_some() {
            len += 1;
        }
        if self.allowed_client_headers.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ext_authz.v2.AuthorizationResponse", len)?;
        if let Some(v) = self.allowed_upstream_headers.as_ref() {
            struct_ser.serialize_field("allowedUpstreamHeaders", v)?;
        }
        if let Some(v) = self.allowed_client_headers.as_ref() {
            struct_ser.serialize_field("allowedClientHeaders", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AuthorizationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_upstream_headers",
            "allowedUpstreamHeaders",
            "allowed_client_headers",
            "allowedClientHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedUpstreamHeaders,
            AllowedClientHeaders,
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
                            "allowedUpstreamHeaders" | "allowed_upstream_headers" => Ok(GeneratedField::AllowedUpstreamHeaders),
                            "allowedClientHeaders" | "allowed_client_headers" => Ok(GeneratedField::AllowedClientHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AuthorizationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ext_authz.v2.AuthorizationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AuthorizationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_upstream_headers__ = None;
                let mut allowed_client_headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowedUpstreamHeaders => {
                            if allowed_upstream_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedUpstreamHeaders"));
                            }
                            allowed_upstream_headers__ = map.next_value()?;
                        }
                        GeneratedField::AllowedClientHeaders => {
                            if allowed_client_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedClientHeaders"));
                            }
                            allowed_client_headers__ = map.next_value()?;
                        }
                    }
                }
                Ok(AuthorizationResponse {
                    allowed_upstream_headers: allowed_upstream_headers__,
                    allowed_client_headers: allowed_client_headers__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ext_authz.v2.AuthorizationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BufferSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_request_bytes != 0 {
            len += 1;
        }
        if self.allow_partial_message {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ext_authz.v2.BufferSettings", len)?;
        if self.max_request_bytes != 0 {
            struct_ser.serialize_field("maxRequestBytes", &self.max_request_bytes)?;
        }
        if self.allow_partial_message {
            struct_ser.serialize_field("allowPartialMessage", &self.allow_partial_message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BufferSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_request_bytes",
            "maxRequestBytes",
            "allow_partial_message",
            "allowPartialMessage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxRequestBytes,
            AllowPartialMessage,
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
                            "maxRequestBytes" | "max_request_bytes" => Ok(GeneratedField::MaxRequestBytes),
                            "allowPartialMessage" | "allow_partial_message" => Ok(GeneratedField::AllowPartialMessage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BufferSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ext_authz.v2.BufferSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BufferSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_request_bytes__ = None;
                let mut allow_partial_message__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxRequestBytes => {
                            if max_request_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestBytes"));
                            }
                            max_request_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AllowPartialMessage => {
                            if allow_partial_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowPartialMessage"));
                            }
                            allow_partial_message__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BufferSettings {
                    max_request_bytes: max_request_bytes__.unwrap_or_default(),
                    allow_partial_message: allow_partial_message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ext_authz.v2.BufferSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckSettings {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.context_extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ext_authz.v2.CheckSettings", len)?;
        if !self.context_extensions.is_empty() {
            struct_ser.serialize_field("contextExtensions", &self.context_extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckSettings {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "context_extensions",
            "contextExtensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContextExtensions,
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
                            "contextExtensions" | "context_extensions" => Ok(GeneratedField::ContextExtensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckSettings;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ext_authz.v2.CheckSettings")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckSettings, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut context_extensions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ContextExtensions => {
                            if context_extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contextExtensions"));
                            }
                            context_extensions__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(CheckSettings {
                    context_extensions: context_extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ext_authz.v2.CheckSettings", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtAuthz {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failure_mode_allow {
            len += 1;
        }
        if self.use_alpha {
            len += 1;
        }
        if self.with_request_body.is_some() {
            len += 1;
        }
        if self.clear_route_cache {
            len += 1;
        }
        if self.status_on_error.is_some() {
            len += 1;
        }
        if !self.metadata_context_namespaces.is_empty() {
            len += 1;
        }
        if self.filter_enabled.is_some() {
            len += 1;
        }
        if self.deny_at_disable.is_some() {
            len += 1;
        }
        if self.include_peer_certificate {
            len += 1;
        }
        if self.services.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ext_authz.v2.ExtAuthz", len)?;
        if self.failure_mode_allow {
            struct_ser.serialize_field("failureModeAllow", &self.failure_mode_allow)?;
        }
        if self.use_alpha {
            struct_ser.serialize_field("useAlpha", &self.use_alpha)?;
        }
        if let Some(v) = self.with_request_body.as_ref() {
            struct_ser.serialize_field("withRequestBody", v)?;
        }
        if self.clear_route_cache {
            struct_ser.serialize_field("clearRouteCache", &self.clear_route_cache)?;
        }
        if let Some(v) = self.status_on_error.as_ref() {
            struct_ser.serialize_field("statusOnError", v)?;
        }
        if !self.metadata_context_namespaces.is_empty() {
            struct_ser.serialize_field("metadataContextNamespaces", &self.metadata_context_namespaces)?;
        }
        if let Some(v) = self.filter_enabled.as_ref() {
            struct_ser.serialize_field("filterEnabled", v)?;
        }
        if let Some(v) = self.deny_at_disable.as_ref() {
            struct_ser.serialize_field("denyAtDisable", v)?;
        }
        if self.include_peer_certificate {
            struct_ser.serialize_field("includePeerCertificate", &self.include_peer_certificate)?;
        }
        if let Some(v) = self.services.as_ref() {
            match v {
                ext_authz::Services::GrpcService(v) => {
                    struct_ser.serialize_field("grpcService", v)?;
                }
                ext_authz::Services::HttpService(v) => {
                    struct_ser.serialize_field("httpService", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtAuthz {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failure_mode_allow",
            "failureModeAllow",
            "use_alpha",
            "useAlpha",
            "with_request_body",
            "withRequestBody",
            "clear_route_cache",
            "clearRouteCache",
            "status_on_error",
            "statusOnError",
            "metadata_context_namespaces",
            "metadataContextNamespaces",
            "filter_enabled",
            "filterEnabled",
            "deny_at_disable",
            "denyAtDisable",
            "include_peer_certificate",
            "includePeerCertificate",
            "grpc_service",
            "grpcService",
            "http_service",
            "httpService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailureModeAllow,
            UseAlpha,
            WithRequestBody,
            ClearRouteCache,
            StatusOnError,
            MetadataContextNamespaces,
            FilterEnabled,
            DenyAtDisable,
            IncludePeerCertificate,
            GrpcService,
            HttpService,
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
                            "failureModeAllow" | "failure_mode_allow" => Ok(GeneratedField::FailureModeAllow),
                            "useAlpha" | "use_alpha" => Ok(GeneratedField::UseAlpha),
                            "withRequestBody" | "with_request_body" => Ok(GeneratedField::WithRequestBody),
                            "clearRouteCache" | "clear_route_cache" => Ok(GeneratedField::ClearRouteCache),
                            "statusOnError" | "status_on_error" => Ok(GeneratedField::StatusOnError),
                            "metadataContextNamespaces" | "metadata_context_namespaces" => Ok(GeneratedField::MetadataContextNamespaces),
                            "filterEnabled" | "filter_enabled" => Ok(GeneratedField::FilterEnabled),
                            "denyAtDisable" | "deny_at_disable" => Ok(GeneratedField::DenyAtDisable),
                            "includePeerCertificate" | "include_peer_certificate" => Ok(GeneratedField::IncludePeerCertificate),
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "httpService" | "http_service" => Ok(GeneratedField::HttpService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtAuthz;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ext_authz.v2.ExtAuthz")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtAuthz, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failure_mode_allow__ = None;
                let mut use_alpha__ = None;
                let mut with_request_body__ = None;
                let mut clear_route_cache__ = None;
                let mut status_on_error__ = None;
                let mut metadata_context_namespaces__ = None;
                let mut filter_enabled__ = None;
                let mut deny_at_disable__ = None;
                let mut include_peer_certificate__ = None;
                let mut services__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FailureModeAllow => {
                            if failure_mode_allow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureModeAllow"));
                            }
                            failure_mode_allow__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseAlpha => {
                            if use_alpha__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useAlpha"));
                            }
                            use_alpha__ = Some(map.next_value()?);
                        }
                        GeneratedField::WithRequestBody => {
                            if with_request_body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withRequestBody"));
                            }
                            with_request_body__ = map.next_value()?;
                        }
                        GeneratedField::ClearRouteCache => {
                            if clear_route_cache__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clearRouteCache"));
                            }
                            clear_route_cache__ = Some(map.next_value()?);
                        }
                        GeneratedField::StatusOnError => {
                            if status_on_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusOnError"));
                            }
                            status_on_error__ = map.next_value()?;
                        }
                        GeneratedField::MetadataContextNamespaces => {
                            if metadata_context_namespaces__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadataContextNamespaces"));
                            }
                            metadata_context_namespaces__ = Some(map.next_value()?);
                        }
                        GeneratedField::FilterEnabled => {
                            if filter_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterEnabled"));
                            }
                            filter_enabled__ = map.next_value()?;
                        }
                        GeneratedField::DenyAtDisable => {
                            if deny_at_disable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denyAtDisable"));
                            }
                            deny_at_disable__ = map.next_value()?;
                        }
                        GeneratedField::IncludePeerCertificate => {
                            if include_peer_certificate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("includePeerCertificate"));
                            }
                            include_peer_certificate__ = Some(map.next_value()?);
                        }
                        GeneratedField::GrpcService => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            services__ = map.next_value::<::std::option::Option<_>>()?.map(ext_authz::Services::GrpcService)
;
                        }
                        GeneratedField::HttpService => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpService"));
                            }
                            services__ = map.next_value::<::std::option::Option<_>>()?.map(ext_authz::Services::HttpService)
;
                        }
                    }
                }
                Ok(ExtAuthz {
                    failure_mode_allow: failure_mode_allow__.unwrap_or_default(),
                    use_alpha: use_alpha__.unwrap_or_default(),
                    with_request_body: with_request_body__,
                    clear_route_cache: clear_route_cache__.unwrap_or_default(),
                    status_on_error: status_on_error__,
                    metadata_context_namespaces: metadata_context_namespaces__.unwrap_or_default(),
                    filter_enabled: filter_enabled__,
                    deny_at_disable: deny_at_disable__,
                    include_peer_certificate: include_peer_certificate__.unwrap_or_default(),
                    services: services__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ext_authz.v2.ExtAuthz", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtAuthzPerRoute {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#override.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ext_authz.v2.ExtAuthzPerRoute", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                ext_authz_per_route::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                ext_authz_per_route::Override::CheckSettings(v) => {
                    struct_ser.serialize_field("checkSettings", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtAuthzPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "check_settings",
            "checkSettings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            CheckSettings,
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
                            "disabled" => Ok(GeneratedField::Disabled),
                            "checkSettings" | "check_settings" => Ok(GeneratedField::CheckSettings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtAuthzPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ext_authz.v2.ExtAuthzPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtAuthzPerRoute, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#override__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Disabled => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(ext_authz_per_route::Override::Disabled);
                        }
                        GeneratedField::CheckSettings => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkSettings"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(ext_authz_per_route::Override::CheckSettings)
;
                        }
                    }
                }
                Ok(ExtAuthzPerRoute {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ext_authz.v2.ExtAuthzPerRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpService {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.server_uri.is_some() {
            len += 1;
        }
        if !self.path_prefix.is_empty() {
            len += 1;
        }
        if self.authorization_request.is_some() {
            len += 1;
        }
        if self.authorization_response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.ext_authz.v2.HttpService", len)?;
        if let Some(v) = self.server_uri.as_ref() {
            struct_ser.serialize_field("serverUri", v)?;
        }
        if !self.path_prefix.is_empty() {
            struct_ser.serialize_field("pathPrefix", &self.path_prefix)?;
        }
        if let Some(v) = self.authorization_request.as_ref() {
            struct_ser.serialize_field("authorizationRequest", v)?;
        }
        if let Some(v) = self.authorization_response.as_ref() {
            struct_ser.serialize_field("authorizationResponse", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpService {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "server_uri",
            "serverUri",
            "path_prefix",
            "pathPrefix",
            "authorization_request",
            "authorizationRequest",
            "authorization_response",
            "authorizationResponse",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServerUri,
            PathPrefix,
            AuthorizationRequest,
            AuthorizationResponse,
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
                            "serverUri" | "server_uri" => Ok(GeneratedField::ServerUri),
                            "pathPrefix" | "path_prefix" => Ok(GeneratedField::PathPrefix),
                            "authorizationRequest" | "authorization_request" => Ok(GeneratedField::AuthorizationRequest),
                            "authorizationResponse" | "authorization_response" => Ok(GeneratedField::AuthorizationResponse),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpService;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.http.ext_authz.v2.HttpService")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpService, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut server_uri__ = None;
                let mut path_prefix__ = None;
                let mut authorization_request__ = None;
                let mut authorization_response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ServerUri => {
                            if server_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverUri"));
                            }
                            server_uri__ = map.next_value()?;
                        }
                        GeneratedField::PathPrefix => {
                            if path_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pathPrefix"));
                            }
                            path_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::AuthorizationRequest => {
                            if authorization_request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationRequest"));
                            }
                            authorization_request__ = map.next_value()?;
                        }
                        GeneratedField::AuthorizationResponse => {
                            if authorization_response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizationResponse"));
                            }
                            authorization_response__ = map.next_value()?;
                        }
                    }
                }
                Ok(HttpService {
                    server_uri: server_uri__,
                    path_prefix: path_prefix__.unwrap_or_default(),
                    authorization_request: authorization_request__,
                    authorization_response: authorization_response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.ext_authz.v2.HttpService", FIELDS, GeneratedVisitor)
    }
}
