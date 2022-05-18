#![no_std]
#![feature(const_btree_new)]

use codec::{Decode, Encode};
use gstd::{exec, msg, prelude::*, ActorId};
pub mod base;
use base::NonFungibleTokenBase;
pub mod token;
use token::TokenMetadata;

use primitive_types::U256;
use scale_info::TypeInfo;

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug, Default)]
pub struct NonFungibleToken {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: BTreeMap<U256, ActorId>,
    pub token_metadata_by_id: BTreeMap<U256, TokenMetadata>,
    pub token_approvals: BTreeMap<U256, ActorId>,
    pub balances: BTreeMap<ActorId, U256>,
    pub operator_approval: BTreeMap<ActorId, ActorId>,
}

impl NonFungibleTokenBase for NonFungibleToken {
    fn init(&mut self, name: String, symbol: String, base_uri: String) {
        self.name = name;
        self.symbol = symbol;
        self.base_uri = base_uri;
    }

    fn transfer(&mut self, to: &ActorId, token_id: U256) {
        if !self.exists(token_id) {
            panic!("NonFungibleToken: token does not exist");
        }

        if to == &ZERO_ID {
            panic!("NonFungibleToken: Transfer to zero address.");
        }

        let owner = *self.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID);

        if &owner == to {
            panic!("NonFungibleToken: Transfer to current owner");
        }

        match self.authorized_actor(token_id, &owner) {
            AuthAccount::None => {
                panic!("NonFungibleToken: is not an authorized source");
            }
            AuthAccount::ApprovedActor => {
                self.token_approvals.remove(&token_id);
            }
            _ => {}
        }

        let from_balance = *self.balances.get(&owner).unwrap_or(&U256::zero());
        let to_balance = *self.balances.get(to).unwrap_or(&U256::zero());

        self.balances
            .insert(owner, from_balance.saturating_sub(U256::one()));
        self.balances
            .insert(*to, to_balance.saturating_add(U256::one()));

        self.owner_by_id.insert(token_id, *to);

        msg::reply(
            Event::Transfer {
                from: owner,
                to: *to,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn approve(&mut self, to: &ActorId, token_id: U256) {
        if to == &ZERO_ID {
            panic!("NonFungibleToken: Approval to zero address.");
        }

        let owner = self.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID);

        if to == owner {
            panic!("NonFungibleToken: Approval to current owner");
        }

        if !self.is_token_owner(owner) {
            panic!("NonFungibleToken: is not owner");
        }

        self.token_approvals.insert(token_id, *to);

        msg::reply(
            Event::Approval {
                from: *owner,
                to: *to,
                token_id,
            },
            0,
        )
        .unwrap();
    }

    fn approve_for_all(&mut self, owner: &ActorId, operator: &ActorId, approved: bool) {
        if operator == &ZERO_ID {
            panic!("NonFungibleToken: Approval for a zero address");
        }
        match approved {
            true => self.operator_approval.insert(*owner, *operator),
            false => self.operator_approval.remove(owner),
        };

        msg::reply(
            Event::ApprovalForAll {
                owner: *owner,
                operator: *operator,
                approved,
            },
            0,
        )
        .unwrap();
    }

    fn balance_of(&self, account: &ActorId) {
        let balance = *self.balances.get(account).unwrap_or(&U256::zero());
        msg::reply(Event::BalanceOf(balance), 0).unwrap();
    }

    fn owner_of(&self, token_id: U256) {
        let owner = self.owner_by_id.get(&token_id).unwrap_or(&ZERO_ID);
        msg::reply(Event::OwnerOf(*owner), 0).unwrap();
    }
}

impl NonFungibleToken {
    pub const fn new() -> NonFungibleToken {
        NonFungibleToken {
            name: String::new(),
            symbol: String::new(),
            base_uri: String::new(),
            owner_by_id: BTreeMap::new(),
            token_metadata_by_id: BTreeMap::new(),
            token_approvals: BTreeMap::new(),
            balances: BTreeMap::new(),
            operator_approval: BTreeMap::new(),
        }
    }

    pub fn is_token_owner(&self, owner: &ActorId) -> bool {
        &msg::source() == owner || &exec::origin() == owner
    }

    pub fn authorized_actor(&self, token_id: U256, owner: &ActorId) -> AuthAccount {
        if owner == &msg::source() || owner == &exec::origin() {
            return AuthAccount::Owner;
        }
        if self.token_approvals.get(&token_id).unwrap_or(&ZERO_ID) == &msg::source() {
            return AuthAccount::ApprovedActor;
        }
        if self.operator_approval.contains_key(owner) {
            return AuthAccount::Operator;
        }
        AuthAccount::None
    }

    pub fn exists(&self, token_id: U256) -> bool {
        self.owner_by_id.contains_key(&token_id)
    }
}

#[derive(Debug, Encode, TypeInfo, Decode)]
pub enum Event {
    Transfer {
        from: ActorId,
        to: ActorId,
        token_id: U256,
    },
    Approval {
        from: ActorId,
        to: ActorId,
        token_id: U256,
    },
    ApprovalForAll {
        owner: ActorId,
        operator: ActorId,
        approved: bool,
    },
    OwnerOf(ActorId),
    BalanceOf(U256),
}

#[derive(Debug, Encode, TypeInfo)]
pub enum AuthAccount {
    Owner,
    ApprovedActor,
    Operator,
    None,
}
