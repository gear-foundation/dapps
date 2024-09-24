use sails_rs::prelude::*;

#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
pub enum Error {
    MessageProcessingSuspended,
    MustBeTwoStrategies,
    NotAdmin,
    NoMessagesForApproval,
    NoSession,
    GameAlreadyStarted,
    DurationTooSmall,
    NotPlayerTurn,
    NotProgram,
    UnexpectedState,
}
