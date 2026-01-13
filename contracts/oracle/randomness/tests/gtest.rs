use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{gtest::System, ActorId};

use randomness_client::randomness::Randomness;
use randomness_client::Random;
use randomness_client::Randomness as ClientRandomness;
use randomness_client::RandomnessCtors;

use ::randomness::WASM_BINARY;

const ACTOR_ID: u64 = 42;

async fn deploy_randomness(
    env: &GtestEnv,
) -> Actor<randomness_client::RandomnessProgram, GtestEnv> {
    let code_id = env.system().submit_code(WASM_BINARY);

    env.deploy::<randomness_client::RandomnessProgram>(code_id, b"salt-randomness".to_vec())
        .new(ActorId::from(ACTOR_ID))
        .await
        .unwrap()
}

#[tokio::test]
async fn test_set_random_value() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ACTOR_ID.into());
    let program = deploy_randomness(&env).await;

    // сервис (название обычно совпадает с сервисом в IDL; чаще всего это `.randomness()`)
    let mut rnd = program.randomness();

    rnd.set_random_value(
        1,
        Random {
            randomness: 100,
            signature: "Signature".to_string(),
            prev_signature: "PrevSignature".to_string(),
        },
    )
    .await
    .unwrap();

    let values = rnd.get_values().await.unwrap();

    assert_eq!(values.len(), 1);
    assert_eq!(values[0].1.randomness, 100);
    assert_eq!(values[0].1.signature, "Signature".to_string());
    assert_eq!(values[0].1.prev_signature, "PrevSignature".to_string());
}

#[tokio::test]
async fn test_get_last_round() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ACTOR_ID.into());
    let program = deploy_randomness(&env).await;

    let mut rnd = program.randomness();

    rnd.set_random_value(
        1,
        Random {
            randomness: 100,
            signature: "Signature".to_string(),
            prev_signature: "PrevSignature".to_string(),
        },
    )
    .await
    .unwrap();

    // в новом API это обычно просто .await (без send_recv)
    let last = rnd.get_last_round_with_random_value().await.unwrap();

    // у тебя раньше было `assert_eq!(result.1, 100);`
    assert_eq!(last.1, 100);
}

#[tokio::test]
async fn test_update_manager() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, ACTOR_ID.into());
    let program = deploy_randomness(&env).await;

    let mut rnd = program.randomness();

    rnd.update_manager(ActorId::from(10_u64)).await.unwrap();

    let manager = rnd.get_manager().await.unwrap();
    assert_eq!(manager, ActorId::from(10_u64));
}
