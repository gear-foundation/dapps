use dex_client::dex::Dex;
use dex_client::DexCtors;

use dex_client::Dex as ClientDex;
use extended_vft_client::vft::Vft;
use extended_vft_client::ExtendedVftClient;
use extended_vft_client::ExtendedVftClientCtors;

use sails_rs::client::*;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{gtest::System, ActorId, U256};

pub const USER_ID: u64 = 10;
const FEE: u64 = 30_000_000_000;

type VftActor = Actor<extended_vft_client::ExtendedVftClientProgram, GtestEnv>;
type DexActor = Actor<dex_client::DexProgram, GtestEnv>;

struct Ctx {
    vft_a: VftActor,
    vft_b: VftActor,
    dex: DexActor,
}

async fn setup(mint_a: U256, mint_b: U256) -> Ctx {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");
    system.mint_to(USER_ID, DEFAULT_USERS_INITIAL_BALANCE);

    let env = GtestEnv::new(system, USER_ID.into());

    let vft_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/extended_vft.opt.wasm");

    let vft_a: VftActor = env
        .deploy::<extended_vft_client::ExtendedVftClientProgram>(
            vft_code_id,
            b"salt-vft-a".to_vec(),
        )
        .new("TokenA".to_string(), "A".to_string(), 10_u8)
        .await
        .unwrap();

    let vft_b: VftActor = env
        .deploy::<extended_vft_client::ExtendedVftClientProgram>(
            vft_code_id,
            b"salt-vft-b".to_vec(),
        )
        .new("TokenB".to_string(), "B".to_string(), 10_u8)
        .await
        .unwrap();

    // mint от USER_ID (env настроен на USER_ID)
    vft_a.vft().mint(USER_ID.into(), mint_a).await.unwrap();
    vft_b.vft().mint(USER_ID.into(), mint_b).await.unwrap();

    let dex_code_id = env
        .system()
        .submit_code_file("../target/wasm32-gear/release/dex.opt.wasm");

    let dex: DexActor = env
        .deploy::<dex_client::DexProgram>(dex_code_id, b"salt-dex".to_vec())
        .new(vft_a.id(), vft_b.id(), FEE, None)
        .await
        .unwrap();

    Ctx { vft_a, vft_b, dex }
}

async fn approve(vft: &VftActor, spender: ActorId, amount: U256) {
    vft.vft().approve(spender, amount).await.unwrap();
}

#[tokio::test]
async fn test_add_liquidity() {
    let Ctx { vft_a, vft_b, dex } = setup(100_000.into(), 100_000.into()).await;

    let dex_id = dex.id();
    approve(&vft_a, dex_id, 30_000.into()).await;
    approve(&vft_b, dex_id, 30_000.into()).await;

    let mut d = dex.dex();

    d.add_liquidity(10_000.into(), 10_000.into()).await.unwrap();

    let total_liquidity = d.total_liquidity().await.unwrap();
    assert_eq!(total_liquidity, 9_000.into());

    assert_eq!(d.reserve_a().await.unwrap(), 10_000.into());
    assert_eq!(d.reserve_b().await.unwrap(), 10_000.into());

    d.add_liquidity(20_000.into(), 20_000.into()).await.unwrap();

    assert_eq!(d.total_liquidity().await.unwrap(), 27_000.into());
    assert_eq!(d.reserve_a().await.unwrap(), 30_000.into());
    assert_eq!(d.reserve_b().await.unwrap(), 30_000.into());
}

#[tokio::test]
async fn test_swap() {
    let Ctx { vft_a, vft_b, dex } = setup(5_000.into(), 6_000.into()).await;

    let dex_id = dex.id();
    approve(&vft_a, dex_id, 5_000.into()).await;
    approve(&vft_b, dex_id, 6_000.into()).await;

    let mut d = dex.dex();
    let a = vft_a.vft();
    let b = vft_b.vft();

    d.add_liquidity(5_000.into(), 5_000.into()).await.unwrap();

    let user_a0 = a.balance_of(USER_ID.into()).await.unwrap();
    let user_b0 = b.balance_of(USER_ID.into()).await.unwrap();
    assert_eq!(user_a0, 0.into());
    assert_eq!(user_b0, 1_000.into());

    let ra0 = d.reserve_a().await.unwrap();
    let rb0 = d.reserve_b().await.unwrap();
    let k0 = ra0 * rb0;

    d.swap(1_000.into(), true).await.unwrap();

    let ra1 = d.reserve_a().await.unwrap();
    let rb1 = d.reserve_b().await.unwrap();
    let k1 = ra1 * rb1;

    assert!(ra1 < ra0);
    assert!(rb1 >= rb0);
    assert!(k1 >= k0);

    let user_a1 = a.balance_of(USER_ID.into()).await.unwrap();
    let user_b1 = b.balance_of(USER_ID.into()).await.unwrap();

    assert_eq!(user_b1, 0.into());

    let out_user = user_a1 - user_a0;
    let out_pool = ra0 - ra1;
    assert!(out_user > 0.into());
    assert_eq!(out_user, out_pool);
}

#[tokio::test]
async fn test_remove_liquidity() {
    let Ctx { vft_a, vft_b, dex } = setup(5_000.into(), 5_000.into()).await;

    let dex_id = dex.id();
    approve(&vft_a, dex_id, 5_000.into()).await;
    approve(&vft_b, dex_id, 5_000.into()).await;

    let mut d = dex.dex();
    let a = vft_a.vft();
    let b = vft_b.vft();

    d.add_liquidity(5_000.into(), 5_000.into()).await.unwrap();

    assert_eq!(a.balance_of(USER_ID.into()).await.unwrap(), 0.into());
    assert_eq!(b.balance_of(USER_ID.into()).await.unwrap(), 0.into());

    let ra0 = d.reserve_a().await.unwrap();
    let rb0 = d.reserve_b().await.unwrap();

    d.remove_liquidity(100.into()).await.unwrap();

    let ra1 = d.reserve_a().await.unwrap();
    let rb1 = d.reserve_b().await.unwrap();

    assert!(ra1 < ra0);
    assert!(rb1 < rb0);

    assert!(a.balance_of(USER_ID.into()).await.unwrap() > 0.into());
    assert!(b.balance_of(USER_ID.into()).await.unwrap() > 0.into());
}
