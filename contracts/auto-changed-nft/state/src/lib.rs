#![no_std]

use auto_changed_nft_io::NFTMetadata;
use gear_lib::non_fungible_token::state::NFTQueryReply;
use gmeta::{metawasm, Metadata};
use gstd::String;

#[metawasm]
pub mod metafns {
    pub type State = <NFTMetadata as Metadata>::State;

    pub fn info(state: State) -> NFTQueryReply {
        NFTQueryReply::NFTInfo {
            name: state.collection.name,
            symbol: String::new(),
            base_uri: String::new(),
        }
    }
}
