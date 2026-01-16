use sails_rs::client::*;
use sails_rs::gtest::System;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;

use tic_tac_toe_client::TicTacToe as ClientTicTacToe;
use tic_tac_toe_client::tic_tac_toe::TicTacToe;
use tic_tac_toe_client::{Config, GameResult, TicTacToeCtors};

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: u64 = 11;

fn default_config() -> Config {
    Config {
        s_per_block: 3,
        gas_to_remove_game: 5_000_000_000,
        time_interval: 20,
        turn_deadline_ms: 30_000,
        gas_to_delete_session: 5_000_000_000,
        minimum_session_duration_ms: 180_000,
    }
}

#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/tic_tac_toe.opt.wasm");

    let program = env
        .deploy::<tic_tac_toe_client::TicTacToeProgram>(code_id, b"salt-ttt".to_vec())
        .new(default_config(), None)
        .await
        .unwrap();

    let mut ttt = program.tic_tac_toe();

    ttt.start_game(None).await.unwrap();

    let game_instance = ttt.game(ADMIN_ID.into()).await.unwrap();
    assert!(game_instance.is_some());

    ttt.turn(0, None).await.unwrap();
    ttt.turn(1, None).await.unwrap();

    let game_instance = ttt.game(ADMIN_ID.into()).await.unwrap().unwrap();
    assert!(game_instance.game_over);
    assert_eq!(game_instance.game_result, Some(GameResult::Bot));
}

#[tokio::test]
async fn add_and_remove_admin() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/tic_tac_toe.opt.wasm");

    let program = env
        .deploy::<tic_tac_toe_client::TicTacToeProgram>(code_id, b"salt-ttt".to_vec())
        .new(default_config(), None)
        .await
        .unwrap();

    let mut ttt = program.tic_tac_toe();

    ttt.add_admin(USER_ID.into()).await.unwrap();

    let admins = ttt.admins().await.unwrap();
    assert_eq!(admins, vec![ADMIN_ID.into(), USER_ID.into()]);

    ttt.remove_admin(USER_ID.into()).await.unwrap();

    let admins = ttt.admins().await.unwrap();
    assert_eq!(admins, vec![ADMIN_ID.into()]);
}

#[tokio::test]
async fn allow_messages() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/tic_tac_toe.opt.wasm");

    let program = env
        .deploy::<tic_tac_toe_client::TicTacToeProgram>(code_id, b"salt-ttt".to_vec())
        .new(default_config(), None)
        .await
        .unwrap();

    let mut ttt = program.tic_tac_toe();

    ttt.allow_messages(false).await.unwrap();

    let allowed = ttt.messages_allowed().await.unwrap();
    assert!(!allowed);

    let res = ttt.start_game(None).with_actor_id(USER_ID.into()).await;
    assert!(res.is_err());

    ttt.allow_messages(true).await.unwrap();

    ttt.start_game(None).await.unwrap();

    let game_instance = ttt.game(ADMIN_ID.into()).await.unwrap();
    assert!(game_instance.is_some());
}
