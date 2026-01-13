#![no_std]
#![allow(static_mut_refs)]
use extended_vft_client::vft::io as vft_io;
use sails_rs::collections::HashMap;
use sails_rs::gstd::{exec, msg};
use sails_rs::prelude::*;

static mut STORAGE: Option<Storage> = None;
const DECIMALS_FACTOR: u128 = 10_u128.pow(20);

#[derive(Debug, Clone, Default)]
struct Storage {
    owner: ActorId,
    reward_token_address: ActorId,
    tokens_per_stake: u128,
    total_staked: u128,
    distribution_time: u64,
    produced_time: u64,
    reward_total: u128,
    all_produced: u128,
    reward_produced: u128,
    stakers: HashMap<ActorId, Staker>,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone, PartialEq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Staker {
    pub balance: u128,
    pub reward_allowed: u128,
    pub reward_debt: u128,
    pub distributed: u128,
}

#[event]
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    StakeAccepted(u128),
    Updated,
    Reward(u128),
    Withdrawn(u128),
}

struct StakingService(());

impl StakingService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn init(reward_token_address: ActorId, distribution_time: u64, reward_total: u128) -> Self {
        if reward_total == 0 {
            panic!("Reward is zero");
        }
        if distribution_time == 0 {
            panic!("Distribution time is zero");
        }
        let storage = Storage {
            owner: msg::source(),
            reward_token_address,
            distribution_time,
            reward_total,
            produced_time: exec::block_timestamp(),
            ..Default::default()
        };

        unsafe { STORAGE = Some(storage) };
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[service(events = Event)]
impl StakingService {
    /// Stakes the tokens
    /// Arguments:
    /// `amount`: the number of tokens for the stake
    #[export]
    pub async fn stake(&mut self, amount: u128) {
        if amount == 0 {
            panic!("Amount is zero");
        }
        let storage = self.get_mut();
        let msg_src = msg::source();

        let request = vft_io::TransferFrom::encode_params_with_prefix(
            "Vft",
            msg_src,
            exec::program_id(),
            amount.into(),
        );

        msg::send_bytes_with_gas_for_reply(
            storage.reward_token_address,
            request,
            5_000_000_000,
            0,
            0,
        )
        .expect("Error in sending a message")
        .await
        .expect("Error in transfer Fungible Token");

        storage.update_reward();

        let amount_per_token = storage.get_max_reward(amount);

        storage
            .stakers
            .entry(msg_src)
            .and_modify(|stake| {
                stake.reward_debt = stake.reward_debt.saturating_add(amount_per_token);
                stake.balance = stake.balance.saturating_add(amount);
            })
            .or_insert(Staker {
                reward_debt: amount_per_token,
                balance: amount,
                ..Default::default()
            });
        storage.total_staked = storage.total_staked.saturating_add(amount);
        self.emit_event(Event::StakeAccepted(amount))
            .expect("Notification Error");
    }

    /// Withdraws the staked the tokens
    /// Arguments:
    /// `amount`: the number of withdrawn tokens
    #[export]
    pub async fn withdraw(&mut self, amount: u128) {
        if amount == 0 {
            panic!("Amount is zero");
        }
        let storage = self.get_mut();
        storage.update_reward();
        let amount_per_token = storage.get_max_reward(amount);
        let msg_src = msg::source();

        let staker = storage.stakers.get_mut(&msg_src).expect("Staker not found");

        if staker.balance < amount {
            panic!("Insufficent balance");
        }

        let request = vft_io::Transfer::encode_params_with_prefix("Vft", msg_src, amount.into());

        msg::send_bytes_with_gas_for_reply(
            storage.reward_token_address,
            request,
            5_000_000_000,
            0,
            0,
        )
        .expect("Error in sending a message")
        .await
        .expect("Error in transfer Fungible Token");

        staker.reward_allowed = staker.reward_allowed.saturating_add(amount_per_token);
        staker.balance = staker.balance.saturating_sub(amount);
        storage.total_staked = storage.total_staked.saturating_sub(amount);

        self.emit_event(Event::Withdrawn(amount))
            .expect("Notification Error");
    }

    /// Updates the staking contract.
    /// Sets the reward to be distributed within distribution time
    /// param 'config' - updated configuration
    #[export]
    pub fn update_staking(
        &mut self,
        reward_token_address: ActorId,
        distribution_time: u64,
        reward_total: u128,
    ) {
        if reward_total == 0 {
            panic!("Reward is zero");
        }

        if distribution_time == 0 {
            panic!("Distribution time is zero");
        }

        let storage = self.get_mut();

        if msg::source() != storage.owner {
            panic!("Not owner");
        }

        storage.reward_token_address = reward_token_address;
        storage.distribution_time = distribution_time;

        storage.update_reward();
        storage.all_produced = storage.reward_produced;
        storage.produced_time = exec::block_timestamp();
        storage.reward_total = reward_total;
        self.emit_event(Event::Updated).expect("Notification Error");
    }

