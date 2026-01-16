use oracle_client::Oracle as ClientOracle;
use oracle_client::OracleCtors;
use oracle_client::oracle::Oracle;

use sails_rs::gtest::Program;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{ActorId, Encode};
use sails_rs::{client::*, gtest::*};

const ACTOR_ID: u64 = 42;

#[derive(Encode)]
#[codec(crate = sails_rs::scale_codec)]
pub struct Random {
    pub randomness: u128,
    pub signature: String,
    pub prev_signature: String,
}

fn init_randomness(sys: &System) -> ActorId {
    let randomness = Program::from_file(sys, "../target/wasm32-gear/release/randomness.opt.wasm");

    // ctor: New(ACTOR_ID)
    let payload: ActorId = ACTOR_ID.into();
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = randomness.send_bytes(ACTOR_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    // call: Randomness::SetRandomValue((1, Random{...}))
    let payload: (u128, Random) = (
        1,
        Random {
            randomness: 100,
            signature: "Signature".to_string(),
            prev_signature: "PrevSignature".to_string(),
        },
    );

    let encoded_request = [
        "Randomness".encode(),
        "SetRandomValue".encode(),
        payload.encode(),
    ]
    .concat();

    let mid = randomness.send_bytes(ACTOR_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    randomness.id()
}

#[tokio::test]
async fn test_request_value() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let oracle_code_id = system.submit_code(oracle::WASM_BINARY);

    let env = GtestEnv::new(system, ACTOR_ID.into());

    let rand_id = init_randomness(env.system());

    let oracle_program = env
        .deploy::<oracle_client::OracleProgram>(oracle_code_id, b"salt-oracle".to_vec())
        .new(ACTOR_ID.into(), rand_id, None)
        .await
        .unwrap();

    let mut oracle = oracle_program.oracle();

    let result = oracle.request_value().await.unwrap();
    assert_eq!(result, 100);
}
