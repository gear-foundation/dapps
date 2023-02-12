#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
pub type AttributeId = u32;
pub type TransactionId = u64;
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<String, ()>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = Tamagotchi;
}

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum TmgAction {
    Name,
    Age,
    Feed,
    Play,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    TmgInfo,
}

#[derive(Encode, Debug, PartialEq, Eq, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    TmgInfo {
        owner: ActorId,
        name: String,
        date_of_birth: u64,
    },
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub rested: u64,
    pub rested_block: u64,
    pub allowed_account: Option<ActorId>,
}
