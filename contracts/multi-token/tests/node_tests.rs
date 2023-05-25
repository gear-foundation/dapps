use gclient::{EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};
use multitoken::WASM_BINARY_OPT;
use multitoken_io::InitMTK;

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
#[ignore]
async fn init() -> Result<()> {
    let api = GearApi::dev_from_path(env!("GEAR_NODE_PATH"))
        .await
        .unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitMTK {
        name: String::from("MTK Simple"),
        symbol: String::from("MTK"),
        base_uri: String::from("http://mtk.simple"),
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.to_vec(),
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
