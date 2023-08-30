#![no_std]

use galactic_express_io::*;
use gmeta::metawasm;
use gstd::prelude::*;

#[metawasm]
pub mod metafns {
    pub type State = LaunchSite;

    pub fn session_info(state: State) -> Option<CurrentSession> {
        state.current_session
    }

    pub fn launch_status(state: State) -> SessionState {
        state.state
    }

    pub fn participants(state: State) -> Vec<ParticipantInfo> {
        let mut participants = Vec::new();
        for (address, info) in state.participants {
            participants.push(ParticipantInfo {
                address,
                name: info.name,
                balance: info.balance,
            })
        }
        participants
    }
}