    /// Sends reward to the staker
    #[export]
    pub async fn get_reward(&mut self) {
        let storage = self.get_mut();
        storage.update_reward();

        let msg_src = msg::source();

        let reward = storage.calc_reward(&msg_src);
        if reward == 0 {
            panic!("Zero reward")
        }

        let request = vft_io::Transfer::encode_params_with_prefix("Vft", msg_src, reward.into());

        msg::send_bytes_with_gas_for_reply(
            storage.reward_token_address,
            request,
            5_000_000_000,
            0,
            0,
        )
        .expect("Error in sending a message")
        .await
        .expect("Error in transfer Fungible Token");

        storage
            .stakers
            .entry(msg::source())
            .and_modify(|stake| stake.distributed = stake.distributed.saturating_add(reward));

        self.emit_event(Event::Reward(reward))
            .expect("Notification Error");
    }

    #[export]
    pub fn owner(&self) -> ActorId {
        self.get().owner
    }

    #[export]
    pub fn reward_token_address(&self) -> ActorId {
        self.get().reward_token_address
    }

    #[export]
    pub fn tokens_per_stake(&self) -> u128 {
        self.get().tokens_per_stake
    }

    #[export]
    pub fn total_staked(&self) -> u128 {
        self.get().total_staked
    }

    #[export]
    pub fn distribution_time(&self) -> u64 {
        self.get().distribution_time
    }

    #[export]
    pub fn produced_time(&self) -> u64 {
        self.get().produced_time
    }

    #[export]
    pub fn reward_total(&self) -> u128 {
        self.get().reward_total
    }

    #[export]
    pub fn all_produced(&self) -> u128 {
        self.get().all_produced
    }

    #[export]
    pub fn reward_produced(&self) -> u128 {
        self.get().reward_produced
    }

    #[export]
    pub fn stakers(&self) -> Vec<(ActorId, Staker)> {
        self.get().stakers.clone().into_iter().collect()
    }

    #[export]
    pub fn calc_reward(&self, id: ActorId) -> u128 {
        self.get().calc_reward(&id)
    }
}

impl Storage {
    /// Updates the reward produced so far and calculates tokens per stake
    fn update_reward(&mut self) {
        let reward_produced_at_now = self.produced();
        if reward_produced_at_now > self.reward_produced {
            let produced_new = reward_produced_at_now - self.reward_produced;
            if self.total_staked > 0 {
                self.tokens_per_stake = self
                    .tokens_per_stake
                    .saturating_add((produced_new * DECIMALS_FACTOR) / self.total_staked);
            }

            self.reward_produced = self.reward_produced.saturating_add(produced_new);
        }
    }
    /// Calculates the reward produced so far
    fn produced(&mut self) -> u128 {
        let mut elapsed_time = exec::block_timestamp() - self.produced_time;

        if elapsed_time > self.distribution_time {
            elapsed_time = self.distribution_time;
        }

        self.all_produced
            + self.reward_total.saturating_mul(elapsed_time as u128)
                / self.distribution_time as u128
    }
    /// Calculates the maximum possible reward
    /// The reward that the depositor would have received if he had initially paid this amount
    /// Arguments:
    /// `amount`: the number of tokens
    fn get_max_reward(&self, amount: u128) -> u128 {
        (amount * self.tokens_per_stake) / DECIMALS_FACTOR
    }

    /// Calculates the reward of the staker that is currently available
    /// The return value cannot be less than zero according to the algorithm
    fn calc_reward(&self, id: &ActorId) -> u128 {
        match self.stakers.get(id) {
            Some(staker) => {
                self.get_max_reward(staker.balance) + staker.reward_allowed
                    - staker.reward_debt
                    - staker.distributed
            }
            None => panic!("Staker not found"),
        }
    }
}

pub struct StakingProgram(());

#[sails_rs::program]
impl StakingProgram {
    // Program's constructor
    pub fn new(reward_token_address: ActorId, distribution_time: u64, reward_total: u128) -> Self {
        StakingService::init(reward_token_address, distribution_time, reward_total);
        Self(())
    }

    // Exposed service
    pub fn staking(&self) -> StakingService {
        StakingService::new()
    }
}
