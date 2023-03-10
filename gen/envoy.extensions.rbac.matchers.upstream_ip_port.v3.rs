// @generated
/// This is configuration for matching upstream ip and port.
/// Note that although both fields are optional, at least one of IP or port must be supplied. If only
/// one is supplied the other is a wildcard match.
/// This matcher requires a filter in the chain to have saved the upstream address in the
/// filter state before the matcher is executed by RBAC filter. The state should be saved with key
/// ``envoy.stream.upstream_address`` (See
/// :repo:`upstream_address.h<source/common/stream_info/upstream_address.h>`).
/// Also, See :repo:`proxy_filter.cc<source/extensions/filters/http/dynamic_forward_proxy/proxy_filter.cc>`
/// for an example of a filter which populates the FilterState.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamIpPortMatcher {
    /// A CIDR block that will be used to match the upstream IP.
    /// Both Ipv4 and Ipv6 ranges can be matched.
    #[prost(message, optional, tag = "1")]
    pub upstream_ip: ::core::option::Option<
        super::super::super::super::super::config::core::v3::CidrRange,
    >,
    /// A port range that will be used to match the upstream port.
    #[prost(message, optional, tag = "2")]
    pub upstream_port_range: ::core::option::Option<
        super::super::super::super::super::r#type::v3::Int64Range,
    >,
}
/// Encoded file descriptor set for the `envoy.extensions.rbac.matchers.upstream_ip_port.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xdc, 0x0e, 0x0a, 0x51, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x65, 0x72, 0x73, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x70, 0x5f,
    0x70, 0x6f, 0x72, 0x74, 0x2f, 0x76, 0x33, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x5f, 0x69, 0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x32, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x65, 0x72, 0x73, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f,
    0x69, 0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x76, 0x33, 0x1a, 0x22, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x33,
    0x2f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x72, 0x61,
    0x6e, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa4, 0x01, 0x0a, 0x15, 0x55, 0x70, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x49, 0x70, 0x50, 0x6f, 0x72, 0x74, 0x4d, 0x61, 0x74, 0x63, 0x68,
    0x65, 0x72, 0x12, 0x40, 0x0a, 0x0b, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x69,
    0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x43,
    0x69, 0x64, 0x72, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x0a, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x49, 0x70, 0x12, 0x49, 0x0a, 0x13, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x5f, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x19, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76,
    0x33, 0x2e, 0x49, 0x6e, 0x74, 0x36, 0x34, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x11, 0x75, 0x70,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x50, 0x6f, 0x72, 0x74, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x42,
    0xd6, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x40, 0x69, 0x6f, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x73, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x5f, 0x69, 0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x76, 0x33, 0x42, 0x1a, 0x55, 0x70,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x49, 0x70, 0x50, 0x6f, 0x72, 0x74, 0x4d, 0x61, 0x74, 0x63,
    0x68, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x6c, 0x67, 0x69, 0x74, 0x68,
    0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78,
    0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61,
    0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x62, 0x61, 0x63, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72,
    0x73, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x69, 0x70, 0x5f, 0x70, 0x6f,
    0x72, 0x74, 0x2f, 0x76, 0x33, 0x3b, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x69,
    0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x76, 0x33, 0x4a, 0xec, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x22, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04,
    0x00, 0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x23, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x07, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09,
    0x00, 0x59, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x59, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0a,
    0x00, 0x3b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12, 0x04, 0x0c, 0x00,
    0x83, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x04, 0x0c, 0x00, 0x83, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99,
    0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a, 0x9e, 0x06, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x1b, 0x00, 0x22, 0x01, 0x1a, 0x9e, 0x05, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x20, 0x69, 0x70, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x2e,
    0x0a, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6c, 0x74, 0x68,
    0x6f, 0x75, 0x67, 0x68, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x2c, 0x20, 0x61,
    0x74, 0x20, 0x6c, 0x65, 0x61, 0x73, 0x74, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x49,
    0x50, 0x20, 0x6f, 0x72, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x0a, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x75, 0x70, 0x70,
    0x6c, 0x69, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x20, 0x77, 0x69, 0x6c, 0x64, 0x63, 0x61, 0x72, 0x64, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65,
    0x72, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x73, 0x20, 0x61, 0x20, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e,
    0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x73, 0x61, 0x76, 0x65, 0x64, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x65,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x73, 0x61, 0x76, 0x65,
    0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x20, 0x60, 0x60, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x60, 0x60, 0x20, 0x28, 0x53,
    0x65, 0x65, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x70, 0x6f, 0x3a, 0x60, 0x75, 0x70, 0x73, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x68, 0x3c, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x68, 0x3e, 0x60, 0x29, 0x2e, 0x0a, 0x20,
    0x41, 0x6c, 0x73, 0x6f, 0x2c, 0x20, 0x53, 0x65, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x70, 0x6f, 0x3a,
    0x60, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x63, 0x63,
    0x3c, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f,
    0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x5f,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x2e, 0x63, 0x63, 0x3e, 0x60, 0x0a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20,
    0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x66, 0x69, 0x6c,
    0x74, 0x65, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61,
    0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x2e, 0x0a, 0x32, 0x71, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64,
    0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x52, 0x42, 0x41, 0x43, 0x20, 0x75,
    0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x49, 0x50, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70,
    0x6f, 0x72, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x72, 0x20, 0x70, 0x6c, 0x75, 0x67,
    0x69, 0x6e, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x72, 0x62, 0x61, 0x63, 0x2e, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x72, 0x73, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5f, 0x69,
    0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x1b, 0x08, 0x1d, 0x0a, 0x72, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x02,
    0x2b, 0x1a, 0x65, 0x20, 0x41, 0x20, 0x43, 0x49, 0x44, 0x52, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x49, 0x50, 0x2e, 0x0a, 0x20, 0x42, 0x6f,
    0x74, 0x68, 0x20, 0x49, 0x70, 0x76, 0x34, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x49, 0x70, 0x76, 0x36,
    0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x1e, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1e, 0x1b, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e,
    0x29, 0x2a, 0x0a, 0x49, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x21, 0x02, 0x2d, 0x1a,
    0x3c, 0x20, 0x41, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x70,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x21, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x15, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x21, 0x2b, 0x2c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.rbac.matchers.upstream_ip_port.v3.serde.rs");
// @@protoc_insertion_point(module)