//! Basic token implementations & primitives.

pub mod fungible;
pub mod multi;
pub mod non_fungible;
pub mod types;

#[cfg(test)]
mod test_helper {
    extern crate std;

    use gstd::{cell::Cell, ActorId};
    use std::thread_local;

    thread_local! {
        static SOURCE: Cell<ActorId> = Cell::new(ActorId::zero());
    }

    pub mod msg {
        use super::*;

        pub fn source() -> ActorId {
            SOURCE.with(Cell::get)
        }

        pub fn set_source(source: ActorId) {
            SOURCE.with(|old_source| old_source.set(source));
        }
    }
}
