#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::prelude::*;
use market_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <MarketMetadata as Metadata>::State;

    fn all_items(state: Self::State) -> Vec<Item> {
        market_io::all_items(state)
    }

    fn item_info(args: ItemInfoArgs, state: Self::State) -> Item {
        market_io::item_info(state, &args).expect("Item not found")
    }
}
