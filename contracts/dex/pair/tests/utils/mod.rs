use common::{InitResult, Program, RunResult, StateReply, TransactionalProgram};
use dex_pair_io::{hidden::U256PairTuple, *};
use dex_pair_state::{WASM_BINARY, WASM_EXPORTS};
use gear_lib::tokens::fungible::FTTransfer;
use gstd::{prelude::*, ActorId};
use gtest::Program as InnerProgram;
use primitive_types::U256;

mod common;
mod factory;
mod fungible_token;

pub mod prelude;

pub use common::initialize_system;
pub use fungible_token::FungibleToken;

pub const FOREIGN_USER: u64 = 1029384756123;
pub const FT_MAIN: &str = "../target/wasm32-unknown-unknown/debug/ft_main.opt.wasm";
pub const FT_STORAGE: &str = "../target/wasm32-unknown-unknown/debug/ft_storage.opt.wasm";
pub const FT_LOGIC: &str = "../target/wasm32-unknown-unknown/debug/ft_logic.opt.wasm";
pub const SPENT_BLOCKS: u32 = 1;

const DEADLINE: u64 = 99999999999999999;

type PairRunResult<T, C = ()> = RunResult<T, C, Event, Error>;

pub struct Pair<'a>(pub InnerProgram<'a>);

impl Program for Pair<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> Pair<'a> {
    pub fn add_liquidity(
        &mut self,
        from: u64,
        desired_amount: (u128, u128),
        min_amount: (u128, u128),
        to: impl Into<ActorId>,
    ) -> PairRunResult<(u64, (u128, u128), u128)> {
        self.add_liquidity_with_deadline(from, desired_amount, min_amount, to, DEADLINE)
    }

    pub fn add_liquidity_with_deadline(
        &mut self,
        from: u64,
        desired_amount: (u128, u128),
        min_amount: (u128, u128),
        to: impl Into<ActorId>,
        deadline: u64,
    ) -> PairRunResult<(u64, (u128, u128), u128)> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::AddLiquidity {
                    amount_a_desired: desired_amount.0,
                    amount_b_desired: desired_amount.1,
                    amount_a_min: min_amount.0,
                    amount_b_min: min_amount.1,
                    to: to.into(),
                    deadline,
                }),
            ),
            |event, (sender, amount, liquidity)| {
                assert_eq!(
                    event,
                    Event::AddedLiquidity {
                        sender: sender.into(),
                        amount_a: amount.0,
                        amount_b: amount.1,
                        liquidity: liquidity.into(),
                    }
                )
            },
        )
    }

    pub fn remove_liquidity(
        &mut self,
        from: u64,
        liquidity: u128,
        amount: (u128, u128),
        to: u64,
    ) -> PairRunResult<(u64, (u128, u128), u64)> {
        self.remove_liquidity_with_deadline(from, liquidity, amount, to, DEADLINE)
    }

    pub fn remove_liquidity_with_deadline(
        &mut self,
        from: u64,
        liquidity: u128,
        amount: (u128, u128),
        to: u64,
        deadline: u64,
    ) -> PairRunResult<(u64, (u128, u128), u64)> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::RemoveLiquidity {
                    liquidity: liquidity.into(),
                    amount_a_min: amount.0,
                    amount_b_min: amount.1,
                    to: to.into(),
                    deadline,
                }),
            ),
            |event, (sender, amount, to)| {
                assert_eq!(
                    event,
                    Event::RemovedLiquidity {
                        sender: sender.into(),
                        amount_a: amount.0,
                        amount_b: amount.1,
                        to: to.into(),
                    }
                )
            },
        )
    }

    pub fn swap_exact_tokens_for_tokens(
        &mut self,
        from: u64,
        amount: (u128, u128),
        to: impl Into<ActorId>,
        swap_kind: SwapKind,
    ) -> PairRunResult<(u64, (u128, u128), u64, SwapKind)> {
        self.swap_exact_tokens_for_tokens_with_deadline(from, amount, to, swap_kind, DEADLINE)
    }

    pub fn swap_exact_tokens_for_tokens_with_deadline(
        &mut self,
        from: u64,
        amount: (u128, u128),
        to: impl Into<ActorId>,
        swap_kind: SwapKind,
        deadline: u64,
    ) -> PairRunResult<(u64, (u128, u128), u64, SwapKind)> {
        self.swap(
            from,
            Action::new(InnerAction::SwapExactTokensForTokens {
                amount_in: amount.0,
                amount_out_min: amount.1,
                to: to.into(),
                deadline,
                swap_kind,
            }),
        )
    }

    fn swap(
        &mut self,
        from: u64,
        action: Action,
    ) -> PairRunResult<(u64, (u128, u128), u64, SwapKind)> {
        RunResult::new(
            self.0.send(from, action),
            |event, (sender, amount, to, kind)| {
                assert_eq!(
                    event,
                    Event::Swap {
                        sender: sender.into(),
                        in_amount: amount.0,
                        out_amount: amount.1,
                        to: to.into(),
                        kind,
                    }
                )
            },
        )
    }

    pub fn swap_tokens_for_exact_tokens(
        &mut self,
        from: u64,
        amount: (u128, u128),
        to: impl Into<ActorId>,
        swap_kind: SwapKind,
    ) -> PairRunResult<(u64, (u128, u128), u64, SwapKind)> {
        self.swap_tokens_for_exact_tokens_with_deadline(from, amount, to, swap_kind, DEADLINE)
    }

    pub fn swap_tokens_for_exact_tokens_with_deadline(
        &mut self,
        from: u64,
        amount: (u128, u128),
        to: impl Into<ActorId>,
        swap_kind: SwapKind,
        deadline: u64,
    ) -> PairRunResult<(u64, (u128, u128), u64, SwapKind)> {
        self.swap(
            from,
            Action::new(InnerAction::SwapTokensForExactTokens {
                amount_out: amount.0,
                amount_in_max: amount.1,
                to: to.into(),
                deadline,
                swap_kind,
            }),
        )
    }

    pub fn sync(&mut self) -> PairRunResult<(u128, u128)> {
        RunResult::new(
            self.0.send(FOREIGN_USER, Action::new(InnerAction::Sync)),
            |event, reserve| {
                assert_eq!(
                    event,
                    Event::Sync {
                        reserve_a: reserve.0,
                        reserve_b: reserve.1,
                    }
                )
            },
        )
    }

    pub fn transfer(
        &mut self,
        from: u64,
        amount: u128,
        to: u64,
    ) -> PairRunResult<(u64, u64, u128)> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Transfer {
                    to: to.into(),
                    amount: amount.into(),
                }),
            ),
            |event, (from, to, amount)| {
                assert_eq!(
                    event,
                    Event::Transfer(FTTransfer {
                        from: from.into(),
                        to: to.into(),
                        amount: amount.into(),
                    })
                )
            },
        )
    }

    pub fn state(&self) -> PairState {
        PairState(&self.0)
    }
}

