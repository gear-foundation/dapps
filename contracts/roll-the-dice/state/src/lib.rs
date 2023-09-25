#![no_std]

use roll_the_dice_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = roll_the_dice_io::State;

    pub fn query(state: State, query: StateQuery) -> StateResponse {
        match query {
            StateQuery::GetUsersData => StateResponse::UsersData(
                state
                    .users_data
                    .iter()
                    .map(|(id, (user, status))| (*id, *user, *status))
                    .collect(),
            ),
        }
    }
}
