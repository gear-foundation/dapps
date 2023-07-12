use dex_factory_io::{Action as FactoryAction, Error as FactoryError, Event as FactoryEvent};
use dex_pair_io::{
    hidden::{
        calculate_in_amount, calculate_out_amount, quote, quote_reserve_unchecked, U256PairTuple,
    },
    *,
};
use gear_lib::{
    tokens::fungible::FTState,
    tx_manager::{ActionKind, Stepper, TransactionManager},
};
use gstd::{errors::Result, exec, msg, prelude::*, ActorId};
use primitive_types::U256;

mod utils;

fn state_mut() -> &'static mut (Contract, TransactionManager<CachedAction>) {
    unsafe { STATE.as_mut().expect("state isn't initialized") }
}

static mut STATE: Option<(Contract, TransactionManager<CachedAction>)> = None;

#[derive(Default)]
struct Contract {
    factory: ActorId,

    token: (ActorId, ActorId),
    reserve: (u128, u128),
    cumulative_price: (U256, U256),
    last_block_ts: u64,
    k_last: U256,
    ft_state: FTState,
}

impl Contract {
    async fn add_liquidity(
        &mut self,
        (tx_manager, kind): (&mut TransactionManager<CachedAction>, ActionKind),
        msg_source: ActorId,
        desired_amount: (u128, u128),
        min_amount: (u128, u128),
        to: ActorId,
    ) -> Result<Event, Error> {
        // Calculating an input amount
        let amount = if self.reserve == (0, 0) {
            desired_amount
        } else {
            let optimal_amount_b = quote(desired_amount.0, self.reserve)?;

            if optimal_amount_b <= desired_amount.1 {
                if optimal_amount_b < min_amount.1 {
                    return Err(Error::InsufficientLatterAmount);
                }

                (desired_amount.0, optimal_amount_b)
            } else {
                let optimal_amount_a =
                    quote_reserve_unchecked(desired_amount.1, (self.reserve.1, self.reserve.0))?;

                if optimal_amount_a < min_amount.0 {
                    return Err(Error::InsufficientFormerAmount);
                }

                (optimal_amount_a, desired_amount.1)
            }
        };

        let mut tx_guard = tx_manager.acquire_transaction(
            msg_source,
            kind.to_tx_kind(CachedAction::AddLiquidity(amount)),
        )?;

        tx_guard
            .tx_data
            .check_tx_data(|tx_data| tx_data == &CachedAction::AddLiquidity(amount))?;

        let balance = if let (Some(balance_a), Some(balance_b)) = (
            self.reserve.0.checked_add(amount.0),
            self.reserve.1.checked_add(amount.1),
        ) {
            (balance_a, balance_b)
        } else {
            return Err(Error::Overflow);
        };

        let (is_fee_on, fee_receiver, fee) = self.calculate_fee().await?;
        let U256PairTuple(amount_u256) = amount.into();
        let program_id = exec::program_id();

        // Calculating liquidity
        let (liquidity, event) = if self.ft_state.total_supply().is_zero() {
            // First minting

            let liquidity = (amount_u256.0 * amount_u256.1)
                .integer_sqrt()
                .checked_sub(MINIMUM_LIQUIDITY.into())
                .ok_or(Error::InsufficientAmount)?;

            let event = self
                .update_liquidity(
                    &mut tx_guard.stepper,
                    program_id,
                    msg_source,
                    amount,
                    balance,
                    liquidity,
                )
                .await?;

            // Locking the `MINIMUM_LIQUIDITY` for safer calculations during
            // further operations.
            self.ft_state
                .mint(program_id, MINIMUM_LIQUIDITY.into())
                .expect("unchecked condition occurred for `FTState`");

            (liquidity, event)
        } else {
            // Subsequent mintings

            // Checking for an overflow on adding `fee` to `total_supply.`
            let total_supply = self
                .ft_state
                .total_supply()
                .checked_add(fee)
                .ok_or(Error::Overflow)?;
            let (Some(numerator_a), Some(numerator_b)) = (
                amount_u256.0.checked_mul(total_supply),
                amount_u256.1.checked_mul(total_supply),
            ) else {
                return Err(Error::Overflow);
            };
            let U256PairTuple(reserve) = self.reserve.into();
            let liquidity = cmp::min(numerator_a / reserve.0, numerator_b / reserve.1);

            // Checking for an overflow on adding `liquidity` to `total_supply.`
            if total_supply.checked_add(liquidity).is_none() {
                return Err(Error::Overflow);
            }

            let event = self
                .update_liquidity(
                    &mut tx_guard.stepper,
                    program_id,
                    msg_source,
                    amount,
                    balance,
                    liquidity,
                )
                .await?;

            if !fee.is_zero() {
                self.ft_state
                    .mint(fee_receiver, fee)
                    .expect("unchecked overflow occurred for `FTState`");
            }

            (liquidity, event)
        };

        if is_fee_on {
            let U256PairTuple(balance) = balance.into();

            self.k_last = balance.0 * balance.1;
        }

        self.ft_state
            .mint(to, liquidity)
            .expect("unchecked condition occurred for `FTState`");

        Ok(event)
    }

