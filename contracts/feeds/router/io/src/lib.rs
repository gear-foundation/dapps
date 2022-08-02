#![no_std]
use gstd::{prelude::*, ActorId};

use codec::{Decode, Encode};
use scale_info::TypeInfo;

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

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum RouterState {
    AllChannels,
    Channel(ActorId),
    SubscribedToChannels(ActorId),
}

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum RouterStateReply {
    AllChannels(Vec<Channel>),
    Channel(Channel),
    SubscribedToChannels(BTreeSet<ActorId>),
}
