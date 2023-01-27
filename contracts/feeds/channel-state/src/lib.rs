#![no_std]

use channel_io::*;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;

#[metawasm]
pub trait Metawasm {
    type State = <ChannelMetadata as Metadata>::State;

    fn all_messages(state: Self::State) -> Vec<Message> {
        state.messages
    }
}
