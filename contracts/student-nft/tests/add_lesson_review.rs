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
        is_provide_hw: false,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);

    let state = student_nft.get_state();
    assert!(state.courses[0].1.lessons[0].reviews.is_empty());

    student_nft.add_lesson_review(user, 1, 0, "1337", false);

    let state = student_nft.get_state();
    assert!(!state.courses[0].1.lessons[0].reviews.is_empty());
    assert_eq!(
        state.courses[0].1.lessons[0].reviews[0].1,
        "1337".to_owned()
    );
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
        is_provide_hw: false,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.add_lesson_review(user, 1, 1337, "1337", true);
}

#[test]
fn fail_review_is_empty() {
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
    student_nft.add_lesson_review(user, 1, 0, "", true);
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
        is_provide_hw: false,
    };

    student_nft.add_lesson(teacher, 1, &lesson, false);
    student_nft.add_lesson_review(user, 1337, 0, "1337", true);
}
