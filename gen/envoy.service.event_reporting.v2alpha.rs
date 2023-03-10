// @generated
/// \[#not-implemented-hide:\]
/// An events envoy sends to the management server.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsRequest {
    /// Identifier data that will only be sent in the first message on the stream. This is effectively
    /// structured metadata and is a performance optimization.
    #[prost(message, optional, tag = "1")]
    pub identifier: ::core::option::Option<stream_events_request::Identifier>,
    /// Batch of events. When the stream is already active, it will be the events occurred
    /// since the last message had been sent. If the server receives unknown event type, it should
    /// silently ignore it.
    ///
    /// The following events are supported:
    ///
    /// * :ref:`HealthCheckEvent <envoy_api_msg_data.core.v2alpha.HealthCheckEvent>`
    /// * :ref:`OutlierDetectionEvent <envoy_api_msg_data.cluster.v2alpha.OutlierDetectionEvent>`
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<::pbjson_types::Any>,
}
/// Nested message and enum types in `StreamEventsRequest`.
pub mod stream_events_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identifier {
        /// The node sending the event messages over the stream.
        #[prost(message, optional, tag = "1")]
        pub node: ::core::option::Option<
            super::super::super::super::api::v2::core::Node,
        >,
    }
}
/// \[#not-implemented-hide:\]
/// The management server may send envoy a StreamEventsResponse to tell which events the server
/// is interested in. In future, with aggregated event reporting service, this message will
/// contain, for example, clusters the envoy should send events for, or event types the server
/// wants to process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsResponse {}
/// Encoded file descriptor set for the `envoy.service.event_reporting.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x86, 0x18, 0x0a, 0x43, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69,
    0x6e, 0x67, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x25, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a,
    0x1c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x32, 0x2f, 0x63, 0x6f,
    0x72, 0x65, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61,
    0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61,
    0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0xf9, 0x01, 0x0a, 0x13, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x65, 0x0a, 0x0a, 0x69, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x45, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x32, 0x61,
    0x6c, 0x70, 0x68, 0x61, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66,
    0x69, 0x65, 0x72, 0x52, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12,
    0x36, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x41, 0x6e, 0x79, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x92, 0x01, 0x02, 0x08, 0x01, 0x52,
    0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x1a, 0x43, 0x0a, 0x0a, 0x49, 0x64, 0x65, 0x6e, 0x74,
    0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x35, 0x0a, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e,
    0x76, 0x32, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x42, 0x08, 0xfa, 0x42,
    0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x22, 0x16, 0x0a, 0x14,
    0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x32, 0xa7, 0x01, 0x0a, 0x15, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x8d,
    0x01, 0x0a, 0x0c, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12,
    0x3a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x2e,
    0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x3b, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x32, 0x61, 0x6c,
    0x70, 0x68, 0x61, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x28, 0x01, 0x30, 0x01, 0x42, 0xd1,
    0x01, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x22, 0x12, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x01, 0x0a, 0x33, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x2e, 0x76,
    0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x1a, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2f, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x5f, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x4a, 0xec, 0x10, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3d, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x2e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x08,
    0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x27, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c,
    0x00, 0x4c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0c, 0x00, 0x4c, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0d,
    0x00, 0x3b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0a, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00,
    0x63, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0f, 0x00, 0x63, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x10, 0x00, 0x5c, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02,
    0x12, 0x03, 0x10, 0x00, 0x5c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x11, 0x00, 0x46, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x11, 0x00, 0x46, 0x0a, 0xf1,
    0x01, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x18, 0x00, 0x1e, 0x01, 0x1a, 0xb1, 0x01, 0x20, 0x5b,
    0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64,
    0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d, 0x0a, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x64,
    0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x45, 0x6e,
    0x76, 0x6f, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x20, 0x6f, 0x66,
    0x0a, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x6d, 0x61,
    0x79, 0x20, 0x62, 0x65, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63,
    0x6b, 0x20, 0x6f, 0x72, 0x20, 0x6f, 0x75, 0x74, 0x6c, 0x69, 0x65, 0x72, 0x20, 0x64, 0x65, 0x74,
    0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x32,
    0x31, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74,
    0x6c, 0x65, 0x3a, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x52,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x1d, 0x0a, 0x84,
    0x02, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x1c, 0x02, 0x1d, 0x03, 0x1a, 0xf5, 0x01,
    0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x65, 0x76, 0x65,
    0x72, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x73, 0x65,
    0x6e, 0x64, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x75, 0x72, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x2e, 0x20, 0x53, 0x65, 0x65, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x77, 0x2e, 0x0a, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x41, 0x50, 0x49, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x73, 0x69, 0x67,
    0x6e, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x68, 0x69, 0x67, 0x68, 0x20, 0x74, 0x68, 0x72,
    0x6f, 0x75, 0x67, 0x68, 0x70, 0x75, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x69, 0x74, 0x20, 0x6d, 0x69, 0x67, 0x68, 0x74, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x6f,
    0x73, 0x73, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1c, 0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x13,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x1a, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x38, 0x3e, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x3f, 0x53, 0x0a, 0x57, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x22, 0x00, 0x35, 0x01, 0x1a, 0x4b, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d,
    0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64, 0x65,
    0x3a, 0x5d, 0x0a, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x22, 0x08, 0x1b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x23, 0x02, 0x26, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x23, 0x0a, 0x14, 0x0a, 0x45, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x25, 0x04, 0x4c, 0x1a, 0x36, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x25, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x25, 0x15, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x25, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03,
    0x25, 0x1e, 0x4b, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08,
    0x11, 0x12, 0x03, 0x25, 0x1f, 0x4a, 0x0a, 0xa6, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x2a, 0x02, 0x1c, 0x1a, 0x98, 0x01, 0x20, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69,
    0x65, 0x72, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63,
    0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x0a, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72,
    0x65, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x69, 0x73, 0x20, 0x61, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65,
    0x20, 0x6f, 0x70, 0x74, 0x69, 0x6d, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2a, 0x02, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x1a, 0x1b, 0x0a, 0xa3, 0x03, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x34, 0x02, 0x57, 0x1a, 0x95, 0x03, 0x20, 0x42, 0x61, 0x74, 0x63, 0x68,
    0x20, 0x6f, 0x66, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x57, 0x68, 0x65, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x61,
    0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x2c, 0x20, 0x69,
    0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x73, 0x20, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x64, 0x0a, 0x20, 0x73,
    0x69, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x68, 0x61, 0x64, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x73,
    0x65, 0x6e, 0x74, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x73, 0x20, 0x75, 0x6e, 0x6b, 0x6e,
    0x6f, 0x77, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20,
    0x69, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x73, 0x69, 0x6c, 0x65, 0x6e,
    0x74, 0x6c, 0x79, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x74, 0x2e, 0x0a, 0x0a,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x65, 0x64, 0x3a, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x48,
    0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x20,
    0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x6d, 0x73, 0x67, 0x5f, 0x64,
    0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x3e, 0x60, 0x0a, 0x20, 0x2a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x4f, 0x75, 0x74,
    0x6c, 0x69, 0x65, 0x72, 0x44, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x6d, 0x73,
    0x67, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x76,
    0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x4f, 0x75, 0x74, 0x6c, 0x69, 0x65, 0x72, 0x44, 0x65,
    0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x3e, 0x60, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x34, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x34, 0x1f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x34, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x34, 0x2a, 0x56, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08,
    0x12, 0x12, 0x03, 0x34, 0x2b, 0x55, 0x0a, 0xcc, 0x02, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x3c,
    0x00, 0x3d, 0x01, 0x1a, 0xbf, 0x02, 0x20, 0x5b, 0x23, 0x6e, 0x6f, 0x74, 0x2d, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2d, 0x68, 0x69, 0x64, 0x65, 0x3a, 0x5d, 0x0a,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x61, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x65, 0x6c, 0x6c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0a, 0x20, 0x69, 0x73,
    0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x2e, 0x20,
    0x49, 0x6e, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x2c, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x64, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x2c, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x63, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x0a, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x63,
    0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x08,
    0x1c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.service.event_reporting.v2alpha.serde.rs");
include!("envoy.service.event_reporting.v2alpha.tonic.rs");
// @@protoc_insertion_point(module)