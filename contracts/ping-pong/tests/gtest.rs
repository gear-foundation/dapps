use ping_pong_client::{ping_pong::PingPong, PingPong as ClientPingPong, PingPongCtors};

use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{client::*, gtest::*};

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn do_ping() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let code_id = system.submit_code(ping_pong::WASM_BINARY);
    let env = GtestEnv::new(system, ACTOR_ID.into());

    let program = env
        .deploy::<ping_pong_client::PingPongProgram>(code_id, b"salt".to_vec())
        .new()
        .await
        .unwrap();

    let mut client = program.ping_pong();

    let result = client.ping().await.unwrap();
    assert_eq!(result, "Pong!".to_string());

    let count = client.get_ping_count().await.unwrap();
    assert_eq!(count, 1.into());
}
