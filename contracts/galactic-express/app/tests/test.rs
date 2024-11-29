use galactic_express::{
    traits::{GalacticExpress, GalacticExpressFactory},
    GalacticExpress as GalacticExpressClient, GalacticExpressFactory as Factory, Participant,
    StageState,
};
use gstd::errors::{ErrorReplyReason, SimpleExecutionError};
use sails_rs::calls::*;
use sails_rs::errors::{Error, RtlError};
use sails_rs::gtest::{calls::*, System};

pub const ADMIN: u64 = 10;
pub const PLAYERS: [u64; 3] = [12, 13, 14];

#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN, 100_000_000_000_000);
    system.mint_to(PLAYERS[0], 100_000_000_000_000);
    system.mint_to(PLAYERS[1], 100_000_000_000_000);
    system.mint_to(PLAYERS[2], 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN.into());
    program_space.system().init_logger();
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/galactic_express.opt.wasm");

    let galactic_express_factory = Factory::new(program_space.clone());
    let galactic_express_id = galactic_express_factory
        .new(None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = GalacticExpressClient::new(program_space.clone());

    let bid = 11_000_000_000_000;
    program_space.system().mint_to(ADMIN, bid);

    // create_new_session
    client
        .create_new_session("Game".to_string())
        .with_value(bid)
        .send_recv(galactic_express_id)
        .await
        .unwrap();
    // check game state
    let state = client.all().recv(galactic_express_id).await.unwrap();
    assert!(!state.games.is_empty());
    assert!(!state.player_to_game_id.is_empty());

    // register
    for player_id in PLAYERS {
        let player = Participant {
            id: player_id.into(),
            name: "player".to_string(),
            fuel_amount: 42,
            payload_amount: 20,
        };
        program_space.system().mint_to(player_id, bid);

        client
            .register(ADMIN.into(), player)
            .with_value(bid)
            .with_args(GTestArgs::new(player_id.into()))
            .send_recv(galactic_express_id)
            .await
            .unwrap();
    }
    // check game state
    let state = client.all().recv(galactic_express_id).await.unwrap();
    assert_eq!(state.player_to_game_id.len(), 4);
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 3);
    }

    // start game
    client
        .start_game(42, 20)
        .send_recv(galactic_express_id)
        .await
        .unwrap();

    let state = client.all().recv(galactic_express_id).await.unwrap();
    if let StageState::Results(results) = &state.games[0].1.stage {
        assert_eq!(results.rankings.len(), 4);
    }
}

#[tokio::test]
async fn cancel_register_and_delete_player() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN, 100_000_000_000_000);
    system.mint_to(PLAYERS[0], 100_000_000_000_000);
    system.mint_to(PLAYERS[1], 100_000_000_000_000);
    system.mint_to(PLAYERS[2], 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN.into());
    program_space.system().init_logger();
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/galactic_express.opt.wasm");

    let galactic_express_factory = Factory::new(program_space.clone());
    let galactic_express_id = galactic_express_factory
        .new(None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = GalacticExpressClient::new(program_space.clone());

    let bid = 11_000_000_000_000;
    program_space.system().mint_to(ADMIN, bid);

    // create_new_session
    client
        .create_new_session("Game".to_string())
        .with_value(bid)
        .send_recv(galactic_express_id)
        .await
        .unwrap();
    // check game state
    let state = client.all().recv(galactic_express_id).await.unwrap();
    assert!(!state.games.is_empty());
    assert!(!state.player_to_game_id.is_empty());

    // register
    for player_id in PLAYERS {
        let player = Participant {
            id: player_id.into(),
            name: "player".to_string(),
            fuel_amount: 42,
            payload_amount: 20,
        };
        program_space.system().mint_to(player_id, bid);

        client
            .register(ADMIN.into(), player)
            .with_value(bid)
            .with_args(GTestArgs::new(player_id.into()))
            .send_recv(galactic_express_id)
            .await
            .unwrap();
    }
    // check game state
    let state = client.all().recv(galactic_express_id).await.unwrap();
    assert_eq!(state.player_to_game_id.len(), 4);
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 3);
    }

    // cancel_register
    client
        .cancel_register()
        .with_args(GTestArgs::new(PLAYERS[0].into()))
        .send_recv(galactic_express_id)
        .await
        .unwrap();

    // check game state
    let state = client.all().recv(galactic_express_id).await.unwrap();
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 2);
    }
    assert_eq!(state.player_to_game_id.len(), 3);

    // delete_player
    client
        .delete_player(PLAYERS[1].into())
        .send_recv(galactic_express_id)
        .await
        .unwrap();

    // check game state
    let state = client.all().recv(galactic_express_id).await.unwrap();
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 1);
    }
    assert_eq!(state.player_to_game_id.len(), 2);
}

