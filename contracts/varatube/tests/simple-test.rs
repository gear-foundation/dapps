use ft_io::{FTAction, FTEvent, InitConfig};
use gstd::{ActorId, Encode};
use gtest::{Program, System};
use varatube_io::*;

const USERS: &[u64] = &[3, 4, 5];

#[test]
fn register_subscribe() {
    let sys = System::new();
    sys.init_logger();

    let ft = Program::from_file(
        &sys,
        "./target/wasm32-unknown-unknown/debug/fungible_token.opt.wasm",
    );

    ft.send(
        USERS[0],
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
            decimals: 18,
        },
    );

    let res = ft.send(USERS[0], FTAction::Mint(1000000));
    assert!(res.contains(&(
        USERS[0],
        FTEvent::Transfer {
            from: 0.into(),
            to: USERS[0].into(),
            amount: 1000000,
        }
        .encode()
    )));

    let varatube = Program::current(&sys);

    let token_id = ft.id().encode();
    let token_id = ActorId::from_slice(token_id.as_slice()).unwrap();

    let state: SubscriptionState = varatube.read_state().unwrap();
    assert!(state.subscribers.is_empty());
    assert!(state.currencies.is_empty());

    // Init
    let action: TokenData = (token_id, 666);
    varatube.send(USERS[0], action);

    let state: SubscriptionState = varatube.read_state().unwrap();
    assert!(state.subscribers.is_empty());
    assert!(!state.currencies.is_empty());

    let varatube_id = varatube.id().encode();
    let varatube_id = ActorId::from_slice(varatube_id.as_slice()).unwrap();

    let res = ft.send(
        USERS[0],
        FTAction::Approve {
            to: varatube_id,
            amount: 666,
        },
    );

    assert!(res.contains(&(
        USERS[0],
        FTEvent::Approve {
            from: USERS[0].into(),
            to: token_id,
            amount: 666,
        }
        .encode()
    )));

    // Register Subscription
    let action = Actions::RegisterSubscription {
        currency_id: token_id,
        period: Period::Month,
        with_renewal: false,
    };

    varatube.send(USERS[0], action);
    let state: SubscriptionState = varatube.read_state().unwrap();
    println!("subscribers = {:?}", state.subscribers);
    println!("currencies = {:?}", state.currencies);

    assert!(!state.subscribers.is_empty());
    assert!(!state.currencies.is_empty());
}
