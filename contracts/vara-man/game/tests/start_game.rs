mod utils;

use gstd::ActorId;
use gtest::{Program, System};
use utils::VaraMan;
use vara_man_io::{Level, Status};

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.games.is_empty());

    let player_0_id: ActorId = utils::PLAYERS[0].into();

    vara_man.register_player(utils::PLAYERS[0], "John", false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);

    let state = vara_man.get_state();

    assert_eq!(state.games.len(), 1);
    assert_eq!(state.games[0].level, Level::Easy);
    assert_eq!(state.games[0].player_address, player_0_id);
    assert_ne!(state.games[0].gold_coins, 0);
    assert_ne!(state.games[0].silver_coins, 0);
    assert_ne!(state.games[0].start_time_ms, 0);
    assert!(!state.games[0].is_claimed);
}

#[test]
fn fail_player_must_register() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.games.is_empty());

    vara_man.start_game(utils::PLAYERS[0], Level::Hard, u64::MAX, true);

    let state = vara_man.get_state();
    assert_eq!(state.games.len(), 0);
}

#[test]
fn fail_player_has_exhausted_all_attempts() {
    let system = System::new();
    system.init_logger();

    let vara_man = Program::vara_man(&system);
    vara_man.change_status(Status::Started);

    let state = vara_man.get_state();
    assert!(state.games.is_empty());

    vara_man.register_player(utils::PLAYERS[0], "John", false);

    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, false);
    vara_man.start_game(utils::PLAYERS[0], Level::Easy, u64::MAX, true);

    let state = vara_man.get_state();
    assert_eq!(state.games.len(), 3);
}
