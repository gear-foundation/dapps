mod utils;

use gtest::{Program, System};
use utils::{FToken, VaraMan};
use vara_man_io::{Config, Level, Status};

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let ft = Program::ftoken(&system);
    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            reward_token_id: utils::FT_ID.into(),
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

    ft.mint(0, utils::ADMIN, utils::VARA_MAN_ID, 100_000_000);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    system.spend_blocks(301);

    ft.check_balance(utils::PLAYERS[0], 0);
    vara_man.claim_reward(utils::PLAYERS[0], 0, 10, 1, false);
    // 20 tokens total, with 6 decimals precision
    ft.check_balance(utils::PLAYERS[0], 20000000);
}

#[test]
fn success_reward_scale() {
    let system = System::new();
    system.init_logger();

    let ft = Program::ftoken(&system);
    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            reward_token_id: utils::FT_ID.into(),
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

    ft.mint(0, utils::ADMIN, utils::VARA_MAN_ID, 100_000_000);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    system.spend_blocks(301);

    ft.check_balance(utils::PLAYERS[0], 0);
    vara_man.claim_reward(utils::PLAYERS[0], 0, 10, 1, false);
    // 20 tokens total, with 6 decimals precision
    ft.check_balance(utils::PLAYERS[0], 22000000);
}

#[test]
fn fail_rewards_already_claimed() {
    let system = System::new();
    system.init_logger();

    let ft = Program::ftoken(&system);
    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            reward_token_id: utils::FT_ID.into(),
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

    ft.mint(0, utils::ADMIN, utils::VARA_MAN_ID, 100_000_000);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    system.spend_blocks(301);

    ft.check_balance(utils::PLAYERS[0], 0);

    vara_man.claim_reward(utils::PLAYERS[0], 0, 10, 1, false);
    ft.check_balance(utils::PLAYERS[0], 20000000);

    vara_man.claim_reward(utils::PLAYERS[0], 0, 10, 1, true);
    ft.check_balance(utils::PLAYERS[0], 20000000);
}

#[test]
fn fail_coin_amount_is_gt_than_allowed() {
    let system = System::new();
    system.init_logger();

    let ft = Program::ftoken(&system);
    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            reward_token_id: utils::FT_ID.into(),
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

    ft.mint(0, utils::ADMIN, utils::VARA_MAN_ID, 100_000_000);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    system.spend_blocks(301);

    ft.check_balance(utils::PLAYERS[0], 0);
    vara_man.claim_reward(utils::PLAYERS[0], 0, 10000, 10000, true);
}

#[test]
fn fail_game_is_not_ended() {
    let system = System::new();
    system.init_logger();

    let ft = Program::ftoken(&system);
    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            reward_token_id: utils::FT_ID.into(),
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

    ft.mint(0, utils::ADMIN, utils::VARA_MAN_ID, 100_000_000);

    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);
    vara_man.claim_reward(utils::PLAYERS[0], 0, 10, 1, true);
}
