mod token;

use core::time::Duration;
use gstd::{prelude::*, ActorId, Encode};
use gtest::{Program, System};
use ico_io::*;
pub use token::*;

pub const TOKEN_ADDRESS: u64 = 1;
pub const ICO_CONTRACT_ID: u64 = 2;
pub const OWNER_ID: u64 = 100001;
pub const USER_ID: u64 = 12345;

pub const ZERO_ID: ActorId = ActorId::zero();

pub const TOKENS_CNT: u128 = 100;
pub const START_PRICE: u128 = 1000;
pub const PRICE_INCREASE_STEP: u128 = 100;
pub const TIME_INCREASE_STEP: u128 = 1000;

fn init_ico(sys: &System) {
    let ico = Program::current_with_id(sys, ICO_CONTRACT_ID);

    let res = ico.send(
        OWNER_ID,
        IcoInit {
            token_address: TOKEN_ADDRESS.into(),
            owner: OWNER_ID.into(),
        },
    );
    assert!(res.log().is_empty());
}

pub fn init(sys: &System) {
    sys.init_logger();

    let ft = Program::ftoken(OWNER_ID, TOKEN_ADDRESS, sys);
    ft.mint(0, OWNER_ID, OWNER_ID, TOKENS_CNT, false);
    ft.approve(1, OWNER_ID, ICO_CONTRACT_ID, TOKENS_CNT, false);

    init_ico(sys);
    sys.mint_to(USER_ID, 100_000);
}

pub fn start_sale(ico: &Program, ico_duration: u64, expected_tx_id: u64) {
    let duration = Duration::from_secs(ico_duration).as_millis() as u64 * 1000;
    let res = ico.send(
        OWNER_ID,
        IcoAction::StartSale {
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP * 1000,
        },
    );

    assert!(res.contains(&(
        OWNER_ID,
        IcoEvent::SaleStarted {
            transaction_id: expected_tx_id,
            duration,
            start_price: START_PRICE,
            tokens_goal: TOKENS_CNT,
            price_increase_step: PRICE_INCREASE_STEP,
            time_increase_step: TIME_INCREASE_STEP * 1000,
        }
        .encode()
    )));
}

pub fn end_sale(ico: &Program, expected_tx_id: u64) {
    let res = ico.send(OWNER_ID, IcoAction::EndSale);
    assert!(res.contains(&(OWNER_ID, IcoEvent::SaleEnded(expected_tx_id).encode())));
}

pub fn buy_tokens(_sys: &System, ico: &Program, amount: u128, price: u128) {
    // TODO: Uncomment after updating to the latest `gtest`
    // https://github.com/gear-dapps/crowdsale-ico/issues/8
    // sys.mint_to(USER_ID, price);
    let res = ico.send_with_value(USER_ID, IcoAction::Buy(amount), price);
    assert!(res.contains(&(
        USER_ID,
        (IcoEvent::Bought {
            buyer: USER_ID.into(),
            amount,
            change: 0
        })
        .encode()
    )));
}

pub fn balance_of(ico: &Program, amount: u128) {
    let res: StateIcoReply = ico
        .meta_state(StateIco::BalanceOf(USER_ID.into()))
        .expect("Error in meta_state");

    if let StateIcoReply::BalanceOf { address, balance } = res {
        assert!(
            address == USER_ID.into() && balance == amount,
            "Error in balance_of()"
        );
    }
}
