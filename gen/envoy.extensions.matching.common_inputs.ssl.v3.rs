// @generated
/// List of comma-delimited URIs in the SAN field of the peer certificate for a downstream.
/// [#extension: envoy.matching.inputs.uri_san]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UriSanInput {}
/// List of comma-delimited DNS entries in the SAN field of the peer certificate for a downstream.
/// [#extension: envoy.matching.inputs.dns_san]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsSanInput {}
/// Input that matches the subject field of the peer certificate in RFC 2253 format for a
/// downstream.
/// [#extension: envoy.matching.inputs.subject]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectInput {}
/// Encoded file descriptor set for the `envoy.extensions.matching.common_inputs.ssl.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xbe, 0x08, 0x0a, 0x3f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2f, 0x63,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2f, 0x73, 0x73, 0x6c,
    0x2f, 0x76, 0x33, 0x2f, 0x73, 0x73, 0x6c, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e,
    0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x73, 0x73,
    0x6c, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x0d, 0x0a, 0x0b, 0x55, 0x72, 0x69, 0x53, 0x61, 0x6e, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x22, 0x0d, 0x0a, 0x0b, 0x44, 0x6e, 0x73, 0x53, 0x61, 0x6e, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x22, 0x0e, 0x0a, 0x0c, 0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x42, 0xb5, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x3c, 0x69, 0x6f,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x70,
    0x75, 0x74, 0x73, 0x2e, 0x73, 0x73, 0x6c, 0x2e, 0x76, 0x33, 0x42, 0x0e, 0x53, 0x73, 0x6c, 0x49,
    0x6e, 0x70, 0x75, 0x74, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x5b, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70,
    0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2f, 0x63,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2f, 0x73, 0x73, 0x6c,
    0x2f, 0x76, 0x33, 0x3b, 0x73, 0x73, 0x6c, 0x76, 0x33, 0x4a, 0xbd, 0x05, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x1c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x37, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x04, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x55, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x06, 0x00, 0x55, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07,
    0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x07, 0x00, 0x2f, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x08,
    0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x72, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x03, 0x09, 0x00, 0x72, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00,
    0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0a, 0x00, 0x46,
    0x0a, 0xc4, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x10, 0x00, 0x11, 0x01, 0x1a, 0x86, 0x01,
    0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x2d, 0x64,
    0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x20, 0x55, 0x52, 0x49, 0x73, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x41, 0x4e, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65, 0x65, 0x72, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69,
    0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x64, 0x6f, 0x77,
    0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x75, 0x72, 0x69,
    0x5f, 0x73, 0x61, 0x6e, 0x5d, 0x0a, 0x32, 0x2f, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x6f,
    0x6e, 0x20, 0x53, 0x53, 0x4c, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x73, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x10, 0x08, 0x13, 0x0a, 0x9a, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x16, 0x01,
    0x1a, 0x8d, 0x01, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x61, 0x2d, 0x64, 0x65, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x20, 0x44, 0x4e, 0x53, 0x20,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53,
    0x41, 0x4e, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x70, 0x65, 0x65, 0x72, 0x20, 0x63, 0x65, 0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a,
    0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x64, 0x6e, 0x73, 0x5f, 0x73, 0x61, 0x6e, 0x5d, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x15, 0x08, 0x13, 0x0a, 0x9e, 0x01, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x1b, 0x00, 0x1c, 0x01, 0x1a, 0x91, 0x01, 0x20, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65, 0x65, 0x72, 0x20, 0x63, 0x65,
    0x72, 0x74, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x52, 0x46, 0x43,
    0x20, 0x32, 0x32, 0x35, 0x33, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x61, 0x0a, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x0a,
    0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70,
    0x75, 0x74, 0x73, 0x2e, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x14, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
include!("envoy.extensions.matching.common_inputs.ssl.v3.serde.rs");
// @@protoc_insertion_point(module)