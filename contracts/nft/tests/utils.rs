use gstd::ActorId;
use gtest::{Program, RunResult, System};
use nft_io::*;

const USERS: &[u64] = &[3, 4, 5];

pub fn init_nft(sys: &System) {
    sys.init_logger();
    let nft = Program::current_opt(sys);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let init_nft = InitNft {
        collection,
        config: Config {
            max_mint_count: Some(100),
        },
    };

    let res = nft.send(USERS[0], init_nft);

    assert!(!res.main_failed());
}

pub fn mint(nft: &Program<'_>, member: u64, to: ActorId) -> RunResult {
    nft.send(
        member,
        NftAction::Mint {
            to,
            token_metadata: TokenMetadata {
                name: "CryptoKitty".to_string(),
                description: "Description".to_string(),
                media: "http://".to_string(),
                reference: "http://".to_string(),
            },
        },
    )
}

pub fn burn(nft: &Program<'_>, member: u64, token_id: u64) -> RunResult {
    nft.send(
        member,
        NftAction::Burn {
            token_id: token_id.into(),
        },
    )
}

pub fn transfer(nft: &Program<'_>, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        NftAction::Transfer {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn owner_of(nft: &Program<'_>, from: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        NftAction::GetOwner {
            token_id: token_id.into(),
        },
    )
}

pub fn is_approved_to(nft: &Program<'_>, from: u64, token_id: u64, to: u64) -> RunResult {
    nft.send(
        from,
        NftAction::CheckIfApproved {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn approve(nft: &Program<'_>, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        NftAction::Approve {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn get_state(nft: &Program<'_>) -> Option<State> {
    let reply = nft
        .read_state(StateQuery::All)
        .expect("Unexpected invalid reply.");
    if let StateReply::All(state) = reply {
        Some(state)
    } else {
        None
    }
}
