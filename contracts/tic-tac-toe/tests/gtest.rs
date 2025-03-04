use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};
use tic_tac_toe_client::{
    traits::{TicTacToe, TicTacToeFactory},
    Config, GameResult, TicTacToe as TicTacToeClient, TicTacToeFactory as Factory,
};

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: u64 = 11;

#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../target/wasm32-unknown-unknown/release/tic_tac_toe.opt.wasm");

    let tic_tac_toe_factory = Factory::new(program_space.clone());
    let config = Config {
        s_per_block: 3,
        gas_to_remove_game: 5_000_000_000,
        time_interval: 20,
        turn_deadline_ms: 30_000,
        gas_to_delete_session: 5_000_000_000,
        minimum_session_duration_ms: 180_000,
    };
    let tic_tac_toe_id = tic_tac_toe_factory
        .new(config, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = TicTacToeClient::new(program_space);
    // start_game
    client
        .start_game(None)
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check game instance
    let game_instance = client
        .game(ADMIN_ID.into())
        .recv(tic_tac_toe_id)
        .await
        .unwrap();
    assert!(game_instance.is_some());

    client
        .turn(0, None)
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();

    client
        .turn(1, None)
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();

    // check game instance
    let game_instance = client
        .game(ADMIN_ID.into())
        .recv(tic_tac_toe_id)
        .await
        .unwrap()
        .unwrap();

    assert!(game_instance.game_over);
    assert_eq!(game_instance.game_result, Some(GameResult::Bot));
}

#[tokio::test]
async fn add_and_remove_admin() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());

    let code_id = program_space
        .system()
        .submit_code_file("../target/wasm32-unknown-unknown/release/tic_tac_toe.opt.wasm");

    let tic_tac_toe_factory = Factory::new(program_space.clone());
    let config = Config {
        s_per_block: 3,
        gas_to_remove_game: 5_000_000_000,
        time_interval: 20,
        turn_deadline_ms: 30_000,
        gas_to_delete_session: 5_000_000_000,
        minimum_session_duration_ms: 180_000,
    };
    let tic_tac_toe_id = tic_tac_toe_factory
        .new(config, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = TicTacToeClient::new(program_space.clone());
    // add admin
    client
        .add_admin(USER_ID.into())
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check state
    let admins = client.admins().recv(tic_tac_toe_id).await.unwrap();
    assert_eq!(admins, vec![ADMIN_ID.into(), USER_ID.into()]);

    // remove admin
    client
        .remove_admin(USER_ID.into())
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check state
    let admins = client.admins().recv(tic_tac_toe_id).await.unwrap();
    assert_eq!(admins, vec![ADMIN_ID.into()]);
}

#[tokio::test]
async fn allow_messages() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ADMIN_ID, 100_000_000_000_000);
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, ADMIN_ID.into());

    let code_id = program_space
        .system()
        .submit_code_file("../target/wasm32-unknown-unknown/release/tic_tac_toe.opt.wasm");

    let tic_tac_toe_factory = Factory::new(program_space.clone());
    let config = Config {
        s_per_block: 3,
        gas_to_remove_game: 5_000_000_000,
        time_interval: 20,
        turn_deadline_ms: 30_000,
        gas_to_delete_session: 5_000_000_000,
        minimum_session_duration_ms: 180_000,
    };
    let tic_tac_toe_id = tic_tac_toe_factory
        .new(config, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = TicTacToeClient::new(program_space.clone());
    // allow messages in false
    client
        .allow_messages(false)
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check state
    let messages_allowed = client
        .messages_allowed()
        .recv(tic_tac_toe_id)
        .await
        .unwrap();
    assert!(!messages_allowed);

    let res = client
        .start_game(None)
        .with_args(GTestArgs::new(USER_ID.into()))
        .send_recv(tic_tac_toe_id)
        .await;
    assert!(res.is_err());

    // start_game
    client
        .start_game(None)
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check game instance
    let game_instance = client
        .game(ADMIN_ID.into())
        .recv(tic_tac_toe_id)
        .await
        .unwrap();
    assert!(game_instance.is_some());
}
