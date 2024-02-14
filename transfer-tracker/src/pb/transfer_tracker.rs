// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
    #[prost(message, repeated, tag="2")]
    pub erc1155_transfer_batchs: ::prost::alloc::vec::Vec<Erc1155TransferBatch>,
    #[prost(message, repeated, tag="3")]
    pub erc1155_transfer_singles: ::prost::alloc::vec::Vec<Erc1155TransferSingle>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(uint32, tag="3")]
    pub call_index: u32,
    #[prost(uint32, tag="4")]
    pub call_depth: u32,
    #[prost(enumeration="Source", tag="5")]
    pub source: i32,
    #[prost(string, tag="6")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub amount: ::prost::alloc::string::String,
    #[prost(enumeration="TokenStandard", tag="13")]
    pub token_type: i32,
    #[prost(uint64, tag="14")]
    pub block_number: u64,
    #[prost(uint64, tag="15")]
    pub block_timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc1155TransferBatch {
    #[prost(string, tag="1")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(enumeration="Source", tag="3")]
    pub source: i32,
    #[prost(string, tag="4")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub token_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="7")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="10")]
    pub amounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="TokenStandard", tag="11")]
    pub token_type: i32,
    #[prost(uint64, tag="12")]
    pub block_number: u64,
    #[prost(uint64, tag="13")]
    pub block_timestamp: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Erc1155TransferSingle {
    #[prost(string, tag="1")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub log_index: u32,
    #[prost(enumeration="Source", tag="3")]
    pub source: i32,
    #[prost(string, tag="4")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_address: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub token_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub amount: ::prost::alloc::string::String,
    #[prost(enumeration="TokenStandard", tag="11")]
    pub token_type: i32,
    #[prost(uint64, tag="12")]
    pub block_number: u64,
    #[prost(uint64, tag="13")]
    pub block_timestamp: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TokenStandard {
    Unknown = 0,
    Erc20 = 1,
    Erc721 = 2,
    Erc1155 = 3,
    BaseToken = 4,
}
impl TokenStandard {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TokenStandard::Unknown => "UNKNOWN",
            TokenStandard::Erc20 => "ERC20",
            TokenStandard::Erc721 => "ERC721",
            TokenStandard::Erc1155 => "ERC1155",
            TokenStandard::BaseToken => "BASE_TOKEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "ERC20" => Some(Self::Erc20),
            "ERC721" => Some(Self::Erc721),
            "ERC1155" => Some(Self::Erc1155),
            "BASE_TOKEN" => Some(Self::BaseToken),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Source {
    Call = 0,
    Log = 1,
}
impl Source {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Source::Call => "CALL",
            Source::Log => "LOG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CALL" => Some(Self::Call),
            "LOG" => Some(Self::Log),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
