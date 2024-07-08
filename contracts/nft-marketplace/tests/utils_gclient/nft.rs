use crate::utils_gclient::common::get_user_to_actor_id;

use super::common;
use gclient::{EventListener, EventProcessor, GearApi};
use gear_lib_old::non_fungible_token::token::TokenMetadata;
use gstd::{prelude::*, ActorId};
use nft_marketplace_io::*;
use non_fungible_token_io::{Collection, Config, InitNFT, NFTAction, NFTEvent};

const NFT_WASM_PATH: &str = "../target/wasm32-unknown-unknown/release/non_fungible_token.opt.wasm";

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_nft_config = InitNFT {
        royalties: Default::default(),
        collection: Collection::default(),
        config: Config {
            authorized_minters: vec![get_user_to_actor_id(common::USERS[4]).await?],
            ..Default::default()
        },
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(NFT_WASM_PATH)?,
            init_nft_config.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(NFT_WASM_PATH)?,
            gclient::now_micros().to_le_bytes(),
            init_nft_config,
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

pub async fn mint(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
) -> gclient::Result<()> {
    let reply = send_message(
        api,
        listener,
        program_id,
        NFTAction::Mint {
            transaction_id: tx_id,
            token_metadata: TokenMetadata {
                name: "a".to_owned(),
                description: "b".to_owned(),
                media: "c".to_owned(),
                reference: "d".to_owned(),
            },
        },
    )
    .await?;

    let NFTEvent::Transfer(_) =
        NFTEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `NFTEvent` data.")
    else {
        std::panic!("Unexpected invalid `NFTEvent`.");
    };

    Ok(())
}

pub async fn approve(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    to: &ActorId,
    token_id: TokenId,
) -> gclient::Result<()> {
    let reply = send_message(
        api,
        listener,
        program_id,
        NFTAction::Approve {
            transaction_id: tx_id,
            to: *to,
            token_id,
        },
    )
    .await?;

    let NFTEvent::Approval(_) =
        NFTEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `NFTEvent` data.")
    else {
        std::panic!("Unexpected invalid `NFTEvent`.");
    };

    Ok(())
}

pub async fn add_minter(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: ActorId,
    tx_id: u64,
    to: ActorId,
) -> gclient::Result<()> {
    let reply = send_message(
        api,
        listener,
        &program_id,
        NFTAction::AddMinter {
            transaction_id: tx_id,
            minter_id: to,
        },
    )
    .await?;

    let NFTEvent::MinterAdded { .. } =
        NFTEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `NFTEvent` data.")
    else {
        std::panic!("Unexpected invalid `NFTEvent`.");
    };

    Ok(())
}

async fn send_message(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    payload: NFTAction,
) -> gclient::Result<Vec<u8>> {
    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.burned * 2, 0)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    Ok(reply_data_result.expect("Unexpected invalid reply."))
}
