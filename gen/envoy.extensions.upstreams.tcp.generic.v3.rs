// @generated
/// A connection pool which forwards downstream TCP as TCP or HTTP to upstream,
/// based on CONNECT configuration.
/// [#extension: envoy.upstreams.tcp.generic]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericConnectionPoolProto {}
/// Encoded file descriptor set for the `envoy.extensions.upstreams.tcp.generic.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfb, 0x05, 0x0a, 0x47, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2f,
    0x74, 0x63, 0x70, 0x2f, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x2f, 0x76, 0x33, 0x2f, 0x67,
    0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x70, 0x6f, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x29, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x75,
    0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74, 0x63, 0x70, 0x2e, 0x67, 0x65, 0x6e,
    0x65, 0x72, 0x69, 0x63, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e,
    0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1c, 0x0a, 0x1a, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69,
    0x63, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x6f, 0x6f, 0x6c, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x42, 0xc5, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a,
    0x37, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74, 0x63, 0x70, 0x2e, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x69, 0x63, 0x2e, 0x76, 0x33, 0x42, 0x24, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69,
    0x63, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x6f, 0x6f, 0x6c, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x4f, 0x75, 0x74, 0x65, 0x72, 0x43, 0x6c, 0x61, 0x73, 0x73, 0x50, 0x01,
    0x5a, 0x5a, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x73, 0x2f, 0x74, 0x63, 0x70, 0x2f, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x2f,
    0x76, 0x33, 0x3b, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x76, 0x33, 0x4a, 0xf7, 0x02, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x32, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x50,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x06, 0x00, 0x50, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x07, 0x00, 0x45, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x07, 0x00, 0x45,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a,
    0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x71, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x09, 0x00, 0x71, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0a, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03,
    0x0a, 0x00, 0x46, 0x0a, 0xd4, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x11, 0x00, 0x12, 0x01,
    0x1a, 0x99, 0x01, 0x20, 0x41, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x66, 0x6f, 0x72, 0x77,
    0x61, 0x72, 0x64, 0x73, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20,
    0x54, 0x43, 0x50, 0x20, 0x61, 0x73, 0x20, 0x54, 0x43, 0x50, 0x20, 0x6f, 0x72, 0x20, 0x48, 0x54,
    0x54, 0x50, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2c, 0x0a,
    0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43,
    0x54, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74,
    0x63, 0x70, 0x2e, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x5d, 0x0a, 0x32, 0x2c, 0x20, 0x5b,
    0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a,
    0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x50, 0x6f, 0x6f, 0x6c, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x11, 0x08, 0x22, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.upstreams.tcp.generic.v3.serde.rs");
// @@protoc_insertion_point(module)