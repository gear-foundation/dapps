use codec::{Decode, Encode};
use gstd::String;
use primitive_types::{H256, U256};
use scale_info::TypeInfo;

#[derive(Debug, Decode, TypeInfo)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct TransferInput {
    pub to: H256,
    pub token_id: U256,
}

