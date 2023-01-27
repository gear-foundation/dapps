#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

pub struct RouterMetadata;

impl Metadata for RouterMetadata {
    type Init = ();
    type Handle = InOut<RouterAction, RouterEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = RouterState;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct RouterState {
    pub channels: Vec<(ActorId, Channel)>,
    pub subscribers: Vec<(ActorId, Vec<ActorId>)>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum RouterAction {
    Register {
        name: String,
        description: String,
        owner_id: ActorId,
    },
    AddSubscriberToChannel(ActorId),
    RemoveSubscriberFromChannel(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum RouterEvent {
    ChannelRegistered {
        channel_contract_id: ActorId,
        name: String,
        owner_id: ActorId,
        description: String,
    },
    SubscriberAddedToChannel {
        subscriber_id: ActorId,
        channel_id: ActorId,
    },
    SubscriberRemovedFromChannel {
        subscriber_id: ActorId,
        channel_id: ActorId,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo, Default, Clone, PartialEq, Eq)]
pub struct Channel {
    pub id: ActorId,
    pub name: String,
    pub owner_id: ActorId,
    pub description: String,
}
