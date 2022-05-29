use codec::{Decode, Encode};
use gstd::prelude::*;
use primitive_types::{H256, U256};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum State {
    BalanceOfUser(H256),
    TokenOwner(U256),
    IsTokenOwner(TokenAndUser),
    GetApproved(U256),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StateReply {
    BalanceOfUser(U256),
    TokenOwner(H256),
    IsTokenOwner(bool),
    GetApproved(H256),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct TokenAndUser {
    pub token_id: U256,
    pub user: H256,
}
