// @generated
impl serde::Serialize for ExtProcOverrides {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.processing_mode.is_some() {
            len += 1;
        }
        if self.async_mode {
            len += 1;
        }
        if !self.request_attributes.is_empty() {
            len += 1;
        }
        if !self.response_attributes.is_empty() {
            len += 1;
        }
        if self.grpc_service.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_proc.v3.ExtProcOverrides", len)?;
        if let Some(v) = self.processing_mode.as_ref() {
            struct_ser.serialize_field("processingMode", v)?;
        }
        if self.async_mode {
            struct_ser.serialize_field("asyncMode", &self.async_mode)?;
        }
        if !self.request_attributes.is_empty() {
            struct_ser.serialize_field("requestAttributes", &self.request_attributes)?;
        }
        if !self.response_attributes.is_empty() {
            struct_ser.serialize_field("responseAttributes", &self.response_attributes)?;
        }
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtProcOverrides {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "processing_mode",
            "processingMode",
            "async_mode",
            "asyncMode",
            "request_attributes",
            "requestAttributes",
            "response_attributes",
            "responseAttributes",
            "grpc_service",
            "grpcService",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProcessingMode,
            AsyncMode,
            RequestAttributes,
            ResponseAttributes,
            GrpcService,
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
                            "processingMode" | "processing_mode" => Ok(GeneratedField::ProcessingMode),
                            "asyncMode" | "async_mode" => Ok(GeneratedField::AsyncMode),
                            "requestAttributes" | "request_attributes" => Ok(GeneratedField::RequestAttributes),
                            "responseAttributes" | "response_attributes" => Ok(GeneratedField::ResponseAttributes),
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtProcOverrides;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_proc.v3.ExtProcOverrides")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtProcOverrides, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut processing_mode__ = None;
                let mut async_mode__ = None;
                let mut request_attributes__ = None;
                let mut response_attributes__ = None;
                let mut grpc_service__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProcessingMode => {
                            if processing_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processingMode"));
                            }
                            processing_mode__ = map.next_value()?;
                        }
                        GeneratedField::AsyncMode => {
                            if async_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asyncMode"));
                            }
                            async_mode__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestAttributes => {
                            if request_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestAttributes"));
                            }
                            request_attributes__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseAttributes => {
                            if response_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseAttributes"));
                            }
                            response_attributes__ = Some(map.next_value()?);
                        }
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExtProcOverrides {
                    processing_mode: processing_mode__,
                    async_mode: async_mode__.unwrap_or_default(),
                    request_attributes: request_attributes__.unwrap_or_default(),
                    response_attributes: response_attributes__.unwrap_or_default(),
                    grpc_service: grpc_service__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_proc.v3.ExtProcOverrides", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExtProcPerRoute {
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
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_proc.v3.ExtProcPerRoute", len)?;
        if let Some(v) = self.r#override.as_ref() {
            match v {
                ext_proc_per_route::Override::Disabled(v) => {
                    struct_ser.serialize_field("disabled", v)?;
                }
                ext_proc_per_route::Override::Overrides(v) => {
                    struct_ser.serialize_field("overrides", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExtProcPerRoute {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disabled",
            "overrides",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disabled,
            Overrides,
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
                            "overrides" => Ok(GeneratedField::Overrides),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExtProcPerRoute;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_proc.v3.ExtProcPerRoute")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExtProcPerRoute, V::Error>
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
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(ext_proc_per_route::Override::Disabled);
                        }
                        GeneratedField::Overrides => {
                            if r#override__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overrides"));
                            }
                            r#override__ = map.next_value::<::std::option::Option<_>>()?.map(ext_proc_per_route::Override::Overrides)
;
                        }
                    }
                }
                Ok(ExtProcPerRoute {
                    r#override: r#override__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_proc.v3.ExtProcPerRoute", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExternalProcessor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.grpc_service.is_some() {
            len += 1;
        }
        if self.failure_mode_allow {
            len += 1;
        }
        if self.processing_mode.is_some() {
            len += 1;
        }
        if self.async_mode {
            len += 1;
        }
        if !self.request_attributes.is_empty() {
            len += 1;
        }
        if !self.response_attributes.is_empty() {
            len += 1;
        }
        if self.message_timeout.is_some() {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if self.mutation_rules.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_proc.v3.ExternalProcessor", len)?;
        if let Some(v) = self.grpc_service.as_ref() {
            struct_ser.serialize_field("grpcService", v)?;
        }
        if self.failure_mode_allow {
            struct_ser.serialize_field("failureModeAllow", &self.failure_mode_allow)?;
        }
        if let Some(v) = self.processing_mode.as_ref() {
            struct_ser.serialize_field("processingMode", v)?;
        }
        if self.async_mode {
            struct_ser.serialize_field("asyncMode", &self.async_mode)?;
        }
        if !self.request_attributes.is_empty() {
            struct_ser.serialize_field("requestAttributes", &self.request_attributes)?;
        }
        if !self.response_attributes.is_empty() {
            struct_ser.serialize_field("responseAttributes", &self.response_attributes)?;
        }
        if let Some(v) = self.message_timeout.as_ref() {
            struct_ser.serialize_field("messageTimeout", v)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if let Some(v) = self.mutation_rules.as_ref() {
            struct_ser.serialize_field("mutationRules", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExternalProcessor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "grpc_service",
            "grpcService",
            "failure_mode_allow",
            "failureModeAllow",
            "processing_mode",
            "processingMode",
            "async_mode",
            "asyncMode",
            "request_attributes",
            "requestAttributes",
            "response_attributes",
            "responseAttributes",
            "message_timeout",
            "messageTimeout",
            "stat_prefix",
            "statPrefix",
            "mutation_rules",
            "mutationRules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            GrpcService,
            FailureModeAllow,
            ProcessingMode,
            AsyncMode,
            RequestAttributes,
            ResponseAttributes,
            MessageTimeout,
            StatPrefix,
            MutationRules,
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
                            "grpcService" | "grpc_service" => Ok(GeneratedField::GrpcService),
                            "failureModeAllow" | "failure_mode_allow" => Ok(GeneratedField::FailureModeAllow),
                            "processingMode" | "processing_mode" => Ok(GeneratedField::ProcessingMode),
                            "asyncMode" | "async_mode" => Ok(GeneratedField::AsyncMode),
                            "requestAttributes" | "request_attributes" => Ok(GeneratedField::RequestAttributes),
                            "responseAttributes" | "response_attributes" => Ok(GeneratedField::ResponseAttributes),
                            "messageTimeout" | "message_timeout" => Ok(GeneratedField::MessageTimeout),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "mutationRules" | "mutation_rules" => Ok(GeneratedField::MutationRules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExternalProcessor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_proc.v3.ExternalProcessor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExternalProcessor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut grpc_service__ = None;
                let mut failure_mode_allow__ = None;
                let mut processing_mode__ = None;
                let mut async_mode__ = None;
                let mut request_attributes__ = None;
                let mut response_attributes__ = None;
                let mut message_timeout__ = None;
                let mut stat_prefix__ = None;
                let mut mutation_rules__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::GrpcService => {
                            if grpc_service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("grpcService"));
                            }
                            grpc_service__ = map.next_value()?;
                        }
                        GeneratedField::FailureModeAllow => {
                            if failure_mode_allow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failureModeAllow"));
                            }
                            failure_mode_allow__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProcessingMode => {
                            if processing_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("processingMode"));
                            }
                            processing_mode__ = map.next_value()?;
                        }
                        GeneratedField::AsyncMode => {
                            if async_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("asyncMode"));
                            }
                            async_mode__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestAttributes => {
                            if request_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestAttributes"));
                            }
                            request_attributes__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseAttributes => {
                            if response_attributes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseAttributes"));
                            }
                            response_attributes__ = Some(map.next_value()?);
                        }
                        GeneratedField::MessageTimeout => {
                            if message_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageTimeout"));
                            }
                            message_timeout__ = map.next_value()?;
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::MutationRules => {
                            if mutation_rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mutationRules"));
                            }
                            mutation_rules__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExternalProcessor {
                    grpc_service: grpc_service__,
                    failure_mode_allow: failure_mode_allow__.unwrap_or_default(),
                    processing_mode: processing_mode__,
                    async_mode: async_mode__.unwrap_or_default(),
                    request_attributes: request_attributes__.unwrap_or_default(),
                    response_attributes: response_attributes__.unwrap_or_default(),
                    message_timeout: message_timeout__,
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    mutation_rules: mutation_rules__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_proc.v3.ExternalProcessor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProcessingMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_header_mode != 0 {
            len += 1;
        }
        if self.response_header_mode != 0 {
            len += 1;
        }
        if self.request_body_mode != 0 {
            len += 1;
        }
        if self.response_body_mode != 0 {
            len += 1;
        }
        if self.request_trailer_mode != 0 {
            len += 1;
        }
        if self.response_trailer_mode != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.ext_proc.v3.ProcessingMode", len)?;
        if self.request_header_mode != 0 {
            let v = processing_mode::HeaderSendMode::from_i32(self.request_header_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.request_header_mode)))?;
            struct_ser.serialize_field("requestHeaderMode", &v)?;
        }
        if self.response_header_mode != 0 {
            let v = processing_mode::HeaderSendMode::from_i32(self.response_header_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.response_header_mode)))?;
            struct_ser.serialize_field("responseHeaderMode", &v)?;
        }
        if self.request_body_mode != 0 {
            let v = processing_mode::BodySendMode::from_i32(self.request_body_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.request_body_mode)))?;
            struct_ser.serialize_field("requestBodyMode", &v)?;
        }
        if self.response_body_mode != 0 {
            let v = processing_mode::BodySendMode::from_i32(self.response_body_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.response_body_mode)))?;
            struct_ser.serialize_field("responseBodyMode", &v)?;
        }
        if self.request_trailer_mode != 0 {
            let v = processing_mode::HeaderSendMode::from_i32(self.request_trailer_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.request_trailer_mode)))?;
            struct_ser.serialize_field("requestTrailerMode", &v)?;
        }
        if self.response_trailer_mode != 0 {
            let v = processing_mode::HeaderSendMode::from_i32(self.response_trailer_mode)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.response_trailer_mode)))?;
            struct_ser.serialize_field("responseTrailerMode", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProcessingMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_header_mode",
            "requestHeaderMode",
            "response_header_mode",
            "responseHeaderMode",
            "request_body_mode",
            "requestBodyMode",
            "response_body_mode",
            "responseBodyMode",
            "request_trailer_mode",
            "requestTrailerMode",
            "response_trailer_mode",
            "responseTrailerMode",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestHeaderMode,
            ResponseHeaderMode,
            RequestBodyMode,
            ResponseBodyMode,
            RequestTrailerMode,
            ResponseTrailerMode,
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
                            "requestHeaderMode" | "request_header_mode" => Ok(GeneratedField::RequestHeaderMode),
                            "responseHeaderMode" | "response_header_mode" => Ok(GeneratedField::ResponseHeaderMode),
                            "requestBodyMode" | "request_body_mode" => Ok(GeneratedField::RequestBodyMode),
                            "responseBodyMode" | "response_body_mode" => Ok(GeneratedField::ResponseBodyMode),
                            "requestTrailerMode" | "request_trailer_mode" => Ok(GeneratedField::RequestTrailerMode),
                            "responseTrailerMode" | "response_trailer_mode" => Ok(GeneratedField::ResponseTrailerMode),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProcessingMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.ext_proc.v3.ProcessingMode")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProcessingMode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_header_mode__ = None;
                let mut response_header_mode__ = None;
                let mut request_body_mode__ = None;
                let mut response_body_mode__ = None;
                let mut request_trailer_mode__ = None;
                let mut response_trailer_mode__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestHeaderMode => {
                            if request_header_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaderMode"));
                            }
                            request_header_mode__ = Some(map.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                        GeneratedField::ResponseHeaderMode => {
                            if response_header_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeaderMode"));
                            }
                            response_header_mode__ = Some(map.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                        GeneratedField::RequestBodyMode => {
                            if request_body_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestBodyMode"));
                            }
                            request_body_mode__ = Some(map.next_value::<processing_mode::BodySendMode>()? as i32);
                        }
                        GeneratedField::ResponseBodyMode => {
                            if response_body_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBodyMode"));
                            }
                            response_body_mode__ = Some(map.next_value::<processing_mode::BodySendMode>()? as i32);
                        }
                        GeneratedField::RequestTrailerMode => {
                            if request_trailer_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTrailerMode"));
                            }
                            request_trailer_mode__ = Some(map.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                        GeneratedField::ResponseTrailerMode => {
                            if response_trailer_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTrailerMode"));
                            }
                            response_trailer_mode__ = Some(map.next_value::<processing_mode::HeaderSendMode>()? as i32);
                        }
                    }
                }
                Ok(ProcessingMode {
                    request_header_mode: request_header_mode__.unwrap_or_default(),
                    response_header_mode: response_header_mode__.unwrap_or_default(),
                    request_body_mode: request_body_mode__.unwrap_or_default(),
                    response_body_mode: response_body_mode__.unwrap_or_default(),
                    request_trailer_mode: request_trailer_mode__.unwrap_or_default(),
                    response_trailer_mode: response_trailer_mode__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.ext_proc.v3.ProcessingMode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for processing_mode::BodySendMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::None => "NONE",
            Self::Streamed => "STREAMED",
            Self::Buffered => "BUFFERED",
            Self::BufferedPartial => "BUFFERED_PARTIAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for processing_mode::BodySendMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NONE",
            "STREAMED",
            "BUFFERED",
            "BUFFERED_PARTIAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = processing_mode::BodySendMode;

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
                    .and_then(processing_mode::BodySendMode::from_i32)
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
                    .and_then(processing_mode::BodySendMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NONE" => Ok(processing_mode::BodySendMode::None),
                    "STREAMED" => Ok(processing_mode::BodySendMode::Streamed),
                    "BUFFERED" => Ok(processing_mode::BodySendMode::Buffered),
                    "BUFFERED_PARTIAL" => Ok(processing_mode::BodySendMode::BufferedPartial),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for processing_mode::HeaderSendMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Default => "DEFAULT",
            Self::Send => "SEND",
            Self::Skip => "SKIP",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for processing_mode::HeaderSendMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEFAULT",
            "SEND",
            "SKIP",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = processing_mode::HeaderSendMode;

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
                    .and_then(processing_mode::HeaderSendMode::from_i32)
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
                    .and_then(processing_mode::HeaderSendMode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEFAULT" => Ok(processing_mode::HeaderSendMode::Default),
                    "SEND" => Ok(processing_mode::HeaderSendMode::Send),
                    "SKIP" => Ok(processing_mode::HeaderSendMode::Skip),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
