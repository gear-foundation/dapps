use sails_rs::client::*;
use sails_rs::gtest::System;

use pts_client::Pts as ClientPts;
use pts_client::PtsCtors;
use pts_client::pts::Pts;

const ACTOR_ID: u64 = 42;
const USER_ID: u64 = 43;

#[tokio::test]
async fn test_transfer() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    system.mint_to(ACTOR_ID, 100_000_000_000_000);
    system.mint_to(USER_ID, 100_000_000_000_000);

    let env = GtestEnv::new(system, ACTOR_ID.into());

    let code_id = env.system().submit_code(::pts::WASM_BINARY);

    let pts_program = env
        .deploy::<pts_client::PtsProgram>(code_id, b"salt".to_vec())
        .new(1_000u128, 60_000u64)
        .await
        .unwrap();

    let mut pts = pts_program.pts();

    pts.get_accural()
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let balance = pts.get_balance(USER_ID.into()).await.unwrap();
    assert_eq!(balance, 1_000);

    pts.transfer(USER_ID.into(), ACTOR_ID.into(), 1_000)
        .with_actor_id(USER_ID.into())
        .await
        .unwrap();

    let balance = pts.get_balance(USER_ID.into()).await.unwrap();
    assert_eq!(balance, 0);

    let balance = pts.get_balance(ACTOR_ID.into()).await.unwrap();
    assert_eq!(balance, 1_000);

    pts.transfer(ACTOR_ID.into(), USER_ID.into(), 1_000)
        .await
        .unwrap();

    let balance = pts.get_balance(ACTOR_ID.into()).await.unwrap();
    assert_eq!(balance, 0);

    let balance = pts.get_balance(USER_ID.into()).await.unwrap();
    assert_eq!(balance, 1_000);
}
