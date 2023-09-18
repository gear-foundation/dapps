mod utils;

use gtest::{Program, System};
use utils::VaraMan;
use vara_man_io::{Config, Level, Status};

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            one_coin_in_value: 1_000_000_000_000,
            tokens_per_gold_coin_easy: 5,
            tokens_per_silver_coin_easy: 1,
            tokens_per_gold_coin_medium: 8,
            tokens_per_silver_coin_medium: 2,
            tokens_per_gold_coin_hard: 10,
            tokens_per_silver_coin_hard: 3,
            gold_coins: 5,
            silver_coins: 20,
            number_of_lives: 3,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    // let state = vara_man.get_state();
    // assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, false);

    // 15 tokens total, with 12 decimals precision
    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 15_000_000_000_000);
}

#[test]
fn success_reward_scale() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            one_coin_in_value: 1_000_000_000_000,
            tokens_per_gold_coin_easy: 5,
            tokens_per_silver_coin_easy: 1,
            tokens_per_gold_coin_medium: 8,
            tokens_per_silver_coin_medium: 2,
            tokens_per_gold_coin_hard: 10,
            tokens_per_silver_coin_hard: 3,
            gold_coins: 5,
            silver_coins: 20,
            number_of_lives: 3,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    // let state = vara_man.get_state();
    // assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, false);

    // 15 tokens total, with 12 decimals precision
    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 15_000_000_000_000);
}

#[test]
fn fail_rewards_already_claimed() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            one_coin_in_value: 1_000_000_000_000,
            tokens_per_gold_coin_easy: 5,
            tokens_per_silver_coin_easy: 1,
            tokens_per_gold_coin_medium: 8,
            tokens_per_silver_coin_medium: 2,
            tokens_per_gold_coin_hard: 10,
            tokens_per_silver_coin_hard: 3,
            gold_coins: 5,
            silver_coins: 20,
            number_of_lives: 3,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    // let state = vara_man.get_state();
    // assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, false);

    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 15_000_000_000_000);

    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, true);

    system.claim_value_from_mailbox(utils::PLAYERS[0]);
    assert_eq!(system.balance_of(utils::PLAYERS[0]), 15_000_000_000_000);
}

#[test]
fn fail_coin_amount_is_gt_than_allowed() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man_with_config(
        &system,
        Config {
            operator: utils::ADMIN.into(),
            one_coin_in_value: 1_000_000_000_000,
            tokens_per_gold_coin_easy: 5,
            tokens_per_silver_coin_easy: 1,
            tokens_per_gold_coin_medium: 8,
            tokens_per_silver_coin_medium: 2,
            tokens_per_gold_coin_hard: 10,
            tokens_per_silver_coin_hard: 3,
            gold_coins: 5,
            silver_coins: 20,
            number_of_lives: 3,
        },
    );

    system.mint_to(utils::VARA_MAN_ID, utils::VARA_MAN_FUND);

    vara_man.change_status(Status::Started);

    // let state = vara_man.get_state();
    // assert!(state.players.is_empty() && state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, false);

    assert_eq!(system.balance_of(utils::PLAYERS[0]), 0);
    vara_man.claim_reward(utils::PLAYERS[0], 10000, 10000, true);
}
