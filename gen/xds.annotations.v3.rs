// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileStatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageStatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldStatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusAnnotation {
    /// The entity is work-in-progress and subject to breaking changes.
    #[prost(bool, tag = "1")]
    pub work_in_progress: bool,
    /// The entity belongs to a package with the given version status.
    #[prost(enumeration = "PackageVersionStatus", tag = "2")]
    pub package_version_status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PackageVersionStatus {
    /// Unknown package version status.
    Unknown = 0,
    /// This version of the package is frozen.
    Frozen = 1,
    /// This version of the package is the active development version.
    Active = 2,
    /// This version of the package is the candidate for the next major version. It
    /// is typically machine generated from the active development version.
    NextMajorVersionCandidate = 3,
}
impl PackageVersionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PackageVersionStatus::Unknown => "UNKNOWN",
            PackageVersionStatus::Frozen => "FROZEN",
            PackageVersionStatus::Active => "ACTIVE",
            PackageVersionStatus::NextMajorVersionCandidate => {
                "NEXT_MAJOR_VERSION_CANDIDATE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "FROZEN" => Some(Self::Frozen),
            "ACTIVE" => Some(Self::Active),
            "NEXT_MAJOR_VERSION_CANDIDATE" => Some(Self::NextMajorVersionCandidate),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `xds.annotations.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc1, 0x13, 0x0a, 0x1f, 0x78, 0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x76, 0x33, 0x2f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x78, 0x64, 0x73, 0x2e, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x33, 0x1a, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x40, 0x0a, 0x14, 0x46, 0x69,
    0x6c, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x6e, 0x5f, 0x70, 0x72,
    0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x77, 0x6f,
    0x72, 0x6b, 0x49, 0x6e, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x22, 0x43, 0x0a, 0x17,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x41, 0x6e, 0x6e,
    0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x77, 0x6f, 0x72, 0x6b, 0x5f,
    0x69, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x08, 0x52, 0x0e, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x6e, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73,
    0x73, 0x22, 0x41, 0x0a, 0x15, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x69, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x6e, 0x50, 0x72, 0x6f, 0x67,
    0x72, 0x65, 0x73, 0x73, 0x22, 0x9c, 0x01, 0x0a, 0x10, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x41,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x77, 0x6f, 0x72,
    0x6b, 0x5f, 0x69, 0x6e, 0x5f, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x08, 0x52, 0x0e, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x6e, 0x50, 0x72, 0x6f, 0x67, 0x72,
    0x65, 0x73, 0x73, 0x12, 0x5e, 0x0a, 0x16, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x5f, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x28, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x14, 0x70,
    0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x2a, 0x5d, 0x0a, 0x14, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0b, 0x0a, 0x07, 0x55,
    0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x46, 0x52, 0x4f, 0x5a,
    0x45, 0x4e, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x41, 0x43, 0x54, 0x49, 0x56, 0x45, 0x10, 0x02,
    0x12, 0x20, 0x0a, 0x1c, 0x4e, 0x45, 0x58, 0x54, 0x5f, 0x4d, 0x41, 0x4a, 0x4f, 0x52, 0x5f, 0x56,
    0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x43, 0x41, 0x4e, 0x44, 0x49, 0x44, 0x41, 0x54, 0x45,
    0x10, 0x03, 0x3a, 0x6a, 0x0a, 0x0b, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x12, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18,
    0xea, 0xc8, 0x94, 0x6c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x61,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x46, 0x69,
    0x6c, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x41, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x0a, 0x66, 0x69, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x3a, 0x76,
    0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x12, 0x1f, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0xea, 0xc8, 0x94, 0x6c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x78, 0x64, 0x73,
    0x2e, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x33, 0x2e,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x41, 0x6e, 0x6e,
    0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x3a, 0x6e, 0x0a, 0x0c, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x1d, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4f, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0xea, 0xc8, 0x94, 0x6c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29,
    0x2e, 0x78, 0x64, 0x73, 0x2e, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x76, 0x33, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x41,
    0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0b, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x42, 0x2b, 0x5a, 0x29, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6e, 0x63, 0x66, 0x2f, 0x78, 0x64, 0x73, 0x2f, 0x67, 0x6f,
    0x2f, 0x78, 0x64, 0x73, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x2f, 0x76, 0x33, 0x4a, 0x96, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3a, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2a, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x40, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x06,
    0x00, 0x40, 0x0a, 0x6f, 0x0a, 0x01, 0x07, 0x12, 0x04, 0x0a, 0x00, 0x0c, 0x01, 0x1a, 0x64, 0x20,
    0x4d, 0x61, 0x67, 0x69, 0x63, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65,
    0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x6f, 0x70, 0x20, 0x32, 0x38, 0x62, 0x69, 0x74,
    0x20, 0x6f, 0x66, 0x20, 0x53, 0x48, 0x41, 0x32, 0x35, 0x36, 0x20, 0x64, 0x69, 0x67, 0x65, 0x73,
    0x74, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x22, 0x78, 0x64, 0x73, 0x2e, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x76, 0x33, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x22, 0x2e, 0x0a, 0x0a, 0x09, 0x0a, 0x02, 0x07, 0x00, 0x12, 0x03, 0x0b, 0x02, 0x2f, 0x0a, 0x0a,
    0x0a, 0x03, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0a, 0x07, 0x22, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00,
    0x06, 0x12, 0x03, 0x0b, 0x02, 0x16, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x17, 0x22, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x25, 0x2e, 0x0a, 0x09,
    0x0a, 0x01, 0x07, 0x12, 0x04, 0x0e, 0x00, 0x10, 0x01, 0x0a, 0x09, 0x0a, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x0f, 0x02, 0x35, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x02, 0x12, 0x03, 0x0e, 0x07, 0x25,
    0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x06, 0x12, 0x03, 0x0f, 0x02, 0x19, 0x0a, 0x0a, 0x0a, 0x03,
    0x07, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x1a, 0x28, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x03, 0x12,
    0x03, 0x0f, 0x2b, 0x34, 0x0a, 0x09, 0x0a, 0x01, 0x07, 0x12, 0x04, 0x12, 0x00, 0x14, 0x01, 0x0a,
    0x09, 0x0a, 0x02, 0x07, 0x02, 0x12, 0x03, 0x13, 0x02, 0x31, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x02,
    0x02, 0x12, 0x03, 0x12, 0x07, 0x23, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x02, 0x06, 0x12, 0x03, 0x13,
    0x02, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x02, 0x01, 0x12, 0x03, 0x13, 0x18, 0x24, 0x0a, 0x0a,
    0x0a, 0x03, 0x07, 0x02, 0x03, 0x12, 0x03, 0x13, 0x27, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x16, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x16,
    0x08, 0x1c, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x18, 0x02, 0x1c, 0x1a,
    0x41, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x20, 0x69, 0x73, 0x20,
    0x77, 0x6f, 0x72, 0x6b, 0x2d, 0x69, 0x6e, 0x2d, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x72, 0x65, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x02, 0x06,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x07, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x1b, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x1b, 0x08, 0x1f, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02,
    0x1c, 0x1a, 0x41, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x20, 0x69,
    0x73, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x2d, 0x69, 0x6e, 0x2d, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x65,
    0x73, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x72, 0x65, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d,
    0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x1a, 0x1b, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x20, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x20, 0x08, 0x1d, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x22, 0x02, 0x1c, 0x1a, 0x41, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x20, 0x69, 0x73, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x2d, 0x69, 0x6e, 0x2d, 0x70, 0x72, 0x6f, 0x67,
    0x72, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x72, 0x65, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x22, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22,
    0x07, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x1a, 0x1b,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x25, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x00, 0x01, 0x12, 0x03, 0x25, 0x05, 0x19, 0x0a, 0x2e, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x27, 0x02, 0x0e, 0x1a, 0x21, 0x20, 0x55, 0x6e, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20,
    0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x27, 0x02, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x27, 0x0c, 0x0d, 0x0a, 0x35, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02,
    0x0d, 0x1a, 0x28, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x7a, 0x65, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x2a, 0x0b, 0x0c, 0x0a, 0x4d, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x2d, 0x02, 0x0d, 0x1a, 0x40, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61,
    0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65,
    0x20, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x2d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2d,
    0x0b, 0x0c, 0x0a, 0xa0, 0x01, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x31, 0x02, 0x23,
    0x1a, 0x92, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x64, 0x69, 0x64, 0x61, 0x74, 0x65,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x6d, 0x61,
    0x6a, 0x6f, 0x72, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x49, 0x74, 0x0a,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x79, 0x70, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x6d, 0x61,
    0x63, 0x68, 0x69, 0x6e, 0x65, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20,
    0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x31, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x31, 0x21,
    0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x34, 0x00, 0x3a, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x34, 0x08, 0x18, 0x0a, 0x4e, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x00, 0x12, 0x03, 0x36, 0x02, 0x1c, 0x1a, 0x41, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74,
    0x69, 0x74, 0x79, 0x20, 0x69, 0x73, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x2d, 0x69, 0x6e, 0x2d, 0x70,
    0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x75, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x72, 0x65, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x20,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x36, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x36, 0x07, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x36, 0x1a, 0x1b, 0x0a, 0x4d, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02, 0x32,
    0x1a, 0x40, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x20, 0x62, 0x65,
    0x6c, 0x6f, 0x6e, 0x67, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61,
    0x67, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65,
    0x6e, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x39, 0x02, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x17, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x30, 0x31, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("xds.annotations.v3.serde.rs");
// @@protoc_insertion_point(module)