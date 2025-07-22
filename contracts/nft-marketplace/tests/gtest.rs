use extended_vft_client::vft::io as vft_io;
use extended_vnft_client::vnft::io as vnft_io;
use extended_vnft_client::TokenMetadata;
use nft_marketplace_client::traits::*;
use sails_rs::gtest::Program;
use sails_rs::prelude::*;
use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
    ActorId, Encode,
};

const USERS: &[u64] = &[3, 4, 5, 6];

fn init_fungible_token(sys: &System, minter: ActorId) -> (ActorId, Program<'_>) {
    let vft = Program::from_file(sys, "../target/wasm32-gear/release/extended_vft.opt.wasm");
    let payload = ("Name".to_string(), "Symbol".to_string(), 10_u8);
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = vft.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    let encoded_request = vft_io::GrantMinterRole::encode_call(minter);
    let mid = vft.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    (vft.id(), vft)
}

fn init_non_fungible_token(sys: &System, minter: ActorId) -> (ActorId, Program<'_>) {
    let vnft = Program::from_file(sys, "../target/wasm32-gear/release/extended_vnft.opt.wasm");
    let payload = ("Name".to_string(), "Symbol".to_string());
    let encoded_request = ["New".encode(), payload.encode()].concat();
    let mid = vnft.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    let encoded_request = vnft_io::GrantMinterRole::encode_call(minter);
    let mid = vnft.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    (vnft.id(), vnft)
}

