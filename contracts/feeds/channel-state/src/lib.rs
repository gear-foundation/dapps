#![no_std]

use channel_io::*;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;

#[metawasm]
pub mod metafns {
    pub type State = <ChannelMetadata as Metadata>::State;

    pub fn all_messages(state: State) -> Vec<Message> {
        state.messages
    }
}
