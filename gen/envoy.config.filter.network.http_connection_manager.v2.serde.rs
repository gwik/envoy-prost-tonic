// @generated
impl serde::Serialize for HttpConnectionManager {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.codec_type != 0 {
            len += 1;
        }
        if !self.stat_prefix.is_empty() {
            len += 1;
        }
        if !self.http_filters.is_empty() {
            len += 1;
        }
        if self.add_user_agent.is_some() {
            len += 1;
        }
        if self.tracing.is_some() {
            len += 1;
        }
        if self.common_http_protocol_options.is_some() {
            len += 1;
        }
        if self.http_protocol_options.is_some() {
            len += 1;
        }
        if self.http2_protocol_options.is_some() {
            len += 1;
        }
        if !self.server_name.is_empty() {
            len += 1;
        }
        if self.server_header_transformation != 0 {
            len += 1;
        }
        if self.max_request_headers_kb.is_some() {
            len += 1;
        }
        if self.idle_timeout.is_some() {
            len += 1;
        }
        if self.stream_idle_timeout.is_some() {
            len += 1;
        }
        if self.request_timeout.is_some() {
            len += 1;
        }
        if self.drain_timeout.is_some() {
            len += 1;
        }
        if self.delayed_close_timeout.is_some() {
            len += 1;
        }
        if !self.access_log.is_empty() {
            len += 1;
        }
        if self.use_remote_address.is_some() {
            len += 1;
        }
        if self.xff_num_trusted_hops != 0 {
            len += 1;
        }
        if self.internal_address_config.is_some() {
            len += 1;
        }
        if self.skip_xff_append {
            len += 1;
        }
        if !self.via.is_empty() {
            len += 1;
        }
        if self.generate_request_id.is_some() {
            len += 1;
        }
        if self.preserve_external_request_id {
            len += 1;
        }
        if self.forward_client_cert_details != 0 {
            len += 1;
        }
        if self.set_current_client_cert_details.is_some() {
            len += 1;
        }
        if self.proxy_100_continue {
            len += 1;
        }
        if self.represent_ipv4_remote_address_as_ipv4_mapped_ipv6 {
            len += 1;
        }
        if !self.upgrade_configs.is_empty() {
            len += 1;
        }
        if self.normalize_path.is_some() {
            len += 1;
        }
        if self.merge_slashes {
            len += 1;
        }
        if self.request_id_extension.is_some() {
            len += 1;
        }
        if self.route_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager", len)?;
        if self.codec_type != 0 {
            let v = http_connection_manager::CodecType::from_i32(self.codec_type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.codec_type)))?;
            struct_ser.serialize_field("codecType", &v)?;
        }
        if !self.stat_prefix.is_empty() {
            struct_ser.serialize_field("statPrefix", &self.stat_prefix)?;
        }
        if !self.http_filters.is_empty() {
            struct_ser.serialize_field("httpFilters", &self.http_filters)?;
        }
        if let Some(v) = self.add_user_agent.as_ref() {
            struct_ser.serialize_field("addUserAgent", v)?;
        }
        if let Some(v) = self.tracing.as_ref() {
            struct_ser.serialize_field("tracing", v)?;
        }
        if let Some(v) = self.common_http_protocol_options.as_ref() {
            struct_ser.serialize_field("commonHttpProtocolOptions", v)?;
        }
        if let Some(v) = self.http_protocol_options.as_ref() {
            struct_ser.serialize_field("httpProtocolOptions", v)?;
        }
        if let Some(v) = self.http2_protocol_options.as_ref() {
            struct_ser.serialize_field("http2ProtocolOptions", v)?;
        }
        if !self.server_name.is_empty() {
            struct_ser.serialize_field("serverName", &self.server_name)?;
        }
        if self.server_header_transformation != 0 {
            let v = http_connection_manager::ServerHeaderTransformation::from_i32(self.server_header_transformation)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.server_header_transformation)))?;
            struct_ser.serialize_field("serverHeaderTransformation", &v)?;
        }
        if let Some(v) = self.max_request_headers_kb.as_ref() {
            struct_ser.serialize_field("maxRequestHeadersKb", v)?;
        }
        if let Some(v) = self.idle_timeout.as_ref() {
            struct_ser.serialize_field("idleTimeout", v)?;
        }
        if let Some(v) = self.stream_idle_timeout.as_ref() {
            struct_ser.serialize_field("streamIdleTimeout", v)?;
        }
        if let Some(v) = self.request_timeout.as_ref() {
            struct_ser.serialize_field("requestTimeout", v)?;
        }
        if let Some(v) = self.drain_timeout.as_ref() {
            struct_ser.serialize_field("drainTimeout", v)?;
        }
        if let Some(v) = self.delayed_close_timeout.as_ref() {
            struct_ser.serialize_field("delayedCloseTimeout", v)?;
        }
        if !self.access_log.is_empty() {
            struct_ser.serialize_field("accessLog", &self.access_log)?;
        }
        if let Some(v) = self.use_remote_address.as_ref() {
            struct_ser.serialize_field("useRemoteAddress", v)?;
        }
        if self.xff_num_trusted_hops != 0 {
            struct_ser.serialize_field("xffNumTrustedHops", &self.xff_num_trusted_hops)?;
        }
        if let Some(v) = self.internal_address_config.as_ref() {
            struct_ser.serialize_field("internalAddressConfig", v)?;
        }
        if self.skip_xff_append {
            struct_ser.serialize_field("skipXffAppend", &self.skip_xff_append)?;
        }
        if !self.via.is_empty() {
            struct_ser.serialize_field("via", &self.via)?;
        }
        if let Some(v) = self.generate_request_id.as_ref() {
            struct_ser.serialize_field("generateRequestId", v)?;
        }
        if self.preserve_external_request_id {
            struct_ser.serialize_field("preserveExternalRequestId", &self.preserve_external_request_id)?;
        }
        if self.forward_client_cert_details != 0 {
            let v = http_connection_manager::ForwardClientCertDetails::from_i32(self.forward_client_cert_details)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.forward_client_cert_details)))?;
            struct_ser.serialize_field("forwardClientCertDetails", &v)?;
        }
        if let Some(v) = self.set_current_client_cert_details.as_ref() {
            struct_ser.serialize_field("setCurrentClientCertDetails", v)?;
        }
        if self.proxy_100_continue {
            struct_ser.serialize_field("proxy100Continue", &self.proxy_100_continue)?;
        }
        if self.represent_ipv4_remote_address_as_ipv4_mapped_ipv6 {
            struct_ser.serialize_field("representIpv4RemoteAddressAsIpv4MappedIpv6", &self.represent_ipv4_remote_address_as_ipv4_mapped_ipv6)?;
        }
        if !self.upgrade_configs.is_empty() {
            struct_ser.serialize_field("upgradeConfigs", &self.upgrade_configs)?;
        }
        if let Some(v) = self.normalize_path.as_ref() {
            struct_ser.serialize_field("normalizePath", v)?;
        }
        if self.merge_slashes {
            struct_ser.serialize_field("mergeSlashes", &self.merge_slashes)?;
        }
        if let Some(v) = self.request_id_extension.as_ref() {
            struct_ser.serialize_field("requestIdExtension", v)?;
        }
        if let Some(v) = self.route_specifier.as_ref() {
            match v {
                http_connection_manager::RouteSpecifier::Rds(v) => {
                    struct_ser.serialize_field("rds", v)?;
                }
                http_connection_manager::RouteSpecifier::RouteConfig(v) => {
                    struct_ser.serialize_field("routeConfig", v)?;
                }
                http_connection_manager::RouteSpecifier::ScopedRoutes(v) => {
                    struct_ser.serialize_field("scopedRoutes", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpConnectionManager {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "codec_type",
            "codecType",
            "stat_prefix",
            "statPrefix",
            "http_filters",
            "httpFilters",
            "add_user_agent",
            "addUserAgent",
            "tracing",
            "common_http_protocol_options",
            "commonHttpProtocolOptions",
            "http_protocol_options",
            "httpProtocolOptions",
            "http2_protocol_options",
            "http2ProtocolOptions",
            "server_name",
            "serverName",
            "server_header_transformation",
            "serverHeaderTransformation",
            "max_request_headers_kb",
            "maxRequestHeadersKb",
            "idle_timeout",
            "idleTimeout",
            "stream_idle_timeout",
            "streamIdleTimeout",
            "request_timeout",
            "requestTimeout",
            "drain_timeout",
            "drainTimeout",
            "delayed_close_timeout",
            "delayedCloseTimeout",
            "access_log",
            "accessLog",
            "use_remote_address",
            "useRemoteAddress",
            "xff_num_trusted_hops",
            "xffNumTrustedHops",
            "internal_address_config",
            "internalAddressConfig",
            "skip_xff_append",
            "skipXffAppend",
            "via",
            "generate_request_id",
            "generateRequestId",
            "preserve_external_request_id",
            "preserveExternalRequestId",
            "forward_client_cert_details",
            "forwardClientCertDetails",
            "set_current_client_cert_details",
            "setCurrentClientCertDetails",
            "proxy_100_continue",
            "proxy100Continue",
            "represent_ipv4_remote_address_as_ipv4_mapped_ipv6",
            "representIpv4RemoteAddressAsIpv4MappedIpv6",
            "upgrade_configs",
            "upgradeConfigs",
            "normalize_path",
            "normalizePath",
            "merge_slashes",
            "mergeSlashes",
            "request_id_extension",
            "requestIdExtension",
            "rds",
            "route_config",
            "routeConfig",
            "scoped_routes",
            "scopedRoutes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CodecType,
            StatPrefix,
            HttpFilters,
            AddUserAgent,
            Tracing,
            CommonHttpProtocolOptions,
            HttpProtocolOptions,
            Http2ProtocolOptions,
            ServerName,
            ServerHeaderTransformation,
            MaxRequestHeadersKb,
            IdleTimeout,
            StreamIdleTimeout,
            RequestTimeout,
            DrainTimeout,
            DelayedCloseTimeout,
            AccessLog,
            UseRemoteAddress,
            XffNumTrustedHops,
            InternalAddressConfig,
            SkipXffAppend,
            Via,
            GenerateRequestId,
            PreserveExternalRequestId,
            ForwardClientCertDetails,
            SetCurrentClientCertDetails,
            Proxy100Continue,
            RepresentIpv4RemoteAddressAsIpv4MappedIpv6,
            UpgradeConfigs,
            NormalizePath,
            MergeSlashes,
            RequestIdExtension,
            Rds,
            RouteConfig,
            ScopedRoutes,
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
                            "codecType" | "codec_type" => Ok(GeneratedField::CodecType),
                            "statPrefix" | "stat_prefix" => Ok(GeneratedField::StatPrefix),
                            "httpFilters" | "http_filters" => Ok(GeneratedField::HttpFilters),
                            "addUserAgent" | "add_user_agent" => Ok(GeneratedField::AddUserAgent),
                            "tracing" => Ok(GeneratedField::Tracing),
                            "commonHttpProtocolOptions" | "common_http_protocol_options" => Ok(GeneratedField::CommonHttpProtocolOptions),
                            "httpProtocolOptions" | "http_protocol_options" => Ok(GeneratedField::HttpProtocolOptions),
                            "http2ProtocolOptions" | "http2_protocol_options" => Ok(GeneratedField::Http2ProtocolOptions),
                            "serverName" | "server_name" => Ok(GeneratedField::ServerName),
                            "serverHeaderTransformation" | "server_header_transformation" => Ok(GeneratedField::ServerHeaderTransformation),
                            "maxRequestHeadersKb" | "max_request_headers_kb" => Ok(GeneratedField::MaxRequestHeadersKb),
                            "idleTimeout" | "idle_timeout" => Ok(GeneratedField::IdleTimeout),
                            "streamIdleTimeout" | "stream_idle_timeout" => Ok(GeneratedField::StreamIdleTimeout),
                            "requestTimeout" | "request_timeout" => Ok(GeneratedField::RequestTimeout),
                            "drainTimeout" | "drain_timeout" => Ok(GeneratedField::DrainTimeout),
                            "delayedCloseTimeout" | "delayed_close_timeout" => Ok(GeneratedField::DelayedCloseTimeout),
                            "accessLog" | "access_log" => Ok(GeneratedField::AccessLog),
                            "useRemoteAddress" | "use_remote_address" => Ok(GeneratedField::UseRemoteAddress),
                            "xffNumTrustedHops" | "xff_num_trusted_hops" => Ok(GeneratedField::XffNumTrustedHops),
                            "internalAddressConfig" | "internal_address_config" => Ok(GeneratedField::InternalAddressConfig),
                            "skipXffAppend" | "skip_xff_append" => Ok(GeneratedField::SkipXffAppend),
                            "via" => Ok(GeneratedField::Via),
                            "generateRequestId" | "generate_request_id" => Ok(GeneratedField::GenerateRequestId),
                            "preserveExternalRequestId" | "preserve_external_request_id" => Ok(GeneratedField::PreserveExternalRequestId),
                            "forwardClientCertDetails" | "forward_client_cert_details" => Ok(GeneratedField::ForwardClientCertDetails),
                            "setCurrentClientCertDetails" | "set_current_client_cert_details" => Ok(GeneratedField::SetCurrentClientCertDetails),
                            "proxy100Continue" | "proxy_100_continue" => Ok(GeneratedField::Proxy100Continue),
                            "representIpv4RemoteAddressAsIpv4MappedIpv6" | "represent_ipv4_remote_address_as_ipv4_mapped_ipv6" => Ok(GeneratedField::RepresentIpv4RemoteAddressAsIpv4MappedIpv6),
                            "upgradeConfigs" | "upgrade_configs" => Ok(GeneratedField::UpgradeConfigs),
                            "normalizePath" | "normalize_path" => Ok(GeneratedField::NormalizePath),
                            "mergeSlashes" | "merge_slashes" => Ok(GeneratedField::MergeSlashes),
                            "requestIdExtension" | "request_id_extension" => Ok(GeneratedField::RequestIdExtension),
                            "rds" => Ok(GeneratedField::Rds),
                            "routeConfig" | "route_config" => Ok(GeneratedField::RouteConfig),
                            "scopedRoutes" | "scoped_routes" => Ok(GeneratedField::ScopedRoutes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpConnectionManager;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpConnectionManager, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut codec_type__ = None;
                let mut stat_prefix__ = None;
                let mut http_filters__ = None;
                let mut add_user_agent__ = None;
                let mut tracing__ = None;
                let mut common_http_protocol_options__ = None;
                let mut http_protocol_options__ = None;
                let mut http2_protocol_options__ = None;
                let mut server_name__ = None;
                let mut server_header_transformation__ = None;
                let mut max_request_headers_kb__ = None;
                let mut idle_timeout__ = None;
                let mut stream_idle_timeout__ = None;
                let mut request_timeout__ = None;
                let mut drain_timeout__ = None;
                let mut delayed_close_timeout__ = None;
                let mut access_log__ = None;
                let mut use_remote_address__ = None;
                let mut xff_num_trusted_hops__ = None;
                let mut internal_address_config__ = None;
                let mut skip_xff_append__ = None;
                let mut via__ = None;
                let mut generate_request_id__ = None;
                let mut preserve_external_request_id__ = None;
                let mut forward_client_cert_details__ = None;
                let mut set_current_client_cert_details__ = None;
                let mut proxy_100_continue__ = None;
                let mut represent_ipv4_remote_address_as_ipv4_mapped_ipv6__ = None;
                let mut upgrade_configs__ = None;
                let mut normalize_path__ = None;
                let mut merge_slashes__ = None;
                let mut request_id_extension__ = None;
                let mut route_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CodecType => {
                            if codec_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("codecType"));
                            }
                            codec_type__ = Some(map.next_value::<http_connection_manager::CodecType>()? as i32);
                        }
                        GeneratedField::StatPrefix => {
                            if stat_prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statPrefix"));
                            }
                            stat_prefix__ = Some(map.next_value()?);
                        }
                        GeneratedField::HttpFilters => {
                            if http_filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpFilters"));
                            }
                            http_filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::AddUserAgent => {
                            if add_user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addUserAgent"));
                            }
                            add_user_agent__ = map.next_value()?;
                        }
                        GeneratedField::Tracing => {
                            if tracing__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tracing"));
                            }
                            tracing__ = map.next_value()?;
                        }
                        GeneratedField::CommonHttpProtocolOptions => {
                            if common_http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonHttpProtocolOptions"));
                            }
                            common_http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::HttpProtocolOptions => {
                            if http_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("httpProtocolOptions"));
                            }
                            http_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::Http2ProtocolOptions => {
                            if http2_protocol_options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("http2ProtocolOptions"));
                            }
                            http2_protocol_options__ = map.next_value()?;
                        }
                        GeneratedField::ServerName => {
                            if server_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverName"));
                            }
                            server_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServerHeaderTransformation => {
                            if server_header_transformation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverHeaderTransformation"));
                            }
                            server_header_transformation__ = Some(map.next_value::<http_connection_manager::ServerHeaderTransformation>()? as i32);
                        }
                        GeneratedField::MaxRequestHeadersKb => {
                            if max_request_headers_kb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRequestHeadersKb"));
                            }
                            max_request_headers_kb__ = map.next_value()?;
                        }
                        GeneratedField::IdleTimeout => {
                            if idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("idleTimeout"));
                            }
                            idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::StreamIdleTimeout => {
                            if stream_idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamIdleTimeout"));
                            }
                            stream_idle_timeout__ = map.next_value()?;
                        }
                        GeneratedField::RequestTimeout => {
                            if request_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestTimeout"));
                            }
                            request_timeout__ = map.next_value()?;
                        }
                        GeneratedField::DrainTimeout => {
                            if drain_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("drainTimeout"));
                            }
                            drain_timeout__ = map.next_value()?;
                        }
                        GeneratedField::DelayedCloseTimeout => {
                            if delayed_close_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayedCloseTimeout"));
                            }
                            delayed_close_timeout__ = map.next_value()?;
                        }
                        GeneratedField::AccessLog => {
                            if access_log__.is_some() {
                                return Err(serde::de::Error::duplicate_field("accessLog"));
                            }
                            access_log__ = Some(map.next_value()?);
                        }
                        GeneratedField::UseRemoteAddress => {
                            if use_remote_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("useRemoteAddress"));
                            }
                            use_remote_address__ = map.next_value()?;
                        }
                        GeneratedField::XffNumTrustedHops => {
                            if xff_num_trusted_hops__.is_some() {
                                return Err(serde::de::Error::duplicate_field("xffNumTrustedHops"));
                            }
                            xff_num_trusted_hops__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InternalAddressConfig => {
                            if internal_address_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalAddressConfig"));
                            }
                            internal_address_config__ = map.next_value()?;
                        }
                        GeneratedField::SkipXffAppend => {
                            if skip_xff_append__.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipXffAppend"));
                            }
                            skip_xff_append__ = Some(map.next_value()?);
                        }
                        GeneratedField::Via => {
                            if via__.is_some() {
                                return Err(serde::de::Error::duplicate_field("via"));
                            }
                            via__ = Some(map.next_value()?);
                        }
                        GeneratedField::GenerateRequestId => {
                            if generate_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("generateRequestId"));
                            }
                            generate_request_id__ = map.next_value()?;
                        }
                        GeneratedField::PreserveExternalRequestId => {
                            if preserve_external_request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("preserveExternalRequestId"));
                            }
                            preserve_external_request_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ForwardClientCertDetails => {
                            if forward_client_cert_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardClientCertDetails"));
                            }
                            forward_client_cert_details__ = Some(map.next_value::<http_connection_manager::ForwardClientCertDetails>()? as i32);
                        }
                        GeneratedField::SetCurrentClientCertDetails => {
                            if set_current_client_cert_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("setCurrentClientCertDetails"));
                            }
                            set_current_client_cert_details__ = map.next_value()?;
                        }
                        GeneratedField::Proxy100Continue => {
                            if proxy_100_continue__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proxy100Continue"));
                            }
                            proxy_100_continue__ = Some(map.next_value()?);
                        }
                        GeneratedField::RepresentIpv4RemoteAddressAsIpv4MappedIpv6 => {
                            if represent_ipv4_remote_address_as_ipv4_mapped_ipv6__.is_some() {
                                return Err(serde::de::Error::duplicate_field("representIpv4RemoteAddressAsIpv4MappedIpv6"));
                            }
                            represent_ipv4_remote_address_as_ipv4_mapped_ipv6__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpgradeConfigs => {
                            if upgrade_configs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeConfigs"));
                            }
                            upgrade_configs__ = Some(map.next_value()?);
                        }
                        GeneratedField::NormalizePath => {
                            if normalize_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("normalizePath"));
                            }
                            normalize_path__ = map.next_value()?;
                        }
                        GeneratedField::MergeSlashes => {
                            if merge_slashes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mergeSlashes"));
                            }
                            merge_slashes__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestIdExtension => {
                            if request_id_extension__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestIdExtension"));
                            }
                            request_id_extension__ = map.next_value()?;
                        }
                        GeneratedField::Rds => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rds"));
                            }
                            route_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::RouteSpecifier::Rds)
