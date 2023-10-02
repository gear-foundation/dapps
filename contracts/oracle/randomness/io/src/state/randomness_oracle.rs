use super::Random;
use gstd::{collections::BTreeMap, prelude::*, ActorId};

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct RandomnessOracle {
    pub owner: ActorId,
    pub values: BTreeMap<u128, Random>,
    pub last_round: u128,
    pub manager: ActorId,
}
