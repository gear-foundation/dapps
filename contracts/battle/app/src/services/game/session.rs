use sails_rs::prelude::*;
use session_service::*;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ActionsForSession {
    CreateNewBattle,
    Registration,
    StartBattle,
    MakeMove,
}

generate_session_system!(ActionsForSession);
