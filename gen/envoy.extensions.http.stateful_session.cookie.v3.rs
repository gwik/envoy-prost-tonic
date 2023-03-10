// @generated
/// This extension allows the session state to be tracked via cookies.
///
/// This extension first encodes the address of the upstream host selected by the load balancer
/// into a ``set-cookie`` response header with the :ref:`cookie configuration
/// <envoy_v3_api_field_extensions.http.stateful_session.cookie.v3.CookieBasedSessionState.cookie>`.
/// when new requests are incoming, this extension will try to parse the specific upstream host
/// address by the cookie name. If the address parsed from the cookie corresponds to a valid
/// upstream host, this upstream host will be selected first. See :ref:`stateful session filter
/// <envoy_v3_api_msg_extensions.filters.http.stateful_session.v3.StatefulSession>`.
///
/// For example, if the cookie name is set to ``sticky-host``, envoy will prefer ``1.2.3.4:80``
/// as the upstream host when the request contains the following header:
///
/// .. code-block:: none
///
///      cookie: sticky-host="MS4yLjMuNDo4MA=="
///
/// When processing the upstream response, if ``1.2.3.4:80`` is indeed the final choice the extension
/// does nothing. If ``1.2.3.4:80`` is not the final choice, the new selected host will be used to
/// update the cookie (via the ``set-cookie`` response header).
///
/// [#extension: envoy.http.stateful_session.cookie]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CookieBasedSessionState {
    /// The cookie configuration used to track session state.
    #[prost(message, optional, tag = "1")]
    pub cookie: ::core::option::Option<
        super::super::super::super::super::r#type::http::v3::Cookie,
    >,
}
/// Encoded file descriptor set for the `envoy.extensions.http.stateful_session.cookie.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xce, 0x10, 0x0a, 0x3d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x66, 0x75, 0x6c, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x63, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x2f, 0x76, 0x33, 0x2f, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x30, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x65, 0x66,
    0x75, 0x6c, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x63, 0x6f, 0x6f, 0x6b, 0x69,
    0x65, 0x2e, 0x76, 0x33, 0x1a, 0x1f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x74, 0x79, 0x70, 0x65,
    0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x76, 0x33, 0x2f, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x57, 0x0a,
    0x17, 0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x42, 0x61, 0x73, 0x65, 0x64, 0x53, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x3c, 0x0a, 0x06, 0x63, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x76, 0x33, 0x2e, 0x43, 0x6f,
    0x6f, 0x6b, 0x69, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x8a, 0x01, 0x02, 0x10, 0x01, 0x52, 0x06,
    0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x42, 0xb9, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10,
    0x02, 0x0a, 0x3e, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x65, 0x66, 0x75, 0x6c, 0x5f,
    0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x2e, 0x76,
    0x33, 0x42, 0x0b, 0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0x5a, 0x60, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65,
    0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x66, 0x75, 0x6c, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2f,
    0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x2f, 0x76, 0x33, 0x3b, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65,
    0x76, 0x33, 0x4a, 0xe4, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2a, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00,
    0x39, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x07,
    0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x57, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x57, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00,
    0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x2c, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0b, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0b, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x77, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x0c, 0x00, 0x77, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x46,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0d, 0x00, 0x46, 0x0a,
    0x9c, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x27, 0x00, 0x2a, 0x01, 0x1a, 0xd1, 0x09, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x61,
    0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72,
    0x61, 0x63, 0x6b, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x20, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x68, 0x6f,
    0x73, 0x74, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x72,
    0x0a, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x60, 0x60, 0x73, 0x65, 0x74, 0x2d, 0x63,
    0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x60, 0x60, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x3c, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70,
    0x2e, 0x73, 0x74, 0x61, 0x74, 0x65, 0x66, 0x75, 0x6c, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x2e, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x43, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x42, 0x61, 0x73, 0x65, 0x64, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x2e, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x77,
    0x68, 0x65, 0x6e, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x74, 0x72, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x75, 0x70, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x0a, 0x20, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6f, 0x6b, 0x69,
    0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x70, 0x61, 0x72, 0x73, 0x65, 0x64, 0x20, 0x66, 0x72,
    0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x20, 0x63, 0x6f,
    0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x0a, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x68,
    0x6f, 0x73, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20,
    0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x2e, 0x20,
    0x53, 0x65, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x73, 0x74, 0x61, 0x74, 0x65, 0x66,
    0x75, 0x6c, 0x20, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x0a, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69,
    0x5f, 0x6d, 0x73, 0x67, 0x5f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x66, 0x75, 0x6c, 0x5f, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x76, 0x33,
    0x2e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x66, 0x75, 0x6c, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x3e, 0x60, 0x2e, 0x0a, 0x0a, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65,
    0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x60, 0x60, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x79, 0x2d, 0x68, 0x6f, 0x73, 0x74, 0x60, 0x60, 0x2c,
    0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x70, 0x72, 0x65, 0x66,
    0x65, 0x72, 0x20, 0x60, 0x60, 0x31, 0x2e, 0x32, 0x2e, 0x33, 0x2e, 0x34, 0x3a, 0x38, 0x30, 0x60,
    0x60, 0x0a, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20,
    0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x3a, 0x0a, 0x0a, 0x20, 0x2e, 0x2e, 0x20, 0x63, 0x6f, 0x64,
    0x65, 0x2d, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x3a, 0x3a, 0x20, 0x6e, 0x6f, 0x6e, 0x65, 0x0a, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x3a, 0x20, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x79, 0x2d, 0x68, 0x6f, 0x73, 0x74, 0x3d, 0x22, 0x4d, 0x53, 0x34, 0x79, 0x4c, 0x6a,
    0x4d, 0x75, 0x4e, 0x44, 0x6f, 0x34, 0x4d, 0x41, 0x3d, 0x3d, 0x22, 0x0a, 0x0a, 0x20, 0x57, 0x68,
    0x65, 0x6e, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x60, 0x60, 0x31, 0x2e, 0x32, 0x2e, 0x33, 0x2e,
    0x34, 0x3a, 0x38, 0x30, 0x60, 0x60, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x65, 0x64,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x63, 0x68, 0x6f, 0x69, 0x63,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x0a,
    0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x2e, 0x20, 0x49,
    0x66, 0x20, 0x60, 0x60, 0x31, 0x2e, 0x32, 0x2e, 0x33, 0x2e, 0x34, 0x3a, 0x38, 0x30, 0x60, 0x60,
    0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6e, 0x61,
    0x6c, 0x20, 0x63, 0x68, 0x6f, 0x69, 0x63, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65,
    0x77, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x0a,
    0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6f, 0x6b,
    0x69, 0x65, 0x20, 0x28, 0x76, 0x69, 0x61, 0x20, 0x74, 0x68, 0x65, 0x20, 0x60, 0x60, 0x73, 0x65,
    0x74, 0x2d, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x60, 0x60, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x29, 0x2e, 0x0a, 0x0a, 0x20, 0x5b,
    0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x65, 0x66, 0x75, 0x6c, 0x5f,
    0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x5d, 0x0a,
    0x32, 0x3c, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69,
    0x74, 0x6c, 0x65, 0x3a, 0x20, 0x43, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x20, 0x62, 0x61, 0x73, 0x65,
    0x64, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x66, 0x75, 0x6c, 0x20, 0x73, 0x65, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x5d, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x27, 0x08, 0x1f, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x4f, 0x1a, 0x37, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f,
    0x6f, 0x6b, 0x69, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b,
    0x20, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x02, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x29, 0x21, 0x4e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00,
    0x08, 0xaf, 0x08, 0x11, 0x12, 0x03, 0x29, 0x22, 0x4d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
include!("envoy.extensions.http.stateful_session.cookie.v3.serde.rs");
// @@protoc_insertion_point(module)