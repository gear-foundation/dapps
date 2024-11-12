use gtest::Log;
use multisig_wallet_client::traits::*;
use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
    ActorId, Encode,
};

const USERS: &[u64] = &[3, 4, 5, 6];

#[tokio::test]
async fn check_submit_and_confirm() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(multisig_wallet::WASM_BINARY);

    let program_factory = multisig_wallet_client::MultisigWalletFactory::new(remoting.clone());

    let program_id = program_factory
        .new(vec![USERS[0].into(), USERS[1].into(), USERS[2].into()], 2)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = multisig_wallet_client::MultisigWallet::new(remoting.clone());

    // submit transaction
    service_client
        .submit_transaction(1.into(), vec![], 0, None)
        .with_value(10_000_000_000_000)
        .send_recv(program_id)
        .await
        .unwrap();

    // check state after submit transaction
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.transaction_count, 1.into());
    assert!(!state.confirmations.is_empty());
    assert_eq!(state.transactions[0].1.executed, false);

    // check that mail is empty
    let mail = remoting.system().get_mailbox(1 as u64);
    let log = Log::builder()
        .dest(1 as u64)
        .payload_bytes(vec![])
        .source(program_id);
    assert!(!mail.contains(&log));

    // confirm transaction
    service_client
        .confirm_transaction(0.into())
        .with_args(GTestArgs::new(USERS[2].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    // check that mail have necessary message
    let mail = remoting.system().get_mailbox(1 as u64);
    let log = Log::builder()
        .dest(1 as u64)
        .payload_bytes(vec![])
        .source(program_id);
    assert!(mail.contains(&log));

    // check state executed
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.transactions[0].1.executed, true);
}

#[tokio::test]
async fn change_required_confirmations_count() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(multisig_wallet::WASM_BINARY);

    let program_factory = multisig_wallet_client::MultisigWalletFactory::new(remoting.clone());

    let program_id = program_factory
        .new(vec![USERS[0].into(), USERS[1].into(), USERS[2].into()], 2)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = multisig_wallet_client::MultisigWallet::new(remoting.clone());

    // submit transaction
    let payload = [
        "MultisigWallet".encode(),
        "ChangeRequiredConfirmationsCount".to_string().encode(),
        (3).encode(),
    ]
    .concat();
    service_client
        .submit_transaction(program_id, payload, 0, None)
        .send_recv(program_id)
        .await
        .unwrap();

    // check state: required
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.required, 2);

    // confirm transaction
    service_client
        .confirm_transaction(0.into())
        .with_args(GTestArgs::new(USERS[2].into()))
        .send_recv(program_id)
        .await
        .unwrap();

    // check state: required + 1
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.required, 3);
}

#[tokio::test]
async fn add_owner_remove_owner_and_replace() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(multisig_wallet::WASM_BINARY);

    let program_factory = multisig_wallet_client::MultisigWalletFactory::new(remoting.clone());

    let program_id = program_factory
        .new(vec![USERS[0].into(), USERS[1].into()], 1)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = multisig_wallet_client::MultisigWallet::new(remoting.clone());

    // check state before add owner
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert!(!state.owners.contains(&USERS[2].into()));

    // add owner
    let payload = [
        "MultisigWallet".encode(),
        "AddOwner".to_string().encode(),
        (<u64 as Into<ActorId>>::into(USERS[2])).encode(),
    ]
    .concat();
    service_client
        .submit_transaction(program_id, payload, 0, None)
        .send_recv(program_id)
        .await
        .unwrap();

    // check state after add owner
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert!(state.owners.contains(&USERS[2].into()));
    assert!(state.owners.contains(&USERS[1].into()));

    // remove owner
    let payload = [
        "MultisigWallet".encode(),
        "RemoveOwner".to_string().encode(),
        (<u64 as Into<ActorId>>::into(USERS[1])).encode(),
    ]
    .concat();

    service_client
        .submit_transaction(program_id, payload, 0, None)
        .send_recv(program_id)
        .await
        .unwrap();

    // check state after remove owner
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert!(!state.owners.contains(&USERS[1].into()));

    // replace owner
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
        .send_recv(program_id)
        .await
        .unwrap();

    // check state after replace owner
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert!(state.owners.contains(&USERS[1].into()));
    assert!(!state.owners.contains(&USERS[2].into()));
}

#[tokio::test]
async fn revoke_confirmation() {
    let system = System::new();
    system.init_logger();
    USERS.iter().for_each(|id| {
        system.mint_to(*id, 1_000_000_000_000_000);
    });

    let remoting = GTestRemoting::new(system, USERS[0].into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(multisig_wallet::WASM_BINARY);

    let program_factory = multisig_wallet_client::MultisigWalletFactory::new(remoting.clone());

    let program_id = program_factory
        .new(vec![USERS[0].into(), USERS[1].into(), USERS[2].into()], 2)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = multisig_wallet_client::MultisigWallet::new(remoting.clone());

    // submit transaction
    service_client
        .submit_transaction(1.into(), vec![], 0, None)
        .with_value(10_000_000_000_000)
        .send_recv(program_id)
        .await
        .unwrap();

    // check state after submit transaction
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.confirmations[0].1.len(), 1);

    // revoke confirmation
    service_client
        .revoke_confirmation(0.into())
        .send_recv(program_id)
        .await
        .unwrap();

    // check state after evoke confirmation
    let state = service_client.get_state().recv(program_id).await.unwrap();
    assert_eq!(state.confirmations[0].1.len(), 0);
}
