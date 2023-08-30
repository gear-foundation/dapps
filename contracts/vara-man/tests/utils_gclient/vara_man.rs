use super::common;
use gclient::{EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};
use vara_man_io::*;

const VARA_MAN_WASM_PATH: &str = "../target/wasm32-unknown-unknown/debug/vara_man.opt.wasm";

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    init_with_config(
        api,
        Config {
            operator: common::get_current_actor_id(api),
            tokens_per_gold_coin: 1,
            tokens_per_silver_coin: 1,
            easy_reward_scale_bps: 0,
            medium_reward_scale_bps: 0,
            hard_reward_scale_bps: 0,
            gold_coins: 5,
            silver_coins: 20,
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
            gas_info.min_limit * 2,
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

pub async fn register_player(
    api: &GearApi,
    program_id: &ActorId,
    name: &str,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        VaraManAction::RegisterPlayer {
            name: name.to_owned(),
        },
        0,
    )
    .await?;

    let event: VaraManEvent =
        VaraManEvent::decode(&mut result.as_ref()).expect("Unexpected invalid result payload.");

    assert_eq!(matches!(event, VaraManEvent::Error(_)), error);

    Ok(())
}

pub async fn start_game(
    api: &GearApi,
    program_id: &ActorId,
    level: Level,
    seed: u64,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(api, program_id, VaraManAction::StartGame { level, seed }, 0).await?;

    let event: VaraManEvent =
        VaraManEvent::decode(&mut result.as_ref()).expect("Unexpected invalid result payload.");

    assert_eq!(matches!(event, VaraManEvent::Error(_)), error);

    Ok(())
}

pub async fn claim_reward(
    api: &GearApi,
    program_id: &ActorId,
    game_id: u64,
    silver_coins: u64,
    gold_coins: u64,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        VaraManAction::ClaimReward {
            silver_coins,
            gold_coins,
        },
        0,
    )
    .await?;

    let event: VaraManEvent =
        VaraManEvent::decode(&mut result.as_ref()).expect("Unexpected invalid result payload.");

    assert_eq!(matches!(event, VaraManEvent::Error(_)), error);

    Ok(())
}

pub async fn change_status(
    api: &GearApi,
    program_id: &ActorId,
    status: Status,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(api, program_id, VaraManAction::ChangeStatus(status), 0).await?;

    let event: VaraManEvent =
        VaraManEvent::decode(&mut result.as_ref()).expect("Unexpected invalid result payload.");

    assert_eq!(matches!(event, VaraManEvent::Error(_)), error);

    Ok(())
}

pub async fn change_config(
    api: &GearApi,
    program_id: &ActorId,
    config: Config,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(api, program_id, VaraManAction::ChangeConfig(config), 0).await?;

    let event: VaraManEvent =
        VaraManEvent::decode(&mut result.as_ref()).expect("Unexpected invalid result payload.");

    assert_eq!(matches!(event, VaraManEvent::Error(_)), error);

    Ok(())
}

pub async fn get_state(api: &GearApi, program_id: &ActorId) -> gclient::Result<VaraMan> {
    let program_id = program_id.encode().as_slice().into();
    api.read_state(program_id, vec![]).await
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
        .send_message(
            program_id.into(),
            payload,
            gas_info.min_limit * 2,
            value,
            false,
        )
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    Ok(reply_data_result.expect("Unexpected invalid reply."))
}
