use gtest::{Program, System};
use varatube_io::*;

use crate::utils::{FTokenTestFuncs, VaratubeTestFuncs};

const USERS: &[u64] = &[3, 4, 5];
pub mod utils;
fn preconfigure(system: &System) -> (Program<'_>, Program<'_>) {
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
            gas_to_start_subscription_update: 500_000_000_000,
            block_duration: 1,
            min_gas_limit: 20_000_000_000,
        },
    );

    ft.approve(USERS[0], varatube.id(), 100_000_000_000);

    varatube.add_token_data(USERS[0], ft.id(), 10_000, None);

    (ft, varatube)
}
#[test]
fn register_subscriber_with_subscription_renewal() {
    let system = System::new();
    system.init_logger();

    let (ft, varatube) = preconfigure(&system);

    // Register Subscription
    varatube.register_subscription(USERS[0], ft.id(), Period::Month, true, None);

    let subscriber_data = varatube
        .get_subscriber_data(USERS[0])
        .expect("Subscriber does not exist");
    let expected_subscriber_data = SubscriberData {
        currency_id: <[u8; 32]>::from(ft.id()).into(),
        period: Period::Month,
        subscription_start: Some((system.block_timestamp(), system.block_height())),
        renewal_date: Some((
            system.block_timestamp() + Period::Month.as_millis(),
            system.block_height() + Period::Month.to_blocks(1),
        )),
    };

    assert_eq!(
        subscriber_data, expected_subscriber_data,
        "Subscriber data do not match"
    );

    let blocks = Period::Month.to_blocks(1);
    system.spend_blocks(blocks);

    let subscriber_data = varatube
        .get_subscriber_data(USERS[0])
        .expect("Subscriber does not exist");

    let expected_subscriber_data = SubscriberData {
        currency_id: <[u8; 32]>::from(ft.id()).into(),
        period: Period::Month,
        subscription_start: Some((system.block_timestamp(), system.block_height())),
        renewal_date: Some((
            system.block_timestamp() + Period::Month.as_millis(),
            system.block_height() + Period::Month.to_blocks(1),
        )),
    };
    assert_eq!(
        subscriber_data, expected_subscriber_data,
        "Subscriber data do not match"
    );
}

#[test]
fn register_subscriber_without_subscription_renewal() {
    let system = System::new();
    system.init_logger();

    let (ft, varatube) = preconfigure(&system);

    // Register Subscription
    varatube.register_subscription(USERS[0], ft.id(), Period::Month, false, None);

    let subscriber_data = varatube
        .get_subscriber_data(USERS[0])
        .expect("Subscriber does not exist");
    let expected_subscriber_data = SubscriberData {
        currency_id: <[u8; 32]>::from(ft.id()).into(),
        period: Period::Month,
        subscription_start: Some((system.block_timestamp(), system.block_height())),
        renewal_date: None,
    };
    assert_eq!(
        subscriber_data, expected_subscriber_data,
        "Subscriber data do not match"
    );

    let blocks = Period::Month.to_blocks(1);
    system.spend_blocks(blocks);

    let subscriber_data = varatube.get_subscriber_data(USERS[0]);

    assert_eq!(subscriber_data, None, "Subscriber data do not match");
}

#[test]
fn cancelling_subscription() {
    let system = System::new();
    system.init_logger();

    let (ft, varatube) = preconfigure(&system);

    // Register Subscription
    varatube.register_subscription(USERS[0], ft.id(), Period::Month, true, None);

    let subscriber_data = varatube
        .get_subscriber_data(USERS[0])
        .expect("Subscriber does not exist");
    let expected_subscriber_data = SubscriberData {
        currency_id: <[u8; 32]>::from(ft.id()).into(),
        period: Period::Month,
        subscription_start: Some((system.block_timestamp(), system.block_height())),
        renewal_date: Some((
            system.block_timestamp() + Period::Month.as_millis(),
            system.block_height() + Period::Month.to_blocks(1),
        )),
    };

    assert_eq!(
        subscriber_data, expected_subscriber_data,
        "Subscriber data do not match"
    );

    let blocks = Period::Month.to_blocks(1);
    system.spend_blocks(blocks);

    varatube.cancel_subscription(USERS[0], None);

    let subscriber_data = varatube
        .get_subscriber_data(USERS[0])
        .expect("Subscriber does not exist");

    let expected_subscriber_data = SubscriberData {
        currency_id: <[u8; 32]>::from(ft.id()).into(),
        period: Period::Month,
        subscription_start: Some((system.block_timestamp(), system.block_height())),
        renewal_date: None,
    };
    assert_eq!(
        subscriber_data, expected_subscriber_data,
        "Subscriber data do not match"
    );

    system.spend_blocks(blocks);
    let subscriber_data = varatube.get_subscriber_data(USERS[0]);

    assert_eq!(subscriber_data, None, "Subscriber data do not match");
}
