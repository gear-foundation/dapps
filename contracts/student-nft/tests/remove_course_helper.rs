mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use utils::student_nft::StudentNft;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let helper = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "", "", false);

    let state = student_nft.get_state();
    assert!(state.courses[0].1.owner_helpers.is_empty());

    student_nft.add_course_helper(user, 1, helper, false);
    student_nft.remove_course_helper(user, 1, helper, false);

    let state = student_nft.get_state();
    assert!(state.courses[0].1.owner_helpers.is_empty());
}

#[test]
fn fail_helper_not_found() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let helper = utils::USERS[1];
    let fake = utils::USERS[2];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "", "", false);

    let state = student_nft.get_state();
    assert!(state.courses[0].1.owner_helpers.is_empty());

    student_nft.add_course_helper(user, 1, helper, false);
    student_nft.remove_course_helper(user, 1, fake, true);
}

#[test]
fn fail_only_owner_can_remove_helper() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let helper = utils::USERS[1];
    let fake = utils::USERS[2];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "", "", false);

    let state = student_nft.get_state();
    assert!(state.courses[0].1.owner_helpers.is_empty());

    student_nft.add_course_helper(user, 1, helper, false);
    student_nft.remove_course_helper(fake, 1, helper, true);
}
