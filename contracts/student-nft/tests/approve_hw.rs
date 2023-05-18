mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use student_nft_io::Lesson;
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
    student_nft.start_course(user, 1, false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 2,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);

    let state = student_nft.get_state();
    assert!(state.nfts[0].1.actual_courses[0].hws.is_empty());

    student_nft.approve_hw(teacher, 1, 1, 0, "1", None, 5, false);

    let state = student_nft.get_state();
    assert!(!state.nfts[0].1.actual_courses[0].hws.is_empty());
}

#[test]
fn fail_course_does_not_provide_hw() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);
    student_nft.start_course(user, 1, false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 2,
        is_provide_hw: false,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "1", None, 5, true);
}

#[test]
fn fail_hw_already_approved() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);
    student_nft.start_course(user, 1, false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 2,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "1", None, 5, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "1", None, 5, true);
}

#[test]
fn fail_course_is_not_started_by_user() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 2,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "1", None, 5, true);
}

#[test]
fn fail_invalid_lesson_id() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);
    student_nft.start_course(user, 1, false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 2,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.approve_hw(teacher, 1, 1, 1337, "1", None, 5, true);
}

#[test]
fn fail_only_owner_can_approve_hw() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let teacher = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.mint(user, false);
    student_nft.create_course(teacher, "1", "2", false);
    student_nft.start_course(user, 1, false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 2,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.approve_hw(user, 1, 1, 0, "1", None, 5, true);
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
    student_nft.start_course(user, 1, false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 2,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.approve_hw(teacher, 1, 1337, 0, "1", None, 5, true);
}
