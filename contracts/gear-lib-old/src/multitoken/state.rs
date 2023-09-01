use crate::multitoken::io::*;
use gstd::{collections::HashMap, prelude::*, ActorId};

#[derive(Debug, Default)]
pub struct MTKState {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub balances: HashMap<TokenId, HashMap<ActorId, u128>>,
    pub approvals: HashMap<ActorId, HashMap<ActorId, bool>>,
    pub token_metadata: HashMap<TokenId, TokenMetadata>,
    // owner for nft
    pub owners: HashMap<TokenId, ActorId>,
}

pub trait StateKeeper {
    fn get(&self) -> &MTKState;
    fn get_mut(&mut self) -> &mut MTKState;
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MTKQuery {
    Name,
    Symbol,
    Uri,
    BalanceOf(ActorId, TokenId),
    MetadataOf(TokenId),
    URI(TokenId),
    TokensForOwner(ActorId),
    TokensIDsForOwner(ActorId),
    Supply(TokenId),
    OwnerOf(TokenId),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MTKQueryReply {
    Name(String),
    Symbol(String),
    Uri(String),
    Balance(TokenId),
    URI(String),
    MetadataOf(TokenMetadata),
    TokensForOwner(Vec<Token>),
    TokensIDsForOwner(Vec<TokenId>),
    Supply(u128),
    OwnerOf(ActorId),
}

pub trait MTKTokenState: StateKeeper {
    fn get_balance(&self, account: &ActorId, id: &TokenId) -> u128 {
        *self
            .get()
            .balances
            .get(id)
            .and_then(|m| m.get(account))
            .unwrap_or(&0)
    }

    fn set_balance(&mut self, account: &ActorId, id: &TokenId, amount: u128) {
        let mut _balance = self
            .get_mut()
            .balances
            .entry(*id)
            .or_default()
            .insert(*account, amount);
    }

    fn get_uri(&self, id: TokenId) -> String {
        self.get()
            .base_uri
            .clone()
            .replace("{id}", &format!("{id}"))
    }

    fn get_metadata(&self, id: TokenId) -> TokenMetadata {
        self.get()
            .token_metadata
            .get(&id)
            .cloned()
            .unwrap_or_default()
    }

    fn tokens_for_owner(&self, owner: &ActorId) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let balances = &self.get().balances;
        for (token, bal) in balances {
            if let Some(amount) = bal.get(owner) {
                // tokens ActorId, u128
                tokens.push(Token {
                    id: *token,
                    amount: *amount,
                    metadata: gstd::Some(
                        self.get()
                            .token_metadata
                            .get(token)
                            .cloned()
                            .unwrap_or_default(),
                    ),
                })
            }
        }
        tokens
    }

    fn tokens_ids_for_owner(&self, owner: &ActorId) -> Vec<TokenId> {
        let mut tokens: Vec<TokenId> = Vec::new();
        let balances = &self.get().balances;
        for (token, bals) in balances {
            if bals.get(owner).is_some() {
                tokens.push(*token);
            }
        }
        tokens
    }

    fn supply(&self, id: TokenId) -> u128 {
        self.get()
            .balances
            .get(&id)
            .expect("Balances always exist; qed")
            .values()
            .sum()
    }

    fn owner_of(&self, id: TokenId) -> ActorId {
        *self.get().owners.get(&id).expect("No owner for a token")
    }

    fn proc_state(&mut self, query: MTKQuery) -> Option<Vec<u8>> {
        let state = match query {
            MTKQuery::Name => MTKQueryReply::Name(self.get().name.clone()),
            MTKQuery::Symbol => MTKQueryReply::Symbol(self.get().symbol.clone()),
            MTKQuery::Uri => MTKQueryReply::Uri(self.get().base_uri.clone()),
            MTKQuery::BalanceOf(account, id) => {
                MTKQueryReply::Balance(Self::get_balance(self, &account, &id))
            }
            MTKQuery::URI(id) => MTKQueryReply::URI(Self::get_uri(self, id)),
            MTKQuery::MetadataOf(id) => MTKQueryReply::MetadataOf(Self::get_metadata(self, id)),
            MTKQuery::TokensIDsForOwner(owner) => {
                MTKQueryReply::TokensIDsForOwner(Self::tokens_ids_for_owner(self, &owner))
            }
            MTKQuery::TokensForOwner(owner) => {
                MTKQueryReply::TokensForOwner(Self::tokens_for_owner(self, &owner))
            }
            MTKQuery::Supply(id) => MTKQueryReply::Supply(Self::supply(self, id)),
            MTKQuery::OwnerOf(id) => MTKQueryReply::OwnerOf(Self::owner_of(self, id)),
        };
        Some(state.encode())
    }
}
