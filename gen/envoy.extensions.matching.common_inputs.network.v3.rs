// @generated
/// Specifies that matching should be performed by the destination IP address.
/// [#extension: envoy.matching.inputs.destination_ip]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationIpInput {}
/// Specifies that matching should be performed by the destination port.
/// [#extension: envoy.matching.inputs.destination_port]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationPortInput {}
/// Specifies that matching should be performed by the source IP address.
/// [#extension: envoy.matching.inputs.source_ip]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceIpInput {}
/// Specifies that matching should be performed by the source port.
/// [#extension: envoy.matching.inputs.source_port]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourcePortInput {}
/// Input that matches by the directly connected source IP address (this
/// will only be different from the source IP address when using a listener
/// filter that overrides the source address, such as the :ref:`Proxy Protocol
/// listener filter <config_listener_filters_proxy_protocol>`).
/// [#extension: envoy.matching.inputs.direct_source_ip]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectSourceIpInput {}
/// Input that matches by the source IP type.
/// Specifies the source IP match type. The values include:
///
/// * ``local`` - matches a connection originating from the same host,
/// [#extension: envoy.matching.inputs.source_type]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceTypeInput {}
/// Input that matches by the requested server name (e.g. SNI in TLS).
///
/// :ref:`TLS Inspector <config_listener_filters_tls_inspector>` provides the requested server name based on SNI,
/// when TLS protocol is detected.
/// [#extension: envoy.matching.inputs.server_name]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerNameInput {}
/// Input that matches by the transport protocol.
///
/// Suggested values include:
///
/// * ``raw_buffer`` - default, used when no transport protocol is detected,
/// * ``tls`` - set by :ref:`envoy.filters.listener.tls_inspector <config_listener_filters_tls_inspector>`
///    when TLS protocol is detected.
/// [#extension: envoy.matching.inputs.transport_protocol]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransportProtocolInput {}
/// List of quoted and comma-separated requested application protocols. The list consists of a
/// single negotiated application protocol once the network stream is established.
///
/// Examples:
///
/// * ``'h2','http/1.1'``
/// * ``'h2c'``
///
/// Suggested values in the list include:
///
/// * ``http/1.1`` - set by :ref:`envoy.filters.listener.tls_inspector
///    <config_listener_filters_tls_inspector>` and :ref:`envoy.filters.listener.http_inspector
///    <config_listener_filters_http_inspector>`,
/// * ``h2`` - set by :ref:`envoy.filters.listener.tls_inspector <config_listener_filters_tls_inspector>`
/// * ``h2c`` - set by :ref:`envoy.filters.listener.http_inspector <config_listener_filters_http_inspector>`
///
/// .. attention::
///
///    Currently, :ref:`TLS Inspector <config_listener_filters_tls_inspector>` provides
///    application protocol detection based on the requested
///    `ALPN <<https://en.wikipedia.org/wiki/Application-Layer_Protocol_Negotiation>`_> values.
///
///    However, the use of ALPN is pretty much limited to the HTTP/2 traffic on the Internet,
///    and matching on values other than ``h2`` is going to lead to a lot of false negatives,
///    unless all connecting clients are known to use ALPN.
/// [#extension: envoy.matching.inputs.application_protocol]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationProtocolInput {}
/// Input that matches by a specific filter state key.
/// The value of the provided filter state key will be the raw string representation of the filter state object
/// [#extension: envoy.matching.inputs.filter_state]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterStateInput {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `envoy.extensions.matching.common_inputs.network.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xbb, 0x21, 0x0a, 0x47, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2f, 0x63,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2f, 0x6e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x76, 0x33, 0x2f, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x5f,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x32, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x76, 0x33,
    0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x14, 0x0a, 0x12, 0x44, 0x65, 0x73, 0x74,
    0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x50, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x22, 0x16,
    0x0a, 0x14, 0x44, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x6f, 0x72,
    0x74, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x22, 0x0f, 0x0a, 0x0d, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65,
    0x49, 0x50, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x22, 0x11, 0x0a, 0x0f, 0x53, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x50, 0x6f, 0x72, 0x74, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x22, 0x15, 0x0a, 0x13, 0x44, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x49, 0x50, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x22, 0x11, 0x0a, 0x0f, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x54, 0x79, 0x70, 0x65, 0x49,
    0x6e, 0x70, 0x75, 0x74, 0x22, 0x11, 0x0a, 0x0f, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4e, 0x61,
    0x6d, 0x65, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x22, 0x18, 0x0a, 0x16, 0x54, 0x72, 0x61, 0x6e, 0x73,
    0x70, 0x6f, 0x72, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x22, 0x1a, 0x0a, 0x18, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x22, 0x2d, 0x0a,
    0x10, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x12, 0x19, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07,
    0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x42, 0xc5, 0x01, 0xba,
    0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x40, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74,
    0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67,
    0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x6e,
    0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x76, 0x33, 0x42, 0x12, 0x4e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a,
    0x63, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e,
    0x67, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2f,
    0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x76, 0x33, 0x3b, 0x6e, 0x65, 0x74, 0x77, 0x6f,
    0x72, 0x6b, 0x76, 0x33, 0x4a, 0xbf, 0x1c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x68, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x02, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x07, 0x00, 0x59, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07, 0x00, 0x59, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x33, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03,
    0x08, 0x00, 0x33, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a,
    0x00, 0x7a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0a, 0x00, 0x7a, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a,
    0x02, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0xc2, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x11,
    0x00, 0x12, 0x01, 0x1a, 0x80, 0x01, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x74, 0x69, 0x6e,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x49, 0x50, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x64, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x5f, 0x69, 0x70, 0x5d, 0x0a, 0x32, 0x33, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x6f,
    0x6e, 0x20, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69,
    0x6e, 0x67, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x11, 0x08, 0x1a, 0x0a, 0x88, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x16, 0x00, 0x17, 0x01, 0x1a, 0x7c, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x74, 0x69, 0x6e,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e,
    0x64, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x6f, 0x72, 0x74,
    0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x16, 0x08, 0x1c, 0x0a, 0x82,
    0x01, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1b, 0x00, 0x1c, 0x01, 0x1a, 0x76, 0x20, 0x53, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x49, 0x50, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67,
    0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x69,
    0x70, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x15, 0x0a,
    0x7e, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x20, 0x00, 0x21, 0x01, 0x1a, 0x72, 0x20, 0x53, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20,
    0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x0a, 0x20, 0x5b,
    0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74,
    0x73, 0x2e, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x5d, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x20, 0x08, 0x17, 0x0a, 0xdb, 0x02, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x28, 0x00, 0x29, 0x01, 0x1a, 0xce, 0x02, 0x20, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x62,
    0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20,
    0x49, 0x50, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x28, 0x74, 0x68, 0x69, 0x73,
    0x0a, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x64,
    0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x49, 0x50, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61,
    0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x0a, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x2c, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x50, 0x72, 0x6f, 0x78, 0x79, 0x20, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x0a, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x20,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6c,
    0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f,
    0x70, 0x72, 0x6f, 0x78, 0x79, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x3e, 0x60,
    0x29, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a,
    0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x5f, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x5f, 0x69, 0x70, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x28, 0x08, 0x1b, 0x0a, 0xe7, 0x01, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x30, 0x00,
    0x31, 0x01, 0x1a, 0xda, 0x01, 0x20, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x49, 0x50, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x0a,
    0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x49, 0x50, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20,
    0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x3a, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60, 0x6c,
    0x6f, 0x63, 0x61, 0x6c, 0x60, 0x60, 0x20, 0x2d, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73,
    0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x72,
    0x69, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x2c, 0x0a, 0x20, 0x5b,
    0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74,
    0x73, 0x2e, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x5d, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x30, 0x08, 0x17, 0x0a, 0x92, 0x02, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x38, 0x00, 0x39, 0x01, 0x1a, 0x85, 0x02, 0x20, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x62,
    0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x28, 0x65, 0x2e, 0x67,
    0x2e, 0x20, 0x53, 0x4e, 0x49, 0x20, 0x69, 0x6e, 0x20, 0x54, 0x4c, 0x53, 0x29, 0x2e, 0x0a, 0x0a,
    0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x54, 0x4c, 0x53, 0x20, 0x49, 0x6e, 0x73, 0x70, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6c, 0x69, 0x73,
    0x74, 0x65, 0x6e, 0x65, 0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x74, 0x6c,
    0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x3e, 0x60, 0x20, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x64, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x65, 0x64, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x53, 0x4e, 0x49, 0x2c, 0x0a, 0x20,
    0x77, 0x68, 0x65, 0x6e, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f,
    0x6c, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x20,
    0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x73, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x5d, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x38, 0x08, 0x17, 0x0a, 0xe5, 0x02, 0x0a,
    0x02, 0x04, 0x07, 0x12, 0x04, 0x43, 0x00, 0x44, 0x01, 0x1a, 0xd8, 0x02, 0x20, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74,
    0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x0a, 0x0a, 0x20, 0x53, 0x75, 0x67,
    0x67, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x69, 0x6e,
    0x63, 0x6c, 0x75, 0x64, 0x65, 0x3a, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60, 0x72, 0x61, 0x77,
    0x5f, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x60, 0x60, 0x20, 0x2d, 0x20, 0x64, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x2c, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x6e,
    0x6f, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x2c, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60, 0x74, 0x6c, 0x73, 0x60, 0x60, 0x20, 0x2d, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x62, 0x79, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e,
    0x65, 0x72, 0x2e, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65,
    0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e,
    0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x3e, 0x60, 0x0a, 0x20, 0x20, 0x20, 0x77, 0x68, 0x65,
    0x6e, 0x20, 0x54, 0x4c, 0x53, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x69,
    0x73, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e,
    0x74, 0x72, 0x61, 0x6e, 0x73, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x43, 0x08, 0x1e,
    0x0a, 0xd2, 0x09, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x60, 0x00, 0x61, 0x01, 0x1a, 0xc5, 0x09,
    0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x71, 0x75, 0x6f, 0x74, 0x65, 0x64, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x2d, 0x73, 0x65, 0x70, 0x61, 0x72, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x63, 0x6f,
    0x6e, 0x73, 0x69, 0x73, 0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x0a, 0x20, 0x73, 0x69, 0x6e,
    0x67, 0x6c, 0x65, 0x20, 0x6e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61,
    0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x20, 0x6f, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x74,
    0x77, 0x6f, 0x72, 0x6b, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x69, 0x73, 0x20, 0x65,
    0x73, 0x74, 0x61, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x45, 0x78,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x3a, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60, 0x27, 0x68,
    0x32, 0x27, 0x2c, 0x27, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x31, 0x2e, 0x31, 0x27, 0x60, 0x60, 0x0a,
    0x20, 0x2a, 0x20, 0x60, 0x60, 0x27, 0x68, 0x32, 0x63, 0x27, 0x60, 0x60, 0x0a, 0x0a, 0x20, 0x53,
    0x75, 0x67, 0x67, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x69, 0x6e, 0x63, 0x6c,
    0x75, 0x64, 0x65, 0x3a, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60, 0x68, 0x74, 0x74, 0x70, 0x2f,
    0x31, 0x2e, 0x31, 0x60, 0x60, 0x20, 0x2d, 0x20, 0x73, 0x65, 0x74, 0x20, 0x62, 0x79, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x74, 0x6c, 0x73, 0x5f,
    0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x20, 0x3c, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x5f, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x3e, 0x60, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69,
    0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x69, 0x6e, 0x73, 0x70,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x0a, 0x20, 0x20, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x3e, 0x60, 0x2c, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60, 0x68, 0x32, 0x60, 0x60, 0x20, 0x2d, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x62, 0x79, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65,
    0x6e, 0x65, 0x72, 0x2e, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e,
    0x65, 0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x69,
    0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x3e, 0x60, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60,
    0x68, 0x32, 0x63, 0x60, 0x60, 0x20, 0x2d, 0x20, 0x73, 0x65, 0x74, 0x20, 0x62, 0x79, 0x20, 0x3a,
    0x72, 0x65, 0x66, 0x3a, 0x60, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x3e, 0x60, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x61, 0x74, 0x74, 0x65, 0x6e, 0x74,
    0x69, 0x6f, 0x6e, 0x3a, 0x3a, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x6c, 0x79, 0x2c, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x54, 0x4c, 0x53, 0x20, 0x49,
    0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x5f, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x5f, 0x74, 0x6c, 0x73, 0x5f, 0x69, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x3e,
    0x60, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x73, 0x0a, 0x20, 0x20, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x62, 0x61, 0x73,
    0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x65, 0x64, 0x0a, 0x20, 0x20, 0x20, 0x60, 0x41, 0x4c, 0x50, 0x4e, 0x20, 0x3c, 0x68, 0x74,
    0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x65, 0x6e, 0x2e, 0x77, 0x69, 0x6b, 0x69, 0x70, 0x65, 0x64,
    0x69, 0x61, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x77, 0x69, 0x6b, 0x69, 0x2f, 0x41, 0x70, 0x70, 0x6c,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2d, 0x4c, 0x61, 0x79, 0x65, 0x72, 0x5f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x5f, 0x4e, 0x65, 0x67, 0x6f, 0x74, 0x69, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x3e, 0x60, 0x5f, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x20,
    0x20, 0x20, 0x48, 0x6f, 0x77, 0x65, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x4c, 0x50, 0x4e, 0x20, 0x69, 0x73, 0x20, 0x70, 0x72,
    0x65, 0x74, 0x74, 0x79, 0x20, 0x6d, 0x75, 0x63, 0x68, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x32, 0x20,
    0x74, 0x72, 0x61, 0x66, 0x66, 0x69, 0x63, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x73, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x60, 0x60,
    0x68, 0x32, 0x60, 0x60, 0x20, 0x69, 0x73, 0x20, 0x67, 0x6f, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f,
    0x20, 0x6c, 0x65, 0x61, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x20, 0x6e, 0x65, 0x67, 0x61, 0x74, 0x69, 0x76, 0x65,
    0x73, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x75, 0x6e, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6c, 0x6c,
    0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20, 0x74, 0x6f,
    0x20, 0x75, 0x73, 0x65, 0x20, 0x41, 0x4c, 0x50, 0x4e, 0x2e, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x61,
    0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x60, 0x08,
    0x20, 0x0a, 0xe0, 0x01, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x66, 0x00, 0x68, 0x01, 0x1a, 0xd3,
    0x01, 0x20, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x73, 0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x63, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20,
    0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6b, 0x65, 0x79,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x77,
    0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x66, 0x08, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x67, 0x02, 0x3a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x67, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x09, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x67, 0x0f, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x67, 0x11, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x09, 0x02, 0x00, 0x08, 0xaf, 0x08,
    0x0e, 0x12, 0x03, 0x67, 0x12, 0x38, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.matching.common_inputs.network.v3.serde.rs");
// @@protoc_insertion_point(module)