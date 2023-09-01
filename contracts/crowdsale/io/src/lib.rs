#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{exec, prelude::*, ActorId};

pub struct CrowdsaleMetadata;

impl Metadata for CrowdsaleMetadata {
    type Init = In<IcoInit>;
    type Handle = InOut<IcoAction, IcoEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone, Copy)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IcoState {
    pub ico_started: bool,
    pub start_time: u64,
    pub duration: u64,
    pub ico_ended: bool,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum IcoAction {
    /// Starts ICO contract.
    ///
    /// # Requirements:
    /// * Only owner can start ICO.
    /// * At least `tokens_goal` tokens need to be minted.
    /// * ICO can be started only once.
    /// * All arguments must be greater than zero.
    ///
    /// On success replies with [`IcoEvent::SaleStarted`].
    StartSale {
        /// ICO duration.
        duration: u64,

        /// Start price.
        start_price: u128,

        /// Tokens goal.
        tokens_goal: u128,

        /// Price increase step.
        price_increase_step: u128,

        /// Time increase step.
        time_increase_step: u128,
    },

    /// Purchase of tokens.
    ///
    /// # Requirements:
    /// * `tokens_cnt` must be greater than zero.
    /// * ICO must be in progress (already started and not finished yet).
    /// * [`msg::value()`](gstd::msg::value) must be greater than or equal to `price * tokens_cnt`.
    /// * At least `tokens_cnt` tokens available for sale.
    ///
    /// On success replies with [`IcoEvent::Bought`].
    Buy(
        /// Amount of tokens to purchase.
        u128,
    ),

    /// Ends ICO contract.
    ///
    /// # Requirements:
    /// * Only owner can end ICO.
    /// * ICO can be ended more only once.
    /// * All tokens must be sold or the ICO duration must end.
    ///
    /// On success replies with [`IcoEvent::SaleEnded`].
    EndSale,
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum IcoEvent {
    SaleStarted {
        transaction_id: u64,
        duration: u64,
        start_price: u128,
        tokens_goal: u128,
        price_increase_step: u128,
        time_increase_step: u128,
    },
    Bought {
        buyer: ActorId,
        amount: u128,
        change: u128,
    },
    SaleEnded(u64),
    TransactionFailed(u64),
}

#[derive(Debug, Decode, Encode, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IcoInit {
    pub token_address: ActorId,
    pub owner: ActorId,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateIco {
    CurrentPrice,
    TokensLeft,
    BalanceOf(ActorId),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateIcoReply {
    CurrentPrice(u128),
    TokensLeft(u128),
    BalanceOf { address: ActorId, balance: u128 },
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub ico_state: IcoState,
    pub start_price: u128,
    pub price_increase_step: u128,
    pub time_increase_step: u128,
    pub tokens_sold: u128,
    pub tokens_goal: u128,
    pub owner: ActorId,
    pub token_address: ActorId,
    pub token_holders: Vec<(ActorId, u128)>,
    pub transaction_id: u64,
    pub transactions: Vec<(ActorId, u64)>,
}

impl State {
    pub fn get_current_price(&self) -> u128 {
        let time_now: u64 = exec::block_timestamp();
        let amount: u128 = (time_now - self.ico_state.start_time).into();

        self.start_price + self.price_increase_step * (amount / self.time_increase_step)
    }

    pub fn get_balance(&self) -> u128 {
        self.tokens_goal - self.tokens_sold
    }

    pub fn balance_of(&self, address: &ActorId) -> u128 {
        match self
            .token_holders
            .iter()
            .find(|(id, _balance)| id.eq(address))
        {
            Some((_id, balance)) => *balance,
            None => 0,
        }
    }
}
