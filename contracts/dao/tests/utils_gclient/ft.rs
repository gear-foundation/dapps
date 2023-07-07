use super::common;
use ft_main_io::{FTokenAction, FTokenEvent, InitFToken, LogicAction};
use gclient::{EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};

const FT_STORAGE_WASM_PATH: &str = "./target/wasm32-unknown-unknown/debug/ft_storage.opt.wasm";
const FT_LOGIC_WASM_PATH: &str = "./target/wasm32-unknown-unknown/debug/ft_logic.opt.wasm";
const FT_MAIN_WASM_PATH: &str = "./target/wasm32-unknown-unknown/debug/ft_main.opt.wasm";

pub async fn init(api: &GearApi) -> gclient::Result<ActorId> {
    let storage_code_hash = common::upload_with_code_hash(api, FT_STORAGE_WASM_PATH).await?;
    let ft_logic_code_hash = common::upload_with_code_hash(api, FT_LOGIC_WASM_PATH).await?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_ftoken_config = InitFToken {
        storage_code_hash: storage_code_hash.into(),
        ft_logic_code_hash: ft_logic_code_hash.into(),
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(FT_MAIN_WASM_PATH)?,
            init_ftoken_config.clone(),
            0,
            true,
        )
        .await?;
    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(FT_MAIN_WASM_PATH)?,
            gclient::now_micros().to_le_bytes(),
            init_ftoken_config,
            gas_info.burned * 5,
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
    program_id: &ActorId,
    tx_id: u64,
    recipient: &ActorId,
    amount: u128,
) -> gclient::Result<()> {
    let reply = send_message(
        api,
        program_id,
        FTokenAction::Message {
            transaction_id: tx_id,
            payload: LogicAction::Mint {
                recipient: *recipient,
                amount,
            },
        },
    )
    .await?;

    assert_ft_ok(&reply);

    Ok(())
}

#[allow(unused)]
pub async fn balance_of(
    api: &GearApi,
    program_id: &ActorId,
    account: &ActorId,
) -> gclient::Result<u128> {
    let reply = send_message(api, program_id, FTokenAction::GetBalance(*account)).await?;

    let FTokenEvent::Balance(balance) = FTokenEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `FTokenEvent` data.") else {
        panic!("Unexpected invalid `FTokenEvent`.");
    };

    Ok(balance)
}

pub async fn approve(
    api: &GearApi,
    program_id: &ActorId,
    tx_id: u64,
    approved_account: &ActorId,
    amount: u128,
) -> gclient::Result<()> {
    let reply = send_message(
        api,
        program_id,
        FTokenAction::Message {
            transaction_id: tx_id,
            payload: LogicAction::Approve {
                approved_account: *approved_account,
                amount,
            },
        },
    )
    .await?;

    assert_ft_ok(&reply);

    Ok(())
}

async fn send_message(
    api: &GearApi,
    program_id: &ActorId,
    payload: FTokenAction,
) -> gclient::Result<Vec<u8>> {
    let mut listener = api.subscribe().await?;

    let program_id: common::Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.burned * 5, 0)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    Ok(reply_data_result.expect("Unexpected invalid reply."))
}

fn assert_ft_ok(reply: &[u8]) {
    #[allow(clippy::useless_asref)]
    let FTokenEvent::Ok = FTokenEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `FTokenEvent` data.") else {
        panic!("Unexpected invalid `FTokenEvent`.");
    };
}
