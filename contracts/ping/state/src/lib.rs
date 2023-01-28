#![no_std]

use demo_ping_io::*;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;

#[metawasm]
pub trait Metawasm {
    type State = <DemoPingMetadata as Metadata>::State;

    fn get_first_message(state: Self::State) -> String {
        state.first().expect("Message log is empty!").to_string()
    }

    fn get_last_message(state: Self::State) -> String {
        state.last().expect("Message log is empty!").to_string()
    }

    fn get_messages_len(state: Self::State) -> u64 {
        state.len() as u64
    }

    fn get_message(index: u64, state: Self::State) -> String {
        state
            .get(index as usize)
            .expect("Invalid index!")
            .to_string()
    }
}
