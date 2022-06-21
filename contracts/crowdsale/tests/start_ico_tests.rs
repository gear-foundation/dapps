use core::time::Duration;

use gstd::{Encode, String};
use gtest::{Program, System};

use ft_io::*;
use ico_io::*;

mod init_ico;
pub use init_ico::*;

#[test]
fn start_ico() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2);
}

#[test]
#[should_panic]
fn not_owner_start_ico() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let duration = Duration::from_secs(20).as_millis() as u64;
    let res = ico.send(
        USER_ID,
        IcoAction::StartSale {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP,
        },
    );

    assert!(res.contains(&(
        USER_ID,
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
fn second_start_ico() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1);
    start_sale(&ico, 1);
}

#[test]
#[should_panic]
fn zero_duration_start_ico() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 0);
}

#[test]
#[should_panic]
fn not_minting_tokens() {
    let sys = System::new();

    let ft = Program::from_file(
        &sys,
        "fungible-token/target/wasm32-unknown-unknown/release/fungible_token.wasm",
    );

    let res = ft.send(
        OWNER_ID,
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    );

    assert!(res.log().is_empty());

    let ico = Program::current(&sys);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: TOKEN_ADDRESS.into(),
            owner: OWNER_ID.into(),
        },
    );
    assert!(res.log().is_empty());

    start_sale(&ico, 1);
}
