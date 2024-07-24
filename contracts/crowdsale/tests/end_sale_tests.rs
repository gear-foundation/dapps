mod init_ico;

use crowdsale_io::*;
use gtest::System;
pub use init_ico::*;

use gstd::Encode;

// TODO: fix test
#[test]
#[ignore]
fn end_sale_no_time_left() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2).unwrap();

    start_sale(&ico, 1, 0);

    sys.spend_blocks(1001);

    end_sale(&ico, 1);
}

// TODO: fix test
#[test]
#[ignore]
fn end_sale_zero_tokens() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2).unwrap();

    start_sale(&ico, 1, 0);

    let amount: u128 = TOKENS_CNT;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    end_sale(&ico, 1);
}

#[test]
#[should_panic]
fn end_sale_time_and_tokens_left() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2).unwrap();

    start_sale(&ico, 1, 0);

    let amount: u128 = TOKENS_CNT - 5;
    buy_tokens(&sys, &ico, amount, amount * START_PRICE);

    end_sale(&ico, 1);
}

#[test]
#[should_panic]
fn not_owner_end_sale() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2).unwrap();

    start_sale(&ico, 1, 0);

    sys.spend_blocks(1001);

    let res = ico.send(USER_ID, IcoAction::EndSale);
    assert!(!res.main_failed());
    assert!(res.contains(&(USER_ID, IcoEvent::SaleEnded(1).encode())));
}

#[test]
#[should_panic]
fn end_sale_before_start() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2).unwrap();

    end_sale(&ico, 0);
}

#[test]
#[should_panic]
fn end_sale_twice() {
    let sys = System::new();
    init(&sys);

    let ico = sys.get_program(2).unwrap();

    start_sale(&ico, 1, 0);

    sys.spend_blocks(1001);

    end_sale(&ico, 1);
    end_sale(&ico, 2);
}
