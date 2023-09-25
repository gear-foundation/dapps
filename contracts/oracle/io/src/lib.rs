#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, TypeInfo};

pub struct OracleMetadata;

impl Metadata for OracleMetadata {
    type Init = In<InitConfig>;
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Oracle>;
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Oracle {
    pub owner: ActorId,
    pub manager: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitConfig {
    pub owner: ActorId,
    pub manager: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    RequestValue,
    ChangeManager(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Event {
    NewValue { value: u128 },
    NewManager(ActorId),
}
