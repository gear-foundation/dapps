use sails_rs::{
    calls::*,
    gtest::{System, calls::*},
};

use pts_client::traits::*;

const ACTOR_ID: u64 = 42;
const USER_ID: u64 = 43;

#[tokio::test]
async fn test_transfer() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_ID, 100_000_000_000_000);
    system.mint_to(USER_ID, 100_000_000_000_000);
    let remoting = GTestRemoting::new(system, ACTOR_ID.into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(pts::WASM_BINARY);

    let program_factory = pts_client::PtsFactory::new(remoting.clone());

    let program_id = program_factory
        .new(1_000, 60_000) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = pts_client::Pts::new(remoting.clone());

    service_client
        .get_accural()
        .with_args(|args| args.with_actor_id(USER_ID.into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let balance = service_client
        .get_balance(USER_ID.into())
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(balance, 1_000);

    service_client
        .transfer(USER_ID.into(), ACTOR_ID.into(), 1_000)
        .send_recv(program_id)
        .await
        .unwrap();

    let balance = service_client
        .get_balance(USER_ID.into())
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(balance, 0);
    let balance = service_client
        .get_balance(ACTOR_ID.into())
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(balance, 1_000);

    service_client
        .transfer(ACTOR_ID.into(), USER_ID.into(), 1_000)
        .send_recv(program_id)
        .await
        .unwrap();

    let balance = service_client
        .get_balance(ACTOR_ID.into())
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(balance, 0);
    let balance = service_client
        .get_balance(USER_ID.into())
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(balance, 1_000);
}
