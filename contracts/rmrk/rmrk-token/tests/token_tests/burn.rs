use crate::utils::*;
use gtest::{Program, System};

#[test]
fn burn_simple() {
    let sys = System::new();
    let rmrk = Program::rmrk(&sys, None);

    let token_id: u64 = 5;

    // mint
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    // burn
    rmrk.burn(USERS[0], token_id, None);

    // // check that token does not exist
    // rmrk.check_rmrk_owner(token_id, None, ZERO_ID);
}

#[test]
fn burn_simple_failures() {
    let sys = System::new();
    sys.init_logger();
    let rmrk = Program::rmrk(&sys, None);

    let token_id: u64 = 5;

    // mint
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    // must fail since caller is not owner and not approved
    rmrk.burn(USERS[3], token_id, Some("RMRK: Wrong owner"));

    // must fail since token does not exist
    rmrk.burn(USERS[3], token_id + 1, Some("RMRK: Token does not exist"));
}

#[test]
fn burn_nested_token() {
    let sys = System::new();

    sys.init_logger();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_accepted_token_id: u64 = 8;
    let child_pending_token_id: u64 = 9;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[1], parent_token_id, None);

    // mint child_token_id to parent_token_id
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_pending_token_id,
        None,
    );

    // mint child_token_id to parent_token_id
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_accepted_token_id,
        None,
    );

    // accept one child
    rmrk_parent.accept_child(
        USERS[1],
        parent_token_id,
        CHILD_NFT_CONTRACT,
        child_accepted_token_id,
        None,
    );

    rmrk_child.burn(USERS[1], child_pending_token_id, None);
    rmrk_child.burn(USERS[1], child_accepted_token_id, None);

    // // check that parent contract has no pending children
    // rmrk_parent.check_pending_children(parent_token_id, BTreeSet::new());

    // // check that parent contract has no accepted children
    // rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());
}

#[test]
fn burn_nested_token_failures() {
    let sys = System::new();

    sys.init_logger();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 9;
    let parent_token_id: u64 = 10;

    // mint `parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[1], parent_token_id, None);

    // mint child_token_id to parent_token_id
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_token_id,
        None,
    );

    // must fail since caller is not root owner of the nested child token
    rmrk_child.burn(USERS[3], child_token_id, Some("RMRK: Wrong owner"));
}

// ownership chain is now USERS[0] > parent_token_id > child_token_id > grand_token_id
// in that test child_token_id is burning
// rmrk_child contract must also burn grand_token_id and must be removed from parent_token_id
#[test]
fn recursive_burn_nested_token() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);
    let rmrk_grand = Program::rmrk(&sys, None);
    let child_token_id: u64 = 9;
    let parent_token_id: u64 = 10;
    let grand_token_id: u64 = 11;

    // ownership chain is  USERS[0] > parent_token_id > child_token_id > grand_token_id
    rmrk_chain(
        &rmrk_grand,
        &rmrk_child,
        &rmrk_parent,
        grand_token_id,
        child_token_id,
        parent_token_id,
    );

    // // check accepted children of parent_token_id
    // let mut accepted_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // accepted_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    // rmrk_parent.check_accepted_children(parent_token_id, accepted_children);

    // // check accepted children of child_token_id
    // let mut accepted_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // accepted_children.insert((3.into(), grand_token_id.into()));
    // rmrk_child.check_accepted_children(child_token_id, accepted_children);

    // // burn child
    // rmrk_child.burn(USERS[0], child_token_id, None);

    // // check that parent_token_id has no accepted children
    // rmrk_parent.check_accepted_children(parent_token_id, BTreeSet::new());

    // // check that child_token_id does not exist
    // rmrk_child.check_rmrk_owner(child_token_id, None, ZERO_ID);

    // // check that grand_token_id does not exist
    // rmrk_grand.check_rmrk_owner(grand_token_id, None, ZERO_ID);
}

// ownership chain is now USERS[0] > parent_token_id > child_token_id > grand_token_id
// in that test parent_token_id is burning
// rmrk_child contract must also burn child_token_id and grand_token_id
#[test]
fn recursive_burn_parent_token() {
    let sys = System::new();
    sys.init_logger();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);
    let rmrk_grand = Program::rmrk(&sys, None);
    let child_token_id: u64 = 9;
    let parent_token_id: u64 = 10;
    let grand_token_id: u64 = 11;

    // ownership chain is  USERS[0] > parent_token_id > child_token_id > grand_token_id
    rmrk_chain(
        &rmrk_grand,
        &rmrk_child,
        &rmrk_parent,
        grand_token_id,
        child_token_id,
        parent_token_id,
    );

    // burn parent_token_id
    rmrk_parent.burn(USERS[0], parent_token_id, None);

    // // check that child_token_id does not exist
    // rmrk_child.check_rmrk_owner(child_token_id, None, ZERO_ID);

    // // check that grand_token_id does not exist
    // rmrk_grand.check_rmrk_owner(grand_token_id, None, ZERO_ID);
}
