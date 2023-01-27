use gstd::{debug, msg, prelude::*, ActorId};
use hashbrown::{HashMap, HashSet};
use router_io::*;

#[derive(Default)]
pub struct Router {
    pub channels: HashMap<ActorId, Channel>,
    pub subscribers: HashMap<ActorId, HashSet<ActorId>>,
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
            .or_insert_with(|| HashSet::from([msg::source()]));

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

impl From<&Router> for RouterState {
    fn from(router: &Router) -> Self {
        RouterState {
            channels: router
                .channels
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            subscribers: router
                .subscribers
                .iter()
                .map(|(key, value)| (*key, value.iter().cloned().collect()))
                .collect(),
        }
    }
}

#[no_mangle]
extern "C" fn handle() {
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
extern "C" fn state() {
    msg::reply(
        unsafe {
            let router = ROUTER.as_ref().expect("Uninitialized router state");
            let state: RouterState = router.into();
            state
        },
        0,
    )
    .expect("Failed to encode or reply with `<AppMetadata as Metadata>::State` from `state()`");
}

#[no_mangle]
extern "C" fn metahash() {
    msg::reply::<[u8; 32]>(include!("../.metahash"), 0)
        .expect("Failed to encode or reply with `[u8; 32]` from `metahash()`");
}
