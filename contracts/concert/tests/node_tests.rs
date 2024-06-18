use concert_io::*;
use gclient::{code_from_os, EventProcessor, GearApi, Result};
use gstd::Encode;
use multi_token_io::InitMtk;

pub const USER: u64 = 193;
pub const MTK_ID: u64 = 2;

#[tokio::test]
async fn gclient_init() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;
    let multitoken_program =
        code_from_os("../target/wasm32-unknown-unknown/release/multi_token.opt.wasm")?;
    let mut listener = api.subscribe().await?; // Subscribing for events.

    let path = "../target/wasm32-unknown-unknown/release/concert.opt.wasm";

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    // Init Multitoken
    let init_multitoken = InitMtk {
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
            gclient::now_micros().to_le_bytes(),
            init_multitoken_payload,
            gas_info.burned * 2,
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
            gclient::code_from_os(path)?,
            init_concert_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path)?,
            gclient::now_micros().to_le_bytes(),
            init_concert_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
