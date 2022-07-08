#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};

mod common;
use common::*;

gstd::metadata! {
    title: "GEAR Workshop Channel Contract",
    handle:
        input: ChannelAction,
        output: ChannelOutput,
    state:
      output: Vec<Message>,
}

#[derive(Default)]
pub struct Channel {
    pub owner_id: ActorId,
    pub name: String,
    pub description: String,
    pub subscribers: BTreeSet<ActorId>,
    pub messages: Vec<Message>,
}
static mut CHANNEL: Option<Channel> = None;

impl Channel {
    pub fn set_owner_id(&mut self, id: ActorId) {
        self.owner_id = id;
    }
    pub fn is_owner(&self, id: ActorId) -> bool {
        id == self.owner_id
    }
    pub fn set_name(&mut self, name: &'static str) {
        self.name = String::from(name);
    }

    pub fn set_description(&mut self, desc: &'static str) {
        self.description = String::from(desc);
    }

    pub fn add_subscriber(&mut self, id: ActorId) {
        self.subscribers.insert(id);
    }

    pub fn remove_subscriber(&mut self, id: ActorId) {
        self.subscribers.retain(|v| *v != id);
    }

    pub fn post(&mut self, text: String) {
        assert!(self.owner_id == msg::source(), "Poster is not an owner");
        let message = Message::new(text);
        self.add_message(message);

        // event
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn subs(&self) -> BTreeSet<ActorId> {
        self.subscribers.clone()
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let mut channel: Channel = Default::default();
    channel.set_owner_id(msg::source());
    channel.set_name("Channel-Coolest-Name");
    channel.set_description("Channel-Coolest-Description");

    let init_message = Message::new(format!("Channel {:?} was created", channel.name));

    channel.add_message(init_message);
    channel.add_subscriber(msg::source());

    debug!(
        "Channel {:?} initialized successfully!",
        channel.name.clone()
    );
    CHANNEL = Some(channel);
}

#[gstd::async_main]
async unsafe fn main() {
    let channel = unsafe { CHANNEL.get_or_insert(Default::default()) };
    let action: ChannelAction = msg::load().unwrap_or_else(|_| {
        panic!(
            "CHANNEL {:?}: Unable to decode Channel Action",
            channel.name
        )
    });

    debug!("CHANNEL {:?}: Received action: {:?}", channel.name, action);
    match action {
        ChannelAction::Meta => {
            let meta = ChannelOutput::Metadata(Meta::new(
                channel.name.clone(),
                channel.description.clone(),
                channel.owner_id,
            ));

            msg::reply(meta, 0).expect("Error in reply ChannelOutput::Metadata");

            debug!("CHANNEL {:?}: Meta sent", channel.name)
        }
        ChannelAction::Subscribe => {
            channel.add_subscriber(msg::source());

            msg::reply((), 0).expect("Error in reply to message  ChannelAction::Subscribe");

            debug!("CHANNEL {:?}: Subscriber added", channel.name)
        }
        ChannelAction::Unsubscribe => {
            channel.remove_subscriber(msg::source());

            msg::reply((), 0).expect("Error in reply to message  ChannelAction::Unsubscribe");

            debug!("CHANNEL {:?}: Subscriber removed", channel.name)
        }
        ChannelAction::Post(text) => {
            if !channel.is_owner(msg::source()) {
                panic!("CHANNEL {:?}: Poster is not an owner", channel.name)
            }

            let message = Message::new(text);

            channel.add_message(message.clone());

            for sub in channel.subscribers.clone() {
                msg::send(sub, ChannelOutput::SingleMessage(message.clone()), 0)
                    .expect("Error in sending message to subscriber");
            }
            msg::reply((), 0).expect("Error in reply to message  ChannelAction::Post");

            debug!("Added a post: {:?}", message);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let channel = CHANNEL.get_or_insert(Default::default());
    let messages: Vec<Message> = channel.messages.clone();
    let encoded = messages.encode();
    gstd::util::to_leak_ptr(encoded)
}
