use crate::utils::*;
use gtest::{Program, System};
use hashbrown::HashSet;
use rmrk_io::RMRKError;
use types::primitives::{CollectionId, TokenId};
#[test]
fn accept_child_simple() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id`, add child to it and accept child
    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // check that parent_token_id has no pending children
    rmrk_parent.check_pending_children(parent_token_id, HashSet::new());

    // check accepted children
    let mut accepted_children: HashSet<(CollectionId, TokenId)> = HashSet::new();
    accepted_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    rmrk_parent.check_accepted_children(parent_token_id, accepted_children);
}

#[test]
fn accept_child_from_approved_address() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id` and add child to it
    mint_parent_and_child(&rmrk_child, &rmrk_parent, child_token_id, parent_token_id);

    rmrk_parent.approve(USERS[0], USERS[3], parent_token_id);

    rmrk_parent.accept_child(
        USERS[3],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );
}

#[test]
fn accept_child_failures() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id` and add child to it
    mint_parent_and_child(&rmrk_child, &rmrk_parent, child_token_id, parent_token_id);

    // fail since the caller is not the owner
    rmrk_parent.accept_child(
        USERS[3],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        Some(RMRKError::NotApprovedAccount),
    );

    // fail since the child with that ID does not exist
    rmrk_parent.accept_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        2,
        Some(RMRKError::ChildDoesNotExist),
    );

    // accept child
    rmrk_parent.accept_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );

    // fail since child has alredy been accepted
    rmrk_parent.accept_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        Some(RMRKError::WrongChildStatus),
    );
}

#[test]
fn reject_child_simple() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id` and add child to it
    mint_parent_and_child(&rmrk_child, &rmrk_parent, child_token_id, parent_token_id);

    rmrk_parent.reject_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );

    // // check that parent_token_id has no pending children
    // rmrk_parent.check_pending_children(parent_token_id, BTreeSet::new());

    // // check that child token in rmrk_child does not exist
    // rmrk_child.check_rmrk_owner(child_token_id, None, ZERO_ID);
}

#[test]
fn reject_child_from_approved_address() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id` and add child to it
    mint_parent_and_child(&rmrk_child, &rmrk_parent, child_token_id, parent_token_id);

    // approve to USERS[3]
    rmrk_parent.approve(USERS[0], USERS[3], parent_token_id);
    // reject child from USERS[3]
    rmrk_parent.reject_child(
        USERS[3],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );
}

#[test]
fn reject_child_failures() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id` and add child to it
    mint_parent_and_child(&rmrk_child, &rmrk_parent, child_token_id, parent_token_id);

    // must fail since the caller is not owner or not approved account
    rmrk_parent.reject_child(
        USERS[3],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        Some(RMRKError::NotApprovedAccount),
    );

    // must fail since the child with indicated id does not exist
    rmrk_parent.reject_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        100,
        Some(RMRKError::ChildDoesNotExist),
    );
}

#[test]
fn remove_child_simple() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id`, add child to it and accept child
    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // remove child
    rmrk_parent.remove_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );

    //     // check that parent_token_id has no accepted children
    //     rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());

    //     // check that child token in rmrk_child does not exist
    //     rmrk_child.check_rmrk_owner(child_token_id, None, ZERO_ID);
}

#[test]
fn remove_child_from_approved_account() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id`, add child to it and accept child
    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    rmrk_parent.approve(USERS[0], USERS[3], parent_token_id);

    rmrk_parent.remove_child(
        USERS[3],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        None,
    );
    // // check that parent_token_id has no accepted children
    // rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());
}

#[test]
fn remove_child_failures() {
    let sys = System::new();
    sys.init_logger();

    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id`, add child to it and accept child
    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // must fail since the caller is not owner or not approved account
    rmrk_parent.remove_child(
        USERS[3],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_token_id,
        Some(RMRKError::NotApprovedAccount),
    );

    // must fail since the child with indicated id does not exist
    rmrk_parent.remove_child(
        USERS[0],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        100,
        Some(RMRKError::ChildDoesNotExist),
    );
}
