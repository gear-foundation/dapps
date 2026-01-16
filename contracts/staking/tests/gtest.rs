use extended_vft_client::vft::Vft;
use extended_vft_client::{ExtendedVftClient, ExtendedVftClientCtors};
use staking_client::Staking as ClientStaking;
use staking_client::StakingCtors;
use staking_client::staking::Staking;

use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{ActorId, U256, gtest::System};

const ACTOR_IDS: [u64; 3] = [40, 41, 42];

fn mint_users(system: &System) {
    for id in ACTOR_IDS {
        system.mint_to(id, DEFAULT_USERS_INITIAL_BALANCE);
    }
}

fn run_blocks(system: &System, n: usize) {
    for _ in 0..n {
        system.run_next_block();
    }
}

#[tokio::test]
async fn test_stake() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    mint_users(&system);

    let env = GtestEnv::new(system, ACTOR_IDS[0].into());

    let vft_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vft.opt.wasm");

    let vft_program = env
        .deploy::<extended_vft_client::ExtendedVftClientProgram>(vft_code_id, b"salt-vft".to_vec())
        .new("Name".to_string(), "Symbol".to_string(), 10_u8)
        .await
        .unwrap();

    let vft_id: ActorId = vft_program.id();

    let staking_code_id = env.system().submit_code(staking::WASM_BINARY);
    let staking_program = env
        .deploy::<staking_client::StakingProgram>(staking_code_id, b"salt-staking".to_vec())
        .new(vft_id, 30_000, 1_000)
        .await
        .unwrap();

    let staking_id: ActorId = staking_program.id();

    let mut vft = vft_program.vft();
    let mut staking = staking_program.staking();

    vft.mint(ACTOR_IDS[0].into(), U256::from(1_000))
        .await
        .unwrap();
    vft.approve(staking_id, U256::from(1_000)).await.unwrap();

    staking.stake(1_000).await.unwrap();

    let total_staked = staking.total_staked().await.unwrap();
    assert_eq!(total_staked, 1_000);

    let bal0 = vft.balance_of(ACTOR_IDS[0].into()).await.unwrap();
    assert_eq!(bal0, 0.into());

    vft.mint(ACTOR_IDS[1].into(), U256::from(3_000))
        .await
        .unwrap();
    vft.approve(staking_id, U256::from(3_000))
        .with_actor_id(ACTOR_IDS[1].into())
        .await
        .unwrap();

    staking
        .stake(3_000)
        .with_actor_id(ACTOR_IDS[1].into())
        .await
        .unwrap();

    let total_staked = staking.total_staked().await.unwrap();
    assert_eq!(total_staked, 4_000);

    let bal0 = vft.balance_of(ACTOR_IDS[0].into()).await.unwrap();
    assert_eq!(bal0, 0.into());
}

#[tokio::test]
async fn test_get_reward() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    mint_users(&system);

    let env = GtestEnv::new(system, ACTOR_IDS[0].into());

    let vft_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vft.opt.wasm");

    let vft_program = env
        .deploy::<extended_vft_client::ExtendedVftClientProgram>(vft_code_id, b"salt-vft".to_vec())
        .new("Name".to_string(), "Symbol".to_string(), 10_u8)
        .await
        .unwrap();

    let vft_id: ActorId = vft_program.id();

    let staking_code_id = env.system().submit_code(staking::WASM_BINARY);
    let staking_program = env
        .deploy::<staking_client::StakingProgram>(staking_code_id, b"salt-staking".to_vec())
        .new(vft_id, 50_000, 1_000)
        .await
        .unwrap();

    let staking_id: ActorId = staking_program.id();

    let mut vft = vft_program.vft();
    let mut staking = staking_program.staking();

    vft.mint(staking_id, U256::from(100_000)).await.unwrap();

    vft.mint(ACTOR_IDS[0].into(), U256::from(1_000))
        .await
        .unwrap();
    vft.approve(staking_id, U256::from(1_000)).await.unwrap();
    staking.stake(1_000).await.unwrap();

    let total_staked = staking.total_staked().await.unwrap();
    assert_eq!(total_staked, 1_000);

    let bal0 = vft.balance_of(ACTOR_IDS[0].into()).await.unwrap();
    assert_eq!(bal0, 0.into());

    run_blocks(env.system(), 2);

    vft.mint(ACTOR_IDS[1].into(), U256::from(1_000))
        .await
        .unwrap();
    vft.approve(staking_id, U256::from(1_000))
        .with_actor_id(ACTOR_IDS[1].into())
        .await
        .unwrap();

    staking
        .stake(1_000)
        .with_actor_id(ACTOR_IDS[1].into())
        .await
        .unwrap();

    let total_staked = staking.total_staked().await.unwrap();
    assert_eq!(total_staked, 2_000);

    run_blocks(env.system(), 2);

    staking
        .get_reward()
        .with_actor_id(ACTOR_IDS[1].into())
        .await
        .unwrap();
    let reward_1 = vft.balance_of(ACTOR_IDS[1].into()).await.unwrap();

    staking.get_reward().await.unwrap();
    let reward_0 = vft.balance_of(ACTOR_IDS[0].into()).await.unwrap();

    assert!(reward_0 > reward_1);
}

#[tokio::test]
async fn test_withdraw() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    mint_users(&system);

    let env = GtestEnv::new(system, ACTOR_IDS[0].into());

    let vft_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vft.opt.wasm");

    let vft_program = env
        .deploy::<extended_vft_client::ExtendedVftClientProgram>(vft_code_id, b"salt-vft".to_vec())
        .new("Name".to_string(), "Symbol".to_string(), 10_u8)
        .await
        .unwrap();

    let vft_id: ActorId = vft_program.id();

    let staking_code_id = env.system().submit_code(staking::WASM_BINARY);
    let staking_program = env
        .deploy::<staking_client::StakingProgram>(staking_code_id, b"salt-staking".to_vec())
        .new(vft_id, 30_000, 1_000)
        .await
        .unwrap();

    let staking_id: ActorId = staking_program.id();

    let mut vft = vft_program.vft();
    let mut staking = staking_program.staking();

    vft.mint(staking_id, U256::from(100_000)).await.unwrap();

    vft.mint(ACTOR_IDS[0].into(), U256::from(1_000))
        .await
        .unwrap();
    vft.approve(staking_id, U256::from(1_000)).await.unwrap();

    staking.stake(1_000).await.unwrap();

    let total_staked = staking.total_staked().await.unwrap();
    assert_eq!(total_staked, 1_000);

    let bal0 = vft.balance_of(ACTOR_IDS[0].into()).await.unwrap();
    assert_eq!(bal0, 0.into());

    run_blocks(env.system(), 2);

    staking.withdraw(500).await.unwrap();

    let bal0 = vft.balance_of(ACTOR_IDS[0].into()).await.unwrap();
    assert!(bal0 > 0.into());

    let stakers = staking.stakers().await.unwrap();
    assert_eq!(stakers[0].1.balance, 500);
    assert_ne!(stakers[0].1.reward_allowed, 0);
}
