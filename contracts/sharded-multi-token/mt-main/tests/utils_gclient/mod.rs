#![allow(unused)]

use blake2_rfc::blake2b;
use gclient::{EventListener, EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};
use mt_main_io::{InitMToken, LogicAction, MTokenAction, MTokenEvent, TokenId};

const MT_LOGIC_WASM_PATH: &str = "../target/wasm32-unknown-unknown/debug/mt_logic.opt.wasm";
const MT_STORAGE_WASM_PATH: &str = "../target/wasm32-unknown-unknown/debug/mt_storage.opt.wasm";
const MT_MAIN_WASM_PATH: &str = "../target/wasm32-unknown-unknown/debug/mt_main.opt.wasm";
const HASH_LENGTH: usize = 32;
type Hash = [u8; HASH_LENGTH];
pub const USER_ACCOUNTS: [&str; 3] = ["//Bob", "//Alice", "//Amy"];

pub async fn setup_gclient() -> gclient::Result<(GearApi, ActorId)> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let user_account_0 = {
        let api = api.clone().with(USER_ACCOUNTS[0])?;
        ActorId::new(api.account_id().clone().into())
    };
    let user_account_2 = {
        let api = api.clone().with(USER_ACCOUNTS[2])?;
        ActorId::new(api.account_id().clone().into())
    };

    let user_fund = api.total_balance(api.account_id()).await? / 3;

    api.transfer(user_account_0.as_ref().into(), user_fund)
        .await?;
    api.transfer(user_account_2.as_ref().into(), user_fund)
        .await?;

    let storage_code_hash = upload_with_code_hash(&api, MT_STORAGE_WASM_PATH).await?;
    let mt_logic_code_hash = upload_with_code_hash(&api, MT_LOGIC_WASM_PATH).await?;

    let init_mtoken_config = InitMToken {
        storage_code_hash: storage_code_hash.into(),
        mt_logic_code_hash: mt_logic_code_hash.into(),
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(MT_MAIN_WASM_PATH)?,
            init_mtoken_config.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            gclient::code_from_os(MT_MAIN_WASM_PATH)?,
            gclient::now_micros().to_le_bytes(),
            init_mtoken_config,
            gas_info.min_limit * 5,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let program_id: Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    Ok((api, program_id.into()))
}

pub fn gclient_with_account(
    api: GearApi,
    account: impl AsRef<str>,
) -> gclient::Result<(GearApi, ActorId)> {
    let api = api.with(account.as_ref())?;
    let actor_id = ActorId::new(api.account_id().clone().into());
    Ok((api, actor_id))
}

pub fn get_actor_id(api: &GearApi, account: impl AsRef<str>) -> gclient::Result<ActorId> {
    let temp_api = api.clone().with(account.as_ref())?;
    let actor_id = ActorId::new(temp_api.account_id().clone().into());

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
        Err(gclient::Error::Subxt(subxt::Error::Runtime(subxt::error::DispatchError::Module(
            subxt::error::ModuleError {
                error_data:
                    subxt::error::ModuleErrorData {
                        pallet_index: 104,
                        error: [6, 0, 0, 0],
                    },
                ..
            },
        )))) => {}
        Err(error) => return Err(error),
        _ => {}
    };

    Ok(code_hash)
}

pub async fn send_mtoken_message(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    mtoken_action: LogicAction,
) -> gclient::Result<()> {
    let payload = MTokenAction::Message {
        transaction_id: tx_id,
        payload: mtoken_action,
    };

    let program_id: Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.min_limit * 5, 0)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    let reply = reply_data_result.expect("Unexpected invalid reply.");

    let mt_main_io::MTokenEvent::Ok = mt_main_io::MTokenEvent::decode(&mut reply.as_ref()).expect("Unexpected invalid `MTokenEvent` data.") else {
        panic!("Unexpected invalid `MTokenEvent`.");
    };

    Ok(())
}

