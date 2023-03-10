// @generated
/// CDN-Loop Header filter config. See the :ref:`configuration overview
/// <config_http_filters_cdn_loop>` for more information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdnLoopConfig {
    /// The CDN identifier to use for loop checks and to append to the
    /// CDN-Loop header.
    ///
    /// RFC 8586 calls this the cdn-id. The cdn-id can either be a
    /// pseudonym or hostname the CDN is in control of.
    ///
    /// cdn_id must not be empty.
    #[prost(string, tag = "1")]
    pub cdn_id: ::prost::alloc::string::String,
    /// The maximum allowed count of cdn_id in the downstream CDN-Loop
    /// request header.
    ///
    /// The default of 0 means a request can transit the CdnLoopFilter
    /// once. A value of 1 means that a request can transit the
    /// CdnLoopFilter twice and so on.
    #[prost(uint32, tag = "2")]
    pub max_allowed_occurrences: u32,
}
/// Encoded file descriptor set for the `envoy.extensions.filters.http.cdn_loop.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xaa, 0x0b, 0x0a, 0x38, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74,
    0x74, 0x70, 0x2f, 0x63, 0x64, 0x6e, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x2f, 0x76, 0x33, 0x2f, 0x63,
    0x64, 0x6e, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x29, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x63, 0x64, 0x6e,
    0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x67, 0x0a, 0x0d, 0x43, 0x64, 0x6e, 0x4c, 0x6f, 0x6f, 0x70, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x12, 0x1e, 0x0a, 0x06, 0x63, 0x64, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x05, 0x63, 0x64, 0x6e, 0x49,
    0x64, 0x12, 0x36, 0x0a, 0x17, 0x6d, 0x61, 0x78, 0x5f, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64,
    0x5f, 0x6f, 0x63, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0d, 0x52, 0x15, 0x6d, 0x61, 0x78, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x4f, 0x63,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x73, 0x42, 0xae, 0x01, 0xba, 0x80, 0xc8, 0xd1,
    0x06, 0x02, 0x10, 0x02, 0x0a, 0x37, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x2e, 0x63, 0x64, 0x6e, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x2e, 0x76, 0x33, 0x42, 0x0c, 0x43,
    0x64, 0x6e, 0x4c, 0x6f, 0x6f, 0x70, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5b, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d,
    0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68,
    0x74, 0x74, 0x70, 0x2f, 0x63, 0x64, 0x6e, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x2f, 0x76, 0x33, 0x3b,
    0x63, 0x64, 0x6e, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x76, 0x33, 0x4a, 0xe8, 0x07, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x23, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x32, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x04, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x21, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x50, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x07, 0x00, 0x50, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x2d, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x08, 0x00, 0x2d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x72, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03,
    0x0a, 0x00, 0x72, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0xde, 0x01, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x12, 0x00, 0x23, 0x01, 0x1a, 0x7c, 0x20, 0x43, 0x44, 0x4e, 0x2d,
    0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x0a,
    0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x63, 0x64, 0x6e, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x3e, 0x60,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x32, 0x54, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x48, 0x54, 0x54, 0x50,
    0x20, 0x43, 0x44, 0x4e, 0x2d, 0x4c, 0x6f, 0x6f, 0x70, 0x20, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74,
    0x74, 0x70, 0x2e, 0x63, 0x64, 0x6e, 0x5f, 0x6c, 0x6f, 0x6f, 0x70, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x15, 0x0a, 0xea, 0x01, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x1a, 0x02, 0x3d, 0x1a, 0xdc, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x43,
    0x44, 0x4e, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x74, 0x6f,
    0x20, 0x75, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6c, 0x6f, 0x6f, 0x70, 0x20, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x70, 0x70, 0x65,
    0x6e, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x43, 0x44, 0x4e, 0x2d, 0x4c,
    0x6f, 0x6f, 0x70, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x20, 0x52, 0x46,
    0x43, 0x20, 0x38, 0x35, 0x38, 0x36, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x64, 0x6e, 0x2d, 0x69, 0x64, 0x2e, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x63, 0x64, 0x6e, 0x2d, 0x69, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x65, 0x69, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x62, 0x65, 0x20, 0x61, 0x0a, 0x20, 0x70, 0x73, 0x65, 0x75, 0x64, 0x6f,
    0x6e, 0x79, 0x6d, 0x20, 0x6f, 0x72, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x43, 0x44, 0x4e, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x6f, 0x66, 0x2e, 0x0a, 0x0a, 0x20, 0x63, 0x64, 0x6e, 0x5f,
    0x69, 0x64, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x65,
    0x6d, 0x70, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x1a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a,
    0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1a, 0x12, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x1a, 0x14, 0x3c, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x1a, 0x15, 0x3b, 0x0a,
    0xf9, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x22, 0x02, 0x25, 0x1a, 0xeb, 0x01,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x61, 0x6c, 0x6c,
    0x6f, 0x77, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x64,
    0x6e, 0x5f, 0x69, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x43, 0x44, 0x4e, 0x2d, 0x4c, 0x6f, 0x6f, 0x70, 0x0a,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e,
    0x0a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x30, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x43, 0x64, 0x6e, 0x4c, 0x6f, 0x6f, 0x70, 0x46, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x0a, 0x20, 0x6f, 0x6e, 0x63, 0x65, 0x2e, 0x20, 0x41, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x31, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x43, 0x64, 0x6e, 0x4c,
    0x6f, 0x6f, 0x70, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x74, 0x77, 0x69, 0x63, 0x65, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x73, 0x6f, 0x20, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x22, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x22, 0x09, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x22, 0x23, 0x24, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.http.cdn_loop.v3.serde.rs");
// @@protoc_insertion_point(module)