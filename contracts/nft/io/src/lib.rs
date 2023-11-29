#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub type TokenId = u128;
pub const ZERO_ID: ActorId = ActorId::zero();

pub struct NftMetadata;

impl Metadata for NftMetadata {
    type Init = In<InitNft>;
    type Handle = InOut<NftAction, NftEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct Config {
    pub max_mint_count: Option<u128>,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct InitNft {
    pub collection: Collection,
    pub config: Config,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct Collection {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub enum NftAction {
    Mint { token_metadata: TokenMetadata },
    Burn { token_id: TokenId },
    Transfer { to: ActorId, token_id: TokenId },
    Approve { to: ActorId, token_id: TokenId },
    Owner { token_id: TokenId },
    IsApproved { to: ActorId, token_id: TokenId },
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub enum NftEvent {
    Mint {
        to: ActorId,
        token_metadata: TokenMetadata,
    },
    Burn {
        token_id: TokenId,
    },
    Transfer {
        from: ActorId,
        to: ActorId,
        token_id: TokenId,
    },
    Approval {
        owner: ActorId,
        approved_account: ActorId,
        token_id: TokenId,
    },
    Owner {
        owner: ActorId,
        token_id: TokenId,
    },
    IsApproved {
        to: ActorId,
        token_id: TokenId,
        approved: bool,
    },
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]

pub struct TokenMetadata {
    // ex. "CryptoKitty #100"
    pub name: String,
    // free-form description
    pub description: String,
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media: String,
    // URL to an off-chain JSON file with more info.
    pub reference: String,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct Nft {
    pub owner: ActorId,
    pub name: String,
    pub description: String,
    pub media_url: String,
    pub attrib_url: String,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub struct State {
    pub owner_by_id: Vec<(TokenId, ActorId)>,
    pub token_approvals: Vec<(TokenId, Vec<ActorId>)>,
    pub token_metadata_by_id: Vec<(TokenId, TokenMetadata)>,
    pub tokens_for_owner: Vec<(ActorId, Vec<TokenId>)>,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub collection: Collection,
    pub config: Config,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    All,
    Config,
    Collection,
    Owner,
    CurrentTokenId,
    OwnerById { token_id: TokenId },
    TokenApprovals { token_id: TokenId },
    TokenMetadata { token_id: TokenId },
    OwnerTokens { owner: ActorId },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(State),
    Config(Config),
    Collection(Collection),
    Owner(ActorId),
    CurrentTokenId(TokenId),
    OwnerById(Option<ActorId>),
    TokenApprovals(Option<Vec<ActorId>>),
    TokenMetadata(Option<TokenMetadata>),
    OwnerTokens(Option<Vec<TokenId>>),
}
