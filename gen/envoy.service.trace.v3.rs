// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTracesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTracesMessage {
    /// Identifier data effectively is a structured metadata.
    /// As a performance optimization this will only be sent in the first message
    /// on the stream.
    #[prost(message, optional, tag = "1")]
    pub identifier: ::core::option::Option<stream_traces_message::Identifier>,
    /// A list of Span entries
    #[prost(message, repeated, tag = "2")]
    pub spans: ::prost::alloc::vec::Vec<
        super::super::super::super::opencensus::proto::trace::v1::Span,
    >,
}
/// Nested message and enum types in `StreamTracesMessage`.
pub mod stream_traces_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identifier {
        /// The node sending the access log messages over the stream.
        #[prost(message, optional, tag = "1")]
        pub node: ::core::option::Option<
            super::super::super::super::config::core::v3::Node,
        >,
    }
}
/// Encoded file descriptor set for the `envoy.service.trace.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa1, 0x11, 0x0a, 0x2a, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2f, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x74, 0x72, 0x61, 0x63,
    0x65, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x16, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74,
    0x72, 0x61, 0x63, 0x65, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x62, 0x61,
    0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x25, 0x6f, 0x70, 0x65, 0x6e, 0x63, 0x65,
    0x6e, 0x73, 0x75, 0x73, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x74, 0x72, 0x61, 0x63, 0x65,
    0x2f, 0x76, 0x31, 0x2f, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4a, 0x0a, 0x14, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x3a, 0x32, 0x9a, 0xc5, 0x88, 0x1e, 0x2d, 0x0a, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2e, 0x76,
    0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0xde, 0x02, 0x0a, 0x13, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x56,
    0x0a, 0x0a, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x36, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2e, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e,
    0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x52, 0x0a, 0x69, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x35, 0x0a, 0x05, 0x73, 0x70, 0x61, 0x6e, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x6f, 0x70, 0x65, 0x6e, 0x63, 0x65, 0x6e, 0x73,
    0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2e, 0x76,
    0x31, 0x2e, 0x53, 0x70, 0x61, 0x6e, 0x52, 0x05, 0x73, 0x70, 0x61, 0x6e, 0x73, 0x1a, 0x84, 0x01,
    0x0a, 0x0a, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x38, 0x0a, 0x04,
    0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76,
    0x33, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01,
    0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x3a, 0x3c, 0x9a, 0xc5, 0x88, 0x1e, 0x37, 0x0a, 0x35, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x72, 0x61,
    0x63, 0x65, 0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x72, 0x61, 0x63,
    0x65, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69,
    0x66, 0x69, 0x65, 0x72, 0x3a, 0x31, 0x9a, 0xc5, 0x88, 0x1e, 0x2c, 0x0a, 0x2a, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x72, 0x61, 0x63, 0x65,
    0x2e, 0x76, 0x32, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x32, 0x7d, 0x0a, 0x0c, 0x54, 0x72, 0x61, 0x63, 0x65,
    0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x6d, 0x0a, 0x0c, 0x53, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73, 0x12, 0x2b, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2e, 0x76, 0x33,
    0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x1a, 0x2c, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x2e, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x53, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x54, 0x72, 0x61, 0x63, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x00, 0x28, 0x01, 0x42, 0x8a, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x02, 0x0a, 0x24, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x74,
    0x72, 0x61, 0x63, 0x65, 0x2e, 0x76, 0x33, 0x42, 0x11, 0x54, 0x72, 0x61, 0x63, 0x65, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x45, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70,
    0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69,
    0x63, 0x65, 0x2f, 0x74, 0x72, 0x61, 0x63, 0x65, 0x2f, 0x76, 0x33, 0x3b, 0x74, 0x72, 0x61, 0x63,
    0x65, 0x76, 0x33, 0x4a, 0xf6, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x36, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x1f, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x08, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x2b, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0a, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0c, 0x00, 0x3d, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0c, 0x00, 0x3d, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x32, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03,
    0x0d, 0x00, 0x32, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f,
    0x00, 0x5c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0f, 0x00, 0x5c, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a,
    0x02, 0x12, 0x03, 0x10, 0x00, 0x46, 0x0a, 0xc4, 0x01, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x16,
    0x00, 0x1c, 0x01, 0x1a, 0x93, 0x01, 0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x72, 0x61,
    0x63, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x75, 0x6d, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x72, 0x61, 0x63, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x20, 0x49, 0x74, 0x0a, 0x20,
    0x75, 0x73, 0x65, 0x73, 0x20, 0x4f, 0x70, 0x65, 0x6e, 0x43, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20,
    0x64, 0x61, 0x74, 0x61, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20,
    0x73, 0x74, 0x61, 0x6e, 0x64, 0x61, 0x72, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72,
    0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x72, 0x61, 0x63, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x32, 0x22, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x54, 0x72,
    0x61, 0x63, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x14, 0x0a, 0xb0, 0x01, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x00, 0x12, 0x04, 0x1a, 0x02, 0x1b, 0x03, 0x1a, 0xa1, 0x01, 0x20, 0x45, 0x6e, 0x76, 0x6f,
    0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x54, 0x72,
    0x61, 0x63, 0x65, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x65, 0x76, 0x65, 0x72, 0x2e, 0x20, 0x49, 0x74,
    0x20, 0x64, 0x6f, 0x65, 0x73, 0x0a, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63,
    0x74, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x73, 0x20, 0x6e, 0x6f, 0x74,
    0x68, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x6f,
    0x6e, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x73, 0x65, 0x0a, 0x20,
    0x6f, 0x66, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x1a, 0x1a, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1a, 0x38, 0x4c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1e, 0x00, 0x21, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x00, 0x07, 0x12, 0x04, 0x1f, 0x02, 0x20, 0x34, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07,
    0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x1f, 0x02, 0x20, 0x34, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x23, 0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x23, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12, 0x04, 0x24, 0x02, 0x25, 0x33,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x01, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x24, 0x02,
    0x25, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x27, 0x02, 0x2d, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x27, 0x0a, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x07, 0x12, 0x04, 0x28, 0x04, 0x29, 0x40, 0x0a, 0x12, 0x0a,
    0x0a, 0x04, 0x01, 0x03, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x28, 0x04, 0x29,
    0x40, 0x0a, 0x4a, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x04, 0x4f,
    0x1a, 0x3b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6c,
    0x6f, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x6f, 0x76, 0x65, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2c, 0x04, 0x17, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x18, 0x1c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1f, 0x20, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2c, 0x21, 0x4e, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x2c, 0x22, 0x4d,
    0x0a, 0xa0, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x1c, 0x1a, 0x92,
    0x01, 0x20, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x0a, 0x20, 0x41, 0x73, 0x20, 0x61, 0x20, 0x70, 0x65,
    0x72, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6d, 0x69,
    0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x0a, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x32, 0x02,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x1a, 0x1b, 0x0a, 0x25, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x35, 0x02, 0x34, 0x1a, 0x18, 0x20, 0x41, 0x20, 0x6c,
    0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x70, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x74, 0x72,
    0x69, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x35,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x35, 0x0b, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x35, 0x2a, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x35, 0x32, 0x33, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.service.trace.v3.serde.rs");
include!("envoy.service.trace.v3.tonic.rs");
// @@protoc_insertion_point(module)