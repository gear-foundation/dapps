use dex_client::{
    traits::{Dex, DexFactory},
    Dex as DexClient, DexFactory as Factory,
};
use extended_vft_client::vft::io as vft_io;
use sails_rs::{
    calls::*,
    gtest::{calls::*, Program, System},
    ActorId, Encode, U256,
};

pub const USER_ID: u64 = 10;

fn init_fungible_token(sys: &System) -> (ActorId, Program<'_>) {
    let vft = Program::from_file(
        sys,
        "../target/wasm32-unknown-unknown/release/extended_vft.opt.wasm",
    );
    let payload = ("Name".to_string(), "Symbol".to_string(), 10_u8);
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = vft.send_bytes(USER_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    (vft.id(), vft)
}

fn mint(sys: &System, vft: &Program, amount: U256) {
    let encoded_request = vft_io::Mint::encode_call(USER_ID.into(), amount);
    let mid = vft.send_bytes(USER_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn approve_ft(vft: &Program<'_>, sys: &System, from: u64, to: ActorId, value: U256) {
    let encoded_request = vft_io::Approve::encode_call(to, value);
    let mid = vft.send_bytes(from, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn get_balance(sys: &System, vft: &Program, account: ActorId) -> U256 {
    let encoded_request = vft_io::BalanceOf::encode_call(account);
    let mid = vft.send_bytes(USER_ID, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    vft_io::BalanceOf::decode_reply(res.log[0].payload()).unwrap()
}

/// Test for adding liquidity to the DEX pool
#[tokio::test]
async fn test_add_liquidity() {
    let system = System::new();
    system.init_logger();

    // Mint some initial tokens for the user
    system.mint_to(USER_ID, 100_000_000_000_000);

    // Initialize the program space and upload the contract
    let program_space = GTestRemoting::new(system, USER_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../target/wasm32-unknown-unknown/release/dex.opt.wasm");

    // Create a new factory and initialize two fungible token contracts
    let dex_factory = Factory::new(program_space.clone());
    let (vft_id_a, vft_program_a) = init_fungible_token(program_space.system());
    let (vft_id_b, vft_program_b) = init_fungible_token(program_space.system());

    // Mint tokens to the fungible token contracts
    mint(program_space.system(), &vft_program_a, 100_000.into());
    mint(program_space.system(), &vft_program_b, 100_000.into());

    // Deploy the DEX contract
    let dex_id = dex_factory
        .new(vft_id_a, vft_id_b, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = DexClient::new(program_space.clone());

    // Approve the DEX contract to spend user tokens
    approve_ft(
        &vft_program_a,
        program_space.system(),
        USER_ID,
        dex_id,
        300.into(),
    );
    approve_ft(
        &vft_program_b,
        program_space.system(),
        USER_ID,
        dex_id,
        300.into(),
    );

    // Add initial liquidity to the DEX pool
    client
        .add_liquidity(100.into(), 100.into())
        .send_recv(dex_id)
        .await
        .unwrap();

    // Verify the state after adding liquidity
    let total_liquidity = client.total_liquidity().recv(dex_id).await.unwrap();
    assert_eq!(total_liquidity, 100.into(), "Total liquidity should match initial addition");
    let reserve_a = client.reserve_a().recv(dex_id).await.unwrap();
    assert_eq!(reserve_a, 100.into(), "Reserve A should match initial addition");
    let reserve_b = client.reserve_b().recv(dex_id).await.unwrap();
    assert_eq!(reserve_b, 100.into(), "Reserve B should match initial addition");

    // Add more liquidity to the DEX pool
    client
        .add_liquidity(200.into(), 200.into())
        .send_recv(dex_id)
        .await
        .unwrap();

    // Verify the updated state
    let total_liquidity = client.total_liquidity().recv(dex_id).await.unwrap();
    assert_eq!(total_liquidity, 300.into(), "Total liquidity should match after second addition");
    let reserve_a = client.reserve_a().recv(dex_id).await.unwrap();
    assert_eq!(reserve_a, 300.into(), "Reserve A should match after second addition");
    let reserve_b = client.reserve_b().recv(dex_id).await.unwrap();
    assert_eq!(reserve_b, 300.into(), "Reserve B should match after second addition");
}

/// Test for the `swap` functionality
#[tokio::test]
async fn test_swap() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, USER_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../target/wasm32-unknown-unknown/release/dex.opt.wasm");

    let dex_factory = Factory::new(program_space.clone());
    let (vft_id_a, vft_program_a) = init_fungible_token(program_space.system());
    let (vft_id_b, vft_program_b) = init_fungible_token(program_space.system());
    mint(program_space.system(), &vft_program_a, 500.into());
    mint(program_space.system(), &vft_program_b, 600.into());

    let dex_id = dex_factory
        .new(vft_id_a, vft_id_b, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = DexClient::new(program_space.clone());

    // Approve tokens for the DEX contract
    approve_ft(
        &vft_program_a,
        program_space.system(),
        USER_ID,
        dex_id,
        500.into(),
    );
    approve_ft(
        &vft_program_b,
        program_space.system(),
        USER_ID,
        dex_id,
        600.into(),
    );

    // Add liquidity to the pool
    client
        .add_liquidity(500.into(), 500.into())
        .send_recv(dex_id)
        .await
        .unwrap();

    let user_balance_a = get_balance(program_space.system(), &vft_program_a, USER_ID.into());
    assert_eq!(
        user_balance_a,
        0.into(),
        "User should receive token A after swap"
    );
    // Perform a swap
    client
        .swap(100.into(), true) // Swap 100 token B for token A
        .send_recv(dex_id)
        .await
        .unwrap();

    // Calculate expected reserves after the swap
    let reserve_a_before = U256::from(500);
    let reserve_b_before = U256::from(500);
    let in_amount = U256::from(100);

    // Expected calculations
    let out_amount = (in_amount * reserve_a_before) / (reserve_b_before + in_amount);
    let reserve_a_expected = reserve_a_before - out_amount;
    let reserve_b_expected = reserve_b_before + in_amount;

    // Check the updated reserves
    let reserve_a = client.reserve_a().recv(dex_id).await.unwrap();
    let reserve_b = client.reserve_b().recv(dex_id).await.unwrap();

    assert_eq!(
        reserve_a, reserve_a_expected,
        "Reserve A mismatch after swap"
    );
    assert_eq!(
        reserve_b, reserve_b_expected,
        "Reserve B mismatch after swap"
    );

    // Check that the user received token A
    let user_balance_a = get_balance(program_space.system(), &vft_program_a, USER_ID.into());
    assert!(
        user_balance_a > 0.into(),
        "User should receive token A after swap"
    );
}

/// Test for the `remove_liquidity` functionality
#[tokio::test]
async fn test_remove_liquidity() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER_ID, 100_000_000_000_000);
    let program_space = GTestRemoting::new(system, USER_ID.into());
    let code_id = program_space
        .system()
        .submit_code_file("../target/wasm32-unknown-unknown/release/dex.opt.wasm");

    let dex_factory = Factory::new(program_space.clone());
    let (vft_id_a, vft_program_a) = init_fungible_token(program_space.system());
    let (vft_id_b, vft_program_b) = init_fungible_token(program_space.system());
    mint(program_space.system(), &vft_program_a, 500.into());
    mint(program_space.system(), &vft_program_b, 500.into());

    let dex_id = dex_factory
        .new(vft_id_a, vft_id_b, None)
        .send_recv(code_id, "123")
        .await
        .unwrap();

    let mut client = DexClient::new(program_space.clone());
    // Approve tokens for the DEX contract
    approve_ft(
        &vft_program_a,
        program_space.system(),
        USER_ID,
        dex_id,
        1_000.into(),
    );
    approve_ft(
        &vft_program_b,
        program_space.system(),
        USER_ID,
        dex_id,
        1_000.into(),
    );

    // Add liquidity to the pool
    client
        .add_liquidity(500.into(), 500.into())
        .send_recv(dex_id)
        .await
        .unwrap();

    // Remove liquidity from the pool
    client
        .remove_liquidity(100.into()) // Remove 100 liquidity tokens
        .send_recv(dex_id)
        .await
        .unwrap();

    // Check the updated reserves
    let reserve_a = client.reserve_a().recv(dex_id).await.unwrap();
    let reserve_b = client.reserve_b().recv(dex_id).await.unwrap();

    assert!(
        reserve_a < 500.into(),
        "Reserve A should decrease after removing liquidity"
    );
    assert!(
        reserve_b < 500.into(),
        "Reserve B should decrease after removing liquidity"
    );

    // Check that the user received their tokens back
    let user_balance_a = get_balance(program_space.system(), &vft_program_a, USER_ID.into());
    let user_balance_b = get_balance(program_space.system(), &vft_program_b, USER_ID.into());

    assert!(
        user_balance_a > 0.into(),
        "User should receive token A after removing liquidity"
    );
    assert!(
        user_balance_b > 0.into(),
        "User should receive token B after removing liquidity"
    );
}
