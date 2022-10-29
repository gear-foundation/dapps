extern crate std;

use cell::Cell;
use gstd::{prelude::*, ActorId};
use std::thread_local;

thread_local! {
    static SOURCE: Cell<ActorId> = Cell::new(ActorId::zero());
    static LAST_REPLY: Cell<Vec<u8>> = Cell::new(Vec::new())
}

pub mod msg {
    use super::*;

    pub fn source() -> ActorId {
        SOURCE.with(|actor_id| actor_id.get())
    }

    pub fn set_source(new_source: ActorId) {
        SOURCE.with(|actor_id| actor_id.set(new_source))
    }
}

pub mod utils {
    use super::*;

    pub fn last_reply<T: Decode>() -> T {
        LAST_REPLY.with(|reply| T::decode(&mut reply.take().as_ref()).unwrap())
    }

    pub fn set_last_reply(new_reply: impl Encode) {
        LAST_REPLY.with(|reply| reply.set(new_reply.encode()))
    }
}