    async fn update_liquidity(
        &mut self,
        stepper: &mut Stepper,
        program_id: ActorId,
        msg_source: ActorId,
        amount: (u128, u128),
        balance: (u128, u128),
        liquidity: U256,
    ) -> Result<Event, Error> {
        if liquidity.is_zero() {
            return Err(Error::InsufficientLiquidity);
        }

        utils::transfer_tokens(stepper, self.token.0, msg_source, program_id, amount.0).await?;

        if let Err(error) =
            utils::transfer_tokens(stepper, self.token.1, msg_source, program_id, amount.1).await
        {
            utils::transfer_tokens(stepper, self.token.0, program_id, msg_source, amount.0).await?;

            Err(error)
        } else {
            self.update(balance);

            Ok(Event::AddedLiquidity {
                sender: msg_source,
                amount_a: amount.0,
                amount_b: amount.1,
                liquidity,
            })
        }
    }

    async fn calculate_fee(&self) -> Result<(bool, ActorId, U256), Error> {
        let fee_to_result: Result<FactoryEvent, FactoryError> =
            utils::send(self.factory, FactoryAction::GetFeeTo)?.await?;
        let Ok(FactoryEvent::FeeToSet(fee_receiver)) = fee_to_result else {
                return Err(Error::FeeToGettingFailed);
        };

        let is_fee_on = !fee_receiver.is_zero();
        let mut fee = U256::zero();

        if is_fee_on && !self.k_last.is_zero() {
            let U256PairTuple(reserve) = self.reserve.into();
            let root_k = (reserve.0 * reserve.1).integer_sqrt();
            let root_k_last = self.k_last.integer_sqrt();

            if root_k > root_k_last {
                let numerator = self
                    .ft_state
                    .total_supply()
                    .checked_mul(root_k - root_k_last)
                    .ok_or(Error::Overflow)?;
                // Shouldn't overflow.
                let denominator = root_k * 5 + root_k_last;

                fee = numerator / denominator;
            }
        }

        Ok((is_fee_on, fee_receiver, fee))
    }

