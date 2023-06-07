use crate::utils::*;
use gtest::{Program, System};
use resource_io::*;
use types::primitives::{CollectionAndToken, ResourceId};

#[test]
fn equip_test() {
    let sys = System::new();
    sys.init_logger();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/debug/rmrk_resource.opt.wasm");
    // init child contract with resource
    let rmrk_child = Program::rmrk(&sys, Some(code_hash_stored.into()));

    // init parent contract with resource
    let rmrk_parent = Program::rmrk(&sys, Some(code_hash_stored.into()));

    // init base contract
    init_base(&sys);

    let parent_token_id: u64 = 200;
    let child_token_id: u64 = 205;
    let slot_part_id = 400;

    let equippable: CollectionAndToken = (PARENT_NFT_CONTRACT.into(), parent_token_id.into());
    // mint parent token
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], parent_token_id, None);

    // mint child token
    rmrk_child.mint_to_root_owner(USERS[0], USERS[0], child_token_id, None);

    // equip child token: fail since token has no resource
    rmrk_child.equip(
        child_token_id,
        CHILD_RESOURCE_ID,
        equippable,
        PARENT_RESOURCE_ID,
        Some("Token has no active resources"),
    );

    // add basic resource to child token
    let basic_resource_id: ResourceId = 10;
    let basic_resource = Resource::Basic(Default::default());
    add_resource_to_token(
        &rmrk_child,
        child_token_id,
        basic_resource_id,
        basic_resource.clone(),
    );

    // equip child token: fail since the indicated resource is not slot
    rmrk_child.equip(
        child_token_id,
        basic_resource_id,
        equippable,
        PARENT_RESOURCE_ID,
        Some("The resource must be slot"),
    );

    // add slot resource for child token
    let slot_resource_id: ResourceId = 11;
    let resource = Resource::Slot(SlotResource {
        base: BASE_ID.into(),
        slot: slot_part_id,
        ..Default::default()
    });
    add_resource_to_token(&rmrk_child, child_token_id, slot_resource_id, resource);

    // equip child token: must fail token is not owned by another token
    rmrk_child.equip(
        child_token_id,
        slot_resource_id,
        equippable,
        PARENT_RESOURCE_ID,
        Some("Error in async message `[RMRKAction::CheckEquippable]`"),
    );

    // transfer child token to parent token
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        parent_token_id,
        None,
    );

    // add basic resource to parent token
    add_resource_to_token(
        &rmrk_parent,
        parent_token_id,
        basic_resource_id,
        basic_resource,
    );

    // equip child token: must fail since parent's resource is not composed
    rmrk_child.equip(
        child_token_id,
        slot_resource_id,
        equippable,
        basic_resource_id,
        Some("Error in async message `[RMRKAction::CheckEquippable]`"),
    );

    // add composed resource to parent token
    let composed_resource_id: ResourceId = 11;
    let resource = Resource::Composed(ComposedResource {
        base: BASE_ID.into(),
        ..Default::default()
    });

    add_resource_to_token(
        &rmrk_parent,
        parent_token_id,
        composed_resource_id,
        resource,
    );

    // should equip
    rmrk_child.equip(
        child_token_id,
        slot_resource_id,
        equippable,
        composed_resource_id,
        None,
    );

    // must fail since token is already equipped
    rmrk_child.equip(
        child_token_id,
        slot_resource_id,
        equippable,
        basic_resource_id,
        Some("Token is already equipped"),
    );
}

fn add_resource_to_token(
    rmrk: &Program,
    token_id: u64,
    resource_id: ResourceId,
    resource: Resource,
) {
    rmrk.add_resource_entry(USERS[0], resource_id, resource, None);
    rmrk.add_resource(USERS[0], token_id, resource_id, 0, None);
    rmrk.accept_resource(USERS[0], token_id, resource_id, None);
}
