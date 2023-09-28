#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId, BTreeMap};
use scale_info::TypeInfo;

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Stream {
    pub broadcaster: ActorId,
    pub start_time: u64,
    pub end_time: u64,
    pub title: String,
    pub img_link: String,
    pub description: Option<String>,
    pub watchers: Vec<ActorId>,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Profile {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub img_link: Option<String>,
    pub stream_ids: Vec<String>,
    pub subscribers: Vec<ActorId>,
    pub subscriptions: Vec<Subscription>,
    pub role: Role,
}

#[derive(Encode, Decode, Default, TypeInfo)]
pub struct Contract {
    pub streams: BTreeMap<String, Stream>,
    pub users: BTreeMap<ActorId, Profile>,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub enum Role {
    Speaker,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Subscription {
    pub account_id: ActorId,
    pub sub_date: u64,
    pub next_write_off: Option<u64>,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Action {
    NewStream {
        title: String,
        description: Option<String>,
        start_time: u64,
        end_time: u64,
        img_link: String,
    },
    Subscribe {
        account_id: ActorId,
    },
    EditProfile {
        name: Option<String>,
        surname: Option<String>,
        img_link: Option<String>,
    },
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ActionResult {
    StreamIsScheduled { id: String },
    Subscribed,
    ProfileEdited,
    StreamIsFinished { id: String },
    Error(String),
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<(), ()>;
    type Handle = InOut<Action, ActionResult>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = Contract;
}
