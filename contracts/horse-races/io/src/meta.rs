use crate::{Horse, Run};
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId, TypeInfo};

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MetaQuery {
    GetRuns,
    GetHorses(u128),
    GetManager,
    GetOwner,
    GetToken,
    GetOracle,
    GetFeeBps,
    GetRunNonce,
    GetRun(u128),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MetaResponse {
    Runs(Vec<(u128, Run)>),
    Horses(Vec<(String, Horse, u128)>),
    Manager(ActorId),
    Owner(ActorId),
    Token(ActorId),
    Oracle(ActorId),
    FeeBps(u16),
    RunNonce(u128),
    Run(Run),
}
