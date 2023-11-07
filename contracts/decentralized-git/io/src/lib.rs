#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{collections::BTreeMap, exec::block_timestamp, prelude::*, ActorId};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitRepoProgram>;
    type Handle = InOut<RepoActionRequests, RepoActionResponses>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Program>;
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Program {
    pub owner: ActorId,
    pub name: String,
    pub user_program_id: ActorId,
    pub collaborator: BTreeMap<ActorId, ActorId>,
    pub branches: BTreeMap<String, Branch>,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitRepoProgram {
    pub owner: ActorId,
    pub name: String,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RepoActionRequests {
    Rename(String),
    CreateBranch(String),
    RenameBranch(RenameBranchInput),
    DeleteBranch(DeleteBranchInput),
    Push(PushInput),
    // Merge(MergeInput),
    // Rebase(RebaseInput),
    // Checkout(CheckoutInput),
    AddCollaborator(ActorId),
    DeleteCollaborator(ActorId),
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum RepoActionResponses {
    Rename { msg: String },
    CreateBranch { msg: String },
    RenameBranch { msg: String },
    DeleteBranch { msg: String },
    Push { msg: Commit },
    // Merge{ msg: Branch },
    // Rebase{ msg: Branch },
    // Checkout{ msg: String },
    AddCollaborator { msg: String },
    DeleteCollaborator { msg: String },
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct RenameBranchInput {
    pub id: String,
    pub name: String,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Branch {
    pub id: String,
    pub owner: ActorId,
    pub name: String,
    pub commits: Vec<Commit>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Commit {
    pub id: String,
    pub owner: ActorId,
    pub hash: String,
    pub created_at: u64,
}

#[derive(Encode, Debug, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CreateBranchInput {
    pub id: String,
    pub name: String,
    pub owner: ActorId,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct DeleteBranchInput {
    pub branch_id: String,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct PushInput {
    pub branch_id: String,
    pub hash: String,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CheckoutInput {
    pub name: String,
    pub hash: Option<String>,
    pub is_new: bool, // for create branch by checkout action
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MergeInput {
    pub branch_name_from: String,
    pub branch_name_to: String,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct RebaseInput {
    pub branch_name_from: String,
    pub branch_name_to: String,
}

impl Branch {
    pub fn new(create_branch_input: CreateBranchInput) -> Self {
        Self {
            id: create_branch_input.id,
            name: create_branch_input.name,
            owner: create_branch_input.owner,
            commits: vec![],
            created_at: block_timestamp(),
            updated_at: block_timestamp(),
        }
    }

    pub fn rename(&mut self, new_name: String) -> String {
        self.name = new_name;

        self.name.clone()
    }

    pub fn add_commit(&mut self, commit: Commit) {
        self.commits.push(commit);
    }

    pub fn get_commits(&self) -> Vec<Commit> {
        self.commits.clone()
    }

    pub fn is_exist_commit_by_hash(&self, hash: String) -> bool {
        for c in self.commits.iter() {
            if c.hash == hash {
                return true;
            }
        }

        false
    }

    pub fn get_commit_by_hash(&self, hash: String) -> Option<Commit> {
        self.commits
            .iter()
            .find(|&commit| commit.hash.eq(&hash)).cloned()
    }
}
