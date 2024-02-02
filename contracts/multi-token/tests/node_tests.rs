use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};
use multi_token_io::*;

pub const TOKEN_ADDRESS: u64 = 1;
pub const ICO_CONTRACT_ID: u64 = 2;
pub const OWNER_ID: u64 = 100001;
pub const USER_ID: u64 = 12345;

pub const ZERO_ID: ActorId = ActorId::zero();

pub const TOKENS_CNT: u128 = 100;
pub const START_PRICE: u128 = 1000;
pub const PRICE_INCREASE_STEP: u128 = 100;
pub const TIME_INCREASE_STEP: u128 = 1000;

#[tokio::test]
async fn gclient_init() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await.unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitMtk {
        name: String::from("Mtk Simple"),
        symbol: String::from("Mtk"),
        base_uri: String::from("http://mtk.simple"),
    };

    let init_payload = init.encode();
    let path = "../target/wasm32-unknown-unknown/debug/multi_token.opt.wasm";

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(path)?,
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path)?,
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
