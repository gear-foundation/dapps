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
    let state = nft_master_mock.get_state();

    assert_eq!(state.nfts[0], (nft_id, "1".to_owned()));

    nft_master_mock.add_nft_contract(utils::ADMIN, &nft_id, "2", false);
    let state = nft_master_mock.get_state();

    assert_eq!(state.nfts[0], (nft_id, "2".to_owned()));
}

#[test]
fn fail_only_operator_can_add_nft() {
    let system = System::new();
    system.init_logger();

    let fake_user = utils::USERS[0];

    let nft = utils::NFTS[0];
    let nft_id = nft.into();

    let nft_master_mock = Program::nft_master_mock(&system);
    nft_master_mock.add_nft_contract(fake_user, &nft_id, "1", true);
}
