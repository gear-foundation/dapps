use super::common;
use gclient::{EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};
use nft_master_io::*;

const NFT_MASTER_WASM_PATH: &str = "../target/wasm32-unknown-unknown/debug/nft_master.opt.wasm";

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let nft_master_init = NFTMasterInit {}.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(NFT_MASTER_WASM_PATH)?,
            nft_master_init.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(NFT_MASTER_WASM_PATH)?,
            gclient::now_micros().to_le_bytes(),
            nft_master_init,
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

pub async fn add_nft_contract(
    api: &GearApi,
    program_id: &ActorId,
    nft_contract: &ActorId,
    meta: &str,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        NFTMasterAction::AddNFTContract {
            nft_contract: *nft_contract,
            meta: meta.to_owned(),
        },
        0,
    )
    .await?;
    assert_eq!(matches!(result, NFTMasterEvent::Error(_)), error);

    Ok(())
}

pub async fn remove_nft_contract(
    api: &GearApi,
    program_id: &ActorId,
    nft_contract: &ActorId,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        NFTMasterAction::RemoveNFTContract {
            nft_contract: *nft_contract,
        },
        0,
    )
    .await?;
    assert_eq!(matches!(result, NFTMasterEvent::Error(_)), error);

    Ok(())
}

pub async fn add_operator(
    api: &GearApi,
    program_id: &ActorId,
    operator: &ActorId,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        NFTMasterAction::AddOperator {
            operator: *operator,
        },
        0,
    )
    .await?;
    assert_eq!(matches!(result, NFTMasterEvent::Error(_)), error);

    Ok(())
}

pub async fn remove_operator(
    api: &GearApi,
    program_id: &ActorId,
    operator: &ActorId,
    error: bool,
) -> gclient::Result<()> {
    let result = send_message(
        api,
        program_id,
        NFTMasterAction::RemoveOperator {
            operator: *operator,
        },
        0,
    )
    .await?;
    assert_eq!(matches!(result, NFTMasterEvent::Error(_)), error);

    Ok(())
}

pub async fn get_state(api: &GearApi, program_id: &ActorId) -> gclient::Result<NFTMasterState> {
    let program_id = program_id.encode().as_slice().into();
    api.read_state(program_id, vec![]).await
}

async fn send_message(
    api: &GearApi,
    program_id: &ActorId,
    payload: NFTMasterAction,
    value: u128,
) -> gclient::Result<NFTMasterEvent> {
    let mut listener = api.subscribe().await?;

    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), value, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.burned * 2, value)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    let reply_data = reply_data_result.expect("Unexpected invalid reply data result.");

    Ok(NFTMasterEvent::decode(&mut reply_data.as_ref())?)
}