fn mint_ft(vft: &Program<'_>, sys: &System, to: ActorId, value: U256) {
    let encoded_request = vft_io::Mint::encode_call(to, value);
    let mid = vft.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn mint_nft(vnft: &Program<'_>, sys: &System, to: ActorId) {
    let metadata = TokenMetadata {
        name: "NftName".to_string(),
        description: "NftDescription".to_string(),
        media: "NftMedia".to_string(),
        reference: "NftReference".to_string(),
    };
    let encoded_request = vnft_io::Mint::encode_call(to, metadata);
    let mid = vnft.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn approve_ft(vft: &Program<'_>, sys: &System, from: u64, to: ActorId, value: U256) {
    let encoded_request = vft_io::Approve::encode_call(to, value);
    let mid = vft.send_bytes(from, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn approve_nft(vnft: &Program<'_>, sys: &System, from: u64, to: ActorId, token_id: U256) {
    let encoded_request = vnft_io::Approve::encode_call(to, token_id);
    let mid = vnft.send_bytes(from, encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
}

fn ft_balance_of(program: &Program<'_>, sys: &System, account: ActorId) -> U256 {
    let encoded_request = vft_io::BalanceOf::encode_call(account);
    let mid = program.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    vft_io::BalanceOf::decode_reply(res.log[0].payload()).unwrap()
}
fn nft_balance_of(program: &Program<'_>, sys: &System, account: ActorId) -> U256 {
    let encoded_request = vnft_io::BalanceOf::encode_call(account);
    let mid = program.send_bytes(USERS[0], encoded_request);
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));
    vnft_io::BalanceOf::decode_reply(res.log[0].payload()).unwrap()
}

#[tokio::test]
async fn success_buy_with_native_tokens() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(nft_marketplace::WASM_BINARY);

    let program_factory = nft_marketplace_client::NftMarketplaceFactory::new(remoting.clone());

    let program_id = program_factory
        .new(USERS[0].into())
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = nft_marketplace_client::NftMarketplace::new(remoting.clone());

    let (nft_contract_id, nft_program) =
        init_non_fungible_token(remoting.system(), USERS[0].into());
    service_client
        .add_nft_contract(nft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();

    mint_nft(&nft_program, remoting.system(), USERS[0].into());
    approve_nft(
        &nft_program,
        remoting.system(),
        USERS[0],
        program_id,
        0.into(),
    );

    service_client
        .add_market_data(nft_contract_id, None, 0.into(), Some(10_000_000_000_000))
        .send_recv(program_id)
        .await
        .unwrap();

    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items.is_empty());

    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[0].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[1].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), program_id),
        1.into()
    );
    let old_balance_user_0 = remoting.system().balance_of(USERS[0]);
    let old_balance_user_1 = remoting.system().balance_of(USERS[1]);

    service_client
        .buy_item(nft_contract_id, 0.into())
        .with_value(10_000_000_000_000)
        .with_args(|args| args.with_actor_id(USERS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let new_balance_user_0 = remoting.system().balance_of(USERS[0]);
    let new_balance_user_1 = remoting.system().balance_of(USERS[1]);
    assert_eq!(new_balance_user_0 - old_balance_user_0, 10_000_000_000_000);
    assert!(old_balance_user_1 - new_balance_user_1 > 10_000_000_000_000); // gas costs are included

    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert_eq!(market.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_buy_with_fungible_tokens() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(nft_marketplace::WASM_BINARY);

    let program_factory = nft_marketplace_client::NftMarketplaceFactory::new(remoting.clone());

    let program_id = program_factory
        .new(USERS[0].into())
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = nft_marketplace_client::NftMarketplace::new(remoting.clone());

    let (nft_contract_id, nft_program) =
        init_non_fungible_token(remoting.system(), USERS[0].into());
    let (ft_contract_id, ft_program) = init_fungible_token(remoting.system(), USERS[0].into());

    service_client
        .add_nft_contract(nft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();
    service_client
        .add_ft_contract(ft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();

    mint_nft(&nft_program, remoting.system(), USERS[0].into());
    approve_nft(
        &nft_program,
        remoting.system(),
        USERS[0],
        program_id,
        0.into(),
    );

    mint_ft(
        &ft_program,
        remoting.system(),
        USERS[1].into(),
        10_000_000_000_000_u128.into(),
    );
    approve_ft(
        &ft_program,
        remoting.system(),
        USERS[1],
        program_id,
        10_000_000_000_000_u128.into(),
    );
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[1].into()),
        10_000_000_000_000_u128.into()
    );

    service_client
        .add_market_data(
            nft_contract_id,
            Some(ft_contract_id),
            0.into(),
            Some(10_000_000_000_000),
        )
        .send_recv(program_id)
        .await
        .unwrap();

    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items.is_empty());

    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[0].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[1].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), program_id),
        1.into()
    );

    service_client
        .buy_item(nft_contract_id, 0.into())
        .with_args(|args| args.with_actor_id(USERS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[1].into()),
        0_u128.into()
    );
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[0].into()),
        10_000_000_000_000_u128.into()
    );

    // let market = service_client.get_market().recv(program_id).await.unwrap();
    // assert_eq!(market.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_offer_native_tokens() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(nft_marketplace::WASM_BINARY);

    let program_factory = nft_marketplace_client::NftMarketplaceFactory::new(remoting.clone());

    let program_id = program_factory
        .new(USERS[0].into())
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = nft_marketplace_client::NftMarketplace::new(remoting.clone());

    let (nft_contract_id, nft_program) =
        init_non_fungible_token(remoting.system(), USERS[0].into());
    service_client
        .add_nft_contract(nft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();

    mint_nft(&nft_program, remoting.system(), USERS[0].into());
    approve_nft(
        &nft_program,
        remoting.system(),
        USERS[0],
        program_id,
        0.into(),
    );

    service_client
        .add_market_data(nft_contract_id, None, 0.into(), None)
        .send_recv(program_id)
        .await
        .unwrap();

    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items.is_empty());

    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[0].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), program_id),
        1.into()
    );

    let old_balance_user_1 = remoting.system().balance_of(USERS[1]);

    service_client
        .add_offer(nft_contract_id, None, 0.into(), 10_000_000_000_000)
        .with_value(10_000_000_000_000)
        .with_args(|args| args.with_actor_id(USERS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items[0].1.offers.is_empty());
    let old_balance_user_0 = remoting.system().balance_of(USERS[0]);

    service_client
        .accept_offer(nft_contract_id, None, 0.into(), 10_000_000_000_000)
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(market.items[0].1.offers.is_empty());

    let new_balance_user_0 = remoting.system().balance_of(USERS[0]);
    let new_balance_user_1 = remoting.system().balance_of(USERS[1]);

    assert!(new_balance_user_0 - old_balance_user_0 > 9_000_000_000_000);
    assert!(old_balance_user_1 - new_balance_user_1 > 10_000_000_000_000);

    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert_eq!(market.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_offer_with_fungible_tokens() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(nft_marketplace::WASM_BINARY);

    let program_factory = nft_marketplace_client::NftMarketplaceFactory::new(remoting.clone());

    let program_id = program_factory
        .new(USERS[0].into())
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = nft_marketplace_client::NftMarketplace::new(remoting.clone());

    let (nft_contract_id, nft_program) =
        init_non_fungible_token(remoting.system(), USERS[0].into());
    let (ft_contract_id, ft_program) = init_fungible_token(remoting.system(), USERS[0].into());

    service_client
        .add_nft_contract(nft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();
    service_client
        .add_ft_contract(ft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();

    mint_nft(&nft_program, remoting.system(), USERS[0].into());
    approve_nft(
        &nft_program,
        remoting.system(),
        USERS[0],
        program_id,
        0.into(),
    );

    mint_ft(
        &ft_program,
        remoting.system(),
        USERS[1].into(),
        10_000_000_000_000_u128.into(),
    );
    approve_ft(
        &ft_program,
        remoting.system(),
        USERS[1],
        program_id,
        10_000_000_000_000_u128.into(),
    );
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[1].into()),
        10_000_000_000_000_u128.into()
    );

    service_client
        .add_market_data(nft_contract_id, Some(ft_contract_id), 0.into(), None)
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items.is_empty());
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[0].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), program_id),
        1.into()
    );

    service_client
        .add_offer(
            nft_contract_id,
            Some(ft_contract_id),
            0.into(),
            10_000_000_000_000,
        )
        .with_args(|args| args.with_actor_id(USERS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items[0].1.offers.is_empty());
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[1].into()),
        0_u128.into()
    );
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), program_id),
        10_000_000_000_000_u128.into()
    );

    service_client
        .accept_offer(
            nft_contract_id,
            Some(ft_contract_id),
            0.into(),
            10_000_000_000_000,
        )
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(market.items[0].1.offers.is_empty());

    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[0].into()),
        10_000_000_000_000_u128.into()
    );
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert_eq!(market.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_auction_with_native_tokens() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(nft_marketplace::WASM_BINARY);

    let program_factory = nft_marketplace_client::NftMarketplaceFactory::new(remoting.clone());

    let program_id = program_factory
        .new(USERS[0].into())
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = nft_marketplace_client::NftMarketplace::new(remoting.clone());

    let (nft_contract_id, nft_program) =
        init_non_fungible_token(remoting.system(), USERS[0].into());
    service_client
        .add_nft_contract(nft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();

    mint_nft(&nft_program, remoting.system(), USERS[0].into());
    approve_nft(
        &nft_program,
        remoting.system(),
        USERS[0],
        program_id,
        0.into(),
    );

    service_client
        .add_market_data(nft_contract_id, None, 0.into(), None)
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items.is_empty());

    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[0].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), program_id),
        1.into()
    );

    service_client
        .create_auction(nft_contract_id, None, 0.into(), 10_000_000_000_000, 300_000)
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(market.items[0].1.auction.is_some());
    service_client
        .add_bid(nft_contract_id, 0.into(), 15_000_000_000_000)
        .with_value(15_000_000_000_000)
        .with_args(|args| args.with_actor_id(USERS[2].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    let old_balance_user = remoting.system().balance_of(USERS[2]);
    service_client
        .add_bid(nft_contract_id, 0.into(), 20_000_000_000_000)
        .with_value(20_000_000_000_000)
        .with_args(|args| args.with_actor_id(USERS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();
    let new_balance_user = remoting.system().balance_of(USERS[2]);
    assert_eq!(new_balance_user - old_balance_user, 15_000_000_000_000);

    remoting
        .system()
        .run_to_block(remoting.system().block_height() + 300_000 / 3_000);
    let old_balance_user = remoting.system().balance_of(USERS[0]);

    service_client
        .settle_auction(nft_contract_id, 0.into())
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(market.items[0].1.auction.is_none());
    let new_balance_user = remoting.system().balance_of(USERS[0]);
    assert!(new_balance_user - old_balance_user > 19_000_000_000_000);

    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert_eq!(market.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_auction_with_fungible_tokens() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(nft_marketplace::WASM_BINARY);

    let program_factory = nft_marketplace_client::NftMarketplaceFactory::new(remoting.clone());

    let program_id = program_factory
        .new(USERS[0].into())
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = nft_marketplace_client::NftMarketplace::new(remoting.clone());

    let (nft_contract_id, nft_program) =
        init_non_fungible_token(remoting.system(), USERS[0].into());
    let (ft_contract_id, ft_program) = init_fungible_token(remoting.system(), USERS[0].into());

    service_client
        .add_nft_contract(nft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();
    service_client
        .add_ft_contract(ft_contract_id)
        .send_recv(program_id)
        .await
        .unwrap();

    mint_nft(&nft_program, remoting.system(), USERS[0].into());
    approve_nft(
        &nft_program,
        remoting.system(),
        USERS[0],
        program_id,
        0.into(),
    );

    mint_ft(
        &ft_program,
        remoting.system(),
        USERS[1].into(),
        20_000_000_000_000_u128.into(),
    );
    approve_ft(
        &ft_program,
        remoting.system(),
        USERS[1],
        program_id,
        20_000_000_000_000_u128.into(),
    );
    mint_ft(
        &ft_program,
        remoting.system(),
        USERS[2].into(),
        15_000_000_000_000_u128.into(),
    );
    approve_ft(
        &ft_program,
        remoting.system(),
        USERS[2],
        program_id,
        15_000_000_000_000_u128.into(),
    );
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[1].into()),
        20_000_000_000_000_u128.into()
    );
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[2].into()),
        15_000_000_000_000_u128.into()
    );

    service_client
        .add_market_data(nft_contract_id, Some(ft_contract_id), 0.into(), None)
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(!market.items.is_empty());

    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), USERS[0].into()),
        0.into()
    );
    assert_eq!(
        nft_balance_of(&nft_program, remoting.system(), program_id),
        1.into()
    );

    service_client
        .create_auction(
            nft_contract_id,
            Some(ft_contract_id),
            0.into(),
            10_000_000_000_000,
            300_000,
        )
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(market.items[0].1.auction.is_some());

    service_client
        .add_bid(nft_contract_id, 0.into(), 15_000_000_000_000)
        .with_args(|args| args.with_actor_id(USERS[2].into()))
        .send_recv(program_id)
        .await
        .unwrap();
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[2].into()),
        0_u128.into()
    );
    service_client
        .add_bid(nft_contract_id, 0.into(), 20_000_000_000_000)
        .with_args(|args| args.with_actor_id(USERS[1].into()))
        .send_recv(program_id)
        .await
        .unwrap();
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[2].into()),
        15_000_000_000_000_u128.into()
    );

    remoting
        .system()
        .run_to_block(remoting.system().block_height() + 300_000 / 3_000);
    service_client
        .settle_auction(nft_contract_id, 0.into())
        .send_recv(program_id)
        .await
        .unwrap();
    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert!(market.items[0].1.auction.is_none());

    let market = service_client.get_market().recv(program_id).await.unwrap();
    assert_eq!(market.items[0].1.owner, USERS[1].into());
    assert_eq!(
        ft_balance_of(&ft_program, remoting.system(), USERS[0].into()),
        20_000_000_000_000_u128.into()
    );
}
