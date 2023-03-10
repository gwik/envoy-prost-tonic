// @generated
/// RBAC filter config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    /// Specify the RBAC rules to be applied globally.
    /// If absent, no enforcing RBAC policy will be applied.
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<super::super::super::super::rbac::v2::Rbac>,
    /// Shadow rules are not enforced by the filter (i.e., returning a 403)
    /// but will emit stats and logs and can be used for rule testing.
    /// If absent, no shadow RBAC policy will be applied.
    #[prost(message, optional, tag = "2")]
    pub shadow_rules: ::core::option::Option<super::super::super::super::rbac::v2::Rbac>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RbacPerRoute {
    /// Override the global configuration of the filter with this new config.
    /// If absent, the global RBAC policy will be disabled for this route.
    #[prost(message, optional, tag = "2")]
    pub rbac: ::core::option::Option<Rbac>,
}
/// Encoded file descriptor set for the `envoy.config.filter.http.rbac.v2` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfa, 0x0c, 0x0a, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x72, 0x62,
    0x61, 0x63, 0x2f, 0x76, 0x32, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e,
    0x76, 0x32, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x76, 0x32, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x77, 0x0a, 0x04, 0x52, 0x42, 0x41, 0x43, 0x12, 0x30, 0x0a, 0x05, 0x72, 0x75,
    0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32,
    0x2e, 0x52, 0x42, 0x41, 0x43, 0x52, 0x05, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x12, 0x3d, 0x0a, 0x0c,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x5f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x42, 0x41, 0x43, 0x52, 0x0b,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x52, 0x75, 0x6c, 0x65, 0x73, 0x22, 0x50, 0x0a, 0x0c, 0x52,
    0x42, 0x41, 0x43, 0x50, 0x65, 0x72, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x12, 0x3a, 0x0a, 0x04, 0x72,
    0x62, 0x61, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e,
    0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x32, 0x2e, 0x52, 0x42, 0x41,
    0x43, 0x52, 0x04, 0x72, 0x62, 0x61, 0x63, 0x4a, 0x04, 0x08, 0x01, 0x10, 0x02, 0x42, 0xc2, 0x01,
    0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x27, 0x12, 0x25, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8,
    0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x2e, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62,
    0x61, 0x63, 0x2e, 0x76, 0x32, 0x42, 0x09, 0x52, 0x62, 0x61, 0x63, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x4e, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68,
    0x74, 0x74, 0x70, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x76, 0x32, 0x3b, 0x72, 0x62, 0x61, 0x63,
    0x76, 0x32, 0x4a, 0xb0, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x26, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07,
    0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x47, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x47, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00,
    0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2a, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x65, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x0c, 0x00, 0x65, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x61,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x61, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80,
    0x99, 0x6a, 0x02, 0x12, 0x03, 0x0e, 0x00, 0x46, 0x0a, 0xb9, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x15, 0x00, 0x1e, 0x01, 0x1a, 0x15, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x0a, 0x32, 0x95, 0x01, 0x20,
    0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x3a, 0x20, 0x52, 0x42, 0x41, 0x43, 0x5d, 0x0a, 0x20, 0x52, 0x6f, 0x6c, 0x65, 0x2d, 0x42, 0x61,
    0x73, 0x65, 0x64, 0x20, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20,
    0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x73, 0x5f, 0x72, 0x62, 0x61, 0x63, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b, 0x23,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x72, 0x62,
    0x61, 0x63, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0c,
    0x0a, 0x73, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x20, 0x1a, 0x66, 0x20,
    0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x42, 0x41, 0x43,
    0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70,
    0x6c, 0x69, 0x65, 0x64, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x6c, 0x79, 0x2e, 0x0a, 0x20,
    0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x6e, 0x6f, 0x20, 0x65, 0x6e,
    0x66, 0x6f, 0x72, 0x63, 0x69, 0x6e, 0x67, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x70, 0x6f, 0x6c,
    0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c,
    0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x18, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x16,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x1e, 0x1f, 0x0a,
    0xc6, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x27, 0x1a, 0xb8, 0x01,
    0x20, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x6e, 0x66, 0x6f, 0x72, 0x63, 0x65, 0x64, 0x20, 0x62,
    0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x28, 0x69, 0x2e,
    0x65, 0x2e, 0x2c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20,
    0x34, 0x30, 0x33, 0x29, 0x0a, 0x20, 0x62, 0x75, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x65,
    0x6d, 0x69, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x6f,
    0x67, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x75, 0x6c, 0x65, 0x20, 0x74, 0x65, 0x73, 0x74,
    0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e, 0x74, 0x2c,
    0x20, 0x6e, 0x6f, 0x20, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20,
    0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61,
    0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x1d, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1d, 0x16, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d,
    0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x20, 0x00, 0x26, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x20, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x09, 0x12, 0x03, 0x21, 0x02, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x09, 0x00, 0x12, 0x03,
    0x21, 0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x09, 0x00, 0x01, 0x12, 0x03, 0x21, 0x0b,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x09, 0x00, 0x02, 0x12, 0x03, 0x21, 0x0b, 0x0c, 0x0a,
    0x99, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x10, 0x1a, 0x8b, 0x01,
    0x20, 0x4f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x6c,
    0x6f, 0x62, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x62, 0x73, 0x65, 0x6e,
    0x74, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20, 0x52, 0x42,
    0x41, 0x43, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62,
    0x65, 0x20, 0x64, 0x69, 0x73, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x25, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x25, 0x07, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x25, 0x0e, 0x0f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.http.rbac.v2.serde.rs");
// @@protoc_insertion_point(module)