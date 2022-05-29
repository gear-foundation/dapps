#![no_std]

use codec::Encode;
use gstd::{debug, exec, msg, prelude::*, ActorId};
use primitive_types::U256;

pub mod state;
pub use state::{State, StateReply};

pub use nft_example_io::{Action, Event, InitConfig};

use non_fungible_token::base::NonFungibleTokenBase;
use non_fungible_token::NonFungibleToken;

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug)]
pub struct NFT {
    pub token: NonFungibleToken,
    pub token_id: U256,
    pub owner: ActorId,
}

static mut CONTRACT: NFT = NFT {
    token: NonFungibleToken::new(),
    token_id: U256::zero(),
    owner: ZERO_ID,
};

impl NFT {
    fn mint(&mut self) {
        self.token.owner_by_id.insert(self.token_id, msg::source());
        let balance = *self
            .token
            .balances
            .get(&msg::source())
            .unwrap_or(&U256::zero());
        self.token
            .balances
            .insert(msg::source(), balance.saturating_add(U256::one()));

        msg::reply(
            Event::Transfer {
                from: ZERO_ID,
                to: msg::source(),
                token_id: self.token_id,
            },
            0,
        )
        .unwrap();
        self.token_id = self.token_id.saturating_add(U256::one());
    }

    fn burn(&mut self, token_id: U256) {
        if !self.token.exists(token_id) {
            panic!("NonFungibleToken: Token does not exist");
        }
        self.check_owner(token_id);
        self.token.token_approvals.remove(&token_id);
        self.token.owner_by_id.remove(&token_id);
        let balance = *self
            .token
            .balances
            .get(&msg::source())
            .unwrap_or(&U256::zero());
        self.token
            .balances
            .insert(msg::source(), balance.saturating_sub(U256::one()));
        msg::reply(
            Event::Transfer {
                from: msg::source(),
                to: ZERO_ID,
                token_id,
            },
            0,
        )
        .unwrap();
    }
    fn check_owner(&self, token_id: U256) {
        if self.token.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID) != &msg::source()
            || self.token.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID) != &exec::origin()
        {
            panic!("Only owner can transfer");
        }
    }
}

gstd::metadata! {
    title: "NftExample",
        init:
            input: InitConfig,
        handle:
            input: Action,
            output: Event,
        state:
            input: State,
            output: StateReply,
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: Action = msg::load().expect("Could not load Action");
    match action {
        Action::Mint => {
            CONTRACT.mint();
        }
        Action::Burn(amount) => {
            CONTRACT.burn(amount);
        }
        Action::Transfer { to, token_id } => {
            CONTRACT.token.transfer(&to, token_id);
        }
        Action::Approve { to, token_id } => {
            CONTRACT.token.approve(&to, token_id);
        }
        Action::ApproveForAll { to, approved } => {
            CONTRACT
                .token
                .approve_for_all(&msg::source(), &to, approved);
        }
        Action::OwnerOf(input) => {
            CONTRACT.token.owner_of(input);
        }
        Action::BalanceOf(input) => {
            CONTRACT.token.balance_of(&input);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let config: InitConfig = msg::load().expect("Unable to decode InitConfig");
    debug!("NFT {:?}", config);
    CONTRACT
        .token
        .init(config.name, config.symbol, config.base_uri);
    CONTRACT.owner = msg::source();
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: State = msg::load().expect("failed to decode input argument");
    let encoded = match query {
        State::BalanceOfUser(input) => {
            StateReply::BalanceOfUser(*CONTRACT.token.balances.get(&input).unwrap_or(&U256::zero()))
        }
        State::TokenOwner(input) => {
            let user = CONTRACT.token.owner_by_id.get(&input).unwrap_or(&ZERO_ID);
            StateReply::TokenOwner(*user)
        }
        State::IsTokenOwner { account, token_id } => {
            let user = CONTRACT
                .token
                .owner_by_id
                .get(&token_id)
                .unwrap_or(&ZERO_ID);
            StateReply::IsTokenOwner(user == &account)
        }
        State::GetApproved(input) => {
            let approved_address = CONTRACT
                .token
                .token_approvals
                .get(&input)
                .unwrap_or(&ZERO_ID);
            StateReply::GetApproved(*approved_address)
        }
    }
    .encode();

    gstd::util::to_leak_ptr(encoded)
}
