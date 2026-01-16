use gtest::Log;

use multisig_wallet_client::MultisigWallet as ClientMultisigWallet;
use multisig_wallet_client::MultisigWalletCtors;
use multisig_wallet_client::multisig_wallet::MultisigWallet;

use sails_rs::client::*;
use sails_rs::gtest::System;
use sails_rs::gtest::constants::DEFAULT_USERS_INITIAL_BALANCE;
use sails_rs::{ActorId, Encode};

const USERS: &[u64] = &[3, 4, 5, 6];

#[tokio::test]
async fn check_submit_and_confirm() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    for id in USERS {
        system.mint_to(*id, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, USERS[0].into());

    let program_code_id = env.system().submit_code(multisig_wallet::WASM_BINARY);

    let program = env
        .deploy::<multisig_wallet_client::MultisigWalletProgram>(program_code_id, b"salt".to_vec())
        .new(vec![USERS[0].into(), USERS[1].into(), USERS[2].into()], 2)
        .await
        .unwrap();

    let program_id = program.id();
    let mut service_client = program.multisig_wallet();

    service_client
        .submit_transaction(1.into(), vec![], 0, None)
        .with_value(10_000_000_000_000u128)
        .await
        .unwrap();

    let state = service_client.get_state().await.unwrap();
    assert_eq!(state.transaction_count, 1.into());
    assert!(!state.confirmations.is_empty());
    assert!(!state.transactions[0].1.executed);

    let mail = env.system().get_mailbox(1_u64);
    let log = Log::builder()
        .dest(1_u64)
        .payload_bytes(vec![])
        .source(program_id);
    assert!(!mail.contains(&log));

    service_client
        .confirm_transaction(0.into())
        .with_actor_id(USERS[2].into())
        .await
        .unwrap();

    let mail = env.system().get_mailbox(1_u64);
    let log = Log::builder()
        .dest(1_u64)
        .payload_bytes(vec![])
        .source(program_id);
    assert!(mail.contains(&log));

    let state = service_client.get_state().await.unwrap();
    assert!(state.transactions[0].1.executed);
}

#[tokio::test]
async fn change_required_confirmations_count() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    for id in USERS {
        system.mint_to(*id, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, USERS[0].into());

    let program_code_id = env.system().submit_code(multisig_wallet::WASM_BINARY);

    let program = env
        .deploy::<multisig_wallet_client::MultisigWalletProgram>(program_code_id, b"salt".to_vec())
        .new(vec![USERS[0].into(), USERS[1].into(), USERS[2].into()], 2)
        .await
        .unwrap();

    let program_id = program.id();
    let mut service_client = program.multisig_wallet();

    let payload = [
        "MultisigWallet".encode(),
        "ChangeRequiredConfirmationsCount".to_string().encode(),
        (3u32).encode(),
    ]
    .concat();

    service_client
        .submit_transaction(program_id, payload, 0, None)
        .await
        .unwrap();

    let state = service_client.get_state().await.unwrap();
    assert_eq!(state.required, 2);

    service_client
        .confirm_transaction(0.into())
        .with_actor_id(USERS[2].into())
        .await
        .unwrap();

    let state = service_client.get_state().await.unwrap();
    assert_eq!(state.required, 3);
}

#[tokio::test]
async fn add_owner_remove_owner_and_replace() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    for id in USERS {
        system.mint_to(*id, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, USERS[0].into());

    let program_code_id = env.system().submit_code(multisig_wallet::WASM_BINARY);

    let program = env
        .deploy::<multisig_wallet_client::MultisigWalletProgram>(program_code_id, b"salt".to_vec())
        .new(vec![USERS[0].into(), USERS[1].into()], 1)
        .await
        .unwrap();

    let program_id = program.id();
    let mut service_client = program.multisig_wallet();

    let state = service_client.get_state().await.unwrap();
    assert!(!state.owners.contains(&USERS[2].into()));

    let payload = [
        "MultisigWallet".encode(),
        "AddOwner".to_string().encode(),
        (<u64 as Into<ActorId>>::into(USERS[2])).encode(),
    ]
    .concat();

    service_client
        .submit_transaction(program_id, payload, 0, None)
        .await
        .unwrap();

    let state = service_client.get_state().await.unwrap();
    assert!(state.owners.contains(&USERS[2].into()));
    assert!(state.owners.contains(&USERS[1].into()));

    let payload = [
        "MultisigWallet".encode(),
        "RemoveOwner".to_string().encode(),
        (<u64 as Into<ActorId>>::into(USERS[1])).encode(),
    ]
    .concat();

    service_client
        .submit_transaction(program_id, payload, 0, None)
        .await
        .unwrap();

    let state = service_client.get_state().await.unwrap();
    assert!(!state.owners.contains(&USERS[1].into()));

    let payload = [
        "MultisigWallet".encode(),
        "ReplaceOwner".to_string().encode(),
        (
            <u64 as Into<ActorId>>::into(USERS[2]),
            <u64 as Into<ActorId>>::into(USERS[1]),
        )
            .encode(),
    ]
    .concat();

    service_client
        .submit_transaction(program_id, payload, 0, None)
        .await
        .unwrap();

    let state = service_client.get_state().await.unwrap();
    assert!(state.owners.contains(&USERS[1].into()));
    assert!(!state.owners.contains(&USERS[2].into()));
}

#[tokio::test]
async fn revoke_confirmation() {
    let system = System::new();
    system.init_logger_with_default_filter("gwasm=debug,gtest=info,sails_rs=debug");

    for id in USERS {
        system.mint_to(*id, DEFAULT_USERS_INITIAL_BALANCE);
    }

    let env = GtestEnv::new(system, USERS[0].into());

    let program_code_id = env.system().submit_code(multisig_wallet::WASM_BINARY);

    let program = env
        .deploy::<multisig_wallet_client::MultisigWalletProgram>(program_code_id, b"salt".to_vec())
        .new(vec![USERS[0].into(), USERS[1].into(), USERS[2].into()], 2)
        .await
        .unwrap();

    let mut service_client = program.multisig_wallet();

    service_client
        .submit_transaction(1.into(), vec![], 0, None)
        .with_value(10_000_000_000_000u128)
        .await
        .unwrap();

    let state = service_client.get_state().await.unwrap();
    assert_eq!(state.confirmations[0].1.len(), 1);

    service_client.revoke_confirmation(0.into()).await.unwrap();

    let state = service_client.get_state().await.unwrap();
    assert_eq!(state.confirmations[0].1.len(), 0);
}
