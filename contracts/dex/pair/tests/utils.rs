use dex_factory_io::*;
use dex_pair_io::FungibleId;
use dex_pair_io::*;
use ft_io::*;
use gstd::{prelude::*, ActorId};
use gtest::{Program, RunResult, System};

pub fn init_factory(sys: &System, user: u64, fee_setter: u64) -> Program {
    sys.init_logger();
    let factory = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/release/dex_factory.wasm",
    );
    assert!(factory
        .send(
            user,
            InitFactory {
                fee_to_setter: ActorId::from(fee_setter),
                pair_code_hash: [0; 32],
            },
        )
        .log()
        .is_empty());

    factory
}

pub fn init_ft(sys: &System, user: u64, name: String, symbol: String, id: u64) -> Program {
    sys.init_logger();
    let ft_program = Program::from_file_with_id(sys, id, "../target/fungible_token.wasm");
    assert!(ft_program
        .send(
            user,
            InitConfig {
                name,
                symbol,
                decimals: 18
            },
        )
        .log()
        .is_empty());

    ft_program
}

pub fn init_pair(
    sys: &System,
    user: u64,
    factory: ActorId,
    token0: FungibleId,
    token1: FungibleId,
) -> Program {
    sys.init_logger();
    let pair_program = Program::current(sys);
    assert!(pair_program
        .send(
            user,
            InitPair {
                factory,
                token0,
                token1
            },
        )
        .log()
        .is_empty());

    pair_program
}

pub fn mint_ft(token: &Program, user: u64, amount: u128) -> RunResult {
    token.send(user, FTAction::Mint(amount))
}

pub fn approve_ft(token: &Program, user: u64, to: ActorId, amount: u128) -> RunResult {
    token.send(user, FTAction::Approve { to, amount })
}

pub fn sync(pair: &Program, user: u64) -> RunResult {
    pair.send(user, PairAction::Sync)
}

pub fn skim(pair: &Program, user: u64, to: ActorId) -> RunResult {
    pair.send(user, PairAction::Skim { to })
}

pub fn add_liquidity(
    pair: &Program,
    user: u64,
    amount0_desired: u128,
    amount1_desired: u128,
    amount0_min: u128,
    amount1_min: u128,
    to: ActorId,
) -> RunResult {
    pair.send(
        user,
        PairAction::AddLiquidity {
            amount0_desired,
            amount1_desired,
            amount0_min,
            amount1_min,
            to,
        },
    )
}

pub fn remove_liquidity(
    pair: &Program,
    user: u64,
    liquidity: u128,
    amount0_min: u128,
    amount1_min: u128,
    to: ActorId,
) -> RunResult {
    pair.send(
        user,
        PairAction::RemoveLiquidity {
            liquidity,
            amount0_min,
            amount1_min,
            to,
        },
    )
}

pub fn swap_exact_for(pair: &Program, user: u64, to: ActorId, amount_in: u128) -> RunResult {
    pair.send(user, PairAction::SwapExactTokensFor { to, amount_in })
}

pub fn swap_for_exact(pair: &Program, user: u64, to: ActorId, amount_out: u128) -> RunResult {
    pair.send(user, PairAction::SwapTokensForExact { to, amount_out })
}

pub fn check_reserves(pair: &Program, reserve0: u128, reserve1: u128) {
    match pair.meta_state(PairStateQuery::Reserves) {
        gstd::Ok(PairStateReply::Reserves {
            reserve0: true_reserve0,
            reserve1: true_reserve1,
        }) => {
            if reserve0 != true_reserve0 || reserve1 != true_reserve1 {
                panic!("PAIR: Actual reserves differ");
            }
        }
        _ => {
            unreachable!(
                "Unreachable metastate reply for the PairStateQuery::Reserves payload has occured"
            )
        }
    }
}

pub fn check_ft_balance(token: &Program, user: u64, address: ActorId, balance: u128) {
    let res = token.send(user, FTAction::BalanceOf(address));
    println!("{:?}", res.decoded_log::<FTEvent>());
    assert!(res.contains(&(user, FTEvent::Balance(balance).encode())));
}

pub fn check_pair_balance(pair: &Program, address: ActorId, balance: u128) {
    match pair.meta_state(PairStateQuery::BalanceOf(address)) {
        gstd::Ok(PairStateReply::Balance(true_balance)) => {
            if true_balance != balance {
                panic!("PAIR: Actual balances differ");
            }
        }
        _ => {
            unreachable!(
                "Unreachable metastate reply for the PairStateQuery::Balance payload has occured"
            )
        }
    }
}
