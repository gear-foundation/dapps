use extended_vft_client::vft::io as vft_io;
use sails_rs::{
    calls::*,
    gtest::{calls::*, Program, System},
    ActorId, Encode, U256,
};

use staking_client::traits::*;

const ACTOR_IDS: &[u64] = &[40, 41, 42];

fn init_fungible_token(sys: &System) -> (ActorId, Program<'_>) {
    let vft = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/release/extended_vft.opt.wasm",
    );
    let payload = ("Name".to_string(), "Symbol".to_string(), 10_u8);
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = vft.send_bytes(ACTOR_IDS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    (vft.id(), vft)
}

fn mint_ft(vft: &Program<'_>, sys: &System, to: ActorId, value: U256) {
    let encoded_request = vft_io::Mint::encode_call(to, value);
    let mid = vft.send_bytes(ACTOR_IDS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn approve_ft(vft: &Program<'_>, sys: &System, from: u64, to: ActorId, value: U256) {
    let encoded_request = vft_io::Approve::encode_call(to, value);
    let mid = vft.send_bytes(from, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn ft_balance_of(program: &Program<'_>, sys: &System, account: ActorId) -> U256 {
    let encoded_request = vft_io::BalanceOf::encode_call(account);
    let mid = program.send_bytes(ACTOR_IDS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    vft_io::BalanceOf::decode_reply(res.log[0].payload()).unwrap()
}

#[tokio::test]
async fn test_stake() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_IDS[0], 100_000_000_000_000);
    system.mint_to(ACTOR_IDS[1], 100_000_000_000_000);
    system.mint_to(ACTOR_IDS[2], 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_IDS[0].into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(staking::WASM_BINARY);

    let program_factory = staking_client::StakingFactory::new(remoting.clone());

    let (vft_id, vft_program) = init_fungible_token(remoting.system());

    let program_id = program_factory
        .new(vft_id, 30000, 1000) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    mint_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[0].into(),
        1000.into(),
    );
    approve_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[0],
        program_id,
        1000.into(),
    );

    let mut service_client = staking_client::Staking::new(remoting.clone());
    service_client
        .stake(1000)
        .send_recv(program_id)
        .await
        .unwrap();

    let total_staked = service_client
        .total_staked()
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(total_staked, 1000);

    let balance = ft_balance_of(&vft_program, remoting.system(), ACTOR_IDS[0].into());
    assert_eq!(balance, 0.into());

    mint_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[1].into(),
        3000.into(),
    );
    approve_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[1],
        program_id,
        3000.into(),
    );

    service_client
        .stake(3000)
        .with_args(GTestArgs::new(ACTOR_IDS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let total_staked = service_client
        .total_staked()
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(total_staked, 4000);

    let balance = ft_balance_of(&vft_program, remoting.system(), ACTOR_IDS[0].into());
    assert_eq!(balance, 0.into());
}

#[tokio::test]
async fn test_get_reward() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_IDS[0], 100_000_000_000_000);
    system.mint_to(ACTOR_IDS[1], 100_000_000_000_000);
    system.mint_to(ACTOR_IDS[2], 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_IDS[0].into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(staking::WASM_BINARY);

    let program_factory = staking_client::StakingFactory::new(remoting.clone());

    let (vft_id, vft_program) = init_fungible_token(remoting.system());

    let program_id = program_factory
        .new(vft_id, 50000, 1000) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    mint_ft(&vft_program, remoting.system(), program_id, 100_000.into());
    mint_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[0].into(),
        1000.into(),
    );
    approve_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[0],
        program_id,
        1000.into(),
    );

    let mut service_client = staking_client::Staking::new(remoting.clone());
    service_client
        .stake(1000)
        .send_recv(program_id)
        .await
        .unwrap();

    let total_staked = service_client
        .total_staked()
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(total_staked, 1000);

    let balance = ft_balance_of(&vft_program, remoting.system(), ACTOR_IDS[0].into());
    assert_eq!(balance, 0.into());

    remoting.system().run_next_block();
    remoting.system().run_next_block();

    mint_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[1].into(),
        1000.into(),
    );
    approve_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[1],
        program_id,
        1000.into(),
    );

    let mut service_client = staking_client::Staking::new(remoting.clone());
    service_client
        .stake(1000)
        .with_args(GTestArgs::new(ACTOR_IDS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let total_staked = service_client
        .total_staked()
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(total_staked, 2000);

    remoting.system().run_next_block();
    remoting.system().run_next_block();

    service_client
        .get_reward()
        .with_args(GTestArgs::new(ACTOR_IDS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let reward_balance_1 = ft_balance_of(&vft_program, remoting.system(), ACTOR_IDS[1].into());

    service_client
        .get_reward()
        .send_recv(program_id)
        .await
        .unwrap();

    let reward_balance_2 = ft_balance_of(&vft_program, remoting.system(), ACTOR_IDS[0].into());
    assert!(reward_balance_2 > reward_balance_1);
}

#[tokio::test]
async fn test_withdraw() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(ACTOR_IDS[0], 100_000_000_000_000);
    system.mint_to(ACTOR_IDS[1], 100_000_000_000_000);
    system.mint_to(ACTOR_IDS[2], 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_IDS[0].into());

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(staking::WASM_BINARY);

    let program_factory = staking_client::StakingFactory::new(remoting.clone());

    let (vft_id, vft_program) = init_fungible_token(remoting.system());

    let program_id = program_factory
        .new(vft_id, 30000, 1000) // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    mint_ft(&vft_program, remoting.system(), program_id, 100_000.into());
    mint_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[0].into(),
        1000.into(),
    );
    approve_ft(
        &vft_program,
        remoting.system(),
        ACTOR_IDS[0],
        program_id,
        1000.into(),
    );

    let mut service_client = staking_client::Staking::new(remoting.clone());
    service_client
        .stake(1000)
        .send_recv(program_id)
        .await
        .unwrap();

    let total_staked = service_client
        .total_staked()
        .recv(program_id)
        .await
        .unwrap();
    assert_eq!(total_staked, 1000);

    let balance = ft_balance_of(&vft_program, remoting.system(), ACTOR_IDS[0].into());
    assert_eq!(balance, 0.into());

    remoting.system().run_next_block();
    remoting.system().run_next_block();

    service_client
        .withdraw(500)
        .send_recv(program_id)
        .await
        .unwrap();

    let balance = ft_balance_of(&vft_program, remoting.system(), ACTOR_IDS[0].into());
    assert_ne!(balance, 0.into());

    let stakers = service_client.stakers().recv(program_id).await.unwrap();
    assert_eq!(stakers[0].1.balance, 500);
    assert_ne!(stakers[0].1.reward_allowed, 0);
}
