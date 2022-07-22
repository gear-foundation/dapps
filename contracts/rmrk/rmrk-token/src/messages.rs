use crate::*;
use base_io::*;
use gstd::{exec, msg, ActorId};
use resource_io::Resource;
use types::primitives::{CollectionId, PartId, ResourceId, TokenId};

pub async fn get_root_owner(to: &ActorId, token_id: TokenId) -> ActorId {
    let response: RMRKEvent = msg::send_for_reply_as(*to, RMRKAction::RootOwner(token_id), 0)
        .expect("Error in sending message [RMRKAction::RootOwner]")
        .await
        .expect("Error in message [RMRKAction::RootOwner]");

    if let RMRKEvent::RootOwner(root_owner) = response {
        root_owner
    } else {
        panic!("wrong received message");
    }
}

pub async fn add_child(
    parent_contract_id: &ActorId,
    parent_token_id: TokenId,
    child_token_id: TokenId,
) {
    msg::send_for_reply_as::<_, RMRKEvent>(
        *parent_contract_id,
        RMRKAction::AddChild {
            parent_token_id,
            child_token_id,
        },
        0,
    )
    .expect("Error in sending message [RMRKAction::AddChild]")
    .await
    .expect("Error in message [RMRKAction::AddChild]");
}

pub async fn burn_from_parent(
    child_contract_id: &ActorId,
    child_token_id: TokenId,
    root_owner: &ActorId,
) {
    msg::send_for_reply_as::<_, RMRKEvent>(
        *child_contract_id,
        RMRKAction::BurnFromParent {
            child_token_id,
            root_owner: *root_owner,
        },
        0,
    )
    .expect("Error in sending message [RMRKAction::BurnFromParent]")
    .await
    .expect("Error in message [RMRKAction::BurnFromParent]");
}

pub async fn burn_child(
    parent_contract_id: &ActorId,
    parent_token_id: TokenId,
    child_token_id: TokenId,
) {
    msg::send_for_reply_as::<_, RMRKEvent>(
        *parent_contract_id,
        RMRKAction::BurnChild {
            parent_token_id,
            child_token_id,
        },
        0,
    )
    .expect("Error in sending message [RMRKAction::BurnChild]")
    .await
    .expect("Error in message [RMRKAction::BurnChild]");
}

pub async fn transfer_child(
    parent_contract_id: &ActorId,
    from: TokenId,
    to: TokenId,
    child_token_id: TokenId,
) {
    msg::send_for_reply_as::<_, RMRKEvent>(
        *parent_contract_id,
        RMRKAction::TransferChild {
            from,
            to,
            child_token_id,
        },
        0,
    )
    .expect("Error in sending async message `[RMRKAction::TransferChild]`")
    .await
    .expect("Error in async message `[RMRKAction::TransferChild]`");
}

pub async fn add_accepted_child(
    parent_contract_id: &ActorId,
    parent_token_id: TokenId,
    child_token_id: TokenId,
) {
    msg::send_for_reply_as::<_, RMRKEvent>(
        *parent_contract_id,
        RMRKAction::AddAcceptedChild {
            parent_token_id,
            child_token_id,
        },
        0,
    )
    .expect("Error in sending async message `[RMRKAction::AddAcceptedChild]`")
    .await
    .expect("Error in  async message `[RMRKAction::AddAcceptedChild]");
}

pub async fn add_resource_entry(to: &ActorId, resource_id: ResourceId, resource: Resource) {
    msg::send_for_reply_as::<_, ResourceEvent>(
        *to,
        ResourceAction::AddResourceEntry {
            resource_id,
            resource,
        },
        0,
    )
    .expect(
        "Error in sending async message `[ResourceAction::AddResourceEntry]` to resource contract",
    )
    .await
    .expect("Error in async message `[ResourceAction::AddResourceEntry]`");
}

pub async fn assert_resource_exists(resource_address: &ActorId, id: u8) {
    msg::send_for_reply_as::<_, ResourceEvent>(
        *resource_address,
        ResourceAction::GetResource { id },
        0,
    )
    .expect("Error in sending async message `[ResourceAction::GetResource]` to resource contract")
    .await
    .expect("Error in async message `[ResourceAction::GetResource]`");
}

pub async fn get_resource(resource_address: &ActorId, id: ResourceId) -> Resource {
    let response: ResourceEvent = msg::send_for_reply_as(
        *resource_address,
        ResourceAction::GetResource { id },
        0,
    )
    .expect("Error in sending async message `[ResourceAction::GetResource]` to resource contract")
    .await
    .expect("Error in async message `[ResourceAction::GetResource]`");
    if let ResourceEvent::Resource(resource) = response {
        resource
    } else {
        panic!("Wrong received message from resource contract");
    }
}

pub async fn check_is_in_equippable_list(base_id: BaseId, part_id: PartId, token_id: TokenId) {
    msg::send_for_reply_as::<_, BaseEvent>(
        base_id,
        BaseAction::CheckEquippable {
            part_id,
            collection_id: exec::program_id(),
            token_id,
        },
        0,
    )
    .expect("Error in sending async message `[BaseAction::CheckEquippable]` to base contract")
    .await
    .expect("Error in async message `[BaseAction::CheckEquippable]`");
}

pub async fn check_equippable(
    parent_contract_id: CollectionId,
    parent_token_id: TokenId,
    child_token_id: TokenId,
    resource_id: ResourceId,
    slot_id: PartId,
) {
    msg::send_for_reply_as::<_, RMRKEvent>(
        parent_contract_id,
        RMRKAction::CheckEquippable {
            parent_token_id,
            child_token_id,
            resource_id,
            slot_id,
        },
        0,
    )
    .expect("Error in sending async message `[RMRKAction::CheckEquippable]` to rmrk contract")
    .await
    .expect("Error in async message `[RMRKAction::CheckEquippable]`");
}

pub async fn add_part_to_resource(
    resource_contract_id: ActorId,
    resource_id: ResourceId,
    part_id: PartId,
) {
    msg::send_for_reply_as::<_, ResourceEvent>(
        resource_contract_id,
        ResourceAction::AddPartToResource {
            resource_id,
            part_id,
        },
        0,
    )
    .expect(
        "Error in sending async message `[ResourceAction::AddPartToResource]` to resource contract",
    )
    .await
    .expect("Error in async message `[ResourceAction::AddPartToResource]`");
}
