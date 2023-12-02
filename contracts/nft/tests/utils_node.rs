use gclient::{GearApi, Result};
use gear_core::ids::{MessageId, ProgramId};
use gstd::{ActorId, Encode};
use nft_io::*;
use sp_core::H256;

pub async fn get_state(api: &GearApi, program_id: &ProgramId) -> Option<State> {
    let reply = api
        .read_state(*program_id, StateQuery::All.encode())
        .await
        .expect("Unexpected invalid reply.");
    if let StateReply::All(state) = reply {
        Some(state)
    } else {
        None
    }
}

pub async fn init(api: &GearApi) -> Result<(MessageId, ProgramId, H256)> {
    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let init_nft = InitNft {
        collection,
        config: Config {
            max_mint_count: Some(100),
        },
    }
    .encode();

    let path = "../target/wasm32-unknown-unknown/debug/nft.opt.wasm";
    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(path)?,
            init_nft.clone(),
            0,
            true,
        )
        .await?;

    api.upload_program_bytes(
        gclient::code_from_os(path)?,
        gclient::now_micros().to_le_bytes(),
        init_nft,
        gas_info.burned * 2,
        0,
    )
    .await
}

pub async fn mint(api: &GearApi, program_id: &ProgramId, to: ActorId) -> Result<(MessageId, H256)> {
    let token_metadata = TokenMetadata {
        name: "CryptoKitty".to_string(),
        description: "Description".to_string(),
        media: "http://".to_string(),
        reference: "http://".to_string(),
    };

    let mint_payload = NftAction::Mint { to, token_metadata };

    let gas_info = api
        .calculate_handle_gas(None, *program_id, mint_payload.encode(), 0, true)
        .await
        .unwrap();

    api.send_message(*program_id, mint_payload, gas_info.burned * 2, 0)
        .await
}

pub async fn burn(
    api: &GearApi,
    program_id: &ProgramId,
    token_id: TokenId,
) -> Result<(MessageId, H256)> {
    let burn_payload = NftAction::Burn { token_id };

    let gas_info = api
        .calculate_handle_gas(None, *program_id, burn_payload.encode(), 0, true)
        .await
        .unwrap();

    api.send_message(*program_id, burn_payload, gas_info.burned * 2, 0)
        .await
}

pub async fn transfer(
    api: &GearApi,
    program_id: &ProgramId,
    to: ActorId,
    token_id: TokenId,
) -> Result<(MessageId, H256)> {
    let transfer_payload = NftAction::Transfer { to, token_id };

    let gas_info = api
        .calculate_handle_gas(None, *program_id, transfer_payload.encode(), 0, true)
        .await?;

    api.send_message(*program_id, transfer_payload, gas_info.burned * 2, 0)
        .await
}

pub async fn approve(
    api: &GearApi,
    program_id: &ProgramId,
    to: ActorId,
    token_id: TokenId,
) -> Result<(MessageId, H256)> {
    let approve_payload = NftAction::Approve { to, token_id };

    let gas_info = api
        .calculate_handle_gas(None, *program_id, approve_payload.encode(), 0, true)
        .await?;

    api.send_message(*program_id, approve_payload, gas_info.burned * 2, 0)
        .await
}
