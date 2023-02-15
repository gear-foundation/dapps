#![no_std]
use gmeta::metawasm;
use gstd::{prelude::*, exec};
use launch_io::LaunchSiteState;

#[metawasm]
pub trait Metawasm {
    type State = Rocket;

    fn current_state(state: Self::State) -> LaunchSiteState {
        LaunchSiteState {
            // todo: populate
            name: state.name,
            current_session: None,
            participants: vec![],
        }
    }
}
