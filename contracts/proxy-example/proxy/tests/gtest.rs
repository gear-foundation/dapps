use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use counter_client::traits::*;
use proxy_client::traits::*;
const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn send_msg_to_counter() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit counter code into the system
    let counter_code_id = remoting.system().submit_code(counter::WASM_BINARY);
    let counter_factory = counter_client::CounterFactory::new(remoting.clone());

    let counter_id = counter_factory
        .new()
        .send_recv(counter_code_id, b"salt")
        .await
        .unwrap();

    let mut counter_client = counter_client::Counter::new(remoting.clone());

    // Submit proxy code into the system
    let proxy_code_id = remoting.system().submit_code(proxy::WASM_BINARY);

    let proxy_factory = proxy_client::ProxyFactory::new(remoting.clone());

    let proxy_id = proxy_factory
        .new(counter_id, ACTOR_ID.into())
        .send_recv(proxy_code_id, b"salt")
        .await
        .unwrap();

    let mut proxy_client = proxy_client::Proxy::new(remoting.clone());

    // Set proxy to counter
    counter_client
        .set_proxy(Some(proxy_id))
        .send_recv(counter_id)
        .await
        .unwrap();

    // increment through proxy
    let payload_bytes = counter_client::counter::io::Increment::encode_call();
    let reply_bytes = proxy_client
        .execute(payload_bytes)
        .send_recv(proxy_id)
        .await
        .unwrap();

    let reply = counter_client::counter::io::Increment::decode_reply(reply_bytes).unwrap();

    assert_eq!(reply, 1);

    // increment through proxy
    let payload_bytes = counter_client::counter::io::Increment::encode_call();
    let reply_bytes = proxy_client
        .execute(payload_bytes)
        .send_recv(proxy_id)
        .await
        .unwrap();

    let reply = counter_client::counter::io::Increment::decode_reply(reply_bytes).unwrap();

    assert_eq!(reply, 2);

    // decrement through proxy
    let payload_bytes = counter_client::counter::io::Decrement::encode_call();
    let reply_bytes = proxy_client
        .execute(payload_bytes)
        .send_recv(proxy_id)
        .await
        .unwrap();

    let reply = counter_client::counter::io::Decrement::decode_reply(reply_bytes).unwrap();

    assert_eq!(reply, 1);

    // Read state
    let payload_bytes = counter_client::counter::io::GetValue::encode_call();
    let reply_bytes = proxy_client
        .execute(payload_bytes)
        .send_recv(proxy_id)
        .await
        .unwrap();

    let reply = counter_client::counter::io::GetValue::decode_reply(reply_bytes).unwrap();
    assert_eq!(reply, 1);
}
