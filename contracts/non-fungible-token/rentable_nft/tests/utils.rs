use gear_lib::non_fungible_token::token::*;
use gstd::ActorId;
use gtest::{Program, RunResult, System};
use io::{InitNFT, NFTAction};

const USERS: &[u64] = &[3, 4, 5];

pub fn init_nft(sys: &System) {
    sys.init_logger();
    let nft = Program::current(sys);

    let res = nft.send(
        USERS[0],
        InitNFT {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            base_uri: String::from(""),
            royalties: None,
        },
    );

    assert!(res.log().is_empty());
}

pub fn mint(nft: &Program, transaction_id: u64, member: u64) -> RunResult {
    nft.send(
        member,
        NFTAction::Mint {
            transaction_id,
            token_metadata: TokenMetadata {
                name: "CryptoKitty".to_string(),
                description: "Description".to_string(),
                media: "http://".to_string(),
                reference: "http://".to_string(),
            },
        },
    )
}

pub fn approve(nft: &Program, transaction_id: u64, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        NFTAction::Approve {
            transaction_id,
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn set_user(
    nft: &Program,
    from: u64,
    address: ActorId,
    token_id: TokenId,
    expires: u64,
) -> RunResult {
    let payload = io::NFTAction::SetUser {
        token_id,
        address,
        duration_in_secs: expires,
        transaction_id: None,
    };
    nft.send(from, payload)
}

pub fn user_of(nft: &Program, from: u64, token_id: TokenId) -> RunResult {
    let payload = io::NFTAction::UserOf { token_id };
    nft.send(from, payload)
}

pub fn user_expires(nft: &Program, from: u64, token_id: TokenId) -> RunResult {
    let payload = io::NFTAction::UserExpires { token_id };
    nft.send(from, payload)
}
