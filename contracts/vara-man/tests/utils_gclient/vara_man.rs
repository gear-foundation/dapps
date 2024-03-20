use super::common;
use gclient::{EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};
use vara_man_io::*;

const VARA_MAN_WASM_PATH: &str = "../target/wasm32-unknown-unknown/debug/vara_man.opt.wasm";

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    init_with_config(
        api,
        Config {
            one_point_in_value: 10_000_000_000_000,
            points_per_gold_coin_easy: 5,
            points_per_silver_coin_easy: 1,
            points_per_gold_coin_medium: 8,
            points_per_silver_coin_medium: 2,
            points_per_gold_coin_hard: 10,
            points_per_silver_coin_hard: 3,
            gas_for_finish_single_game: 10_000_000_000,
            gas_for_finish_tournament: 10_000_000_000,
            time_for_single_round: 15_000,
        },
    )
    .await
}

pub async fn init_with_config(api: &GearApi, config: Config) -> gclient::Result<ActorId> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let vara_man_init = VaraManInit { config }.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(VARA_MAN_WASM_PATH)?,
            vara_man_init.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(VARA_MAN_WASM_PATH)?,
            gclient::now_micros().to_le_bytes(),
            vara_man_init,
            gas_info.burned * 2,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    Ok(program_id.into())
}

pub async fn create_tournament(
    api: &GearApi,
    program_id: &ActorId,
    tournament_name: String,
    name: String,
    level: Level,
    duration_ms: u32,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        VaraManAction::CreateNewTournament {
            tournament_name,
            name,
            level,
            duration_ms,
        },
        10_000_000_000_000,
    )
    .await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}
pub async fn register_for_tournament(
    api: &GearApi,
    program_id: &ActorId,
    admin_id: ActorId,
    name: String,
    value: u128,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        VaraManAction::RegisterForTournament { admin_id, name },
        value,
    )
    .await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}
pub async fn start_tournament(
    api: &GearApi,
    program_id: &ActorId,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(api, program_id, VaraManAction::StartTournament, 0).await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}
pub async fn record_tournament_result(
    api: &GearApi,
    program_id: &ActorId,
    time: u128,
    gold_coins: u128,
    silver_coins: u128,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        VaraManAction::RecordTournamentResult {
            time,
            gold_coins,
            silver_coins,
        },
        0,
    )
    .await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    println!("EVENT: {:?}", event);

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}
pub async fn finish_single_game(
    api: &GearApi,
    program_id: &ActorId,
    gold_coins: u128,
    silver_coins: u128,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        VaraManAction::FinishSingleGame {
            gold_coins,
            silver_coins,
            level: Level::Easy,
        },
        0,
    )
    .await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}
pub async fn leave_game(
    api: &GearApi,
    program_id: &ActorId,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(api, program_id, VaraManAction::LeaveGame, 0).await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}
pub async fn change_status(
    api: &GearApi,
    program_id: &ActorId,
    status: Status,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(api, program_id, VaraManAction::ChangeStatus(status), 0).await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}

pub async fn change_config(
    api: &GearApi,
    program_id: &ActorId,
    config: Config,
    error: Option<VaraManError>,
) -> gclient::Result<()> {
    let result = send_message(api, program_id, VaraManAction::ChangeConfig(config), 0).await?;

    let event: Result<VaraManEvent, VaraManError> =
        Result::<VaraManEvent, VaraManError>::decode(&mut result.as_ref())
            .expect("Unexpected invalid result payload.");

    if let Some(error) = error {
        assert_eq!(event.unwrap_err(), error);
    }

    Ok(())
}

pub async fn get_state(api: &GearApi, program_id: &ActorId) -> Option<VaraManState> {
    let program_id = program_id.encode().as_slice().into();
    let reply = api
        .read_state(program_id, StateQuery::All.encode())
        .await
        .expect("Unexpected invalid reply.");
    if let StateReply::All(state) = reply {
        Some(state)
    } else {
        None
    }
}

async fn send_message(
    api: &GearApi,
    program_id: &ActorId,
    payload: VaraManAction,
    value: u128,
) -> gclient::Result<Vec<u8>> {
    let mut listener = api.subscribe().await?;

    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), value, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.min_limit, value)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    Ok(reply_data_result.expect("Unexpected invalid reply."))
}
