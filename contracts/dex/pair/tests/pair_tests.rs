use dex_pair_io::*;
use ft_io::*;
use gear_lib::fungible_token::io::*;
use gstd::{prelude::*, ActorId};
use gtest::Program;
use gtest::System;
mod utils;

pub const USER: u64 = 10;
pub const FT_USER: u64 = 11;
pub const FEE_SETTER: u64 = 12;
pub const TOKEN_0_ID: u64 = 111;
pub const TOKEN_0_AMOUNT: u128 = 10000;
pub const TOKEN_1_ID: u64 = 112;
pub const TOKEN_1_AMOUNT: u128 = 20000;
pub const FACTORY_ID: u64 = 1;
pub const PAIR_ID: u64 = 2;
// liquidity params
pub const TOKEN_0_LIQ: u128 = 1000;
pub const TOKEN_1_LIQ: u128 = 1000;
pub const LIQUIDITY: u128 = 2000;

fn pre_test(sys: &System, token0_id: u64, token1_id: u64) -> Program {
    let _factory = utils::init_factory(sys, USER, FEE_SETTER);

    // MINT TOKEN0
    let token0 = utils::init_ft(
        sys,
        USER,
        String::from("TOKEN0"),
        String::from("TK0"),
        token0_id,
    );
    let res = utils::mint_ft(&token0, USER, TOKEN_0_AMOUNT);
    assert!(res.contains(&(
        USER,
        FTEvent::Transfer {
            from: ActorId::zero(),
            to: ActorId::from(USER),
            amount: TOKEN_0_AMOUNT,
        }
        .encode()
    )));

    let res = utils::approve_ft(&token0, USER, ActorId::from(USER), TOKEN_0_AMOUNT);
    assert!(res.contains(&(
        USER,
        FTEvent::Approve {
            from: ActorId::from(USER),
            to: ActorId::from(USER),
            amount: TOKEN_0_AMOUNT,
        }
        .encode()
    )));

    utils::check_ft_balance(&token0, USER, ActorId::from(USER), TOKEN_0_AMOUNT);

    // MINT TOKEN1
    let token1 = utils::init_ft(
        sys,
        USER,
        String::from("TOKEN1"),
        String::from("TK1"),
        token1_id,
    );

    let res = utils::mint_ft(&token1, USER, TOKEN_1_AMOUNT);
    assert!(res.contains(&(
        USER,
        FTEvent::Transfer {
            from: ActorId::zero(),
            to: ActorId::from(USER),
            amount: TOKEN_1_AMOUNT,
        }
        .encode()
    )));

    let res = utils::approve_ft(&token1, USER, ActorId::from(USER), TOKEN_1_AMOUNT);
    assert!(res.contains(&(
        USER,
        FTEvent::Approve {
            from: ActorId::from(USER),
            to: ActorId::from(USER),
            amount: TOKEN_1_AMOUNT,
        }
        .encode()
    )));

    utils::check_ft_balance(&token1, USER, ActorId::from(USER), TOKEN_1_AMOUNT);

    utils::init_pair(sys, USER, 1.into(), token0_id.into(), token1_id.into())
}

fn pre_test_add_liquidity(sys: &System) -> Program {
    let pair = pre_test(sys, TOKEN_0_ID, TOKEN_1_ID);
    let res = utils::add_liquidity(
        &pair,
        USER,
        TOKEN_0_LIQ,
        TOKEN_1_LIQ,
        TOKEN_0_LIQ,
        TOKEN_1_LIQ,
        ActorId::from(USER),
    );
    assert!(res.contains(&(
        USER,
        PairEvent::AddedLiquidity {
            amount0: TOKEN_0_LIQ,
            amount1: TOKEN_1_LIQ,
            liquidity: LIQUIDITY,
            to: ActorId::from(USER),
        }
        .encode()
    )));

    // check that we actually minted liquidity tokens to the pair contract
    // by checking pair_balance and tokens' balances for a user
    utils::check_pair_balance(&pair, ActorId::from(USER), LIQUIDITY);
    let token0 = sys.get_program(TOKEN_0_ID);
    utils::check_ft_balance(
        &token0,
        USER,
        ActorId::from(USER),
        TOKEN_0_AMOUNT - TOKEN_0_LIQ,
    );
    utils::check_ft_balance(&token0, USER, ActorId::from(PAIR_ID), TOKEN_0_LIQ);

    let token1 = sys.get_program(TOKEN_1_ID);
    utils::check_ft_balance(
        &token1,
        USER,
        ActorId::from(USER),
        TOKEN_1_AMOUNT - TOKEN_1_LIQ,
    );
    utils::check_ft_balance(&token1, USER, ActorId::from(PAIR_ID), TOKEN_1_LIQ);

    // check reserves
    utils::check_reserves(&pair, TOKEN_0_LIQ, TOKEN_1_LIQ);
    pair
}
// add_liquidity
#[test]
fn add_liquidity() {
    let sys = System::new();
    sys.init_logger();
    pre_test_add_liquidity(&sys);
}

