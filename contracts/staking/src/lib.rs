#![no_std]

pub mod io;

use ft_io::*;
use gstd::{exec, msg, prelude::*, ActorId};

use crate::io::*;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
struct Staking {
    owner: ActorId,
    staking_token_address: ActorId,
    reward_token_address: ActorId,
    tokens_per_stake: u128,
    total_staked: u128,
    distribution_time: u64,
    produced_time: u64,
    reward_total: u128,
    all_produced: u128,
    reward_produced: u128,
    stakers: BTreeMap<ActorId, Staker>,
}

static mut STAKING: Option<Staking> = None;
const DECIMALS_FACTOR: u128 = 10_u128.pow(20);

/// Transfers `amount` tokens from `sender` account to `recipient` account.
/// Arguments:
/// * `from`: sender account
/// * `to`: recipient account
/// * `amount`: amount of tokens
async fn transfer_tokens(
    token_address: &ActorId,
    from: &ActorId,
    to: &ActorId,
    amount_tokens: u128,
) {
    msg::send_for_reply(
        *token_address,
        FTAction::Transfer {
            from: *from,
            to: *to,
            amount: amount_tokens,
        },
        0,
    )
    .expect("Error in sending message")
    .await
    .expect("Error in transfer");
}

impl Staking {
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

    /// Calculates the maximum possible reward
    /// The reward that the depositor would have received if he had initially paid this amount
    /// Arguments:
    /// `amount`: the number of tokens
    fn get_max_reward(&self, amount: u128) -> u128 {
        (amount * self.tokens_per_stake) / DECIMALS_FACTOR
    }

    /// Calculates the reward of the staker that is currently avaiable
    /// The return value cannot be less than zero according to the algorithm
    fn calc_reward(&mut self) -> u128 {
        let staker = self
            .stakers
            .get(&msg::source())
            .unwrap_or_else(|| panic!("calc_reward(): Staker {:?} not found", msg::source()));

        self.get_max_reward(staker.balance) + staker.reward_allowed
            - staker.reward_debt
            - staker.distributed
    }

    /// Updates the staking contract.
    /// Sets the reward to be distributed within distribution time
    /// param 'config' - updated configuration
    fn update_staking(&mut self, config: InitStaking) {
        if msg::source() != self.owner {
            panic!("update_staking(): only the owner can update the staking");
        }

        if config.reward_total == 0 {
            panic!("update_staking(): reward_total is null");
        }

        if config.distribution_time == 0 {
            panic!("update_staking(): distribution_time is null");
        }

        self.staking_token_address = config.staking_token_address;
        self.reward_token_address = config.reward_token_address;
        self.distribution_time = config.distribution_time;

        self.update_reward();
        self.all_produced = self.reward_produced;
        self.produced_time = exec::block_timestamp();
        self.reward_total = config.reward_total;
    }

    /// Stakes the tokens
    /// Arguments:
    /// `amount`: the number of tokens for the stake
    async fn stake(&mut self, amount: u128) {
        if amount == 0 {
            panic!("stake(): amount is null");
        }

        let token_address = self.staking_token_address;

        transfer_tokens(&token_address, &msg::source(), &exec::program_id(), amount).await;

        self.update_reward();
        let amount_per_token = self.get_max_reward(amount);

        self.stakers
            .entry(msg::source())
            .and_modify(|stake| {
                stake.reward_debt = stake.reward_debt.saturating_add(amount_per_token);
                stake.balance = stake.balance.saturating_add(amount);
            })
            .or_insert(Staker {
                reward_debt: amount_per_token,
                balance: amount,
                ..Default::default()
            });

        self.total_staked = self.total_staked.saturating_add(amount);
        msg::reply(StakingEvent::StakeAccepted(amount), 0).expect("reply: 'StakeAccepted' error");
    }

    ///Sends reward to the staker
    async fn send_reward(&mut self) {
        self.update_reward();
        let reward = self.calc_reward();

        if reward == 0 {
            panic!("send_reward(): reward is null");
        }

        let token_address = self.reward_token_address;

        transfer_tokens(&token_address, &exec::program_id(), &msg::source(), reward).await;

        self.stakers
            .entry(msg::source())
            .and_modify(|stake| stake.distributed = stake.distributed.saturating_add(reward));

        msg::reply(StakingEvent::Reward(reward), 0).expect("reply: 'Reward' error");
    }

    /// Withdraws the staked the tokens
    /// Arguments:
    /// `amount`: the number of withdrawn tokens
    async fn withdraw(&mut self, amount: u128) {
        if amount == 0 {
            panic!("withdraw(): amount is null");
        }

        self.update_reward();
        let amount_per_token = self.get_max_reward(amount);

        let staker = self
            .stakers
            .get_mut(&msg::source())
            .unwrap_or_else(|| panic!("withdraw(): Staker {:?} not found", msg::source()));

        if staker.balance < amount {
            panic!("withdraw(): staker.balance < amount");
        }

        let token_address = self.staking_token_address;
        transfer_tokens(&token_address, &exec::program_id(), &msg::source(), amount).await;

        staker.reward_allowed = staker.reward_allowed.saturating_add(amount_per_token);
        staker.balance = staker.balance.saturating_sub(amount);
        self.total_staked = self.total_staked.saturating_sub(amount);

        msg::reply(StakingEvent::Withdrawn(amount), 0).expect("reply: 'Withdrawn' error");
    }
}

#[gstd::async_main]
async fn main() {
    let staking = unsafe { STAKING.get_or_insert(Staking::default()) };

    let action: StakingAction = msg::load().expect("Could not load Action");

    match action {
        StakingAction::Stake(amount) => {
            staking.stake(amount).await;
        }

        StakingAction::Withdraw(amount) => {
            staking.withdraw(amount).await;
        }

        StakingAction::UpdateStaking(config) => {
            staking.update_staking(config);
            msg::reply(StakingEvent::Updated, 0).expect("reply: 'Updated' error");
        }

        StakingAction::GetReward => {
            staking.send_reward().await;
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: InitStaking = msg::load().expect("Unable to decode InitConfig");

    let mut staking = Staking {
        owner: msg::source(),
        ..Default::default()
    };

    staking.update_staking(config);
    unsafe { STAKING = Some(staking) };
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: StakingState = msg::load().expect("failed to decode input argument");
    let staking = unsafe { STAKING.get_or_insert(Staking::default()) };

    let encoded = match query {
        StakingState::GetStakers => StakingStateReply::Stakers(staking.stakers.clone()).encode(),

        StakingState::GetStaker(address) => {
            if let Some(staker) = staking.stakers.get(&address) {
                StakingStateReply::Staker(staker.clone()).encode()
            } else {
                panic!("meta_state(): Staker {:?} not found", address);
            }
        }
    };

    gstd::util::to_leak_ptr(encoded)
}

gstd::metadata! {
    title: "Staking",
    init:
        input: InitStaking,
    handle:
        input: StakingAction,
        output: StakingEvent,
    state:
        input: StakingState,
        output: StakingStateReply,
}
