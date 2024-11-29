pub mod utils;

use gstd::prelude::*;
use gtest::System;
use oracle_io::*;
use utils::*;

#[test]
fn success_init() {
    let sys = System::new();
    let oracle_program = load_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);

    let mid = oracle_program.send(
        OWNER,
        InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    let oracle_state: Oracle = oracle_program.read_state(0).expect("Invalid state.");

    assert_eq!(oracle_state.owner, OWNER.into());
    assert_eq!(oracle_state.manager, MANAGER.into());
}

#[test]
fn success_change_manager() {
    let sys = System::new();
    let oracle_program = load_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);

    oracle_program.send(
        OWNER,
        InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );

    oracle_program.send(OWNER, Action::ChangeManager(NEW_MANAGER.into()));
    let res = sys.run_next_block();
    assert!(res.contains(&(OWNER, Event::NewManager(NEW_MANAGER.into()).encode())));

    oracle_program.send(OWNER, Action::ChangeManager(OWNER.into()));
    let res = sys.run_next_block();
    assert!(res.contains(&(OWNER, Event::NewManager(OWNER.into()).encode())));
}

#[test]
fn fail_change_manager_invalid_owner() {
    let sys = System::new();
    let oracle_program = load_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);
    sys.mint_to(FAKE_OWNER, 100_000_000_000_000);

    oracle_program.send(
        OWNER,
        InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );

    let mid = oracle_program.send(FAKE_OWNER, Action::ChangeManager(FAKE_MANAGER.into()));
    let res = sys.run_next_block();
    assert!(res.failed.contains(&mid));
}
