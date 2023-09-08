mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use utils::student_nft::StudentNft;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let user_id = user.into();

    let student_nft = Program::student_nft(&system);
    let state = student_nft.get_state();

    assert!(state.nfts.is_empty());
    assert!(state.nft_owners.is_empty());
    assert_eq!(state.nft_nonce, 0);

    student_nft.mint(user, false);

    let state = student_nft.get_state();

    assert!(!state.nfts.is_empty());
    assert!(!state.nft_owners.is_empty());
    assert_eq!(state.nft_nonce, 1);
    assert_eq!(state.nft_owners[0], (user_id, 1));
}

#[test]
fn fail_user_already_has_nft() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system);
    student_nft.mint(user, false);
    student_nft.mint(user, true);
}
