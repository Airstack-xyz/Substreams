// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowNftDeployedMessage {
    #[prost(message, repeated, tag="1")]
    pub follow_nft_deployed_events: ::prost::alloc::vec::Vec<FollowNftDeployed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FollowNftDeployed {
    #[prost(uint64, tag="1")]
    pub profile_id: u64,
    #[prost(string, tag="2")]
    pub follow_nft: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub timestamp: u64,
}
// @@protoc_insertion_point(module)
