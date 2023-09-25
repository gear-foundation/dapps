use crate::state;
use gstd::{prelude::*, ActorId, Decode, Encode, TypeInfo};

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    SetRandomValue { round: u128, value: state::Random },
    GetLastRoundWithRandomValue,
    UpdateManager(ActorId),
}
