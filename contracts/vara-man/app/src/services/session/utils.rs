use sails_rs::{collections::HashMap, prelude::*};
pub type SessionMap = HashMap<ActorId, SessionData>;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum SessionError {
    BadSignature,
    BadPublicKey,
    VerificationFailed,
    DurationIsSmall,
    ThereAreNoAllowedMessages,
    MessageOnlyForProgram,
    TooEarlyToDeleteSession,
    NoSession,
    AlreadyHaveActiveSession,
}

// This structure is for creating a gaming session, which allows players to predefine certain actions for an account that will play the game on their behalf for a certain period of time.
// Sessions can be used to send transactions from a dApp on behalf of a user without requiring their confirmation with a wallet.
// The user is guaranteed that the dApp can only execute transactions that comply with the allowed_actions of the session until the session expires.
#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct SessionData {
    // the address of the player who will play on behalf of the user
    pub key: ActorId,
    // until what time the session is valid
    pub expires: u64,
    // what messages are allowed to be sent by the account (key)
    pub allowed_actions: Vec<ActionsForSession>,
    pub expires_at_block: u32,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum ActionsForSession {
    CreateNewTournament,
    RegisterForTournament,
    CancelRegister,
    CancelTournament,
    DeletePlayer,
    FinishSingleGame,
    StartTournament,
    RecordTournamentResult,
    LeaveGame,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct SignatureData {
    pub key: ActorId,
    pub duration: u64,
    pub allowed_actions: Vec<ActionsForSession>,
}
