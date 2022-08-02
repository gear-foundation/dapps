#![no_std]

use channel_io::*;
use gstd::{debug, msg, prelude::*, ActorId};
use router_io::*;

gstd::metadata! {
    title: "GEAR Workshop Channel Contract",
    init:
        input: ChannelInit,
    handle:
        input: ChannelAction,
        output: ChannelOutput,
    state:
      output: Vec<Message>,
}

#[derive(Default)]
pub struct Channel {
    pub owner_id: ActorId,
    pub router_id: ActorId,
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

    pub fn set_router_id(&mut self, id: ActorId) {
        self.router_id = id;
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

    pub async fn add_subscriber(&mut self, id: ActorId) {
        assert!(
            self.subscribers.insert(id),
            "Already subscribed to that channel"
        );
        // send message to router contract to inform about new subscriber
        msg::send_for_reply_as::<_, RouterEvent>(
            self.router_id,
            RouterAction::AddSubscriberToChannel(msg::source()),
            0,
        )
        .expect("Error in sending async message `[RouterAction::AddSubscriberToChannel]` to router contract")
        .await
        .expect("Error in async message `[RouterAction::AddSubscriberToChannel]`");
        msg::reply(ChannelOutput::SubscriberAdded(id), 0)
            .expect("Error in reply to message  ChannelAction::Subscribe");

        debug!("CHANNEL {:?}: Subscriber added", self.name)
    }

    pub async fn remove_subscriber(&mut self, id: ActorId) {
        if self.subscribers.remove(&id) {
            // send message to router contract to delete a subscriber
            msg::send_for_reply_as::<_, RouterEvent>(
                self.router_id,
                RouterAction::RemoveSubscriberFromChannel(msg::source()),
                0,
            )
            .expect("Error in sending async message `[RouterAction::RemoveSubscriberFromChannel]` to router contract")
            .await
            .expect("Error in async message `[RouterAction::RemoveSubscriberFromChannel]`");
        } else {
            panic!("The msg::source() does not subscribe to that channel");
        }

        msg::reply(ChannelOutput::SubscriberRemoved(id), 0)
            .expect("Error in reply to message  ChannelAction::Unsubscribe");

        debug!("CHANNEL {:?}: Subscriber removed", self.name)
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn subs(&self) -> BTreeSet<ActorId> {
        self.subscribers.clone()
    }
}

#[gstd::async_init]
async fn init() {
    let channel_init: ChannelInit = msg::load().expect("Unable to decode ChannelInit");

    let mut channel: Channel = Default::default();
    channel.set_owner_id(msg::source());
    channel.set_router_id(channel_init.router_contract_id);
    channel.set_name("Channel-Coolest-Name");
    channel.set_description("Channel-Coolest-Description");

    debug!("ROUTER ID {:?}", channel_init.router_contract_id);
    // sends message to router contract to register
    msg::send_for_reply_as::<_, RouterEvent>(
        channel_init.router_contract_id,
        RouterAction::Register {
            name: channel.name.clone(),
            description: channel.description.clone(),
            owner_id: msg::source(),
        },
        0,
    )
    .expect("Error in sending async message `[RouterAction::Register]` to router contract")
    .await
    .expect("Error in async message `[RouterAction::Register]`");

    let init_message = Message::new(format!("Channel {:?} was created", channel.name));

    channel.add_message(init_message);
    channel.add_subscriber(msg::source()).await;

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
        ChannelAction::Subscribe => {
            channel.add_subscriber(msg::source()).await;
        }
        ChannelAction::Unsubscribe => {
            channel.remove_subscriber(msg::source()).await;
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
            msg::reply(ChannelOutput::MessagePosted(message.clone()), 0)
                .expect("Error in reply to message  ChannelAction::Post");

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
