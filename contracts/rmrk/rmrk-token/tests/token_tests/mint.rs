use crate::utils::*;
use gstd::BTreeSet;
use gtest::{Program, System};
use types::primitives::{CollectionId, TokenId};

#[test]
fn mint_to_root_owner() {
    let sys = System::new();
    let rmrk = Program::rmrk(&sys, None);
    let token_id: u64 = 100;

    // mint
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    // check rmrk owner
    rmrk.check_rmrk_owner(token_id, None, USERS[0]);

    // check balance
    rmrk.check_balance(USERS[0], 1);
}

#[test]
fn mint_to_root_owner_failures() {
    let sys = System::new();
    let rmrk = Program::rmrk(&sys, None);
    let token_id: u64 = 100;

    // mint
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    // mints already minted token
    rmrk.mint_to_root_owner(
        USERS[0],
        USERS[0],
        token_id,
        Some("RMRK: Token already exists"),
    );

    // mints to zero address
    rmrk.mint_to_root_owner(USERS[0], ZERO_ID, token_id + 1, Some("RMRK: Zero address"));
}

#[test]
fn mint_to_nft_failures() {
    let sys = System::new();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);

    let child_token_id: u64 = 1;
    let parent_token_id: u64 = 10;
    let wrong_parent_token_id: u64 = 100;

    // mint `parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], parent_token_id, None);

    // nest mint to a non-existent token
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        wrong_parent_token_id,
        child_token_id,
        Some("Error in message [RMRKAction::AddChild]"),
    );

    // mint RMRK child token to RMRK parent token
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_token_id,
        None,
    );

    // nest mint already minted token
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_token_id,
        Some("RMRK: Token already exists"),
    );

    // nest mint already minted token to a different parent
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        wrong_parent_token_id,
        child_token_id,
        Some("RMRK: Token already exists"),
    );

    // nest mint to zero address (TO DO)
    // assert!(mint_to_nft(&rmrk_child, USERS[1], ZERO_ID, 2.into(), 12.into()).main_failed());
}

#[test]
fn mint_to_nft_success() {
    let sys = System::new();
    sys.init_logger();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);
    let parent_token_id: u64 = 10;

    // mint `parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[0], parent_token_id, None);

    let mut pending_children: BTreeSet<(CollectionId, TokenId)> = BTreeSet::new();
    // mint  RMRK children
    for child_token_id in 0..10_u64 {
        rmrk_child.mint_to_nft(
            USERS[1],
            PARENT_NFT_CONTRACT,
            parent_token_id,
            child_token_id,
            None,
        );
        // check that owner is another NFT in parent token contract
        rmrk_child.check_rmrk_owner(
            child_token_id,
            Some(parent_token_id.into()),
            PARENT_NFT_CONTRACT,
        );
        // add to pending children
        pending_children.insert((CHILD_NFT_CONTRACT.into(), child_token_id.into()));
    }

    // another RMRK child contract
    let rmrk_child_2 = Program::rmrk(&sys, None);
    let rmrk_child_2_id: u64 = 3;

    for child_token_id in 0..20_u64 {
        rmrk_child_2.mint_to_nft(
            USERS[1],
            PARENT_NFT_CONTRACT,
            parent_token_id,
            child_token_id,
            None,
        );

        // check that owner is NFT in parent contract
        rmrk_child_2.check_rmrk_owner(
            child_token_id,
            Some(parent_token_id.into()),
            PARENT_NFT_CONTRACT,
        );

        //insert pending children
        pending_children.insert((rmrk_child_2_id.into(), child_token_id.into()));
    }
    // check pending children
    rmrk_parent.check_pending_children(parent_token_id, pending_children);
}

#[test]
fn mint_child_to_child() {
    let sys = System::new();

    sys.init_logger();
    let rmrk_child = Program::rmrk(&sys, None);
    let rmrk_parent = Program::rmrk(&sys, None);
    let rmrk_grand_child = Program::rmrk(&sys, None);

    let parent_token_id: u64 = 10;
    let child_token_id: u64 = 1;
    let grand_child_id: u64 = 2;

    // mint `parent_token_id`
    rmrk_parent.mint_to_root_owner(USERS[0], USERS[1], parent_token_id, None);

    // mint `child_token_id` to `parent_token_id`
    rmrk_child.mint_to_nft(
        USERS[1],
        PARENT_NFT_CONTRACT,
        parent_token_id,
        child_token_id,
        None,
    );

    // mint grand_token_id to child_token_id
    rmrk_grand_child.mint_to_nft(
        USERS[1],
        CHILD_NFT_CONTRACT,
        child_token_id,
        grand_child_id,
        None,
    );

    // root owner of grand_token_id must be USERS[0]
    rmrk_grand_child.check_root_owner(grand_child_id, USERS[1]);
}
