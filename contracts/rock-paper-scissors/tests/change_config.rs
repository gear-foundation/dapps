use gtest::System;
use rock_paper_scissors_io::*;

mod routines;
pub use routines::*;

// check_common
// failure_with_wrong_timouts
// failure_with_not_owners_request
#[test]
fn common() {
    let sys = System::new();
    let game = common_init_and_register(&sys);
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

    println!("{}", sys.block_timestamp());

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Rock],
    );

    println!("{}", sys.block_timestamp());
    check_register_player(&game, USERS[1], 0);
    check_register_player(&game, USERS[2], 0);
    check_register_player(&game, USERS[3], 0);
    failure_register_player(&game, USERS[0], 0);

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT * 2 / 1000));
    failure_user_move(&game, USERS[1], Move::Rock);
    sys.spend_blocks(1);
    check_user_move(&game, USERS[2], Move::Paper);
    check_user_move(&game, USERS[1], Move::Rock);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT * 3 / 1000));
    failure_user_reveal(&game, USERS[1], Move::Rock);
    sys.spend_blocks(1);
    check_user_reveal_with_continue(&game, USERS[1], Move::Rock);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT * 4 / 1000));
    failure_register_player(&game, USERS[0], 0);
    sys.spend_blocks(1);

    check_register_player(&game, USERS[1], 0);
}

// checks that the config doesn't change immediately
#[test]
fn check_round_start() {
    let sys = System::new();
    let game = common_init(&sys);
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

    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);
    check_register_player(&game, USERS[2], COMMON_BET);
    check_register_player(&game, USERS[3], COMMON_BET);
}

#[test]
fn check_two_times() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

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

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Rock],
    );

    check_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 500,
            players_count_limit: 4,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );

    check_register_player(&game, USERS[0], 0);
    check_register_player(&game, USERS[1], 0);
    check_register_player(&game, USERS[2], 0);
    failure_register_player(&game, USERS[3], 0);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT * 2 + 1));

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Rock],
    );

    failure_register_player(&game, USERS[0], 0);
    check_register_player(&game, USERS[0], 30_000_000_000_000);
    check_register_player(&game, USERS[1], 30_000_000_000_000);
    check_register_player(&game, USERS[2], 30_000_000_000_000);
    check_register_player(&game, USERS[3], 30_000_000_000_000);
}

#[test]
fn check_twice_in_a_row() {
    let sys = System::new();
    let game = common_init_and_register(&sys);
    sys.mint_to(USERS[3] + 1, 100_000_000_000_000);
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

    check_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 500,
            players_count_limit: 4,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Rock],
    );

    failure_register_player(&game, USERS[0], 0);
    check_register_player(&game, USERS[0], 30_000_000_000_000);
    check_register_player(&game, USERS[1], 30_000_000_000_000);
    check_register_player(&game, USERS[2], 30_000_000_000_000);
    check_register_player(&game, USERS[3], 30_000_000_000_000);
    failure_register_player(&game, USERS[3] + 1, 30_000_000_000_000);
}

#[test]
fn failure_with_wrong_timouts() {
    let sys = System::new();
    let game = common_init(&sys);

    failure_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 500,
            players_count_limit: 4,
            entry_timeout_ms: 4999,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );
    failure_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 500,
            players_count_limit: 4,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: 4999,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );
    failure_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 500,
            players_count_limit: 4,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: 4999,
        },
    );
}

#[test]
fn failure_with_not_owners_request() {
    let sys = System::new();
    let game = common_init(&sys);

    failure_change_next_game_config(
        &game,
        USERS[1],
        GameConfig {
            bet_size: 500,
            players_count_limit: 4,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );
}

#[test]
fn failure_with_inappropriate_users_limit() {
    let sys = System::new();
    let game = common_init(&sys);

    check_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 500,
            players_count_limit: 2,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );

    failure_change_next_game_config(
        &game,
        USERS[0],
        GameConfig {
            bet_size: 500,
            players_count_limit: 1,
            entry_timeout_ms: COMMON_TIMEOUT,
            move_timeout_ms: COMMON_TIMEOUT,
            reveal_timeout_ms: COMMON_TIMEOUT,
        },
    );
}
