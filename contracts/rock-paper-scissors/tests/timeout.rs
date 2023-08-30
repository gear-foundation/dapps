use gtest::System;
use rock_paper_scissors_io::*;

mod routines;
pub use routines::*;

#[test]
fn timeout_on_preparation_without_users() {
    let sys = System::new();
    let game = common_init(&sys);

    check_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 0,
            players_count_limit: 2,
            entry_timeout_ms: COMMON_TIMEOUT * 2,
            move_timeout_ms: COMMON_TIMEOUT * 3,
            reveal_timeout_ms: COMMON_TIMEOUT * 4,
        },
    );

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);
    check_register_player(&game, USERS[2], COMMON_BET);
    check_register_player(&game, USERS[3], COMMON_BET);
}

#[test]
fn timeout_on_preparation_with_one_users() {
    let sys = System::new();
    let game = common_init(&sys);

    check_register_player(&game, USERS[0], COMMON_BET);

    check_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 0,
            players_count_limit: 2,
            entry_timeout_ms: COMMON_TIMEOUT * 2,
            move_timeout_ms: COMMON_TIMEOUT * 3,
            reveal_timeout_ms: COMMON_TIMEOUT * 4,
        },
    );

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    failure_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);
    check_register_player(&game, USERS[2], COMMON_BET);
    check_register_player(&game, USERS[3], COMMON_BET);
}

#[test]
fn timeout_on_preparation_with_two_users() {
    let sys = System::new();
    let game = common_init(&sys);

    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);

    check_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 0,
            players_count_limit: 3,
            entry_timeout_ms: COMMON_TIMEOUT * 2,
            move_timeout_ms: COMMON_TIMEOUT * 3,
            reveal_timeout_ms: COMMON_TIMEOUT * 4,
        },
    );

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    failure_register_player(&game, USERS[2], COMMON_BET);
    failure_register_player(&game, USERS[3], COMMON_BET);
    check_user_move(&game, USERS[0], Move::Rock);
}

#[test]
fn timeout_on_move_stage_without_users() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    failure_register_player(&game, USERS[3], COMMON_BET);

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    check_user_move(&game, USERS[0], Move::Rock);
    check_user_move(&game, USERS[1], Move::Rock);
    check_user_move(&game, USERS[2], Move::Rock);
}

#[test]
fn timeout_on_move_stage_with_one_users() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    check_user_move(&game, USERS[0], Move::Rock);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);
}

#[test]
fn timeout_on_move_stage_with_two_users() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    check_user_move(&game, USERS[0], Move::Rock);
    check_user_move(&game, USERS[1], Move::Scissors);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    check_user_reveal_with_continue(&game, USERS[0], Move::Rock);
    check_user_reveal_with_game_over(&game, USERS[1], Move::Scissors, USERS[0].into());
}

#[test]
fn timeout_on_reveal_without_users() {
    let sys = System::new();
    let game = reach_reveal_stage_with_init(
        &sys,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Rock, Move::Rock],
    );

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    check_user_reveal_with_continue(&game, USERS[0], Move::Rock);
    check_user_reveal_with_continue(&game, USERS[2], Move::Rock);
}

#[test]
fn timeout_on_reveal_with_one_users() {
    let sys = System::new();
    let game = reach_reveal_stage_with_init(
        &sys,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Rock, Move::Rock],
    );

    check_user_reveal_with_continue(&game, USERS[0], Move::Rock);

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    failure_user_reveal(&game, USERS[1], Move::Rock);
    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);
    check_register_player(&game, USERS[2], COMMON_BET);
}

#[test]
fn timeout_on_reveal_with_two_users() {
    let sys = System::new();
    let game = reach_reveal_stage_with_init(
        &sys,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Rock, Move::Rock],
    );

    check_user_reveal_with_continue(&game, USERS[0], Move::Rock);
    check_user_reveal_with_continue(&game, USERS[1], Move::Rock);

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    failure_user_reveal(&game, USERS[2], Move::Rock);

    check_user_move(&game, USERS[0], Move::Rock);
    check_user_move(&game, USERS[1], Move::Scissors);
    failure_user_move(&game, USERS[2], Move::Scissors);
}
