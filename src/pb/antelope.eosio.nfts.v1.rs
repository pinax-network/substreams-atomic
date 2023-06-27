// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<TransferEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    // contract & scope
    // string contract = 3;
    // string action = 4;

    /// data payload
    #[prost(string, tag="5")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub to: ::prost::alloc::string::String,
    /// string memo = 9;
    #[prost(uint64, repeated, tag="8")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaEvents {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SchemaEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaEvent {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="2")]
    pub authorized_creator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub schema_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub schema_format: ::prost::alloc::vec::Vec<Format>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Format {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub dtype: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
