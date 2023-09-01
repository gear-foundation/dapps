#![no_std]

use gstd::prelude::*;
use nft_marketplace_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = Market;

    pub fn all_items(state: State) -> Vec<Item> {
        nft_marketplace_io::all_items(state)
    }

    pub fn item_info(state: State, args: ItemInfoArgs) -> Option<Item> {
        nft_marketplace_io::item_info(state, &args)
    }
}
