use crate::utils::*;
use gtest::{Program, System};

#[test]
fn transfer_simple() {
    let sys = System::new();
    let rmrk = Program::rmrk(&sys, None);
    let token_id: u64 = 9;

    // mint token
    rmrk.mint_to_root_owner(USERS[0], USERS[0], token_id, None);

    // transfer token
    rmrk.transfer(USERS[0], USERS[3], token_id, None);

    // // check that RMRK owner
    // rmrk.check_rmrk_owner(token_id, None, USERS[3]);

    // // check the balance of previous owner
    // rmrk.check_balance(USERS[0], 0);

    // // check the balance of new owner
    // rmrk.check_balance(USERS[3], 1);
}

#[test]
fn transfer_parent_with_child() {
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

    rmrk_parent.transfer(USERS[0], USERS[3], parent_token_id, None);

    // check root_owner of child_token_id
    rmrk_child.check_root_owner(child_token_id, USERS[3]);

    // check root_owner of grand_token_id
    rmrk_grand.check_root_owner(grand_token_id, USERS[3]);
}