    async fn remove_liquidity(
        &mut self,
        stepper: &mut Stepper,
        is_burned: &mut bool,
        msg_source: ActorId,
        liquidity: U256,
        min_amount: (u128, u128),
        to: ActorId,
    ) -> Result<Event, Error> {
        if *is_burned {
            self.ft_state
                .mint(msg_source, liquidity)
                .expect("unexpected overflow occurred for `FTState`");
            *is_burned = false;
        }

        if self.ft_state.balance_of(msg_source) < liquidity {
            return Err(Error::InsufficientLiquidity);
        }

        let (is_fee_on, fee_receiver, fee) = self.calculate_fee().await?;
        let U256PairTuple(reserve) = self.reserve.into();

        // Calculating an output amount
        let amount = if let (Some(amount_a), Some(amount_b)) = (
            liquidity.checked_mul(reserve.0),
            liquidity.checked_mul(reserve.1),
        ) {
            // Checking for an overflow on adding `fee` to `total_supply.`
            if let Some(total_supply) = self.ft_state.total_supply().checked_add(fee) {
                // Shouldn't be more than u128::MAX, so casting doesn't lose
                // data.
                (
                    (amount_a / total_supply).low_u128(),
                    (amount_b / total_supply).low_u128(),
                )
            } else {
                return Err(Error::Overflow);
            }
        } else {
            return Err(Error::Overflow);
        };

        if amount.0 == 0 || amount.1 == 0 {
            return Err(Error::InsufficientLiquidity);
        }

        if amount.0 < min_amount.0 {
            return Err(Error::InsufficientFormerAmount);
        }

        if amount.1 < min_amount.1 {
            return Err(Error::InsufficientLatterAmount);
        }

        self.ft_state
            .burn(msg_source, liquidity)
            .expect("unchecked overflow occurred for `FTState`");

        *is_burned = true;

        let program_id = exec::program_id();

        utils::transfer_tokens(stepper, self.token.0, program_id, to, amount.0).await?;
        utils::transfer_tokens(stepper, self.token.1, program_id, to, amount.1).await?;

        let balance = (self.reserve.0 - amount.0, self.reserve.1 - amount.1);

        if is_fee_on {
            if !fee.is_zero() {
                self.ft_state
                    .mint(fee_receiver, fee)
                    .expect("unchecked overflow occurred for `FTState`");
            }

            let U256PairTuple(balance) = balance.into();

            self.k_last = balance.0 * balance.1;
        }

        self.update(balance);

        Ok(Event::RemovedLiquidity {
            sender: msg_source,
            amount_a: amount.0,
            amount_b: amount.1,
            to,
        })
    }

    async fn skim(&self, stepper: &mut Stepper, to: ActorId) -> Result<Event, Error> {
        let program_id = exec::program_id();
        let contract_balance = self.balances(program_id).await?;

        let (Some(excess_a), Some(excess_b)) = (
            contract_balance.0.checked_sub(self.reserve.0),
            contract_balance.1.checked_sub(self.reserve.1),
        ) else {
            return Err(Error::Overflow);
        };

        utils::transfer_tokens(stepper, self.token.0, program_id, to, excess_a).await?;
        utils::transfer_tokens(stepper, self.token.1, program_id, to, excess_b).await?;

        Ok(Event::Skim {
            amount_a: excess_a,
            amount_b: excess_b,
            to,
        })
    }

    async fn sync(&mut self) -> Result<Event, Error> {
        let program_id = exec::program_id();
        let balance = self.balances(program_id).await?;

        self.update(balance);

        Ok(Event::Sync {
            reserve_a: balance.0,
            reserve_b: balance.1,
        })
    }

    async fn balances(&self, program_id: ActorId) -> Result<(u128, u128)> {
        Ok((
            utils::balance_of(self.token.0, program_id).await?,
            utils::balance_of(self.token.1, program_id).await?,
        ))
    }

    fn update(&mut self, balance: (u128, u128)) {
        let block_ts = exec::block_timestamp();
        let time_elapsed = block_ts - self.last_block_ts;

        if time_elapsed > 0 && self.reserve != (0, 0) {
            let U256PairTuple(reserve) = self.reserve.into();
            let calculate_cp = |reserve: (U256, U256)| {
                // The `u64` suffix is needed for a faster conversion.
                ((reserve.1 << U256::from(128u64)) / reserve.0)
                    // TODO: replace `overflowing_mul` with `wrapping_mul`.
                    // At the moment "primitive-types" doesn't have this method.
                    .overflowing_mul(time_elapsed.into())
                    .0
            };

            self.cumulative_price.0 += calculate_cp(reserve);
            self.cumulative_price.1 += calculate_cp((reserve.1, reserve.0));
        }

        self.reserve = balance;
        self.last_block_ts = block_ts;
    }

    fn swap_pattern(&self, kind: SwapKind) -> SwapPattern {
        match kind {
            SwapKind::AForB => SwapPattern {
                token: self.token,
                reserve: self.reserve,
                normalize_balance: convert::identity,
            },
            SwapKind::BForA => SwapPattern {
                token: (self.token.1, self.token.0),
                reserve: (self.reserve.1, self.reserve.0),
                normalize_balance: |amount| (amount.1, amount.0),
            },
        }
    }

