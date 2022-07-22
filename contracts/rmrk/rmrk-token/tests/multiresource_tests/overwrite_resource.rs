use crate::utils::*;
use gstd::BTreeSet;
use gtest::{Program, System};
use resource_io::Resource;
use types::primitives::ResourceId;

#[test]
fn overwrite_resource() {
    let sys = System::new();

    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id: ResourceId = 1;
    let new_resource_id: ResourceId = 2;
    let resource = Resource::Basic(Default::default());
    let new_resource = Resource::Basic(Default::default());
    let token_id: u64 = 10;

    // mint token
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    // add resource entry to storage contract
    rmrk.add_resource_entry(USERS[0], resource_id, resource, None);
    // add overwrite resource to storage contract
    rmrk.add_resource_entry(USERS[0], new_resource_id, new_resource, None);

    // add and accept resource_id
    rmrk.add_resource(USERS[0], token_id, resource_id, 0, None);
    rmrk.accept_resource(USERS[0], token_id, resource_id, None);

    // add resource to overwrite
    rmrk.add_resource(USERS[0], token_id, new_resource_id, resource_id, None);

    // check pending resources
    let mut resources: BTreeSet<ResourceId> = BTreeSet::new();
    resources.insert(new_resource_id);
    rmrk.check_pending_resources(token_id, resources.clone());

    // accept new resource instead of previous one
    rmrk.accept_resource(USERS[0], token_id, new_resource_id, None);
    // check active resources
    rmrk.check_active_resources(token_id, resources);
}

#[test]
fn overwrite_resource_failures() {
    let sys = System::new();
    sys.init_logger();
    // Prepare resource
    let code_hash_stored =
        sys.submit_code("../target/wasm32-unknown-unknown/release/rmrk_resource.opt.wasm");

    let rmrk = Program::rmrk(&sys, Some(code_hash_stored.into()));

    let resource_id_1: ResourceId = 1;
    let resource_id_2: ResourceId = 2;
    let resource_id_3: ResourceId = 3;

    let resource_1 = Resource::Basic(Default::default());
    let resource_2 = Resource::Basic(Default::default());
    let resource_3 = Resource::Basic(Default::default());
    // add resources entry to storage contract
    rmrk.add_resource_entry(USERS[0], resource_id_1, resource_1, None);
    rmrk.add_resource_entry(USERS[0], resource_id_2, resource_2, None);
    rmrk.add_resource_entry(USERS[0], resource_id_3, resource_3, None);

    let token_id: u64 = 10;

    // mint token
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    // must fail since no resource to overwrite
    rmrk.add_resource(
        USERS[0],
        token_id,
        resource_id_2,
        resource_id_1,
        Some("No resources to overwrite"),
    );

    // add and accept resource_id
    rmrk.add_resource(USERS[0], token_id, resource_id_1, 0, None);
    rmrk.accept_resource(USERS[0], token_id, resource_id_1, None);

    // must fail since Proposed overwritten resource must exist on token
    rmrk.add_resource(
        USERS[0],
        token_id,
        resource_id_3,
        resource_id_2,
        Some("Proposed overwritten resource must exist on token"),
    );
}