// add_liquidity_failures
#[test]
fn add_liquidity_failures() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test(&sys, TOKEN_0_ID, TOKEN_1_ID);
    // MUST fail, not enough token0 for a USER
    let res = utils::add_liquidity(
        &pair,
        USER,
        TOKEN_0_AMOUNT + 1,
        TOKEN_1_LIQ,
        TOKEN_0_AMOUNT + 1,
        TOKEN_1_LIQ,
        ActorId::from(USER),
    );
    assert!(res.main_failed());
    // MUST fail, not enough token1 for a USER
    let res = utils::add_liquidity(
        &pair,
        USER,
        TOKEN_0_LIQ,
        TOKEN_1_AMOUNT + 1,
        TOKEN_0_LIQ,
        TOKEN_1_AMOUNT + 1,
        ActorId::from(USER),
    );
    assert!(res.main_failed());

    // MUST fail because of zero liquidity
    utils::add_liquidity(
        &pair,
        USER,
        TOKEN_0_LIQ,
        TOKEN_1_LIQ,
        TOKEN_0_LIQ,
        TOKEN_1_LIQ,
        ActorId::from(USER),
    );
    let res = utils::add_liquidity(&pair, USER, 0, 0, 0, 0, ActorId::from(USER));
    assert!(res.main_failed());
}

// remove_liquidity
#[test]
fn remove_liquidity() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);
    let res = utils::remove_liquidity(&pair, USER, 1000, 100, 100, ActorId::from(USER));
    assert!(res.contains(&(
        USER,
        FTTransfer {
            from: ActorId::from(USER),
            to: ActorId::from(PAIR_ID),
            amount: 1000,
        }
        .encode()
    )));

    // check pair balance
    utils::check_pair_balance(&pair, ActorId::from(PAIR_ID), 1000);

    // check reserves
    utils::check_reserves(&pair, TOKEN_0_LIQ - 250, TOKEN_1_LIQ - 250);

    // check user balances
    let token0 = sys.get_program(TOKEN_0_ID);
    utils::check_ft_balance(
        &token0,
        USER,
        ActorId::from(USER),
        TOKEN_0_AMOUNT - TOKEN_0_LIQ + 250,
    );

    let token1 = sys.get_program(TOKEN_1_ID);
    utils::check_ft_balance(
        &token1,
        USER,
        ActorId::from(USER),
        TOKEN_1_AMOUNT - TOKEN_1_LIQ + 250,
    );
}

// remove_liquidity_failures
#[test]
fn remove_liquidity_failures() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);

    // MUST fail since not enough token0
    let res = utils::remove_liquidity(&pair, USER, 1000, 251, 100, ActorId::from(USER));
    assert!(res.main_failed());

    // MUST fail since USER has insufficient tokens
    let res = utils::remove_liquidity(&pair, USER, 1000, 100, 251, ActorId::from(USER));
    assert!(res.main_failed());
}

// sync
#[test]
fn sync() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);
    let res = utils::sync(&pair, USER);
    assert!(res.contains(&(
        USER,
        PairEvent::Sync {
            balance0: TOKEN_0_LIQ,
            balance1: TOKEN_1_LIQ,
            reserve0: TOKEN_0_LIQ,
            reserve1: TOKEN_1_LIQ,
        }
        .encode()
    )));
}

// skim
#[test]
fn skim() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);
    let res = utils::skim(&pair, USER, ActorId::from(USER));
    assert!(res.contains(&(
        USER,
        PairEvent::Skim {
            to: ActorId::from(USER),
            amount0: 0,
            amount1: 0,
        }
        .encode()
    )));
}

