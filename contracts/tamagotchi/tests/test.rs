use gstd::prelude::*;
use gtest::{Program, System};
use tamagotchi_io::{Error, Tamagotchi, TmgAction, TmgInit, TmgReply};

pub const USERS: [u64; 3] = [5, 6, 7];

fn init_tamagotchi(sys: &System) {
    let tamagotchi = Program::current(sys);

    let res = tamagotchi.send(
        USERS[0],
        TmgInit {
            name: "MyName".to_string(),
        },
    );
    assert!(!res.main_failed());
}

#[test]
fn success_test() {
    let system = System::new();
    system.init_logger();
    init_tamagotchi(&system);
    let tamagotchi = system.get_program(1);

    let time_sec = 20_000;
    let time_ms = time_sec * 1_000;

    system.spend_blocks(time_sec);

    let res = tamagotchi.send(USERS[0], TmgAction::Age);
    assert!(!res.main_failed());
    let reply = res.log()[0].payload();
    let expected_reply = Ok::<TmgReply, Error>(TmgReply::Age(time_ms.into())).encode();
    assert_eq!(reply, expected_reply);

    // system.spend_blocks(1000);
    let res = tamagotchi.send(USERS[0], TmgAction::Feed);
    assert!(!res.main_failed());
    let res = tamagotchi.send(USERS[0], TmgAction::Play);
    assert!(!res.main_failed());
    let res = tamagotchi.send(USERS[0], TmgAction::Sleep);
    assert!(!res.main_failed());
    let state: Tamagotchi = tamagotchi.read_state(0).unwrap();
    assert_eq!(state.fed, 90_000, "Wrong fed");
    assert_eq!(state.entertained, 70_000, "Wrong entertained");
    assert_eq!(state.rested, 70_000, "Wrong rested");
    println!("STATE: {:?}", state);
}

#[test]
fn failures_test() {
    let system = System::new();
    system.init_logger();
    init_tamagotchi(&system);
    let tamagotchi = system.get_program(1);

    let time_sec = 100_000;
    let time_ms = time_sec * 1_000;

    system.spend_blocks(time_sec);

    let res = tamagotchi.send(USERS[0], TmgAction::Age);
    assert!(!res.main_failed());
    let reply = res.log()[0].payload();
    let expected_reply = Err::<TmgReply, Error>(Error::TamagotchiHasDied).encode();
    assert_eq!(reply, expected_reply);
}
