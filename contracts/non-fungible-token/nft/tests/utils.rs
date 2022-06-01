use gear_lib::non_fungible_token::token::*;
use gtest::{Program, RunResult, System};
use nft_io::*;
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

pub fn mint(nft: &Program, member: u64) -> RunResult {
    nft.send(
        member,
        NFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "CryptoKitty".to_string(),
                description: "Description".to_string(),
                media: "http://".to_string(),
                reference: "http://".to_string(),
            },
        },
    )
}

pub fn burn(nft: &Program, member: u64, token_id: u64) -> RunResult {
    nft.send(
        member,
        NFTAction::Burn {
            token_id: token_id.into(),
        },
    )
}

pub fn transfer(nft: &Program, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        NFTAction::Transfer {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn approve(nft: &Program, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        NFTAction::Approve {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}
