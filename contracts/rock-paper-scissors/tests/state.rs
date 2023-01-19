// let expected_players: LtStateReply = lt
// .meta_state(LtState::GetPlayers)
// .expect("Error in reading meta_state");
// println!("meta players: {:?}", expected_players,);
//
// assert_eq!(expected_players, LtStateReply::Players(players.clone()));

use gstd::ActorId;
use gtest::System;
use rps_io::*;
use std::collections::BTreeSet;

mod routines;
pub use routines::*;

#[test]
fn common_config_tests() {
    let sys = System::new();
    let game = common_init_and_register(&sys);
    let next_config = GameConfig {
        bet_size: 0,
        players_count_limit: 3,
        entry_timeout_ms: COMMON_TIMEOUT * 2,
        move_timeout_ms: COMMON_TIMEOUT * 3,
        reveal_timeout_ms: COMMON_TIMEOUT * 4,
    };
    check_change_next_game_config(&game, USERS[0], next_config.clone());

    if let StateReply::Config(config) = game.meta_state(State::Config).unwrap() {
        assert_eq!(config, COMMON_CONFIG);
    } else {
        panic!("not suitable reply")
    }

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Rock],
    );

    if let StateReply::Config(config) = game.meta_state(State::Config).unwrap() {
        assert_eq!(config, next_config);
    } else {
        panic!("not suitable reply")
    }
}

#[test]
fn common_stage_tests() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    if let StateReply::GameStage(stage) = game.meta_state(State::GameStage).unwrap() {
        match stage {
            GameStage::Preparation => {}
            GameStage::Reveal(_) | GameStage::InProgress(_) => panic!("wrong"),
        }
    } else {
        panic!("not suitable reply")
    }

    check_user_move(&game, USERS[0], Move::Rock);

    if let StateReply::GameStage(stage) = game.meta_state(State::GameStage).unwrap() {
        match stage {
            GameStage::InProgress(description) => {
                let mut anticipated = COMMON_USERS_SET
                    .iter()
                    .cloned()
                    .map(Into::into)
                    .collect::<BTreeSet<ActorId>>();
                let done = anticipated.take(&USERS[0].into()).unwrap();
                assert_eq!(description.anticipated_players, anticipated);
                assert_eq!(description.finished_players, BTreeSet::from([done]));
            }
            GameStage::Reveal(_) | GameStage::Preparation => panic!("wrong"),
        }
    } else {
        panic!("not suitable reply")
    }
}

#[test]
fn lobby_list() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    if let StateReply::LobbyList(mut lobby) = game.meta_state(State::LobbyList).unwrap() {
        let list = COMMON_USERS_SET
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<ActorId>>();
        lobby.sort();
        assert_eq!(lobby, list);
    } else {
        panic!("not suitable reply")
    }

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Paper],
    );

    if let StateReply::LobbyList(mut lobby) = game.meta_state(State::LobbyList).unwrap() {
        let list = COMMON_USERS_SET
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<ActorId>>();
        lobby.sort();
        assert_eq!(lobby, list);
    } else {
        panic!("not suitable reply")
    }
}
