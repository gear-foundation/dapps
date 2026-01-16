use extended_vft_client::ExtendedVftClient;
use extended_vft_client::ExtendedVftClientCtors;
use extended_vft_client::vft::Vft;
use extended_vnft_client::ExtendedVnftClient;
use extended_vnft_client::ExtendedVnftClientCtors;
use extended_vnft_client::TokenMetadata;
use extended_vnft_client::vnft::Vnft;

use nft_marketplace_client::NftMarketplace as ClientNftMarketplace;
use nft_marketplace_client::NftMarketplaceCtors;
use nft_marketplace_client::nft_marketplace::NftMarketplace;
use sails_rs::client::*;
use sails_rs::gtest::System;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;

const USERS: &[u64] = &[3, 4, 5, 6];

const PRICE: u128 = 10_000_000_000_000;
const BID_15: u128 = 15_000_000_000_000;
const BID_20: u128 = 20_000_000_000_000;
const AUCTION_DURATION_MS: u32 = 300_000;
const BLOCK_TIME_MS: u32 = 3_000;

fn default_meta() -> TokenMetadata {
    TokenMetadata {
        name: "NftName".to_string(),
        description: "NftDescription".to_string(),
        media: "NftMedia".to_string(),
        reference: "NftReference".to_string(),
    }
}

