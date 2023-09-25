use auto_changed_nft_io::*;
use gear_lib_old::non_fungible_token::token::*;
use gstd::ActorId;
use gtest::{Program, RunResult, System};

const USERS: &[u64] = &[3, 4, 5];

pub fn init_nft(sys: &System) {
    sys.init_logger();
    let nft = Program::current_opt(sys);

    let collection = Collection {
        name: String::from("MyToken"),
        description: String::from("My token"),
    };

    let init_nft = InitNFT {
        collection,
        royalties: None,
        config: Config {
            max_mint_count: Some(100),
            authorized_minters: vec![USERS[0].into()],
        },
    };

    let res = nft.send(USERS[0], init_nft);

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

pub fn add_minter(
    nft: &Program<'_>,
    transaction_id: u64,
    minter_id: ActorId,
    member: u64,
) -> RunResult {
    nft.send(
        member,
        NFTAction::AddMinter {
            transaction_id,
            minter_id,
        },
    )
}

pub fn add_url(nft: &Program<'_>, token_id: TokenId, url: &str, member: u64) -> RunResult {
    nft.send(
        member,
        NFTAction::AddMedia {
            token_id,
            media: url.to_string(),
        },
    )
}

pub fn start_auto_changing(
    nft: &Program<'_>,
    token_ids: Vec<TokenId>,
    updates_count: u32,
    update_period: u32,
    member: u64,
) -> RunResult {
    nft.send(
        member,
        NFTAction::StartAutoChanging {
            updates_count,
            update_period,
            token_ids,
        },
    )
}

pub fn burn(nft: &Program<'_>, transaction_id: u64, member: u64, token_id: u64) -> RunResult {
    nft.send(
        member,
        NFTAction::Burn {
            transaction_id,
            token_id: token_id.into(),
        },
    )
}

pub fn transfer(
    nft: &Program<'_>,
    transaction_id: u64,
    from: u64,
    to: u64,
    token_id: u64,
) -> RunResult {
    nft.send(
        from,
        NFTAction::Transfer {
            transaction_id,
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn owner_of(nft: &Program<'_>, from: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        NFTAction::Owner {
            token_id: token_id.into(),
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

pub fn approve(
    nft: &Program<'_>,
    transaction_id: u64,
    from: u64,
    to: u64,
    token_id: u64,
) -> RunResult {
    nft.send(
        from,
        NFTAction::Approve {
            transaction_id,
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn delegated_approve(
    nft: &Program<'_>,
    transaction_id: u64,
    from: u64,
    message: DelegatedApproveMessage,
    signature: [u8; 64],
) -> RunResult {
    let action = NFTAction::DelegatedApprove {
        transaction_id,
        message,
        signature,
    };
    nft.send(from, action)
}

pub fn mint_to_actor(nft: &Program<'_>, transaction_id: u64, member: [u8; 32]) -> RunResult {
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

pub fn current_media(nft: &Program<'_>, token_id: TokenId) -> String {
    let state: State = nft.read_state().unwrap();

    state
        .tokens
        .into_iter()
        .find_map(|(id, meta)| (token_id == id).then_some(meta))
        .unwrap()
        .media_url
}