pub struct PairState<'a>(&'a InnerProgram<'a>);

impl PairState<'_> {
    fn query_state_common<A: Encode, T: Decode>(
        self,
        fn_index: usize,
        argument: Option<A>,
    ) -> StateReply<T> {
        StateReply(
            self.0
                .read_state_using_wasm(WASM_EXPORTS[fn_index], WASM_BINARY.into(), argument)
                .unwrap(),
        )
    }

    fn query_state_with_argument<A: Encode, T: Decode>(
        self,
        fn_index: usize,
        argument: A,
    ) -> StateReply<T> {
        self.query_state_common(fn_index, Some(argument))
    }

    fn query_state<T: Decode>(self, fn_index: usize) -> StateReply<T> {
        self.query_state_common::<(), _>(fn_index, None)
    }

    pub fn token(self) -> StateReply<(ActorId, ActorId)> {
        self.query_state(1)
    }

    pub fn reserve(self) -> StateReply<(u128, u128)> {
        self.query_state(2)
    }

    pub fn price(self) -> StateReply<(U256, U256)> {
        self.query_state(3)
    }

    pub fn balance_of(self, actor: impl Into<ActorId>) -> StateReply<u128> {
        StateReply(
            self.query_state_with_argument::<_, U256>(5, actor.into())
                .0
                .try_into()
                .unwrap(),
        )
    }

    pub fn factory(self) -> StateReply<ActorId> {
        self.query_state(6)
    }

    pub fn calculate_out_amount(
        self,
        swap_kind: SwapKind,
        in_amount: u128,
    ) -> StateReply<Result<u128, Error>> {
        self.query_state_with_argument(9, (swap_kind, in_amount))
    }

    pub fn calculate_in_amount(
        self,
        swap_kind: SwapKind,
        out_amount: u128,
    ) -> StateReply<Result<u128, Error>> {
        self.query_state_with_argument(10, (swap_kind, out_amount))
    }
}

pub fn calculate_cp(reserve: (u128, u128)) -> U256 {
    let U256PairTuple(reserve) = reserve.into();

    ((reserve.1 << U256::from(128u64)) / reserve.0)
        .overflowing_mul((SPENT_BLOCKS * 1000).into())
        .0
}
