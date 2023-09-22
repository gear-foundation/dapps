#![no_std]

use gstd::ActorId;
use oracle_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = Oracle;

    pub fn get_owner(state: State) -> ActorId {
        state.owner
    }

    pub fn get_manager(state: State) -> ActorId {
        state.manager
    }
}
