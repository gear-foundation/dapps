use gstd::{
    prelude::{String, Vec},
    ActorId,
};

use crate::Message;
use circular_buffer::CircularBuffer;
use codec::Encode;
use primitive_types::H256;

#[derive(Clone)]
pub struct State {
    owner_id: Option<ActorId>,
    name: Option<String>,
    description: Option<String>,
    subscribers: Vec<ActorId>,
    messages: Option<CircularBuffer<Message>>,
}

impl State {
    pub const fn new() -> Self {
        Self {
            name: None,
            description: None,
            owner_id: None,
            subscribers: Vec::new(),
            messages: None,
        }
    }

    pub fn set_owner_id(&mut self, id: ActorId) {
        if self.owner_id.is_none() {
            self.owner_id = Some(id);
        } else {
            panic!("Owner ID for the channel was already set!")
        }
    }

    pub fn set_name(&mut self, name: &'static str) {
        if self.name.is_none() {
            self.name = Some(String::from(name));
        } else {
            panic!("Name for the channel was already set!")
        }
    }

    pub fn set_description(&mut self, desc: &'static str) {
        if self.description.is_none() {
            self.description = Some(String::from(desc));
        } else {
            panic!("Description for the channel was already set!")
        }
    }

    pub fn add_subscriber(&mut self, id: ActorId) {
        self.subscribers.push(id);
    }

    pub fn remove_subscriber(&mut self, id: ActorId) {
        self.subscribers.retain(|v| *v != id);
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages
            .get_or_insert_with(|| CircularBuffer::new(5))
            .push(message);
    }

    pub fn is_owner(&self, id: ActorId) -> bool {
        if let Some(owner_id) = self.owner_id {
            owner_id == id
        } else {
            panic!("CHANNEL {:?}: Owner was not set", self.name())
        }
    }

    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| String::from("UNKNOWN"))
    }

    pub fn description(&self) -> String {
        self.description
            .clone()
            .unwrap_or_else(|| String::from("UNKNOWN"))
    }

    pub fn owner(&self) -> H256 {
        H256(self.owner_id.unwrap().into())
    }

    pub fn subs(&self) -> Vec<ActorId> {
        self.subscribers.clone()
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let messages: Vec<Message> = crate::STATE
        .messages
        .clone()
        .map(|v| v.into_iter().collect())
        .unwrap_or_default();
    let encoded = messages.encode();
    let result = gstd::macros::util::to_wasm_ptr(&encoded[..]);
    core::mem::forget(encoded);

    result
}
