// let expected_players: LtStateReply = lt
// .meta_state(LtState::GetPlayers)
// .expect("Error in reading meta_state");
// println!("meta players: {:?}", expected_players,);
//
// assert_eq!(expected_players, LtStateReply::Players(players.clone()));

use gstd::ActorId;
use gtest::System;
use rock_paper_scissors_io::*;
// use rps_state::state::metafns::{
//     config, current_stage_start_timestamp, game_stage, lobby_list, State,
// };

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

    let state: ContractState = game.read_state().expect("Not suitable reply");
    assert_eq!(COMMON_CONFIG, state.game_config);

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Rock],
    );

    let state: ContractState = game.read_state().expect("Not suitable reply");
    assert_eq!(next_config, state.game_config);
}

#[test]
fn common_stage_tests() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    let state: ContractState = game.read_state().expect("Not suitable reply");
    let stage = state.stage;
    match stage {
        GameStage::Preparation => {}
        GameStage::Reveal(_) | GameStage::InProgress(_) => panic!("wrong"),
    }

    check_user_move(&game, USERS[0], Move::Rock);

    let state: ContractState = game.read_state().expect("Not suitable reply");

    let list = COMMON_USERS_SET
        .iter()
        .cloned()
        .map(Into::into)
        .collect::<Vec<ActorId>>();
    let mut lobby = state.lobby;
    lobby.sort();
    assert_eq!(lobby, list);

    let state: ContractState = game.read_state().expect("Not suitable reply");
    let stage = state.stage;
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
}

#[test]
fn lobby_list_test() {
    let sys = System::new();
    let game = common_init_and_register(&sys);

    let state: ContractState = game.read_state().expect("Not suiable reply");

    let list = COMMON_USERS_SET
        .iter()
        .cloned()
        .map(Into::into)
        .collect::<Vec<ActorId>>();
    let mut lobby = state.lobby;
    lobby.sort();
    assert_eq!(lobby, list);

    play_round(
        &game,
        COMMON_USERS_SET,
        &[Move::Rock, Move::Paper, Move::Paper],
    );

    let state: ContractState = game.read_state().expect("Not suiable reply");

    let list = COMMON_USERS_SET
        .iter()
        .cloned()
        .map(Into::into)
        .collect::<Vec<ActorId>>();
    let mut lobby = state.lobby;
    lobby.sort();
    assert_eq!(lobby, list);
}
