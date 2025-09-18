use randomness_client::Random;
use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use randomness_client::traits::*;

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn test_set_random_value() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(randomness::WASM_BINARY);

    let program_factory = randomness_client::RandomnessFactory::new(remoting.clone());

    let program_id = program_factory
        .new(ACTOR_ID.into()) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = randomness_client::Randomness::new(remoting.clone());

    service_client
        .set_random_value(
            1,
            Random {
                randomness: 100,
                signature: "Signature".to_string(),
                prev_signature: "PrevSignature".to_string(),
            },
        )
        .send_recv(program_id)
        .await
        .unwrap();

    let result = service_client.get_values().recv(program_id).await.unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].1.randomness, 100);
    assert_eq!(result[0].1.signature, "Signature".to_string());
    assert_eq!(result[0].1.prev_signature, "PrevSignature".to_string());
}

#[tokio::test]
async fn test_get_last_round() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(randomness::WASM_BINARY);

    let program_factory = randomness_client::RandomnessFactory::new(remoting.clone());

    let program_id = program_factory
        .new(ACTOR_ID.into()) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = randomness_client::Randomness::new(remoting.clone());

    service_client
        .set_random_value(
            1,
            Random {
                randomness: 100,
                signature: "Signature".to_string(),
                prev_signature: "PrevSignature".to_string(),
            },
        )
        .send_recv(program_id)
        .await
        .unwrap();

    let result = service_client
        .get_last_round_with_random_value()
        .send_recv(program_id)
        .await
        .unwrap();

    assert_eq!(result.1, 100);
}

#[tokio::test]
async fn test_update_manager() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(randomness::WASM_BINARY);

    let program_factory = randomness_client::RandomnessFactory::new(remoting.clone());

    let program_id = program_factory
        .new(ACTOR_ID.into()) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = randomness_client::Randomness::new(remoting.clone());

    service_client
        .update_manager(10.into())
        .send_recv(program_id)
        .await
        .unwrap();

    let result = service_client.get_manager().recv(program_id).await.unwrap();

    assert_eq!(result, 10.into());
}
