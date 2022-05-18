use gstd::{exec, prelude::String};

use codec::{Decode, Encode};
use primitive_types::H256;
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

#[derive(Clone, Debug, Encode, Decode, TypeInfo)]
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
    pub owner_id: H256,
}

impl Meta {
    pub const fn new(name: String, description: String, owner_id: H256) -> Self {
        Self {
            name,
            description,
            owner_id,
        }
    }
}
