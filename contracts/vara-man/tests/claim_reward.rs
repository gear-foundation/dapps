mod utils;

use gtest::{Program, System};
use utils::VaraMan;
use vara_man_io::*;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            // 10 tokens per gold coin, with 12 decimals precision
            tokens_per_gold_coin: 10_000_000_000_000,
            // 5 token per silver coin, with 12 decimals precision
            tokens_per_silver_coin: 5_000_000_000_000,
            easy_reward_scale_bps: 0,
            medium_reward_scale_bps: 0,
            hard_reward_scale_bps: 0,
            gold_coins: 5,
            silver_coins: 20,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, false);

    // 60 tokens total, with 12 decimals precision
    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 60_000_000_000_000);
}

#[test]
fn success_reward_scale() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            // 10 tokens per gold coin, with 6 decimals precision
            tokens_per_gold_coin: 10000000,
            // 1 token per silver coin, with 6 decimals precision
            tokens_per_silver_coin: 1000000,
            // Scale rewards to 10%
            easy_reward_scale_bps: 1000,
            medium_reward_scale_bps: 0,
            hard_reward_scale_bps: 0,
            gold_coins: 5,
            silver_coins: 20,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, false);

    // 20 tokens total, with 6 decimals precision
    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 22000000);
}

#[test]
fn fail_rewards_already_claimed() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            // 10 tokens per gold coin, with 6 decimals precision
            tokens_per_gold_coin: 10000000,
            // 1 token per silver coin, with 6 decimals precision
            tokens_per_silver_coin: 1000000,
            easy_reward_scale_bps: 0,
            medium_reward_scale_bps: 0,
            hard_reward_scale_bps: 0,
            gold_coins: 5,
            silver_coins: 20,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, false);

    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 20000000);

    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, true);

    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 20000000);
}

#[test]
fn fail_coin_amount_is_gt_than_allowed() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            // 10 tokens per gold coin, with 6 decimals precision
            tokens_per_gold_coin: 10000000,
            // 1 token per silver coin, with 6 decimals precision
            tokens_per_silver_coin: 1000000,
            easy_reward_scale_bps: 0,
            medium_reward_scale_bps: 0,
            hard_reward_scale_bps: 0,
            gold_coins: 5,
            silver_coins: 20,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10000, 10000, true);
}