// swap_exact_for
#[test]
fn swap_exact_for() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);

    let res = utils::swap_exact_for(&pair, USER, ActorId::from(USER), 110);
    assert!(res.contains(&(
        USER,
        PairEvent::SwapExactTokensFor {
            to: ActorId::from(USER),
            amount_in: 110,
            amount_out: 97,
        }
        .encode()
    )));

    // check all the balances, considering that's a forward trade
    let token0 = sys.get_program(TOKEN_0_ID);
    // user should have -110 token0, program should have +110 token0
    utils::check_ft_balance(
        &token0,
        USER,
        ActorId::from(USER),
        TOKEN_0_AMOUNT - TOKEN_0_LIQ - 110,
    );
    utils::check_ft_balance(&token0, USER, ActorId::from(PAIR_ID), TOKEN_0_LIQ + 110);

    // user should have +97 tokens, program should have -97 tokens
    let token1 = sys.get_program(TOKEN_1_ID);
    utils::check_ft_balance(
        &token1,
        USER,
        ActorId::from(USER),
        TOKEN_1_AMOUNT - TOKEN_1_LIQ + 97,
    );
    utils::check_ft_balance(&token1, USER, ActorId::from(PAIR_ID), TOKEN_1_LIQ - 97);

    // check reserves after all
    utils::check_reserves(&pair, TOKEN_0_LIQ + 110, TOKEN_1_LIQ - 97);
}

// swap_exact_for_failures

#[test]
fn swap_exact_for_failures() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);

    // MUST fail since the amount_in is 0
    let res = utils::swap_exact_for(&pair, USER, ActorId::from(USER), 0);
    assert!(res.main_failed());

    // MUST fail since we're trading more than there is
    let res = utils::swap_exact_for(&pair, USER, ActorId::from(USER), TOKEN_0_LIQ + 100);
    assert!(res.main_failed());

    // MUST fail since the reserve is 0
    let pair_no_liq = pre_test(&sys, TOKEN_0_ID + 100, TOKEN_1_ID + 100);
    let res = utils::swap_exact_for(&pair_no_liq, USER, ActorId::from(USER), 100);
    assert!(res.main_failed());
}

// swap_for_exact
#[test]
fn swap_for_exact() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);

    // should be 110, since the k is exactly the same an in trading forward
    let res = utils::swap_for_exact(&pair, USER, ActorId::from(USER), 97);
    assert!(res.contains(&(
        USER,
        PairEvent::SwapTokensForExact {
            to: ActorId::from(USER),
            amount_in: 110,
            amount_out: 97,
        }
        .encode()
    )));

    // check all the balances, considering that's a forward trade
    let token0 = sys.get_program(TOKEN_0_ID);
    // user should have +110 token0, program should have -110 token0
    utils::check_ft_balance(
        &token0,
        USER,
        ActorId::from(USER),
        TOKEN_0_AMOUNT - TOKEN_0_LIQ + 110,
    );
    utils::check_ft_balance(&token0, USER, ActorId::from(PAIR_ID), TOKEN_0_LIQ - 110);

    // user should have -97 tokens, program should have +97 tokens
    let token1 = sys.get_program(TOKEN_1_ID);
    utils::check_ft_balance(
        &token1,
        USER,
        ActorId::from(USER),
        TOKEN_1_AMOUNT - TOKEN_1_LIQ - 97,
    );
    utils::check_ft_balance(&token1, USER, ActorId::from(PAIR_ID), TOKEN_1_LIQ + 97);

    // check reserves after all
    utils::check_reserves(&pair, TOKEN_0_LIQ - 110, TOKEN_1_LIQ + 97);
}

// swap_for_exact_failures
#[test]
fn swap_for_exact_failures() {
    let sys = System::new();
    sys.init_logger();
    let pair = pre_test_add_liquidity(&sys);

    // MUST fail since the amount_in is 0
    let res = utils::swap_for_exact(&pair, USER, ActorId::from(USER), 0);
    assert!(res.main_failed());

    // MUST fail since we're trading more than there is
    let res = utils::swap_for_exact(&pair, USER, ActorId::from(USER), TOKEN_0_LIQ + 100);
    assert!(res.main_failed());

    // MUST fail since the reserve is 0
    let pair_no_liq = pre_test(&sys, TOKEN_0_ID + 100, TOKEN_1_ID + 100);
    let res = utils::swap_for_exact(&pair_no_liq, USER, ActorId::from(USER), 100);
    assert!(res.main_failed());
}
