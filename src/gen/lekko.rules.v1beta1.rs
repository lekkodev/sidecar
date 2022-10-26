// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraint {
    #[prost(message, repeated, tag="1")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// For now, we will only allow one condition linker that applies
    /// to all sets of conditions where len(conditions) > 1. This will be
    /// unset when len(conditions) == 1.
    #[prost(enumeration="ConditionLinker", tag="2")]
    pub condition_linker: i32,
    #[prost(message, optional, tag="3")]
    pub resulting_value: ::core::option::Option<::prost_types::Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    #[prost(string, tag="1")]
    pub context_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub comparison_value: ::core::option::Option<::prost_types::Value>,
    /// For operators, context is on the left, comparison value on the right.
    #[prost(enumeration="LogicalOperator", tag="3")]
    pub logical_operator: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="Type", tag="2")]
    pub r#type: i32,
    #[prost(message, optional, tag="3")]
    pub default_value: ::core::option::Option<::prost_types::Value>,
    #[prost(message, repeated, tag="4")]
    pub constraints: ::prost::alloc::vec::Vec<Constraint>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    Unspecified = 0,
    Bool = 1,
    Number = 2,
    String = 3,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::Bool => "TYPE_BOOL",
            Type::Number => "TYPE_NUMBER",
            Type::String => "TYPE_STRING",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogicalOperator {
    Unspecified = 0,
    Equals = 1,
    /// > < >= <= only applies to number values.
    LessThan = 2,
    LessThanOrEquals = 3,
    GreaterThan = 4,
    GreaterThanOrEquals = 5,
    /// Contained within or not contained within only applies to list values.
    ContainedWithin = 6,
    NotContainedWithin = 7,
}
impl LogicalOperator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LogicalOperator::Unspecified => "LOGICAL_OPERATOR_UNSPECIFIED",
            LogicalOperator::Equals => "LOGICAL_OPERATOR_EQUALS",
            LogicalOperator::LessThan => "LOGICAL_OPERATOR_LESS_THAN",
            LogicalOperator::LessThanOrEquals => "LOGICAL_OPERATOR_LESS_THAN_OR_EQUALS",
            LogicalOperator::GreaterThan => "LOGICAL_OPERATOR_GREATER_THAN",
            LogicalOperator::GreaterThanOrEquals => "LOGICAL_OPERATOR_GREATER_THAN_OR_EQUALS",
            LogicalOperator::ContainedWithin => "LOGICAL_OPERATOR_CONTAINED_WITHIN",
            LogicalOperator::NotContainedWithin => "LOGICAL_OPERATOR_NOT_CONTAINED_WITHIN",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConditionLinker {
    Unspecified = 0,
    And = 1,
    Or = 2,
}
impl ConditionLinker {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConditionLinker::Unspecified => "CONDITION_LINKER_UNSPECIFIED",
            ConditionLinker::And => "CONDITION_LINKER_AND",
            ConditionLinker::Or => "CONDITION_LINKER_OR",
        }
    }
}
/// Encoded file descriptor set for the `lekko.rules.v1beta1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd2, 0x1b, 0x0a, 0x1f, 0x6c, 0x65, 0x6b, 0x6b, 0x6f, 0x2f, 0x72, 0x75, 0x6c, 0x65, 0x73,
    0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x6c, 0x65, 0x6b, 0x6b, 0x6f, 0x2e, 0x72, 0x75, 0x6c, 0x65,
    0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xde, 0x01, 0x0a, 0x0a, 0x43, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x61, 0x69, 0x6e, 0x74, 0x12, 0x3e, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x6c, 0x65, 0x6b,
    0x6b, 0x6f, 0x2e, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
    0x2e, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x4f, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x6c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x24, 0x2e, 0x6c, 0x65, 0x6b, 0x6b, 0x6f, 0x2e, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x2e, 0x76,
    0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x4c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x52, 0x0f, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x4c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x12, 0x3f, 0x0a, 0x0f, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x69, 0x6e, 0x67, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x0e, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x69, 0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xc0, 0x01, 0x0a, 0x09, 0x43, 0x6f, 0x6e,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x78, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x41, 0x0a, 0x10, 0x63, 0x6f, 0x6d, 0x70, 0x61,
    0x72, 0x69, 0x73, 0x6f, 0x6e, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x0f, 0x63, 0x6f, 0x6d, 0x70, 0x61,
    0x72, 0x69, 0x73, 0x6f, 0x6e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x4f, 0x0a, 0x10, 0x6c, 0x6f,
    0x67, 0x69, 0x63, 0x61, 0x6c, 0x5f, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x24, 0x2e, 0x6c, 0x65, 0x6b, 0x6b, 0x6f, 0x2e, 0x72, 0x75, 0x6c,
    0x65, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x69, 0x63,
    0x61, 0x6c, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x52, 0x0f, 0x6c, 0x6f, 0x67, 0x69,
    0x63, 0x61, 0x6c, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x22, 0xcc, 0x01, 0x0a, 0x07,
    0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x6c, 0x65, 0x6b, 0x6b,
    0x6f, 0x2e, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x3b, 0x0a, 0x0d, 0x64, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x0c, 0x64, 0x65, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x41, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x73, 0x74,
    0x72, 0x61, 0x69, 0x6e, 0x74, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x6c,
    0x65, 0x6b, 0x6b, 0x6f, 0x2e, 0x72, 0x75, 0x6c, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74,
    0x61, 0x31, 0x2e, 0x43, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x74, 0x52, 0x0b, 0x63,
    0x6f, 0x6e, 0x73, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x74, 0x73, 0x2a, 0x4d, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x14, 0x0a, 0x10, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45,
    0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09, 0x54, 0x59, 0x50, 0x45,
    0x5f, 0x42, 0x4f, 0x4f, 0x4c, 0x10, 0x01, 0x12, 0x0f, 0x0a, 0x0b, 0x54, 0x59, 0x50, 0x45, 0x5f,
    0x4e, 0x55, 0x4d, 0x42, 0x45, 0x52, 0x10, 0x02, 0x12, 0x0f, 0x0a, 0x0b, 0x54, 0x59, 0x50, 0x45,
    0x5f, 0x53, 0x54, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x03, 0x2a, 0xbc, 0x02, 0x0a, 0x0f, 0x4c, 0x6f,
    0x67, 0x69, 0x63, 0x61, 0x6c, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x12, 0x20, 0x0a,
    0x1c, 0x4c, 0x4f, 0x47, 0x49, 0x43, 0x41, 0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x4f,
    0x52, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12,
    0x1b, 0x0a, 0x17, 0x4c, 0x4f, 0x47, 0x49, 0x43, 0x41, 0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41,
    0x54, 0x4f, 0x52, 0x5f, 0x45, 0x51, 0x55, 0x41, 0x4c, 0x53, 0x10, 0x01, 0x12, 0x1e, 0x0a, 0x1a,
    0x4c, 0x4f, 0x47, 0x49, 0x43, 0x41, 0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x4f, 0x52,
    0x5f, 0x4c, 0x45, 0x53, 0x53, 0x5f, 0x54, 0x48, 0x41, 0x4e, 0x10, 0x02, 0x12, 0x28, 0x0a, 0x24,
    0x4c, 0x4f, 0x47, 0x49, 0x43, 0x41, 0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x4f, 0x52,
    0x5f, 0x4c, 0x45, 0x53, 0x53, 0x5f, 0x54, 0x48, 0x41, 0x4e, 0x5f, 0x4f, 0x52, 0x5f, 0x45, 0x51,
    0x55, 0x41, 0x4c, 0x53, 0x10, 0x03, 0x12, 0x21, 0x0a, 0x1d, 0x4c, 0x4f, 0x47, 0x49, 0x43, 0x41,
    0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x47, 0x52, 0x45, 0x41, 0x54,
    0x45, 0x52, 0x5f, 0x54, 0x48, 0x41, 0x4e, 0x10, 0x04, 0x12, 0x2b, 0x0a, 0x27, 0x4c, 0x4f, 0x47,
    0x49, 0x43, 0x41, 0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x47, 0x52,
    0x45, 0x41, 0x54, 0x45, 0x52, 0x5f, 0x54, 0x48, 0x41, 0x4e, 0x5f, 0x4f, 0x52, 0x5f, 0x45, 0x51,
    0x55, 0x41, 0x4c, 0x53, 0x10, 0x05, 0x12, 0x25, 0x0a, 0x21, 0x4c, 0x4f, 0x47, 0x49, 0x43, 0x41,
    0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x4f, 0x52, 0x5f, 0x43, 0x4f, 0x4e, 0x54, 0x41,
    0x49, 0x4e, 0x45, 0x44, 0x5f, 0x57, 0x49, 0x54, 0x48, 0x49, 0x4e, 0x10, 0x06, 0x12, 0x29, 0x0a,
    0x25, 0x4c, 0x4f, 0x47, 0x49, 0x43, 0x41, 0x4c, 0x5f, 0x4f, 0x50, 0x45, 0x52, 0x41, 0x54, 0x4f,
    0x52, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x43, 0x4f, 0x4e, 0x54, 0x41, 0x49, 0x4e, 0x45, 0x44, 0x5f,
    0x57, 0x49, 0x54, 0x48, 0x49, 0x4e, 0x10, 0x07, 0x2a, 0x66, 0x0a, 0x0f, 0x43, 0x6f, 0x6e, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x69, 0x6e, 0x6b, 0x65, 0x72, 0x12, 0x20, 0x0a, 0x1c, 0x43,
    0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4c, 0x49, 0x4e, 0x4b, 0x45, 0x52, 0x5f,
    0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x18, 0x0a,
    0x14, 0x43, 0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4c, 0x49, 0x4e, 0x4b, 0x45,
    0x52, 0x5f, 0x41, 0x4e, 0x44, 0x10, 0x01, 0x12, 0x17, 0x0a, 0x13, 0x43, 0x4f, 0x4e, 0x44, 0x49,
    0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4c, 0x49, 0x4e, 0x4b, 0x45, 0x52, 0x5f, 0x4f, 0x52, 0x10, 0x02,
    0x4a, 0x8a, 0x12, 0x0a, 0x06, 0x12, 0x04, 0x0e, 0x00, 0x43, 0x01, 0x0a, 0xcb, 0x04, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x0e, 0x00, 0x12, 0x32, 0xc0, 0x04, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69,
    0x67, 0x68, 0x74, 0x20, 0x32, 0x30, 0x32, 0x32, 0x20, 0x4c, 0x65, 0x6b, 0x6b, 0x6f, 0x20, 0x54,
    0x65, 0x63, 0x68, 0x6e, 0x6f, 0x6c, 0x6f, 0x67, 0x69, 0x65, 0x73, 0x2c, 0x20, 0x49, 0x6e, 0x63,
    0x2e, 0x0a, 0x0a, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64,
    0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x4c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x32,
    0x2e, 0x30, 0x20, 0x28, 0x74, 0x68, 0x65, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65,
    0x22, 0x29, 0x3b, 0x0a, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x65,
    0x78, 0x63, 0x65, 0x70, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x69, 0x61,
    0x6e, 0x63, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63,
    0x65, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6f,
    0x62, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x74, 0x0a, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77,
    0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x6f, 0x72, 0x67, 0x2f, 0x6c, 0x69, 0x63, 0x65,
    0x6e, 0x73, 0x65, 0x73, 0x2f, 0x4c, 0x49, 0x43, 0x45, 0x4e, 0x53, 0x45, 0x2d, 0x32, 0x2e, 0x30,
    0x0a, 0x0a, 0x20, 0x55, 0x6e, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x6c, 0x61, 0x77, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x72, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x73, 0x6f,
    0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69,
    0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x22, 0x41, 0x53, 0x20, 0x49,
    0x53, 0x22, 0x20, 0x42, 0x41, 0x53, 0x49, 0x53, 0x2c, 0x0a, 0x20, 0x57, 0x49, 0x54, 0x48, 0x4f,
    0x55, 0x54, 0x20, 0x57, 0x41, 0x52, 0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x20, 0x4f, 0x52,
    0x20, 0x43, 0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49, 0x4f, 0x4e, 0x53, 0x20, 0x4f, 0x46, 0x20, 0x41,
    0x4e, 0x59, 0x20, 0x4b, 0x49, 0x4e, 0x44, 0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20,
    0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69,
    0x65, 0x64, 0x2e, 0x0a, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63,
    0x65, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x67,
    0x6f, 0x76, 0x65, 0x72, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x10, 0x00, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x12, 0x00, 0x26, 0x0a, 0x0a,
    0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x14, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00,
    0x01, 0x12, 0x03, 0x14, 0x05, 0x09, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x15, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x02,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x15, 0x15, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x17, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x17, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x17, 0x10,
    0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x18, 0x02, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x18, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x18, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x1b, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x1b,
    0x05, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1d, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x1d, 0x1c, 0x1d, 0x0a, 0x37, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x02,
    0x21, 0x1a, 0x2a, 0x20, 0x3e, 0x20, 0x3c, 0x20, 0x3e, 0x3d, 0x20, 0x3c, 0x3d, 0x20, 0x6f, 0x6e,
    0x6c, 0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x03, 0x12, 0x03, 0x20, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x20, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x20,
    0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x21, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x21, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x21, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x05, 0x12, 0x03, 0x22, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x22, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x02, 0x12,
    0x03, 0x22, 0x2c, 0x2d, 0x0a, 0x54, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12, 0x03, 0x24, 0x02,
    0x28, 0x1a, 0x47, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x69, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x61, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x69, 0x73,
    0x74, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x24, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x06,
    0x02, 0x12, 0x03, 0x24, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x07, 0x12, 0x03,
    0x25, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x01, 0x12, 0x03, 0x25, 0x02,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x02, 0x12, 0x03, 0x25, 0x2a, 0x2b, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x28, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x28, 0x05, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x29, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x29, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x2a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x2b, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x2b, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2b,
    0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x2e, 0x00, 0x35, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x2f, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x2f, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x15,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x22, 0x23, 0x0a,
    0xb2, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x33, 0x02, 0x27, 0x1a, 0xa4, 0x01,
    0x20, 0x46, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x77, 0x2c, 0x20, 0x77, 0x65, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x6f, 0x6e, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6c, 0x69, 0x6e, 0x6b, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x0a, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x74, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x63,
    0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x6c, 0x65, 0x6e, 0x28, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x29, 0x20,
    0x3e, 0x20, 0x31, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62,
    0x65, 0x0a, 0x20, 0x75, 0x6e, 0x73, 0x65, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x6c, 0x65,
    0x6e, 0x28, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x29, 0x20, 0x3d, 0x3d,
    0x20, 0x31, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x33,
    0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x33, 0x12, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33, 0x25, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x34, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x34, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x34, 0x18, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x34, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x37, 0x00, 0x3c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x37, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x38, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x38, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02,
    0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x39, 0x02, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x18, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x2b, 0x2c, 0x0a, 0x54, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x3b, 0x02, 0x27, 0x1a, 0x47, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x66, 0x74,
    0x2c, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x72, 0x69, 0x73, 0x6f, 0x6e, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x3b, 0x02, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x3e, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x3e, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3f, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x40, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x40, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x40, 0x07, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x40, 0x0e,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x41, 0x02, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x41, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x41, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x41, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x42, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x42, 0x0b,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x42, 0x16, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x42, 0x24, 0x25, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)