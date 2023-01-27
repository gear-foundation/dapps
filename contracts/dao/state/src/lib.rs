#![no_std]

use dao_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <DaoMetadata as Metadata>::State;

    fn is_member(account: ActorId, state: Self::State) -> bool {
        DaoState::is_member(state, &account)
    }

    fn is_in_whitelist(account: ActorId, state: Self::State) -> bool {
        DaoState::is_in_whitelist(state, &account)
    }

    fn get_proposal_id(state: Self::State) -> u128 {
        DaoState::get_proposal_id(state)
    }

    fn get_proposal_info(id: u128, state: Self::State) -> Proposal {
        DaoState::get_proposal_info(state, id).expect("Invalid proposal id")
    }

    fn get_member_info(account: ActorId, state: Self::State) -> Member {
        DaoState::get_member_info(state, &account).expect("Invalid member account")
    }
}
