use ft_io::{FTAction, FTEvent, InitConfig as InitFT};
use gear_lib::non_fungible_token::{
    state::{NFTQuery, NFTQueryReply},
    token::Token,
};
pub use gstd::prelude::*;
use gstd::ActorId;
pub use gtest::Program;
use gtest::System;
use nft_io::InitNFT;
pub use supply_chain_io::*;

pub mod check;
pub mod fail;

pub const FT_PROGRAM_ID: u64 = 1;
pub const NFT_PROGRAM_ID: u64 = 2;
pub const SUPPLY_CHAIN_PROGRAM_ID: u64 = 3;
pub const PRODUCER: [u64; 2] = [4, 5];
pub const DISTRIBUTOR: [u64; 2] = [6, 7];
pub const RETAILER: [u64; 2] = [8, 9];
pub const CONSUMER: [u64; 2] = [10, 11];
pub const FOREIGN_USER: u64 = 1337;
pub const ITEM_ID: [u128; 2] = [0, 1];
pub const NONEXISTEND_ITEM: u128 = 999999;
pub const ITEM_NAME: [&str; 2] = ["Banana", "Watermelon"];
pub const ITEM_DESCRIPTION: [&str; 2] = ["Tasty", "Fresh"];
pub const ITEM_PRICE_BY_PRODUCER: [u128; 2] = [1234, 4321];
pub const ITEM_PRICE_BY_DISTRIBUTOR: [u128; 2] = [12345, 54321];
pub const ITEM_PRICE_BY_RETAILER: [u128; 2] = [123456, 654321];
pub const DELIVERY_TIME: [u64; 2] = [604800000, 1209600000];
pub const ZERO_ID: ActorId = ActorId::new([0; 32]);

pub fn init_system() -> System {
    let system = System::new();
    system.init_logger();

    system
}

pub fn init_ft_program(system: &System) -> Program {
    let ft_program = Program::from_file(system, "./target/fungible_token.wasm");

    assert!(ft_program
        .send(
            FOREIGN_USER,
            InitFT {
                name: "MyToken".into(),
                symbol: "MTK".into(),
            },
        )
        .log()
        .is_empty());

    ft_program
}

pub fn init_nft_program(system: &System) -> Program {
    let nft_program = Program::from_file(system, "./target/nft.wasm");

    assert!(nft_program
        .send(
            FOREIGN_USER,
            InitNFT {
                name: "Item".into(),
                symbol: "ITM".into(),
                base_uri: Default::default(),
                royalties: Default::default(),
            },
        )
        .log()
        .is_empty());

    nft_program
}

pub fn mint(ft_program: &Program, actor: u64, amount: u128) {
    assert!(ft_program.send(actor, FTAction::Mint(amount)).contains(&(
        actor,
        FTEvent::Transfer {
            from: 0.into(),
            to: actor.into(),
            amount,
        }
        .encode()
    )));
}

pub fn check_nft_owner(nft_program: &Program, nft: u128, actor: u64) {
    let actor = actor.into();
    let nft = nft.into();
    match nft_program.meta_state(NFTQuery::Token { token_id: nft }) {
        NFTQueryReply::Token {
            token: Token { owner_id, .. },
        } => {
            if owner_id != actor {
                panic!(
                    "Owner assertion failed.\n\
                     NFT ID: {nft}\n\
                     Given address: {actor:?}\n\
                     Owner address: {owner_id:?}"
                );
            }
        }
        _ => {
            unreachable!("Unreachable metastate reply for the NFTQuery::Token payload has occured")
        }
    }
}

pub fn check_nft_name_n_description(
    nft_program: &Program,
    nft: u128,
    name: &str,
    description: &str,
) {
    let nft = nft.into();
    match nft_program.meta_state(NFTQuery::Token { token_id: nft }) {
        NFTQueryReply::Token {
            token:
                Token {
                    name: true_name,
                    description: true_description,
                    ..
                },
        } => {
            if name != true_name || description != true_description {
                panic!(
                    "Name & description assertion failed.\n\
                     NFT ID: {nft}\n\
                     Given name: {name:?}\n\
                     Given description: {description:?}\n\
                     True name: {true_name:?}\n\
                     True description: {true_description:?}"
                )
            }
        }
        _ => {
            unreachable!("Unreachable metastate reply for the NFTQuery::Token payload has occured")
        }
    }
}

pub fn check_balance(ft_program: &Program, user: u64, balance: u128) {
    ft_program
        .send(FOREIGN_USER, FTAction::BalanceOf(user.into()))
        .contains(&(FOREIGN_USER, FTEvent::Balance(balance).encode()));
}
