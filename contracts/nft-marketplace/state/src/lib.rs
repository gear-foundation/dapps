#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::prelude::*;
use market_io::*;

#[metawasm]
pub mod metafns {
    pub type State = <MarketMetadata as Metadata>::State;

    pub fn all_items(state: State) -> Vec<Item> {
        market_io::all_items(state)
    }

    pub fn item_info(state: State, args: ItemInfoArgs) -> Option<Item> {
        market_io::item_info(state, &args)
    }
}
