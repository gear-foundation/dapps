#![no_std]

use ddns_io::*;
use gstd::{prelude::*, ActorId};

#[gmeta::metawasm]
pub mod metafns {
    pub type State = Vec<DnsRecord>;

    pub fn get_by_id(state: State, id: ActorId) -> Option<DnsRecord> {
        state.get_by_id(id)
    }

    pub fn get_by_name(state: State, name: String) -> Vec<DnsRecord> {
        state.get_by_name(name)
    }

    pub fn get_by_creator(state: State, creator: ActorId) -> Vec<DnsRecord> {
        state.get_by_creator(creator)
    }

    pub fn get_by_description(state: State, description: String) -> Vec<DnsRecord> {
        state.get_by_description(description)
    }

    pub fn get_by_pattern(state: State, pattern: String) -> Vec<DnsRecord> {
        state.get_by_pattern(pattern)
    }
}
