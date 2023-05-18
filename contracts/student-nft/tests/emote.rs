mod utils;

use gstd::prelude::*;
use gtest::{Program, System};
use student_nft_io::EmoteAction;
use utils::student_nft::StudentNft;

#[test]
fn success() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "2", false);

    let state = student_nft.get_state();
    assert!(state.emotes[0].1.upvotes.is_empty());
    assert!(state.emotes[0].1.reactions.is_empty());

    // Toggle first time
    student_nft.emote(user, 1, EmoteAction::Upvote, false);

    let state = student_nft.get_state();
    assert!(!state.emotes[0].1.upvotes.is_empty());

    // Toggle second time: remove
    student_nft.emote(user, 1, EmoteAction::Upvote, false);

    let state = student_nft.get_state();
    assert!(state.emotes[0].1.upvotes.is_empty());

    // Toggle third time with emoji
    student_nft.emote(
        user,
        1,
        EmoteAction::Reaction {
            emoji: Some("🚀".to_owned()),
        },
        false,
    );

    let state = student_nft.get_state();
    assert!(!state.emotes[0].1.reactions.is_empty());

    // Toggle 4th time with emoji: remove
    student_nft.emote(user, 1, EmoteAction::Reaction { emoji: None }, false);

    let state = student_nft.get_state();
    assert!(state.emotes[0].1.reactions.is_empty());
}

#[test]
fn fail_invalid_emote_id() {
    let system = System::new();
    system.init_logger();

    let user = utils::USERS[0];

    let student_nft = Program::student_nft(&system);
    student_nft.create_course(user, "1", "2", false);
    student_nft.emote(user, 1337, EmoteAction::Upvote, true);
}