#[tokio::test]
async fn errors() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN, 100_000_000_000_000);
    system.mint_to(PLAYERS[0], 100_000_000_000_000);
    system.mint_to(PLAYERS[1], 100_000_000_000_000);
    system.mint_to(PLAYERS[2], 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN.into());
    program_space.system().init_logger();
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/release/galactic_express.opt.wasm");

    let galactic_express_factory = Factory::new(program_space.clone());
    let galactic_express_id = galactic_express_factory
        .new(None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = GalacticExpressClient::new(program_space.clone());

    let bid = 11_000_000_000_000;
    program_space.system().mint_to(ADMIN, bid);

    let player = Participant {
        id: ADMIN.into(),
        name: "player".to_string(),
        fuel_amount: 42,
        payload_amount: 20,
    };

    let res = client
        .register(ADMIN.into(), player)
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "NoSuchGame".to_string());

    client
        .create_new_session("Game".to_string())
        .with_value(bid)
        .send_recv(galactic_express_id)
        .await
        .unwrap();

    let player = Participant {
        id: ADMIN.into(),
        name: "player".to_string(),
        fuel_amount: 42,
        payload_amount: 20,
    };

    let res = client
        .register(ADMIN.into(), player)
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "SeveralRegistrations".to_string());

    let res = client
        .start_game(42, 20)
        .with_args(GTestArgs::new(PLAYERS[0].into()))
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "NoSuchGame".to_string());

    let res = client
        .start_game(42, 20)
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "NotEnoughParticipants".to_string());

    // register
    for player_id in PLAYERS {
        let player = Participant {
            id: player_id.into(),
            name: "player".to_string(),
            fuel_amount: 42,
            payload_amount: 20,
        };
        program_space.system().mint_to(player_id, bid);

        client
            .register(ADMIN.into(), player)
            .with_value(bid)
            .with_args(GTestArgs::new(player_id.into()))
            .send_recv(galactic_express_id)
            .await
            .unwrap();
    }

    let res = client
        .start_game(101, 100)
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "FuelOrPayloadOverload".to_string());

    let res = client
        .start_game(100, 101)
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "FuelOrPayloadOverload".to_string());

    let res = client
        .start_game(101, 101)
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "FuelOrPayloadOverload".to_string());

    let player = Participant {
        id: 100.into(),
        name: "player".to_string(),
        fuel_amount: 42,
        payload_amount: 20,
    };
    program_space.system().mint_to(100, 100_000_000_000_000);

    let res = client
        .register(ADMIN.into(), player)
        .with_value(bid)
        .with_args(GTestArgs::new(100.into()))
        .send_recv(galactic_express_id)
        .await;

    assert_error(&res, "SessionFull".to_string());
}

fn assert_error(res: &Result<(), Error>, error: String) {
    assert!(matches!(
        res,
        Err(sails_rs::errors::Error::Rtl(RtlError::ReplyHasError(
            ErrorReplyReason::Execution(SimpleExecutionError::UserspacePanic),
            message
        ))) if *message == "Panic occurred: ".to_string() + &error
    ));
}
