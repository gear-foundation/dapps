use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::gtest::System;

use counter_client::counter::Counter;
use counter_client::Counter as ClientCounter;
use counter_client::CounterCtors;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn contribute_works() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ACTOR_ID.into());

    let code_id = env.system().submit_code(counter::WASM_BINARY);

    let program = env
        .deploy::<counter_client::CounterProgram>(code_id, b"salt-counter".to_vec())
        .new(20_000_000_000)
        .await
        .unwrap();

    let mut counter = program.counter();

    let result = counter
        .contribute(None)
        .with_value(10_000_000_000)
        .await
        .unwrap();

    assert_eq!(result, 10_000_000_000);

    let value = counter.get_value().await.unwrap();
    assert_eq!(value, 10_000_000_000);
}
