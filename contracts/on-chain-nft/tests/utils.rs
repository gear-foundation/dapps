use gear_lib_old::non_fungible_token::{state::*, token::*};
use gstd::{prelude::*, ActorId};
use gtest::{Program, RunResult, System};
use on_chain_nft_io::*;
use on_chain_nft_state::WASM_BINARY;

const USERS: &[u64] = &[3, 4, 5];

pub fn init_nft(sys: &System) {
    sys.init_logger();
    let nft = Program::current_opt(sys);

    let mut layers = vec![];
    let first_layer = vec![
        String::from(
        "PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpsaW1lO3N0cm9rZTpwdXJwbGU7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+",
        ),
        String::from(
            "PHN2ZyBoZWlnaHQ9JzIxMCcgd2lkdGg9JzUwMCc+PHBvbHlnb24gcG9pbnRzPScxMDAsMTAgNDAsMTk4IDE5MCw3OCAxMCw3OCAxNjAsMTk4JyBzdHlsZT0nZmlsbDpibHVlO3N0cm9rZTpyZWQ7c3Ryb2tlLXdpZHRoOjU7ZmlsbC1ydWxlOm5vbnplcm87Jy8+PC9zdmc+",
        )
    ];
    let second_layer = vec![
        String::from(
            "PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0ncmVkJz5PbiBDaGFpbiBORlQ8L3RleHQ+PC9zdmc+"
        ),
        String::from(
            "PHN2ZyBoZWlnaHQ9JzMwJyB3aWR0aD0nMjAwJz48dGV4dCB4PScwJyB5PScxNScgZmlsbD0nZ3JlZW4nPk9uIENoYWluIE5GVDwvdGV4dD48L3N2Zz4="
        )
    ];
    layers.push((0, first_layer));
    layers.push((1, second_layer));
    let res = nft.send(
        USERS[0],
        InitOnChainNFT {
            name: String::from("OnChainToken"),
            symbol: String::from("OCT"),
            base_uri: String::from(""),
            royalties: None,
            base_image: String::from("<svg height='100' width='100'><circle cx='50' cy='50' r='40' stroke='black' stroke-width='3' fill='red' /></svg>"),
            layers,
        },
    );

    assert!(!res.main_failed());
}

pub fn mint(nft: &Program<'_>, member: u64, description: Vec<ItemId>) -> RunResult {
    nft.send(
        member,
        OnChainNFTAction::Mint {
            token_metadata: TokenMetadata {
                name: "CryptoKitty".to_string(),
                description: "Description".to_string(),
                media: "http://".to_string(),
                reference: "http://".to_string(),
            },
            description,
        },
    )
}

pub fn burn(nft: &Program<'_>, member: u64, token_id: u64) -> RunResult {
    nft.send(
        member,
        OnChainNFTAction::Burn {
            token_id: token_id.into(),
        },
    )
}

pub fn transfer(nft: &Program<'_>, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        OnChainNFTAction::Transfer {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn approve(nft: &Program<'_>, from: u64, to: u64, token_id: u64) -> RunResult {
    nft.send(
        from,
        OnChainNFTAction::Approve {
            to: to.into(),
            token_id: token_id.into(),
        },
    )
}

pub fn check_token_uri(
    nft: &Program<'_>,
    token_id: u64,
    metadata: TokenMetadata,
    content: Vec<String>,
) {
    match nft.read_state_using_wasm::<NFTQuery, _, Option<Vec<u8>>>(
        0,
        "token_uri",
        WASM_BINARY.into(),
        Some(NFTQuery::Token {
            token_id: token_id.into(),
        }),
    ) {
        Ok(token_uri) => {
            let token_uri = TokenURI::decode(&mut token_uri.unwrap().as_ref()).unwrap();

            let rec_metadata = token_uri.metadata;
            let rec_content = token_uri.content;

            // since they don't have PartialEq do it manually
            if metadata.name != rec_metadata.name {
                std::panic!("Metadata name is different");
            }
            if metadata.description != rec_metadata.description {
                std::panic!("Metadata description is different");
            }
            if metadata.media != rec_metadata.media {
                std::panic!("Metadata media is different");
            }
            if metadata.reference != rec_metadata.reference {
                std::panic!("Metadata reference is different");
            }
            if content != rec_content {
                std::panic!("Content is different");
            }
        }
        _ => unreachable!(
            "Unreachable metastate reply for the OnChainNFTQuery::TokenURI payload has occured"
        ),
    }
}

pub fn check_token_from_state(nft: &Program<'_>, owner_id: u64, token_id: u64) {
    match nft.read_state_using_wasm::<NFTQuery, _, Option<Vec<u8>>>(
        0,
        "base",
        WASM_BINARY.into(),
        Some(NFTQuery::Token {
            token_id: token_id.into(),
        }),
    ) {
        Ok(reply) => {
            let NFTQueryReply::Token { token } =
                NFTQueryReply::decode(&mut reply.unwrap().as_ref()).unwrap()
            else {
                std::panic!()
            };

            let true_token_id = token.id;
            let true_owner_id = token.owner_id;

            if !(ActorId::from(owner_id) == true_owner_id
                && TokenId::from(token_id) == true_token_id)
            {
                std::panic!(
                    "There is no such token with token_id ({token_id:?}) for the owner ({owner_id:?})"
                )
            }
        }
        _ => {
            unreachable!("Unreachable metastate reply for the NFTQuery::Token payload has occured")
        }
    }
}
