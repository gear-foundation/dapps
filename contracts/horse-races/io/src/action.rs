use crate::Horse;
use gstd::{collections::BTreeMap, prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Action {
    UpdateFeeBps(u16),
    UpdateManager(ActorId),
    UpdateOracle(ActorId),
    ProgressLastRun,
    CancelLastRun,
    CreateRun {
        bidding_duration_ms: u64,
        horses: BTreeMap<String, Horse>,
    },
    FinishLastRun,
    Bid {
        horse_name: String,
        amount: u128,
    },
    WithdrawCanceled(u128),
    WithdrawFinished(u128),
}
