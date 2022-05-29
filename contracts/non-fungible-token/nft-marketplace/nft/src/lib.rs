#![no_std]

use gstd::{exec, msg, prelude::*, ActorId};
use primitive_types::U256;

pub use nft_io::*;
pub use royalties::*;

pub mod state;
pub use state::{State, StateReply};

use non_fungible_token::base::NonFungibleTokenBase;
use non_fungible_token::token::TokenMetadata;
use non_fungible_token::NonFungibleToken;

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug, Default)]
pub struct NFT {
    pub tokens: NonFungibleToken,
    pub owner: ActorId,
    pub owner_to_ids: BTreeMap<ActorId, Vec<U256>>,
    pub supply: U256,
    pub royalties: Option<Royalties>,
    pub token_id: U256,
}

static mut CONTRACT: Option<NFT> = None;

impl NFT {
    fn mint(&mut self, media: String, reference: String) {
        if self.token_id >= self.supply {
            panic!("No tokens left");
        }
        let token_id = self.token_id;
        self.token_id = self.token_id.saturating_add(U256::one());

        self.tokens.owner_by_id.insert(token_id, msg::source());
        self.owner_to_ids
            .entry(msg::source())
            .and_modify(|ids| ids.push(token_id))
            .or_insert_with(|| vec![token_id]);
        let metadata = TokenMetadata {
            title: None,
            description: None,
            media: Some(media),
            reference: Some(reference),
        };
        self.tokens
            .token_metadata_by_id
            .insert(self.token_id, metadata);
        let balance = *self
            .tokens
            .balances
            .get(&msg::source())
            .unwrap_or(&U256::zero());
        self.tokens
            .balances
            .insert(msg::source(), balance.saturating_add(U256::one()));

        msg::reply(
            NFTEvent::Transfer {
                from: ZERO_ID,
                to: msg::source(),
                token_id: self.token_id,
            },
            0,
        )
        .unwrap();
    }

    fn burn(&mut self, token_id: U256) {
        if !self.tokens.exists(token_id) {
            panic!("NonFungibleToken: Token does not exist");
        }
        self.check_owner(token_id);
        self.tokens.token_approvals.remove(&token_id);
        self.tokens.owner_by_id.remove(&token_id);
        let balance = *self
            .tokens
            .balances
            .get(&msg::source())
            .unwrap_or(&U256::zero());
        self.tokens
            .balances
            .insert(msg::source(), balance.saturating_sub(U256::one()));

        msg::reply(
            NFTEvent::Transfer {
                from: msg::source(),
                to: ZERO_ID,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn nft_payout(&self, owner: &ActorId, amount: u128) {
        let payouts: Payout = if self.royalties.is_some() {
            self.royalties.as_ref().unwrap().payouts(owner, amount)
        } else {
            let mut single_payout = BTreeMap::new();
            single_payout.insert(*owner, amount);
            single_payout
        };
        msg::reply(NFTEvent::NFTPayout(payouts), 0).unwrap();
    }

    fn check_owner(&self, token_id: U256) {
        if self.tokens.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID) != &msg::source()
            || self.tokens.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID) != &exec::origin()
        {
            panic!("Only owner can transfer");
        }
    }
}

gstd::metadata! {
    title: "NFT",
        init:
            input: InitNFT,
        handle:
            input: NFTAction,
            output: NFTEvent,
        state:
            input: State,
            output: StateReply,
}

#[gstd::async_main]
async fn main() {
    let action: NFTAction = msg::load().expect("Could not load Action");
    let nft: &mut NFT = unsafe { CONTRACT.get_or_insert(NFT::default()) };
    match action {
        NFTAction::Mint { media, reference } => {
            nft.mint(media, reference);
        }
        NFTAction::Burn(token_id) => {
            nft.burn(token_id);
        }
        NFTAction::Transfer { to, token_id } => {
            nft.tokens.transfer(&to, token_id);
        }
        NFTAction::TokensForOwner(account) => {
            let tokens = nft.owner_to_ids.get(&account).unwrap_or(&vec![]).clone();
            msg::reply(NFTEvent::TokensForOwner(tokens), 0).unwrap();
        }
        NFTAction::NFTPayout { owner, amount } => {
            nft.nft_payout(&owner, amount);
        }
        NFTAction::Approve { to, token_id } => {
            nft.tokens.approve(&to, token_id);
        }
        NFTAction::OwnerOf(token_id) => {
            let owner = nft.tokens.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID);
            msg::reply(NFTEvent::OwnerOf(*owner), 0).unwrap();
        }
        NFTAction::BalanceOf(account) => {
            nft.tokens.balance_of(&account);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitNFT = msg::load().expect("Unable to decode InitConfig");
    let mut nft = NFT {
        tokens: NonFungibleToken::new(),
        owner: msg::source(),
        supply: config.supply,
        ..NFT::default()
    };
    nft.tokens.init(config.name, config.symbol, config.base_uri);
    CONTRACT = Some(nft);
}
