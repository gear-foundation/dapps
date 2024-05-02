#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata, Out};
use gstd::{collections::BTreeMap, prelude::*, ActorId};
use scale_info::TypeInfo;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
pub struct Stream {
    pub broadcaster: ActorId,
    pub start_time: u64,
    pub end_time: u64,
    pub title: String,
    pub img_link: String,
    pub description: Option<String>,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Profile {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub img_link: Option<String>,
    pub stream_ids: Vec<String>,
    pub subscribers: Vec<ActorId>,
    pub subscriptions: Vec<Subscription>,
}

#[derive(Encode, Decode, Default, TypeInfo, Clone)]
pub struct Program {
    pub streams: BTreeMap<String, Stream>,
    pub users: BTreeMap<ActorId, Profile>,
}

#[derive(Encode, Decode, Default, TypeInfo, Clone)]
pub struct State {
    pub streams: Vec<(String, Stream)>,
    pub users: Vec<(ActorId, Profile)>,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Subscription {
    pub account_id: ActorId,
    pub sub_date: u64,
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
    DeleteStream {
        stream_id: String,
    },
    EditStream {
        stream_id: String,
        start_time: Option<u64>,
        end_time: Option<u64>,
        title: Option<String>,
        img_link: Option<String>,
        description: Option<String>,
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
pub enum Event {
    StreamIsScheduled { id: String },
    StreamDeleted { id: String },
    StreamEdited,
    Subscribed,
    ProfileEdited,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

impl From<Program> for State {
    fn from(program: Program) -> Self {
        Self {
            streams: program.streams.into_iter().collect(),
            users: program.users.into_iter().collect(),
        }
    }
}
