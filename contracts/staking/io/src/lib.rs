#![no_std]
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
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

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum StakingAction {
    Stake(u128),
    Withdraw(u128),
    UpdateStaking(InitStaking),
    GetReward,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum StakingEvent {
    StakeAccepted(u128),
    Withdrawn(u128),
    Updated,
    Reward(u128),
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq)]
pub enum StakingState {
    GetStakers,
    GetStaker(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq)]
pub enum StakingStateReply {
    Stakers(BTreeMap<ActorId, Staker>),
    Staker(Staker),
}
