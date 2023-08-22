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

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "2", false);

    let state = student_nft.get_state();
    assert!(state.courses[0].1.lessons.is_empty());

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

    student_nft.add_lesson(user, 1, &lesson, false);

    let state = student_nft.get_state();

    assert_eq!(lesson, state.courses[0].1.lessons[0]);
}

#[test]
fn fail_only_owner_can_add_lessons() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];
    let fake_user = utils::USERS[1];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "2", false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        emote_id: 0,
        is_provide_hw: false,
    };

    student_nft.add_lesson(fake_user, 1, &lesson, true);
}

#[test]
fn fail_provided_course_not_exist() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "2", false);

    let lesson = Lesson {
        name: "".to_owned(),
        description: "".to_owned(),
        media_url: "".to_owned(),
        thumb_url: "".to_owned(),
        reviews: Vec::new(),
        emote_id: 0,
        is_provide_hw: false,
    };

    student_nft.add_lesson(user, 1337, &lesson, true);
}
