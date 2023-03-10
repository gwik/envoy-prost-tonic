// @generated
impl serde::Serialize for AccessLogCommon {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sample_rate != 0. {
            len += 1;
        }
        if self.downstream_remote_address.is_some() {
            len += 1;
        }
        if self.downstream_local_address.is_some() {
            len += 1;
        }
        if self.tls_properties.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.time_to_last_rx_byte.is_some() {
            len += 1;
        }
        if self.time_to_first_upstream_tx_byte.is_some() {
            len += 1;
        }
        if self.time_to_last_upstream_tx_byte.is_some() {
            len += 1;
        }
        if self.time_to_first_upstream_rx_byte.is_some() {
            len += 1;
        }
        if self.time_to_last_upstream_rx_byte.is_some() {
            len += 1;
        }
        if self.time_to_first_downstream_tx_byte.is_some() {
            len += 1;
        }
        if self.time_to_last_downstream_tx_byte.is_some() {
            len += 1;
        }
        if self.upstream_remote_address.is_some() {
            len += 1;
        }
        if self.upstream_local_address.is_some() {
            len += 1;
        }
        if !self.upstream_cluster.is_empty() {
            len += 1;
        }
        if self.response_flags.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        if !self.upstream_transport_failure_reason.is_empty() {
            len += 1;
        }
        if !self.route_name.is_empty() {
            len += 1;
        }
        if self.downstream_direct_remote_address.is_some() {
            len += 1;
        }
        if !self.filter_state_objects.is_empty() {
            len += 1;
        }
        if !self.custom_tags.is_empty() {
            len += 1;
        }
        if self.duration.is_some() {
            len += 1;
        }
        if self.upstream_request_attempt_count != 0 {
            len += 1;
        }
        if !self.connection_termination_details.is_empty() {
            len += 1;
        }
        if !self.stream_id.is_empty() {
            len += 1;
        }
        if self.intermediate_log_entry {
            len += 1;
        }
        if !self.downstream_transport_failure_reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.AccessLogCommon", len)?;
        if self.sample_rate != 0. {
            struct_ser.serialize_field("sampleRate", &self.sample_rate)?;
        }
        if let Some(v) = self.downstream_remote_address.as_ref() {
            struct_ser.serialize_field("downstreamRemoteAddress", v)?;
        }
        if let Some(v) = self.downstream_local_address.as_ref() {
            struct_ser.serialize_field("downstreamLocalAddress", v)?;
        }
        if let Some(v) = self.tls_properties.as_ref() {
            struct_ser.serialize_field("tlsProperties", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", v)?;
        }
        if let Some(v) = self.time_to_last_rx_byte.as_ref() {
            struct_ser.serialize_field("timeToLastRxByte", v)?;
        }
        if let Some(v) = self.time_to_first_upstream_tx_byte.as_ref() {
            struct_ser.serialize_field("timeToFirstUpstreamTxByte", v)?;
        }
        if let Some(v) = self.time_to_last_upstream_tx_byte.as_ref() {
            struct_ser.serialize_field("timeToLastUpstreamTxByte", v)?;
        }
        if let Some(v) = self.time_to_first_upstream_rx_byte.as_ref() {
            struct_ser.serialize_field("timeToFirstUpstreamRxByte", v)?;
        }
        if let Some(v) = self.time_to_last_upstream_rx_byte.as_ref() {
            struct_ser.serialize_field("timeToLastUpstreamRxByte", v)?;
        }
        if let Some(v) = self.time_to_first_downstream_tx_byte.as_ref() {
            struct_ser.serialize_field("timeToFirstDownstreamTxByte", v)?;
        }
        if let Some(v) = self.time_to_last_downstream_tx_byte.as_ref() {
            struct_ser.serialize_field("timeToLastDownstreamTxByte", v)?;
        }
        if let Some(v) = self.upstream_remote_address.as_ref() {
            struct_ser.serialize_field("upstreamRemoteAddress", v)?;
        }
        if let Some(v) = self.upstream_local_address.as_ref() {
            struct_ser.serialize_field("upstreamLocalAddress", v)?;
        }
        if !self.upstream_cluster.is_empty() {
            struct_ser.serialize_field("upstreamCluster", &self.upstream_cluster)?;
        }
        if let Some(v) = self.response_flags.as_ref() {
            struct_ser.serialize_field("responseFlags", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if !self.upstream_transport_failure_reason.is_empty() {
            struct_ser.serialize_field("upstreamTransportFailureReason", &self.upstream_transport_failure_reason)?;
        }
        if !self.route_name.is_empty() {
            struct_ser.serialize_field("routeName", &self.route_name)?;
        }
        if let Some(v) = self.downstream_direct_remote_address.as_ref() {
            struct_ser.serialize_field("downstreamDirectRemoteAddress", v)?;
        }
        if !self.filter_state_objects.is_empty() {
            struct_ser.serialize_field("filterStateObjects", &self.filter_state_objects)?;
        }
        if !self.custom_tags.is_empty() {
            struct_ser.serialize_field("customTags", &self.custom_tags)?;
        }
        if let Some(v) = self.duration.as_ref() {
            struct_ser.serialize_field("duration", v)?;
        }
        if self.upstream_request_attempt_count != 0 {
            struct_ser.serialize_field("upstreamRequestAttemptCount", &self.upstream_request_attempt_count)?;
        }
        if !self.connection_termination_details.is_empty() {
            struct_ser.serialize_field("connectionTerminationDetails", &self.connection_termination_details)?;
        }
        if !self.stream_id.is_empty() {
            struct_ser.serialize_field("streamId", &self.stream_id)?;
        }
        if self.intermediate_log_entry {
            struct_ser.serialize_field("intermediateLogEntry", &self.intermediate_log_entry)?;
        }
        if !self.downstream_transport_failure_reason.is_empty() {
            struct_ser.serialize_field("downstreamTransportFailureReason", &self.downstream_transport_failure_reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccessLogCommon {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sample_rate",
            "sampleRate",
            "downstream_remote_address",
            "downstreamRemoteAddress",
            "downstream_local_address",
            "downstreamLocalAddress",
            "tls_properties",
            "tlsProperties",
            "start_time",
            "startTime",
            "time_to_last_rx_byte",
            "timeToLastRxByte",
            "time_to_first_upstream_tx_byte",
            "timeToFirstUpstreamTxByte",
            "time_to_last_upstream_tx_byte",
            "timeToLastUpstreamTxByte",
            "time_to_first_upstream_rx_byte",
            "timeToFirstUpstreamRxByte",
            "time_to_last_upstream_rx_byte",
            "timeToLastUpstreamRxByte",
            "time_to_first_downstream_tx_byte",
            "timeToFirstDownstreamTxByte",
            "time_to_last_downstream_tx_byte",
            "timeToLastDownstreamTxByte",
            "upstream_remote_address",
            "upstreamRemoteAddress",
            "upstream_local_address",
            "upstreamLocalAddress",
            "upstream_cluster",
            "upstreamCluster",
            "response_flags",
            "responseFlags",
            "metadata",
            "upstream_transport_failure_reason",
            "upstreamTransportFailureReason",
            "route_name",
            "routeName",
            "downstream_direct_remote_address",
            "downstreamDirectRemoteAddress",
            "filter_state_objects",
            "filterStateObjects",
            "custom_tags",
            "customTags",
            "duration",
            "upstream_request_attempt_count",
            "upstreamRequestAttemptCount",
            "connection_termination_details",
            "connectionTerminationDetails",
            "stream_id",
            "streamId",
            "intermediate_log_entry",
            "intermediateLogEntry",
            "downstream_transport_failure_reason",
            "downstreamTransportFailureReason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SampleRate,
            DownstreamRemoteAddress,
            DownstreamLocalAddress,
            TlsProperties,
            StartTime,
            TimeToLastRxByte,
            TimeToFirstUpstreamTxByte,
            TimeToLastUpstreamTxByte,
            TimeToFirstUpstreamRxByte,
            TimeToLastUpstreamRxByte,
            TimeToFirstDownstreamTxByte,
            TimeToLastDownstreamTxByte,
            UpstreamRemoteAddress,
            UpstreamLocalAddress,
            UpstreamCluster,
            ResponseFlags,
            Metadata,
            UpstreamTransportFailureReason,
            RouteName,
            DownstreamDirectRemoteAddress,
            FilterStateObjects,
            CustomTags,
            Duration,
            UpstreamRequestAttemptCount,
            ConnectionTerminationDetails,
            StreamId,
            IntermediateLogEntry,
            DownstreamTransportFailureReason,
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
                            "sampleRate" | "sample_rate" => Ok(GeneratedField::SampleRate),
                            "downstreamRemoteAddress" | "downstream_remote_address" => Ok(GeneratedField::DownstreamRemoteAddress),
                            "downstreamLocalAddress" | "downstream_local_address" => Ok(GeneratedField::DownstreamLocalAddress),
                            "tlsProperties" | "tls_properties" => Ok(GeneratedField::TlsProperties),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "timeToLastRxByte" | "time_to_last_rx_byte" => Ok(GeneratedField::TimeToLastRxByte),
                            "timeToFirstUpstreamTxByte" | "time_to_first_upstream_tx_byte" => Ok(GeneratedField::TimeToFirstUpstreamTxByte),
                            "timeToLastUpstreamTxByte" | "time_to_last_upstream_tx_byte" => Ok(GeneratedField::TimeToLastUpstreamTxByte),
                            "timeToFirstUpstreamRxByte" | "time_to_first_upstream_rx_byte" => Ok(GeneratedField::TimeToFirstUpstreamRxByte),
                            "timeToLastUpstreamRxByte" | "time_to_last_upstream_rx_byte" => Ok(GeneratedField::TimeToLastUpstreamRxByte),
                            "timeToFirstDownstreamTxByte" | "time_to_first_downstream_tx_byte" => Ok(GeneratedField::TimeToFirstDownstreamTxByte),
                            "timeToLastDownstreamTxByte" | "time_to_last_downstream_tx_byte" => Ok(GeneratedField::TimeToLastDownstreamTxByte),
                            "upstreamRemoteAddress" | "upstream_remote_address" => Ok(GeneratedField::UpstreamRemoteAddress),
                            "upstreamLocalAddress" | "upstream_local_address" => Ok(GeneratedField::UpstreamLocalAddress),
                            "upstreamCluster" | "upstream_cluster" => Ok(GeneratedField::UpstreamCluster),
                            "responseFlags" | "response_flags" => Ok(GeneratedField::ResponseFlags),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "upstreamTransportFailureReason" | "upstream_transport_failure_reason" => Ok(GeneratedField::UpstreamTransportFailureReason),
                            "routeName" | "route_name" => Ok(GeneratedField::RouteName),
                            "downstreamDirectRemoteAddress" | "downstream_direct_remote_address" => Ok(GeneratedField::DownstreamDirectRemoteAddress),
                            "filterStateObjects" | "filter_state_objects" => Ok(GeneratedField::FilterStateObjects),
                            "customTags" | "custom_tags" => Ok(GeneratedField::CustomTags),
                            "duration" => Ok(GeneratedField::Duration),
                            "upstreamRequestAttemptCount" | "upstream_request_attempt_count" => Ok(GeneratedField::UpstreamRequestAttemptCount),
                            "connectionTerminationDetails" | "connection_termination_details" => Ok(GeneratedField::ConnectionTerminationDetails),
                            "streamId" | "stream_id" => Ok(GeneratedField::StreamId),
                            "intermediateLogEntry" | "intermediate_log_entry" => Ok(GeneratedField::IntermediateLogEntry),
                            "downstreamTransportFailureReason" | "downstream_transport_failure_reason" => Ok(GeneratedField::DownstreamTransportFailureReason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccessLogCommon;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.AccessLogCommon")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AccessLogCommon, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sample_rate__ = None;
                let mut downstream_remote_address__ = None;
                let mut downstream_local_address__ = None;
                let mut tls_properties__ = None;
                let mut start_time__ = None;
                let mut time_to_last_rx_byte__ = None;
                let mut time_to_first_upstream_tx_byte__ = None;
                let mut time_to_last_upstream_tx_byte__ = None;
                let mut time_to_first_upstream_rx_byte__ = None;
                let mut time_to_last_upstream_rx_byte__ = None;
                let mut time_to_first_downstream_tx_byte__ = None;
                let mut time_to_last_downstream_tx_byte__ = None;
                let mut upstream_remote_address__ = None;
                let mut upstream_local_address__ = None;
                let mut upstream_cluster__ = None;
                let mut response_flags__ = None;
                let mut metadata__ = None;
                let mut upstream_transport_failure_reason__ = None;
                let mut route_name__ = None;
                let mut downstream_direct_remote_address__ = None;
                let mut filter_state_objects__ = None;
                let mut custom_tags__ = None;
                let mut duration__ = None;
                let mut upstream_request_attempt_count__ = None;
                let mut connection_termination_details__ = None;
                let mut stream_id__ = None;
                let mut intermediate_log_entry__ = None;
                let mut downstream_transport_failure_reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SampleRate => {
                            if sample_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sampleRate"));
                            }
                            sample_rate__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DownstreamRemoteAddress => {
                            if downstream_remote_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamRemoteAddress"));
                            }
                            downstream_remote_address__ = map.next_value()?;
                        }
                        GeneratedField::DownstreamLocalAddress => {
                            if downstream_local_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamLocalAddress"));
                            }
                            downstream_local_address__ = map.next_value()?;
                        }
                        GeneratedField::TlsProperties => {
                            if tls_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsProperties"));
                            }
                            tls_properties__ = map.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ = map.next_value()?;
                        }
                        GeneratedField::TimeToLastRxByte => {
                            if time_to_last_rx_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeToLastRxByte"));
                            }
                            time_to_last_rx_byte__ = map.next_value()?;
                        }
                        GeneratedField::TimeToFirstUpstreamTxByte => {
                            if time_to_first_upstream_tx_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeToFirstUpstreamTxByte"));
                            }
                            time_to_first_upstream_tx_byte__ = map.next_value()?;
                        }
                        GeneratedField::TimeToLastUpstreamTxByte => {
                            if time_to_last_upstream_tx_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeToLastUpstreamTxByte"));
                            }
                            time_to_last_upstream_tx_byte__ = map.next_value()?;
                        }
                        GeneratedField::TimeToFirstUpstreamRxByte => {
                            if time_to_first_upstream_rx_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeToFirstUpstreamRxByte"));
                            }
                            time_to_first_upstream_rx_byte__ = map.next_value()?;
                        }
                        GeneratedField::TimeToLastUpstreamRxByte => {
                            if time_to_last_upstream_rx_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeToLastUpstreamRxByte"));
                            }
                            time_to_last_upstream_rx_byte__ = map.next_value()?;
                        }
                        GeneratedField::TimeToFirstDownstreamTxByte => {
                            if time_to_first_downstream_tx_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeToFirstDownstreamTxByte"));
                            }
                            time_to_first_downstream_tx_byte__ = map.next_value()?;
                        }
                        GeneratedField::TimeToLastDownstreamTxByte => {
                            if time_to_last_downstream_tx_byte__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeToLastDownstreamTxByte"));
                            }
                            time_to_last_downstream_tx_byte__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamRemoteAddress => {
                            if upstream_remote_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamRemoteAddress"));
                            }
                            upstream_remote_address__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamLocalAddress => {
                            if upstream_local_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamLocalAddress"));
                            }
                            upstream_local_address__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamCluster => {
                            if upstream_cluster__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamCluster"));
                            }
                            upstream_cluster__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseFlags => {
                            if response_flags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseFlags"));
                            }
                            response_flags__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamTransportFailureReason => {
                            if upstream_transport_failure_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamTransportFailureReason"));
                            }
                            upstream_transport_failure_reason__ = Some(map.next_value()?);
                        }
                        GeneratedField::RouteName => {
                            if route_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("routeName"));
                            }
                            route_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DownstreamDirectRemoteAddress => {
                            if downstream_direct_remote_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamDirectRemoteAddress"));
                            }
                            downstream_direct_remote_address__ = map.next_value()?;
                        }
                        GeneratedField::FilterStateObjects => {
                            if filter_state_objects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filterStateObjects"));
                            }
                            filter_state_objects__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::CustomTags => {
                            if custom_tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("customTags"));
                            }
                            custom_tags__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Duration => {
                            if duration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            duration__ = map.next_value()?;
                        }
                        GeneratedField::UpstreamRequestAttemptCount => {
                            if upstream_request_attempt_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamRequestAttemptCount"));
                            }
                            upstream_request_attempt_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ConnectionTerminationDetails => {
                            if connection_termination_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionTerminationDetails"));
                            }
                            connection_termination_details__ = Some(map.next_value()?);
                        }
                        GeneratedField::StreamId => {
                            if stream_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamId"));
                            }
                            stream_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::IntermediateLogEntry => {
                            if intermediate_log_entry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intermediateLogEntry"));
                            }
                            intermediate_log_entry__ = Some(map.next_value()?);
                        }
                        GeneratedField::DownstreamTransportFailureReason => {
                            if downstream_transport_failure_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamTransportFailureReason"));
                            }
                            downstream_transport_failure_reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AccessLogCommon {
                    sample_rate: sample_rate__.unwrap_or_default(),
                    downstream_remote_address: downstream_remote_address__,
                    downstream_local_address: downstream_local_address__,
                    tls_properties: tls_properties__,
                    start_time: start_time__,
                    time_to_last_rx_byte: time_to_last_rx_byte__,
                    time_to_first_upstream_tx_byte: time_to_first_upstream_tx_byte__,
                    time_to_last_upstream_tx_byte: time_to_last_upstream_tx_byte__,
                    time_to_first_upstream_rx_byte: time_to_first_upstream_rx_byte__,
                    time_to_last_upstream_rx_byte: time_to_last_upstream_rx_byte__,
                    time_to_first_downstream_tx_byte: time_to_first_downstream_tx_byte__,
                    time_to_last_downstream_tx_byte: time_to_last_downstream_tx_byte__,
                    upstream_remote_address: upstream_remote_address__,
                    upstream_local_address: upstream_local_address__,
                    upstream_cluster: upstream_cluster__.unwrap_or_default(),
                    response_flags: response_flags__,
                    metadata: metadata__,
                    upstream_transport_failure_reason: upstream_transport_failure_reason__.unwrap_or_default(),
                    route_name: route_name__.unwrap_or_default(),
                    downstream_direct_remote_address: downstream_direct_remote_address__,
                    filter_state_objects: filter_state_objects__.unwrap_or_default(),
                    custom_tags: custom_tags__.unwrap_or_default(),
                    duration: duration__,
                    upstream_request_attempt_count: upstream_request_attempt_count__.unwrap_or_default(),
                    connection_termination_details: connection_termination_details__.unwrap_or_default(),
                    stream_id: stream_id__.unwrap_or_default(),
                    intermediate_log_entry: intermediate_log_entry__.unwrap_or_default(),
                    downstream_transport_failure_reason: downstream_transport_failure_reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.AccessLogCommon", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConnectionProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.received_bytes != 0 {
            len += 1;
        }
        if self.sent_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.ConnectionProperties", len)?;
        if self.received_bytes != 0 {
            struct_ser.serialize_field("receivedBytes", ToString::to_string(&self.received_bytes).as_str())?;
        }
        if self.sent_bytes != 0 {
            struct_ser.serialize_field("sentBytes", ToString::to_string(&self.sent_bytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "received_bytes",
            "receivedBytes",
            "sent_bytes",
            "sentBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReceivedBytes,
            SentBytes,
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
                            "receivedBytes" | "received_bytes" => Ok(GeneratedField::ReceivedBytes),
                            "sentBytes" | "sent_bytes" => Ok(GeneratedField::SentBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConnectionProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.ConnectionProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConnectionProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut received_bytes__ = None;
                let mut sent_bytes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReceivedBytes => {
                            if received_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("receivedBytes"));
                            }
                            received_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SentBytes => {
                            if sent_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sentBytes"));
                            }
                            sent_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConnectionProperties {
                    received_bytes: received_bytes__.unwrap_or_default(),
                    sent_bytes: sent_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.ConnectionProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpAccessLogEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_properties.is_some() {
            len += 1;
        }
        if self.protocol_version != 0 {
            len += 1;
        }
        if self.request.is_some() {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.HTTPAccessLogEntry", len)?;
        if let Some(v) = self.common_properties.as_ref() {
            struct_ser.serialize_field("commonProperties", v)?;
        }
        if self.protocol_version != 0 {
            let v = http_access_log_entry::HttpVersion::from_i32(self.protocol_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.protocol_version)))?;
            struct_ser.serialize_field("protocolVersion", &v)?;
        }
        if let Some(v) = self.request.as_ref() {
            struct_ser.serialize_field("request", v)?;
        }
        if let Some(v) = self.response.as_ref() {
            struct_ser.serialize_field("response", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpAccessLogEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_properties",
            "commonProperties",
            "protocol_version",
            "protocolVersion",
            "request",
            "response",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonProperties,
            ProtocolVersion,
            Request,
            Response,
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
                            "commonProperties" | "common_properties" => Ok(GeneratedField::CommonProperties),
                            "protocolVersion" | "protocol_version" => Ok(GeneratedField::ProtocolVersion),
                            "request" => Ok(GeneratedField::Request),
                            "response" => Ok(GeneratedField::Response),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpAccessLogEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.HTTPAccessLogEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpAccessLogEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_properties__ = None;
                let mut protocol_version__ = None;
                let mut request__ = None;
                let mut response__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonProperties => {
                            if common_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonProperties"));
                            }
                            common_properties__ = map.next_value()?;
                        }
                        GeneratedField::ProtocolVersion => {
                            if protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolVersion"));
                            }
                            protocol_version__ = Some(map.next_value::<http_access_log_entry::HttpVersion>()? as i32);
                        }
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = map.next_value()?;
                        }
                        GeneratedField::Response => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            response__ = map.next_value()?;
                        }
                    }
                }
                Ok(HttpAccessLogEntry {
                    common_properties: common_properties__,
                    protocol_version: protocol_version__.unwrap_or_default(),
                    request: request__,
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.HTTPAccessLogEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for http_access_log_entry::HttpVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ProtocolUnspecified => "PROTOCOL_UNSPECIFIED",
            Self::Http10 => "HTTP10",
            Self::Http11 => "HTTP11",
            Self::Http2 => "HTTP2",
            Self::Http3 => "HTTP3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for http_access_log_entry::HttpVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROTOCOL_UNSPECIFIED",
            "HTTP10",
            "HTTP11",
            "HTTP2",
            "HTTP3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = http_access_log_entry::HttpVersion;

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
                    .and_then(http_access_log_entry::HttpVersion::from_i32)
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
                    .and_then(http_access_log_entry::HttpVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PROTOCOL_UNSPECIFIED" => Ok(http_access_log_entry::HttpVersion::ProtocolUnspecified),
                    "HTTP10" => Ok(http_access_log_entry::HttpVersion::Http10),
                    "HTTP11" => Ok(http_access_log_entry::HttpVersion::Http11),
                    "HTTP2" => Ok(http_access_log_entry::HttpVersion::Http2),
                    "HTTP3" => Ok(http_access_log_entry::HttpVersion::Http3),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for HttpRequestProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.request_method != 0 {
            len += 1;
        }
        if !self.scheme.is_empty() {
            len += 1;
        }
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.port.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if !self.user_agent.is_empty() {
            len += 1;
        }
        if !self.referer.is_empty() {
            len += 1;
        }
        if !self.forwarded_for.is_empty() {
            len += 1;
        }
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.original_path.is_empty() {
            len += 1;
        }
        if self.request_headers_bytes != 0 {
            len += 1;
        }
        if self.request_body_bytes != 0 {
            len += 1;
        }
        if !self.request_headers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.HTTPRequestProperties", len)?;
        if self.request_method != 0 {
            let v = super::super::super::config::core::v3::RequestMethod::from_i32(self.request_method)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.request_method)))?;
            struct_ser.serialize_field("requestMethod", &v)?;
        }
        if !self.scheme.is_empty() {
            struct_ser.serialize_field("scheme", &self.scheme)?;
        }
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.port.as_ref() {
            struct_ser.serialize_field("port", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if !self.user_agent.is_empty() {
            struct_ser.serialize_field("userAgent", &self.user_agent)?;
        }
        if !self.referer.is_empty() {
            struct_ser.serialize_field("referer", &self.referer)?;
        }
        if !self.forwarded_for.is_empty() {
            struct_ser.serialize_field("forwardedFor", &self.forwarded_for)?;
        }
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.original_path.is_empty() {
            struct_ser.serialize_field("originalPath", &self.original_path)?;
        }
        if self.request_headers_bytes != 0 {
            struct_ser.serialize_field("requestHeadersBytes", ToString::to_string(&self.request_headers_bytes).as_str())?;
        }
        if self.request_body_bytes != 0 {
            struct_ser.serialize_field("requestBodyBytes", ToString::to_string(&self.request_body_bytes).as_str())?;
        }
        if !self.request_headers.is_empty() {
            struct_ser.serialize_field("requestHeaders", &self.request_headers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpRequestProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_method",
            "requestMethod",
            "scheme",
            "authority",
            "port",
            "path",
            "user_agent",
            "userAgent",
            "referer",
            "forwarded_for",
            "forwardedFor",
            "request_id",
            "requestId",
            "original_path",
            "originalPath",
            "request_headers_bytes",
            "requestHeadersBytes",
            "request_body_bytes",
            "requestBodyBytes",
            "request_headers",
            "requestHeaders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestMethod,
            Scheme,
            Authority,
            Port,
            Path,
            UserAgent,
            Referer,
            ForwardedFor,
            RequestId,
            OriginalPath,
            RequestHeadersBytes,
            RequestBodyBytes,
            RequestHeaders,
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
                            "requestMethod" | "request_method" => Ok(GeneratedField::RequestMethod),
                            "scheme" => Ok(GeneratedField::Scheme),
                            "authority" => Ok(GeneratedField::Authority),
                            "port" => Ok(GeneratedField::Port),
                            "path" => Ok(GeneratedField::Path),
                            "userAgent" | "user_agent" => Ok(GeneratedField::UserAgent),
                            "referer" => Ok(GeneratedField::Referer),
                            "forwardedFor" | "forwarded_for" => Ok(GeneratedField::ForwardedFor),
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "originalPath" | "original_path" => Ok(GeneratedField::OriginalPath),
                            "requestHeadersBytes" | "request_headers_bytes" => Ok(GeneratedField::RequestHeadersBytes),
                            "requestBodyBytes" | "request_body_bytes" => Ok(GeneratedField::RequestBodyBytes),
                            "requestHeaders" | "request_headers" => Ok(GeneratedField::RequestHeaders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpRequestProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.HTTPRequestProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpRequestProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_method__ = None;
                let mut scheme__ = None;
                let mut authority__ = None;
                let mut port__ = None;
                let mut path__ = None;
                let mut user_agent__ = None;
                let mut referer__ = None;
                let mut forwarded_for__ = None;
                let mut request_id__ = None;
                let mut original_path__ = None;
                let mut request_headers_bytes__ = None;
                let mut request_body_bytes__ = None;
                let mut request_headers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RequestMethod => {
                            if request_method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestMethod"));
                            }
                            request_method__ = Some(map.next_value::<super::super::super::config::core::v3::RequestMethod>()? as i32);
                        }
                        GeneratedField::Scheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            scheme__ = Some(map.next_value()?);
                        }
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Port => {
                            if port__.is_some() {
                                return Err(serde::de::Error::duplicate_field("port"));
                            }
                            port__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserAgent => {
                            if user_agent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userAgent"));
                            }
                            user_agent__ = Some(map.next_value()?);
                        }
                        GeneratedField::Referer => {
                            if referer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referer"));
                            }
                            referer__ = Some(map.next_value()?);
                        }
                        GeneratedField::ForwardedFor => {
                            if forwarded_for__.is_some() {
                                return Err(serde::de::Error::duplicate_field("forwardedFor"));
                            }
                            forwarded_for__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OriginalPath => {
                            if original_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalPath"));
                            }
                            original_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::RequestHeadersBytes => {
                            if request_headers_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeadersBytes"));
                            }
                            request_headers_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequestBodyBytes => {
                            if request_body_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestBodyBytes"));
                            }
                            request_body_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RequestHeaders => {
                            if request_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestHeaders"));
                            }
                            request_headers__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(HttpRequestProperties {
                    request_method: request_method__.unwrap_or_default(),
                    scheme: scheme__.unwrap_or_default(),
                    authority: authority__.unwrap_or_default(),
                    port: port__,
                    path: path__.unwrap_or_default(),
                    user_agent: user_agent__.unwrap_or_default(),
                    referer: referer__.unwrap_or_default(),
                    forwarded_for: forwarded_for__.unwrap_or_default(),
                    request_id: request_id__.unwrap_or_default(),
                    original_path: original_path__.unwrap_or_default(),
                    request_headers_bytes: request_headers_bytes__.unwrap_or_default(),
                    request_body_bytes: request_body_bytes__.unwrap_or_default(),
                    request_headers: request_headers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.HTTPRequestProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpResponseProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response_code.is_some() {
            len += 1;
        }
        if self.response_headers_bytes != 0 {
            len += 1;
        }
        if self.response_body_bytes != 0 {
            len += 1;
        }
        if !self.response_headers.is_empty() {
            len += 1;
        }
        if !self.response_trailers.is_empty() {
            len += 1;
        }
        if !self.response_code_details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.HTTPResponseProperties", len)?;
        if let Some(v) = self.response_code.as_ref() {
            struct_ser.serialize_field("responseCode", v)?;
        }
        if self.response_headers_bytes != 0 {
            struct_ser.serialize_field("responseHeadersBytes", ToString::to_string(&self.response_headers_bytes).as_str())?;
        }
        if self.response_body_bytes != 0 {
            struct_ser.serialize_field("responseBodyBytes", ToString::to_string(&self.response_body_bytes).as_str())?;
        }
        if !self.response_headers.is_empty() {
            struct_ser.serialize_field("responseHeaders", &self.response_headers)?;
        }
        if !self.response_trailers.is_empty() {
            struct_ser.serialize_field("responseTrailers", &self.response_trailers)?;
        }
        if !self.response_code_details.is_empty() {
            struct_ser.serialize_field("responseCodeDetails", &self.response_code_details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpResponseProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "response_code",
            "responseCode",
            "response_headers_bytes",
            "responseHeadersBytes",
            "response_body_bytes",
            "responseBodyBytes",
            "response_headers",
            "responseHeaders",
            "response_trailers",
            "responseTrailers",
            "response_code_details",
            "responseCodeDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResponseCode,
            ResponseHeadersBytes,
            ResponseBodyBytes,
            ResponseHeaders,
            ResponseTrailers,
            ResponseCodeDetails,
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
                            "responseCode" | "response_code" => Ok(GeneratedField::ResponseCode),
                            "responseHeadersBytes" | "response_headers_bytes" => Ok(GeneratedField::ResponseHeadersBytes),
                            "responseBodyBytes" | "response_body_bytes" => Ok(GeneratedField::ResponseBodyBytes),
                            "responseHeaders" | "response_headers" => Ok(GeneratedField::ResponseHeaders),
                            "responseTrailers" | "response_trailers" => Ok(GeneratedField::ResponseTrailers),
                            "responseCodeDetails" | "response_code_details" => Ok(GeneratedField::ResponseCodeDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpResponseProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.HTTPResponseProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpResponseProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response_code__ = None;
                let mut response_headers_bytes__ = None;
                let mut response_body_bytes__ = None;
                let mut response_headers__ = None;
                let mut response_trailers__ = None;
                let mut response_code_details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ResponseCode => {
                            if response_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseCode"));
                            }
                            response_code__ = map.next_value()?;
                        }
                        GeneratedField::ResponseHeadersBytes => {
                            if response_headers_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeadersBytes"));
                            }
                            response_headers_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseBodyBytes => {
                            if response_body_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseBodyBytes"));
                            }
                            response_body_bytes__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResponseHeaders => {
                            if response_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseHeaders"));
                            }
                            response_headers__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ResponseTrailers => {
                            if response_trailers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseTrailers"));
                            }
                            response_trailers__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ResponseCodeDetails => {
                            if response_code_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseCodeDetails"));
                            }
                            response_code_details__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(HttpResponseProperties {
                    response_code: response_code__,
                    response_headers_bytes: response_headers_bytes__.unwrap_or_default(),
                    response_body_bytes: response_body_bytes__.unwrap_or_default(),
                    response_headers: response_headers__.unwrap_or_default(),
                    response_trailers: response_trailers__.unwrap_or_default(),
                    response_code_details: response_code_details__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.HTTPResponseProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseFlags {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.failed_local_healthcheck {
            len += 1;
        }
        if self.no_healthy_upstream {
            len += 1;
        }
        if self.upstream_request_timeout {
            len += 1;
        }
        if self.local_reset {
            len += 1;
        }
        if self.upstream_remote_reset {
            len += 1;
        }
        if self.upstream_connection_failure {
            len += 1;
        }
        if self.upstream_connection_termination {
            len += 1;
        }
        if self.upstream_overflow {
            len += 1;
        }
        if self.no_route_found {
            len += 1;
        }
        if self.delay_injected {
            len += 1;
        }
        if self.fault_injected {
            len += 1;
        }
        if self.rate_limited {
            len += 1;
        }
        if self.unauthorized_details.is_some() {
            len += 1;
        }
        if self.rate_limit_service_error {
            len += 1;
        }
        if self.downstream_connection_termination {
            len += 1;
        }
        if self.upstream_retry_limit_exceeded {
            len += 1;
        }
        if self.stream_idle_timeout {
            len += 1;
        }
        if self.invalid_envoy_request_headers {
            len += 1;
        }
        if self.downstream_protocol_error {
            len += 1;
        }
        if self.upstream_max_stream_duration_reached {
            len += 1;
        }
        if self.response_from_cache_filter {
            len += 1;
        }
        if self.no_filter_config_found {
            len += 1;
        }
        if self.duration_timeout {
            len += 1;
        }
        if self.upstream_protocol_error {
            len += 1;
        }
        if self.no_cluster_found {
            len += 1;
        }
        if self.overload_manager {
            len += 1;
        }
        if self.dns_resolution_failure {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.ResponseFlags", len)?;
        if self.failed_local_healthcheck {
            struct_ser.serialize_field("failedLocalHealthcheck", &self.failed_local_healthcheck)?;
        }
        if self.no_healthy_upstream {
            struct_ser.serialize_field("noHealthyUpstream", &self.no_healthy_upstream)?;
        }
        if self.upstream_request_timeout {
            struct_ser.serialize_field("upstreamRequestTimeout", &self.upstream_request_timeout)?;
        }
        if self.local_reset {
            struct_ser.serialize_field("localReset", &self.local_reset)?;
        }
        if self.upstream_remote_reset {
            struct_ser.serialize_field("upstreamRemoteReset", &self.upstream_remote_reset)?;
        }
        if self.upstream_connection_failure {
            struct_ser.serialize_field("upstreamConnectionFailure", &self.upstream_connection_failure)?;
        }
        if self.upstream_connection_termination {
            struct_ser.serialize_field("upstreamConnectionTermination", &self.upstream_connection_termination)?;
        }
        if self.upstream_overflow {
            struct_ser.serialize_field("upstreamOverflow", &self.upstream_overflow)?;
        }
        if self.no_route_found {
            struct_ser.serialize_field("noRouteFound", &self.no_route_found)?;
        }
        if self.delay_injected {
            struct_ser.serialize_field("delayInjected", &self.delay_injected)?;
        }
        if self.fault_injected {
            struct_ser.serialize_field("faultInjected", &self.fault_injected)?;
        }
        if self.rate_limited {
            struct_ser.serialize_field("rateLimited", &self.rate_limited)?;
        }
        if let Some(v) = self.unauthorized_details.as_ref() {
            struct_ser.serialize_field("unauthorizedDetails", v)?;
        }
        if self.rate_limit_service_error {
            struct_ser.serialize_field("rateLimitServiceError", &self.rate_limit_service_error)?;
        }
        if self.downstream_connection_termination {
            struct_ser.serialize_field("downstreamConnectionTermination", &self.downstream_connection_termination)?;
        }
        if self.upstream_retry_limit_exceeded {
            struct_ser.serialize_field("upstreamRetryLimitExceeded", &self.upstream_retry_limit_exceeded)?;
        }
        if self.stream_idle_timeout {
            struct_ser.serialize_field("streamIdleTimeout", &self.stream_idle_timeout)?;
        }
        if self.invalid_envoy_request_headers {
            struct_ser.serialize_field("invalidEnvoyRequestHeaders", &self.invalid_envoy_request_headers)?;
        }
        if self.downstream_protocol_error {
            struct_ser.serialize_field("downstreamProtocolError", &self.downstream_protocol_error)?;
        }
        if self.upstream_max_stream_duration_reached {
            struct_ser.serialize_field("upstreamMaxStreamDurationReached", &self.upstream_max_stream_duration_reached)?;
        }
        if self.response_from_cache_filter {
            struct_ser.serialize_field("responseFromCacheFilter", &self.response_from_cache_filter)?;
        }
        if self.no_filter_config_found {
            struct_ser.serialize_field("noFilterConfigFound", &self.no_filter_config_found)?;
        }
        if self.duration_timeout {
            struct_ser.serialize_field("durationTimeout", &self.duration_timeout)?;
        }
        if self.upstream_protocol_error {
            struct_ser.serialize_field("upstreamProtocolError", &self.upstream_protocol_error)?;
        }
        if self.no_cluster_found {
            struct_ser.serialize_field("noClusterFound", &self.no_cluster_found)?;
        }
        if self.overload_manager {
            struct_ser.serialize_field("overloadManager", &self.overload_manager)?;
        }
        if self.dns_resolution_failure {
            struct_ser.serialize_field("dnsResolutionFailure", &self.dns_resolution_failure)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseFlags {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "failed_local_healthcheck",
            "failedLocalHealthcheck",
            "no_healthy_upstream",
            "noHealthyUpstream",
            "upstream_request_timeout",
            "upstreamRequestTimeout",
            "local_reset",
            "localReset",
            "upstream_remote_reset",
            "upstreamRemoteReset",
            "upstream_connection_failure",
            "upstreamConnectionFailure",
            "upstream_connection_termination",
            "upstreamConnectionTermination",
            "upstream_overflow",
            "upstreamOverflow",
            "no_route_found",
            "noRouteFound",
            "delay_injected",
            "delayInjected",
            "fault_injected",
            "faultInjected",
            "rate_limited",
            "rateLimited",
            "unauthorized_details",
            "unauthorizedDetails",
            "rate_limit_service_error",
            "rateLimitServiceError",
            "downstream_connection_termination",
            "downstreamConnectionTermination",
            "upstream_retry_limit_exceeded",
            "upstreamRetryLimitExceeded",
            "stream_idle_timeout",
            "streamIdleTimeout",
            "invalid_envoy_request_headers",
            "invalidEnvoyRequestHeaders",
            "downstream_protocol_error",
            "downstreamProtocolError",
            "upstream_max_stream_duration_reached",
            "upstreamMaxStreamDurationReached",
            "response_from_cache_filter",
            "responseFromCacheFilter",
            "no_filter_config_found",
            "noFilterConfigFound",
            "duration_timeout",
            "durationTimeout",
            "upstream_protocol_error",
            "upstreamProtocolError",
            "no_cluster_found",
            "noClusterFound",
            "overload_manager",
            "overloadManager",
            "dns_resolution_failure",
            "dnsResolutionFailure",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailedLocalHealthcheck,
            NoHealthyUpstream,
            UpstreamRequestTimeout,
            LocalReset,
            UpstreamRemoteReset,
            UpstreamConnectionFailure,
            UpstreamConnectionTermination,
            UpstreamOverflow,
            NoRouteFound,
            DelayInjected,
            FaultInjected,
            RateLimited,
            UnauthorizedDetails,
            RateLimitServiceError,
            DownstreamConnectionTermination,
            UpstreamRetryLimitExceeded,
            StreamIdleTimeout,
            InvalidEnvoyRequestHeaders,
            DownstreamProtocolError,
            UpstreamMaxStreamDurationReached,
            ResponseFromCacheFilter,
            NoFilterConfigFound,
            DurationTimeout,
            UpstreamProtocolError,
            NoClusterFound,
            OverloadManager,
            DnsResolutionFailure,
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
                            "failedLocalHealthcheck" | "failed_local_healthcheck" => Ok(GeneratedField::FailedLocalHealthcheck),
                            "noHealthyUpstream" | "no_healthy_upstream" => Ok(GeneratedField::NoHealthyUpstream),
                            "upstreamRequestTimeout" | "upstream_request_timeout" => Ok(GeneratedField::UpstreamRequestTimeout),
                            "localReset" | "local_reset" => Ok(GeneratedField::LocalReset),
                            "upstreamRemoteReset" | "upstream_remote_reset" => Ok(GeneratedField::UpstreamRemoteReset),
                            "upstreamConnectionFailure" | "upstream_connection_failure" => Ok(GeneratedField::UpstreamConnectionFailure),
                            "upstreamConnectionTermination" | "upstream_connection_termination" => Ok(GeneratedField::UpstreamConnectionTermination),
                            "upstreamOverflow" | "upstream_overflow" => Ok(GeneratedField::UpstreamOverflow),
                            "noRouteFound" | "no_route_found" => Ok(GeneratedField::NoRouteFound),
                            "delayInjected" | "delay_injected" => Ok(GeneratedField::DelayInjected),
                            "faultInjected" | "fault_injected" => Ok(GeneratedField::FaultInjected),
                            "rateLimited" | "rate_limited" => Ok(GeneratedField::RateLimited),
                            "unauthorizedDetails" | "unauthorized_details" => Ok(GeneratedField::UnauthorizedDetails),
                            "rateLimitServiceError" | "rate_limit_service_error" => Ok(GeneratedField::RateLimitServiceError),
                            "downstreamConnectionTermination" | "downstream_connection_termination" => Ok(GeneratedField::DownstreamConnectionTermination),
                            "upstreamRetryLimitExceeded" | "upstream_retry_limit_exceeded" => Ok(GeneratedField::UpstreamRetryLimitExceeded),
                            "streamIdleTimeout" | "stream_idle_timeout" => Ok(GeneratedField::StreamIdleTimeout),
                            "invalidEnvoyRequestHeaders" | "invalid_envoy_request_headers" => Ok(GeneratedField::InvalidEnvoyRequestHeaders),
                            "downstreamProtocolError" | "downstream_protocol_error" => Ok(GeneratedField::DownstreamProtocolError),
                            "upstreamMaxStreamDurationReached" | "upstream_max_stream_duration_reached" => Ok(GeneratedField::UpstreamMaxStreamDurationReached),
                            "responseFromCacheFilter" | "response_from_cache_filter" => Ok(GeneratedField::ResponseFromCacheFilter),
                            "noFilterConfigFound" | "no_filter_config_found" => Ok(GeneratedField::NoFilterConfigFound),
                            "durationTimeout" | "duration_timeout" => Ok(GeneratedField::DurationTimeout),
                            "upstreamProtocolError" | "upstream_protocol_error" => Ok(GeneratedField::UpstreamProtocolError),
                            "noClusterFound" | "no_cluster_found" => Ok(GeneratedField::NoClusterFound),
                            "overloadManager" | "overload_manager" => Ok(GeneratedField::OverloadManager),
                            "dnsResolutionFailure" | "dns_resolution_failure" => Ok(GeneratedField::DnsResolutionFailure),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseFlags;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.ResponseFlags")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ResponseFlags, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut failed_local_healthcheck__ = None;
                let mut no_healthy_upstream__ = None;
                let mut upstream_request_timeout__ = None;
                let mut local_reset__ = None;
                let mut upstream_remote_reset__ = None;
                let mut upstream_connection_failure__ = None;
                let mut upstream_connection_termination__ = None;
                let mut upstream_overflow__ = None;
                let mut no_route_found__ = None;
                let mut delay_injected__ = None;
                let mut fault_injected__ = None;
                let mut rate_limited__ = None;
                let mut unauthorized_details__ = None;
                let mut rate_limit_service_error__ = None;
                let mut downstream_connection_termination__ = None;
                let mut upstream_retry_limit_exceeded__ = None;
                let mut stream_idle_timeout__ = None;
                let mut invalid_envoy_request_headers__ = None;
                let mut downstream_protocol_error__ = None;
                let mut upstream_max_stream_duration_reached__ = None;
                let mut response_from_cache_filter__ = None;
                let mut no_filter_config_found__ = None;
                let mut duration_timeout__ = None;
                let mut upstream_protocol_error__ = None;
                let mut no_cluster_found__ = None;
                let mut overload_manager__ = None;
                let mut dns_resolution_failure__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FailedLocalHealthcheck => {
                            if failed_local_healthcheck__.is_some() {
                                return Err(serde::de::Error::duplicate_field("failedLocalHealthcheck"));
                            }
                            failed_local_healthcheck__ = Some(map.next_value()?);
                        }
                        GeneratedField::NoHealthyUpstream => {
                            if no_healthy_upstream__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noHealthyUpstream"));
                            }
                            no_healthy_upstream__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamRequestTimeout => {
                            if upstream_request_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamRequestTimeout"));
                            }
                            upstream_request_timeout__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalReset => {
                            if local_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localReset"));
                            }
                            local_reset__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamRemoteReset => {
                            if upstream_remote_reset__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamRemoteReset"));
                            }
                            upstream_remote_reset__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamConnectionFailure => {
                            if upstream_connection_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamConnectionFailure"));
                            }
                            upstream_connection_failure__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamConnectionTermination => {
                            if upstream_connection_termination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamConnectionTermination"));
                            }
                            upstream_connection_termination__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamOverflow => {
                            if upstream_overflow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamOverflow"));
                            }
                            upstream_overflow__ = Some(map.next_value()?);
                        }
                        GeneratedField::NoRouteFound => {
                            if no_route_found__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noRouteFound"));
                            }
                            no_route_found__ = Some(map.next_value()?);
                        }
                        GeneratedField::DelayInjected => {
                            if delay_injected__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayInjected"));
                            }
                            delay_injected__ = Some(map.next_value()?);
                        }
                        GeneratedField::FaultInjected => {
                            if fault_injected__.is_some() {
                                return Err(serde::de::Error::duplicate_field("faultInjected"));
                            }
                            fault_injected__ = Some(map.next_value()?);
                        }
                        GeneratedField::RateLimited => {
                            if rate_limited__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimited"));
                            }
                            rate_limited__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnauthorizedDetails => {
                            if unauthorized_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unauthorizedDetails"));
                            }
                            unauthorized_details__ = map.next_value()?;
                        }
                        GeneratedField::RateLimitServiceError => {
                            if rate_limit_service_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimitServiceError"));
                            }
                            rate_limit_service_error__ = Some(map.next_value()?);
                        }
                        GeneratedField::DownstreamConnectionTermination => {
                            if downstream_connection_termination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamConnectionTermination"));
                            }
                            downstream_connection_termination__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamRetryLimitExceeded => {
                            if upstream_retry_limit_exceeded__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamRetryLimitExceeded"));
                            }
                            upstream_retry_limit_exceeded__ = Some(map.next_value()?);
                        }
                        GeneratedField::StreamIdleTimeout => {
                            if stream_idle_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamIdleTimeout"));
                            }
                            stream_idle_timeout__ = Some(map.next_value()?);
                        }
                        GeneratedField::InvalidEnvoyRequestHeaders => {
                            if invalid_envoy_request_headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("invalidEnvoyRequestHeaders"));
                            }
                            invalid_envoy_request_headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::DownstreamProtocolError => {
                            if downstream_protocol_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("downstreamProtocolError"));
                            }
                            downstream_protocol_error__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamMaxStreamDurationReached => {
                            if upstream_max_stream_duration_reached__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamMaxStreamDurationReached"));
                            }
                            upstream_max_stream_duration_reached__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResponseFromCacheFilter => {
                            if response_from_cache_filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responseFromCacheFilter"));
                            }
                            response_from_cache_filter__ = Some(map.next_value()?);
                        }
                        GeneratedField::NoFilterConfigFound => {
                            if no_filter_config_found__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noFilterConfigFound"));
                            }
                            no_filter_config_found__ = Some(map.next_value()?);
                        }
                        GeneratedField::DurationTimeout => {
                            if duration_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("durationTimeout"));
                            }
                            duration_timeout__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpstreamProtocolError => {
                            if upstream_protocol_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("upstreamProtocolError"));
                            }
                            upstream_protocol_error__ = Some(map.next_value()?);
                        }
                        GeneratedField::NoClusterFound => {
                            if no_cluster_found__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noClusterFound"));
                            }
                            no_cluster_found__ = Some(map.next_value()?);
                        }
                        GeneratedField::OverloadManager => {
                            if overload_manager__.is_some() {
                                return Err(serde::de::Error::duplicate_field("overloadManager"));
                            }
                            overload_manager__ = Some(map.next_value()?);
                        }
                        GeneratedField::DnsResolutionFailure => {
                            if dns_resolution_failure__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dnsResolutionFailure"));
                            }
                            dns_resolution_failure__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ResponseFlags {
                    failed_local_healthcheck: failed_local_healthcheck__.unwrap_or_default(),
                    no_healthy_upstream: no_healthy_upstream__.unwrap_or_default(),
                    upstream_request_timeout: upstream_request_timeout__.unwrap_or_default(),
                    local_reset: local_reset__.unwrap_or_default(),
                    upstream_remote_reset: upstream_remote_reset__.unwrap_or_default(),
                    upstream_connection_failure: upstream_connection_failure__.unwrap_or_default(),
                    upstream_connection_termination: upstream_connection_termination__.unwrap_or_default(),
                    upstream_overflow: upstream_overflow__.unwrap_or_default(),
                    no_route_found: no_route_found__.unwrap_or_default(),
                    delay_injected: delay_injected__.unwrap_or_default(),
                    fault_injected: fault_injected__.unwrap_or_default(),
                    rate_limited: rate_limited__.unwrap_or_default(),
                    unauthorized_details: unauthorized_details__,
                    rate_limit_service_error: rate_limit_service_error__.unwrap_or_default(),
                    downstream_connection_termination: downstream_connection_termination__.unwrap_or_default(),
                    upstream_retry_limit_exceeded: upstream_retry_limit_exceeded__.unwrap_or_default(),
                    stream_idle_timeout: stream_idle_timeout__.unwrap_or_default(),
                    invalid_envoy_request_headers: invalid_envoy_request_headers__.unwrap_or_default(),
                    downstream_protocol_error: downstream_protocol_error__.unwrap_or_default(),
                    upstream_max_stream_duration_reached: upstream_max_stream_duration_reached__.unwrap_or_default(),
                    response_from_cache_filter: response_from_cache_filter__.unwrap_or_default(),
                    no_filter_config_found: no_filter_config_found__.unwrap_or_default(),
                    duration_timeout: duration_timeout__.unwrap_or_default(),
                    upstream_protocol_error: upstream_protocol_error__.unwrap_or_default(),
                    no_cluster_found: no_cluster_found__.unwrap_or_default(),
                    overload_manager: overload_manager__.unwrap_or_default(),
                    dns_resolution_failure: dns_resolution_failure__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.ResponseFlags", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for response_flags::Unauthorized {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reason != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.ResponseFlags.Unauthorized", len)?;
        if self.reason != 0 {
            let v = response_flags::unauthorized::Reason::from_i32(self.reason)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.reason)))?;
            struct_ser.serialize_field("reason", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for response_flags::Unauthorized {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reason,
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
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = response_flags::Unauthorized;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.ResponseFlags.Unauthorized")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<response_flags::Unauthorized, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value::<response_flags::unauthorized::Reason>()? as i32);
                        }
                    }
                }
                Ok(response_flags::Unauthorized {
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.ResponseFlags.Unauthorized", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for response_flags::unauthorized::Reason {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REASON_UNSPECIFIED",
            Self::ExternalService => "EXTERNAL_SERVICE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for response_flags::unauthorized::Reason {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REASON_UNSPECIFIED",
            "EXTERNAL_SERVICE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = response_flags::unauthorized::Reason;

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
                    .and_then(response_flags::unauthorized::Reason::from_i32)
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
                    .and_then(response_flags::unauthorized::Reason::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REASON_UNSPECIFIED" => Ok(response_flags::unauthorized::Reason::Unspecified),
                    "EXTERNAL_SERVICE" => Ok(response_flags::unauthorized::Reason::ExternalService),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TcpAccessLogEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.common_properties.is_some() {
            len += 1;
        }
        if self.connection_properties.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.TCPAccessLogEntry", len)?;
        if let Some(v) = self.common_properties.as_ref() {
            struct_ser.serialize_field("commonProperties", v)?;
        }
        if let Some(v) = self.connection_properties.as_ref() {
            struct_ser.serialize_field("connectionProperties", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TcpAccessLogEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "common_properties",
            "commonProperties",
            "connection_properties",
            "connectionProperties",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommonProperties,
            ConnectionProperties,
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
                            "commonProperties" | "common_properties" => Ok(GeneratedField::CommonProperties),
                            "connectionProperties" | "connection_properties" => Ok(GeneratedField::ConnectionProperties),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TcpAccessLogEntry;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.TCPAccessLogEntry")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TcpAccessLogEntry, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut common_properties__ = None;
                let mut connection_properties__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommonProperties => {
                            if common_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commonProperties"));
                            }
                            common_properties__ = map.next_value()?;
                        }
                        GeneratedField::ConnectionProperties => {
                            if connection_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionProperties"));
                            }
                            connection_properties__ = map.next_value()?;
                        }
                    }
                }
                Ok(TcpAccessLogEntry {
                    common_properties: common_properties__,
                    connection_properties: connection_properties__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.TCPAccessLogEntry", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TlsProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tls_version != 0 {
            len += 1;
        }
        if self.tls_cipher_suite.is_some() {
            len += 1;
        }
        if !self.tls_sni_hostname.is_empty() {
            len += 1;
        }
        if self.local_certificate_properties.is_some() {
            len += 1;
        }
        if self.peer_certificate_properties.is_some() {
            len += 1;
        }
        if !self.tls_session_id.is_empty() {
            len += 1;
        }
        if !self.ja3_fingerprint.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.TLSProperties", len)?;
        if self.tls_version != 0 {
            let v = tls_properties::TlsVersion::from_i32(self.tls_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.tls_version)))?;
            struct_ser.serialize_field("tlsVersion", &v)?;
        }
        if let Some(v) = self.tls_cipher_suite.as_ref() {
            struct_ser.serialize_field("tlsCipherSuite", v)?;
        }
        if !self.tls_sni_hostname.is_empty() {
            struct_ser.serialize_field("tlsSniHostname", &self.tls_sni_hostname)?;
        }
        if let Some(v) = self.local_certificate_properties.as_ref() {
            struct_ser.serialize_field("localCertificateProperties", v)?;
        }
        if let Some(v) = self.peer_certificate_properties.as_ref() {
            struct_ser.serialize_field("peerCertificateProperties", v)?;
        }
        if !self.tls_session_id.is_empty() {
            struct_ser.serialize_field("tlsSessionId", &self.tls_session_id)?;
        }
        if !self.ja3_fingerprint.is_empty() {
            struct_ser.serialize_field("ja3Fingerprint", &self.ja3_fingerprint)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TlsProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tls_version",
            "tlsVersion",
            "tls_cipher_suite",
            "tlsCipherSuite",
            "tls_sni_hostname",
            "tlsSniHostname",
            "local_certificate_properties",
            "localCertificateProperties",
            "peer_certificate_properties",
            "peerCertificateProperties",
            "tls_session_id",
            "tlsSessionId",
            "ja3_fingerprint",
            "ja3Fingerprint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TlsVersion,
            TlsCipherSuite,
            TlsSniHostname,
            LocalCertificateProperties,
            PeerCertificateProperties,
            TlsSessionId,
            Ja3Fingerprint,
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
                            "tlsVersion" | "tls_version" => Ok(GeneratedField::TlsVersion),
                            "tlsCipherSuite" | "tls_cipher_suite" => Ok(GeneratedField::TlsCipherSuite),
                            "tlsSniHostname" | "tls_sni_hostname" => Ok(GeneratedField::TlsSniHostname),
                            "localCertificateProperties" | "local_certificate_properties" => Ok(GeneratedField::LocalCertificateProperties),
                            "peerCertificateProperties" | "peer_certificate_properties" => Ok(GeneratedField::PeerCertificateProperties),
                            "tlsSessionId" | "tls_session_id" => Ok(GeneratedField::TlsSessionId),
                            "ja3Fingerprint" | "ja3_fingerprint" => Ok(GeneratedField::Ja3Fingerprint),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TlsProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.TLSProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TlsProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tls_version__ = None;
                let mut tls_cipher_suite__ = None;
                let mut tls_sni_hostname__ = None;
                let mut local_certificate_properties__ = None;
                let mut peer_certificate_properties__ = None;
                let mut tls_session_id__ = None;
                let mut ja3_fingerprint__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TlsVersion => {
                            if tls_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsVersion"));
                            }
                            tls_version__ = Some(map.next_value::<tls_properties::TlsVersion>()? as i32);
                        }
                        GeneratedField::TlsCipherSuite => {
                            if tls_cipher_suite__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsCipherSuite"));
                            }
                            tls_cipher_suite__ = map.next_value()?;
                        }
                        GeneratedField::TlsSniHostname => {
                            if tls_sni_hostname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsSniHostname"));
                            }
                            tls_sni_hostname__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalCertificateProperties => {
                            if local_certificate_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localCertificateProperties"));
                            }
                            local_certificate_properties__ = map.next_value()?;
                        }
                        GeneratedField::PeerCertificateProperties => {
                            if peer_certificate_properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerCertificateProperties"));
                            }
                            peer_certificate_properties__ = map.next_value()?;
                        }
                        GeneratedField::TlsSessionId => {
                            if tls_session_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tlsSessionId"));
                            }
                            tls_session_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ja3Fingerprint => {
                            if ja3_fingerprint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ja3Fingerprint"));
                            }
                            ja3_fingerprint__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TlsProperties {
                    tls_version: tls_version__.unwrap_or_default(),
                    tls_cipher_suite: tls_cipher_suite__,
                    tls_sni_hostname: tls_sni_hostname__.unwrap_or_default(),
                    local_certificate_properties: local_certificate_properties__,
                    peer_certificate_properties: peer_certificate_properties__,
                    tls_session_id: tls_session_id__.unwrap_or_default(),
                    ja3_fingerprint: ja3_fingerprint__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.TLSProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tls_properties::CertificateProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.subject_alt_name.is_empty() {
            len += 1;
        }
        if !self.subject.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.TLSProperties.CertificateProperties", len)?;
        if !self.subject_alt_name.is_empty() {
            struct_ser.serialize_field("subjectAltName", &self.subject_alt_name)?;
        }
        if !self.subject.is_empty() {
            struct_ser.serialize_field("subject", &self.subject)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tls_properties::CertificateProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject_alt_name",
            "subjectAltName",
            "subject",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SubjectAltName,
            Subject,
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
                            "subjectAltName" | "subject_alt_name" => Ok(GeneratedField::SubjectAltName),
                            "subject" => Ok(GeneratedField::Subject),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tls_properties::CertificateProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.TLSProperties.CertificateProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tls_properties::CertificateProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject_alt_name__ = None;
                let mut subject__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SubjectAltName => {
                            if subject_alt_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectAltName"));
                            }
                            subject_alt_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(tls_properties::CertificateProperties {
                    subject_alt_name: subject_alt_name__.unwrap_or_default(),
                    subject: subject__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.TLSProperties.CertificateProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tls_properties::certificate_properties::SubjectAltName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.san.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.data.accesslog.v3.TLSProperties.CertificateProperties.SubjectAltName", len)?;
        if let Some(v) = self.san.as_ref() {
            match v {
                tls_properties::certificate_properties::subject_alt_name::San::Uri(v) => {
                    struct_ser.serialize_field("uri", v)?;
                }
                tls_properties::certificate_properties::subject_alt_name::San::Dns(v) => {
                    struct_ser.serialize_field("dns", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for tls_properties::certificate_properties::SubjectAltName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "uri",
            "dns",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Uri,
            Dns,
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
                            "uri" => Ok(GeneratedField::Uri),
                            "dns" => Ok(GeneratedField::Dns),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tls_properties::certificate_properties::SubjectAltName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.data.accesslog.v3.TLSProperties.CertificateProperties.SubjectAltName")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<tls_properties::certificate_properties::SubjectAltName, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut san__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Uri => {
                            if san__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            san__ = map.next_value::<::std::option::Option<_>>()?.map(tls_properties::certificate_properties::subject_alt_name::San::Uri);
                        }
                        GeneratedField::Dns => {
                            if san__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dns"));
                            }
                            san__ = map.next_value::<::std::option::Option<_>>()?.map(tls_properties::certificate_properties::subject_alt_name::San::Dns);
                        }
                    }
                }
                Ok(tls_properties::certificate_properties::SubjectAltName {
                    san: san__,
                })
            }
        }
        deserializer.deserialize_struct("envoy.data.accesslog.v3.TLSProperties.CertificateProperties.SubjectAltName", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for tls_properties::TlsVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::VersionUnspecified => "VERSION_UNSPECIFIED",
            Self::TlSv1 => "TLSv1",
            Self::TlSv11 => "TLSv1_1",
            Self::TlSv12 => "TLSv1_2",
            Self::TlSv13 => "TLSv1_3",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for tls_properties::TlsVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VERSION_UNSPECIFIED",
            "TLSv1",
            "TLSv1_1",
            "TLSv1_2",
            "TLSv1_3",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = tls_properties::TlsVersion;

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
                    .and_then(tls_properties::TlsVersion::from_i32)
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
                    .and_then(tls_properties::TlsVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "VERSION_UNSPECIFIED" => Ok(tls_properties::TlsVersion::VersionUnspecified),
                    "TLSv1" => Ok(tls_properties::TlsVersion::TlSv1),
                    "TLSv1_1" => Ok(tls_properties::TlsVersion::TlSv11),
                    "TLSv1_2" => Ok(tls_properties::TlsVersion::TlSv12),
                    "TLSv1_3" => Ok(tls_properties::TlsVersion::TlSv13),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
