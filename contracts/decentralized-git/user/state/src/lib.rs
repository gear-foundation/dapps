#![no_std]
use decentralized_git_user_io::*;
use gmeta::metawasm;
use gstd::prelude::*;

#[metawasm]
pub mod metafns {
    pub type State = Program;

    pub fn get_program_data(state: State) -> Program {
        state
    }

    pub fn get_user_repos(state: State) -> Vec<Repository> {
        let mut repos: Vec<Repository> = vec![];

        for (_, repo) in state.repos.iter() {
            repos.push(repo.clone())
        }

        repos
    }
}
