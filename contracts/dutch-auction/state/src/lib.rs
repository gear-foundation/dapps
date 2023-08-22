#![no_std]

use auction_io::{
    auction::{AuctionInfo, Status},
    io::AuctionMetadata,
};
use gmeta::{metawasm, Metadata};
use gstd::{exec, prelude::*};

#[metawasm]
pub mod metafns {
    pub type State = <AuctionMetadata as Metadata>::State;

    pub fn info(mut state: State) -> AuctionInfo {
        if matches!(state.status, Status::IsRunning) && exec::block_timestamp() >= state.expires_at
        {
            state.status = Status::Expired
        }
        state
    }
}
