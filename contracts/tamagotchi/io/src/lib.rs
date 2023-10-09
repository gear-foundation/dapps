#![no_std]

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

pub type AttributeId = u32;
pub type TransactionId = u64;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<TmgInit>;
    type Handle = InOut<TmgAction, TmgReply>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Tamagotchi>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TmgInit {
    pub name: String,
}

#[derive(Encode, Decode, TypeInfo, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    Name,
    Age,
    Feed,
    Play,
    Sleep,
    TmgInfo,
}

#[derive(Encode, Debug, PartialEq, Eq, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgReply {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    TmgInfo {
        owner: ActorId,
        name: String,
        date_of_birth: u64,
    },
}

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
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

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TmgCurrentState {
    pub fed: u64,
    pub entertained: u64,
    pub rested: u64,
}

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;

pub const FILL_PER_FEED: u64 = 2_000;
pub const FILL_PER_ENTERTAINMENT: u64 = 2_000;
pub const FILL_PER_SLEEP: u64 = 2_000;

pub const MAX_VALUE: u64 = 10_000;