;
                        }
                        GeneratedField::RouteConfig => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfig"));
                            }
                            route_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::RouteSpecifier::RouteConfig)
;
                        }
                        GeneratedField::ScopedRoutes => {
                            if route_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRoutes"));
                            }
                            route_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(http_connection_manager::RouteSpecifier::ScopedRoutes)
;
                        }
                    }
                }
                Ok(HttpConnectionManager {
                    codec_type: codec_type__.unwrap_or_default(),
                    stat_prefix: stat_prefix__.unwrap_or_default(),
                    http_filters: http_filters__.unwrap_or_default(),
                    add_user_agent: add_user_agent__,
                    tracing: tracing__,
                    common_http_protocol_options: common_http_protocol_options__,
                    http_protocol_options: http_protocol_options__,
                    http2_protocol_options: http2_protocol_options__,
                    server_name: server_name__.unwrap_or_default(),
                    server_header_transformation: server_header_transformation__.unwrap_or_default(),
                    max_request_headers_kb: max_request_headers_kb__,
                    idle_timeout: idle_timeout__,
                    stream_idle_timeout: stream_idle_timeout__,
                    request_timeout: request_timeout__,
                    drain_timeout: drain_timeout__,
                    delayed_close_timeout: delayed_close_timeout__,
                    access_log: access_log__.unwrap_or_default(),
                    use_remote_address: use_remote_address__,
                    xff_num_trusted_hops: xff_num_trusted_hops__.unwrap_or_default(),
                    internal_address_config: internal_address_config__,
                    skip_xff_append: skip_xff_append__.unwrap_or_default(),
                    via: via__.unwrap_or_default(),
                    generate_request_id: generate_request_id__,
                    preserve_external_request_id: preserve_external_request_id__.unwrap_or_default(),
                    forward_client_cert_details: forward_client_cert_details__.unwrap_or_default(),
                    set_current_client_cert_details: set_current_client_cert_details__,
                    proxy_100_continue: proxy_100_continue__.unwrap_or_default(),
                    represent_ipv4_remote_address_as_ipv4_mapped_ipv6: represent_ipv4_remote_address_as_ipv4_mapped_ipv6__.unwrap_or_default(),
                    upgrade_configs: upgrade_configs__.unwrap_or_default(),
                    normalize_path: normalize_path__,
                    merge_slashes: merge_slashes__.unwrap_or_default(),
                    request_id_extension: request_id_extension__,
                    route_specifier: route_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::CodecType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Auto => "AUTO",
            Self::Http1 => "HTTP1",
            Self::Http2 => "HTTP2",
            Self::Http3 => "HTTP3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::CodecType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AUTO",
            "HTTP1",
            "HTTP2",
            "HTTP3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::CodecType;

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
                    .and_then(http_connection_manager::CodecType::from_i32)
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
                    .and_then(http_connection_manager::CodecType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AUTO" => Ok(http_connection_manager::CodecType::Auto),
                    "HTTP1" => Ok(http_connection_manager::CodecType::Http1),
                    "HTTP2" => Ok(http_connection_manager::CodecType::Http2),
                    "HTTP3" => Ok(http_connection_manager::CodecType::Http3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::ForwardClientCertDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Sanitize => "SANITIZE",
            Self::ForwardOnly => "FORWARD_ONLY",
            Self::AppendForward => "APPEND_FORWARD",
            Self::SanitizeSet => "SANITIZE_SET",
            Self::AlwaysForwardOnly => "ALWAYS_FORWARD_ONLY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::ForwardClientCertDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SANITIZE",
            "FORWARD_ONLY",
            "APPEND_FORWARD",
            "SANITIZE_SET",
            "ALWAYS_FORWARD_ONLY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::ForwardClientCertDetails;

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
                    .and_then(http_connection_manager::ForwardClientCertDetails::from_i32)
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
                    .and_then(http_connection_manager::ForwardClientCertDetails::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SANITIZE" => Ok(http_connection_manager::ForwardClientCertDetails::Sanitize),
                    "FORWARD_ONLY" => Ok(http_connection_manager::ForwardClientCertDetails::ForwardOnly),
                    "APPEND_FORWARD" => Ok(http_connection_manager::ForwardClientCertDetails::AppendForward),
                    "SANITIZE_SET" => Ok(http_connection_manager::ForwardClientCertDetails::SanitizeSet),
                    "ALWAYS_FORWARD_ONLY" => Ok(http_connection_manager::ForwardClientCertDetails::AlwaysForwardOnly),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::InternalAddressConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unix_sockets {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.InternalAddressConfig", len)?;
        if self.unix_sockets {
            struct_ser.serialize_field("unixSockets", &self.unix_sockets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::InternalAddressConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "unix_sockets",
            "unixSockets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnixSockets,
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
                            "unixSockets" | "unix_sockets" => Ok(GeneratedField::UnixSockets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::InternalAddressConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.InternalAddressConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http_connection_manager::InternalAddressConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut unix_sockets__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UnixSockets => {
                            if unix_sockets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unixSockets"));
                            }
                            unix_sockets__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(http_connection_manager::InternalAddressConfig {
                    unix_sockets: unix_sockets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.InternalAddressConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::ServerHeaderTransformation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Overwrite => "OVERWRITE",
            Self::AppendIfAbsent => "APPEND_IF_ABSENT",
            Self::PassThrough => "PASS_THROUGH",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::ServerHeaderTransformation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OVERWRITE",
            "APPEND_IF_ABSENT",
            "PASS_THROUGH",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::ServerHeaderTransformation;

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
                    .and_then(http_connection_manager::ServerHeaderTransformation::from_i32)
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
                    .and_then(http_connection_manager::ServerHeaderTransformation::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OVERWRITE" => Ok(http_connection_manager::ServerHeaderTransformation::Overwrite),
                    "APPEND_IF_ABSENT" => Ok(http_connection_manager::ServerHeaderTransformation::AppendIfAbsent),
                    "PASS_THROUGH" => Ok(http_connection_manager::ServerHeaderTransformation::PassThrough),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::SetCurrentClientCertDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subject.is_some() {
            len += 1;
        }
        if self.cert {
            len += 1;
        }
        if self.chain {
            len += 1;
        }
        if self.dns {
            len += 1;
        }
        if self.uri {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.SetCurrentClientCertDetails", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if self.cert {
            struct_ser.serialize_field("cert", &self.cert)?;
        }
        if self.chain {
            struct_ser.serialize_field("chain", &self.chain)?;
        }
        if self.dns {
            struct_ser.serialize_field("dns", &self.dns)?;
        }
        if self.uri {
            struct_ser.serialize_field("uri", &self.uri)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::SetCurrentClientCertDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "cert",
            "chain",
            "dns",
            "uri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Cert,
            Chain,
            Dns,
            Uri,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "cert" => Ok(GeneratedField::Cert),
                            "chain" => Ok(GeneratedField::Chain),
                            "dns" => Ok(GeneratedField::Dns),
                            "uri" => Ok(GeneratedField::Uri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::SetCurrentClientCertDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.SetCurrentClientCertDetails")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http_connection_manager::SetCurrentClientCertDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut cert__ = None;
                let mut chain__ = None;
                let mut dns__ = None;
                let mut uri__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Cert => {
                            if cert__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cert"));
                            }
                            cert__ = Some(map.next_value()?);
                        }
                        GeneratedField::Chain => {
                            if chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chain"));
                            }
                            chain__ = Some(map.next_value()?);
                        }
                        GeneratedField::Dns => {
                            if dns__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dns"));
                            }
                            dns__ = Some(map.next_value()?);
                        }
                        GeneratedField::Uri => {
                            if uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            uri__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(http_connection_manager::SetCurrentClientCertDetails {
                    subject: subject__,
                    cert: cert__.unwrap_or_default(),
                    chain: chain__.unwrap_or_default(),
                    dns: dns__.unwrap_or_default(),
                    uri: uri__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.SetCurrentClientCertDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::Tracing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.operation_name != 0 {
            len += 1;
        }
        if !self.request_headers_for_tags.is_empty() {
            len += 1;
        }
        if self.client_sampling.is_some() {
            len += 1;
        }
        if self.random_sampling.is_some() {
            len += 1;
        }
        if self.overall_sampling.is_some() {
            len += 1;
        }
        if self.verbose {
            len += 1;
        }
        if self.max_path_tag_length.is_some() {
            len += 1;
        }
        if !self.custom_tags.is_empty() {
            len += 1;
        }
        if self.provider.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.Tracing", len)?;
        if self.operation_name != 0 {
            let v = http_connection_manager::tracing::OperationName::from_i32(self.operation_name)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.operation_name)))?;
            struct_ser.serialize_field("operationName", &v)?;
        }
        if !self.request_headers_for_tags.is_empty() {
            struct_ser.serialize_field("requestHeadersForTags", &self.request_headers_for_tags)?;
        }
        if let Some(v) = self.client_sampling.as_ref() {
            struct_ser.serialize_field("clientSampling", v)?;
        }
        if let Some(v) = self.random_sampling.as_ref() {
            struct_ser.serialize_field("randomSampling", v)?;
        }
        if let Some(v) = self.overall_sampling.as_ref() {
            struct_ser.serialize_field("overallSampling", v)?;
        }
        if self.verbose {
            struct_ser.serialize_field("verbose", &self.verbose)?;
        }
        if let Some(v) = self.max_path_tag_length.as_ref() {
            struct_ser.serialize_field("maxPathTagLength", v)?;
        }
        if !self.custom_tags.is_empty() {
            struct_ser.serialize_field("customTags", &self.custom_tags)?;
        }
        if let Some(v) = self.provider.as_ref() {
            struct_ser.serialize_field("provider", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::Tracing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation_name",
            "operationName",
            "request_headers_for_tags",
            "requestHeadersForTags",
            "client_sampling",
            "clientSampling",
            "random_sampling",
            "randomSampling",
            "overall_sampling",
            "overallSampling",
            "verbose",
            "max_path_tag_length",
            "maxPathTagLength",
            "custom_tags",
            "customTags",
            "provider",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationName,
            RequestHeadersForTags,
            ClientSampling,
            RandomSampling,
            OverallSampling,
            Verbose,
            MaxPathTagLength,
            CustomTags,
            Provider,
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
                            "operationName" | "operation_name" => Ok(GeneratedField::OperationName),
                            "requestHeadersForTags" | "request_headers_for_tags" => Ok(GeneratedField::RequestHeadersForTags),
                            "clientSampling" | "client_sampling" => Ok(GeneratedField::ClientSampling),
                            "randomSampling" | "random_sampling" => Ok(GeneratedField::RandomSampling),
                            "overallSampling" | "overall_sampling" => Ok(GeneratedField::OverallSampling),
                            "verbose" => Ok(GeneratedField::Verbose),
                            "maxPathTagLength" | "max_path_tag_length" => Ok(GeneratedField::MaxPathTagLength),
                            "customTags" | "custom_tags" => Ok(GeneratedField::CustomTags),
                            "provider" => Ok(GeneratedField::Provider),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::Tracing;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.Tracing")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http_connection_manager::Tracing, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_name__ = None;
                let mut request_headers_for_tags__ = None;
                let mut client_sampling__ = None;
                let mut random_sampling__ = None;
                let mut overall_sampling__ = None;
                let mut verbose__ = None;
                let mut max_path_tag_length__ = None;
                let mut custom_tags__ = None;
                let mut provider__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OperationName => {
                            if operation_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationName"));
                            }
                            operation_name__ = Some(map.next_value::<http_connection_manager::tracing::OperationName>()? as i32);
                        }
                        GeneratedField::RequestHeadersForTags => {
                            if request_headers_for_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersForTags"));
                            }
                            request_headers_for_tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClientSampling => {
                            if client_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientSampling"));
                            }
                            client_sampling__ = map.next_value()?;
                        }
                        GeneratedField::RandomSampling => {
                            if random_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("randomSampling"));
                            }
                            random_sampling__ = map.next_value()?;
                        }
                        GeneratedField::OverallSampling => {
                            if overall_sampling__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overallSampling"));
                            }
                            overall_sampling__ = map.next_value()?;
                        }
                        GeneratedField::Verbose => {
                            if verbose__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verbose"));
                            }
                            verbose__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxPathTagLength => {
                            if max_path_tag_length__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPathTagLength"));
                            }
                            max_path_tag_length__ = map.next_value()?;
                        }
                        GeneratedField::CustomTags => {
                            if custom_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customTags"));
                            }
                            custom_tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = map.next_value()?;
                        }
                    }
                }
                Ok(http_connection_manager::Tracing {
                    operation_name: operation_name__.unwrap_or_default(),
                    request_headers_for_tags: request_headers_for_tags__.unwrap_or_default(),
                    client_sampling: client_sampling__,
                    random_sampling: random_sampling__,
                    overall_sampling: overall_sampling__,
                    verbose: verbose__.unwrap_or_default(),
                    max_path_tag_length: max_path_tag_length__,
                    custom_tags: custom_tags__.unwrap_or_default(),
                    provider: provider__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.Tracing", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::tracing::OperationName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Ingress => "INGRESS",
            Self::Egress => "EGRESS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::tracing::OperationName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INGRESS",
            "EGRESS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::tracing::OperationName;

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
                    .and_then(http_connection_manager::tracing::OperationName::from_i32)
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
                    .and_then(http_connection_manager::tracing::OperationName::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "INGRESS" => Ok(http_connection_manager::tracing::OperationName::Ingress),
                    "EGRESS" => Ok(http_connection_manager::tracing::OperationName::Egress),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for http_connection_manager::UpgradeConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.upgrade_type.is_empty() {
            len += 1;
        }
        if !self.filters.is_empty() {
            len += 1;
        }
        if self.enabled.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.UpgradeConfig", len)?;
        if !self.upgrade_type.is_empty() {
            struct_ser.serialize_field("upgradeType", &self.upgrade_type)?;
        }
        if !self.filters.is_empty() {
            struct_ser.serialize_field("filters", &self.filters)?;
        }
        if let Some(v) = self.enabled.as_ref() {
            struct_ser.serialize_field("enabled", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for http_connection_manager::UpgradeConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "upgrade_type",
            "upgradeType",
            "filters",
            "enabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UpgradeType,
            Filters,
            Enabled,
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
                            "upgradeType" | "upgrade_type" => Ok(GeneratedField::UpgradeType),
                            "filters" => Ok(GeneratedField::Filters),
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_connection_manager::UpgradeConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.UpgradeConfig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<http_connection_manager::UpgradeConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut upgrade_type__ = None;
                let mut filters__ = None;
                let mut enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UpgradeType => {
                            if upgrade_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upgradeType"));
                            }
                            upgrade_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Filters => {
                            if filters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filters"));
                            }
                            filters__ = Some(map.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = map.next_value()?;
                        }
                    }
                }
                Ok(http_connection_manager::UpgradeConfig {
                    upgrade_type: upgrade_type__.unwrap_or_default(),
                    filters: filters__.unwrap_or_default(),
                    enabled: enabled__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager.UpgradeConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.config_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpFilter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.config_type.as_ref() {
            match v {
                http_filter::ConfigType::Config(v) => {
                    struct_ser.serialize_field("config", v)?;
                }
                http_filter::ConfigType::TypedConfig(v) => {
                    struct_ser.serialize_field("typedConfig", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "config",
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Config,
            TypedConfig,
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
                            "name" => Ok(GeneratedField::Name),
                            "config" => Ok(GeneratedField::Config),
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.HttpFilter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut config_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(http_filter::ConfigType::Config)
;
                        }
                        GeneratedField::TypedConfig => {
                            if config_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            config_type__ = map.next_value::<::std::option::Option<_>>()?.map(http_filter::ConfigType::TypedConfig)
;
                        }
                    }
                }
                Ok(HttpFilter {
                    name: name__.unwrap_or_default(),
                    config_type: config_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.HttpFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.config_source.is_some() {
            len += 1;
        }
        if !self.route_config_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.Rds", len)?;
        if let Some(v) = self.config_source.as_ref() {
            struct_ser.serialize_field("configSource", v)?;
        }
        if !self.route_config_name.is_empty() {
            struct_ser.serialize_field("routeConfigName", &self.route_config_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "config_source",
            "configSource",
            "route_config_name",
            "routeConfigName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConfigSource,
            RouteConfigName,
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
                            "configSource" | "config_source" => Ok(GeneratedField::ConfigSource),
                            "routeConfigName" | "route_config_name" => Ok(GeneratedField::RouteConfigName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.Rds")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Rds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut config_source__ = None;
                let mut route_config_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConfigSource => {
                            if config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("configSource"));
                            }
                            config_source__ = map.next_value()?;
                        }
                        GeneratedField::RouteConfigName => {
                            if route_config_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeConfigName"));
                            }
                            route_config_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Rds {
                    config_source: config_source__,
                    route_config_name: route_config_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.Rds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RequestIdExtension {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.typed_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.RequestIDExtension", len)?;
        if let Some(v) = self.typed_config.as_ref() {
            struct_ser.serialize_field("typedConfig", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RequestIdExtension {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "typed_config",
            "typedConfig",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypedConfig,
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
                            "typedConfig" | "typed_config" => Ok(GeneratedField::TypedConfig),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RequestIdExtension;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.RequestIDExtension")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RequestIdExtension, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut typed_config__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TypedConfig => {
                            if typed_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typedConfig"));
                            }
                            typed_config__ = map.next_value()?;
                        }
                    }
                }
                Ok(RequestIdExtension {
                    typed_config: typed_config__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.RequestIDExtension", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRds {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.scoped_rds_config_source.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRds", len)?;
        if let Some(v) = self.scoped_rds_config_source.as_ref() {
            struct_ser.serialize_field("scopedRdsConfigSource", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRds {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scoped_rds_config_source",
            "scopedRdsConfigSource",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScopedRdsConfigSource,
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
                            "scopedRdsConfigSource" | "scoped_rds_config_source" => Ok(GeneratedField::ScopedRdsConfigSource),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRds;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.ScopedRds")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScopedRds, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scoped_rds_config_source__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ScopedRdsConfigSource => {
                            if scoped_rds_config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRdsConfigSource"));
                            }
                            scoped_rds_config_source__ = map.next_value()?;
                        }
                    }
                }
                Ok(ScopedRds {
                    scoped_rds_config_source: scoped_rds_config_source__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRds", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRouteConfigurationsList {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.scoped_route_configurations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRouteConfigurationsList", len)?;
        if !self.scoped_route_configurations.is_empty() {
            struct_ser.serialize_field("scopedRouteConfigurations", &self.scoped_route_configurations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRouteConfigurationsList {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "scoped_route_configurations",
            "scopedRouteConfigurations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ScopedRouteConfigurations,
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
                            "scopedRouteConfigurations" | "scoped_route_configurations" => Ok(GeneratedField::ScopedRouteConfigurations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRouteConfigurationsList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.ScopedRouteConfigurationsList")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScopedRouteConfigurationsList, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut scoped_route_configurations__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ScopedRouteConfigurations => {
                            if scoped_route_configurations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigurations"));
                            }
                            scoped_route_configurations__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ScopedRouteConfigurationsList {
                    scoped_route_configurations: scoped_route_configurations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRouteConfigurationsList", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ScopedRoutes {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if self.scope_key_builder.is_some() {
            len += 1;
        }
        if self.rds_config_source.is_some() {
            len += 1;
        }
        if self.config_specifier.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if let Some(v) = self.scope_key_builder.as_ref() {
            struct_ser.serialize_field("scopeKeyBuilder", v)?;
        }
        if let Some(v) = self.rds_config_source.as_ref() {
            struct_ser.serialize_field("rdsConfigSource", v)?;
        }
        if let Some(v) = self.config_specifier.as_ref() {
            match v {
                scoped_routes::ConfigSpecifier::ScopedRouteConfigurationsList(v) => {
                    struct_ser.serialize_field("scopedRouteConfigurationsList", v)?;
                }
                scoped_routes::ConfigSpecifier::ScopedRds(v) => {
                    struct_ser.serialize_field("scopedRds", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ScopedRoutes {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "scope_key_builder",
            "scopeKeyBuilder",
            "rds_config_source",
            "rdsConfigSource",
            "scoped_route_configurations_list",
            "scopedRouteConfigurationsList",
            "scoped_rds",
            "scopedRds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ScopeKeyBuilder,
            RdsConfigSource,
            ScopedRouteConfigurationsList,
            ScopedRds,
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
                            "name" => Ok(GeneratedField::Name),
                            "scopeKeyBuilder" | "scope_key_builder" => Ok(GeneratedField::ScopeKeyBuilder),
                            "rdsConfigSource" | "rds_config_source" => Ok(GeneratedField::RdsConfigSource),
                            "scopedRouteConfigurationsList" | "scoped_route_configurations_list" => Ok(GeneratedField::ScopedRouteConfigurationsList),
                            "scopedRds" | "scoped_rds" => Ok(GeneratedField::ScopedRds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ScopedRoutes;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ScopedRoutes, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut scope_key_builder__ = None;
                let mut rds_config_source__ = None;
                let mut config_specifier__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ScopeKeyBuilder => {
                            if scope_key_builder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopeKeyBuilder"));
                            }
                            scope_key_builder__ = map.next_value()?;
                        }
                        GeneratedField::RdsConfigSource => {
                            if rds_config_source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rdsConfigSource"));
                            }
                            rds_config_source__ = map.next_value()?;
                        }
                        GeneratedField::ScopedRouteConfigurationsList => {
                            if config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRouteConfigurationsList"));
                            }
                            config_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(scoped_routes::ConfigSpecifier::ScopedRouteConfigurationsList)
;
                        }
                        GeneratedField::ScopedRds => {
                            if config_specifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scopedRds"));
                            }
                            config_specifier__ = map.next_value::<::std::option::Option<_>>()?.map(scoped_routes::ConfigSpecifier::ScopedRds)
;
                        }
                    }
                }
                Ok(ScopedRoutes {
                    name: name__.unwrap_or_default(),
                    scope_key_builder: scope_key_builder__,
                    rds_config_source: rds_config_source__,
                    config_specifier: config_specifier__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::ScopeKeyBuilder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.fragments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder", len)?;
        if !self.fragments.is_empty() {
            struct_ser.serialize_field("fragments", &self.fragments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::ScopeKeyBuilder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fragments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fragments,
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
                            "fragments" => Ok(GeneratedField::Fragments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::ScopeKeyBuilder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_routes::ScopeKeyBuilder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fragments__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fragments => {
                            if fragments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fragments"));
                            }
                            fragments__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(scoped_routes::ScopeKeyBuilder {
                    fragments: fragments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::scope_key_builder::FragmentBuilder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder", len)?;
        if let Some(v) = self.r#type.as_ref() {
            match v {
                scoped_routes::scope_key_builder::fragment_builder::Type::HeaderValueExtractor(v) => {
                    struct_ser.serialize_field("headerValueExtractor", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::scope_key_builder::FragmentBuilder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header_value_extractor",
            "headerValueExtractor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HeaderValueExtractor,
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
                            "headerValueExtractor" | "header_value_extractor" => Ok(GeneratedField::HeaderValueExtractor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::scope_key_builder::FragmentBuilder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_routes::scope_key_builder::FragmentBuilder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HeaderValueExtractor => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headerValueExtractor"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<_>>()?.map(scoped_routes::scope_key_builder::fragment_builder::Type::HeaderValueExtractor)
;
                        }
                    }
                }
                Ok(scoped_routes::scope_key_builder::FragmentBuilder {
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.element_separator.is_empty() {
            len += 1;
        }
        if self.extract_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.element_separator.is_empty() {
            struct_ser.serialize_field("elementSeparator", &self.element_separator)?;
        }
        if let Some(v) = self.extract_type.as_ref() {
            match v {
                scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Index(v) => {
                    struct_ser.serialize_field("index", v)?;
                }
                scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Element(v) => {
                    struct_ser.serialize_field("element", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "element_separator",
            "elementSeparator",
            "index",
            "element",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ElementSeparator,
            Index,
            Element,
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
                            "name" => Ok(GeneratedField::Name),
                            "elementSeparator" | "element_separator" => Ok(GeneratedField::ElementSeparator),
                            "index" => Ok(GeneratedField::Index),
                            "element" => Ok(GeneratedField::Element),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut element_separator__ = None;
                let mut extract_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ElementSeparator => {
                            if element_separator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("elementSeparator"));
                            }
                            element_separator__ = Some(map.next_value()?);
                        }
                        GeneratedField::Index => {
                            if extract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            extract_type__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Index(x.0));
                        }
                        GeneratedField::Element => {
                            if extract_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("element"));
                            }
                            extract_type__ = map.next_value::<::std::option::Option<_>>()?.map(scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::ExtractType::Element)
;
                        }
                    }
                }
                Ok(scoped_routes::scope_key_builder::fragment_builder::HeaderValueExtractor {
                    name: name__.unwrap_or_default(),
                    element_separator: element_separator__.unwrap_or_default(),
                    extract_type: extract_type__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.separator.is_empty() {
            len += 1;
        }
        if !self.key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement", len)?;
        if !self.separator.is_empty() {
            struct_ser.serialize_field("separator", &self.separator)?;
        }
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "separator",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Separator,
            Key,
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
                            "separator" => Ok(GeneratedField::Separator),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut separator__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Separator => {
                            if separator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("separator"));
                            }
                            separator__ = Some(map.next_value()?);
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(scoped_routes::scope_key_builder::fragment_builder::header_value_extractor::KvElement {
                    separator: separator__.unwrap_or_default(),
                    key: key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.filter.network.http_connection_manager.v2.ScopedRoutes.ScopeKeyBuilder.FragmentBuilder.HeaderValueExtractor.KvElement", FIELDS, GeneratedVisitor)
    }
}
