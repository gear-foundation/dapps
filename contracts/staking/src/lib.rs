#![no_std]

use gmeta::Metadata;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::HashMap;
use sharded_fungible_token_io::*;
use staking_io::*;

#[derive(Debug, Clone, Default)]
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
    stakers: HashMap<ActorId, Staker>,

    transactions: BTreeMap<ActorId, Transaction<StakingAction>>,
    current_tid: TransactionId,
}

static mut STAKING: Option<Staking> = None;
const DECIMALS_FACTOR: u128 = 10_u128.pow(20);

impl Staking {
    /// Transfers `amount` tokens from `sender` account to `recipient` account.
    /// Arguments:
    /// * `from`: sender account
    /// * `to`: recipient account
    /// * `amount`: amount of tokens
    async fn transfer_tokens(
        &mut self,
        token_address: &ActorId,
        from: &ActorId,
        to: &ActorId,
        amount_tokens: u128,
    ) -> Result<(), Error> {
        let payload = LogicAction::Transfer {
            sender: *from,
            recipient: *to,
            amount: amount_tokens,
        };

        let transaction_id = self.current_tid;
        self.current_tid = self.current_tid.saturating_add(99);

        let payload = FTokenAction::Message {
            transaction_id,
            payload,
        };

        let result = msg::send_for_reply_as(*token_address, payload, 0, 0)?.await?;

        if let FTokenEvent::Err = result {
            Err(Error::TransferTokens)
        } else {
            Ok(())
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

    /// Calculates the reward of the staker that is currently available
    /// The return value cannot be less than zero according to the algorithm
    fn calc_reward(&mut self) -> Result<u128, Error> {
        match self.stakers.get(&msg::source()) {
            Some(staker) => Ok(self.get_max_reward(staker.balance) + staker.reward_allowed
                - staker.reward_debt
                - staker.distributed),
            None => Err(Error::StakerNotFound),
        }
    }

    /// Updates the staking contract.
    /// Sets the reward to be distributed within distribution time
    /// param 'config' - updated configuration
    fn update_staking(&mut self, config: InitStaking) -> Result<StakingEvent, Error> {
        if msg::source() != self.owner {
            return Err(Error::NotOwner);
        }

        if config.reward_total == 0 {
            return Err(Error::ZeroReward);
        }

        if config.distribution_time == 0 {
            return Err(Error::ZeroTime);
        }

        self.staking_token_address = config.staking_token_address;
        self.reward_token_address = config.reward_token_address;
        self.distribution_time = config.distribution_time;

        self.update_reward();
        self.all_produced = self.reward_produced;
        self.produced_time = exec::block_timestamp();
        self.reward_total = config.reward_total;

        Ok(StakingEvent::Updated)
    }

    /// Stakes the tokens
    /// Arguments:
    /// `amount`: the number of tokens for the stake
    async fn stake(&mut self, amount: u128) -> Result<StakingEvent, Error> {
        if amount == 0 {
            return Err(Error::ZeroAmount);
        }

        let token_address = self.staking_token_address;

        self.transfer_tokens(&token_address, &msg::source(), &exec::program_id(), amount)
            .await?;

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
        Ok(StakingEvent::StakeAccepted(amount))
    }

    ///Sends reward to the staker
    async fn send_reward(&mut self) -> Result<StakingEvent, Error> {
        self.update_reward();
        let reward = self.calc_reward()?;

        if reward == 0 {
            return Err(Error::ZeroReward);
        }

        let token_address = self.reward_token_address;

        self.transfer_tokens(&token_address, &exec::program_id(), &msg::source(), reward)
            .await?;

        self.stakers
            .entry(msg::source())
            .and_modify(|stake| stake.distributed = stake.distributed.saturating_add(reward));

        Ok(StakingEvent::Reward(reward))
    }

    /// Withdraws the staked the tokens
    /// Arguments:
    /// `amount`: the number of withdrawn tokens
    async fn withdraw(&mut self, amount: u128) -> Result<StakingEvent, Error> {
        if amount == 0 {
            return Err(Error::ZeroAmount);
        }

        self.update_reward();
        let amount_per_token = self.get_max_reward(amount);

        match self.stakers.get(&msg::source()) {
            Some(staker) => {
                if staker.balance < amount {
                    return Err(Error::InsufficentBalance);
                }
            }
            None => return Err(Error::StakerNotFound),
        };

        let token_address = self.staking_token_address;
        self.transfer_tokens(&token_address, &exec::program_id(), &msg::source(), amount)
            .await?;

        let staker = self
            .stakers
            .get_mut(&msg::source())
            .ok_or(Error::StakerNotFound)?;

        staker.reward_allowed = staker.reward_allowed.saturating_add(amount_per_token);
        staker.balance = staker.balance.saturating_sub(amount);
        self.total_staked = self.total_staked.saturating_sub(amount);

        Ok(StakingEvent::Withdrawn(amount))
    }
}

#[gstd::async_main]
async fn main() {
    let staking = unsafe { STAKING.get_or_insert(Staking::default()) };

    let action: StakingAction = msg::load().expect("Could not load Action");
    let msg_source = msg::source();

    let _reply: Result<StakingEvent, Error> = Err(Error::PreviousTxMustBeCompleted);
    let _transaction_id = if let Some(Transaction {
        id,
        action: pend_action,
    }) = staking.transactions.get(&msg_source)
    {
        if action != *pend_action {
            reply(_reply).expect("Failed to encode or reply with `Result<StakingEvent, Error>`");
            return;
        }
        *id
    } else {
        let transaction_id = staking.current_tid;
        staking.current_tid = staking.current_tid.saturating_add(1);
        staking.transactions.insert(
            msg_source,
            Transaction {
                id: transaction_id,
                action: action.clone(),
            },
        );
        transaction_id
    };
    let result = match action {
        StakingAction::Stake(amount) => {
            let result = staking.stake(amount).await;
            staking.transactions.remove(&msg_source);
            result
        }
        StakingAction::Withdraw(amount) => {
            let result = staking.withdraw(amount).await;
            staking.transactions.remove(&msg_source);
            result
        }
        StakingAction::UpdateStaking(config) => {
            let result = staking.update_staking(config);
            staking.transactions.remove(&msg_source);
            result
        }
        StakingAction::GetReward => {
            let result = staking.send_reward().await;
            staking.transactions.remove(&msg_source);
            result
        }
    };
    reply(result).expect("Failed to encode or reply with `Result<StakingEvent, Error>`");
}

#[no_mangle]
extern fn init() {
    let config: InitStaking = msg::load().expect("Unable to decode InitConfig");

    let mut staking = Staking {
        owner: msg::source(),
        ..Default::default()
    };

    let result = staking.update_staking(config);
    let is_err = result.is_err();

    reply(result).expect("Failed to encode or reply with `Result<(), Error>` from `init()`");

    if is_err {
        exec::exit(ActorId::zero());
    }

    unsafe { STAKING = Some(staking) };
}

fn common_state() -> <StakingMetadata as Metadata>::State {
    let state = static_mut_state();

    let Staking {
        owner,
        staking_token_address,
        reward_token_address,
        tokens_per_stake,
        total_staked,
        distribution_time,
        produced_time,
        reward_total,
        all_produced,
        reward_produced,
        stakers,
        transactions,
        current_tid,
    } = state.clone();

    let stakers = stakers.iter().map(|(k, v)| (*k, v.clone())).collect();

    IoStaking {
        owner,
        staking_token_address,
        reward_token_address,
        tokens_per_stake,
        total_staked,
        distribution_time,
        produced_time,
        reward_total,
        all_produced,
        reward_produced,
        stakers,
        transactions,
        current_tid,
    }
}

fn static_mut_state() -> &'static mut Staking {
    unsafe { STAKING.get_or_insert(Default::default()) }
}

#[no_mangle]
extern fn state() {
    reply(common_state())
        .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}
