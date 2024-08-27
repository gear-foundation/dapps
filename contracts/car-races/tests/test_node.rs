use car_races_io::*;
use gclient::{EventListener, EventProcessor, GearApi, Result};
use gstd::{ActorId, Encode};

async fn upload_program(
    client: &GearApi,
    listener: &mut EventListener,
    path: &str,
    payload: impl Encode,
) -> Result<[u8; 32]> {
    let (message_id, program_id) =
        common_upload_program(client, gclient::code_from_os(path)?, payload).await?;

    assert!(listener
        .message_processed(message_id.into())
        .await?
        .succeed());

    Ok(program_id)
}
async fn common_upload_program(
    client: &GearApi,
    code: Vec<u8>,
    payload: impl Encode,
) -> Result<([u8; 32], [u8; 32])> {
    let encoded_payload = payload.encode();
    let gas_limit = client
        .calculate_upload_gas(None, code.clone(), encoded_payload, 0, true)
        .await?
        .min_limit;
    let (message_id, program_id, _) = client
        .upload_program(
            code,
            gclient::now_micros().to_le_bytes(),
            payload,
            gas_limit,
            0,
        )
        .await?;

    Ok((message_id.into(), program_id.into()))
}

#[tokio::test]
async fn gclient_start_game_test() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await?;

    let mut listener = api.subscribe().await?; // Subscribing for events.
                                               // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init_game = GameInit {
        config: Config {
            gas_to_remove_game: 20_000_000_000,
            initial_speed: 100,
            min_speed: 10,
            max_speed: 2_000,
            gas_for_round: 100_000_000_000,
            time_interval: 20,
            max_distance: 3_242,
            time: 1,
            time_for_game_storage: 200,
        },
    }
    .encode();

    let path = "../target/wasm32-unknown-unknown/release/car_races.opt.wasm";

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(path)?,
            init_game.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(path)?,
            gclient::now_micros().to_le_bytes(),
            init_game,
            gas_info.min_limit,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let allow_messages_payload = GameAction::AllowMessages(true);

    let gas_info = api
        .calculate_handle_gas(None, program_id, allow_messages_payload.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(program_id, allow_messages_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let car_1_id = upload_program(
        &api,
        &mut listener,
        "../target/wasm32-unknown-unknown/release/car_3.opt.wasm",
        0,
    )
    .await?;

    let car_2_id = upload_program(
        &api,
        &mut listener,
        "../target/wasm32-unknown-unknown/release/car_3.opt.wasm",
        0,
    )
    .await?;

    let car_ids: Vec<ActorId> = vec![car_1_id.into(), car_2_id.into()];
    let add_strategy_payload = GameAction::AddStrategyIds { car_ids };

    let gas_info = api
        .calculate_handle_gas(None, program_id, add_strategy_payload.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(program_id, add_strategy_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let start_game_payload = GameAction::StartGame;

    let gas_info = api
        .calculate_handle_gas(None, program_id, start_game_payload.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(program_id, start_game_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    let move_payload = GameAction::PlayerMove {
        strategy_action: StrategyAction::BuyShell,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_id, move_payload.encode(), 0, true)
        .await?;
    let (message_id, _) = api
        .send_message(program_id, move_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());
    assert!(listener.blocks_running().await?);

    Ok(())
}
