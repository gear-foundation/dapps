use gstd::prelude::*;
use gtest::System;
use rock_paper_scissors_io::*;

mod routines;
pub use routines::*;

#[test]
fn check_during_the_first_round() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Scissors, Move::Rock];

    let game = init_and_register_with_users(&sys, USERS);

    check_user_move(&game, USERS[0], moves[0].clone());
    check_user_move(&game, USERS[2], moves[2].clone());

    check_stop_the_game(&game, USERS[0], USERS);

    // USERS
    //     .iter()
    //     .for_each(|user| sys.claim_value_from_mailbox(*user));

    USERS
        .iter()
        .for_each(|user| check_users_balance(&sys, user, START_BALANCE))
}

#[test]
fn check_during_reveal_in_first_round() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Scissors, Move::Rock];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    check_stop_the_game(&game, USERS[0], USERS);
}

#[test]
fn check_during_reveal_in_first_round_with_some_reveals() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Scissors, Move::Rock];
    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);
    check_user_reveal_with_continue(&game, USERS[1], moves[1].clone());
    check_user_reveal_with_continue(&game, USERS[3], moves[3].clone());

    check_stop_the_game(&game, USERS[0], USERS);
}

#[test]
fn check_all_players_in_start_of_second_round() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Scissors, Move::Rock];
    let game = init_and_register_with_users(&sys, USERS);
    play_round(&game, USERS, &moves);

    check_stop_the_game(&game, USERS[0], USERS);
}

#[test]
fn check_all_players_in_progress_of_second_round() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Scissors, Move::Rock];
    let game = init_and_register_with_users(&sys, USERS);
    play_round(&game, USERS, &moves);
    check_user_move(&game, USERS[0], moves[0].clone());
    check_user_move(&game, USERS[2], moves[2].clone());

    check_stop_the_game(&game, USERS[0], USERS);
}

#[test]
fn check_not_all_players_in_progress_of_second_round() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Lizard, Move::Lizard];
    let game = init_and_register_with_users(&sys, USERS);
    play_round(&game, USERS, &moves);
    check_user_move(&game, USERS[0], moves[0].clone());
    check_user_move(&game, USERS[2], moves[2].clone());

    let rewarding_users = [USERS[0], USERS[2], USERS[3]];
    check_stop_the_game(&game, USERS[0], &rewarding_users);

    // USERS
    //     .iter()
    //     .for_each(|user| sys.claim_value_from_mailbox(*user));

    rewarding_users.iter().for_each(|user| {
        check_users_balance(
            &sys,
            user,
            (START_BALANCE - COMMON_BET) + (COMMON_BET * 4 / 3),
        )
    });
}

#[test]
fn check_game_is_not_in_progress() {
    let sys = System::new();
    let game = common_init_with_owner_and_bet(&sys, USERS[0], COMMON_BET);
    register_players(&game, USERS, COMMON_BET);
    check_stop_the_game(&game, USERS[0], USERS);
}

#[test]
fn check_not_owner_stop() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Lizard, Move::Lizard];
    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    failure_stop_the_game(&game, USERS[1]);
}
