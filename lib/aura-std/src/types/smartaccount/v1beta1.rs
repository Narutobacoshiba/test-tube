use aura_std_derive::CosmwasmExt;

/// Params defines the parameters for the smartaccount module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.smartaccount.v1beta1.CodeID")]
pub struct CodeID {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,

    #[prost(bool, tag = "2")]
    pub status: bool,
}

/// Params defines the parameters for the smartaccount module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.smartaccount.v1beta1.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub whitelist_code_id: ::prost::alloc::vec::Vec<CodeID>,

    #[prost(string, repeated, tag = "2")]
    pub disable_msgs_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,

    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_gas_execute: u64,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/aura.smartaccount.v1beta1.MsgActivateAccount")]
pub struct MsgActivateAccountRequest {
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,

    #[prost(bytes = "vec", tag = "2")]
    pub salt: ::prost::alloc::vec::Vec<u8>,

    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,

    #[prost(message, optional, tag = "4")]
    pub pub_key: ::core::option::Option<crate::shim::Any>,

    #[prost(bytes = "vec", tag = "5")]
    pub init_msg: ::prost::alloc::vec::Vec<u8>
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgActivateAccountResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/aura.smartaccount.v1beta1.MsgRecover")]
pub struct MsgRecoverRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,

    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,

    #[prost(message, optional, tag = "3")]
    pub public_key: ::core::option::Option<crate::shim::Any>,

    #[prost(string, tag = "4")]
    pub credentials: ::prost::alloc::string::String
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRecoverResponse {}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/aura.smartaccount.v1beta1.QueryGenerateAccountRequest")]
#[proto_query(
    path = "/aura.smartaccount.v1beta1.Query/GenerateAccount",
    response_type = QueryGenerateAccountResponse
)]
pub struct QueryGenerateAccountRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_id: u64,

    #[prost(bytes = "vec", tag = "2")]
    pub salt: ::prost::alloc::vec::Vec<u8>,

    #[prost(bytes = "vec", tag = "3")]
    pub init_msg: ::prost::alloc::vec::Vec<u8>,

    #[prost(message, optional, tag = "4")]
    pub public_key: ::core::option::Option<crate::shim::Any>
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/aura.smartaccount.v1beta1.QueryGenerateAccountResponse")]
pub struct QueryGenerateAccountResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/aura.smartaccount.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/aura.smartaccount.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/aura.smartaccount.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}