#![no_std]

use gstd::{debug, msg, prelude::*};

mod common;
mod state;

use common::*;
use state::State;

pub use state::meta_state;

gstd::metadata! {
    title: "GEAR Workshop Channel Contract",
    handle:
        input: ChannelAction,
        output: ChannelOutput,
    state:
      output: Vec<Message>,
}

static mut STATE: State = State::new();

#[no_mangle]
pub unsafe extern "C" fn init() {
    STATE.set_owner_id(msg::source());
    STATE.set_name("Channel-Coolest-Name");
    STATE.set_description("Channel-Coolest-Description");

    let init_message = Message::new(format!("Channel {:?} was created", STATE.name()));

    STATE.add_message(init_message);
    STATE.add_subscriber(msg::source());

    debug!("Channel {:?} initialized successfully!", STATE.name());
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let action: ChannelAction = msg::load().unwrap_or_else(|_| {
        panic!(
            "CHANNEL {:?}: Unable to decode Channel Action",
            STATE.name()
        )
    });

    debug!("CHANNEL {:?}: Received action: {:?}", STATE.name(), action);

    match action {
        ChannelAction::Meta => {
            let meta = ChannelOutput::Metadata(Meta::new(
                STATE.name(),
                STATE.description(),
                STATE.owner(),
            ));

            msg::reply(meta, 0).unwrap();

            debug!("CHANNEL {:?}: Meta sent", STATE.name())
        }
        ChannelAction::Subscribe => {
            STATE.add_subscriber(msg::source());

            msg::reply((), 0).unwrap();

            debug!("CHANNEL {:?}: Subscriber added", STATE.name())
        }
        ChannelAction::Unsubscribe => {
            STATE.remove_subscriber(msg::source());

            msg::reply((), 0).unwrap();

            debug!("CHANNEL {:?}: Subscriber removed", STATE.name())
        }
        ChannelAction::Post(text) => {
            if !STATE.is_owner(msg::source()) {
                panic!("CHANNEL {:?}: Poster is not an owner", STATE.name())
            }

            let message = Message::new(text);

            STATE.add_message(message.clone());

            for sub in STATE.subs() {
                msg::send(sub, ChannelOutput::SingleMessage(message.clone()), 0).unwrap();
            }

            msg::reply((), 0).unwrap();

            debug!("Added a post: {:?}", message);
        }
    }
}
