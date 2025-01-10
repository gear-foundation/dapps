use sails_rs::prelude::*;
use session_service::*;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
pub enum ActionsForSession {
    StartGame,
    Move,
    Skip,
}

generate_session_system!(ActionsForSession);
