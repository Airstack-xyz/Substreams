// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowEvents {
    #[prost(message, repeated, tag="1")]
    pub follow_nft_deployed_events: ::prost::alloc::vec::Vec<FollowNftDeployed>,
    #[prost(message, repeated, tag="2")]
    pub follow_nft_transferred_events: ::prost::alloc::vec::Vec<FollowNftTransferred>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowNftDeployed {
    #[prost(string, tag="1")]
    pub follow_profile_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub follow_token_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub activity_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub log_index: u32,
    #[prost(uint64, tag="6")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowNftTransferred {
    #[prost(string, tag="1")]
    pub follow_profile_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub follow_token_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub follower_address: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub activity_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub follow_type: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub log_index: u32,
    #[prost(uint64, tag="8")]
    pub block_number: u64,
}
// @@protoc_insertion_point(module)
