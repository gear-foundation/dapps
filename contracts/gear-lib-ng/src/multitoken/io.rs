use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;
pub type TokenId = u128;

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub reference: Option<String>,
}

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone)]
pub struct Token {
    pub id: TokenId,
    pub amount: u128,
    pub metadata: Option<TokenMetadata>,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitConfig {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct BalanceReply {
    pub account: ActorId,
    pub id: TokenId,
    pub amount: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MTKEvent {
    Transfer {
        operator: ActorId,
        from: ActorId,
        to: ActorId,
        ids: Vec<TokenId>,
        amounts: Vec<u128>,
    },
    BalanceOf(Vec<BalanceReply>),
    Approval {
        from: ActorId,
        to: ActorId,
    },
    RevokeApproval {
        from: ActorId,
        to: ActorId,
    },
}
