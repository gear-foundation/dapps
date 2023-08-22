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
    student_nft.finish_course(teacher, 1, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "", None, 3, false);

    let state = student_nft.get_state();
    assert!(!state.nfts[0].1.actual_courses[0].is_completed);

    student_nft.complete_course(user, 1, false);

    let state = student_nft.get_state();
    assert!(state.nfts[0].1.actual_courses[0].is_completed);
}

#[test]
fn fail_course_already_completed() {
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

    let lesson_1 = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 3,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.add_lesson(teacher, 1, &lesson_1, false);
    student_nft.finish_course(teacher, 1, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "", None, 3, false);
    student_nft.approve_hw(teacher, 1, 1, 1, "", None, 5, false);
    student_nft.complete_course(user, 1, false);
    student_nft.complete_course(user, 1, true);
}

#[test]
fn fail_required_hws_are_not_completed() {
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

    let lesson_1 = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        // This field is not required, because will be filled automatically on-chain
        // but required for test assert
        emote_id: 3,
        is_provide_hw: true,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.add_lesson(teacher, 1, &lesson_1, false);
    student_nft.finish_course(teacher, 1, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "", None, 3, false);
    student_nft.complete_course(user, 1, true);
}

#[test]
fn fail_course_is_not_finished_by_owner() {
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
    student_nft.approve_hw(teacher, 1, 1, 0, "", None, 3, false);
    student_nft.complete_course(user, 1, true);
}

#[test]
fn fail_course_does_not_exist() {
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
    student_nft.finish_course(teacher, 1, false);
    student_nft.approve_hw(teacher, 1, 1, 0, "", None, 3, false);
    student_nft.complete_course(user, 1337, true);
}
