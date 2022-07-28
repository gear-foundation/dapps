use gstd::Encode;
use gtest::System;

use ico_io::*;

mod init_ico;
pub use init_ico::*;

#[test]
fn end_sale_no_time_left() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1);

    sys.spend_blocks(1001);

    end_sale(&ico);
}

#[test]
fn end_sale_zero_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1);

    let amount: u128 = TOKENS_CNT;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    end_sale(&ico);
}

#[test]
#[should_panic]
fn end_sale_time_and_tokens_left() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1);

    let amount: u128 = TOKENS_CNT - 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    end_sale(&ico);
}

#[test]
#[should_panic]
fn not_owner_end_sale() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1);

    sys.spend_blocks(1001);

    let res = ico.send(USER_ID, IcoAction::EndSale);
    assert!(res.contains(&(USER_ID, IcoEvent::SaleEnded.encode())));
}

#[test]
#[should_panic]
fn end_sale_before_start() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    end_sale(&ico);
}

#[test]
#[should_panic]
fn end_sale_twice() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2);

    start_sale(&ico, 1);

    sys.spend_blocks(1001);

    end_sale(&ico);
    end_sale(&ico);
}
