use gstd::{prelude::*, ActorId};
use gtest::System;
use rps_io::*;

mod routines;
pub use routines::*;

#[test]
fn common_check() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Paper, Move::Scissors, Move::Rock];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    let next_round_players: BTreeSet<ActorId> = USERS.iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[3], moves[3].clone(), next_round_players);
}

#[test]
fn check_game_over() {
    let sys = System::new();
    let moves = [Move::Spock, Move::Lizard, Move::Spock, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    check_user_reveal_with_game_over(&game, USERS[3], moves[3].clone(), USERS[1].into());

    USERS
        .iter()
        .for_each(|user| sys.claim_value_from_mailbox(*user));

    check_users_balance(&sys, &USERS[1], 1_000_000_000 + COMMON_BET * 3);
}

#[test]
fn check_paper_paper_pair_winner() {
    let sys = System::new();
    let moves = [Move::Paper, Move::Paper];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    let next_round_players: BTreeSet<ActorId> =
        USERS[0..2].iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[1], moves[1].clone(), next_round_players);
}

#[test]
fn check_rock_rock_pair_winner() {
    let sys = System::new();
    let moves = [Move::Rock, Move::Rock];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    let next_round_players: BTreeSet<ActorId> =
        USERS[0..2].iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[1], moves[1].clone(), next_round_players);
}

#[test]
fn check_scissors_scissors_pair_winner() {
    let sys = System::new();
    let moves = [Move::Scissors, Move::Scissors];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    let next_round_players: BTreeSet<ActorId> =
        USERS[0..2].iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[1], moves[1].clone(), next_round_players);
}

#[test]
fn check_lizard_lizard_pair_winner() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Lizard];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    let next_round_players: BTreeSet<ActorId> =
        USERS[0..2].iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[1], moves[1].clone(), next_round_players);
}

#[test]
fn check_spock_spock_pair_winner() {
    let sys = System::new();
    let moves = [Move::Spock, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    let next_round_players: BTreeSet<ActorId> =
        USERS[0..2].iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[1], moves[1].clone(), next_round_players);
}

#[test]
fn check_paper_scissors_pair_winner() {
    let sys = System::new();
    let moves = [Move::Paper, Move::Scissors];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[1].into());
}

#[test]
fn check_paper_rock_pair_winner() {
    let sys = System::new();
    let moves = [Move::Paper, Move::Rock];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[0].into());
}

#[test]
fn check_paper_lizzard_pair_winner() {
    let sys = System::new();
    let moves = [Move::Paper, Move::Lizard];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[1].into());
}

#[test]
fn check_paper_spock_pair_winner() {
    let sys = System::new();
    let moves = [Move::Paper, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[0].into());
}

#[test]
fn check_rock_scissors_pair_winner() {
    let sys = System::new();
    let moves = [Move::Rock, Move::Scissors];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[0].into());
}

#[test]
fn check_rock_lizard_pair_winner() {
    let sys = System::new();
    let moves = [Move::Rock, Move::Lizard];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[0].into());
}

#[test]
fn check_rock_spock_pair_winner() {
    let sys = System::new();
    let moves = [Move::Rock, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[1].into());
}

#[test]
fn check_scissors_lizard_pair_winner() {
    let sys = System::new();
    let moves = [Move::Scissors, Move::Lizard];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[0].into());
}

#[test]
fn check_scissors_spock_pair_winner() {
    let sys = System::new();
    let moves = [Move::Scissors, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[1].into());
}