    async fn swap_exact_tokens_for_tokens(
        &mut self,
        stepper: &mut Stepper,
        msg_source: ActorId,
        in_amount: u128,
        min_out_amount: u128,
        to: ActorId,
        kind: SwapKind,
    ) -> Result<Event, Error> {
        self.check_recipient(to)?;

        let swap_pattern = self.swap_pattern(kind);
        let out_amount = calculate_out_amount(in_amount, swap_pattern.reserve)?;

        if out_amount < min_out_amount {
            return Err(Error::InsufficientLatterAmount);
        }

        self.swap(
            stepper,
            msg_source,
            kind,
            (in_amount, out_amount),
            to,
            swap_pattern,
        )
        .await
    }

    fn check_recipient(&self, recipient: ActorId) -> Result<(), Error> {
        if recipient == self.token.0 || recipient == self.token.1 {
            Err(Error::InvalidRecipient)
        } else {
            Ok(())
        }
    }

    async fn swap_tokens_for_exact_tokens(
        &mut self,
        (tx_manager, action_kind): (&mut TransactionManager<CachedAction>, ActionKind),
        msg_source: ActorId,
        out_amount: u128,
        max_in_amount: u128,
        to: ActorId,
        swap_kind: SwapKind,
    ) -> Result<Event, Error> {
        self.check_recipient(to)?;

        let swap_pattern = self.swap_pattern(swap_kind);
        let in_amount = calculate_in_amount(out_amount, swap_pattern.reserve)?;

        let mut tx_guard = tx_manager.acquire_transaction(
            msg_source,
            action_kind.to_tx_kind(CachedAction::Swap(in_amount)),
        )?;

        tx_guard
            .tx_data
            .check_tx_data(|tx_data| tx_data == &CachedAction::Swap(in_amount))?;

        if in_amount > max_in_amount {
            return Err(Error::InsufficientFormerAmount);
        }

        self.swap(
            &mut tx_guard.stepper,
            msg_source,
            swap_kind,
            (in_amount, out_amount),
            to,
            swap_pattern,
        )
        .await
    }

    async fn swap(
        &mut self,
        stepper: &mut Stepper,
        msg_source: ActorId,
        kind: SwapKind,
        (in_amount, out_amount): (u128, u128),
        to: ActorId,
        SwapPattern {
            token: (in_token, out_token),
            reserve,
            normalize_balance,
        }: SwapPattern,
    ) -> Result<Event, Error> {
        let program_id = exec::program_id();

        utils::transfer_tokens(stepper, in_token, msg_source, program_id, in_amount).await?;

        if let Err(error) =
            utils::transfer_tokens(stepper, out_token, program_id, to, out_amount).await
        {
            utils::transfer_tokens(stepper, in_token, program_id, msg_source, in_amount).await?;

            return Err(error);
        }

        self.update(normalize_balance((
            reserve.0 + in_amount,
            reserve.1 - out_amount,
        )));

        Ok(Event::Swap {
            sender: msg_source,
            in_amount,
            out_amount,
            to,
            kind,
        })
    }
}

struct SwapPattern {
    token: (ActorId, ActorId),
    reserve: (u128, u128),
    normalize_balance: fn((u128, u128)) -> (u128, u128),
}

fn check_deadline(deadline: u64) -> Result<(), Error> {
    if exec::block_timestamp() > deadline {
        Err(Error::DeadlineExceeded)
    } else {
        Ok(())
    }
}

#[no_mangle]
extern "C" fn init() {
    let result = process_init();
    let is_err = result.is_err();

    msg::reply(result, 0).expect("failed to encode or reply from `init()`");

    if is_err {
        exec::exit(ActorId::zero());
    }
}

fn process_init() -> Result<(), Error> {
    let Initialize {
        pair: token,
        factory,
    } = msg::load()?;

    if token.0.is_zero() || token.1.is_zero() {
        return Err(Error::ZeroActorId);
    }

    if token.0 == token.1 {
        return Err(Error::IdenticalTokens);
    }

    unsafe {
        STATE = Some((
            Contract {
                token,
                factory,
                ..Default::default()
            },
            TransactionManager::default(),
        ));
    };

    Ok(())
}

