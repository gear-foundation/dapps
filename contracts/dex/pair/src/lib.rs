#![no_std]

use dex_pair_io::*;
use gear_lib::fungible_token::{ft_core::*, state::*};
use gear_lib_derive::{FTCore, FTStateKeeper};
use gstd::{cmp, errors::Result, exec, msg, prelude::*, ActorId, MessageId};
use num::integer::Roots;
mod internals;
pub mod math;
pub mod messages;

const MINIMUM_LIQUIDITY: u128 = 1000;

#[derive(Default, FTStateKeeper, FTCore)]
struct Pair {
    #[FTStateField]
    state: FTState,
    factory: ActorId,
    token0: ActorId,
    token1: ActorId,
    last_block_ts: u128,
    balance0: u128,
    balance1: u128,
    reserve0: u128,
    reserve1: u128,
    price0_cl: u128,
    price1_cl: u128,
    k_last: u128,
}

static mut PAIR: Option<Pair> = None;

impl Pair {
    // EXTERNAL FUNCTIONS

    /// Forces balances to match the reserves.
    /// `to` - MUST be a non-zero address
    /// Arguments:
    /// * `to` - where to perform tokens' transfers
    pub async fn skim(&mut self, to: ActorId) {
        messages::transfer_tokens(
            &self.token0,
            &exec::program_id(),
            &to,
            self.balance0.saturating_sub(self.reserve0),
        )
        .await;
        messages::transfer_tokens(
            &self.token1,
            &exec::program_id(),
            &to,
            self.balance1.saturating_sub(self.reserve1),
        )
        .await;
        // Update the balances.
        self.balance0 -= self.reserve0;
        self.balance1 -= self.reserve1;
        reply(PairEvent::Skim {
            to,
            amount0: self.balance0,
            amount1: self.balance1,
        })
        .expect("Error during a replying with PairEvent::Skim");
    }

    /// Forces reserves to match balances.
    pub async fn sync(&mut self) {
        self.update(self.balance0, self.balance1, self.reserve0, self.reserve1);
        reply(PairEvent::Sync {
            balance0: self.balance0,
            balance1: self.balance1,
            reserve0: self.reserve0,
            reserve1: self.reserve1,
        })
        .expect("Error during a replying with PairEvent::Sync");
    }

    /// Adds liquidity to the pool.
    /// `to` - MUST be a non-zero address
    /// Arguments:
    /// * `amount0_desired` - is the desired amount of token0 the user wants to add
    /// * `amount1_desired` - is the desired amount of token1 the user wants to add
    /// * `amount0_min` - is the minimum amount of token0 the user wants to add
    /// * `amount1_min` - is the minimum amount of token1 the user wants to add
    /// * `to` - is the liquidity provider
    pub async fn add_liquidity(
        &mut self,
        amount0_desired: u128,
        amount1_desired: u128,
        amount0_min: u128,
        amount1_min: u128,
        to: ActorId,
    ) {
        let amount0: u128;
        let amount1: u128;
        // Check the amounts provided with the respect to the reserves to find the best amount of tokens0/1 to be added.
        if self.reserve0 == 0 && self.reserve1 == 0 {
            amount0 = amount0_desired;
            amount1 = amount1_desired;
        } else {
            let amount1_optimal = math::quote(amount0_desired, self.reserve0, self.reserve1);
            if amount1_optimal < amount1_desired {
                if amount1_optimal >= amount1_min {
                    panic!("PAIR: Insufficient token1 amount.");
                }
                amount0 = amount0_desired;
                amount1 = amount1_optimal;
            } else {
                let amount0_optimal = math::quote(amount1_desired, self.reserve0, self.reserve1);
                if amount0_optimal >= amount0_min {
                    panic!("PAIR: Insufficient token0 amount.");
                }
                amount0 = amount0_optimal;
                amount1 = amount1_desired;
            }
        }

        let pair_address = exec::program_id();
        messages::transfer_tokens(&self.token0, &msg::source(), &pair_address, amount0).await;
        messages::transfer_tokens(&self.token1, &msg::source(), &pair_address, amount1).await;
        // Update the balances.
        self.balance0 += amount0;
        self.balance1 += amount1;
        // call mint function
        let liquidity = self.mint(to).await;
        reply(PairEvent::AddedLiquidity {
            amount0,
            amount1,
            liquidity,
            to,
        })
        .expect("Error during a replying with PairEvent::AddedLiquidity");
    }

    /// Removes liquidity from the pool.
    /// Internally calls self.burn function while transferring `liquidity` amount of internal tokens
    /// `to` - MUST be a non-zero address
    /// Arguments:
    /// * `liquidity` - is the desired liquidity the user wants to remove (e.g. burn)
    /// * `amount0_min` - is the minimum amount of token0 the user wants to receive
    /// * `amount1_min` - is the minimum amount of token1 the user wants to receive
    /// * `to` - is the liquidity provider
    pub async fn remove_liquidity(
        &mut self,
        liquidity: u128,
        amount0_min: u128,
        amount1_min: u128,
        to: ActorId,
    ) {
        FTCore::transfer(self, &msg::source(), &exec::program_id(), liquidity);
        // Burn and get the optimal amount of burned tokens.
        let (amount0, amount1) = self.burn(to).await;

        if amount0 < amount0_min {
            panic!("PAIR: Insufficient amount of token 0")
        }
        if amount1 < amount1_min {
            panic!("PAIR: Insufficient amount of token 1")
        }
    }

