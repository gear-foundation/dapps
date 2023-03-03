use gstd::{prelude::*, ActorId};

use primitive_types::U256;

pub type TransactionId = u64;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct AuctionInfo {
    pub nft_contract_actor_id: ActorId,
    pub token_id: U256,
    pub token_owner: ActorId,
    pub auction_owner: ActorId,
    pub starting_price: u128,
    pub current_price: u128,
    pub discount_rate: u128,
    pub time_left: u64,
    pub expires_at: u64,
    pub status: Status,
    pub transactions: BTreeMap<ActorId, Transaction<Action>>,
    pub current_tid: u64,
}

#[derive(Debug, Decode, Default, Encode, TypeInfo, Clone)]
pub enum Status {
    #[default]
    None,
    IsRunning,
    Purchased {
        price: u128,
    },
    Expired,
    Stopped,
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
pub struct Transaction<T: Clone> {
    pub id: TransactionId,
    pub action: T,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum Action {
    Buy,
    Create(CreateConfig),
    ForceStop,
    Reward,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    AuctionStarted {
        token_owner: ActorId,
        price: u128,
        token_id: U256,
    },
    Bought {
        price: u128,
    },
    AuctionStopped {
        token_owner: ActorId,
        token_id: U256,
    },
    Rewarded {
        price: u128,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct Duration {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct CreateConfig {
    pub nft_contract_actor_id: ActorId,
    pub token_id: U256,
    pub starting_price: u128,
    pub discount_rate: u128,
    pub duration: Duration,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Error {
    BuyError,
    PreviousTxMustBeCompleted,
    SendingError,
    NftValidateFailed,
    NftTransferFailed,
    NftOwnerFailed,
    NftNotApproved,
    WrongReply,
    RewardSendFailed,
    NotOwner,
    AlreadyRunning,
    StartPriceLessThatMinimal,
    AlreadyStopped,
    InsufficientMoney,
    Expired,
    WrongState,
    IncorrectRewarder,
}
