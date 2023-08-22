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
    let state = student_nft.get_state();

    assert!(state.courses.is_empty());
    assert!(state.emotes.is_empty());
    assert_eq!(state.course_nonce, 0);
    assert_eq!(state.emote_nonce, 0);

    student_nft.create_course(user, "", "", false);

    let state = student_nft.get_state();

    assert!(!state.courses.is_empty());
    assert!(!state.emotes.is_empty());
    assert_eq!(state.course_nonce, 1);
    assert_eq!(state.emote_nonce, 1);
}
