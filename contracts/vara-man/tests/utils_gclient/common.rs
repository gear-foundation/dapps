use super::{common, vara_man};
use blake2_rfc::blake2b;
use fungible_token_io::*;
use gclient::{Error as GclientError, EventProcessor, GearApi, Result};
use gear_core::ids::{MessageId, ProgramId};
use gstd::{prelude::*, ActorId};
use sp_core::H256;
use vara_man_io::*;

pub const HASH_LENGTH: usize = 32;
pub type Hash = [u8; HASH_LENGTH];

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    let vara_man = vara_man::init(api).await?;

    // Fund users
    let destination = get_user_to_actor_id("//Peter")
        .await?
        .encode()
        .as_slice()
        .try_into()
        .expect("Unexpected invalid `ProgramId`.");
    api.transfer_keep_alive(destination, api.total_balance(api.account_id()).await? / 5)
        .await?;
    api.transfer_keep_alive(
        get_user_to_actor_id("//Alex")
            .await?
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        api.total_balance(api.account_id()).await? / 5,
    )
    .await?;

    api.transfer_keep_alive(
        vara_man
            .encode()
            .as_slice()
            .try_into()
            .expect("Unexpected invalid `ProgramId`."),
        api.total_balance(api.account_id()).await? / 5,
    )
    .await?;
    Ok(vara_man)
}

pub async fn init_with_config(api: &GearApi, config: Config) -> gclient::Result<ActorId> {
    let vara_man = vara_man::init_with_config(api, config).await?;

    Ok(vara_man)
}

pub fn get_current_actor_id(api: &GearApi) -> ActorId {
    ActorId::new(*api.account_id().clone().as_ref())
}

pub async fn get_user_to_actor_id(user: impl AsRef<str>) -> gclient::Result<ActorId> {
    let api = GearApi::dev_from_path("../target/tmp/gear")
        .await?
        .with(user)?;
    let actor_id = ActorId::new(*api.account_id().clone().as_ref());

    Ok(actor_id)
}

pub async fn upload_with_code_hash(
    api: &GearApi,
    wasm_path: impl AsRef<str>,
) -> gclient::Result<Hash> {
    let mut code_hash: Hash = Default::default();
    let wasm_code = gclient::code_from_os(wasm_path.as_ref())?;

    code_hash[..].copy_from_slice(blake2b::blake2b(HASH_LENGTH, &[], &wasm_code).as_bytes());

    match api.upload_code(wasm_code).await {
        // Catch re-upload
        Err(GclientError::ProgramAlreadyExists(_)) => {}
        Err(error) => return Err(error),
        _ => {}
    };

    Ok(code_hash)
}

pub async fn init_ft(api: &GearApi) -> Result<(MessageId, ProgramId, H256)> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let ft_init = InitConfig {
        name: String::from("MyToken"),
        symbol: String::from("MTK"),
        decimals: 18,
    }
    .encode();

    let path = "../target/wasm32-unknown-unknown/release/fungible_token.opt.wasm";

    let gas_info = api
        .calculate_upload_gas(None, gclient::code_from_os(path)?, ft_init.clone(), 0, true)
        .await?;

    api.upload_program_bytes(
        gclient::code_from_os(path)?,
        gclient::now_micros().to_le_bytes(),
        ft_init,
        gas_info.burned * 2,
        0,
    )
    .await
}

pub async fn init_mint_transfer_ft(api: &GearApi, to: ActorId) -> gclient::Result<ActorId> {
    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let (message_id, program_ft_id, _hash) = init_ft(api).await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let ft_mint_payload = FTAction::Mint(100_000_000_000_000);

    let gas_info = api
        .calculate_handle_gas(None, program_ft_id, ft_mint_payload.encode(), 0, true)
        .await
        .unwrap();

    let (message_id, _) = api
        .send_message(program_ft_id, ft_mint_payload, gas_info.burned * 2, 0)
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let address = ActorId::new(
        api.account_id()
            .encode()
            .try_into()
            .expect("Unexpected invalid account id length."),
    );

    let ft_transfer_payload = FTAction::Transfer {
        from: address,
        to,
        amount: 100_000_000_000_000,
    };

    let gas_info = api
        .calculate_handle_gas(None, program_ft_id, ft_transfer_payload.encode(), 0, true)
        .await
        .unwrap();

    let (message_id, _) = api
        .send_message(program_ft_id, ft_transfer_payload, gas_info.burned * 2, 0)
        .await?;
    let program_ft_id: common::Hash = program_ft_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    Ok(program_ft_id.into())
}