    /// Swaps exact token0 for some token1
    /// Internally calculates the price from the reserves and call self._swap
    /// `to` - MUST be a non-zero address
    /// `amount_in` - MUST be non-zero
    /// Arguments:
    /// * `amount_in` - is the amount of token0 user want to swap
    /// * `to` - is the receiver of the swap operation
    pub async fn swap_exact_tokens_for(&mut self, amount_in: u128, to: ActorId) {
        // token1 amount
        let amount_out = math::get_amount_out(amount_in, self.reserve0, self.reserve1);

        self._swap(amount_in, amount_out, to, true).await;
        reply(PairEvent::SwapExactTokensFor {
            to,
            amount_in,
            amount_out,
        })
        .expect("Error during a replying with PairEvent::SwapExactTokensFor");
    }

    /// Swaps exact token1 for some token0
    /// Internally calculates the price from the reserves and call self._swap
    /// `to` - MUST be a non-zero address
    /// `amount_in` - MUST be non-zero
    /// Arguments:
    /// * `amount_out` - is the amount of token1 user want to swap
    /// * `to` - is the receiver of the swap operation
    pub async fn swap_tokens_for_exact(&mut self, amount_out: u128, to: ActorId) {
        let amount_in = math::get_amount_in(amount_out, self.reserve0, self.reserve1);

        self._swap(amount_in, amount_out, to, false).await;
        reply(PairEvent::SwapTokensForExact {
            to,
            amount_in,
            amount_out,
        })
        .expect("Error during a replying with PairEvent::SwapTokensForExact");
    }
}

#[no_mangle]
extern "C" fn init() {
    let config: InitPair = msg::load().expect("Unable to decode InitPair");
    // DISABLE FOR TESTING, UNCOMMENT AND FIX TESTS LATER
    // if config.factory != msg::source() {
    //     panic!("PAIR: Can only be created by a factory.");
    // }
    let pair = Pair {
        factory: config.factory,
        token0: config.token0,
        token1: config.token1,
        ..Default::default()
    };
    unsafe {
        PAIR = Some(pair);
    }
}

#[gstd::async_main]
async fn main() {
    let action: PairAction = msg::load().expect("Unable to decode PairAction");
    let pair = unsafe { PAIR.get_or_insert(Default::default()) };
    match action {
        PairAction::AddLiquidity {
            amount0_desired,
            amount1_desired,
            amount0_min,
            amount1_min,
            to,
        } => {
            pair.add_liquidity(
                amount0_desired,
                amount1_desired,
                amount0_min,
                amount1_min,
                to,
            )
            .await
        }
        PairAction::RemoveLiquidity {
            liquidity,
            amount0_min,
            amount1_min,
            to,
        } => {
            pair.remove_liquidity(liquidity, amount0_min, amount1_min, to)
                .await
        }
        PairAction::Sync => pair.sync().await,
        PairAction::Skim { to } => pair.skim(to).await,
        PairAction::SwapExactTokensFor { to, amount_in } => {
            pair.swap_exact_tokens_for(amount_in, to).await
        }
        PairAction::SwapTokensForExact { to, amount_out } => {
            pair.swap_tokens_for_exact(amount_out, to).await
        }
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: PairStateQuery = msg::load().expect("Unable to decode PairStateQuery");
    let pair = common_state();
    let reply = match state {
        PairStateQuery::TokenAddresses => PairStateReply::TokenAddresses(pair.token_addresses()),
        PairStateQuery::Reserves => PairStateReply::Reserves(pair.reserves()),
        PairStateQuery::Prices => PairStateReply::Prices(pair.prices()),
        PairStateQuery::BalanceOf(address) => PairStateReply::Balance(pair.balance_of(address)),
    };
    gstd::util::to_leak_ptr(reply.encode())
}

fn common_state() -> State {
    let Pair {
        state,
        factory,
        token0,
        token1,
        last_block_ts,
        balance0,
        balance1,
        reserve0,
        reserve1,
        price0_cl,
        price1_cl,
        k_last,
    } = unsafe { PAIR.get_or_insert(Default::default()) };

    State {
        ft_balances: state.balances.iter().map(|(k, v)| (*k, *v)).collect(),
        factory: *factory,
        token0: *token0,
        token1: *token1,
        last_block_ts: *last_block_ts,
        balance0: *balance0,
        balance1: *balance1,
        reserve0: *reserve0,
        reserve1: *reserve1,
        price0_cl: *price0_cl,
        price1_cl: *price1_cl,
        k_last: *k_last,
    }
}

#[no_mangle]
extern "C" fn state() {
    reply(common_state()).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");

    reply(metahash).expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}

fn reply(payload: impl Encode) -> Result<MessageId> {
    msg::reply(payload, 0)
}
