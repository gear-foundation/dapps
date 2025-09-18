use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use ping_pong_client::traits::*;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn do_ping() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(ping_pong::WASM_BINARY);

    let program_factory = ping_pong_client::PingPongFactory::new(remoting.clone());

    let program_id = program_factory
        .new()
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = ping_pong_client::PingPong::new(remoting.clone());

    let result = service_client.ping().send_recv(program_id).await.unwrap();

    assert_eq!(result, "Pong!".to_string());

    let result = service_client
        .get_ping_count()
        .recv(program_id)
        .await
        .unwrap();

    assert_eq!(result, 1.into());
}
