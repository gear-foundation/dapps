use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;

use crate::auction::AuctionInfo;

pub struct AuctionMetadata;

impl Metadata for AuctionMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = AuctionInfo;
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum Action {
    Buy,
    Create(CreateConfig),
    ForceStop,
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
    AuctionStoped {
        token_owner: ActorId,
        token_id: U256,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Duration {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct CreateConfig {
    pub nft_contract_actor_id: ActorId,
    pub token_id: U256,
    pub starting_price: u128,
    pub discount_rate: u128,
    pub duration: Duration,
}
