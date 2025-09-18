use oracle_client::traits::*;
use sails_rs::{
    calls::*,
    gtest::{calls::*, Program, System},
    ActorId, Encode,
};

const ACTOR_ID: u64 = 42;

#[derive(Encode)]
#[codec(crate = sails_rs::scale_codec)]
pub struct Random {
    pub randomness: u128,
    pub signature: String,
    pub prev_signature: String,
}

fn init_randomness(sys: &System) -> (ActorId, Program<'_>) {
    let randomness = Program::from_file(sys, "../target/wasm32-gear/release/randomness.opt.wasm");
    let payload: ActorId = ACTOR_ID.into();
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = randomness.send_bytes(ACTOR_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

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

    (randomness.id(), randomness)
}

#[tokio::test]
async fn test_request_value() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(oracle::WASM_BINARY);

    let program_factory = oracle_client::OracleFactory::new(remoting.clone());
    let (rand_id, _) = init_randomness(remoting.system());

    let program_id = program_factory
        .new(ACTOR_ID.into(), rand_id, None) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = oracle_client::Oracle::new(remoting.clone());

    let result = service_client
        .request_value()
        .send_recv(program_id)
        .await
        .unwrap();

    assert_eq!(result, 100);
}
