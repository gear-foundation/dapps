use gtest::System;
use syndote_io::*;
pub mod utils;
use syndote::game::INITIAL_BALANCE;
use utils::{preconfigure, upload_strategy, SyndoteTestFunctions, ADMIN_ID, PLAYERS};

// Test for successful registration in a game where a fee is required.
#[test]
fn successful_registration_fee_required() {
    let system = System::new();
    let game = preconfigure(&system);

    // fee for the game: 50 VARA
    let fee = 50_000_000_000_000;
    let strategy = upload_strategy(&system);
    game.create_game_session(
        ADMIN_ID,
        strategy.id().into(),
        "Alice".to_string(),
        Some(fee),
        None,
    );

    let strategy = upload_strategy(&system);

    system.mint_to(PLAYERS[0], 50_000_000_000_000);

    game.register(PLAYERS[0], ADMIN_ID, strategy.id().into(), Some(fee), None);

    let balance_after = system.balance_of(PLAYERS[0]);
    assert_eq!(balance_after, 0);

    let program_balance = system.balance_of(game.id());
    assert_eq!(program_balance, fee);

    let player_info = game
        .get_player_info(ADMIN_ID)
        .expect("Player does not exist");
    assert_eq!(player_info.owner_id, PLAYERS[0].into());
    assert_eq!(player_info.balance, INITIAL_BALANCE);
    assert!(player_info.reservation_id.is_some());

    let game_session = game
        .get_game_session(ADMIN_ID)
        .expect("Game does not exist");
    assert_eq!(game_session.prize_pool, fee);
}

// Test for successful registration in a game where no fee is required.
#[test]
fn successful_registration_no_fee_required() {
    let system = System::new();
    let game = preconfigure(&system);
    let strategy = upload_strategy(&system);
    game.create_game_session(
        ADMIN_ID,
        strategy.id().into(),
        "Alice".to_string(),
        None,
        None,
    );

    let strategy = upload_strategy(&system);

    game.register(PLAYERS[0], ADMIN_ID, strategy.id().into(), None, None);

    let player_info = game
        .get_player_info(ADMIN_ID)
        .expect("Player does not exist");
    assert_eq!(player_info.owner_id, PLAYERS[0].into());
    assert_eq!(player_info.balance, INITIAL_BALANCE);
    assert!(player_info.reservation_id.is_some());
}

#[test]
fn registration_failed_cases() {
    let system = System::new();
    let game = preconfigure(&system);

    // fee for the game: 50 VARA
    let fee = 50_000_000_000_000;
    let strategy = upload_strategy(&system);

    game.create_game_session(
        ADMIN_ID,
        strategy.id().into(),
        "Alice".to_string(),
        Some(fee),
        None,
    );

    let strategy = upload_strategy(&system);

    system.mint_to(PLAYERS[0], fee);
    system.mint_to(PLAYERS[1], fee);

    game.register(PLAYERS[0], ADMIN_ID, strategy.id().into(), Some(fee), None);

    // Attempting to register the same strategy.
    game.register(
        PLAYERS[1],
        ADMIN_ID,
        strategy.id().into(),
        Some(fee),
        Some(GameError::StrategyAlreadyReistered),
    );

    // check that vara was returned to PLAYERS[1]
    system.claim_value_from_mailbox(PLAYERS[1]);
    assert_eq!(system.balance_of(PLAYERS[1]), fee);

    // Test for attempting to register from the same account.
    let new_strategy = upload_strategy(&system);
    system.mint_to(PLAYERS[0], fee);
    game.register(
        PLAYERS[0],
        ADMIN_ID,
        new_strategy.id().into(),
        Some(fee),
        Some(GameError::AccountAlreadyRegistered),
    );

    // The players send the wrong vara amount
    game.register(
        PLAYERS[1],
        ADMIN_ID,
        new_strategy.id().into(),
        Some(20_000_000_000_000),
        Some(GameError::WrongValueAmount),
    );
}

// Successful registration cancellation
// Player leaves the game
#[test]
fn exit_game() {
    let system = System::new();
    let game = preconfigure(&system);

    // fee for the game: 50 VARA
    let fee = 50_000_000_000_000;
    let strategy = upload_strategy(&system);
    game.create_game_session(
        ADMIN_ID,
        strategy.id().into(),
        "Alice".to_string(),
        Some(fee),
        None,
    );

    let strategy = upload_strategy(&system);

    system.mint_to(PLAYERS[0], fee);

    game.register(PLAYERS[0], ADMIN_ID, strategy.id().into(), Some(fee), None);

    game.exit_game(PLAYERS[0], ADMIN_ID, None);

    system.claim_value_from_mailbox(PLAYERS[0]);
    assert_eq!(system.balance_of(PLAYERS[0]), fee);

    let player_info = game.get_player_info(ADMIN_ID);
    assert!(player_info.is_none());

    let game_session = game
        .get_game_session(ADMIN_ID)
        .expect("Game session does not exist");
    assert!(game_session.owners_to_strategy_ids.is_empty());
    assert!(game_session.players.is_empty());
}

// Successful game session cancellation
#[test]
fn cancel_game_session() {
    let system = System::new();
    let game = preconfigure(&system);

    // fee for the game: 50 VARA
    let fee = 50_000_000_000_000;
    let strategy = upload_strategy(&system);
    system.mint_to(ADMIN_ID, fee);
    game.create_game_session(
        ADMIN_ID,
        strategy.id().into(),
        "Alice".to_string(),
        Some(fee),
        None,
    );

    for player in PLAYERS.iter().take(3) {
        system.mint_to(*player, fee);
        let strategy = upload_strategy(&system);
        game.register(*player, ADMIN_ID, strategy.id().into(), Some(fee), None);
    }

    game.cancel_game_session(ADMIN_ID, ADMIN_ID, None);

    for player in PLAYERS.iter().take(3) {
        system.claim_value_from_mailbox(*player);
        assert_eq!(system.balance_of(*player), fee);
    }
    let game_session = game.get_game_session(ADMIN_ID);
    assert!(game_session.is_none());

    let game_session = game.get_game_session(PLAYERS[0]);
    println!("{:?}", game_session);

    system.mint_to(ADMIN_ID, fee);
    game.create_game_session(
        ADMIN_ID,
        strategy.id().into(),
        "Alice".to_string(),
        Some(fee),
        None,
    );

    let game_session = game.get_game_session(PLAYERS[1]);
    println!("{:?}", game_session);
}
