use crate::Item;
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::U256;
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum State {
    AllItems,
    ItemInfo {
        nft_contract_id: ActorId,
        token_id: U256,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    AllItems(Vec<Item>),
    ItemInfo(Item),
}
