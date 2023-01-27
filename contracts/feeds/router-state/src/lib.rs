#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use router_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <RouterMetadata as Metadata>::State;

    fn all_channels(state: Self::State) -> Vec<Channel> {
        state.channels.iter().map(|(_, c)| c.clone()).collect()
    }

    fn channel(id: ActorId, state: Self::State) -> Channel {
        let (_, channel) = state
            .channels
            .iter()
            .find(|(channel_id, _)| channel_id == &id)
            .expect("Invalid id!");

        channel.clone()
    }

    fn subscribed_to_channels(id: ActorId, state: Self::State) -> Vec<ActorId> {
        let (_, subs) = state
            .subscribers
            .iter()
            .find(|(user_id, _)| user_id == &id)
            .expect("Invalid id!");

        subs.clone()
    }
}
