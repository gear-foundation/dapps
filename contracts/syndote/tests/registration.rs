use gtest::{Program, System};
use syndote_io::*;
pub mod utils;
use syndote::game::{GameSessionActions, INITIAL_BALANCE};
use utils::{preconfigure, upload_strategy, SyndoteTestFunctions, ADMIN_ID, PLAYERS};

// Test for successful registration in a game where a fee is required.
#[test]
fn successful_registration_fee_required() {
    let system = System::new();
    let game = preconfigure(&system);

    // fee for the game: 50 VARA
    let fee = 50_000_000_000_000;
    game.create_game_session(ADMIN_ID, Some(fee), None);

    let strategy = upload_strategy(&system);

    system.mint_to(PLAYERS[0], 50_000_000_000_000);

    game.register(PLAYERS[0], ADMIN_ID, strategy.id().into(), Some(fee), None);

    let balance_after = system.balance_of(PLAYERS[0]);
    assert_eq!(balance_after, 0);

    let program_balance = system.balance_of(game.id());
    assert_eq!(program_balance, fee);

    let player_info = game
        .get_player_info(ADMIN_ID, PLAYERS[0])
        .expect("Player does not exist");
    assert_eq!(player_info.owner_id, PLAYERS[0].into());
    assert_eq!(player_info.balance, INITIAL_BALANCE);
    assert!(player_info.reservation_id.is_some());

    let game_session = game.get_game_session(ADMIN_ID).expect("Game does not exist");
    assert_eq!(game_session.prize_pool, fee);
}

// Test for successful registration in a game where no fee is required.
#[test]
fn successful_registration_no_fee_required() {

}

#[test]
fn failed_cases() {
    //     Test for attempting to register the same strategy.
    // Test for attempting to register from the same account.
    // Test for attempting to register when the game is already in progress.
}
