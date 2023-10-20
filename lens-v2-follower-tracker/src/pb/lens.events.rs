// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowEvents {
    #[prost(message, repeated, tag="1")]
    pub follow_events: ::prost::alloc::vec::Vec<Follow>,
    #[prost(message, repeated, tag="2")]
    pub unfollow_events: ::prost::alloc::vec::Vec<Unfollow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Follow {
    #[prost(string, tag="1")]
    pub follower_profile_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id_of_profile_followed: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub follow_token_id_assigned: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub follow_module_data: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub process_follow_module_return_data: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub transaction_executor: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub timestamp: u64,
    #[prost(string, tag="8")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub log_index: ::prost::alloc::string::String,
    #[prost(uint64, tag="10")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unfollow {
    #[prost(string, tag="1")]
    pub unfollower_profile_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id_of_profile_unfollowed: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub transaction_executor: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
    #[prost(string, tag="5")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub log_index: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub block_number: u64,
}
// @@protoc_insertion_point(module)
