use gear_lib::non_fungible_token::state::{NFTQuery, NFTQueryReply};
use gear_lib::non_fungible_token::token::TokenId;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;
use onchain_nft_io::*;

#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;

    pub fn token_uri(state: State, token_id: TokenId) -> Option<Vec<u8>> {
        let metadata = state
            .token
            .token_metadata_by_id
            .iter()
            .find(|(id, _)| token_id.eq(id))
            .and_then(|(_id, metadata)| metadata.clone())
            .unwrap_or_default();
        // construct media
        let mut content: Vec<String> = Vec::new();
        // check if exists

        if let Some((_id, nft)) = state.nfts.iter().find(|(id, _)| token_id.eq(id)) {
            for (i, layer_item_id) in nft.iter().enumerate() {
                if let Some((_id, layer_content)) =
                    state.layers.iter().find(|(id, _)| (i as u128).eq(id))
                {
                    let s = layer_content
                        .get(*layer_item_id as usize)
                        .expect("No such layer item");
                    content.push(s.clone());
                }
            }
        }

        Some(TokenURI { metadata, content }.encode())
    }

    pub fn base(state: State, query: NFTQuery) -> Option<Vec<u8>> {
        let encoded = match query {
            NFTQuery::NFTInfo => NFTQueryReply::NFTInfo {
                name: state.token.name.clone(),
                symbol: state.token.symbol.clone(),
                base_uri: state.token.base_uri,
            },
            NFTQuery::Token { token_id } => NFTQueryReply::Token {
                token: state.token.token(token_id),
            },
            NFTQuery::TokensForOwner { owner } => NFTQueryReply::TokensForOwner {
                tokens: state.token.tokens_for_owner(&owner),
            },
            NFTQuery::TotalSupply => NFTQueryReply::TotalSupply {
                total_supply: state.token.total_supply(),
            },
            NFTQuery::SupplyForOwner { owner } => NFTQueryReply::SupplyForOwner {
                supply: state.token.supply_for_owner(&owner),
            },
            NFTQuery::AllTokens => NFTQueryReply::AllTokens {
                tokens: state.token.all_tokens(),
            },
            NFTQuery::ApprovedTokens { account } => NFTQueryReply::ApprovedTokens {
                tokens: state.token.approved_tokens(&account),
            },
        }
        .encode();
        Some(encoded)
    }
}
