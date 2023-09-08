mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use utils::student_nft::StudentNft;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "1", false);

    let state = student_nft.get_state();
    assert!(!state.courses[0].1.is_finished);

    student_nft.finish_course(user, 1, false);

    let state = student_nft.get_state();
    assert!(state.courses[0].1.is_finished);
}

#[test]
fn fail_only_owner_can_finish_course() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let fake_user = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "1", false);
    student_nft.finish_course(fake_user, 1, true);
}

#[test]
fn fail_course_already_finished() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "1", false);
    student_nft.finish_course(user, 1, false);
    student_nft.finish_course(user, 1, true);
}

#[test]
fn fail_course_does_not_exist() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "1", false);
    student_nft.finish_course(user, 1337, true);
}
