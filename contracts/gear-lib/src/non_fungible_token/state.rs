use crate::non_fungible_token::{royalties::*, token::*};
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Debug, Default)]
pub struct NFTState {
    pub name: String,
    pub symbol: String,
    pub base_uri: String,
    pub owner_by_id: BTreeMap<TokenId, ActorId>,
    pub token_approvals: BTreeMap<TokenId, Vec<ActorId>>,
    pub token_metadata_by_id: BTreeMap<TokenId, Option<TokenMetadata>>,
    pub tokens_for_owner: BTreeMap<ActorId, Vec<TokenId>>,
    pub royalties: Option<Royalties>,
}

pub trait NFTStateKeeper {
    fn get(&self) -> &NFTState;
    fn get_mut(&mut self) -> &mut NFTState;
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum NFTQuery {
    NFTInfo,
    Token { token_id: TokenId },
    TokensForOwner { owner: ActorId },
    TotalSupply,
    SupplyForOwner { owner: ActorId },
    AllTokens,
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum NFTQueryReply {
    NFTInfo {
        name: String,
        symbol: String,
        base_uri: String,
    },
    Token {
        token: Token,
    },
    TokensForOwner {
        tokens: Vec<Token>,
    },
    TotalSupply {
        total_supply: u128,
    },
    SupplyForOwner {
        supply: u128,
    },
    AllTokens {
        tokens: Vec<Token>,
    },
}

pub trait NFTMetaState: NFTStateKeeper {
    fn token(&self, token_id: TokenId) -> Token {
        let mut token = Token::default();
        if let Some(owner_id) = self.get().owner_by_id.get(&token_id) {
            token.id = token_id;
            token.owner_id = *owner_id;
        }
        if let Some(approved_account_ids) = self.get().token_approvals.get(&token_id) {
            token.approved_account_ids = approved_account_ids.clone();
        }
        if let Some(Some(metadata)) = self.get().token_metadata_by_id.get(&token_id) {
            token.name = metadata.name.clone();
            token.description = metadata.description.clone();
            token.media = metadata.media.clone();
            token.reference = metadata.reference.clone();
        }
        token
    }

    fn tokens_for_owner(&self, owner: &ActorId) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        if let Some(token_ids) = self.get().tokens_for_owner.get(owner) {
            for token_id in token_ids {
                tokens.push(self.token(*token_id));
            }
        }
        tokens
    }

    fn total_supply(&self) -> u128 {
        self.get().owner_by_id.len() as u128
    }

    fn supply_for_owner(&self, owner: &ActorId) -> u128 {
        self.get()
            .tokens_for_owner
            .get(owner)
            .map(|tokens| tokens.len() as u128)
            .unwrap_or(0)
    }
    fn all_tokens(&self) -> Vec<Token> {
        self.get()
            .owner_by_id
            .keys()
            .map(|id| self.token(*id))
            .collect()
    }

    fn proc_state(&self, query: NFTQuery) -> Option<Vec<u8>> {
        let encoded = match query {
            NFTQuery::NFTInfo => NFTQueryReply::NFTInfo {
                name: self.get().name.clone(),
                symbol: self.get().symbol.clone(),
                base_uri: self.get().base_uri.clone(),
            }
            .encode(),
            NFTQuery::Token { token_id } => NFTQueryReply::Token {
                token: self.token(token_id),
            }
            .encode(),
            NFTQuery::TokensForOwner { owner } => NFTQueryReply::TokensForOwner {
                tokens: self.tokens_for_owner(&owner),
            }
            .encode(),
            NFTQuery::TotalSupply => NFTQueryReply::TotalSupply {
                total_supply: self.total_supply(),
            }
            .encode(),
            NFTQuery::SupplyForOwner { owner } => NFTQueryReply::SupplyForOwner {
                supply: self.supply_for_owner(&owner),
            }
            .encode(),
            NFTQuery::AllTokens => NFTQueryReply::AllTokens {
                tokens: self.all_tokens(),
            }
            .encode(),
        };
        Some(encoded)
    }
}
