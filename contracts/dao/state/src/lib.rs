#![no_std]

use dao_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = <DaoMetadata as Metadata>::State;

    pub fn is_member(state: State, account: ActorId) -> bool {
        DaoState::is_member(state, &account)
    }

    pub fn is_in_whitelist(state: State, account: ActorId) -> bool {
        DaoState::is_in_whitelist(state, &account)
    }

    pub fn get_proposal_id(state: State) -> u128 {
        DaoState::get_proposal_id(state)
    }

    pub fn get_proposal_info(state: State, id: u128) -> Proposal {
        DaoState::get_proposal_info(state, id).expect("Invalid proposal id")
    }

    pub fn get_member_info(state: State, account: ActorId) -> Member {
        DaoState::get_member_info(state, &account).expect("Invalid member account")
    }
}
