#![no_std]

use dao_light_io::*;
use gstd::{prelude::*, ActorId};

#[gmeta::metawasm]
pub mod metafns {
    pub type State = DaoState;

    pub fn user_status(state: State, account: ActorId) -> Role {
        if state.is_member(&account) {
            Role::Member
        } else {
            Role::None
        }
    }

    pub fn all_proposals(state: State) -> Vec<Proposal> {
        state
            .proposals
            .iter()
            .map(|(_, proposal)| proposal.clone())
            .collect()
    }

    pub fn is_member(state: State, account: ActorId) -> bool {
        state.is_member(&account)
    }

    pub fn proposal_id(state: State) -> u128 {
        state.proposal_id
    }

    pub fn proposal_info(state: State, proposal_id: u128) -> Proposal {
        let (_, proposal) = state
            .proposals
            .iter()
            .find(|(id, _)| proposal_id == *id)
            .expect("Invalid proposal id");
        proposal.clone()
    }

    pub fn member_info(state: State, account: ActorId) -> Member {
        let (_, member) = state
            .members
            .iter()
            .find(|(id, _)| account == *id)
            .expect("Invalid account");

        member.clone()
    }

    pub fn member_power(state: State, account: ActorId) -> u128 {
        let (_, member) = state
            .members
            .iter()
            .find(|(id, _)| account == *id)
            .expect("Invalid account");

        member.shares
    }
}
