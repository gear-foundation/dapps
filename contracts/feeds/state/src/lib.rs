#![no_std]

use feeds_io::*;
use gstd::{prelude::*, ActorId};

#[gmeta::metawasm]
pub mod metafns {
    pub type State = RouterState;

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
