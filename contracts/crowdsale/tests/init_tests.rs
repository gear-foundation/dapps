use core::time::Duration;

use gstd::Encode;
use gtest::{Program, System};

use ico_io::*;

mod init_ico;
pub use init_ico::*;

#[test]
fn test_init() {
    let sys = System::new();
    init(&sys);
}

#[test]
#[should_panic]
fn zero_owner_id_init() {
    let sys = System::new();
    sys.init_logger();

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: TOKEN_ADDRESS.into(),
            owner: ZERO_ID,
        },
    );

    assert!(res.log().is_empty());
}

#[test]
#[should_panic]
fn zero_token_address_init() {
    let sys = System::new();
    sys.init_logger();

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: ZERO_ID,
            owner: OWNER_ID.into(),
        },
    );

    assert!(res.log().is_empty());

    let duration = Duration::from_secs(2).as_millis() as u64;
    let res = ico.send(
        OWNER_ID,
        IcoAction::StartSale {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP,
        },
    );

    assert!(res.contains(&(
        OWNER_ID,
        IcoEvent::SaleStarted {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP,
        }
        .encode()
    )));
}

#[test]
#[should_panic]
fn zero_tokens_goal_init() {
    let sys = System::new();
    sys.init_logger();

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: TOKEN_ADDRESS.into(),
            owner: OWNER_ID.into(),
        },
    );

    assert!(res.log().is_empty());

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(
        OWNER_ID,
        IcoAction::StartSale {
            duration,
            start_price: START_PRICE,
            tokens_goal: 0,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP,
        },
    );

    assert!(res.contains(&(
        OWNER_ID,
        IcoEvent::SaleStarted {
            duration,
            start_price: START_PRICE,
            tokens_goal: 0,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP,
        }
        .encode()
    )));
}

#[test]
#[should_panic]
fn zero_start_price_init() {
    let sys = System::new();
    sys.init_logger();

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: TOKEN_ADDRESS.into(),
            owner: OWNER_ID.into(),
        },
    );

    assert!(res.log().is_empty());

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(
        OWNER_ID,
        IcoAction::StartSale {
            duration,
            start_price: 0,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP,
        },
    );

    assert!(res.contains(&(
        OWNER_ID,
        IcoEvent::SaleStarted {
            duration,
            start_price: 0,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP,
        }
        .encode()
    )));
}

#[test]
#[should_panic]
fn zero_price_increase_init() {
    let sys = System::new();
    sys.init_logger();

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: TOKEN_ADDRESS.into(),
            owner: OWNER_ID.into(),
        },
    );

    assert!(res.log().is_empty());

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(
        OWNER_ID,
        IcoAction::StartSale {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: 0,
            time_increase_step: TIME_INCREASE_STEP,
        },
    );

    assert!(res.contains(&(
        OWNER_ID,
        IcoEvent::SaleStarted {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: 0,
            time_increase_step: TIME_INCREASE_STEP,
        }
        .encode()
    )));
}

#[test]
#[should_panic]
fn zero_time_increase_init() {
    let sys = System::new();
    sys.init_logger();

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: TOKEN_ADDRESS.into(),
            owner: OWNER_ID.into(),
        },
    );

    assert!(res.log().is_empty());

    let duration = Duration::from_secs(1).as_millis() as u64;
    let res = ico.send(
        OWNER_ID,
        IcoAction::StartSale {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: 0,
        },
    );

    assert!(res.contains(&(
        OWNER_ID,
        IcoEvent::SaleStarted {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: 0,
        }
        .encode()
    )));
}
