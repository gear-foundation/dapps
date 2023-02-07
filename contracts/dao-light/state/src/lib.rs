#![no_std]

use dao_light_io::*;
use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};

#[metawasm]
pub trait Metawasm {
    type State = <DaoLightMetadata as Metadata>::State;

    fn user_status(account: ActorId, state: Self::State) -> Role {
        if state.is_member(&account) {
            Role::Member
        } else {
            Role::None
        }
    }

    fn all_proposals(state: Self::State) -> Vec<Proposal> {
        state
            .proposals
            .iter()
            .map(|(_, proposal)| proposal.clone())
            .collect()
    }

    fn is_member(account: ActorId, state: Self::State) -> bool {
        state.is_member(&account)
    }

    fn proposal_id(state: Self::State) -> u128 {
        state.proposal_id
    }

    fn proposal_info(proposal_id: u128, state: Self::State) -> Proposal {
        let (_, proposal) = state
            .proposals
            .iter()
            .find(|(id, _)| proposal_id == *id)
            .expect("Invalid proposal id");
        proposal.clone()
    }

    fn member_info(account: ActorId, state: Self::State) -> Member {
        let (_, member) = state
            .members
            .iter()
            .find(|(id, _)| account == *id)
            .expect("Invalid account");

        member.clone()
    }

    fn member_power(account: ActorId, state: Self::State) -> u128 {
        let (_, member) = state
            .members
            .iter()
            .find(|(id, _)| account == *id)
            .expect("Invalid account");

        member.shares
    }
}
