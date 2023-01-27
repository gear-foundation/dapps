use auction_io::{auction::*, io::Action};
use dutch_auction::state::{State, StateReply};
use gtest::System;

mod routines;
use routines::*;

#[test]
fn is_not_active_after_time_is_over() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);

    if let Ok(StateReply::Info(info)) = auction.meta_state(State::Info) {
        assert!(!matches!(info.status, Status::IsRunning))
    }
}

#[test]
fn is_active_before_deal() {
    let sys = System::new();

    let auction = init(&sys);

    if let Ok(StateReply::Info(info)) = auction.meta_state(State::Info) {
        assert!(matches!(info.status, Status::IsRunning));
    } else {
        panic!("Can't get state");
    }
}

#[test]
fn is_not_active_after_deal() {
    let sys = System::new();

    let auction = init(&sys);
    auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    if let Ok(StateReply::Info(info)) = auction.meta_state(State::Info) {
        assert!(matches!(
            info.status,
            Status::Purchased {
                price: 1_000_000_000
            }
        ));
    } else {
        panic!("Can't get state");
    }
}
