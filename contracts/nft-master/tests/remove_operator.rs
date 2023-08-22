mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use utils::nft_master::NFTMasterMock;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let user_id = user.into();

    let nft_master_mock = Program::nft_master_mock(&system);
    let state = nft_master_mock.get_state();

    assert!(state.nfts.is_empty());
    assert_eq!(state.operators[0], utils::ADMIN.into());

    nft_master_mock.add_operator(utils::ADMIN, &user_id, false);
    nft_master_mock.remove_operator(utils::ADMIN, &user_id, false);
    let state = nft_master_mock.get_state();

    assert!(!state.operators.contains(&user_id));
}

#[test]
fn fail_operator_does_not_exist() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let user_id = user.into();

    let fake_user = utils::USERS[1];
    let fake_user_id = fake_user.into();

    let nft_master_mock = Program::nft_master_mock(&system);
    let state = nft_master_mock.get_state();

    assert!(state.nfts.is_empty());
    assert_eq!(state.operators[0], utils::ADMIN.into());

    nft_master_mock.add_operator(utils::ADMIN, &user_id, false);
    nft_master_mock.remove_operator(utils::ADMIN, &fake_user_id, true);
}

#[test]
fn fail_only_operator_can_remove_operators() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let user_id = user.into();

    let fake_user = utils::USERS[1];

    let nft_master_mock = Program::nft_master_mock(&system);
    let state = nft_master_mock.get_state();

    assert!(state.nfts.is_empty());
    assert_eq!(state.operators[0], utils::ADMIN.into());

    nft_master_mock.add_operator(utils::ADMIN, &user_id, false);
    nft_master_mock.remove_operator(fake_user, &user_id, true);
}