#[test]
fn check_lizard_spock_pair_winner() {
    let sys = System::new();
    let moves = [Move::Lizard, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, &USERS[0..2], &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    check_user_reveal_with_game_over(&game, USERS[1], moves[1].clone(), USERS[0].into());
}

#[test]
fn check_several_rounds() {
    let sys = System::new();
    let moves = [Move::Spock, Move::Lizard, Move::Lizard, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    let users = &USERS[1..3];
    let next_round_players: BTreeSet<ActorId> = users.iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(
        &game,
        USERS[3],
        moves[3].clone(),
        next_round_players.clone(),
    );

    let moves = [Move::Scissors, Move::Scissors];
    reach_reveal_stage(&game, users, &moves);

    check_user_reveal_with_continue(&game, users[0], moves[0].clone());
    check_user_reveal_with_next_round(&game, users[1], moves[1].clone(), next_round_players);

    let moves = [Move::Rock, Move::Scissors];
    reach_reveal_stage(&game, users, &moves);

    check_user_reveal_with_continue(&game, users[0], moves[0].clone());
    check_user_reveal_with_game_over(&game, users[1], moves[1].clone(), users[0].into());
}

#[test]
fn check_one_move_one_player_wins_one_move() {
    let sys = System::new();
    let moves = [Move::Rock, Move::Rock, Move::Spock, Move::Rock];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    check_user_reveal_with_game_over(&game, USERS[3], moves[3].clone(), USERS[2].into());
}

#[test]
fn check_one_move_one_player_wins_several_moves() {
    let sys = System::new();
    let moves = [Move::Rock, Move::Rock, Move::Spock, Move::Scissors];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    check_user_reveal_with_game_over(&game, USERS[3], moves[3].clone(), USERS[2].into());
}

#[test]
fn check_one_move_several_players_wins_one_move() {
    let sys = System::new();
    let moves = [Move::Spock, Move::Lizard, Move::Lizard, Move::Spock];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    let users = &USERS[1..3];
    let next_round_players: BTreeSet<ActorId> = users.iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[3], moves[3].clone(), next_round_players);
}

#[test]
fn check_one_move_several_players_wins_several_moves() {
    let sys = System::new();
    let moves = [Move::Spock, Move::Lizard, Move::Lizard, Move::Paper];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    let users = &USERS[1..3];
    let next_round_players: BTreeSet<ActorId> = users.iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[3], moves[3].clone(), next_round_players);
}

#[test]
fn check_four_different_moves() {
    let sys = System::new();
    let moves = [Move::Spock, Move::Rock, Move::Lizard, Move::Paper];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    for index in 0..3 {
        check_user_reveal_with_continue(&game, USERS[index], moves[index].clone());
    }

    let next_round_players: BTreeSet<ActorId> = USERS.iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, USERS[3], moves[3].clone(), next_round_players);
}

#[test]
fn check_five_different_moves() {
    let sys = System::new();
    let moves = [
        Move::Spock,
        Move::Rock,
        Move::Lizard,
        Move::Paper,
        Move::Scissors,
    ];
    let mut users = USERS.to_vec();
    users.push(USERS[3] + 1);
    sys.mint_to(USERS[3] + 1, 1_000_000_000);

    let game = reach_reveal_stage_with_init(&sys, users.as_slice(), &moves);

    for index in 0..4 {
        check_user_reveal_with_continue(&game, users[index], moves[index].clone());
    }

    let next_round_players: BTreeSet<ActorId> = users.iter().copied().map(|id| id.into()).collect();

    check_user_reveal_with_next_round(&game, users[4], moves[4].clone(), next_round_players);
}

// failure tests

#[test]
fn check_reveal_on_preparation_stage() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    failure_user_reveal(&game, USERS[0], Move::Rock)
}

#[test]
fn check_reveal_on_progress_stage() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    check_user_move(&game, USERS[0], Move::Rock);

    failure_user_reveal(&game, USERS[0], Move::Rock);
    failure_user_reveal(&game, USERS[1], Move::Rock);
}

#[test]
fn check_reveal_twice() {
    let sys = System::new();
    let moves = [Move::Spock, Move::Lizard, Move::Lizard, Move::Paper];

    let game = reach_reveal_stage_with_init(&sys, USERS, &moves);

    check_user_reveal_with_continue(&game, USERS[0], moves[0].clone());

    failure_user_reveal(&game, USERS[0], moves[0].clone());
    failure_user_reveal(&game, USERS[0], Move::Rock);
}

#[test]
fn check_third_party_player_reveal() {
    let sys = System::new();
    let game = common_init_and_register(&sys);
    let moves = [Move::Spock, Move::Lizard, Move::Lizard];

    reach_reveal_stage_with_init(&sys, &USERS[..3], &moves);

    failure_user_reveal(&game, USERS[3], Move::Rock);
}

#[test]
fn check_other_password_reveal() {
    let sys = System::new();
    let game = common_init_and_register(&sys);
    let moves = [Move::Spock, Move::Lizard, Move::Lizard, Move::Lizard];

    reach_reveal_stage_with_init(&sys, USERS, &moves);

    failure_user_reveal_with_password(&game, USERS[3], Move::Rock, "pass");
}
