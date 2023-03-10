// @generated
/// Describes a type of API listener, which is used in non-proxy clients. The type of API
/// exposed to the non-proxy application depends on the type of API listener.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiListener {
    /// The type in this field determines the type of API listener. At present, the following
    /// types are supported:
    /// envoy.config.filter.network.http_connection_manager.v2.HttpConnectionManager (HTTP)
    /// [#next-major-version: In the v3 API, replace this Any field with a oneof containing the
    /// specific config message for each type of API listener. We could not do this in v2 because
    /// it would have caused circular dependencies for go protos: lds.proto depends on this file,
    /// and http_connection_manager.proto depends on rds.proto, which is in the same directory as
    /// lds.proto, so lds.proto cannot depend on this file.]
    #[prost(message, optional, tag = "1")]
    pub api_listener: ::core::option::Option<::pbjson_types::Any>,
}
/// Encoded file descriptor set for the `envoy.config.listener.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe8, 0x0b, 0x0a, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2f, 0x76, 0x32, 0x2f, 0x61, 0x70,
    0x69, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x18, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x6c,
    0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x32, 0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x46, 0x0a, 0x0b, 0x41, 0x70, 0x69, 0x4c, 0x69, 0x73, 0x74, 0x65,
    0x6e, 0x65, 0x72, 0x12, 0x37, 0x0a, 0x0c, 0x61, 0x70, 0x69, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65,
    0x6e, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x52,
    0x0b, 0x61, 0x70, 0x69, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x42, 0xb0, 0x01, 0xf2,
    0x98, 0xfe, 0x8f, 0x05, 0x1a, 0x12, 0x18, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x33, 0xba,
    0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x26, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x76, 0x32, 0x42,
    0x10, 0x41, 0x70, 0x69, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0x5a, 0x4a, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65,
    0x72, 0x2f, 0x76, 0x32, 0x3b, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x76, 0x32, 0x4a,
    0xc1, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x21, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x09, 0x00, 0x3f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x31, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x31, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x61, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x0c, 0x00, 0x61, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x54, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x54, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02,
    0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0xd2, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14, 0x00,
    0x1e, 0x01, 0x1a, 0xa2, 0x01, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20,
    0x61, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x50, 0x49, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x70, 0x72, 0x6f,
    0x78, 0x79, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x50, 0x49, 0x0a, 0x20, 0x65, 0x78, 0x70,
    0x6f, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x6e, 0x2d,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x50, 0x49, 0x20, 0x6c, 0x69, 0x73,
    0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x0a, 0x32, 0x21, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x41, 0x50, 0x49, 0x20,
    0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x14, 0x08, 0x13, 0x0a, 0xf0, 0x04, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x1d, 0x02, 0x27, 0x1a, 0xe2, 0x04, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x64,
    0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x50, 0x49, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e,
    0x65, 0x72, 0x2e, 0x20, 0x41, 0x74, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74,
    0x65, 0x64, 0x3a, 0x0a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x2e, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x2e, 0x76, 0x32, 0x2e, 0x48, 0x74, 0x74, 0x70,
    0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65,
    0x72, 0x20, 0x28, 0x48, 0x54, 0x54, 0x50, 0x29, 0x0a, 0x20, 0x5b, 0x23, 0x6e, 0x65, 0x78, 0x74,
    0x2d, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x2d, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20,
    0x49, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x33, 0x20, 0x41, 0x50, 0x49, 0x2c, 0x20, 0x72,
    0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x41, 0x6e, 0x79, 0x20,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x6f, 0x6e, 0x65,
    0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65,
    0x61, 0x63, 0x68, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x50, 0x49, 0x20,
    0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x20, 0x57, 0x65, 0x20, 0x63, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x64, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69,
    0x6e, 0x20, 0x76, 0x32, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65, 0x0a, 0x20, 0x69, 0x74,
    0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x63, 0x61, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x63, 0x69, 0x72, 0x63, 0x75, 0x6c, 0x61, 0x72, 0x20, 0x64, 0x65, 0x70, 0x65,
    0x6e, 0x64, 0x65, 0x6e, 0x63, 0x69, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x67, 0x6f, 0x20,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x3a, 0x20, 0x6c, 0x64, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2c, 0x0a, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x68, 0x74, 0x74,
    0x70, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x61, 0x6e,
    0x61, 0x67, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e,
    0x64, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x72, 0x64, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2c,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20,
    0x61, 0x73, 0x0a, 0x20, 0x6c, 0x64, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2c, 0x20, 0x73,
    0x6f, 0x20, 0x6c, 0x64, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x63, 0x61, 0x6e, 0x6e,
    0x6f, 0x74, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x5d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x1d, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1d, 0x16, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1d, 0x25, 0x26, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.listener.v2.serde.rs");
// @@protoc_insertion_point(module)