#[gstd::async_main]
async fn main() {
    msg::reply(process_handle().await, 0).expect("failed to encode or reply `handle()`");
}

async fn process_handle() -> Result<Event, Error> {
    let Action {
        action,
        kind: action_kind,
    } = msg::load()?;
    let (contract, tx_manager) = state_mut();
    let msg_source = msg::source();

    match action {
        InnerAction::AddLiquidity {
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        } => {
            check_deadline(deadline)?;

            contract
                .add_liquidity(
                    (tx_manager, action_kind),
                    msg_source,
                    (amount_a_desired, amount_b_desired),
                    (amount_a_min, amount_b_min),
                    to,
                )
                .await
        }
        InnerAction::RemoveLiquidity {
            liquidity,
            amount_a_min,
            amount_b_min,
            to,
            deadline,
        } => {
            let mut tx_guard = tx_manager.acquire_transaction(
                msg_source,
                action_kind.to_tx_kind(CachedAction::RemovedLiquidity {
                    amount: liquidity,
                    is_burned: false,
                }),
            )?;

            let is_burned = tx_guard.tx_data.check_and_get_tx_data(|tx_data| {
                if let CachedAction::RemovedLiquidity { amount, is_burned } = tx_data {
                    if *amount == liquidity {
                        Some(is_burned)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })?;

            check_deadline(deadline)?;

            contract
                .remove_liquidity(
                    &mut tx_guard.stepper,
                    is_burned,
                    msg_source,
                    liquidity,
                    (amount_a_min, amount_b_min),
                    to,
                )
                .await
        }
        InnerAction::SwapExactTokensForTokens {
            amount_in,
            amount_out_min,
            to,
            deadline,
            swap_kind,
        } => {
            let mut tx_guard = tx_manager.acquire_transaction(
                msg_source,
                action_kind.to_tx_kind(CachedAction::Swap(amount_in)),
            )?;

            tx_guard
                .tx_data
                .check_tx_data(|tx_data| *tx_data == CachedAction::Swap(amount_in))?;

            check_deadline(deadline)?;

            contract
                .swap_exact_tokens_for_tokens(
                    &mut tx_guard.stepper,
                    msg_source,
                    amount_in,
                    amount_out_min,
                    to,
                    swap_kind,
                )
                .await
        }
        InnerAction::SwapTokensForExactTokens {
            amount_out,
            amount_in_max,
            to,
            deadline,
            swap_kind,
        } => {
            check_deadline(deadline)?;

            contract
                .swap_tokens_for_exact_tokens(
                    (tx_manager, action_kind),
                    msg_source,
                    amount_out,
                    amount_in_max,
                    to,
                    swap_kind,
                )
                .await
        }
        InnerAction::Skim(to) => {
            let mut tx_guard = tx_manager
                .acquire_transaction(msg_source, action_kind.to_tx_kind(CachedAction::Other))?;

            tx_guard
                .tx_data
                .check_tx_data(|tx_data| tx_data == &CachedAction::Other)?;

            contract.skim(&mut tx_guard.stepper, to).await
        }
        InnerAction::Sync => contract.sync().await,
        InnerAction::Transfer { to, amount } => contract
            .ft_state
            .transfer(to, amount)
            .map(Into::into)
            .map_err(Into::into),
    }
}

#[no_mangle]
extern "C" fn state() {
    let (
        Contract {
            factory,

            token,
            reserve,
            cumulative_price,
            last_block_ts,
            k_last,
            ft_state,
        },
        tx_manager,
    ) = state_mut();

    msg::reply(
        State {
            factory: *factory,

            token: *token,
            reserve: *reserve,
            cumulative_price: *cumulative_price,

            last_block_ts: *last_block_ts,
            k_last: *k_last,

            ft_state: ft_state.clone().into(),

            cached_actions: tx_manager
                .cached_transactions()
                .map(|(k, v)| (*k, *v))
                .collect(),
        },
        0,
    )
    .expect("failed to encode or reply from `state()`");
}
