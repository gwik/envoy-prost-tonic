// @generated
/// gRPC HTTP/1.1 Bridge filter config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// If true then requests with content type set to ``application/x-protobuf`` will be automatically converted to gRPC.
    /// This works by prepending the payload data with the gRPC header frame, as defined by the wiring format, and
    /// Content-Type will be updated accordingly before sending the request.
    /// For the requests that went through this upgrade the filter will also strip the frame before forwarding the
    /// response to the client.
    #[prost(bool, tag = "1")]
    pub upgrade_protobuf_to_grpc: bool,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.http.grpc_http1_bridge.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb1, 0x0b, 0x0a, 0x3f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74,
    0x74, 0x70, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x31, 0x5f, 0x62, 0x72,
    0x69, 0x64, 0x67, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x32, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68,
    0x74, 0x74, 0x70, 0x2e, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x31, 0x5f, 0x62,
    0x72, 0x69, 0x64, 0x67, 0x65, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7e, 0x0a, 0x06, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x12, 0x37, 0x0a, 0x18, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x5f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x5f, 0x74, 0x6f, 0x5f, 0x67, 0x72, 0x70, 0x63,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x15, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x54, 0x6f, 0x47, 0x72, 0x70, 0x63, 0x3a, 0x3b, 0x9a,
    0xc5, 0x88, 0x1e, 0x36, 0x0a, 0x34, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x67,
    0x72, 0x70, 0x63, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x31, 0x5f, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65,
    0x2e, 0x76, 0x32, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0xc8, 0x01, 0xba, 0x80, 0xc8,
    0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x40, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x31, 0x5f, 0x62, 0x72,
    0x69, 0x64, 0x67, 0x65, 0x2e, 0x76, 0x33, 0x42, 0x0b, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x6d, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f,
    0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x67, 0x72, 0x70,
    0x63, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x31, 0x5f, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x2f, 0x76,
    0x33, 0x3b, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x31, 0x5f, 0x62, 0x72, 0x69,
    0x64, 0x67, 0x65, 0x76, 0x33, 0x4a, 0xa4, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1c, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x2b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x07, 0x00, 0x59, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07, 0x00, 0x59, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12,
    0x03, 0x08, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04,
    0x0a, 0x00, 0x84, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0a, 0x00, 0x84, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87,
    0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0xef, 0x01, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x12, 0x00, 0x1c, 0x01, 0x1a, 0x25, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x48, 0x54,
    0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x20, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x20, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x0a, 0x32, 0xbb, 0x01,
    0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c,
    0x65, 0x3a, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31,
    0x20, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5d, 0x0a, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x48,
    0x54, 0x54, 0x50, 0x2f, 0x31, 0x2e, 0x31, 0x20, 0x42, 0x72, 0x69, 0x64, 0x67, 0x65, 0x20, 0x46,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69,
    0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x62, 0x72, 0x69,
    0x64, 0x67, 0x65, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x68, 0x74, 0x74,
    0x70, 0x31, 0x5f, 0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04,
    0x13, 0x02, 0x14, 0x3d, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01,
    0x12, 0x04, 0x13, 0x02, 0x14, 0x3d, 0x0a, 0xb9, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x1b, 0x02, 0x24, 0x1a, 0xab, 0x03, 0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x60, 0x60, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2f, 0x78, 0x2d, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x60,
    0x60, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x75, 0x74, 0x6f, 0x6d, 0x61,
    0x74, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x52, 0x50, 0x43, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x62, 0x79, 0x20, 0x70, 0x72, 0x65, 0x70, 0x65, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67,
    0x52, 0x50, 0x43, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x2c, 0x20, 0x61, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x77, 0x69, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2d,
    0x54, 0x79, 0x70, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x6c, 0x79,
    0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x20, 0x46, 0x6f,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x77, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x6c,
    0x73, 0x6f, 0x20, 0x73, 0x74, 0x72, 0x69, 0x70, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x72, 0x61,
    0x6d, 0x65, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x02, 0x06,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x07, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x22, 0x23, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.http.grpc_http1_bridge.v3.serde.rs");
// @@protoc_insertion_point(module)