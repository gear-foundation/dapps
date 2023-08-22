#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{errors::Error as GstdError, prelude::*, ActorId};

pub type TransactionId = u64;

pub struct StakingMetadata;

impl Metadata for StakingMetadata {
    type Init = In<InitStaking>;
    type Handle = InOut<StakingAction, Result<StakingEvent, Error>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = IoStaking;
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub struct InitStaking {
    pub staking_token_address: ActorId,
    pub reward_token_address: ActorId,
    pub distribution_time: u64,
    pub reward_total: u128,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone, PartialEq)]
pub struct Staker {
    pub balance: u128,
    pub reward_allowed: u128,
    pub reward_debt: u128,
    pub distributed: u128,
}

#[derive(Debug, Clone, Decode, Encode, TypeInfo, PartialEq, Eq)]
pub enum StakingAction {
    Stake(u128),
    Withdraw(u128),
    UpdateStaking(InitStaking),
    GetReward,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StakingEvent {
    StakeAccepted(u128),
    Updated,
    Reward(u128),
    Withdrawn(u128),
}

#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
pub struct IoStaking {
    pub owner: ActorId,
    pub staking_token_address: ActorId,
    pub reward_token_address: ActorId,
    pub tokens_per_stake: u128,
    pub total_staked: u128,
    pub distribution_time: u64,
    pub produced_time: u64,
    pub reward_total: u128,
    pub all_produced: u128,
    pub reward_produced: u128,
    pub stakers: Vec<(ActorId, Staker)>,
    pub transactions: BTreeMap<ActorId, Transaction<StakingAction>>,
    pub current_tid: TransactionId,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Error {
    ZeroAmount,
    ZeroReward,
    ZeroTime,
    TransferTokens,
    PreviousTxMustBeCompleted,
    InsufficentBalance,
    NotOwner,
    StakerNotFound,
    ContractError(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Transaction<T> {
    pub id: TransactionId,
    pub action: T,
}

impl From<GstdError> for Error {
    fn from(value: GstdError) -> Self {
        Self::ContractError(value.to_string())
    }
}
