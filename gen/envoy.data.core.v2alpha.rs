// @generated
/// [#next-free-field: 10]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckEvent {
    #[prost(enumeration = "HealthCheckerType", tag = "1")]
    pub health_checker_type: i32,
    #[prost(message, optional, tag = "2")]
    pub host: ::core::option::Option<super::super::super::api::v2::core::Address>,
    #[prost(string, tag = "3")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Timestamp for event.
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<::pbjson_types::Timestamp>,
    #[prost(oneof = "health_check_event::Event", tags = "4, 5, 7, 8, 9")]
    pub event: ::core::option::Option<health_check_event::Event>,
}
/// Nested message and enum types in `HealthCheckEvent`.
pub mod health_check_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// Host ejection.
        #[prost(message, tag = "4")]
        EjectUnhealthyEvent(super::HealthCheckEjectUnhealthy),
        /// Host addition.
        #[prost(message, tag = "5")]
        AddHealthyEvent(super::HealthCheckAddHealthy),
        /// Host failure.
        #[prost(message, tag = "7")]
        HealthCheckFailureEvent(super::HealthCheckFailure),
        /// Healthy host became degraded.
        #[prost(message, tag = "8")]
        DegradedHealthyHost(super::DegradedHealthyHost),
        /// A degraded host returned to being healthy.
        #[prost(message, tag = "9")]
        NoLongerDegradedHost(super::NoLongerDegradedHost),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckEjectUnhealthy {
    /// The type of failure that caused this ejection.
    #[prost(enumeration = "HealthCheckFailureType", tag = "1")]
    pub failure_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckAddHealthy {
    /// Whether this addition is the result of the first ever health check on a host, in which case
    /// the configured :ref:`healthy threshold <envoy_api_field_core.HealthCheck.healthy_threshold>`
    /// is bypassed and the host is immediately added.
    #[prost(bool, tag = "1")]
    pub first_check: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckFailure {
    /// The type of failure that caused this event.
    #[prost(enumeration = "HealthCheckFailureType", tag = "1")]
    pub failure_type: i32,
    /// Whether this event is the result of the first ever health check on a host.
    #[prost(bool, tag = "2")]
    pub first_check: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DegradedHealthyHost {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoLongerDegradedHost {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthCheckFailureType {
    Active = 0,
    Passive = 1,
    Network = 2,
}
impl HealthCheckFailureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HealthCheckFailureType::Active => "ACTIVE",
            HealthCheckFailureType::Passive => "PASSIVE",
            HealthCheckFailureType::Network => "NETWORK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTIVE" => Some(Self::Active),
            "PASSIVE" => Some(Self::Passive),
            "NETWORK" => Some(Self::Network),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthCheckerType {
    Http = 0,
    Tcp = 1,
    Grpc = 2,
    Redis = 3,
}
impl HealthCheckerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HealthCheckerType::Http => "HTTP",
            HealthCheckerType::Tcp => "TCP",
            HealthCheckerType::Grpc => "GRPC",
            HealthCheckerType::Redis => "REDIS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP" => Some(Self::Http),
            "TCP" => Some(Self::Tcp),
            "GRPC" => Some(Self::Grpc),
            "REDIS" => Some(Self::Redis),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `envoy.data.core.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xcf, 0x1e, 0x0a, 0x30, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f,
    0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x68, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x1f,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x32, 0x2f, 0x63, 0x6f, 0x72,
    0x65, 0x2f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x9c, 0x06, 0x0a, 0x10, 0x48, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x64, 0x0a,
    0x13, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x65, 0x72, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2a, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61,
    0x6c, 0x70, 0x68, 0x61, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x82, 0x01, 0x02, 0x10, 0x01,
    0x52, 0x11, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x65, 0x72, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x2e, 0x0a, 0x04, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1a, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x32,
    0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x52, 0x04, 0x68,
    0x6f, 0x73, 0x74, 0x12, 0x2a, 0x0a, 0x0c, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02,
    0x20, 0x01, 0x52, 0x0b, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12,
    0x68, 0x0a, 0x15, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x75, 0x6e, 0x68, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x79, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32,
    0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65,
    0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43,
    0x68, 0x65, 0x63, 0x6b, 0x45, 0x6a, 0x65, 0x63, 0x74, 0x55, 0x6e, 0x68, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x79, 0x48, 0x00, 0x52, 0x13, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x55, 0x6e, 0x68, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x5c, 0x0a, 0x11, 0x61, 0x64, 0x64,
    0x5f, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x48,
    0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x41, 0x64, 0x64, 0x48, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x79, 0x48, 0x00, 0x52, 0x0f, 0x61, 0x64, 0x64, 0x48, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x6a, 0x0a, 0x1a, 0x68, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63,
    0x6b, 0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x48, 0x00, 0x52, 0x17, 0x68, 0x65, 0x61, 0x6c,
    0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x12, 0x62, 0x0a, 0x15, 0x64, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x5f,
    0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x5f, 0x68, 0x6f, 0x73, 0x74, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x44, 0x65, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x64, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x48, 0x6f, 0x73, 0x74,
    0x48, 0x00, 0x52, 0x13, 0x64, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x48, 0x65, 0x61, 0x6c,
    0x74, 0x68, 0x79, 0x48, 0x6f, 0x73, 0x74, 0x12, 0x66, 0x0a, 0x17, 0x6e, 0x6f, 0x5f, 0x6c, 0x6f,
    0x6e, 0x67, 0x65, 0x72, 0x5f, 0x64, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x5f, 0x68, 0x6f,
    0x73, 0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x2e, 0x4e, 0x6f, 0x4c, 0x6f, 0x6e, 0x67, 0x65, 0x72, 0x44, 0x65, 0x67, 0x72, 0x61,
    0x64, 0x65, 0x64, 0x48, 0x6f, 0x73, 0x74, 0x48, 0x00, 0x52, 0x14, 0x6e, 0x6f, 0x4c, 0x6f, 0x6e,
    0x67, 0x65, 0x72, 0x44, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x48, 0x6f, 0x73, 0x74, 0x12,
    0x38, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x42, 0x0c, 0x0a, 0x05, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x12, 0x03, 0xf8, 0x42, 0x01, 0x22, 0x79, 0x0a, 0x19, 0x48, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x45, 0x6a, 0x65, 0x63, 0x74, 0x55, 0x6e, 0x68, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x79, 0x12, 0x5c, 0x0a, 0x0c, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2f, 0x2e, 0x65, 0x6e, 0x76,
    0x6f, 0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61,
    0x6c, 0x70, 0x68, 0x61, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x54, 0x79, 0x70, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05,
    0x82, 0x01, 0x02, 0x10, 0x01, 0x52, 0x0b, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x22, 0x38, 0x0a, 0x15, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63,
    0x6b, 0x41, 0x64, 0x64, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x66,
    0x69, 0x72, 0x73, 0x74, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08,
    0x52, 0x0a, 0x66, 0x69, 0x72, 0x73, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x22, 0x93, 0x01, 0x0a,
    0x12, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x46, 0x61, 0x69, 0x6c,
    0x75, 0x72, 0x65, 0x12, 0x5c, 0x0a, 0x0c, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2f, 0x2e, 0x65, 0x6e, 0x76, 0x6f,
    0x79, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c,
    0x70, 0x68, 0x61, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x46,
    0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x54, 0x79, 0x70, 0x65, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x82,
    0x01, 0x02, 0x10, 0x01, 0x52, 0x0b, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x66, 0x69, 0x72, 0x73, 0x74, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x66, 0x69, 0x72, 0x73, 0x74, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x22, 0x15, 0x0a, 0x13, 0x44, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x48, 0x65,
    0x61, 0x6c, 0x74, 0x68, 0x79, 0x48, 0x6f, 0x73, 0x74, 0x22, 0x16, 0x0a, 0x14, 0x4e, 0x6f, 0x4c,
    0x6f, 0x6e, 0x67, 0x65, 0x72, 0x44, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x48, 0x6f, 0x73,
    0x74, 0x2a, 0x3e, 0x0a, 0x16, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x46, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x41,
    0x43, 0x54, 0x49, 0x56, 0x45, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x50, 0x41, 0x53, 0x53, 0x49,
    0x56, 0x45, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x4e, 0x45, 0x54, 0x57, 0x4f, 0x52, 0x4b, 0x10,
    0x02, 0x2a, 0x3b, 0x0a, 0x11, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x65, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x54, 0x54, 0x50, 0x10, 0x00,
    0x12, 0x07, 0x0a, 0x03, 0x54, 0x43, 0x50, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x47, 0x52, 0x50,
    0x43, 0x10, 0x02, 0x12, 0x09, 0x0a, 0x05, 0x52, 0x45, 0x44, 0x49, 0x53, 0x10, 0x03, 0x42, 0x88,
    0x01, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x25, 0x69, 0x6f, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x64,
    0x61, 0x74, 0x61, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x42, 0x15, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x3e, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72, 0x6f, 0x78, 0x79,
    0x2f, 0x67, 0x6f, 0x2d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2d, 0x70, 0x6c, 0x61, 0x6e,
    0x65, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x63, 0x6f, 0x72,
    0x65, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x4a, 0xde, 0x11, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x58, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08,
    0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x04, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x06, 0x00, 0x29, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x08, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x09, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x3e, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x0b, 0x00, 0x3e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0c, 0x00, 0x36, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x36, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03,
    0x0d, 0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x55, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0b, 0x12, 0x03, 0x0e, 0x00, 0x55, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f,
    0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00,
    0x46, 0x0a, 0x7f, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x14, 0x00, 0x18, 0x01, 0x32, 0x73, 0x20,
    0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65,
    0x3a, 0x20, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x6c,
    0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x5d, 0x0a, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x20, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x3c, 0x61, 0x72, 0x63, 0x68,
    0x5f, 0x6f, 0x76, 0x65, 0x72, 0x76, 0x69, 0x65, 0x77, 0x5f, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x5f, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x3e, 0x60,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x14, 0x05, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x15, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x16, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16,
    0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x0c, 0x0d,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x17, 0x02, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x17, 0x0c, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x1a, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x05,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x09, 0x0a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x1c, 0x02, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x1c, 0x08, 0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x02, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x06, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x09, 0x0a, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x1e, 0x0a, 0x0b, 0x0a, 0x24, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x22, 0x00, 0x3e,
    0x01, 0x1a, 0x18, 0x20, 0x5b, 0x23, 0x6e, 0x65, 0x78, 0x74, 0x2d, 0x66, 0x72, 0x65, 0x65, 0x2d,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x3a, 0x20, 0x31, 0x30, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x22, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x23, 0x02, 0x5b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x23,
    0x02, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x14, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x2a, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x23, 0x2c, 0x5a, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x10, 0x12, 0x03, 0x23, 0x2d, 0x59, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x25, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x25, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x25, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x25, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x27, 0x02,
    0x45, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x27, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x09, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x27, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x27, 0x1a, 0x44, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x02, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x27, 0x1b, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00,
    0x08, 0x00, 0x12, 0x04, 0x29, 0x02, 0x3a, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00,
    0x01, 0x12, 0x03, 0x29, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x02, 0x12,
    0x03, 0x2a, 0x04, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x08, 0x00, 0x02, 0xaf, 0x08, 0x12,
    0x03, 0x2a, 0x04, 0x26, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x04,
    0x38, 0x1a, 0x10, 0x20, 0x48, 0x6f, 0x73, 0x74, 0x20, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2d, 0x04,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2d, 0x1e, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2d, 0x36, 0x37, 0x0a, 0x1d, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x30, 0x04, 0x30, 0x1a, 0x10, 0x20, 0x48, 0x6f, 0x73,
    0x74, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x30, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x30, 0x1a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x30, 0x2e, 0x2f, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x33, 0x04, 0x36, 0x1a, 0x0f, 0x20, 0x48, 0x6f, 0x73, 0x74, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75,
    0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x33,
    0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x33, 0x17, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x33, 0x34, 0x35, 0x0a, 0x2c,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x36, 0x04, 0x32, 0x1a, 0x1f, 0x20, 0x48, 0x65,
    0x61, 0x6c, 0x74, 0x68, 0x79, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x62, 0x65, 0x63, 0x61, 0x6d,
    0x65, 0x20, 0x64, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x36, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x36, 0x18, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x03, 0x12, 0x03, 0x36, 0x30, 0x31, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03,
    0x39, 0x04, 0x35, 0x1a, 0x2c, 0x20, 0x41, 0x20, 0x64, 0x65, 0x67, 0x72, 0x61, 0x64, 0x65, 0x64,
    0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x39, 0x04, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x39, 0x19, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x39, 0x33, 0x34, 0x0a, 0x23, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x08, 0x12, 0x03, 0x3d, 0x02, 0x2a, 0x1a, 0x16, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x3d, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x3d, 0x1c, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x03, 0x12, 0x03, 0x3d, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x40, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x40,
    0x08, 0x21, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x42, 0x02, 0x59, 0x1a,
    0x30, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x61,
    0x69, 0x6c, 0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x42, 0x02, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x19, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x08, 0x12, 0x03, 0x42, 0x2a, 0x58, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02,
    0x00, 0x08, 0xaf, 0x08, 0x10, 0x12, 0x03, 0x42, 0x2b, 0x57, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x45, 0x00, 0x4a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x45,
    0x08, 0x1d, 0x0a, 0xf9, 0x01, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x49, 0x02, 0x17,
    0x1a, 0xeb, 0x01, 0x20, 0x57, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x69, 0x72, 0x73, 0x74, 0x20, 0x65, 0x76, 0x65, 0x72, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x68, 0x6f, 0x73, 0x74,
    0x2c, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x61, 0x73, 0x65, 0x0a,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x20, 0x74, 0x68,
    0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61,
    0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x48, 0x65,
    0x61, 0x6c, 0x74, 0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x2e, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x79, 0x5f, 0x74, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x3e, 0x60, 0x0a, 0x20, 0x69,
    0x73, 0x20, 0x62, 0x79, 0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6d, 0x6d, 0x65, 0x64,
    0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x49, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49, 0x07, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x4c, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x1a,
    0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x02, 0x59, 0x1a, 0x2d, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x61, 0x69, 0x6c,
    0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4e, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x19, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x4e, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12,
    0x03, 0x4e, 0x2a, 0x58, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x10,
    0x12, 0x03, 0x4e, 0x2b, 0x57, 0x0a, 0x59, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x51,
    0x02, 0x17, 0x1a, 0x4c, 0x20, 0x57, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72,
    0x73, 0x74, 0x20, 0x65, 0x76, 0x65, 0x72, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63,
    0x68, 0x65, 0x63, 0x6b, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x51, 0x02, 0x06, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x51, 0x07, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x51, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x54, 0x00, 0x55, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x54,
    0x08, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x57, 0x00, 0x58, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x57, 0x08, 0x1c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33,
];
include!("envoy.data.core.v2alpha.serde.rs");
// @@protoc_insertion_point(module)