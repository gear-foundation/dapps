use gclient::{EventProcessor, GearApi, Result};
use gstd::Encode;
use rock_paper_scissors::WASM_BINARY_OPT;
use rps_io::*;

mod routines;
pub use routines::*;

#[tokio::test]
#[ignore]
async fn init() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let entry_timeout_ms = COMMON_TIMEOUT;
    let move_timeout_ms = COMMON_TIMEOUT + 1;
    let reveal_timeout_ms = COMMON_TIMEOUT + 2;

    let init = GameConfig {
        bet_size: COMMON_BET,
        players_count_limit: COMMON_PLAYERS_COUNT_LIMIT,
        entry_timeout_ms,
        move_timeout_ms,
        reveal_timeout_ms,
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
            gclient::now_in_micros().to_le_bytes(),
            init_payload,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}
