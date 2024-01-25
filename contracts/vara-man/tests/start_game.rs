mod utils;
use crate::utils::ADMIN;
use gstd::ActorId;
use gtest::{Program, System};
use utils::VaraMan;
use vara_man_io::{Level, Status, VaraManError};

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    vara_man.change_status(ADMIN, Status::StartedWithNativeToken);

    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert!(state.games.is_empty());

    let player_0_id: ActorId = utils::PLAYERS[0].into();

    vara_man.register_player(utils::PLAYERS[0], "John", None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);

    let state = vara_man.get_state().expect("Unexpected invalid state.");

    assert_eq!(state.games.len(), 1);
    assert_eq!(state.games[0].0, player_0_id);
    assert_ne!(state.games[0].1.gold_coins, 0);
    assert_ne!(state.games[0].1.silver_coins, 0);
}

#[test]
fn fail_player_must_register() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    vara_man.change_status(ADMIN, Status::StartedWithNativeToken);

    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert!(state.games.is_empty());

    vara_man.start_game(
        utils::PLAYERS[0],
        Level::Hard,
        Some(VaraManError::NotRegistered),
    );

    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert_eq!(state.games.len(), 0);
}

#[test]
fn fail_player_has_exhausted_all_attempts() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);

    vara_man.change_status(ADMIN, Status::StartedWithNativeToken);
    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert!(state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", None);

    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(
        utils::PLAYERS[0],
        Level::Easy,
        Some(VaraManError::LivesEnded),
    );

    let state = vara_man.get_state().expect("Unexpected invalid state.");
    assert_eq!(state.games.len(), 0);
}

#[test]
fn success_add_admin() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    vara_man.change_status(ADMIN, Status::StartedWithNativeToken);

    vara_man.register_player(utils::PLAYERS[0], "John", None);

    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(
        utils::PLAYERS[0],
        Level::Easy,
        Some(VaraManError::LivesEnded),
    );

    vara_man.add_admin(ADMIN, utils::PLAYERS[0].into());

    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
    vara_man.claim_reward(utils::PLAYERS[0], 10, 1, None);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, None);
}
