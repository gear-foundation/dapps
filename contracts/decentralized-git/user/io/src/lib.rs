#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{collections::BTreeMap, prelude::*, ActorId, CodeId};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<InitUserProgram>;
    type Handle = InOut<UserActionRequest, UserActionResponse>;
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
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub repos: BTreeMap<ActorId, Repository>,
    pub repo_code_id: ActorId,
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitUserProgram {
    pub owner: ActorId,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub repo_code_id: CodeId,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum UserActionRequest {
    UpdateUserData(UpdateUserDataInput),
    CreateRepository(CreateRepositoryInput),
    RenameRepository(String),
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum UserActionResponse {
    UpdateUserData { message: String },
    CreateRepository { message: String },
    Ok,
    Err,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UpdateUserDataInput {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Encode, Debug, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct CreateRepositoryInput {
    pub name: String,
}

#[derive(Encode, Debug, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Repository {
    pub id: ActorId,
    pub name: String,
    pub created_at: u64,
    pub updated_at: u64,
}
