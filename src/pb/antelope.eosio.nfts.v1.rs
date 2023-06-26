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
    #[prost(uint64, repeated, tag="8")]
    pub asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(string, tag="9")]
    pub memo: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
