#![no_std]

use gstd::prelude::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = Vec<String>;

    pub fn get_first_message(state: State) -> String {
        state.first().expect("Message log is empty!").to_string()
    }

    pub fn get_last_message(state: State) -> String {
        state.last().expect("Message log is empty!").to_string()
    }

    pub fn get_messages_len(state: State) -> u64 {
        state.len() as u64
    }

    pub fn get_message(state: State, index: u64) -> String {
        state
            .get(index as usize)
            .expect("Invalid index!")
            .to_string()
    }
}
