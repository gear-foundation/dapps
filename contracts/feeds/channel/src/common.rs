use gstd::{exec, prelude::String, ActorId};

use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ChannelAction {
    Meta,
    Subscribe,
    Unsubscribe,
    Post(String),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ChannelOutput {
    Metadata(Meta),
    SingleMessage(Message),
}

#[derive(Clone, Debug, Encode, Decode, TypeInfo, Default)]
pub struct Message {
    pub text: String,
    pub timestamp: u32,
}

impl Message {
    pub fn new(text: String) -> Self {
        Self {
            text,
            timestamp: exec::block_height(),
        }
    }
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Meta {
    pub name: String,
    pub description: String,
    pub owner_id: ActorId,
}

impl Meta {
    pub const fn new(name: String, description: String, owner_id: ActorId) -> Self {
        Self {
            name,
            description,
            owner_id,
        }
    }
}
