#![no_std]

use dutch_auction_io::auction::*;
use gstd::{exec, prelude::*};

#[gmeta::metawasm]
pub mod metafns {
    pub type State = AuctionInfo;

    pub fn info(mut state: State) -> AuctionInfo {
        if matches!(state.status, Status::IsRunning) && exec::block_timestamp() >= state.expires_at
        {
            state.status = Status::Expired
        }
        state
    }
}
