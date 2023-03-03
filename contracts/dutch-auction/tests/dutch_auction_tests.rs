use auction_io::auction::{Action, Error, Event};
use gstd::{ActorId, Encode};
use gtest::{Log, System};
mod routines;
use routines::*;

#[test]
fn buy() {
    let sys = System::new();

    let auction = init(&sys);

    let nft_program = sys.get_program(2);
    let token_id: u64 = 0;
    let result = auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    println!("{:?}", result.decoded_log::<Result<Event, Error>>());
    assert!(result.contains(&(
        USERS[1],
        Ok::<Event, Error>(Event::Bought {
            price: 1_000_000_000,
        })
        .encode()
    )));

    let res = nft_owner(&nft_program, USERS[0], token_id.into());
    let new_owner = ActorId::from(USERS[1]);

    let log = Log::builder()
        .dest(USERS[0])
        .payload(nft_io::NFTEvent::Owner {
            owner: new_owner,
            token_id: token_id.into(),
        });

    assert!(res.contains(&log));

    sys.claim_value_from_mailbox(USERS[0]);

    auction.send_with_value(USERS[0], Action::Reward, 0);
    sys.claim_value_from_mailbox(USERS[0]);

    let buyer_balance = sys.balance_of(USERS[1]);
    let seller_balance = sys.balance_of(USERS[0]);

    assert_eq!(buyer_balance, 0);
    assert_eq!(seller_balance, 2_000_000_000);
}

#[test]
fn buy_later_with_lower_price() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(100_000);
    let result = auction.send_with_value(USERS[1], Action::Buy, 900_000_000);

    assert!(result.contains(&(
        USERS[1],
        Ok::<Event, Error>(Event::Bought { price: 900_000_000 }).encode()
    )));

    sys.claim_value_from_mailbox(USERS[0]);

    auction.send_with_value(USERS[0], Action::Reward, 0);
    sys.claim_value_from_mailbox(USERS[0]);

    let buyer_balance = sys.balance_of(USERS[1]);
    let seller_balance = sys.balance_of(USERS[0]);

    assert_eq!(buyer_balance, 100_000_000);
    assert_eq!(seller_balance, 1_900_000_000);
}

#[test]
fn buy_two_times() {
    let sys = System::new();

    let auction = init(&sys);
    auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);
    let result = auction.send_with_value(USERS[2], Action::Buy, 1_000_000_000);
    println!("{:?}", result.decoded_log::<Result<Event, Error>>());
    assert!(result.contains(&(
        USERS[2],
        Err::<Event, Error>(Error::AlreadyStopped).encode()
    )));
}

#[test]
fn buy_too_late() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    let result = auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    assert!(result.contains(&(
        USERS[1],
        Err::<Event, Error>(Error::AlreadyStopped).encode()
    )));
}

#[test]
fn buy_with_less_money() {
    let sys = System::new();

    let auction = init(&sys);
    let result = auction.send_with_value(USERS[1], Action::Buy, 999_000_000);

    assert!(result.contains(&(
        USERS[1],
        Err::<Event, Error>(Error::InsufficientMoney).encode()
    )));
}

#[test]
fn create_auction_twice_in_a_row() {
    let sys = System::new();

    let auction = init(&sys);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);

    assert!(result.contains(&(
        USERS[1],
        Err::<Event, Error>(Error::AlreadyRunning).encode()
    )));
}

#[test]
fn create_auction_twice_after_time_and_stop() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    let owner_user = USERS[0];
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);
    println!("{:?}", result.decoded_log::<Result<Event, Error>>());

    let result = auction.send(owner_user, Action::ForceStop);

    assert!(result.contains(&(
        owner_user,
        Ok::<Event, Error>(Event::AuctionStopped {
            token_owner: owner_user.into(),
            token_id: 0.into(),
        })
        .encode()
    )));
}

#[test]
fn create_auction_with_low_price() {
    let sys = System::new();

    let auction = init(&sys);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, (DURATION / 1000 - 1).into());

    assert!(result.contains(&(
        USERS[1],
        Err::<Event, Error>(Error::AlreadyRunning).encode()
    )));
}

#[test]
fn create_and_stop() {
    let sys = System::new();
    let owner_user = USERS[0];
    let auction = init(&sys);

    let result = auction.send(owner_user, Action::ForceStop);

    assert!(result.contains(&(
        owner_user,
        Ok::<Event, Error>(Event::AuctionStopped {
            token_owner: owner_user.into(),
            token_id: 0.into(),
        })
        .encode()
    )));
}
