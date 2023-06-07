use gstd::{prelude::*, ActorId};

use primitive_types::U256;

pub type TransactionId = u64;

/// An auction info and auction state
#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct AuctionInfo {
    /// NFT contract address
    pub nft_contract_actor_id: ActorId,
    /// NFT token id
    pub token_id: U256,
    /// NFT owner
    pub token_owner: ActorId,
    /// Auction owner
    pub auction_owner: ActorId,
    /// Starting price of NFT at auction
    pub starting_price: u128,
    /// Current price of NFT
    pub current_price: u128,
    /// Price step by which the NFT price decreases
    pub discount_rate: u128,
    /// Time left until the end of the auction
    pub time_left: u64,
    /// Time when the auction expires
    pub expires_at: u64,
    /// Current auction status
    pub status: Status,

    /// Transactions that cached on contract
    pub transactions: BTreeMap<ActorId, Transaction<Action>>,
    /// Current transaction id
    pub current_tid: u64,
}

/// An enum that represent current auction status
#[derive(Debug, Decode, Default, Encode, TypeInfo, Clone)]
pub enum Status {
    #[default]
    None,
    /// Auction is running right now
    IsRunning,
    /// Someone purchased NFT, but previous NFT owner not rewarded
    Purchased { price: u128 },
    /// Someone purchased NFT and previous NFT owner rewarded
    Rewarded { price: u128 },
    /// Time for the auction has expired and no one has made a purchase.
    Expired,
    /// Auction stopped by auction owner
    Stopped,
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
pub struct Transaction<T: Clone> {
    pub id: TransactionId,
    pub action: T,
}

/// An enum to send the program info about what it should do.
///
/// After a successful processing of this enum, the program replies with [`Event`].
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum Action {
    /// Creates auction
    Create(CreateConfig),
    /// Buy current NFT
    Buy,
    /// Stop Auction
    ForceStop,
    /// Reward gas to NFT seller
    Reward,
}

/// An enum that contains a result of processed [`Action`].
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Event {
    AuctionStarted {
        /// Owner of auction NFT
        token_owner: ActorId,
        /// Started price of NFT
        price: u128,
        /// NFT token id
        token_id: U256,
    },
    Bought {
        /// Price for which the NFT were bought
        price: u128,
    },
    AuctionStopped {
        token_owner: ActorId,
        token_id: U256,
    },
    Rewarded {
        /// Reward that owner received
        price: u128,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct Duration {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

/// Dutch Auction config
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub struct CreateConfig {
    /// Address of NFT contract
    pub nft_contract_actor_id: ActorId,
    /// NFT token id
    pub token_id: U256,
    /// Starting price
    pub starting_price: u128,
    /// Price step by which the NFT price decreases
    pub discount_rate: u128,
    /// Auction duration
    pub duration: Duration,
}

/// An enum that contains a error of processed [`Action`].
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Error {
    PreviousTxMustBeCompleted,
    SendingError,
    NftValidateFailed,
    NftTransferFailed,
    NftOwnerFailed,
    NftNotApproved,
    NotRewarded,
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
