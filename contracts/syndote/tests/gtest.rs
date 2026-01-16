use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{ActorId, gtest::System};

use syndote_client::Syndote as ClientSyndote;
use syndote_client::syndote::Syndote;
use syndote_client::{GameStatus, SyndoteCtors};
use syndote_player_client::SyndotePlayerCtors;

pub const ADMIN_ID: u64 = 10;
pub const USER_ID: u64 = 11;

#[tokio::test]
async fn test_play_game() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ADMIN_ID, DEFAULT_USERS_INITIAL_BALANCE);
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ADMIN_ID.into());

    let syndote_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/syndote.opt.wasm");

    let syndote_program = env
        .deploy::<syndote_client::SyndoteProgram>(syndote_code_id, b"salt-syndote".to_vec())
        .new(None)
        .await
        .unwrap();

    let player_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/syndote_player.opt.wasm");

    let player_1 = env
        .deploy::<syndote_player_client::SyndotePlayerProgram>(player_code_id, b"p1".to_vec())
        .new()
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let player_2 = env
        .deploy::<syndote_player_client::SyndotePlayerProgram>(player_code_id, b"p2".to_vec())
        .new()
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let player_3 = env
        .deploy::<syndote_player_client::SyndotePlayerProgram>(player_code_id, b"p3".to_vec())
        .new()
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let player_4 = env
        .deploy::<syndote_player_client::SyndotePlayerProgram>(player_code_id, b"p4".to_vec())
        .new()
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let mut syndote = syndote_program.syndote();

    let state = syndote.get_storage().await.unwrap();
    assert_eq!(state.game_status, GameStatus::Registration);

    syndote.register(player_1.id()).await.unwrap();
    syndote.register(player_2.id()).await.unwrap();
    syndote.register(player_3.id()).await.unwrap();
    syndote.register(player_4.id()).await.unwrap();

    let state = syndote.get_storage().await.unwrap();
    assert_eq!(state.game_status, GameStatus::Play);
    assert_eq!(state.round, 0);
    assert_eq!(state.winner, ActorId::zero());

    syndote.play().await.unwrap();

    let state = syndote.get_storage().await.unwrap();
    assert_eq!(state.game_status, GameStatus::Finished);
    assert_ne!(state.round, 0);
    assert_ne!(state.winner, ActorId::zero());
}
