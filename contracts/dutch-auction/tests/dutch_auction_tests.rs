use auction_io::*;
use codec::Encode;
use gtest::{Program, RunResult, System};

const USERS: &[u64] = &[4, 5, 6];
const DURATION: u32 = 7 * 24 * 60 * 60 * 1000;

fn init(sys: &System) -> Program {
    let owner_user = USERS[0];

    sys.init_logger();

    let auction_program = Program::current(sys);

    auction_program.send(owner_user, InitConfig {});

    init_nft(sys, owner_user);
    let result = update_auction(&auction_program, owner_user, 2, 1_000_000_000);

    assert!(result.contains(&(
        owner_user,
        Event::AuctionStarted {
            token_owner: owner_user.into(),
            price: 1_000_000_000,
            token_id: 0.into(),
        }
        .encode()
    )));

    auction_program
}

fn init_nft(sys: &System, owner: u64) {
    let nft_program = Program::from_file(sys, "./target/nft_example.wasm");

    nft_program.send(
        owner,
        nft_example_io::InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            base_uri: String::from(""),
        },
    );

    nft_program.send(owner, nft_example_io::Action::Mint);
    nft_program.send(
        owner,
        nft_example_io::Action::Approve {
            to: 1.into(),
            token_id: 0.into(),
        },
    );
}

fn update_auction(
    auction: &Program,
    owner: u64,
    nft_contract_id: u64,
    starting_price: u128,
) -> RunResult {
    auction.send(
        owner,
        Action::Create(CreateConfig {
            nft_contract_actor_id: nft_contract_id.into(),
            token_owner: owner.into(),
            starting_price,
            discount_rate: 1,
            token_id: 0.into(),
            duration: Duration {
                days: 7,
                hours: 0,
                minutes: 0,
            },
        }),
    )
}

#[test]
fn buy() {
    let sys = System::new();

    let auction = init(&sys);
    auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    let result = sys
        .get_program(2)
        .send(USERS[0], nft_example_io::Action::OwnerOf(0.into()));

    assert!(result.contains(&(
        USERS[0],
        nft_example_io::Event::OwnerOf(USERS[1].into()).encode()
    )));
}

#[test]
fn buy_later_with_lower_price() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(100_000_000);
    auction.send_with_value(USERS[1], Action::Buy, 900_000_000);

    let result = sys
        .get_program(2)
        .send(USERS[0], nft_example_io::Action::OwnerOf(0.into()));

    assert!(result.contains(&(
        USERS[0],
        nft_example_io::Event::OwnerOf(USERS[1].into()).encode()
    )));
}

#[test]
fn buy_two_times() {
    let sys = System::new();

    let auction = init(&sys);
    auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);
    let result = auction.send_with_value(USERS[2], Action::Buy, 1_000_000_000);

    assert!(result.main_failed());
}

#[test]
fn buy_too_late() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    let result = auction.send_with_value(USERS[1], Action::Buy, 1_000_000_000);

    assert!(result.main_failed());
}

#[test]
fn buy_with_less_money() {
    let sys = System::new();

    let auction = init(&sys);
    let result = auction.send_with_value(USERS[1], Action::Buy, 999_000_000);

    assert!(result.main_failed());
}

#[test]
fn create_auction_twice_in_a_row() {
    let sys = System::new();

    let auction = init(&sys);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);

    assert!(result.main_failed());
}

#[test]
fn create_auction_twice_after_time() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, 999_000_000);

    assert!(result.contains(&(
        USERS[1],
        Event::AuctionStarted {
            token_owner: USERS[1].into(),
            price: 999_000_000,
            token_id: 0.into(),
        }
        .encode()
    )));
}

#[test]
fn create_auction_with_low_price() {
    let sys = System::new();

    let auction = init(&sys);
    sys.spend_blocks(DURATION);
    init_nft(&sys, USERS[1]);
    let result = update_auction(&auction, USERS[1], 3, (DURATION - 1).into());

    assert!(result.main_failed());
}

#[test]
fn create_and_stop() {
    let sys = System::new();
    let owner_user = USERS[0];
    let auction = init(&sys);

    let result = auction.send(owner_user, Action::ForceStop);

    assert!(result.contains(&(
        owner_user,
        Event::AuctionStoped {
            token_owner: owner_user.into(),
            token_id: 0.into(),
        }
        .encode()
    )));
}
