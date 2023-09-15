use escrow_io::*;
use gclient::{EventProcessor, GearApi, Result};
use gstd::Encode;

const PATH: &str = "../target/wasm32-unknown-unknown/debug/escrow.opt.wasm";

pub const FT_PROGRAM_ID: u64 = 2;
pub const ESCROW_PROGRAM_ID: u64 = 13370;
pub const FOREIGN_USER: u64 = 1337;
pub const BUYER: [u64; 2] = [12, 34];
pub const SELLER: [u64; 2] = [56, 78];
pub const AMOUNT: [u128; 2] = [12345, 54321];
pub const WALLET: [u128; 2] = [0, 1];
pub const AMOUNT_REMAINDER: u128 = 20000;
pub const NONEXISTENT_WALLET: u128 = 999999;

#[tokio::test]
async fn gclient_init() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await.unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitEscrow {
        ft_program_id: FT_PROGRAM_ID.into(),
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}

#[tokio::test]
async fn gclient_create() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await.unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitEscrow {
        ft_program_id: FT_PROGRAM_ID.into(),
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    let escrow_create = EscrowAction::Create {
        buyer: BUYER[0].into(),
        seller: SELLER[0].into(),
        amount: AMOUNT[0],
    };

    let escrow_create_payload = escrow_create.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            escrow_create_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            escrow_create_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    Ok(())
}

#[tokio::test]
async fn gclient_deposit_not_enough_tokens() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await.unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitEscrow {
        ft_program_id: FT_PROGRAM_ID.into(),
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    let escrow_create = EscrowAction::Create {
        buyer: BUYER[0].into(),
        seller: SELLER[0].into(),
        amount: AMOUNT[0],
    };

    let escrow_create_payload = escrow_create.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            escrow_create_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            escrow_create_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let deposit = EscrowAction::Deposit(WALLET[0].into());

    let deposit_payload = deposit.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            deposit_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            deposit_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn gclient_not_buyer_confirm() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await.unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitEscrow {
        ft_program_id: FT_PROGRAM_ID.into(),
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    let escrow_create = EscrowAction::Create {
        buyer: BUYER[0].into(),
        seller: SELLER[0].into(),
        amount: AMOUNT[0],
    };

    let escrow_create_payload = escrow_create.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            escrow_create_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            escrow_create_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let deposit = EscrowAction::Deposit(WALLET[0].into());

    let deposit_payload = deposit.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            deposit_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            deposit_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    let confirm = EscrowAction::Confirm(WALLET[0].into());

    let confirm_payload = confirm.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            confirm_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            confirm_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn gclient_cancel_paid() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await.unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitEscrow {
        ft_program_id: FT_PROGRAM_ID.into(),
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    let escrow_create = EscrowAction::Create {
        buyer: BUYER[0].into(),
        seller: SELLER[0].into(),
        amount: AMOUNT[0],
    };

    let escrow_create_payload = escrow_create.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            escrow_create_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            escrow_create_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let deposit = EscrowAction::Deposit(WALLET[0].into());

    let deposit_payload = deposit.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            deposit_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            deposit_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    let cancel = EscrowAction::Cancel(WALLET[0].into());

    let cancel_payload = cancel.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            cancel_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            cancel_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn gclient_refund_not_paid() -> Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear").await.unwrap();

    let mut listener = api.subscribe().await?; // Subscribing for events.

    // Checking that blocks still running.
    assert!(listener.blocks_running().await?);

    let init = InitEscrow {
        ft_program_id: FT_PROGRAM_ID.into(),
    };

    let init_payload = init.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            init_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            init_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    let escrow_create = EscrowAction::Create {
        buyer: BUYER[0].into(),
        seller: SELLER[0].into(),
        amount: AMOUNT[0],
    };

    let escrow_create_payload = escrow_create.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            escrow_create_payload.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            escrow_create_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    assert!(listener.message_processed(message_id).await?.succeed());

    let refund = EscrowAction::Refund(WALLET[0].into());

    let refund_payload = refund.encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            gclient::code_from_os(PATH)?,
            refund_payload.clone(),
            0,
            true,
        )
        .await?;

    let (_message_id, _program_id, _hash) = api
        .upload_program_bytes_by_path(
            PATH,
            gclient::now_micros().to_le_bytes(),
            refund_payload,
            gas_info.burned * 2,
            0,
        )
        .await?;

    Ok(())
}
