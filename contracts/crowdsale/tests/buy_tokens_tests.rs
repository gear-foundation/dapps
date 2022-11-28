mod init_ico;

use crowdsale::io::{IcoAction, IcoEvent};
use gstd::Encode;
use gtest::System;
pub use init_ico::*;

#[test]
fn common_buy_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
fn buy_tokens_with_change() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = 5;
    let change = 600;
    let res = ico.send_with_value(
        USER_ID,
        IcoAction::Buy(amount),
        amount * START_PRICE + change,
    );
    assert!(res.contains(&(
        USER_ID,
        (IcoEvent::Bought {
            buyer: USER_ID.into(),
            amount,
            change
        })
        .encode()
    )));
}

#[test]
fn buy_tokens_after_price_update() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    sys.spend_blocks(TIME_INCREASE_STEP as _);

    let amount: u128 = 5;
    buy_tokens(
        &sys,
        &ico,
        amount,
        amount * (START_PRICE + PRICE_INCREASE_STEP),
    );

    sys.spend_blocks((TIME_INCREASE_STEP - 1) as _);

    buy_tokens(
        &sys,
        &ico,
        amount,
        amount * (START_PRICE + PRICE_INCREASE_STEP),
    );

    sys.spend_blocks(1);

    buy_tokens(
        &sys,
        &ico,
        amount,
        amount * (START_PRICE + PRICE_INCREASE_STEP * 2),
    );
}

#[test]
#[should_panic]
fn buy_when_no_time_left() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    sys.spend_blocks(3000); // 3 sec

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
#[should_panic]
fn wrong_value_sent() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE - 1);
}

#[test]
#[should_panic]
fn wrong_value_after_price_update() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    sys.spend_blocks(
        (TIME_INCREASE_STEP + 1)
            .try_into()
            .expect("Can't cast type"),
    );

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
#[should_panic]
fn all_tokens_bought() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = TOKENS_CNT;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    buy_tokens(&sys, &ico, 1, START_PRICE);
}

#[test]
#[should_panic]
fn buy_before_start() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
#[should_panic]
fn buy_after_end_sale() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    sys.spend_blocks(1001);

    end_sale(&ico, 1);

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
#[should_panic]
fn buy_more_than_goal_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = TOKENS_CNT + 1;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
#[should_panic]
fn buy_too_many_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    let amount: u128 = TOKENS_CNT - 4;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
#[should_panic]
fn buy_zero_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = 0;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);
}

#[test]
#[should_panic]
fn overflowing_multiplication_buy() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 2, 0);

    let amount: u128 = u128::MAX / START_PRICE + 1;
    buy_tokens(&sys, &ico, amount, 544);
}
