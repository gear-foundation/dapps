#![no_std]

use feeds_channel_io::*;
use gstd::prelude::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = Channel;

    pub fn all_messages(state: State) -> Vec<Message> {
        state.messages
    }
}
