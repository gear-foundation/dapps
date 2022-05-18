#![no_std]

use gstd::{debug, msg, prelude::*};

use codec::{Decode, Encode};
use primitive_types::H256;
use scale_info::TypeInfo;

gstd::metadata! {
  title: "GEAR Workshop Router Contract",
  handle:
      input: Register,
      output: Channel,
}

#[derive(Decode, TypeInfo)]
struct Register {
    address: H256,
}

#[derive(Encode, TypeInfo)]
struct Channel {
    id: H256,
    name: String,
    owner_id: H256,
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
    owner_id: H256,
}

impl Channel {
    fn new(id: H256, meta: Meta) -> Self {
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
        msg::send_and_wait_for_reply(register.address.into(), ChannelAction::Meta, 0)
            .unwrap()
            .await
            .expect("ROUTER: Error processing async message");

    msg::reply(Channel::new(register.address, meta.clone()), 0).unwrap();

    debug!(
        "ROUTER: Successfully added channel\nName: {:?}\nAddress: {:?}\nOwner: {:?}",
        meta.name, register.address, meta.owner_id
    );
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    debug!("Router Contract initialized successfully!");
}
