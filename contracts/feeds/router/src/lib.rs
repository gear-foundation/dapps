#![no_std]

use gstd::{debug, msg, prelude::*, ActorId};

use codec::{Decode, Encode};
use scale_info::TypeInfo;

gstd::metadata! {
  title: "GEAR Workshop Router Contract",
  handle:
      input: Register,
      output: Channel,
}

#[derive(Decode, TypeInfo)]
struct Register {
    address: ActorId,
}

#[derive(Encode, TypeInfo)]
struct Channel {
    id: ActorId,
    name: String,
    owner_id: ActorId,
    description: String,
}

#[derive(Encode, TypeInfo)]
enum ChannelAction {
    Meta,
}

#[derive(Decode, TypeInfo)]
enum ChannelOutput {
    Metadata(Meta),
}

#[derive(Clone, Decode, TypeInfo)]
struct Meta {
    name: String,
    description: String,
    owner_id: ActorId,
}

impl Channel {
    fn new(id: ActorId, meta: Meta) -> Self {
        Self {
            id,
            name: meta.name,
            owner_id: meta.owner_id,
            description: meta.description,
        }
    }
}

#[gstd::async_main]
async fn main() {
    let register: Register = msg::load().expect("ROUTER: Unable to decode Register");

    debug!("ROUTER: Starting registering {:?}", register.address);

    let ChannelOutput::Metadata(meta) =
        msg::send_and_wait_for_reply(register.address, ChannelAction::Meta, 0)
            .expect("ROUTER: Error sending async message")
            .await
            .expect("ROUTER: Error processing async message");

    msg::reply(Channel::new(register.address, meta.clone()), 0).expect("Error sending reply");

    debug!(
        "ROUTER: Successfully added channel\nName: {:?}\nAddress: {:?}\nOwner: {:?}",
        meta.name, register.address, meta.owner_id
    );
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("Router Contract initialized successfully!");
}
