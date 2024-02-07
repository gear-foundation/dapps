use gtest::{Program, System};
use syndote_io::*;
pub mod utils;
use syndote::game::GameSessionActions;
use utils::{preconfigure, SyndoteTestFunctions, ADMIN_ID};

// Test for successful creation of a game session with a game fee.
#[test]
fn create_game_session_with_fee() {
    let system = System::new();
    let game = preconfigure(&system);

    // fee for the game: 50 VARA
    let fee = 50_000_000_000_000;
    game.create_game_session(ADMIN_ID, Some(fee), None);

    let game_session = game
        .get_game_session(ADMIN_ID)
        .expect("Game session doesn't exist");

    let mut exp_game_session: Game = Default::default();
    exp_game_session.init_properties();
    exp_game_session.admin_id = ADMIN_ID.into();
    exp_game_session.entry_fee = Some(fee);

    assert_eq!(game_session, exp_game_session.into());
}

// Test for successful creation of a game session without game fee.
#[test]
fn create_game_session_without_fee() {
    let system = System::new();
    let game = preconfigure(&system);

    game.create_game_session(ADMIN_ID, None, None);

    let game_session = game
        .get_game_session(ADMIN_ID)
        .expect("Game session doesn't exist");

    let mut exp_game_session: Game = Default::default();
    exp_game_session.init_properties();
    exp_game_session.admin_id = ADMIN_ID.into();

    assert_eq!(game_session, exp_game_session.into());
}

#[test]
fn failed_cases() {
    let system = System::new();
    system.init_logger();
    let game = preconfigure(&system);

    // The admin tries to create a game session specifying an entry fee that is less than ED.

    // fee for the game: 9 VARA (Less than ED)
    let fee = 9_000_000_000_000;
    game.create_game_session(ADMIN_ID, Some(fee), Some(GameError::FeeIsLessThanED));

    // successfull game session rcreation
    game.create_game_session(ADMIN_ID, None, None);

    //The admin tries to create a game session specifying an entry fee that is less than ED.
    game.create_game_session(ADMIN_ID, None, Some(GameError::GameSessionAlreadyExists));
}
