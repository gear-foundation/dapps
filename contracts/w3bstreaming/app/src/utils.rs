use crate::Program;
use sails_rs::prelude::*;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Stream {
    pub broadcaster: ActorId,
    pub start_time: u64,
    pub end_time: u64,
    pub title: String,
    pub img_link: String,
    pub description: Option<String>,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Profile {
    pub name: Option<String>,
    pub surname: Option<String>,
    pub img_link: Option<String>,
    pub time_zone: Option<String>,
    pub stream_ids: Vec<String>,
    pub subscribers: Vec<ActorId>,
    pub subscriptions: Vec<Subscription>,
}

#[derive(Encode, Decode, Default, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ProgramState {
    pub streams: Vec<(String, Stream)>,
    pub users: Vec<(ActorId, Profile)>,
    pub admins: Vec<ActorId>,
    pub dns_info: Option<(ActorId, String)>,
}

impl From<Program> for ProgramState {
    fn from(program: Program) -> Self {
        Self {
            streams: program.streams.into_iter().collect(),
            users: program.users.into_iter().collect(),
            admins: program.admins,
            dns_info: program.dns_info,
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Subscription {
    pub account_id: ActorId,
    pub sub_date: u64,
}
