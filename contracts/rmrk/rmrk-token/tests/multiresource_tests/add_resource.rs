use crate::utils::*;
use gstd::BTreeSet;
use gtest::{Program, System};
use resource_io::Resource;
use types::primitives::ResourceId;

// adds resource entry to the resource storage contract through the rmrk token contract
#[test]
fn add_resource_entry_simple() {
    let sys = System::new();

    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let token_id: u64 = 10;
    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());

    mint_token_and_add_resource(&rmrk, token_id, resource_id, resource);
}

#[test]
fn add_resource_entry_failures() {
    let sys = System::new();
    sys.init_logger();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let token_id: u64 = 10;
    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());
    mint_token_and_add_resource(&rmrk, token_id, resource_id, resource.clone());

    // must fail since resource already exists
    rmrk.add_resource_entry(
        USERS[0],
        resource_id,
        resource.clone(),
        Some("Error in async message `[ResourceAction::AddResourceEntry]`"),
    );

    // must fail since resource id is zero
    rmrk.add_resource_entry(
        USERS[0],
        0,
        resource,
        Some("Error in async message `[ResourceAction::AddResourceEntry]`"),
    );
}

// propose resource for the token
#[test]
fn add_resource_to_token() {
    let sys = System::new();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let token_id: u64 = 10;
    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());

    mint_token_and_add_resource_to_token(&rmrk, token_id, resource_id, resource, 0);

    // check pending resources
    let mut pending_resources: BTreeSet<ResourceId> = BTreeSet::new();
    pending_resources.insert(resource_id);
    rmrk.check_pending_resources(token_id, pending_resources);

    // check active resources
    rmrk.check_active_resources(token_id, BTreeSet::new());
}

#[test]
fn add_resource_to_token_failures() {
    let sys = System::new();
    sys.init_logger();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let token_id: u64 = 10;
    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());

    mint_token_and_add_resource(&rmrk, token_id, resource_id, resource.clone());

    let mut pending_resources: BTreeSet<ResourceId> = BTreeSet::new();

    // must fail since cannot add resource with not added resource id to resource contract
    rmrk.add_resource(
        USERS[0],
        token_id,
        2,
        0,
        Some("Error in async message `[ResourceAction::GetResource]`"),
    );

    // must fail since cannot since token does not exist
    rmrk.add_resource(
        USERS[0],
        11,
        resource_id,
        0,
        Some("RMRK: Token does not exist"),
    );

    // add resource
    rmrk.add_resource(USERS[0], token_id, resource_id, 0, None);

    rmrk.accept_resource(USERS[0], token_id, resource_id, None);

    // must fail since that resource has already been added
    rmrk.add_resource(
        USERS[0],
        token_id,
        resource_id,
        0,
        Some("Resource already exists on token"),
    );

    for resource_id in 2..130 {
        rmrk.add_resource_entry(USERS[0], resource_id, resource.clone(), None);
        pending_resources.insert(resource_id);
        rmrk.add_resource(USERS[0], token_id, resource_id, 0, None);
    }

    // must fail since too many resources have already been added to token
    let resource_id: ResourceId = 130;
    rmrk.add_resource_entry(USERS[0], resource_id, resource, None);
    rmrk.add_resource(
        USERS[0],
        token_id,
        resource_id,
        0,
        Some("Max pending resources reached"),
    );

    // check pending resources
    rmrk.check_pending_resources(token_id, pending_resources);
}

#[test]
fn add_resource_to_different_tokens() {
    let sys = System::new();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");
    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));
    let token_id_0: u64 = 10;
    let token_id_1: u64 = 11;
    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());

    mint_token_and_add_resource_to_token(&rmrk, token_id_0, resource_id, resource, 0);

    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id_1, None);

    // add the same resource to token_id_1
    rmrk.add_resource(USERS[0], token_id_1, resource_id, 0, None);

    let mut pending_resources: BTreeSet<ResourceId> = BTreeSet::new();
    pending_resources.insert(resource_id);

    // check pending resources of token_id_0
    rmrk.check_pending_resources(token_id_0, pending_resources.clone());

    // check pending resources of token_id_1
    rmrk.check_pending_resources(token_id_1, pending_resources);
}
