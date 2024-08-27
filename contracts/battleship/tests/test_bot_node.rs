use battleship_bot::BotBattleshipAction;
use battleship_io::Entity;
use gclient::{EventProcessor, GearApi, Result};
use gstd::Encode;

#[tokio::test]
async fn gclient_start_game_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.
                                               // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let path = "../target/wasm32-unknown-unknown/release/battleship_bot.opt.wasm";

    let gas_info = api
        .calculate_upload_gas(None, gclient::code_from_os(path)?, vec![0], 0, true)
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path)?,
            gclient::now_micros().to_le_bytes(),
            vec![0],
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let start_payload = BotBattleshipAction::Start;

    let gas_info = api
        .calculate_handle_gas(None, program_id, start_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, start_payload, gas_info.burned * 2, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    assert!(listener.blocks_running().await?);

    Ok(())
}

#[tokio::test]
async fn gclient_turn_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.
                                               // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let path = "../target/wasm32-unknown-unknown/release/battleship_bot.opt.wasm";

    let gas_info = api
        .calculate_upload_gas(None, gclient::code_from_os(path)?, vec![], 0, true)
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path)?,
            gclient::now_micros().to_le_bytes(),
            vec![],
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let mut board = vec![Entity::Unknown; 25];
    board[12] = Entity::BoomShip;
    let start_payload = BotBattleshipAction::Turn(board);

    let gas_info = api
        .calculate_handle_gas(None, program_id, start_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, start_payload, gas_info.burned * 2, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    Ok(())
}
