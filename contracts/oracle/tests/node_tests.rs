use gclient::{EventProcessor, GearApi};
use gstd::{prelude::*, ActorId};
use oracle::WASM_BINARY_OPT;
use oracle_io::{Action, InitConfig, Oracle};
use utils::{
    FAKE_OWNER_GCLIENT, MANAGER_GCLIENT, NEW_MANAGER_GCLIENT, OWNER_GCLIENT, RANDOM_VALUE,
};

pub mod utils;

#[tokio::test]
pub async fn gclient_success_init() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear")
        .await?
        .with(OWNER_GCLIENT)?;
    let owner_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(MANAGER_GCLIENT)?;
    let manager_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(OWNER_GCLIENT)?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_oracle_config = InitConfig {
        owner: owner_id,
        manager: manager_id,
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.to_vec(),
            init_oracle_config.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_oracle_config,
            gas_info.min_limit,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let oracle_state: Oracle = api
        .read_state(program_id, vec![])
        .await
        .expect("Invalid state.");

    assert_eq!(oracle_state.manager, manager_id);
    assert_eq!(oracle_state.owner, owner_id);

    Ok(())
}

#[tokio::test]
pub async fn gclient_success_change_manager() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear")
        .await?
        .with(OWNER_GCLIENT)?;
    let owner_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(MANAGER_GCLIENT)?;
    let manager_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(NEW_MANAGER_GCLIENT)?;
    let new_manager_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(OWNER_GCLIENT)?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_oracle_config = InitConfig {
        owner: owner_id,
        manager: manager_id,
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.to_vec(),
            init_oracle_config.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_oracle_config,
            gas_info.min_limit,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let change_manager_payload = Action::ChangeManager(new_manager_id);

    let gas_info = api
        .calculate_handle_gas(None, program_id, change_manager_payload.encode(), 0, true)
        .await?;

    let (message_id, _) = api
        .send_message(program_id, change_manager_payload, gas_info.min_limit, 0)
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let oracle_state: Oracle = api
        .read_state(program_id, vec![])
        .await
        .expect("Invalid state.");

    assert_eq!(oracle_state.manager, new_manager_id);
    assert_eq!(oracle_state.owner, owner_id);

    Ok(())
}

#[tokio::test]
pub async fn gclient_success_request_value() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear")
        .await?
        .with(OWNER_GCLIENT)?;
    let owner_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(MANAGER_GCLIENT)?;
    let manager_account_id = api.account_id().clone();
    let manager_program_id = manager_account_id.encode().as_slice().into();
    let manager_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(OWNER_GCLIENT)?;
    api.transfer(
        manager_program_id,
        api.total_balance(api.account_id())
            .await
            .expect("Unexpected invalid total balance.")
            / 3,
    )
    .await?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_oracle_config = InitConfig {
        owner: owner_id,
        manager: manager_id,
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.to_vec(),
            init_oracle_config.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_oracle_config,
            gas_info.min_limit * 2,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let request_value_payload = Action::RequestValue;

    let gas_info = api
        .calculate_handle_gas(None, program_id, request_value_payload.encode(), 0, true)
        .await?;

    let (request_message_id, _) = api
        .send_message(program_id, request_value_payload, gas_info.min_limit * 2, 0)
        .await?;

    let api = api.with(MANAGER_GCLIENT)?;

    let maybe_mailbox = api
        .get_mailbox_account_messages(manager_account_id, 1)
        .await?;
    assert!(!maybe_mailbox.is_empty());

    let payload_reply_id = maybe_mailbox[0].0.id();

    api.send_reply(payload_reply_id, RANDOM_VALUE, gas_info.min_limit * 2, 0)
        .await?;
    assert!(listener
        .message_processed(request_message_id)
        .await?
        .succeed());

    Ok(())
}

#[tokio::test]
pub async fn gclient_fail_change_manager_invalid_owner() -> gclient::Result<()> {
    let api = GearApi::dev_from_path("../target/tmp/gear")
        .await?
        .with(OWNER_GCLIENT)?;
    let owner_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(MANAGER_GCLIENT)?;
    let manager_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(NEW_MANAGER_GCLIENT)?;
    let new_manager_id = ActorId::new((*api.account_id()).clone().into());

    let api = api.with(OWNER_GCLIENT)?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    let init_oracle_config = InitConfig {
        owner: owner_id,
        manager: manager_id,
    }
    .encode();

    let gas_info = api
        .calculate_upload_gas(
            None,
            WASM_BINARY_OPT.to_vec(),
            init_oracle_config.clone(),
            0,
            true,
        )
        .await?;

    let (message_id, program_id, _hash) = api
        .upload_program_bytes(
            WASM_BINARY_OPT.to_vec(),
            gclient::now_micros().to_le_bytes(),
            init_oracle_config,
            gas_info.min_limit,
            0,
        )
        .await?;
    assert!(listener.message_processed(message_id).await?.succeed());

    let change_manager_payload = Action::ChangeManager(new_manager_id);

    let gas_info = api
        .calculate_handle_gas(None, program_id, change_manager_payload.encode(), 0, true)
        .await?;

    let api = api.with(FAKE_OWNER_GCLIENT)?;

    assert!(api
        .send_message(program_id, change_manager_payload, gas_info.min_limit, 0,)
        .await
        .is_err());

    Ok(())
}
