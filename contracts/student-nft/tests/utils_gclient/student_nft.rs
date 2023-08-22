use super::common;
use gclient::{EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};
use student_nft_io::{StudentNftAction, StudentNftEvent, StudentNftInit, StudentNftState};

const STUDENT_NFT_WASM_PATH: &str = "./target/wasm32-unknown-unknown/debug/student_nft.opt.wasm";

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let student_nft_init = StudentNftInit {}.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(STUDENT_NFT_WASM_PATH)?,
            student_nft_init.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(STUDENT_NFT_WASM_PATH)?,
            gclient::now_micros().to_le_bytes(),
            student_nft_init,
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

pub async fn mint(api: &GearApi, program_id: &ActorId, error: bool) -> gclient::Result<()> {
    let result = send_message(api, program_id, StudentNftAction::Mint, 0).await?;
    assert_eq!(matches!(result, StudentNftEvent::Error(_)), error);

    Ok(())
}

pub async fn get_state(api: &GearApi, program_id: &ActorId) -> gclient::Result<StudentNftState> {
    let program_id = program_id.encode().as_slice().into();
    api.read_state(program_id).await
}

async fn send_message(
    api: &GearApi,
    program_id: &ActorId,
    payload: StudentNftAction,
    value: u128,
) -> gclient::Result<StudentNftEvent> {
    let mut listener = api.subscribe().await?;

    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), value, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.min_limit * 2, value)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    let reply_data = reply_data_result.expect("Unexpected invalid reply data result.");

    Ok(StudentNftEvent::decode(&mut reply_data.as_ref())?)
}
