#![no_std]

use auto_changed_nft_io::*;
use gear_lib_old::non_fungible_token::state::NFTQueryReply;
use gstd::String;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = NFTState2;

    pub fn info(state: State) -> NFTQueryReply {
        NFTQueryReply::NFTInfo {
            name: state.collection.name,
            symbol: String::new(),
            base_uri: String::new(),
        }
    }
}
