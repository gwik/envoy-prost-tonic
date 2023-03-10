// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpProtocolOptions {
    /// The idle timeout for the connection. The idle timeout is defined as the period in which
    /// the connection is not associated with a downstream connection. When the idle timeout is
    /// reached, the connection will be closed.
    ///
    /// If not set, the default idle timeout is 10 minutes. To disable idle timeouts, explicitly set this to 0.
    ///
    /// .. warning::
    ///    Disabling this timeout has a highly likelihood of yielding connection leaks due to lost TCP
    ///    FIN packets, etc.
    #[prost(message, optional, tag = "1")]
    pub idle_timeout: ::core::option::Option<::pbjson_types::Duration>,
}
/// Encoded file descriptor set for the `envoy.extensions.upstreams.tcp.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb8, 0x0e, 0x0a, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2f,
    0x74, 0x63, 0x70, 0x2f, 0x76, 0x33, 0x2f, 0x74, 0x63, 0x70, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x21, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74, 0x63,
    0x70, 0x2e, 0x76, 0x33, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x52, 0x0a, 0x12, 0x54, 0x63, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x3c, 0x0a, 0x0c, 0x69, 0x64, 0x6c,
    0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x69, 0x64, 0x6c, 0x65,
    0x54, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x42, 0xa4, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02,
    0x10, 0x02, 0x0a, 0x2f, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78,
    0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f,
    0x6e, 0x73, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74, 0x63, 0x70,
    0x2e, 0x76, 0x33, 0x42, 0x17, 0x54, 0x63, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4e,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x73, 0x2f, 0x74, 0x63, 0x70, 0x2f, 0x76, 0x33, 0x3b, 0x74, 0x63, 0x70, 0x76, 0x33, 0x4a, 0x92,
    0x0b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x2a, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x06, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x48, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x08, 0x00, 0x48, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09,
    0x00, 0x38, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x09, 0x00, 0x38, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0a,
    0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x65, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x03, 0x0b, 0x00, 0x65, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00,
    0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0c, 0x00, 0x46,
    0x0a, 0xe1, 0x05, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x21, 0x00, 0x2c, 0x01, 0x32, 0x61, 0x20,
    0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x3a, 0x20, 0x54, 0x43, 0x50, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x4f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x75, 0x70, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74, 0x63, 0x70, 0x2e, 0x74, 0x63, 0x70, 0x5f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x5d, 0x0a,
    0x32, 0xf1, 0x04, 0x20, 0x54, 0x43, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x4f, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20,
    0x54, 0x43, 0x50, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x74, 0x79,
    0x70, 0x65, 0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x76, 0x33, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x74, 0x79, 0x70, 0x65,
    0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3e, 0x60, 0x2c, 0x0a,
    0x20, 0x6b, 0x65, 0x79, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x20, 0x60, 0x60, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e,
    0x74, 0x63, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x54, 0x63, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x60, 0x60, 0x2e, 0x0a, 0x0a, 0x20, 0x2e,
    0x2e, 0x20, 0x63, 0x6f, 0x64, 0x65, 0x3a, 0x3a, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x63, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x73, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2d, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x3a, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x3a, 0x20, 0x35, 0x73, 0x0a, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x74, 0x79, 0x70, 0x65, 0x64, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x6f, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x75,
    0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74, 0x63, 0x70, 0x2e, 0x76, 0x33, 0x2e,
    0x54, 0x63, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x4f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22,
    0x40, 0x74, 0x79, 0x70, 0x65, 0x22, 0x3a, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x61, 0x70, 0x69, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x75, 0x70, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x73, 0x2e, 0x74, 0x63, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x54, 0x63,
    0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x69, 0x64, 0x6c, 0x65,
    0x5f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x3a, 0x20, 0x31, 0x30, 0x6d, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2e, 0x2e, 0x2e, 0x2e, 0x20, 0x5b, 0x66, 0x75, 0x72, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08, 0x1a,
    0x0a, 0xd6, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x2c, 0x1a, 0xc8,
    0x03, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x64, 0x6c, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x6f,
    0x75, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x64, 0x6c, 0x65, 0x20,
    0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x61, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x69, 0x64, 0x6c, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20,
    0x69, 0x73, 0x0a, 0x20, 0x72, 0x65, 0x61, 0x63, 0x68, 0x65, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x49, 0x66,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x64, 0x6c, 0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x6f,
    0x75, 0x74, 0x20, 0x69, 0x73, 0x20, 0x31, 0x30, 0x20, 0x6d, 0x69, 0x6e, 0x75, 0x74, 0x65, 0x73,
    0x2e, 0x20, 0x54, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x64, 0x6c,
    0x65, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x73, 0x2c, 0x20, 0x65, 0x78, 0x70, 0x6c,
    0x69, 0x63, 0x69, 0x74, 0x6c, 0x79, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x30, 0x2e, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x77, 0x61, 0x72, 0x6e, 0x69,
    0x6e, 0x67, 0x3a, 0x3a, 0x0a, 0x20, 0x20, 0x20, 0x44, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x20, 0x68,
    0x61, 0x73, 0x20, 0x61, 0x20, 0x68, 0x69, 0x67, 0x68, 0x6c, 0x79, 0x20, 0x6c, 0x69, 0x6b, 0x65,
    0x6c, 0x69, 0x68, 0x6f, 0x6f, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x79, 0x69, 0x65, 0x6c, 0x64, 0x69,
    0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6c, 0x65,
    0x61, 0x6b, 0x73, 0x20, 0x64, 0x75, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x6f, 0x73, 0x74, 0x20,
    0x54, 0x43, 0x50, 0x0a, 0x20, 0x20, 0x20, 0x46, 0x49, 0x4e, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x65,
    0x74, 0x73, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x2b, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2b, 0x1b, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2b, 0x2a, 0x2b, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.upstreams.tcp.v3.serde.rs");
// @@protoc_insertion_point(module)