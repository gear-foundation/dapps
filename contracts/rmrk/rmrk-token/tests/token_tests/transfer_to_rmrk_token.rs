use crate::utils::*;
use gtest::{Program, System};
// Root owner transfers accepted child token to between his RMRK tokens inside one contract
#[test]
fn transfer_accepted_child_to_token_with_same_owner() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let new_parent_token_id: u64 = 8;

    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // mint `new_parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], new_parent_token_id, None);

    // USERS[0] transfer child to another his token
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        new_parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(new_parent_token_id.into()),
    //     PARENT_NFT_CONTRACT,
    // );

    // // check accepted children of parent_token_id
    // rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());

    // // check accepted children of new_parent_token_id
    // let mut accepted_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // accepted_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // rmrk_parent.check_accepted_children(new_parent_token_id, accepted_children);
}

// Root owner transfers pending child token to between his RMRK tokens inside one contract
#[test]
fn transfer_pending_child_to_token_with_same_owner() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let new_parent_token_id: u64 = 8;

    mint_parent_and_child(&rmrk_child, &rmrk_parent, child_token_id, parent_token_id);

    // mint `new_parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], new_parent_token_id, None);

    // USERS[0] transfer child to another his token
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        new_parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(new_parent_token_id.into()),
    //     PARENT_NFT_CONTRACT,
    // );

    // // check pending children of parent_token_id
    // rmrk_parent.check_pending_children(parent_token_id, BTreeSet::new());

    // // check pending children of new_parent_token_id
    // let mut pending_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // pending_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // rmrk_parent.check_pending_children(new_parent_token_id, pending_children);
}

// Root owner transfers accepted child token to RMRK token that he does not own inside one contract
#[test]
fn transfer_accepted_child_to_token_with_different_owner() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let new_parent_token_id: u64 = 12;

    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // mint `new_parent_token_id` to USERS[1]
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[1], new_parent_token_id, None);

    // USERS[0] transfer child to token that he does not own
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        new_parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(new_parent_token_id.into()),
    //     PARENT_NFT_CONTRACT,
    // );

    // // check accepted children of parent_token_id
    // rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());

    // // check pending children of new_parent_token_id
    // let mut pending_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // pending_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // rmrk_parent.check_pending_children(new_parent_token_id, pending_children);
}

// Root owner transfers pending child token to  RMRK token that he does not own inside one contract
#[test]
fn transfer_pending_child_to_token_with_different_owner() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let new_parent_token_id: u64 = 12;

    mint_parent_and_child(&rmrk_child, &rmrk_parent, child_token_id, parent_token_id);

    // mint `new_parent_token_id` to USERS[1]
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[1], new_parent_token_id, None);

    // USERS[0] transfer child to another his token
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        new_parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(new_parent_token_id.into()),
    //     PARENT_NFT_CONTRACT,
    // );

    // // check accepted children of parent_token_id
    // rmrk_parent.check_pending_children(parent_token_id, BTreeSet::new());

    // // check pending children of new_parent_token_id
    // let mut pending_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // pending_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // rmrk_parent.check_pending_children(new_parent_token_id, pending_children);
}

// Root owner transfers accepted child token to his RMRK token in another RMRK contract
#[test]
fn transfer_accepted_child_to_token_with_same_owner_another_contract() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);
    let new_rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let new_parent_token_id: u64 = 8;
    let new_parent_contract_id: u64 = 3;

    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // mint `new_parent_token_id`
    new_rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], new_parent_token_id, None);

    // USERS[0] transfer child to another his token in another rmrk contract
    rmrk_child.transfer_to_nft(
        USERS[0],
        new_parent_contract_id,
        child_token_id,
        new_parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(new_parent_token_id.into()),
    //     new_parent_contract_id,
    // );

    // // check accepted children of parent_token_id
    // rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());

    // // check accepted children of new_parent_token_id
    // let mut accepted_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // accepted_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // new_rmrk_parent.check_accepted_children(new_parent_token_id, accepted_children);
}

// Root owner transfers accepted child token to  RMRK token with different owner in another RMRK contract
#[test]
fn transfer_accepted_child_to_token_with_different_owner_another_contract() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);
    let new_rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let new_parent_token_id: u64 = 8;
    let new_parent_contract_id: u64 = 3;

    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // mint `new_parent_token_id`
    new_rmrk_parent.mint_to_root_owner(USERS[1], USERS[1], new_parent_token_id, None);

    // USERS[0] transfer child to token that he does not own in another rmrk contract
    rmrk_child.transfer_to_nft(
        USERS[0],
        new_parent_contract_id,
        child_token_id,
        new_parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(new_parent_token_id.into()),
    //     new_parent_contract_id,
    // );

    // // check accepted children of parent_token_id
    // rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());

    // // check pending children of new_parent_token_id
    // let mut pending_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // pending_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // new_rmrk_parent.check_pending_children(new_parent_token_id, pending_children);
}

// Root owner transfers usual token to his RMRK token
#[test]
fn transfer_token_to_token_with_same_owner() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;

    // mint future child token
    rmrk_child.mint_to_root_owner(USERS[0], USERS[0], child_token_id, None);

    // mint parent token
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], parent_token_id, None);

    // USERS[0] transfer child to another his token in another rmrk contract
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(parent_token_id.into()),
    //     PARENT_NFT_CONTRACT,
    // );

    // // check accepted children
    // let mut accepted_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // accepted_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // rmrk_parent.check_accepted_children(parent_token_id, accepted_children);
}

// Root owner transfers usual token to  RMRK token with different owner
#[test]
fn transfer_usual_token_to_token_with_different_owner() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 12;

    // mint future child token
    rmrk_child.mint_to_root_owner(USERS[0], USERS[0], child_token_id, None);

    // mint parent token
    rmrk_parent.mint_to_root_owner(USERS[1], USERS[1], parent_token_id, None);

    // USERS[0] transfers child to token that he does not owner
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        parent_token_id,
        None,
    );

    // // check owner
    // rmrk_child.check_rmrk_owner(
    //     child_token_id,
    //     Some(parent_token_id.into()),
    //     PARENT_NFT_CONTRACT,
    // );

    // // check pending children of parent_token_id
    // let mut pending_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // pending_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // rmrk_parent.check_pending_children(parent_token_id, pending_children);
}

#[test]
fn transfer_to_token_failures() {
    let sys = System::new();
    sys.init_logger();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let new_parent_token_id: u64 = 8;

    mint_parent_and_child_with_acceptance(
        &rmrk_child,
        &rmrk_parent,
        child_token_id,
        parent_token_id,
    );

    // mint `new_parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], new_parent_token_id, None);

    // must fail since USERS[1] is not root owner
    rmrk_child.transfer_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        child_token_id,
        new_parent_token_id,
        Some("RMRK: Wrong owner"),
    );

    // must fail since token does not exist
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id + 1,
        new_parent_token_id,
        Some("RMRK: Token does not exist"),
    );

    // must fail since destination token does not exist
    rmrk_child.transfer_to_nft(
        USERS[0],
        PARENT_NFT_CONTRACT,
        child_token_id,
        new_parent_token_id + 100,
        Some("Error in message [RMRKAction::RootOwner]"),
    );
}
