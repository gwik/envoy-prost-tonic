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
        if self.descriptor_set.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.transcoder.v2.GrpcJsonTranscoder", len)?;
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
                formatter.write_str("struct envoy.config.filter.http.transcoder.v2.GrpcJsonTranscoder")
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
                    descriptor_set: descriptor_set__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.transcoder.v2.GrpcJsonTranscoder", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.http.transcoder.v2.GrpcJsonTranscoder.PrintOptions", len)?;
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AddWhitespace,
            AlwaysPrintPrimitiveFields,
            AlwaysPrintEnumsAsInts,
            PreserveProtoFieldNames,
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
                formatter.write_str("struct envoy.config.filter.http.transcoder.v2.GrpcJsonTranscoder.PrintOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<grpc_json_transcoder::PrintOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut add_whitespace__ = None;
                let mut always_print_primitive_fields__ = None;
                let mut always_print_enums_as_ints__ = None;
                let mut preserve_proto_field_names__ = None;
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
                    }
                }
                Ok(grpc_json_transcoder::PrintOptions {
                    add_whitespace: add_whitespace__.unwrap_or_default(),
                    always_print_primitive_fields: always_print_primitive_fields__.unwrap_or_default(),
                    always_print_enums_as_ints: always_print_enums_as_ints__.unwrap_or_default(),
                    preserve_proto_field_names: preserve_proto_field_names__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.http.transcoder.v2.GrpcJsonTranscoder.PrintOptions", FIELDS, GeneratedVisitor)
    }
}
