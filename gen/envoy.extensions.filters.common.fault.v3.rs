// @generated
/// Delay specification is used to inject latency into the
/// HTTP/Mongo operation.
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaultDelay {
    /// The percentage of operations/connections/requests on which the delay will be injected.
    #[prost(message, optional, tag = "4")]
    pub percentage: ::core::option::Option<
        super::super::super::super::super::r#type::v3::FractionalPercent,
    >,
    #[prost(oneof = "fault_delay::FaultDelaySecifier", tags = "3, 5")]
    pub fault_delay_secifier: ::core::option::Option<fault_delay::FaultDelaySecifier>,
}
/// Nested message and enum types in `FaultDelay`.
pub mod fault_delay {
    /// Fault delays are controlled via an HTTP header (if applicable). See the
    /// :ref:`HTTP fault filter <config_http_filters_fault_injection_http_header>` documentation for
    /// more information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderDelay {}
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
    pub enum FaultDelayType {
        /// Unused and deprecated.
        Fixed = 0,
    }
    impl FaultDelayType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FaultDelayType::Fixed => "FIXED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIXED" => Some(Self::Fixed),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FaultDelaySecifier {
        /// Add a fixed delay before forwarding the operation upstream. See
        /// <https://developers.google.com/protocol-buffers/docs/proto3#json> for
        /// the JSON/YAML Duration mapping. For HTTP/Mongo, the specified
        /// delay will be injected before a new request/operation.
        /// This is required if type is FIXED.
        #[prost(message, tag = "3")]
        FixedDelay(::pbjson_types::Duration),
        /// Fault delays are controlled via an HTTP header (if applicable).
        #[prost(message, tag = "5")]
        HeaderDelay(HeaderDelay),
    }
}
/// Describes a rate limit to be applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaultRateLimit {
    /// The percentage of operations/connections/requests on which the rate limit will be injected.
    #[prost(message, optional, tag = "2")]
    pub percentage: ::core::option::Option<
        super::super::super::super::super::r#type::v3::FractionalPercent,
    >,
    #[prost(oneof = "fault_rate_limit::LimitType", tags = "1, 3")]
    pub limit_type: ::core::option::Option<fault_rate_limit::LimitType>,
}
/// Nested message and enum types in `FaultRateLimit`.
pub mod fault_rate_limit {
    /// Describes a fixed/constant rate limit.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixedLimit {
        /// The limit supplied in KiB/s.
        #[prost(uint64, tag = "1")]
        pub limit_kbps: u64,
    }
    /// Rate limits are controlled via an HTTP header (if applicable). See the
    /// :ref:`HTTP fault filter <config_http_filters_fault_injection_http_header>` documentation for
    /// more information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HeaderLimit {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LimitType {
        /// A fixed rate limit.
        #[prost(message, tag = "1")]
        FixedLimit(FixedLimit),
        /// Rate limits are controlled via an HTTP header (if applicable).
        #[prost(message, tag = "3")]
        HeaderLimit(HeaderLimit),
    }
}
/// Encoded file descriptor set for the `envoy.extensions.filters.common.fault.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb1, 0x1f, 0x0a, 0x34, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x63, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x2f, 0x76, 0x33, 0x2f, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x28, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x2e, 0x76, 0x33, 0x1a, 0x1b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f,
    0x76, 0x33, 0x2f, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x21, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc4, 0x03, 0x0a, 0x0a,
    0x46, 0x61, 0x75, 0x6c, 0x74, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x12, 0x46, 0x0a, 0x0b, 0x66, 0x69,
    0x78, 0x65, 0x64, 0x5f, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x08, 0xfa, 0x42, 0x05, 0xaa,
    0x01, 0x02, 0x2a, 0x00, 0x48, 0x00, 0x52, 0x0a, 0x66, 0x69, 0x78, 0x65, 0x64, 0x44, 0x65, 0x6c,
    0x61, 0x79, 0x12, 0x65, 0x0a, 0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x64, 0x65, 0x6c,
    0x61, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x40, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74,
    0x65, 0x72, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x2e, 0x76, 0x33, 0x2e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x2e, 0x48,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x48, 0x00, 0x52, 0x0b, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x12, 0x40, 0x0a, 0x0a, 0x70, 0x65, 0x72,
    0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x46, 0x72,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x52,
    0x0a, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x1a, 0x49, 0x0a, 0x0b, 0x48,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x3a, 0x3a, 0x9a, 0xc5, 0x88, 0x1e,
    0x35, 0x0a, 0x33, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x2e, 0x76, 0x32, 0x2e,
    0x46, 0x61, 0x75, 0x6c, 0x74, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x22, 0x1b, 0x0a, 0x0e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x44,
    0x65, 0x6c, 0x61, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x49, 0x58, 0x45,
    0x44, 0x10, 0x00, 0x3a, 0x2e, 0x9a, 0xc5, 0x88, 0x1e, 0x29, 0x0a, 0x27, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x2e, 0x76, 0x32, 0x2e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x44, 0x65,
    0x6c, 0x61, 0x79, 0x42, 0x1b, 0x0a, 0x14, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x64, 0x65, 0x6c,
    0x61, 0x79, 0x5f, 0x73, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x03, 0xf8, 0x42, 0x01,
    0x4a, 0x04, 0x08, 0x02, 0x10, 0x03, 0x4a, 0x04, 0x08, 0x01, 0x10, 0x02, 0x52, 0x04, 0x74, 0x79,
    0x70, 0x65, 0x22, 0xb0, 0x04, 0x0a, 0x0e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x52, 0x61, 0x74, 0x65,
    0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x66, 0x0a, 0x0b, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x43, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x2e, 0x76, 0x33, 0x2e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x52, 0x61, 0x74, 0x65, 0x4c,
    0x69, 0x6d, 0x69, 0x74, 0x2e, 0x46, 0x69, 0x78, 0x65, 0x64, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x48,
    0x00, 0x52, 0x0a, 0x66, 0x69, 0x78, 0x65, 0x64, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x69, 0x0a,
    0x0c, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x44, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65,
    0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x63,
    0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x2e, 0x76, 0x33, 0x2e, 0x46,
    0x61, 0x75, 0x6c, 0x74, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x48, 0x00, 0x52, 0x0b, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x40, 0x0a, 0x0a, 0x70, 0x65, 0x72, 0x63,
    0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x33, 0x2e, 0x46, 0x72, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x50, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x52, 0x0a,
    0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x1a, 0x73, 0x0a, 0x0a, 0x46, 0x69,
    0x78, 0x65, 0x64, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x26, 0x0a, 0x0a, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x5f, 0x6b, 0x62, 0x70, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x07, 0xfa, 0x42,
    0x04, 0x32, 0x02, 0x28, 0x01, 0x52, 0x09, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x4b, 0x62, 0x70, 0x73,
    0x3a, 0x3d, 0x9a, 0xc5, 0x88, 0x1e, 0x38, 0x0a, 0x36, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x2e, 0x76, 0x32, 0x2e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x52, 0x61, 0x74, 0x65, 0x4c,
    0x69, 0x6d, 0x69, 0x74, 0x2e, 0x46, 0x69, 0x78, 0x65, 0x64, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x1a,
    0x4d, 0x0a, 0x0b, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x3a, 0x3e,
    0x9a, 0xc5, 0x88, 0x1e, 0x39, 0x0a, 0x37, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x2e, 0x76, 0x32, 0x2e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d,
    0x69, 0x74, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x3a, 0x32,
    0x9a, 0xc5, 0x88, 0x1e, 0x2d, 0x0a, 0x2b, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x2e, 0x66, 0x61, 0x75, 0x6c, 0x74,
    0x2e, 0x76, 0x32, 0x2e, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d,
    0x69, 0x74, 0x42, 0x11, 0x0a, 0x0a, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x12, 0x03, 0xf8, 0x42, 0x01, 0x42, 0xa7, 0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02,
    0x0a, 0x36, 0x69, 0x6f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x2e, 0x76, 0x33, 0x42, 0x0a, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x57, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2f, 0x67, 0x6f,
    0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2f,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x2f, 0x76, 0x33, 0x3b, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x76, 0x33, 0x4a,
    0x8a, 0x14, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x60, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x31, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x06, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x08, 0x00, 0x27, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x09, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x0a, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x4f, 0x0a,
    0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0c, 0x00, 0x4f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x0d, 0x00, 0x2b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x2b, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12,
    0x03, 0x0e, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x6e, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0f, 0x00, 0x6e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x10, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x10,
    0x00, 0x46, 0x0a, 0xa5, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x17, 0x00, 0x3c, 0x01, 0x1a,
    0x66, 0x20, 0x44, 0x65, 0x6c, 0x61, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x20,
    0x69, 0x6e, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x4d,
    0x6f, 0x6e, 0x67, 0x6f, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x20, 0x5b, 0x23, 0x6e, 0x65, 0x78, 0x74, 0x2d, 0x66, 0x72, 0x65, 0x65, 0x2d, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x3a, 0x20, 0x36, 0x5d, 0x0a, 0x32, 0x31, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x43, 0x6f, 0x6d, 0x6d,
    0x6f, 0x6e, 0x20, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x17, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x18,
    0x02, 0x19, 0x30, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12,
    0x04, 0x18, 0x02, 0x19, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x1b,
    0x02, 0x1e, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x07,
    0x15, 0x0a, 0x27, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x04, 0x0e,
    0x1a, 0x18, 0x20, 0x55, 0x6e, 0x75, 0x73, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x64, 0x65,
    0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1d, 0x0c, 0x0d, 0x0a, 0xc9, 0x01, 0x0a, 0x04, 0x04,
    0x00, 0x03, 0x00, 0x12, 0x04, 0x23, 0x02, 0x26, 0x03, 0x1a, 0xba, 0x01, 0x20, 0x46, 0x61, 0x75,
    0x6c, 0x74, 0x20, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x20, 0x61, 0x6e, 0x20,
    0x48, 0x54, 0x54, 0x50, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x28, 0x69, 0x66, 0x20,
    0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x29, 0x2e, 0x20, 0x53, 0x65, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x48, 0x54, 0x54, 0x50,
    0x20, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x3c, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65,
    0x72, 0x73, 0x5f, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x3e, 0x60,
    0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x0a, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12,
    0x03, 0x23, 0x0a, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x07, 0x12, 0x04, 0x24,
    0x04, 0x25, 0x3e, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x00, 0x03, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03,
    0x01, 0x12, 0x04, 0x24, 0x04, 0x25, 0x3e, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x09, 0x12, 0x03,
    0x28, 0x02, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x09, 0x00, 0x12, 0x03, 0x28, 0x0b, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x09, 0x00, 0x01, 0x12, 0x03, 0x28, 0x0b, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x09, 0x00, 0x02, 0x12, 0x03, 0x28, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x09, 0x01, 0x12, 0x03, 0x28, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x09,
    0x01, 0x01, 0x12, 0x03, 0x28, 0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x09, 0x01, 0x02,
    0x12, 0x03, 0x28, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x0a, 0x12, 0x03, 0x2a, 0x02,
    0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x0a, 0x00, 0x12, 0x03, 0x2a, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x2c, 0x02, 0x38, 0x03, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x08, 0x00, 0x02, 0x12, 0x03, 0x2d, 0x04, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x08, 0x00,
    0x02, 0xaf, 0x08, 0x12, 0x03, 0x2d, 0x04, 0x26, 0x0a, 0xaf, 0x02, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x34, 0x04, 0x53, 0x1a, 0xa1, 0x02, 0x20, 0x41, 0x64, 0x64, 0x20, 0x61, 0x20,
    0x66, 0x69, 0x78, 0x65, 0x64, 0x20, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x20, 0x62, 0x65, 0x66, 0x6f,
    0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x75, 0x70, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x2e, 0x20, 0x53, 0x65, 0x65, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73,
    0x3a, 0x2f, 0x2f, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x73, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f,
    0x6c, 0x2d, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x73, 0x2f, 0x64, 0x6f, 0x63, 0x73, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33, 0x23, 0x6a, 0x73, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x2f, 0x59, 0x41, 0x4d, 0x4c, 0x20, 0x44, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x61, 0x70, 0x70, 0x69, 0x6e, 0x67, 0x2e, 0x20,
    0x46, 0x6f, 0x72, 0x20, 0x48, 0x54, 0x54, 0x50, 0x2f, 0x4d, 0x6f, 0x6e, 0x67, 0x6f, 0x2c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x0a, 0x20, 0x64,
    0x65, 0x6c, 0x61, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x6a,
    0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x61, 0x20, 0x6e,
    0x65, 0x77, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2f, 0x6f, 0x70, 0x65, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x46, 0x49, 0x58, 0x45, 0x44, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x34, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x34, 0x1d, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x34, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x34,
    0x2d, 0x52, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x15, 0x12, 0x03,
    0x34, 0x2e, 0x51, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x21,
    0x1a, 0x41, 0x20, 0x46, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x64, 0x65, 0x6c, 0x61, 0x79, 0x73, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x76,
    0x69, 0x61, 0x20, 0x61, 0x6e, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x20, 0x28, 0x69, 0x66, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65,
    0x29, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x37, 0x04,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x10, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x1f, 0x20, 0x0a, 0x65, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x3b, 0x02, 0x2b, 0x1a, 0x58, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e, 0x74, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x6f,
    0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x6c, 0x61,
    0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x3b,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x1c, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x29, 0x2a, 0x0a, 0x33,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x3f, 0x00, 0x60, 0x01, 0x1a, 0x27, 0x20, 0x44, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69,
    0x6d, 0x69, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x16, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12, 0x04, 0x40, 0x02, 0x41, 0x34, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x01, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12, 0x04, 0x40, 0x02, 0x41, 0x34, 0x0a, 0x36,
    0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12, 0x04, 0x44, 0x02, 0x4a, 0x03, 0x1a, 0x28, 0x20, 0x44,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x66, 0x69, 0x78, 0x65, 0x64,
    0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12,
    0x03, 0x44, 0x0a, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x07, 0x12, 0x04, 0x45,
    0x04, 0x46, 0x41, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x07, 0xd3, 0x88, 0xe1, 0x03,
    0x01, 0x12, 0x04, 0x45, 0x04, 0x46, 0x41, 0x0a, 0x2d, 0x0a, 0x06, 0x04, 0x01, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x49, 0x04, 0x3f, 0x1a, 0x1e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x4b,
    0x69, 0x42, 0x2f, 0x73, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x49, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x49, 0x0b, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x49, 0x18, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00,
    0x08, 0x12, 0x03, 0x49, 0x1a, 0x3e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x00, 0x02, 0x00,
    0x08, 0xaf, 0x08, 0x06, 0x12, 0x03, 0x49, 0x1b, 0x3d, 0x0a, 0xc8, 0x01, 0x0a, 0x04, 0x04, 0x01,
    0x03, 0x01, 0x12, 0x04, 0x4f, 0x02, 0x52, 0x03, 0x1a, 0xb9, 0x01, 0x20, 0x52, 0x61, 0x74, 0x65,
    0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x20, 0x61, 0x6e, 0x20, 0x48, 0x54,
    0x54, 0x50, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x28, 0x69, 0x66, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x29, 0x2e, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x48, 0x54, 0x54, 0x50, 0x20, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x20, 0x3c, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x5f, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x5f, 0x68, 0x74, 0x74, 0x70, 0x5f, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x3e, 0x60, 0x20, 0x64,
    0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72,
    0x0a, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x01, 0x01, 0x12, 0x03, 0x4f,
    0x0a, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x01, 0x07, 0x12, 0x04, 0x50, 0x04, 0x51,
    0x42, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x01, 0x03, 0x01, 0x07, 0xd3, 0x88, 0xe1, 0x03, 0x01, 0x12,
    0x04, 0x50, 0x04, 0x51, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x54,
    0x02, 0x5c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x54, 0x08,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x02, 0x12, 0x03, 0x55, 0x04, 0x26, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x08, 0x00, 0x02, 0xaf, 0x08, 0x12, 0x03, 0x55, 0x04, 0x26, 0x0a,
    0x22, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x58, 0x04, 0x1f, 0x1a, 0x15, 0x20, 0x41,
    0x20, 0x66, 0x69, 0x78, 0x65, 0x64, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x58, 0x04,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58, 0x0f, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x58, 0x1d, 0x1e, 0x0a, 0x4d, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x04, 0x21, 0x1a, 0x40, 0x20, 0x52, 0x61, 0x74,
    0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x20, 0x61, 0x6e, 0x20, 0x48,
    0x54, 0x54, 0x50, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x28, 0x69, 0x66, 0x20, 0x61,
    0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x29, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5b, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x5b, 0x1f, 0x20, 0x0a, 0x6a, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x5f, 0x02, 0x2b, 0x1a, 0x5d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x65, 0x72, 0x63, 0x65, 0x6e,
    0x74, 0x61, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x5f, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x1c, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5f, 0x29, 0x2a, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.extensions.filters.common.fault.v3.serde.rs");
// @@protoc_insertion_point(module)