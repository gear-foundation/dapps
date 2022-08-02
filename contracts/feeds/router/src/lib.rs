#![no_std]

use codec::Encode;
use gstd::{debug, msg, prelude::*, ActorId};
use router_io::*;

gstd::metadata! {
title: "GEAR Workshop Router Contract",
    handle:
        input: RouterAction,
        output: RouterEvent,
    state:
        input: RouterState,
        output: RouterStateReply,
}

#[derive(Default)]
pub struct Router {
    pub channels: BTreeMap<ActorId, Channel>,
    pub subscribers: BTreeMap<ActorId, BTreeSet<ActorId>>,
}
static mut ROUTER: Option<Router> = None;

impl Router {
    fn register_channel(&mut self, name: String, description: String, owner_id: ActorId) {
        debug!("ROUTER: Starting registering {:?}", msg::source());
        let channel = Channel {
            id: msg::source(),
            owner_id,
            name: name.clone(),
            description: description.clone(),
        };
        assert!(
            self.channels.insert(msg::source(), channel).is_none(),
            "That channel was already added"
        );
        debug!(
            "ROUTER: Successfully added channel\nName: {:?}\nAddress: {:?}\nOwner: {:?}",
            name,
            msg::source(),
            owner_id
        );
        msg::reply(
            RouterEvent::ChannelRegistered {
                channel_contract_id: msg::source(),
                name,
                owner_id,
                description,
            },
            0,
        )
        .expect("Error in sending reply `[RouterEvent::ChannelRegistered]`");
    }

    fn add_subscriber_to_channel(&mut self, subscriber_id: ActorId) {
        assert!(
            self.channels.contains_key(&msg::source()),
            "That channel is not registered"
        );
        self.subscribers
            .entry(subscriber_id)
            .and_modify(|channels| {
                channels.insert(msg::source());
            })
            .or_insert_with(|| BTreeSet::from([msg::source()]));

        msg::reply(
            RouterEvent::SubscriberAddedToChannel {
                subscriber_id,
                channel_id: msg::source(),
            },
            0,
        )
        .expect("Error in sending reply `[RouterEvent::SubscriberAddedToChannel]`");
    }

    fn remove_subscriber_from_channel(&mut self, subscriber_id: ActorId) {
        assert!(
            self.channels.contains_key(&msg::source()),
            "That channel is not registered"
        );
        self.subscribers
            .entry(subscriber_id)
            .and_modify(|channels| {
                channels.remove(&msg::source());
            });

        msg::reply(
            RouterEvent::SubscriberRemovedFromChannel {
                subscriber_id,
                channel_id: msg::source(),
            },
            0,
        )
        .expect("Error in sending reply `[RouterEvent::SubscriberAddedToChannel]`");
    }
}

#[gstd::async_main]
async fn main() {
    let action: RouterAction = msg::load().expect("ROUTER: Unable to decode RouterAction");

    let router = unsafe { ROUTER.get_or_insert(Default::default()) };

    match action {
        RouterAction::Register {
            name,
            description,
            owner_id,
        } => {
            router.register_channel(name, description, owner_id);
        }
        RouterAction::AddSubscriberToChannel(subscriber_id) => {
            router.add_subscriber_to_channel(subscriber_id);
        }
        RouterAction::RemoveSubscriberFromChannel(subscriber_id) => {
            router.remove_subscriber_from_channel(subscriber_id);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("Router Contract initialized successfully!");
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: RouterState = msg::load().expect("failed to decode RouterState");
    let router = ROUTER.get_or_insert(Default::default());
    let encoded = match query {
        RouterState::AllChannels => {
            let all_channels = router.channels.values().cloned().collect();
            RouterStateReply::AllChannels(all_channels)
        }
        RouterState::Channel(id) => {
            let channel = router
                .channels
                .get(&id)
                .unwrap_or(&Default::default())
                .clone();
            RouterStateReply::Channel(channel)
        }
        RouterState::SubscribedToChannels(user_id) => {
            let channel_ids = router
                .subscribers
                .get(&user_id)
                .unwrap_or(&BTreeSet::new())
                .clone();
            RouterStateReply::SubscribedToChannels(channel_ids)
        }
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}
