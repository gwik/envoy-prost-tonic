// @generated
/// Indicates that during forwarding, portions of the path that match the
/// pattern should be rewritten, even allowing the substitution of variables
/// from the match pattern into the new path as specified by the rewrite template.
/// This is useful to allow application paths to be
/// rewritten in a way that is aware of segments with variable content like
/// identifiers. The router filter will place the original path as it was
/// before the rewrite into the :ref:`x-envoy-original-path
/// <config_http_filters_router_x-envoy-original-path>` header.
///
/// Only one of :ref:`prefix_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.prefix_rewrite>`,
/// :ref:`regex_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.regex_rewrite>`,
/// or *path_template_rewrite* may be specified.
///
/// Template pattern matching types:
///
/// * ``*`` : Matches a single path component, up to the next path separator: /
///
/// * ``**`` : Matches zero or more path segments. If present, must be the last operator.
///
/// * ``{name} or {name=*}`` :  A named variable matching one path segment up to the next path separator: /.
///
/// * ``{name=videos/*}`` : A named variable matching more than one path segment.
///       The path component matching videos/* is captured as the named variable.
///
/// * ``{name=**}`` : A named variable matching zero or more path segments.
///
/// Only named matches can be used to perform rewrites.
///
/// Examples using path_template_rewrite:
///
/// * The pattern ``/{one}/{two}`` paired with a substitution string of ``/{two}/{one}`` would
///    transform ``/cat/dog`` into ``/dog/cat``.
///
/// * The pattern ``/videos/{language=lang/*}/*`` paired with a substitution string of
///    ``/{language}`` would transform ``/videos/lang/en/video.m4s`` into ``lang/en``.
///
/// * The path pattern ``/content/{format}/{lang}/{id}/{file}.vtt`` paired with a substitution
///    string of ``/{lang}/{format}/{file}.vtt`` would transform ``/content/hls/en-us/12345/en_193913.vtt``
///    into ``/en-us/hls/en_193913.vtt``.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UriTemplateRewriteConfig {
    #[prost(string, tag = "1")]
    pub path_template_rewrite: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `envoy.extensions.path.rewrite.uri_template.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x80, 0x16, 0x0a, 0x48, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x70, 0x61, 0x74, 0x68, 0x2f, 0x72, 0x65, 0x77, 0x72, 0x69,
    0x74, 0x65, 0x2f, 0x75, 0x72, 0x69, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x2f,
    0x76, 0x33, 0x2f, 0x75, 0x72, 0x69, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x5f,
    0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2d, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e,
    0x70, 0x61, 0x74, 0x68, 0x2e, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2e, 0x75, 0x72, 0x69,
    0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64,
    0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5a, 0x0a, 0x18, 0x55, 0x72, 0x69, 0x54, 0x65, 0x6d, 0x70, 0x6c,
    0x61, 0x74, 0x65, 0x52, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x12, 0x3e, 0x0a, 0x15, 0x70, 0x61, 0x74, 0x68, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74,
    0x65, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42,
    0x0a, 0xfa, 0x42, 0x07, 0x72, 0x05, 0x10, 0x01, 0x18, 0x80, 0x02, 0x52, 0x13, 0x70, 0x61, 0x74,
    0x68, 0x54, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x52, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x42, 0xc5, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a, 0x3b, 0x69, 0x6f, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x61, 0x74, 0x68,
    0x2e, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2e, 0x75, 0x72, 0x69, 0x5f, 0x74, 0x65, 0x6d,
    0x70, 0x6c, 0x61, 0x74, 0x65, 0x2e, 0x76, 0x33, 0x42, 0x17, 0x55, 0x72, 0x69, 0x54, 0x65, 0x6d,
    0x70, 0x6c, 0x61, 0x74, 0x65, 0x52, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0x5a, 0x63, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x70, 0x61, 0x74,
    0x68, 0x2f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2f, 0x75, 0x72, 0x69, 0x5f, 0x74, 0x65,
    0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x33, 0x3b, 0x75, 0x72, 0x69, 0x5f, 0x74, 0x65,
    0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x76, 0x33, 0x4a, 0xa0, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x39, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x36, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04,
    0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x21, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x54, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x07,
    0x00, 0x54, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x38, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x08, 0x12, 0x03, 0x08, 0x00, 0x38, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00,
    0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0a, 0x00, 0x7a, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0a, 0x00,
    0x7a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0x9c, 0x10, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x37, 0x00, 0x39, 0x01, 0x1a, 0x98, 0x0f, 0x20, 0x49, 0x6e, 0x64, 0x69, 0x63,
    0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x64, 0x75, 0x72, 0x69, 0x6e, 0x67,
    0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x70, 0x6f, 0x72,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x74,
    0x68, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x70, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6e, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6e, 0x2c, 0x20, 0x65,
    0x76, 0x65, 0x6e, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x75, 0x62, 0x73, 0x74, 0x69, 0x74, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x0a, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x70, 0x61, 0x74, 0x74, 0x65,
    0x72, 0x6e, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x70, 0x61, 0x74, 0x68, 0x20, 0x61, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65,
    0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65,
    0x20, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x70, 0x61, 0x74, 0x68, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x77,
    0x72, 0x69, 0x74, 0x74, 0x65, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x77, 0x61, 0x79, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x77, 0x61, 0x72, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x76,
    0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20,
    0x6c, 0x69, 0x6b, 0x65, 0x0a, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x72, 0x20, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x70, 0x61, 0x74,
    0x68, 0x20, 0x61, 0x73, 0x20, 0x69, 0x74, 0x20, 0x77, 0x61, 0x73, 0x0a, 0x20, 0x62, 0x65, 0x66,
    0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20,
    0x69, 0x6e, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x78,
    0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2d, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x2d,
    0x70, 0x61, 0x74, 0x68, 0x0a, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74,
    0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f, 0x72, 0x6f, 0x75, 0x74, 0x65,
    0x72, 0x5f, 0x78, 0x2d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2d, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x61, 0x6c, 0x2d, 0x70, 0x61, 0x74, 0x68, 0x3e, 0x60, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x2e, 0x0a, 0x0a, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x5f, 0x72, 0x65, 0x77,
    0x72, 0x69, 0x74, 0x65, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61,
    0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x52, 0x6f, 0x75, 0x74, 0x65, 0x41, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x5f, 0x72, 0x65, 0x77, 0x72,
    0x69, 0x74, 0x65, 0x3e, 0x60, 0x2c, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x72, 0x65,
    0x67, 0x65, 0x78, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x3c, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x2e, 0x76, 0x33, 0x2e,
    0x52, 0x6f, 0x75, 0x74, 0x65, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x72, 0x65, 0x67, 0x65,
    0x78, 0x5f, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x3e, 0x60, 0x2c, 0x0a, 0x20, 0x6f, 0x72,
    0x20, 0x2a, 0x70, 0x61, 0x74, 0x68, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x5f,
    0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2a, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x65, 0x6d,
    0x70, 0x6c, 0x61, 0x74, 0x65, 0x20, 0x70, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6e, 0x20, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x3a, 0x0a, 0x0a, 0x20,
    0x2a, 0x20, 0x60, 0x60, 0x2a, 0x60, 0x60, 0x20, 0x3a, 0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x75, 0x70, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x73,
    0x65, 0x70, 0x61, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x3a, 0x20, 0x2f, 0x0a, 0x0a, 0x20, 0x2a, 0x20,
    0x60, 0x60, 0x2a, 0x2a, 0x60, 0x60, 0x20, 0x3a, 0x20, 0x4d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73,
    0x20, 0x7a, 0x65, 0x72, 0x6f, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x70, 0x61,
    0x74, 0x68, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x49, 0x66, 0x20,
    0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x60, 0x60, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d,
    0x20, 0x6f, 0x72, 0x20, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x3d, 0x2a, 0x7d, 0x60, 0x60, 0x20, 0x3a,
    0x20, 0x20, 0x41, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62,
    0x6c, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x65, 0x20,
    0x70, 0x61, 0x74, 0x68, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x75, 0x70, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x70, 0x61, 0x74, 0x68,
    0x20, 0x73, 0x65, 0x70, 0x61, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x3a, 0x20, 0x2f, 0x2e, 0x0a, 0x0a,
    0x20, 0x2a, 0x20, 0x60, 0x60, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x3d, 0x76, 0x69, 0x64, 0x65, 0x6f,
    0x73, 0x2f, 0x2a, 0x7d, 0x60, 0x60, 0x20, 0x3a, 0x20, 0x41, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x64,
    0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69,
    0x6e, 0x67, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x65,
    0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x63, 0x6f,
    0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67,
    0x20, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x73, 0x2f, 0x2a, 0x20, 0x69, 0x73, 0x20, 0x63, 0x61, 0x70,
    0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x64, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x20, 0x2a,
    0x20, 0x60, 0x60, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x3d, 0x2a, 0x2a, 0x7d, 0x60, 0x60, 0x20, 0x3a,
    0x20, 0x41, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x7a, 0x65, 0x72, 0x6f, 0x20,
    0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x73, 0x65, 0x67,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x64, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20,
    0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f,
    0x72, 0x6d, 0x20, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x45,
    0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x61,
    0x74, 0x68, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x77, 0x72,
    0x69, 0x74, 0x65, 0x3a, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x61, 0x74,
    0x74, 0x65, 0x72, 0x6e, 0x20, 0x60, 0x60, 0x2f, 0x7b, 0x6f, 0x6e, 0x65, 0x7d, 0x2f, 0x7b, 0x74,
    0x77, 0x6f, 0x7d, 0x60, 0x60, 0x20, 0x70, 0x61, 0x69, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x20, 0x61, 0x20, 0x73, 0x75, 0x62, 0x73, 0x74, 0x69, 0x74, 0x75, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x66, 0x20, 0x60, 0x60, 0x2f, 0x7b, 0x74,
    0x77, 0x6f, 0x7d, 0x2f, 0x7b, 0x6f, 0x6e, 0x65, 0x7d, 0x60, 0x60, 0x20, 0x77, 0x6f, 0x75, 0x6c,
    0x64, 0x0a, 0x20, 0x20, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x60,
    0x60, 0x2f, 0x63, 0x61, 0x74, 0x2f, 0x64, 0x6f, 0x67, 0x60, 0x60, 0x20, 0x69, 0x6e, 0x74, 0x6f,
    0x20, 0x60, 0x60, 0x2f, 0x64, 0x6f, 0x67, 0x2f, 0x63, 0x61, 0x74, 0x60, 0x60, 0x2e, 0x0a, 0x0a,
    0x20, 0x2a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6e, 0x20, 0x60,
    0x60, 0x2f, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x73, 0x2f, 0x7b, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61,
    0x67, 0x65, 0x3d, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x2a, 0x7d, 0x2f, 0x2a, 0x60, 0x60, 0x20, 0x70,
    0x61, 0x69, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x73, 0x75, 0x62,
    0x73, 0x74, 0x69, 0x74, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x20, 0x6f, 0x66, 0x0a, 0x20, 0x20, 0x20, 0x60, 0x60, 0x2f, 0x7b, 0x6c, 0x61, 0x6e, 0x67, 0x75,
    0x61, 0x67, 0x65, 0x7d, 0x60, 0x60, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x60, 0x60, 0x2f, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x73,
    0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f, 0x65, 0x6e, 0x2f, 0x76, 0x69, 0x64, 0x65, 0x6f, 0x2e, 0x6d,
    0x34, 0x73, 0x60, 0x60, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x60, 0x60, 0x6c, 0x61, 0x6e, 0x67,
    0x2f, 0x65, 0x6e, 0x60, 0x60, 0x2e, 0x0a, 0x0a, 0x20, 0x2a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70,
    0x61, 0x74, 0x68, 0x20, 0x70, 0x61, 0x74, 0x74, 0x65, 0x72, 0x6e, 0x20, 0x60, 0x60, 0x2f, 0x63,
    0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2f, 0x7b, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x7d, 0x2f,
    0x7b, 0x6c, 0x61, 0x6e, 0x67, 0x7d, 0x2f, 0x7b, 0x69, 0x64, 0x7d, 0x2f, 0x7b, 0x66, 0x69, 0x6c,
    0x65, 0x7d, 0x2e, 0x76, 0x74, 0x74, 0x60, 0x60, 0x20, 0x70, 0x61, 0x69, 0x72, 0x65, 0x64, 0x20,
    0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x73, 0x75, 0x62, 0x73, 0x74, 0x69, 0x74, 0x75, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x20, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x66,
    0x20, 0x60, 0x60, 0x2f, 0x7b, 0x6c, 0x61, 0x6e, 0x67, 0x7d, 0x2f, 0x7b, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x7d, 0x2f, 0x7b, 0x66, 0x69, 0x6c, 0x65, 0x7d, 0x2e, 0x76, 0x74, 0x74, 0x60, 0x60,
    0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x6f, 0x72, 0x6d,
    0x20, 0x60, 0x60, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x2f, 0x68, 0x6c, 0x73, 0x2f,
    0x65, 0x6e, 0x2d, 0x75, 0x73, 0x2f, 0x31, 0x32, 0x33, 0x34, 0x35, 0x2f, 0x65, 0x6e, 0x5f, 0x31,
    0x39, 0x33, 0x39, 0x31, 0x33, 0x2e, 0x76, 0x74, 0x74, 0x60, 0x60, 0x0a, 0x20, 0x20, 0x20, 0x69,
    0x6e, 0x74, 0x6f, 0x20, 0x60, 0x60, 0x2f, 0x65, 0x6e, 0x2d, 0x75, 0x73, 0x2f, 0x68, 0x6c, 0x73,
    0x2f, 0x65, 0x6e, 0x5f, 0x31, 0x39, 0x33, 0x39, 0x31, 0x33, 0x2e, 0x76, 0x74, 0x74, 0x60, 0x60,
    0x2e, 0x0a, 0x32, 0x75, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d,
    0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x55, 0x72, 0x69, 0x20, 0x54, 0x65, 0x6d, 0x70, 0x6c,
    0x61, 0x74, 0x65, 0x20, 0x52, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x70, 0x61, 0x74, 0x68, 0x2e, 0x72, 0x65, 0x77,
    0x72, 0x69, 0x74, 0x65, 0x2e, 0x75, 0x72, 0x69, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74,
    0x65, 0x2e, 0x75, 0x72, 0x69, 0x5f, 0x74, 0x65, 0x6d, 0x70, 0x6c, 0x61, 0x74, 0x65, 0x5f, 0x72,
    0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x37, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x38,
    0x02, 0x59, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x09, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x38, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x38, 0x23, 0x58, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00,
    0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x38, 0x24, 0x57, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("envoy.extensions.path.rewrite.uri_template.v3.serde.rs");
// @@protoc_insertion_point(module)