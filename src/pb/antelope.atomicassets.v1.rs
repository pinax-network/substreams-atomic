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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schemas {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Schema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// database operation
    #[prost(string, tag="3")]
    pub action: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="4")]
    pub schema_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub format: ::prost::alloc::vec::Vec<Format>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collections {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Collection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// database operation
    #[prost(string, tag="3")]
    pub action: ::prost::alloc::string::String,
    /// data payload
    #[prost(string, tag="4")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub author: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub allow_notify: bool,
    #[prost(string, repeated, tag="7")]
    pub authorized_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="8")]
    pub notify_accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// repeated uint32 serialized_data = 10;
    #[prost(double, tag="9")]
    pub market_fee: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Templates {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Template>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Template {
    /// trace information
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub action_index: u32,
    /// databas operation
    #[prost(string, tag="3")]
    pub action: ::prost::alloc::string::String,
    /// data payload
    #[prost(int32, tag="4")]
    pub template_id: i32,
    #[prost(string, tag="5")]
    pub schema_name: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub transferable: bool,
    #[prost(bool, tag="7")]
    pub burnable: bool,
    #[prost(uint32, tag="8")]
    pub max_supply: u32,
    #[prost(uint32, tag="9")]
    pub issued_supply: u32,
}
// @@protoc_insertion_point(module)
