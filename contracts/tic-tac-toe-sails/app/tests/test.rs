use tic_tac_toe_wasm::{
    traits::{TicTacToeFactory, TicTacToe},
    TicTacToeFactory as Factory, TicTacToe as TicTacToeClient,
    Config, GameResult
};
use sails_rs::calls::*;
use sails_rs::gtest::calls::*;

#[tokio::test]
async fn test_play_game() {
    let program_space = GTestRemoting::new(100.into());
    program_space.system().init_logger();
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/debug/tic_tac_toe_wasm.opt.wasm");

    let tic_tac_toe_factory = Factory::new(program_space.clone());
    let config = Config {
        s_per_block: 3,
        gas_to_remove_game: 5_000_000_000,
        time_interval: 2_000,
        turn_deadline_ms: 10_000,
    };
    let tic_tac_toe_id = tic_tac_toe_factory
        .new(config)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = TicTacToeClient::new(program_space);
    // start_game
    client
        .start_game()
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check game instance
    let game_instance = client
        .game(100.into())
        .recv(tic_tac_toe_id)
        .await
        .unwrap();
    assert!(game_instance.is_some());

    client
        .turn(0.into())
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();

    client
        .turn(1.into())
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();

    // check game instance
    let game_instance = client
        .game(100.into())
        .recv(tic_tac_toe_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(game_instance.game_over, true);
    assert_eq!(game_instance.game_result, Some(GameResult::Bot));
    // println!("GAME: {:?}", game_instance);

}

#[tokio::test]
async fn add_and_remove_admin() {
    let program_space = GTestRemoting::new(100.into());
    program_space.system().init_logger();
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/debug/tic_tac_toe_wasm.opt.wasm");

    let tic_tac_toe_factory = Factory::new(program_space.clone());
    let config = Config {
        s_per_block: 3,
        gas_to_remove_game: 5_000_000_000,
        time_interval: 2_000,
        turn_deadline_ms: 10_000,
    };
    let tic_tac_toe_id = tic_tac_toe_factory
        .new(config)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = TicTacToeClient::new(program_space.clone());
    // add admin
    client
        .add_admin(101.into())
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check state
    let admins = client
        .admins()
        .recv(tic_tac_toe_id)
        .await
        .unwrap();
    assert_eq!(admins, vec![100.into(), 101.into()]);

    // remove admin
        client
        .remove_admin(101.into())
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check state
    let admins = client
        .admins()
        .recv(tic_tac_toe_id)
        .await
        .unwrap();
    assert_eq!(admins, vec![100.into()]);
}

#[tokio::test]
async fn allow_messages() {
    let program_space = GTestRemoting::new(100.into());
    program_space.system().init_logger();
    let code_id = program_space
        .system()
        .submit_code_file("../../target/wasm32-unknown-unknown/debug/tic_tac_toe_wasm.opt.wasm");

    let tic_tac_toe_factory = Factory::new(program_space.clone());
    let config = Config {
        s_per_block: 3,
        gas_to_remove_game: 5_000_000_000,
        time_interval: 2_000,
        turn_deadline_ms: 10_000,
    };
    let tic_tac_toe_id = tic_tac_toe_factory
        .new(config)
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
    assert_eq!(messages_allowed, false);

    let res = client
        .start_game()
        .with_args(GTestArgs::new(101.into()))
        .send_recv(tic_tac_toe_id)
        .await;
    assert!(res.is_err());

    // start_game
    client
        .start_game()
        .send_recv(tic_tac_toe_id)
        .await
        .unwrap();
    // check game instance
    let game_instance = client
        .game(100.into())
        .recv(tic_tac_toe_id)
        .await
        .unwrap();
    assert!(game_instance.is_some());
}
