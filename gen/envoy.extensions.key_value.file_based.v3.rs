// @generated
/// [#extension: envoy.key_value.file_based]
/// This is configuration to flush a key value store out to disk.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileBasedKeyValueStoreConfig {
    /// The filename to read the keys and values from, and write the keys and
    /// values to.
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    /// The interval at which the key value store should be flushed to the file.
    #[prost(message, optional, tag = "2")]
    pub flush_interval: ::core::option::Option<::pbjson_types::Duration>,
    /// The maximum number of entries to cache, or 0 to allow for unlimited entries.
    /// Defaults to 1000 if not present.
    #[prost(message, optional, tag = "3")]
    pub max_entries: ::core::option::Option<::pbjson_types::UInt32Value>,
}
/// Encoded file descriptor set for the `envoy.extensions.key_value.file_based.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfe, 0x0b, 0x0a, 0x35, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6b, 0x65, 0x79, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2f,
    0x66, 0x69, 0x6c, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x64, 0x2f, 0x76, 0x33, 0x2f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x28, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6b, 0x65, 0x79,
    0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65,
    0x64, 0x2e, 0x76, 0x33, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x78, 0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xce, 0x01,
    0x0a, 0x1c, 0x46, 0x69, 0x6c, 0x65, 0x42, 0x61, 0x73, 0x65, 0x64, 0x4b, 0x65, 0x79, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x23,
    0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x40, 0x0a, 0x0e, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x5f, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x76, 0x61, 0x6c, 0x12, 0x3d, 0x0a, 0x0b, 0x6d, 0x61, 0x78, 0x5f, 0x65, 0x6e, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x55, 0x49, 0x6e,
    0x74, 0x33, 0x32, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x0a, 0x6d, 0x61, 0x78, 0x45, 0x6e, 0x74,
    0x72, 0x69, 0x65, 0x73, 0x3a, 0x08, 0xd2, 0xc6, 0xa4, 0xe1, 0x06, 0x02, 0x08, 0x01, 0x42, 0xad,
    0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x36, 0x69, 0x6f, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6b, 0x65, 0x79, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x2e, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x64, 0x2e, 0x76,
    0x33, 0x42, 0x0b, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0x5a, 0x5c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6b, 0x65, 0x79, 0x5f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x2f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x64, 0x2f, 0x76,
    0x33, 0x3b, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x64, 0x76, 0x33, 0x4a, 0xf8,
    0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x23, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x31, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x05, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x29, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x0a, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x4f, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0c, 0x00, 0x4f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0d, 0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x2c, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03,
    0x0e, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x73, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0b, 0x12, 0x03, 0x0f, 0x00, 0x73, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x10,
    0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x10, 0x00,
    0x46, 0x0a, 0xb5, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x16, 0x00, 0x23, 0x01, 0x1a, 0x69,
    0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x6b, 0x65, 0x79, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x66, 0x69,
    0x6c, 0x65, 0x5f, 0x62, 0x61, 0x73, 0x65, 0x64, 0x5d, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20,
    0x69, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x20, 0x61, 0x20, 0x6b, 0x65, 0x79, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x6f, 0x75, 0x74, 0x20,
    0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x6b, 0x2e, 0x0a, 0x32, 0x3e, 0x20, 0x5b, 0x23, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4b, 0x65,
    0x79, 0x2f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2d, 0x62, 0x61, 0x73,
    0x65, 0x64, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65,
    0x20, 0x70, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x16, 0x08, 0x24, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x17, 0x02,
    0x45, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xea, 0xc8, 0x94, 0x6c, 0x01, 0x12, 0x03, 0x17,
    0x02, 0x45, 0x0a, 0x60, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x3f, 0x1a,
    0x53, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x73, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x2c,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b,
    0x65, 0x79, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20,
    0x74, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x09, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x14, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1b, 0x16, 0x3e, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x1b, 0x17, 0x3d, 0x0a, 0x57, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x2e, 0x1a, 0x4a, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x61, 0x74, 0x20, 0x77, 0x68, 0x69,
    0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65,
    0x20, 0x66, 0x6c, 0x75, 0x73, 0x68, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x1e, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e,
    0x1b, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e, 0x2c, 0x2d,
    0x0a, 0x7d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x22, 0x02, 0x2e, 0x1a, 0x70, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x6e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x30, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x75, 0x6e, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x65, 0x64, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x2e, 0x0a, 0x20, 0x44, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x31, 0x30, 0x30, 0x30, 0x20, 0x69,
    0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x22, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x1e, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x22, 0x2c, 0x2d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
include!("envoy.extensions.key_value.file_based.v3.serde.rs");
// @@protoc_insertion_point(module)