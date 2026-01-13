use galactic_express_client::galactic_express::GalacticExpress;
use galactic_express_client::GalacticExpress as OtherGalacticExpress;
use galactic_express_client::GalacticExpressCtors;
use galactic_express_client::{Participant, StageState};

use gstd::errors::{ErrorReplyReason, SimpleExecutionError};
use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::gtest::System;

pub const ADMIN: u64 = 10;
pub const PLAYERS: [u64; 3] = [12, 13, 14];

const BID: u128 = 11_000_000_000_000;

#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    system.mint_to(ADMIN, DEFAULT_USERS_INITIAL_BALANCE);
    for p in PLAYERS {
        system.mint_to(p, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, ADMIN.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/galactic_express.opt.wasm");

    let program = env
        .deploy::<galactic_express_client::GalacticExpressProgram>(code_id, b"salt".to_vec())
        .new(None)
        .await
        .unwrap();

    let mut client = program.galactic_express();

    env.system().mint_to(ADMIN, BID);

    client
        .create_new_session("Game".to_string())
        .with_value(BID)
        .await
        .unwrap();

    let state = client.all().await.unwrap();
    assert!(!state.games.is_empty());
    assert!(!state.player_to_game_id.is_empty());

    for player_id in PLAYERS {
        let player = Participant {
            id: player_id.into(),
            name: "player".to_string(),
            fuel_amount: 42,
            payload_amount: 20,
        };

        env.system().mint_to(player_id, BID);

        client
            .register(ADMIN.into(), player)
            .with_value(BID)
            .with_actor_id(player_id.into())
            .await
            .unwrap();
    }

    let state = client.all().await.unwrap();
    assert_eq!(state.player_to_game_id.len(), 4);
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 3);
    } else {
        panic!("unexpected stage");
    }

    client.start_game(42, 20).await.unwrap();

    let state = client.all().await.unwrap();
    if let StageState::Results(results) = &state.games[0].1.stage {
        assert_eq!(results.rankings.len(), 4);
    } else {
        panic!("unexpected stage");
    }
}

#[tokio::test]
async fn cancel_register_and_delete_player() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    system.mint_to(ADMIN, DEFAULT_USERS_INITIAL_BALANCE);
    for p in PLAYERS {
        system.mint_to(p, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, ADMIN.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/galactic_express.opt.wasm");

    let program = env
        .deploy::<galactic_express_client::GalacticExpressProgram>(code_id, b"salt".to_vec())
        .new(None)
        .await
        .unwrap();

    let mut client = program.galactic_express();

    env.system().mint_to(ADMIN, BID);

    client
        .create_new_session("Game".to_string())
        .with_value(BID)
        .await
        .unwrap();

    let state = client.all().await.unwrap();
    assert!(!state.games.is_empty());
    assert!(!state.player_to_game_id.is_empty());

    for player_id in PLAYERS {
        let player = Participant {
            id: player_id.into(),
            name: "player".to_string(),
            fuel_amount: 42,
            payload_amount: 20,
        };

        env.system().mint_to(player_id, BID);

        client
            .register(ADMIN.into(), player)
            .with_value(BID)
            .with_actor_id(player_id.into())
            .await
            .unwrap();
    }

    let state = client.all().await.unwrap();
    assert_eq!(state.player_to_game_id.len(), 4);
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 3);
    } else {
        panic!("unexpected stage");
    }

    client
        .cancel_register()
        .with_actor_id(PLAYERS[0].into())
        .await
        .unwrap();

    let state = client.all().await.unwrap();
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 2);
    } else {
        panic!("unexpected stage");
    }
    assert_eq!(state.player_to_game_id.len(), 3);

    client.delete_player(PLAYERS[1].into()).await.unwrap();

    let state = client.all().await.unwrap();
    if let StageState::Registration(participants) = &state.games[0].1.stage {
        assert_eq!(participants.len(), 1);
    } else {
        panic!("unexpected stage");
    }
    assert_eq!(state.player_to_game_id.len(), 2);
}

#[tokio::test]
async fn errors() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    system.mint_to(ADMIN, DEFAULT_USERS_INITIAL_BALANCE);
    for p in PLAYERS {
        system.mint_to(p, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, ADMIN.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/galactic_express.opt.wasm");

    let program = env
        .deploy::<galactic_express_client::GalacticExpressProgram>(code_id, b"salt".to_vec())
        .new(None)
        .await
        .unwrap();

    let mut client = program.galactic_express();

    env.system().mint_to(ADMIN, BID);

    let player = Participant {
        id: ADMIN.into(),
        name: "player".to_string(),
        fuel_amount: 42,
        payload_amount: 20,
    };

    let res = client.register(ADMIN.into(), player).await;
    assert_error(&res, b"NoSuchGame");

    client
        .create_new_session("Game".to_string())
        .with_value(BID)
        .await
        .unwrap();

    let player = Participant {
        id: ADMIN.into(),
        name: "player".to_string(),
        fuel_amount: 42,
        payload_amount: 20,
    };

    let res = client.register(ADMIN.into(), player).await;
    assert_error(&res, b"SeveralRegistrations");

    let res = client
        .start_game(42, 20)
        .with_actor_id(PLAYERS[0].into())
        .await;
    assert_error(&res, b"NoSuchGame");

    let res = client.start_game(42, 20).await;
    assert_error(&res, b"NotEnoughParticipants");

    for player_id in PLAYERS {
        let player = Participant {
            id: player_id.into(),
            name: "player".to_string(),
            fuel_amount: 42,
            payload_amount: 20,
        };

        env.system().mint_to(player_id, BID);

        client
            .register(ADMIN.into(), player)
            .with_value(BID)
            .with_actor_id(player_id.into())
            .await
            .unwrap();
    }

    let res = client.start_game(101, 100).await;
    assert_error(&res, b"FuelOrPayloadOverload");

    let res = client.start_game(100, 101).await;
    assert_error(&res, b"FuelOrPayloadOverload");

    let res = client.start_game(101, 101).await;
    assert_error(&res, b"FuelOrPayloadOverload");

    let player = Participant {
        id: 100.into(),
        name: "player".to_string(),
        fuel_amount: 42,
        payload_amount: 20,
    };
    env.system().mint_to(100, DEFAULT_USERS_INITIAL_BALANCE);

    let res = client
        .register(ADMIN.into(), player)
        .with_value(BID)
        .with_actor_id(100.into())
        .await;

    assert_error(&res, b"SessionFull");
}

fn assert_error(res: &Result<(), GtestError>, error: &[u8]) {
    assert!(matches!(
        res,
        Err(GtestError::ReplyHasError(
            ErrorReplyReason::Execution(SimpleExecutionError::UserspacePanic),
            message
        )) if message == error
    ));
}
