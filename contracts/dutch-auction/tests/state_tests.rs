use auction_io::*;
// use codec::Encode;
use dutch_auction::{State, StateReply, Status};
use gtest::System;

mod routines;
use routines::*;

// #[test]
// fn is_not_active_after_time_is_over() {
//     let sys = System::new();
//
//     let auction = init(&sys);
//     sys.spend_blocks(DURATION);
//
//     if let StateReply::IsActive(is_active) = auction.meta_state(State::IsActive()).unwrap() {
//         assert!(!is_active);
//     } else {
//         panic!("Can't get state");
//     }
// }

#[test]
fn is_active_before_deal() {
    let sys = System::new();

    let auction = init(&sys);

    if let StateReply::Status(status) = auction.meta_state(State::Status).unwrap() {
        assert!(matches!(status, Status::IsRunning));
    } else {
        panic!("Can't get state");
    }
}

#[test]
fn is_not_active_after_deal() {
    let sys = System::new();

    let auction = init(&sys);
    auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    if let StateReply::Status(status) = auction.meta_state(State::Status).unwrap() {
        assert!(matches!(
            status,
            Status::Purchased {
                price: 1_000_000_000
            }
        ));
    } else {
        panic!("Can't get state");
    }
}
