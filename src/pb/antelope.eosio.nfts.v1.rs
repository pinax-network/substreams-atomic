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
    /// contract & scope
    #[prost(string, tag="3")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub action: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
