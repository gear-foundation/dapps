#![no_std]
use gstd::{collections::BTreeMap, msg, prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum CarAction {
    YourTurn(BTreeMap<ActorId, Car>),
}
#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Car {
    pub balance: u32,
    pub position: u32,
    pub speed: u32,
    pub penalty: u8,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StrategyAction {
    BuyAcceleration,
    BuyShell,
    Skip,
}

#[no_mangle]
extern fn handle() {
    msg::reply(StrategyAction::BuyAcceleration, 0).expect("Error in sending a message");
}
