mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use utils::nft_master::NFTMasterMock;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let nft = utils::NFTS[0];
    let nft_id = nft.into();

    let nft_master_mock = Program::nft_master_mock(&system);
    let state = nft_master_mock.get_state();

    assert!(state.nfts.is_empty());
    assert_eq!(state.operators[0], utils::ADMIN.into());

    nft_master_mock.add_nft_contract(utils::ADMIN, &nft_id, "1", false);
    nft_master_mock.remove_nft_contract(utils::ADMIN, &nft_id, false);
    let state = nft_master_mock.get_state();

    assert!(state.nfts.is_empty());
}

#[test]
fn fail_nft_does_not_exist() {
    let system = System::new();
    system.init_logger();

    let nft = utils::NFTS[0];
    let nft_id = nft.into();

    let nft_master_mock = Program::nft_master_mock(&system);
    let state = nft_master_mock.get_state();

    assert!(state.nfts.is_empty());
    assert_eq!(state.operators[0], utils::ADMIN.into());

    nft_master_mock.add_nft_contract(utils::ADMIN, &nft_id, "1", false);
    nft_master_mock.remove_nft_contract(utils::ADMIN, &nft_id, false);
    nft_master_mock.remove_nft_contract(utils::ADMIN, &nft_id, true);
}

#[test]
fn fail_only_operator_can_remove_nft() {
    let system = System::new();
    system.init_logger();

    let nft = utils::NFTS[0];
    let nft_id = nft.into();

    let fake_user = utils::USERS[0];

    let nft_master_mock = Program::nft_master_mock(&system);
    let state = nft_master_mock.get_state();

    assert!(state.nfts.is_empty());
    assert_eq!(state.operators[0], utils::ADMIN.into());

    nft_master_mock.add_nft_contract(utils::ADMIN, &nft_id, "1", false);
    nft_master_mock.remove_nft_contract(fake_user, &nft_id, true);
}
