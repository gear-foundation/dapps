use concert::WASM_BINARY_OPT;
use concert_io::InitConcert;
use gclient::{code_from_os, EventProcessor, GearApi, Result};
use gear_lib::multitoken::io::InitConfig;
use gstd::Encode;

pub const USER: u64 = 193;
pub const MTK_ID: u64 = 2;

#[tokio::test]
#[ignore]
async fn init() -> Result<()> {
    let api = GearApi::dev().await?;
    let multitoken_program = code_from_os("target/multi_token.wasm")?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init Multitoken
    let init_multitoken = InitConfig {
        name: String::from("Multitoken for a concert"),
        symbol: String::from("MTC"),
        base_uri: String::from(""),
    };

    let init_multitoken_payload = init_multitoken.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            multitoken_program.clone(),
            init_multitoken_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes(
            multitoken_program,
            gclient::now_in_micros().to_le_bytes(),
            init_multitoken_payload,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    // Init Concert
    let init_concert = InitConcert {
        owner_id: USER.into(),
        mtk_contract: MTK_ID.into(),
    };

    let init_concert_payload = init_concert.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.to_vec(),
            init_concert_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_in_micros().to_le_bytes(),
            init_concert_payload,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
