// @generated
impl serde::Serialize for GrpcJsonTranscoder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.services.is_empty() {
            len += 1;
        }
        if self.print_options.is_some() {
            len += 1;
        }
        if self.match_incoming_request_route {
            len += 1;
        }
        if !self.ignored_query_parameters.is_empty() {
            len += 1;
        }
        if self.auto_mapping {
            len += 1;
        }
        if self.ignore_unknown_query_parameters {
            len += 1;
        }
        if self.convert_grpc_status {
            len += 1;
        }
        if self.url_unescape_spec != 0 {
            len += 1;
        }
        if self.query_param_unescape_plus {
            len += 1;
        }
        if self.match_unregistered_custom_verb {
            len += 1;
        }
        if self.request_validation_options.is_some() {
            len += 1;
        }
        if self.case_insensitive_enum_parsing {
            len += 1;
        }
        if self.max_request_body_size.is_some() {
            len += 1;
        }
        if self.max_response_body_size.is_some() {
            len += 1;
        }
        if self.descriptor_set.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder", len)?;
        if !self.services.is_empty() {
            struct_ser.serialize_field("services", &self.services)?;
        }
        if let Some(v) = self.print_options.as_ref() {
            struct_ser.serialize_field("printOptions", v)?;
        }
        if self.match_incoming_request_route {
            struct_ser.serialize_field("matchIncomingRequestRoute", &self.match_incoming_request_route)?;
        }
        if !self.ignored_query_parameters.is_empty() {
            struct_ser.serialize_field("ignoredQueryParameters", &self.ignored_query_parameters)?;
        }
        if self.auto_mapping {
            struct_ser.serialize_field("autoMapping", &self.auto_mapping)?;
        }
        if self.ignore_unknown_query_parameters {
            struct_ser.serialize_field("ignoreUnknownQueryParameters", &self.ignore_unknown_query_parameters)?;
        }
        if self.convert_grpc_status {
            struct_ser.serialize_field("convertGrpcStatus", &self.convert_grpc_status)?;
        }
        if self.url_unescape_spec != 0 {
            let v = grpc_json_transcoder::UrlUnescapeSpec::from_i32(self.url_unescape_spec)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.url_unescape_spec)))?;
            struct_ser.serialize_field("urlUnescapeSpec", &v)?;
        }
        if self.query_param_unescape_plus {
            struct_ser.serialize_field("queryParamUnescapePlus", &self.query_param_unescape_plus)?;
        }
        if self.match_unregistered_custom_verb {
            struct_ser.serialize_field("matchUnregisteredCustomVerb", &self.match_unregistered_custom_verb)?;
        }
        if let Some(v) = self.request_validation_options.as_ref() {
            struct_ser.serialize_field("requestValidationOptions", v)?;
        }
        if self.case_insensitive_enum_parsing {
            struct_ser.serialize_field("caseInsensitiveEnumParsing", &self.case_insensitive_enum_parsing)?;
        }
        if let Some(v) = self.max_request_body_size.as_ref() {
            struct_ser.serialize_field("maxRequestBodySize", v)?;
        }
        if let Some(v) = self.max_response_body_size.as_ref() {
            struct_ser.serialize_field("maxResponseBodySize", v)?;
        }
        if let Some(v) = self.descriptor_set.as_ref() {
            match v {
                grpc_json_transcoder::DescriptorSet::ProtoDescriptor(v) => {
                    struct_ser.serialize_field("protoDescriptor", v)?;
                }
                grpc_json_transcoder::DescriptorSet::ProtoDescriptorBin(v) => {
                    struct_ser.serialize_field("protoDescriptorBin", pbjson::private::base64::encode(&v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GrpcJsonTranscoder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "services",
            "print_options",
            "printOptions",
            "match_incoming_request_route",
            "matchIncomingRequestRoute",
            "ignored_query_parameters",
            "ignoredQueryParameters",
            "auto_mapping",
            "autoMapping",
            "ignore_unknown_query_parameters",
            "ignoreUnknownQueryParameters",
            "convert_grpc_status",
            "convertGrpcStatus",
            "url_unescape_spec",
            "urlUnescapeSpec",
            "query_param_unescape_plus",
            "queryParamUnescapePlus",
            "match_unregistered_custom_verb",
            "matchUnregisteredCustomVerb",
            "request_validation_options",
            "requestValidationOptions",
            "case_insensitive_enum_parsing",
            "caseInsensitiveEnumParsing",
            "max_request_body_size",
            "maxRequestBodySize",
            "max_response_body_size",
            "maxResponseBodySize",
            "proto_descriptor",
            "protoDescriptor",
            "proto_descriptor_bin",
            "protoDescriptorBin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Services,
            PrintOptions,
            MatchIncomingRequestRoute,
            IgnoredQueryParameters,
            AutoMapping,
            IgnoreUnknownQueryParameters,
            ConvertGrpcStatus,
            UrlUnescapeSpec,
            QueryParamUnescapePlus,
            MatchUnregisteredCustomVerb,
            RequestValidationOptions,
            CaseInsensitiveEnumParsing,
            MaxRequestBodySize,
            MaxResponseBodySize,
            ProtoDescriptor,
            ProtoDescriptorBin,
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
                            "services" => Ok(GeneratedField::Services),
                            "printOptions" | "print_options" => Ok(GeneratedField::PrintOptions),
                            "matchIncomingRequestRoute" | "match_incoming_request_route" => Ok(GeneratedField::MatchIncomingRequestRoute),
                            "ignoredQueryParameters" | "ignored_query_parameters" => Ok(GeneratedField::IgnoredQueryParameters),
                            "autoMapping" | "auto_mapping" => Ok(GeneratedField::AutoMapping),
                            "ignoreUnknownQueryParameters" | "ignore_unknown_query_parameters" => Ok(GeneratedField::IgnoreUnknownQueryParameters),
                            "convertGrpcStatus" | "convert_grpc_status" => Ok(GeneratedField::ConvertGrpcStatus),
                            "urlUnescapeSpec" | "url_unescape_spec" => Ok(GeneratedField::UrlUnescapeSpec),
                            "queryParamUnescapePlus" | "query_param_unescape_plus" => Ok(GeneratedField::QueryParamUnescapePlus),
                            "matchUnregisteredCustomVerb" | "match_unregistered_custom_verb" => Ok(GeneratedField::MatchUnregisteredCustomVerb),
                            "requestValidationOptions" | "request_validation_options" => Ok(GeneratedField::RequestValidationOptions),
                            "caseInsensitiveEnumParsing" | "case_insensitive_enum_parsing" => Ok(GeneratedField::CaseInsensitiveEnumParsing),
                            "maxRequestBodySize" | "max_request_body_size" => Ok(GeneratedField::MaxRequestBodySize),
                            "maxResponseBodySize" | "max_response_body_size" => Ok(GeneratedField::MaxResponseBodySize),
                            "protoDescriptor" | "proto_descriptor" => Ok(GeneratedField::ProtoDescriptor),
                            "protoDescriptorBin" | "proto_descriptor_bin" => Ok(GeneratedField::ProtoDescriptorBin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GrpcJsonTranscoder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GrpcJsonTranscoder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut services__ = None;
                let mut print_options__ = None;
                let mut match_incoming_request_route__ = None;
                let mut ignored_query_parameters__ = None;
                let mut auto_mapping__ = None;
                let mut ignore_unknown_query_parameters__ = None;
                let mut convert_grpc_status__ = None;
                let mut url_unescape_spec__ = None;
                let mut query_param_unescape_plus__ = None;
                let mut match_unregistered_custom_verb__ = None;
                let mut request_validation_options__ = None;
                let mut case_insensitive_enum_parsing__ = None;
                let mut max_request_body_size__ = None;
                let mut max_response_body_size__ = None;
                let mut descriptor_set__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Services => {
                            if services__.is_some() {
                                return Err(serde::de::Error::duplicate_field("services"));
                            }
                            services__ = Some(map.next_value()?);
                        }
                        GeneratedField::PrintOptions => {
                            if print_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("printOptions"));
                            }
                            print_options__ = map.next_value()?;
                        }
                        GeneratedField::MatchIncomingRequestRoute => {
                            if match_incoming_request_route__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchIncomingRequestRoute"));
                            }
                            match_incoming_request_route__ = Some(map.next_value()?);
                        }
                        GeneratedField::IgnoredQueryParameters => {
                            if ignored_query_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoredQueryParameters"));
                            }
                            ignored_query_parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::AutoMapping => {
                            if auto_mapping__.is_some() {
                                return Err(serde::de::Error::duplicate_field("autoMapping"));
                            }
                            auto_mapping__ = Some(map.next_value()?);
                        }
                        GeneratedField::IgnoreUnknownQueryParameters => {
                            if ignore_unknown_query_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ignoreUnknownQueryParameters"));
                            }
                            ignore_unknown_query_parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConvertGrpcStatus => {
                            if convert_grpc_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("convertGrpcStatus"));
                            }
                            convert_grpc_status__ = Some(map.next_value()?);
                        }
                        GeneratedField::UrlUnescapeSpec => {
                            if url_unescape_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("urlUnescapeSpec"));
                            }
                            url_unescape_spec__ = Some(map.next_value::<grpc_json_transcoder::UrlUnescapeSpec>()? as i32);
                        }
                        GeneratedField::QueryParamUnescapePlus => {
                            if query_param_unescape_plus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryParamUnescapePlus"));
                            }
                            query_param_unescape_plus__ = Some(map.next_value()?);
                        }
                        GeneratedField::MatchUnregisteredCustomVerb => {
                            if match_unregistered_custom_verb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matchUnregisteredCustomVerb"));
                            }
                            match_unregistered_custom_verb__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestValidationOptions => {
                            if request_validation_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestValidationOptions"));
                            }
                            request_validation_options__ = map.next_value()?;
                        }
                        GeneratedField::CaseInsensitiveEnumParsing => {
                            if case_insensitive_enum_parsing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("caseInsensitiveEnumParsing"));
                            }
                            case_insensitive_enum_parsing__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxRequestBodySize => {
                            if max_request_body_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestBodySize"));
                            }
                            max_request_body_size__ = map.next_value()?;
                        }
                        GeneratedField::MaxResponseBodySize => {
                            if max_response_body_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResponseBodySize"));
                            }
                            max_response_body_size__ = map.next_value()?;
                        }
                        GeneratedField::ProtoDescriptor => {
                            if descriptor_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoDescriptor"));
                            }
                            descriptor_set__ = map.next_value::<::std::option::Option<_>>()?.map(grpc_json_transcoder::DescriptorSet::ProtoDescriptor);
                        }
                        GeneratedField::ProtoDescriptorBin => {
                            if descriptor_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protoDescriptorBin"));
                            }
                            descriptor_set__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| grpc_json_transcoder::DescriptorSet::ProtoDescriptorBin(x.0));
                        }
                    }
                }
                Ok(GrpcJsonTranscoder {
                    services: services__.unwrap_or_default(),
                    print_options: print_options__,
                    match_incoming_request_route: match_incoming_request_route__.unwrap_or_default(),
                    ignored_query_parameters: ignored_query_parameters__.unwrap_or_default(),
                    auto_mapping: auto_mapping__.unwrap_or_default(),
                    ignore_unknown_query_parameters: ignore_unknown_query_parameters__.unwrap_or_default(),
                    convert_grpc_status: convert_grpc_status__.unwrap_or_default(),
                    url_unescape_spec: url_unescape_spec__.unwrap_or_default(),
                    query_param_unescape_plus: query_param_unescape_plus__.unwrap_or_default(),
                    match_unregistered_custom_verb: match_unregistered_custom_verb__.unwrap_or_default(),
                    request_validation_options: request_validation_options__,
                    case_insensitive_enum_parsing: case_insensitive_enum_parsing__.unwrap_or_default(),
                    max_request_body_size: max_request_body_size__,
                    max_response_body_size: max_response_body_size__,
                    descriptor_set: descriptor_set__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_json_transcoder::PrintOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.add_whitespace {
            len += 1;
        }
        if self.always_print_primitive_fields {
            len += 1;
        }
        if self.always_print_enums_as_ints {
            len += 1;
        }
        if self.preserve_proto_field_names {
            len += 1;
        }
        if self.stream_newline_delimited {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.PrintOptions", len)?;
        if self.add_whitespace {
            struct_ser.serialize_field("addWhitespace", &self.add_whitespace)?;
        }
        if self.always_print_primitive_fields {
            struct_ser.serialize_field("alwaysPrintPrimitiveFields", &self.always_print_primitive_fields)?;
        }
        if self.always_print_enums_as_ints {
            struct_ser.serialize_field("alwaysPrintEnumsAsInts", &self.always_print_enums_as_ints)?;
        }
        if self.preserve_proto_field_names {
            struct_ser.serialize_field("preserveProtoFieldNames", &self.preserve_proto_field_names)?;
        }
        if self.stream_newline_delimited {
            struct_ser.serialize_field("streamNewlineDelimited", &self.stream_newline_delimited)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_json_transcoder::PrintOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "add_whitespace",
            "addWhitespace",
            "always_print_primitive_fields",
            "alwaysPrintPrimitiveFields",
            "always_print_enums_as_ints",
            "alwaysPrintEnumsAsInts",
            "preserve_proto_field_names",
            "preserveProtoFieldNames",
            "stream_newline_delimited",
            "streamNewlineDelimited",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddWhitespace,
            AlwaysPrintPrimitiveFields,
            AlwaysPrintEnumsAsInts,
            PreserveProtoFieldNames,
            StreamNewlineDelimited,
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
                            "addWhitespace" | "add_whitespace" => Ok(GeneratedField::AddWhitespace),
                            "alwaysPrintPrimitiveFields" | "always_print_primitive_fields" => Ok(GeneratedField::AlwaysPrintPrimitiveFields),
                            "alwaysPrintEnumsAsInts" | "always_print_enums_as_ints" => Ok(GeneratedField::AlwaysPrintEnumsAsInts),
                            "preserveProtoFieldNames" | "preserve_proto_field_names" => Ok(GeneratedField::PreserveProtoFieldNames),
                            "streamNewlineDelimited" | "stream_newline_delimited" => Ok(GeneratedField::StreamNewlineDelimited),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_json_transcoder::PrintOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.PrintOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_json_transcoder::PrintOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut add_whitespace__ = None;
                let mut always_print_primitive_fields__ = None;
                let mut always_print_enums_as_ints__ = None;
                let mut preserve_proto_field_names__ = None;
                let mut stream_newline_delimited__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AddWhitespace => {
                            if add_whitespace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addWhitespace"));
                            }
                            add_whitespace__ = Some(map.next_value()?);
                        }
                        GeneratedField::AlwaysPrintPrimitiveFields => {
                            if always_print_primitive_fields__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alwaysPrintPrimitiveFields"));
                            }
                            always_print_primitive_fields__ = Some(map.next_value()?);
                        }
                        GeneratedField::AlwaysPrintEnumsAsInts => {
                            if always_print_enums_as_ints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("alwaysPrintEnumsAsInts"));
                            }
                            always_print_enums_as_ints__ = Some(map.next_value()?);
                        }
                        GeneratedField::PreserveProtoFieldNames => {
                            if preserve_proto_field_names__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preserveProtoFieldNames"));
                            }
                            preserve_proto_field_names__ = Some(map.next_value()?);
                        }
                        GeneratedField::StreamNewlineDelimited => {
                            if stream_newline_delimited__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamNewlineDelimited"));
                            }
                            stream_newline_delimited__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(grpc_json_transcoder::PrintOptions {
                    add_whitespace: add_whitespace__.unwrap_or_default(),
                    always_print_primitive_fields: always_print_primitive_fields__.unwrap_or_default(),
                    always_print_enums_as_ints: always_print_enums_as_ints__.unwrap_or_default(),
                    preserve_proto_field_names: preserve_proto_field_names__.unwrap_or_default(),
                    stream_newline_delimited: stream_newline_delimited__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.PrintOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_json_transcoder::RequestValidationOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reject_unknown_method {
            len += 1;
        }
        if self.reject_unknown_query_parameters {
            len += 1;
        }
        if self.reject_binding_body_field_collisions {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.RequestValidationOptions", len)?;
        if self.reject_unknown_method {
            struct_ser.serialize_field("rejectUnknownMethod", &self.reject_unknown_method)?;
        }
        if self.reject_unknown_query_parameters {
            struct_ser.serialize_field("rejectUnknownQueryParameters", &self.reject_unknown_query_parameters)?;
        }
        if self.reject_binding_body_field_collisions {
            struct_ser.serialize_field("rejectBindingBodyFieldCollisions", &self.reject_binding_body_field_collisions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for grpc_json_transcoder::RequestValidationOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reject_unknown_method",
            "rejectUnknownMethod",
            "reject_unknown_query_parameters",
            "rejectUnknownQueryParameters",
            "reject_binding_body_field_collisions",
            "rejectBindingBodyFieldCollisions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RejectUnknownMethod,
            RejectUnknownQueryParameters,
            RejectBindingBodyFieldCollisions,
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
                            "rejectUnknownMethod" | "reject_unknown_method" => Ok(GeneratedField::RejectUnknownMethod),
                            "rejectUnknownQueryParameters" | "reject_unknown_query_parameters" => Ok(GeneratedField::RejectUnknownQueryParameters),
                            "rejectBindingBodyFieldCollisions" | "reject_binding_body_field_collisions" => Ok(GeneratedField::RejectBindingBodyFieldCollisions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_json_transcoder::RequestValidationOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.RequestValidationOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_json_transcoder::RequestValidationOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reject_unknown_method__ = None;
                let mut reject_unknown_query_parameters__ = None;
                let mut reject_binding_body_field_collisions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RejectUnknownMethod => {
                            if reject_unknown_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectUnknownMethod"));
                            }
                            reject_unknown_method__ = Some(map.next_value()?);
                        }
                        GeneratedField::RejectUnknownQueryParameters => {
                            if reject_unknown_query_parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectUnknownQueryParameters"));
                            }
                            reject_unknown_query_parameters__ = Some(map.next_value()?);
                        }
                        GeneratedField::RejectBindingBodyFieldCollisions => {
                            if reject_binding_body_field_collisions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rejectBindingBodyFieldCollisions"));
                            }
                            reject_binding_body_field_collisions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(grpc_json_transcoder::RequestValidationOptions {
                    reject_unknown_method: reject_unknown_method__.unwrap_or_default(),
                    reject_unknown_query_parameters: reject_unknown_query_parameters__.unwrap_or_default(),
                    reject_binding_body_field_collisions: reject_binding_body_field_collisions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.extensions.filters.http.grpc_json_transcoder.v3.GrpcJsonTranscoder.RequestValidationOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for grpc_json_transcoder::UrlUnescapeSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AllCharactersExceptReserved => "ALL_CHARACTERS_EXCEPT_RESERVED",
            Self::AllCharactersExceptSlash => "ALL_CHARACTERS_EXCEPT_SLASH",
            Self::AllCharacters => "ALL_CHARACTERS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for grpc_json_transcoder::UrlUnescapeSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ALL_CHARACTERS_EXCEPT_RESERVED",
            "ALL_CHARACTERS_EXCEPT_SLASH",
            "ALL_CHARACTERS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = grpc_json_transcoder::UrlUnescapeSpec;

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
                    .and_then(grpc_json_transcoder::UrlUnescapeSpec::from_i32)
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
                    .and_then(grpc_json_transcoder::UrlUnescapeSpec::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ALL_CHARACTERS_EXCEPT_RESERVED" => Ok(grpc_json_transcoder::UrlUnescapeSpec::AllCharactersExceptReserved),
                    "ALL_CHARACTERS_EXCEPT_SLASH" => Ok(grpc_json_transcoder::UrlUnescapeSpec::AllCharactersExceptSlash),
                    "ALL_CHARACTERS" => Ok(grpc_json_transcoder::UrlUnescapeSpec::AllCharacters),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
