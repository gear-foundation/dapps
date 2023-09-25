#![no_std]
use decentralized_git_io::*;
use gmeta::metawasm;
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = Program;

    pub fn branch(state: State, branch_id: String) -> Option<Branch> {
        if let Some(b) = state.branches.get(&branch_id) {
            return Some(b.clone());
        }

        None
    }

    pub fn branches(state: State) -> Vec<Branch> {
        let mut result: Vec<Branch> = vec![];

        for (_, b) in state.branches.iter() {
            result.push(b.clone())
        }

        result
    }

    pub fn get_collaborators(state: State) -> Vec<ActorId> {
        let mut response: Vec<ActorId> = vec![];

        for (_, c) in state.collaborator.iter() {
            response.push(*c)
        }

        response
    }
}