pub async fn mtoken_create(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    initial_amount: u128,
    uri: impl AsRef<str>,
    is_nft: bool,
) -> gclient::Result<()> {
    send_mtoken_message(
        api,
        listener,
        program_id,
        tx_id,
        LogicAction::Create {
            initial_amount,
            uri: uri.as_ref().to_owned(),
            is_nft,
        },
    )
    .await
}

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
pub async fn mtoken_transfer(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    token_id: TokenId,
    from: ActorId,
    to: ActorId,
    amount: u128,
) -> gclient::Result<()> {
    send_mtoken_message(
        api,
        listener,
        program_id,
        tx_id,
        LogicAction::Transfer {
            token_id,
            sender: from,
            recipient: to,
            amount,
        },
    )
    .await
}

#[allow(unused)]
pub async fn mtoken_approve(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    account: ActorId,
    is_approved: bool,
) -> gclient::Result<()> {
    send_mtoken_message(
        api,
        listener,
        program_id,
        tx_id,
        LogicAction::Approve {
            account,
            is_approved,
        },
    )
    .await
}

#[allow(unused)]
pub async fn mtoken_mint_batch_ft(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    token_id: TokenId,
    to: Vec<ActorId>,
    amounts: Vec<u128>,
) -> gclient::Result<()> {
    send_mtoken_message(
        api,
        listener,
        program_id,
        tx_id,
        LogicAction::MintBatchFT {
            token_id,
            to,
            amounts,
        },
    )
    .await
}

#[allow(unused)]
pub async fn mtoken_mint_batch_nft(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    token_id: TokenId,
    to: Vec<ActorId>,
) -> gclient::Result<()> {
    send_mtoken_message(
        api,
        listener,
        program_id,
        tx_id,
        LogicAction::MintBatchNFT { token_id, to },
    )
    .await
}

#[allow(unused)]
pub async fn mtoken_burn_batch_ft(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    token_id: TokenId,
    burn_from: Vec<ActorId>,
    amounts: Vec<u128>,
) -> gclient::Result<()> {
    send_mtoken_message(
        api,
        listener,
        program_id,
        tx_id,
        LogicAction::BurnBatchFT {
            token_id,
            burn_from,
            amounts,
        },
    )
    .await
}

#[allow(unused)]
pub async fn mtoken_burn_nft(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    tx_id: u64,
    token_id: TokenId,
    from: ActorId,
) -> gclient::Result<()> {
    send_mtoken_message(
        api,
        listener,
        program_id,
        tx_id,
        LogicAction::BurnNFT { token_id, from },
    )
    .await
}

#[allow(unused)]
pub async fn mtoken_get_balance(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    token_id: TokenId,
    account: ActorId,
) -> gclient::Result<u128> {
    let payload = MTokenAction::GetBalance { token_id, account };

    let program_id: Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.min_limit * 5, 0)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    let reply_data = reply_data_result.expect("Unexpected invalid reply.");

    let MTokenEvent::Balance(balance) = MTokenEvent::decode(&mut reply_data.as_ref()).expect("Unexpected invalid `MTokenEvent` data.") else {
        panic!("Unexpected invalid `MTokenEvent`.");
    };

    Ok(balance)
}

#[allow(unused)]
pub async fn mtoken_get_approval(
    api: &GearApi,
    listener: &mut EventListener,
    program_id: &ActorId,
    account: ActorId,
    approval_target: ActorId,
) -> gclient::Result<bool> {
    let payload = MTokenAction::GetApproval {
        account,
        approval_target,
    };

    let program_id: Hash = program_id
        .encode()
        .try_into()
        .expect("Unexpected invalid program id.");

    let gas_info = api
        .calculate_handle_gas(None, program_id.into(), payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id.into(), payload, gas_info.min_limit * 5, 0)
        .await?;

    let (_, reply_data_result, _) = listener.reply_bytes_on(message_id).await?;
    let reply_data = reply_data_result.expect("Unexpected invalid reply.");

    let MTokenEvent::Approval(approval) = MTokenEvent::decode(&mut reply_data.as_ref()).expect("Unexpected invalid `MTokenEvent` data.") else {
        panic!("Unexpected invalid `MTokenEvent`.");
    };

    Ok(approval)
}
