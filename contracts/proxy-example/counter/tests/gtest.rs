use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use counter_client::traits::*;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn contribute_works() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    let program_code_id = remoting.system().submit_code(counter::WASM_BINARY);

    let program_factory = counter_client::CounterFactory::new(remoting.clone());

    let program_id = program_factory
        .new(20_000_000_000)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = counter_client::Counter::new(remoting.clone());

    let result = service_client
        .contribute(None)
        .with_value(10_000_000_000)
        .send_recv(program_id)
        .await
        .unwrap();

    assert_eq!(result, 10_000_000_000);

    let result = service_client.get_value().recv(program_id).await.unwrap();

    assert_eq!(result, 10_000_000_000);
}
