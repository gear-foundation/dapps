use super::common;
use gclient::{EventListener, EventProcessor, GearApi};
use gear_lib::non_fungible_token::token::TokenMetadata;
use gstd::{prelude::*, ActorId};
use market_io::TokenId;
use nft_io::{InitNFT, NFTAction, NFTEvent};

const NFT_WASM_PATH: &str = "./target/nft.wasm";

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_nft_config = InitNFT {
        name: Default::default(),
        symbol: Default::default(),
        base_uri: Default::default(),
        royalties: None,
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
            gas_info.min_limit,
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

    let NFTEvent::Transfer(_) = NFTEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `NFTEvent` data.") else {
        panic!("Unexpected invalid `NFTEvent`.");
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

    let NFTEvent::Approval(_) = NFTEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `NFTEvent` data.") else {
        panic!("Unexpected invalid `NFTEvent`.");
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
        .send_message(program_id.into(), payload, gas_info.min_limit, 0)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    Ok(reply_data_result.expect("Unexpected invalid reply."))
}
