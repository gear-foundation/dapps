#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{exec, msg, prelude::*, ActorId, Decode, Encode, TypeInfo};

pub struct ChannelMetadata;

impl Metadata for ChannelMetadata {
    type Init = ();
    type Handle = InOut<ChannelAction, ChannelOutput>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Channel>;
}

#[derive(Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Channel {
    pub owner_id: ActorId,
    pub router_id: ActorId,
    pub name: String,
    pub description: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ChannelAction {
    Register { router_contract_id: ActorId },
    Subscribe,
    Unsubscribe,
    Post(String),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ChannelOutput {
    SubscriberAdded(ActorId),
    SubscriberRemoved(ActorId),
    MessagePosted(Message),
    SingleMessage(Message),
    Registered,
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo, Default, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Message {
    pub owner: ActorId,
    pub text: String,
    pub timestamp: u32,
}

impl Message {
    pub fn new(text: String) -> Self {
        Self {
            owner: msg::source(),
            text,
            timestamp: exec::block_height(),
        }
    }
}
