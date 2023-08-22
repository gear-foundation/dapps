use auction_io::auction::{Action, CreateConfig, Duration, Error, Event};
use gear_lib::non_fungible_token::{
    io::NFTApproval,
    token::{TokenId, TokenMetadata},
};
use gstd::Encode;
use gtest::{Log, Program, RunResult, System};
use nft_io::{Constraints, InitNFT, NFTEvent};

pub const USERS: &[u64] = &[4, 5, 6];
#[allow(dead_code)]
pub const DURATION: u32 = 169 * 60 * 60;

pub fn init(sys: &System) -> Program {
    USERS
        .iter()
        .for_each(|user| sys.mint_to(*user, 1_000_000_000));
    let owner_user = USERS[0];

    sys.init_logger();

    let auction_program = Program::current(sys);

    auction_program.send(owner_user, ());

    init_nft(sys, owner_user);
    let result = update_auction(&auction_program, owner_user, 2, 1_000_000_000);
    println!(
        "update_auction result = {:?}",
        result.decoded_log::<Result<Event, Error>>()
    );

    assert!(result.contains(&(
        owner_user,
        Ok::<auction_io::auction::Event, Error>(Event::AuctionStarted {
            token_owner: owner_user.into(),
            price: 1_000_000_000,
            token_id: 0.into(),
        })
        .encode()
    )));

    auction_program
}

pub fn init_nft(sys: &System, owner: u64) {
    let nft_program = Program::from_file(sys, "target/wasm32-unknown-unknown/debug/nft.opt.wasm");

    let res = nft_program.send(
        owner,
        InitNFT {
            royalties: None,
            collection: Default::default(),
            constraints: Constraints {
                authorized_minters: vec![owner.into()],
                ..Default::default()
            },
        },
    );

    assert!(!res.main_failed());

    let res = nft_program.send(
        owner,
        nft_io::NFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "MyNFT".to_string(),
                description: "NFTForAuction".to_string(),
                media: "".to_string(),
                reference: "".to_string(),
            },
            transaction_id: 0u64,
        },
    );

    assert!(!res.main_failed());

    let res = nft_owner(&nft_program, owner, 0.into());
    let log = Log::builder().dest(owner).payload(nft_io::NFTEvent::Owner {
        owner: owner.into(),
        token_id: 0.into(),
    });
    assert!(res.contains(&log));

    let res = nft_program.send(
        owner,
        nft_io::NFTAction::Approve {
            to: 1.into(),
            token_id: 0.into(),
            transaction_id: 1u64,
        },
    );
    let approval = NFTApproval {
        owner: owner.into(),
        approved_account: 1.into(),
        token_id: 0.into(),
    };
    let log = Log::builder()
        .dest(owner)
        .payload(nft_io::NFTEvent::Approval(approval));
    assert!(!res.main_failed());
    println!("approve result = {:?}", res.decoded_log::<NFTEvent>());
    assert!(res.contains(&log));
}

pub fn update_auction(
    auction: &Program,
    from: u64,
    nft_contract_id: u64,
    starting_price: u128,
) -> RunResult {
    auction.send(
        from,
        Action::Create(CreateConfig {
            nft_contract_actor_id: nft_contract_id.into(),
            starting_price,
            discount_rate: 1_000,
            token_id: 0.into(),
            duration: Duration {
                hours: 168,
                minutes: 0,
                seconds: 0,
            },
        }),
    )
}

#[allow(dead_code)]
pub fn nft_owner(nft_program: &Program, from: u64, token_id: TokenId) -> RunResult {
    nft_program.send(from, nft_io::NFTAction::Owner { token_id })
}
