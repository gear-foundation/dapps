use crate::*;
use catalog_io::*;
use gstd::{exec, msg, ActorId};
use types::primitives::{CollectionId, PartId, TokenId};
pub const REPLY_PROVISION: u64 = 1_000_000_000;

pub fn add_child_msg(
    parent_contract_id: &ActorId,
    parent_token_id: TokenId,
    child_token_id: TokenId,
) -> MessageId {
    let msg_id = msg::send(
        *parent_contract_id,
        RMRKAction::AddChild {
            parent_token_id,
            child_token_id,
        },
        0,
    )
    .expect("Error in sending message [RMRKAction::AddChild]");
    exec::reply_deposit(msg_id, REPLY_PROVISION).expect("Failed to create a reply provision");
    msg_id
}

pub fn burn_child_msg(
    parent_contract_id: &ActorId,
    parent_token_id: TokenId,
    child_token_id: TokenId,
) -> MessageId {
    let msg_id = msg::send(
        *parent_contract_id,
        RMRKAction::BurnChild {
            parent_token_id,
            child_token_id,
        },
        0,
    )
    .expect("Error in sending message [RMRKAction::BurnChild]");
    exec::reply_deposit(msg_id, REPLY_PROVISION).expect("Failed to create a reply provision");
    msg_id
}
pub fn get_root_owner_msg(contract_id: &ActorId, token_id: TokenId) -> MessageId {
    let msg_id = msg::send(*contract_id, RMRKAction::RootOwner(token_id), 0)
        .expect("Error in sending message [RMRKAction::RootOwner]");
    exec::reply_deposit(msg_id, REPLY_PROVISION).expect("Failed to create a reply provision");
    msg_id
}

pub fn burn_from_parent_msg(child_contract_id: &ActorId, child_token_id: TokenId) -> MessageId {
    let msg_id = msg::send(
        *child_contract_id,
        RMRKAction::BurnFromParent { child_token_id },
        0,
    )
    .expect("Error in sending message [RMRKAction::BurnFromParent]");
    exec::reply_deposit(msg_id, REPLY_PROVISION).expect("Failed to create a reply provision");
    msg_id
}

pub fn transfer_child_msg(
    parent_contract_id: &ActorId,
    from: TokenId,
    to: TokenId,
    child_token_id: TokenId,
) -> MessageId {
    let msg_id = msg::send(
        *parent_contract_id,
        RMRKAction::TransferChild {
            from,
            to,
            child_token_id,
        },
        0,
    )
    .expect("Error in sending async message `[RMRKAction::TransferChild]`");
    exec::reply_deposit(msg_id, REPLY_PROVISION).expect("Failed to create a reply provision");
    msg_id
}

pub fn check_equippable_msg(
    catalog_id: &ActorId,
    part_id: PartId,
    collection_id: &CollectionId,
) -> MessageId {
    let msg_id = msg::send(
        *catalog_id,
        CatalogAction::CheckEquippable {
            part_id,
            collection_id: *collection_id,
        },
        0,
    )
    .expect("Error in sending a message");
    exec::reply_deposit(msg_id, REPLY_PROVISION).expect("Failed to create a reply provision");
    msg_id
}

pub fn can_token_be_equipped_msg(
    child_id: &ActorId,
    parent_id: &ActorId,
    token_id: TokenId,
    asset_id: u64,
    slot_part_id: PartId,
) -> MessageId {
    let msg_id = msg::send(
        *child_id,
        RMRKAction::CanTokenBeEquippedWithAssetIntoSlot {
            parent_id: *parent_id,
            token_id,
            asset_id,
            slot_part_id,
        },
        0,
    )
    .expect("Error in sending a message");
    exec::reply_deposit(msg_id, REPLY_PROVISION).expect("Failed to create a reply provision");
    msg_id
}
