mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use utils::student_nft::StudentNft;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);

    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);

    let state = student_nft.get_state();
    assert!(state.nfts[0].1.actual_courses.is_empty());

    student_nft.start_course(user, 1, false);

    let state = student_nft.get_state();
    assert!(!state.nfts[0].1.actual_courses.is_empty());
}

#[test]
fn fail_course_already_started() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);

    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);
    student_nft.start_course(user, 1, false);
    student_nft.start_course(user, 1, true);
}

#[test]
fn fail_provided_course_not_exist() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);

    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);
    student_nft.start_course(user, 1337, true);
}

#[test]
fn fail_user_dont_have_nft() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);

    student_nft.create_course(teacher, "1", "2", false);
    student_nft.start_course(user, 1, true);
}
