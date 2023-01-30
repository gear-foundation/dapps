use crate::utils::*;
use gtest::{Program, System};
use resource_io::Resource;
use types::primitives::ResourceId;

#[test]
fn accept_resource_simple() {
    let sys = System::new();

    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());
    let token_id: u64 = 10;

    mint_token_and_add_resource_to_token_with_acceptance(
        &rmrk,
        USERS[0],
        token_id,
        resource_id,
        resource,
    );

    // // check pending resources
    // rmrk.check_pending_resources(token_id, BTreeSet::new());

    // // check active resources
    // let mut active_resources: BTreeSet<ResourceId> = BTreeSet::new();
    // active_resources.insert(resource_id);
    // rmrk.check_active_resources(token_id, active_resources);
}

#[test]
fn accept_resource_from_approved_address() {
    let sys = System::new();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());
    let token_id: u64 = 10;

    mint_token_and_add_resource_to_token(&rmrk, token_id, resource_id, resource, 0);

    rmrk.approve(USERS[0], USERS[3], token_id);

    rmrk.accept_resource(USERS[3], token_id, resource_id, None);

    // // check pending resources
    // rmrk.check_pending_resources(token_id, BTreeSet::new());

    // // check active resources
    // let mut active_resources: BTreeSet<ResourceId> = BTreeSet::new();
    // active_resources.insert(resource_id);
    // rmrk.check_active_resources(token_id, active_resources);
}

#[test]
fn accept_resource_failures() {
    let sys = System::new();
    sys.init_logger();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());
    let token_id: u64 = 10;

    mint_token_and_add_resource(&rmrk, token_id, resource_id, resource);

    // must fail since token has no pending resources
    rmrk.accept_resource(
        USERS[0],
        token_id,
        resource_id,
        Some("RMRK: Token has no pending resources"),
    );

    rmrk.add_resource(USERS[0], token_id, resource_id, 0, None);

    // must fail since not owner/approved tries to accept resource
    rmrk.accept_resource(USERS[2], token_id, resource_id, Some("RMRK: Wrong owner"));

    // must fail since resource with indicated id does not exist
    rmrk.accept_resource(
        USERS[0],
        token_id,
        2,
        Some("RMRK: Resource does not exist in token pending resources"),
    );
}

#[test]
fn accept_multiple_resources() {
    let sys = System::new();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));
    let token_id: u64 = 10;
    let resource_id_1: ResourceId = 1;
    let resource_id_2: ResourceId = 2;
    let resource = Resource::Basic(Default::default());

    mint_token_and_add_resource_to_token_with_acceptance(
        &rmrk,
        USERS[0],
        token_id,
        resource_id_1,
        resource.clone(),
    );

    rmrk.add_resource_entry(USERS[0], resource_id_2, resource, None);

    rmrk.add_resource(USERS[0], token_id, resource_id_2, 0, None);

    rmrk.accept_resource(USERS[0], token_id, resource_id_2, None);

    // // check pending resources
    // rmrk.check_pending_resources(token_id, BTreeSet::new());

    // // check active resources
    // let mut active_resources: BTreeSet<ResourceId> = BTreeSet::new();
    // active_resources.insert(resource_id_1);
    // active_resources.insert(resource_id_2);
    // rmrk.check_active_resources(token_id, active_resources);
}

#[test]
fn reorder_prioroties() {
    let sys = System::new();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));
    let token_id: u64 = 10;
    let resource = Resource::Basic(Default::default());

    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    for resource_id in 1..6 {
        rmrk.add_resource_entry(USERS[0], resource_id, resource.clone(), None);
        rmrk.add_resource(USERS[0], token_id, resource_id, 0, None);
        rmrk.accept_resource(USERS[0], token_id, resource_id, None);
    }
    let mut priorities = vec![1, 0, 4, 3, 2];
    rmrk.set_priority(USERS[0], token_id, priorities.clone(), None);

    // failures

    // must fail since not owner/approved tries to reorder priorities
    rmrk.set_priority(
        USERS[1],
        token_id,
        priorities.clone(),
        Some("RMRK: Wrong owner"),
    );

    // must fail since the new order has does not have the same length
    priorities.push(8);
    rmrk.set_priority(
        USERS[0],
        token_id,
        priorities.clone(),
        Some("Wrong priority list length"),
    );

    // reorder from approved address
    priorities.pop();
    rmrk.approve(USERS[0], USERS[3], token_id);
    rmrk.set_priority(USERS[3], token_id, priorities, None);
}

#[test]
fn reject_resource_simple() {
    let sys = System::new();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());
    let token_id: u64 = 10;

    mint_token_and_add_resource_to_token(&rmrk, token_id, resource_id, resource, 0);

    rmrk.reject_resource(USERS[0], token_id, resource_id, None);

    // // check pending resources
    // rmrk.check_pending_resources(token_id, BTreeSet::new());

    // // check active resources
    // rmrk.check_active_resources(token_id, BTreeSet::new());
}

#[test]
fn reject_resource_failures() {
    let sys = System::new();
    sys.init_logger();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());
    let token_id: u64 = 10;

    mint_token_and_add_resource(&rmrk, token_id, resource_id, resource);

    // must fail since token does not have any pending resources
    rmrk.reject_resource(
        USERS[0],
        token_id,
        resource_id,
        Some("RMRK: Token has no pending resources"),
    );

    // add resource index
    rmrk.add_resource(USERS[0], token_id, resource_id, 0, None);

    // must fail since resource does not exist
    rmrk.reject_resource(
        USERS[0],
        token_id,
        10,
        Some("RMRK: Resource does not exist"),
    );

    // must fail since not owner/approved tries to reject resource
    rmrk.reject_resource(USERS[3], token_id, resource_id, Some("RMRK: Wrong owner"));
}

#[test]
fn reject_resource_from_approved_address() {
    let sys = System::new();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id: ResourceId = 1;
    let resource = Resource::Basic(Default::default());
    let token_id: u64 = 10;

    mint_token_and_add_resource_to_token(&rmrk, token_id, resource_id, resource, 0);

    rmrk.approve(USERS[0], USERS[3], token_id);

    rmrk.reject_resource(USERS[3], token_id, resource_id, None);

    // // check pending resources
    // rmrk.check_pending_resources(token_id, BTreeSet::new());

    // // check active resources
    // rmrk.check_active_resources(token_id, BTreeSet::new());
}
