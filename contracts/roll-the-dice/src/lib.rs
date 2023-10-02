#![no_std]

use gstd::{collections::HashMap, msg, prelude::*, ActorId};
use roll_the_dice_io::*;

#[derive(Debug, Default)]
struct Contract {
    users_data: HashMap<u128, (ActorId, RollStatus)>,
    oracle: ActorId,
}

impl Contract {
    /// Request random value from `oracle`.
    pub async fn roll(&mut self) {
        let oracle_reply: oracle_io::Event =
            msg::send_for_reply_as(self.oracle, oracle_io::Action::RequestValue, 0, 0)
                .expect("Unable to request value from oracle!")
                .await
                .expect("Unable to decode oracle reply!");

        if let oracle_io::Event::NewValue { value: _ } = oracle_reply {
            // TODO: Implement random value handler
            /* self.users_data
                .insert(id, (msg::source(), RollStatus::Rolling));
            msg::reply(Event::RollValueRequested(id), 0).unwrap(); */
        } else {
            panic!("Invalid oracle reply!");
        }
    }

    /// Handle reply from `oracle` with random value and id.
    pub fn roll_finished(&mut self, id: u128, value: u128) {
        let (_, roll_status) = self.users_data.get_mut(&id).expect("Invalid ID!");
        *roll_status = RollStatus::Finished(value % 2 == 0);

        msg::reply(Event::RollFinished((id, value)).encode(), 0).expect("Unable to reply!");
    }
}

static mut ROLL_DICE: Option<Contract> = None;

#[no_mangle]
unsafe extern fn init() {
    let config: InitConfig = msg::load().expect("Unable to decode InitConfig.");
    let roll_dice = Contract {
        oracle: config.oracle,
        ..Default::default()
    };

    ROLL_DICE = Some(roll_dice);
}

#[gstd::async_main]
async fn main() {
    let roll_dice: &mut Contract = unsafe { ROLL_DICE.get_or_insert(Contract::default()) };

    // Handler(proxy) for oracle messages
    if msg::source() == roll_dice.oracle {
        let payload = msg::load_bytes().expect("Unable to load payload bytes.");
        let id: u128 = u128::from_le_bytes(
            payload[1..17]
                .try_into()
                .expect("Unable to obtain id bytes."),
        );
        let value: u128 = u128::from_le_bytes(
            payload[17..]
                .try_into()
                .expect("Unable to obtain value bytes."),
        );

        roll_dice.roll_finished(id, value);
        return;
    }

    let action: Action = msg::load().expect("Unable to decode Action.");
    match action {
        Action::Roll => roll_dice.roll().await,
    }
}

#[no_mangle]
unsafe extern fn state() {
    let roll_dice = ROLL_DICE.get_or_insert(Default::default());

    msg::reply(
        State {
            users_data: roll_dice.users_data.clone().into_iter().collect(),
            oracle: roll_dice.oracle,
        },
        0,
    )
    .expect("Unable to reply!");
}
