use gear_lib_old::non_fungible_token::token::*;
use gstd::ActorId;
use gtest::{Program, RunResult, System};
use rentable_nft_io::*;

const USERS: &[u64] = &[3, 4, 5];

pub fn init_nft(sys: &System) {
    sys.init_logger();
    let nft = Program::current_opt(sys);

    let res = nft.send(
        USERS[0],
        InitNft {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            base_uri: String::from(""),
            royalties: None,
        },
    );

    assert!(!res.main_failed());
}

pub fn mint(nft: &Program<'_>, transaction_id: u64, member: u64) -> RunResult {
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

pub fn is_approved_to(nft: &Program<'_>, from: u64, token_id: u64, to: u64) -> RunResult {
    nft.send(
        from,
        NFTAction::IsApproved {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn approve(nft: &Program<'_>, transaction_id: u64, from: u64, to: u64, token_id: u64) -> RunResult {
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
    nft: &Program<'_>,
    from: u64,
    address: ActorId,
    token_id: TokenId,
    expires: u64,
    transaction_id: u64,
) -> RunResult {
    let payload = NFTAction::SetUser {
        token_id,
        address,
        expires,
        transaction_id,
    };
    nft.send(from, payload)
}

pub fn user_of(nft: &Program<'_>, from: u64, token_id: TokenId) -> RunResult {
    let payload = NFTAction::UserOf { token_id };
    nft.send(from, payload)
}

pub fn user_expires(nft: &Program<'_>, from: u64, token_id: TokenId) -> RunResult {
    let payload = NFTAction::UserExpires { token_id };
    nft.send(from, payload)
}
