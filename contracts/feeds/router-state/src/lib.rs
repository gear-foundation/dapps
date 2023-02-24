#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use router_io::*;

#[metawasm]
pub mod metafns {
    pub type State = <RouterMetadata as Metadata>::State;

    pub fn all_channels(state: State) -> Vec<Channel> {
        state.channels.iter().map(|(_, c)| c.clone()).collect()
    }

    pub fn channel(state: State, id: ActorId) -> Channel {
        let (_, channel) = state
            .channels
            .iter()
            .find(|(channel_id, _)| channel_id == &id)
            .expect("Invalid id!");

        channel.clone()
    }

    pub fn subscribed_to_channels(state: State, id: ActorId) -> Vec<ActorId> {
        let (_, subs) = state
            .subscribers
            .iter()
            .find(|(user_id, _)| user_id == &id)
            .expect("Invalid id!");

        subs.clone()
    }
}
