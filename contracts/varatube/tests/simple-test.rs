use fungible_token_io::{FTAction, FTEvent, InitConfig};
use gstd::{ActorId, Encode};
use gtest::{Program, System};
use varatube_io::*;

use crate::utils::{FTokenTestFuncs, VaratubeTestFuncs};

const USERS: &[u64] = &[3, 4, 5];
pub mod utils;
fn preconfigure(system: &System) -> (Program, Program) {
    let ft = Program::ftoken(
        system,
        USERS[0],
        String::from("MyToken"),
        String::from("MTK"),
        12,
    );

    ft.mint(USERS[0], 100_000_000_000);

    let varatube = Program::varatube(
        system,
        USERS[0],
        Config {
            gas_for_token_transfer: 10_000_000_000,
            gas_for_delayed_msg: 500_000_000_000,
            block_duration: 1,
            min_gas_limit: 20_000_000_000,
        },
    );

    ft.approve(USERS[0], varatube.id(), 100_000_000_000);

    varatube.add_token_data(USERS[0], ft.id(), 10_000, None);

    (ft, varatube)
}
#[test]
fn register_subscriber() {
    let system = System::new();
    system.init_logger();

    let (ft, varatube) = preconfigure(&system);

    // Register Subscription
    varatube.register_subscription(USERS[0], ft.id(), Period::Month, true, None);

    let reply: StateReply = varatube
        .read_state(StateQuery::Subscribers)
        .expect("Error in reading state");
    if let StateReply::Subscribers(subcribers) = reply {
        println!("{:?}", subcribers);
    }

    system.spend_blocks(40);

    let reply: StateReply = varatube
        .read_state(StateQuery::Subscribers)
        .expect("Error in reading state");
    if let StateReply::Subscribers(subcribers) = reply {
        println!("{:?}", subcribers);
    }

    // let state: SubscriptionState = varatube.read_state(0).unwrap();
    // println!("currencies = {:?}", state.currencies);

    // assert!(!state.subscribers.is_empty());
    // assert!(!state.currencies.is_empty());
}
