use alloc::string::String;
use gstd::{prelude::*, ActorId};
use primitive_types::U256;

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum State {
    ConfirmationsCount(U256),
    TransactionsCount {
        pending: bool,
        executed: bool,
    },
    Owners,
    Confirmations(U256),
    TransactionIds {
        from_index: u64,
        to_index: u64,
        pending: bool,
        executed: bool,
    },
    IsConfirmed(U256),
    Description(U256),
}

#[derive(Debug, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateReply {
    ConfirmationCount(u64),
    TransactionsCount(u64),
    Owners(Vec<ActorId>),
    Confirmations(Vec<ActorId>),
    TransactionIds(Vec<U256>),
    IsConfirmed(bool),
    Description(Option<String>),
}
