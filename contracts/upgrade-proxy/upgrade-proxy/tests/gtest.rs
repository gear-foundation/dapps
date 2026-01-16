use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{ActorId, gtest::System};

use counter_client::Counter as ClientCounter;
use counter_client::CounterCtors;
use counter_client::counter::{self, Counter};

use proxy_client::Proxy as ClientProxy;
use proxy_client::ProxyCtors;
use proxy_client::proxy::Proxy;

use ::counter::WASM_BINARY as COUNTER_WASM_BINARY;
use proxy::WASM_BINARY as PROXY_WASM_BINARY;

const ACTOR_ID: u64 = 42;
const USERS: [u64; 5] = [43, 44, 45, 46, 47];

#[tokio::test]
async fn send_msg_to_counter() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    // fund admin + users
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);
    for u in USERS {
        system.mint_to(u, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, ACTOR_ID.into());

    // deploy Counter
    let counter_code_id = env.system().submit_code(COUNTER_WASM_BINARY);
    let limit = 20_000_000_000;

    let counter_program = env
        .deploy::<counter_client::CounterProgram>(counter_code_id, b"salt-counter".to_vec())
        .new(limit)
        .await
        .unwrap();
    let counter_id: ActorId = counter_program.id();

    // deploy Proxy
    let proxy_code_id = env.system().submit_code(PROXY_WASM_BINARY);
    let proxy_program = env
        .deploy::<proxy_client::ProxyProgram>(proxy_code_id, b"salt-proxy".to_vec())
        .new(counter_id, ACTOR_ID.into())
        .await
        .unwrap();
    let proxy_id: ActorId = proxy_program.id();

    // set proxy in Counter
    let mut counter = counter_program.counter();
    counter.set_proxy(Some(proxy_id)).await.unwrap();

    let mut proxy = proxy_program.proxy();

    // contribute via proxy
    let mut expected: u128 = 0;
    for u in USERS {
        let payload =
            counter::io::Contribute::encode_params_with_prefix("Counter", Some(ActorId::from(u)));

        let reply_bytes = proxy
            .execute_msg(payload)
            .with_value(10_000_000_000)
            .with_actor_id(ActorId::from(u))
            .await
            .unwrap();

        let reply =
            counter::io::Contribute::decode_reply_with_prefix("Counter", reply_bytes).unwrap();
        expected += 10_000_000_000;
        assert_eq!(reply, expected);
    }

    // distribute via proxy
    let payload =
        counter::io::Distribute::encode_params_with_prefix("Counter", Some(ACTOR_ID.into()));
    proxy.execute_msg(payload).await.unwrap();

    // read state via proxy
    let payload = counter::io::GetValue::encode_params_with_prefix("Counter");
    let reply_bytes = proxy.read_state(payload).await.unwrap();
    let value = counter::io::GetValue::decode_reply_with_prefix("Counter", reply_bytes).unwrap();
    assert_eq!(value, 0);
}
