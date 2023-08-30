use gtest::System;
use rock_paper_scissors_io::*;

mod routines;
pub use routines::*;

#[test]
fn common() {
    let sys = System::new();
    let game = common_init(&sys);

    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);
    check_register_player(&game, USERS[2], COMMON_BET);
    check_register_player(&game, USERS[3], COMMON_BET);
}

#[test]
fn check_register_twice() {
    let sys = System::new();
    let game = common_init(&sys);

    check_register_player(&game, USERS[0], COMMON_BET);
    failure_register_player(&game, USERS[0], COMMON_BET);
}

#[test]
fn check_register_after_registration_stage_prolongation() {
    let sys = System::new();
    let game = common_init(&sys);

    check_register_player(&game, USERS[0], COMMON_BET);

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    failure_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    check_user_move(&game, USERS[0], Move::Rock);
    check_user_move(&game, USERS[1], Move::Paper);
}

#[test]
fn check_register_in_progress() {
    let sys = System::new();
    let game = common_init(&sys);

    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);

    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));

    failure_register_player(&game, USERS[0], COMMON_BET);
    failure_register_player(&game, USERS[1], COMMON_BET);
    failure_register_player(&game, USERS[2], COMMON_BET);
}

#[test]
fn check_register_on_reveal_stage() {
    let sys = System::new();
    let game =
        reach_reveal_stage_with_init(&sys, &USERS[0..3], &[Move::Rock, Move::Rock, Move::Rock]);

    failure_register_player(&game, USERS[3], COMMON_BET);
}

#[test]
fn check_register_after_first_round() {
    let sys = System::new();
    let game = common_init(&sys);
    register_players(&game, &USERS[0..3], COMMON_BET);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));
    play_round(&game, &USERS[0..3], &[Move::Rock, Move::Rock, Move::Rock]);

    failure_register_player(&game, USERS[3], COMMON_BET);
}

#[test]
fn check_register_after_game() {
    let sys = System::new();
    let game = common_init(&sys);
    register_players(&game, &USERS[0..3], COMMON_BET);
    sys.spend_blocks(blocks_count(COMMON_TIMEOUT + 1));
    play_round(&game, &USERS[0..3], &[Move::Paper, Move::Rock, Move::Rock]);

    check_register_player(&game, USERS[0], COMMON_BET);
    check_register_player(&game, USERS[1], COMMON_BET);
    check_register_player(&game, USERS[2], COMMON_BET);
    check_register_player(&game, USERS[3], COMMON_BET);
}

#[test]
fn check_register_more_then_possible() {
    let sys = System::new();
    let game = common_init(&sys);

    sys.mint_to(USERS[3] + 1, 1_000_000_000);
    sys.mint_to(USERS[3] + 2, 1_000_000_000);

    register_players(&game, &USERS[0..4], COMMON_BET);
    check_register_player(&game, USERS[3] + 1, COMMON_BET);
    failure_register_player(&game, USERS[3] + 2, COMMON_BET);
}
