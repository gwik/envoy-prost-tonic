// @generated
/// The fixed heap resource monitor reports the Envoy process memory pressure, computed as a
/// fraction of currently reserved heap memory divided by a statically configured maximum
/// specified in the FixedHeapConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedHeapConfig {
    #[prost(uint64, tag = "1")]
    pub max_heap_size_bytes: u64,
}
/// Encoded file descriptor set for the `envoy.config.resource_monitor.fixed_heap.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf1, 0x07, 0x0a, 0x41, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74,
    0x6f, 0x72, 0x2f, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x2f, 0x76, 0x32,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x30, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f,
    0x6e, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70,
    0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x49, 0x0a, 0x0f, 0x46, 0x69, 0x78, 0x65, 0x64, 0x48, 0x65, 0x61, 0x70, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x12, 0x36, 0x0a, 0x13, 0x6d, 0x61, 0x78, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x5f,
    0x73, 0x69, 0x7a, 0x65, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x42, 0x07, 0xfa, 0x42, 0x04, 0x32, 0x02, 0x20, 0x00, 0x52, 0x10, 0x6d, 0x61, 0x78, 0x48, 0x65,
    0x61, 0x70, 0x53, 0x69, 0x7a, 0x65, 0x42, 0x79, 0x74, 0x65, 0x73, 0x42, 0xb3, 0x01, 0xba, 0x80,
    0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x3e, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69,
    0x74, 0x6f, 0x72, 0x2e, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x2e, 0x76,
    0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x0e, 0x46, 0x69, 0x78, 0x65, 0x64, 0x48, 0x65, 0x61,
    0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x57, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f,
    0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65,
    0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x72, 0x65,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x2f, 0x66,
    0x69, 0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x4a, 0xb8, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x15, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x39,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x05, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x57,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07, 0x00, 0x57, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x08, 0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x08, 0x00, 0x2f,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a,
    0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x6e, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0a, 0x00, 0x6e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0b, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03,
    0x0b, 0x00, 0x46, 0x0a, 0xb4, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x13, 0x00, 0x15, 0x01,
    0x1a, 0xd4, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x78, 0x65, 0x64, 0x20, 0x68, 0x65,
    0x61, 0x70, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x6d, 0x6f, 0x6e, 0x69,
    0x74, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x20, 0x6d, 0x65,
    0x6d, 0x6f, 0x72, 0x79, 0x20, 0x70, 0x72, 0x65, 0x73, 0x73, 0x75, 0x72, 0x65, 0x2c, 0x20, 0x63,
    0x6f, 0x6d, 0x70, 0x75, 0x74, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x0a, 0x20, 0x66, 0x72,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x20, 0x68, 0x65, 0x61,
    0x70, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x64, 0x69, 0x76, 0x69, 0x64, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79,
    0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x61, 0x78, 0x69,
    0x6d, 0x75, 0x6d, 0x0a, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x46, 0x69, 0x78, 0x65, 0x64, 0x48, 0x65, 0x61, 0x70, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x0a, 0x32, 0x51, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x46, 0x69, 0x78, 0x65,
    0x64, 0x20, 0x68, 0x65, 0x61, 0x70, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x73, 0x2e, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x13, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x14, 0x02, 0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x09, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x1f, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x14, 0x21, 0x44, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x06, 0x12, 0x03, 0x14, 0x22, 0x43, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.resource_monitor.fixed_heap.v2alpha.serde.rs");
// @@protoc_insertion_point(module)