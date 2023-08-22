use gstd::{msg, prelude::*, ActorId};
use mt_storage_io::{MTStorageAction, MTStorageEvent};
use primitive_types::H256;

pub async fn get_balance(
    storage_id: &ActorId,
    token_id: u128,
    account: &ActorId,
) -> Result<u128, ()> {
    let result = msg::send_for_reply_as::<_, MTStorageEvent>(
        *storage_id,
        MTStorageAction::GetBalance {
            token_id,
            account: *account,
        },
        0,
    )
    .expect("Error in sending a message `MTStorageAction::GetBalance`.")
    .await;

    match result {
        Ok(storage_event) => match storage_event {
            MTStorageEvent::Balance(balance) => Ok(balance),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub async fn get_approval(
    storage_id: &ActorId,
    account: &ActorId,
    approval_target: &ActorId,
) -> Result<bool, ()> {
    let result = msg::send_for_reply_as::<_, MTStorageEvent>(
        *storage_id,
        MTStorageAction::GetApproval {
            account: *account,
            approval_target: *approval_target,
        },
        0,
    )
    .expect("Error in sending a message `MTStorageAction::GetApproval`.")
    .await;

    match result {
        Ok(storage_event) => match storage_event {
            MTStorageEvent::Approval(approval) => Ok(approval),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub async fn transfer(
    storage_id: &ActorId,
    transaction_hash: H256,
    token_id: u128,
    msg_source: &ActorId,
    sender: &ActorId,
    recipient: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let result = msg::send_for_reply_as::<_, MTStorageEvent>(
        *storage_id,
        MTStorageAction::Transfer {
            transaction_hash,
            token_id,
            msg_source: *msg_source,
            sender: *sender,
            recipient: *recipient,
            amount,
        },
        0,
    )
    .expect("Error in sending a message `MTStorageAction::Transfer`.")
    .await;

    match result {
        Ok(storage_event) => match storage_event {
            MTStorageEvent::Ok => Ok(()),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

pub async fn approve(
    storage_id: &ActorId,
    transaction_hash: H256,
    msg_source: &ActorId,
    account: &ActorId,
    approve: bool,
) -> Result<(), ()> {
    let result = msg::send_for_reply_as::<_, MTStorageEvent>(
        *storage_id,
        MTStorageAction::Approve {
            transaction_hash,
            msg_source: *msg_source,
            account: *account,
            approve,
        },
        0,
    )
    .expect("Error in sending a message `MTStorageAction::Approve`.")
    .await;

    match result {
        Ok(storage_event) => match storage_event {
            MTStorageEvent::Ok => Ok(()),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

#[allow(unused)]
pub async fn increase_balance(
    transaction_hash: H256,
    storage_id: &ActorId,
    token_id: u128,
    account: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let result = msg::send_for_reply_as::<_, MTStorageEvent>(
        *storage_id,
        MTStorageAction::IncreaseBalance {
            transaction_hash,
            token_id,
            account: *account,
            amount,
        },
        0,
    )
    .expect("Error in sending a message `MTStorageAction::IncreaseBalance`.")
    .await;

    match result {
        Ok(storage_event) => match storage_event {
            MTStorageEvent::Ok => Ok(()),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}

#[allow(unused)]
pub async fn decrease_balance(
    transaction_hash: H256,
    storage_id: &ActorId,
    token_id: u128,
    msg_source: &ActorId,
    account: &ActorId,
    amount: u128,
) -> Result<(), ()> {
    let result = msg::send_for_reply_as::<_, MTStorageEvent>(
        *storage_id,
        MTStorageAction::DecreaseBalance {
            transaction_hash,
            token_id,
            msg_source: *msg_source,
            account: *account,
            amount,
        },
        0,
    )
    .expect("Error in sending a message `MTStorageAction::DecreaseBalance`.")
    .await;

    match result {
        Ok(storage_event) => match storage_event {
            MTStorageEvent::Ok => Ok(()),
            _ => Err(()),
        },
        Err(_) => Err(()),
    }
}
