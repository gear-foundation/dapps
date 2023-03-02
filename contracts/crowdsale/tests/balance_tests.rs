mod init_ico;

use crowdsale_io::*;
use gtest::System;
pub use init_ico::*;

#[test]
fn balance_after_two_purchases() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, (TIME_INCREASE_STEP + 1) as _, 0);

    balance_of(&ico, 0);

    let amount: u128 = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    balance_of(&ico, amount);

    sys.spend_blocks((TIME_INCREASE_STEP + 1) as _);

    buy_tokens(
        &sys,
        &ico,
        amount,
        amount * (START_PRICE + PRICE_INCREASE_STEP),
    );

    balance_of(&ico, amount * 2);
}

#[test]
fn owner_balance() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1, 0);

    let amount = 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    sys.spend_blocks(1001);

    let res: State = ico.read_state().unwrap();
    assert_eq!(0, res.balance_of(&OWNER_ID.into()), "Error in balance_of()");

    end_sale(&ico, 1);

    let res: State = ico.read_state().unwrap();
    assert_eq!(
        TOKENS_CNT - amount,
        res.balance_of(&OWNER_ID.into()),
        "Error in balance_of()"
    );
}
