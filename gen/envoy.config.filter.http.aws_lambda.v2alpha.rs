// @generated
/// AWS Lambda filter config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The ARN of the AWS Lambda to invoke when the filter is engaged
    /// Must be in the following format:
    /// arn:<partition>:lambda:<region>:<account-number>:function:<function-name>
    #[prost(string, tag = "1")]
    pub arn: ::prost::alloc::string::String,
    /// Whether to transform the request (headers and body) to a JSON payload or pass it as is.
    #[prost(bool, tag = "2")]
    pub payload_passthrough: bool,
    /// Determines the way to invoke the Lambda function.
    #[prost(enumeration = "config::InvocationMode", tag = "3")]
    pub invocation_mode: i32,
}
/// Nested message and enum types in `Config`.
pub mod config {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum InvocationMode {
        /// This is the more common mode of invocation, in which Lambda responds after it has completed the function. In
        /// this mode the output of the Lambda function becomes the response of the HTTP request.
        Synchronous = 0,
        /// In this mode Lambda responds immediately but continues to process the function asynchronously. This mode can be
        /// used to signal events for example. In this mode, Lambda responds with an acknowledgment that it received the
        /// call which is translated to an HTTP 200 OK by the filter.
        Asynchronous = 1,
    }
    impl InvocationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InvocationMode::Synchronous => "SYNCHRONOUS",
                InvocationMode::Asynchronous => "ASYNCHRONOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYNCHRONOUS" => Some(Self::Synchronous),
                "ASYNCHRONOUS" => Some(Self::Asynchronous),
                _ => None,
            }
        }
    }
}
/// Per-route configuration for AWS Lambda. This can be useful when invoking a different Lambda function or a different
/// version of the same Lambda depending on the route.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerRouteConfig {
    #[prost(message, optional, tag = "1")]
    pub invoke_config: ::core::option::Option<Config>,
}
/// Encoded file descriptor set for the `envoy.config.filter.http.aws_lambda.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xcd, 0x14, 0x0a, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68, 0x74, 0x74, 0x70, 0x2f, 0x61, 0x77,
    0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x2f, 0x61, 0x77, 0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61, 0x77, 0x73, 0x5f,
    0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x1e,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d,
    0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x80, 0x02, 0x0a, 0x06, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x12, 0x19, 0x0a, 0x03, 0x61, 0x72, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07,
    0xfa, 0x42, 0x04, 0x72, 0x02, 0x10, 0x01, 0x52, 0x03, 0x61, 0x72, 0x6e, 0x12, 0x2f, 0x0a, 0x13,
    0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x70, 0x61, 0x73, 0x73, 0x74, 0x68, 0x72, 0x6f,
    0x75, 0x67, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x70, 0x61, 0x79, 0x6c, 0x6f,
    0x61, 0x64, 0x50, 0x61, 0x73, 0x73, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x12, 0x75, 0x0a,
    0x0f, 0x69, 0x6e, 0x76, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x6f, 0x64, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x42, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74,
    0x70, 0x2e, 0x61, 0x77, 0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2e, 0x76, 0x32, 0x61,
    0x6c, 0x70, 0x68, 0x61, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x49, 0x6e, 0x76, 0x6f,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x6f, 0x64, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x82,
    0x01, 0x02, 0x10, 0x01, 0x52, 0x0e, 0x69, 0x6e, 0x76, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x4d, 0x6f, 0x64, 0x65, 0x22, 0x33, 0x0a, 0x0e, 0x49, 0x6e, 0x76, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x4d, 0x6f, 0x64, 0x65, 0x12, 0x0f, 0x0a, 0x0b, 0x53, 0x59, 0x4e, 0x43, 0x48, 0x52,
    0x4f, 0x4e, 0x4f, 0x55, 0x53, 0x10, 0x00, 0x12, 0x10, 0x0a, 0x0c, 0x41, 0x53, 0x59, 0x4e, 0x43,
    0x48, 0x52, 0x4f, 0x4e, 0x4f, 0x55, 0x53, 0x10, 0x01, 0x22, 0x6a, 0x0a, 0x0e, 0x50, 0x65, 0x72,
    0x52, 0x6f, 0x75, 0x74, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x58, 0x0a, 0x0d, 0x69,
    0x6e, 0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x33, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61, 0x77,
    0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x0c, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x42, 0xde, 0x01, 0xf2, 0x98, 0xfe, 0x8f, 0x05, 0x2d, 0x12, 0x2b,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61, 0x77,
    0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2e, 0x76, 0x33, 0xba, 0x80, 0xc8, 0xd1, 0x06,
    0x04, 0x08, 0x01, 0x10, 0x01, 0x0a, 0x39, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70,
    0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61, 0x77,
    0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x42, 0x0e, 0x41, 0x77, 0x73, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x52, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2f, 0x68,
    0x74, 0x74, 0x70, 0x2f, 0x61, 0x77, 0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x2f, 0x76,
    0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x4a, 0xaf, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x32,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x34, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x28,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x06, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x52,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x08, 0x00, 0x52, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x09, 0x00, 0x2f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x09, 0x00, 0x2f,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a,
    0x12, 0x03, 0x0a, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x69, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0b, 0x00, 0x69, 0x0a, 0x09, 0x0a, 0x01, 0x08, 0x12,
    0x04, 0x0c, 0x00, 0x0d, 0x32, 0x0a, 0x0e, 0x0a, 0x06, 0x08, 0x8e, 0xe3, 0xff, 0x51, 0x02, 0x12,
    0x04, 0x0c, 0x00, 0x0d, 0x32, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x3e, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x01, 0x12, 0x03, 0x0e, 0x00, 0x3e, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99,
    0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0xc1, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x16, 0x00, 0x2c, 0x01, 0x1a, 0x1a, 0x20, 0x41, 0x57, 0x53, 0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64,
    0x61, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x0a,
    0x32, 0x98, 0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74,
    0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x41, 0x57, 0x53, 0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61,
    0x5d, 0x0a, 0x20, 0x41, 0x57, 0x53, 0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x20, 0x3a, 0x72,
    0x65, 0x66, 0x3a, 0x60, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x20, 0x3c, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x5f,
    0x61, 0x77, 0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x3e, 0x60, 0x2e, 0x0a, 0x20, 0x5b,
    0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x68, 0x74, 0x74, 0x70, 0x2e, 0x61,
    0x77, 0x73, 0x5f, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x0e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12,
    0x04, 0x17, 0x02, 0x20, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x17, 0x07, 0x15, 0x0a, 0xd5, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x1a, 0x04, 0x14, 0x1a, 0xc5, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20, 0x6d,
    0x6f, 0x64, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x6e, 0x76, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2c, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x4c, 0x61, 0x6d, 0x62,
    0x64, 0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x61, 0x66, 0x74, 0x65,
    0x72, 0x20, 0x69, 0x74, 0x20, 0x68, 0x61, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x20, 0x49, 0x6e, 0x0a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x62, 0x65, 0x63, 0x6f, 0x6d, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x48, 0x54, 0x54,
    0x50, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1a, 0x12, 0x13, 0x0a, 0xaa, 0x02, 0x0a, 0x06,
    0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x04, 0x15, 0x1a, 0x9a, 0x02, 0x20, 0x49,
    0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x20, 0x4c, 0x61, 0x6d, 0x62,
    0x64, 0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x69, 0x6d, 0x6d, 0x65,
    0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x20, 0x62, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x69, 0x6e, 0x75, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x73,
    0x79, 0x6e, 0x63, 0x68, 0x72, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x6c, 0x79, 0x2e, 0x20, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x0a, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x2c,
    0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x63, 0x6b, 0x6e, 0x6f, 0x77, 0x6c,
    0x65, 0x64, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x74, 0x20,
    0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x63, 0x61,
    0x6c, 0x6c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x48, 0x54, 0x54,
    0x50, 0x20, 0x32, 0x30, 0x30, 0x20, 0x4f, 0x4b, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x1f, 0x13, 0x14, 0x0a, 0xbb, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x25, 0x02, 0x3a, 0x1a, 0xad, 0x01, 0x20, 0x54, 0x68, 0x65, 0x20, 0x41, 0x52,
    0x4e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x57, 0x53, 0x20, 0x4c, 0x61, 0x6d,
    0x62, 0x64, 0x61, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73,
    0x20, 0x65, 0x6e, 0x67, 0x61, 0x67, 0x65, 0x64, 0x0a, 0x20, 0x4d, 0x75, 0x73, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69,
    0x6e, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x3a, 0x0a, 0x20, 0x61, 0x72, 0x6e, 0x3a,
    0x3c, 0x70, 0x61, 0x72, 0x74, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3e, 0x3a, 0x6c, 0x61, 0x6d, 0x62,
    0x64, 0x61, 0x3a, 0x3c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x3e, 0x3a, 0x3c, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x2d, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x3e, 0x3a, 0x66, 0x75, 0x6e,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x3c, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2d,
    0x6e, 0x61, 0x6d, 0x65, 0x3e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x25, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25,
    0x09, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x0f, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x25, 0x11, 0x39, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x25, 0x12, 0x38, 0x0a,
    0x66, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x28, 0x02, 0x1f, 0x1a, 0x59, 0x20, 0x57,
    0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x6f, 0x72, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x28, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x62, 0x6f, 0x64,
    0x79, 0x29, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x20, 0x70, 0x61, 0x79,
    0x6c, 0x6f, 0x61, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x70, 0x61, 0x73, 0x73, 0x20, 0x69, 0x74, 0x20,
    0x61, 0x73, 0x20, 0x69, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x28, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x28, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x28, 0x1d,
    0x1e, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x54, 0x1a, 0x33,
    0x20, 0x44, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x77, 0x61, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x76, 0x6f, 0x6b, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2b, 0x02,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x11, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x23, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x2b, 0x25, 0x53, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x02, 0x02, 0x08, 0xaf, 0x08, 0x10, 0x12, 0x03, 0x2b, 0x26, 0x52, 0x0a, 0xb6, 0x01, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x30, 0x00, 0x32, 0x01, 0x1a, 0xa9, 0x01, 0x20, 0x50, 0x65, 0x72,
    0x2d, 0x72, 0x6f, 0x75, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x41, 0x57, 0x53, 0x20, 0x4c, 0x61, 0x6d,
    0x62, 0x64, 0x61, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65,
    0x20, 0x75, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x69, 0x6e, 0x76,
    0x6f, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74,
    0x0a, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x4c, 0x61, 0x6d, 0x62, 0x64, 0x61, 0x20, 0x64, 0x65, 0x70,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f,
    0x75, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x30, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x19, 0x1a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.filter.http.aws_lambda.v2alpha.serde.rs");
// @@protoc_insertion_point(module)