#[cfg(test)]
extern crate std;
#[cfg(test)]
use std::println;

use codec::Encode;
use ft_io::*;
use gstd::String;
use gtest::{Program, System};
use lt_io::*;
const USERS: &[u64] = &[1, 2, 3, 4, 5];

fn init_lottery(sys: &System) {
    let lt = Program::current(sys);

    sys.mint_to(USERS[2], 10000);
    let res = lt.send_bytes_with_value(USERS[2], b"Init", 10000);

    assert!(res.log().is_empty());
}

fn init_fungible_token(sys: &System) {
    let ft = Program::from_file(sys, "./target/fungible_token.wasm");

    let res = ft.send(
        USERS[2],
        InitConfig {
            name: String::from("MyToken"),
            symbol: String::from("MTK"),
        },
    );

    assert!(res.log().is_empty());

    let res = ft.send(USERS[3], FTAction::Mint(1000));
    assert!(!res.main_failed());

    let res = ft.send(USERS[2], FTAction::BalanceOf(USERS[3].into()));
    assert!(res.contains(&(USERS[2], FTEvent::Balance(1000).encode())));

    let res = ft.send(USERS[4], FTAction::Mint(2000));
    assert!(!res.main_failed());

    let res = ft.send(USERS[2], FTAction::BalanceOf(USERS[4].into()));
    assert!(res.contains(&(USERS[2], FTEvent::Balance(2000).encode())));
}

#[test]
fn enter() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_lottery(&sys);
    sys.init_logger();
    let ft = sys.get_program(1);
    let lt = sys.get_program(2);

    let res = lt.send(
        USERS[2],
        LtAction::StartLottery {
            duration: 5000,
            token_address: Some(USERS[0].into()),
            participation_cost: 1000,
            prize_fund: 2000,
        },
    );
    assert!(res.log().is_empty());

    let res = ft.send(USERS[2], FTAction::TotalSupply);
    println!("TotalSupply(u128): {:?}", res.decoded_log::<FTEvent>());
    assert!(res.contains(&(USERS[2], FTEvent::TotalSupply(3000).encode())));

    sys.mint_to(USERS[3], 1000);
    let res = lt.send_with_value(USERS[3], LtAction::Enter(1000), 1000);
    assert!(res.contains(&(USERS[3], LtEvent::PlayerAdded(0).encode())));

    let res = ft.send(USERS[2], FTAction::BalanceOf(USERS[1].into()));
    println!("Balance(u128): {:?}", res.decoded_log::<FTEvent>());
    assert!(res.contains(&(USERS[2], FTEvent::Balance(1000).encode())));

    sys.mint_to(USERS[4], 1000);
    let res = lt.send_with_value(USERS[4], LtAction::Enter(1000), 1000);
    assert!(res.contains(&(USERS[4], LtEvent::PlayerAdded(1).encode())));

    let res = ft.send(USERS[2], FTAction::BalanceOf(USERS[1].into()));
    println!("Balance(u128): {:?}", res.decoded_log::<FTEvent>());
    assert!(res.contains(&(USERS[2], FTEvent::Balance(2000).encode())));
}

#[test]
fn pick_winner() {
    let sys = System::new();
    init_fungible_token(&sys);
    init_lottery(&sys);
    sys.init_logger();
    let ft = sys.get_program(1);
    let lt = sys.get_program(2);

    let res = lt.send(
        USERS[2],
        LtAction::StartLottery {
            duration: 5000,
            token_address: Some(USERS[0].into()),
            participation_cost: 1000,
            prize_fund: 2000,
        },
    );
    assert!(res.log().is_empty());

    sys.mint_to(USERS[3], 1000);
    let res = lt.send_with_value(USERS[3], LtAction::Enter(1000), 1000);
    assert!(res.contains(&(USERS[3], LtEvent::PlayerAdded(0).encode())));

    sys.mint_to(USERS[4], 1000);
    let res = lt.send_with_value(USERS[4], LtAction::Enter(1000), 1000);
    assert!(res.contains(&(USERS[4], LtEvent::PlayerAdded(1).encode())));

    sys.spend_blocks(5000);

    let res = lt.send(USERS[2], LtAction::PickWinner);

    println!("Winner index: {:?}", res.decoded_log::<LtEvent>());
    assert!(
        res.contains(&(USERS[2], LtEvent::Winner(0).encode()))
            || res.contains(&(USERS[2], LtEvent::Winner(1).encode()))
    );

    let res = ft.send(USERS[2], FTAction::BalanceOf(USERS[1].into()));
    println!("Balance(u128): {:?}", res.decoded_log::<FTEvent>());
    assert!(res.contains(&(USERS[2], FTEvent::Balance(0).encode())));
}