async fn deploy_market_env() -> (
    GtestEnv,
    Actor<nft_marketplace_client::NftMarketplaceProgram, GtestEnv>,
) {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    for id in USERS {
        system.mint_to(*id, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, USERS[0].into());
    let market_code_id = env.system().submit_code(nft_marketplace::WASM_BINARY);

    let market = env
        .deploy::<nft_marketplace_client::NftMarketplaceProgram>(
            market_code_id,
            b"salt-market".to_vec(),
        )
        .new(USERS[0].into())
        .await
        .unwrap();

    (env, market)
}

async fn deploy_vnft(
    env: &GtestEnv,
) -> Actor<extended_vnft_client::ExtendedVnftClientProgram, GtestEnv> {
    let vnft_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vnft.opt.wasm");

    env.deploy::<extended_vnft_client::ExtendedVnftClientProgram>(
        vnft_code_id,
        b"salt-vnft".to_vec(),
    )
    .new("Name".to_string(), "Symbol".to_string())
    .await
    .unwrap()
}

async fn deploy_vft(
    env: &GtestEnv,
    salt: &[u8],
) -> Actor<extended_vft_client::ExtendedVftClientProgram, GtestEnv> {
    let vft_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vft.opt.wasm");

    env.deploy::<extended_vft_client::ExtendedVftClientProgram>(vft_code_id, salt.to_vec())
        .new("Name".to_string(), "Symbol".to_string(), 10_u8)
        .await
        .unwrap()
}

#[tokio::test]
async fn success_buy_with_native_tokens() {
    let (env, market_program) = deploy_market_env().await;
    let market_id = market_program.id();

    let vnft_program = deploy_vnft(&env).await;
    let vnft_id = vnft_program.id();

    let mut market = market_program.nft_marketplace();
    let mut vnft = vnft_program.vnft();

    market.add_nft_contract(vnft_id).await.unwrap();

    vnft.mint(USERS[0].into(), default_meta()).await.unwrap();
    vnft.approve(market_id, 0.into()).await.unwrap();

    market
        .add_market_data(vnft_id, None, 0.into(), Some(PRICE))
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items.is_empty());

    assert_eq!(vnft.balance_of(USERS[0].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(USERS[1].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(market_id).await.unwrap(), 1.into());

    let old0 = env.system().balance_of(USERS[0]);
    let old1 = env.system().balance_of(USERS[1]);

    market
        .buy_item(vnft_id, 0.into())
        .with_value(PRICE)
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    let new0 = env.system().balance_of(USERS[0]);
    let new1 = env.system().balance_of(USERS[1]);

    assert_eq!(new0 - old0, PRICE);
    assert!(old1 - new1 > PRICE);

    let m = market.get_market().await.unwrap();
    assert_eq!(m.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_buy_with_fungible_tokens() {
    let (env, market_program) = deploy_market_env().await;
    let market_id = market_program.id();

    let vnft_program = deploy_vnft(&env).await;
    let vnft_id = vnft_program.id();

    let vft_program = deploy_vft(&env, b"salt-vft").await;
    let vft_id = vft_program.id();

    let mut market = market_program.nft_marketplace();
    let mut vnft = vnft_program.vnft();
    let mut vft = vft_program.vft();

    market.add_nft_contract(vnft_id).await.unwrap();
    market.add_ft_contract(vft_id).await.unwrap();

    vnft.mint(USERS[0].into(), default_meta()).await.unwrap();
    vnft.approve(market_id, 0.into()).await.unwrap();

    vft.mint(USERS[1].into(), PRICE.into()).await.unwrap();
    vft.approve(market_id, PRICE.into())
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    assert_eq!(vft.balance_of(USERS[1].into()).await.unwrap(), PRICE.into());

    market
        .add_market_data(vnft_id, Some(vft_id), 0.into(), Some(PRICE))
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items.is_empty());

    assert_eq!(vnft.balance_of(USERS[0].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(USERS[1].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(market_id).await.unwrap(), 1.into());

    market
        .buy_item(vnft_id, 0.into())
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    assert_eq!(vft.balance_of(USERS[1].into()).await.unwrap(), 0.into());
    assert_eq!(vft.balance_of(USERS[0].into()).await.unwrap(), PRICE.into());

    let m = market.get_market().await.unwrap();
    assert_eq!(m.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_offer_native_tokens() {
    let (env, market_program) = deploy_market_env().await;
    let market_id = market_program.id();

    let vnft_program = deploy_vnft(&env).await;
    let vnft_id = vnft_program.id();

    let mut market = market_program.nft_marketplace();
    let mut vnft = vnft_program.vnft();

    market.add_nft_contract(vnft_id).await.unwrap();

    vnft.mint(USERS[0].into(), default_meta()).await.unwrap();
    vnft.approve(market_id, 0.into()).await.unwrap();

    market
        .add_market_data(vnft_id, None, 0.into(), None)
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items.is_empty());

    assert_eq!(vnft.balance_of(USERS[0].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(market_id).await.unwrap(), 1.into());

    let old1 = env.system().balance_of(USERS[1]);

    market
        .add_offer(vnft_id, None, 0.into(), PRICE)
        .with_value(PRICE)
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items[0].1.offers.is_empty());

    let old0 = env.system().balance_of(USERS[0]);

    market
        .accept_offer(vnft_id, None, 0.into(), PRICE)
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(m.items[0].1.offers.is_empty());

    let new0 = env.system().balance_of(USERS[0]);
    let new1 = env.system().balance_of(USERS[1]);

    assert!(new0 - old0 > 9_000_000_000_000);
    assert!(old1 - new1 > PRICE);

    let m = market.get_market().await.unwrap();
    assert_eq!(m.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_offer_with_fungible_tokens() {
    let (env, market_program) = deploy_market_env().await;
    let market_id = market_program.id();

    let vnft_program = deploy_vnft(&env).await;
    let vnft_id = vnft_program.id();

    let vft_program = deploy_vft(&env, b"salt-vft-offer").await;
    let vft_id = vft_program.id();

    let mut market = market_program.nft_marketplace();
    let mut vnft = vnft_program.vnft();
    let mut vft = vft_program.vft();

    market.add_nft_contract(vnft_id).await.unwrap();
    market.add_ft_contract(vft_id).await.unwrap();

    vnft.mint(USERS[0].into(), default_meta()).await.unwrap();
    vnft.approve(market_id, 0.into()).await.unwrap();

    vft.mint(USERS[1].into(), PRICE.into()).await.unwrap();
    vft.approve(market_id, PRICE.into())
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    assert_eq!(vft.balance_of(USERS[1].into()).await.unwrap(), PRICE.into());

    market
        .add_market_data(vnft_id, Some(vft_id), 0.into(), None)
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items.is_empty());
    assert_eq!(vnft.balance_of(USERS[0].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(market_id).await.unwrap(), 1.into());

    market
        .add_offer(vnft_id, Some(vft_id), 0.into(), PRICE)
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items[0].1.offers.is_empty());

    assert_eq!(vft.balance_of(USERS[1].into()).await.unwrap(), 0.into());
    assert_eq!(vft.balance_of(market_id).await.unwrap(), PRICE.into());

    market
        .accept_offer(vnft_id, Some(vft_id), 0.into(), PRICE)
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(m.items[0].1.offers.is_empty());

    assert_eq!(vft.balance_of(USERS[0].into()).await.unwrap(), PRICE.into());

    let m = market.get_market().await.unwrap();
    assert_eq!(m.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_auction_with_native_tokens() {
    let (env, market_program) = deploy_market_env().await;
    let market_id = market_program.id();

    let vnft_program = deploy_vnft(&env).await;
    let vnft_id = vnft_program.id();

    let mut market = market_program.nft_marketplace();
    let mut vnft = vnft_program.vnft();

    market.add_nft_contract(vnft_id).await.unwrap();

    vnft.mint(USERS[0].into(), default_meta()).await.unwrap();
    vnft.approve(market_id, 0.into()).await.unwrap();

    market
        .add_market_data(vnft_id, None, 0.into(), None)
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items.is_empty());

    assert_eq!(vnft.balance_of(USERS[0].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(market_id).await.unwrap(), 1.into());

    market
        .create_auction(vnft_id, None, 0.into(), PRICE, AUCTION_DURATION_MS.into())
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(m.items[0].1.auction.is_some());

    market
        .add_bid(vnft_id, 0.into(), BID_15)
        .with_value(BID_15)
        .with_actor_id(USERS[2].into())
        .await
        .unwrap();

    let old_2 = env.system().balance_of(USERS[2]);

    market
        .add_bid(vnft_id, 0.into(), BID_20)
        .with_value(BID_20)
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    let new_2 = env.system().balance_of(USERS[2]);
    assert_eq!(new_2 - old_2, BID_15);

    let blocks = AUCTION_DURATION_MS / BLOCK_TIME_MS;
    env.system()
        .run_to_block(env.system().block_height() + blocks);

    let old_0 = env.system().balance_of(USERS[0]);

    market.settle_auction(vnft_id, 0.into()).await.unwrap();

    let m = market.get_market().await.unwrap();
    assert!(m.items[0].1.auction.is_none());

    let new_0 = env.system().balance_of(USERS[0]);
    assert!(new_0 - old_0 > 19_000_000_000_000);

    let m = market.get_market().await.unwrap();
    assert_eq!(m.items[0].1.owner, USERS[1].into());
}

#[tokio::test]
async fn success_auction_with_fungible_tokens() {
    let (env, market_program) = deploy_market_env().await;
    let market_id = market_program.id();

    let vnft_program = deploy_vnft(&env).await;
    let vnft_id = vnft_program.id();

    let vft_program = deploy_vft(&env, b"salt-vft-auction").await;
    let vft_id = vft_program.id();

    let mut market = market_program.nft_marketplace();
    let mut vnft = vnft_program.vnft();
    let mut vft = vft_program.vft();

    market.add_nft_contract(vnft_id).await.unwrap();
    market.add_ft_contract(vft_id).await.unwrap();

    vnft.mint(USERS[0].into(), default_meta()).await.unwrap();
    vnft.approve(market_id, 0.into()).await.unwrap();

    vft.mint(USERS[1].into(), BID_20.into()).await.unwrap();
    vft.approve(market_id, BID_20.into())
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    vft.mint(USERS[2].into(), BID_15.into()).await.unwrap();
    vft.approve(market_id, BID_15.into())
        .with_actor_id(USERS[2].into())
        .await
        .unwrap();

    assert_eq!(
        vft.balance_of(USERS[1].into()).await.unwrap(),
        BID_20.into()
    );
    assert_eq!(
        vft.balance_of(USERS[2].into()).await.unwrap(),
        BID_15.into()
    );

    market
        .add_market_data(vnft_id, Some(vft_id), 0.into(), None)
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(!m.items.is_empty());

    assert_eq!(vnft.balance_of(USERS[0].into()).await.unwrap(), 0.into());
    assert_eq!(vnft.balance_of(market_id).await.unwrap(), 1.into());

    market
        .create_auction(
            vnft_id,
            Some(vft_id),
            0.into(),
            PRICE,
            AUCTION_DURATION_MS.into(),
        )
        .await
        .unwrap();

    let m = market.get_market().await.unwrap();
    assert!(m.items[0].1.auction.is_some());

    market
        .add_bid(vnft_id, 0.into(), BID_15)
        .with_actor_id(USERS[2].into())
        .await
        .unwrap();

    assert_eq!(vft.balance_of(USERS[2].into()).await.unwrap(), 0.into());

    market
        .add_bid(vnft_id, 0.into(), BID_20)
        .with_actor_id(USERS[1].into())
        .await
        .unwrap();

    assert_eq!(
        vft.balance_of(USERS[2].into()).await.unwrap(),
        BID_15.into()
    );

    let blocks = AUCTION_DURATION_MS / BLOCK_TIME_MS;
    env.system()
        .run_to_block(env.system().block_height() + blocks);

    market.settle_auction(vnft_id, 0.into()).await.unwrap();

    let m = market.get_market().await.unwrap();
    assert!(m.items[0].1.auction.is_none());

    let m = market.get_market().await.unwrap();
    assert_eq!(m.items[0].1.owner, USERS[1].into());

    assert_eq!(
        vft.balance_of(USERS[0].into()).await.unwrap(),
        BID_20.into()
    );
}
