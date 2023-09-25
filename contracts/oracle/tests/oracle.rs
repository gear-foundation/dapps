pub mod utils;

use gstd::prelude::*;
use gtest::System;
use oracle_io::*;
use utils::*;

#[test]
fn success_init() {
    let sys = System::new();
    let oracle_program = load_program(&sys);

    let result = oracle_program.send(
        OWNER,
        InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );
    assert!(!result.main_failed());

    let oracle_state: Oracle = oracle_program.read_state().expect("Invalid state.");

    assert_eq!(oracle_state.owner, OWNER.into());
    assert_eq!(oracle_state.manager, MANAGER.into());
}

#[test]
fn success_change_manager() {
    let sys = System::new();
    let oracle_program = load_program(&sys);

    oracle_program.send(
        OWNER,
        InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );

    let result = oracle_program.send(OWNER, Action::ChangeManager(NEW_MANAGER.into()));
    assert!(result.contains(&(OWNER, Event::NewManager(NEW_MANAGER.into()).encode())));

    let result = oracle_program.send(OWNER, Action::ChangeManager(OWNER.into()));
    assert!(result.contains(&(OWNER, Event::NewManager(OWNER.into()).encode())));
}

#[test]
fn fail_change_manager_invalid_owner() {
    let sys = System::new();
    let oracle_program = load_program(&sys);

    oracle_program.send(
        OWNER,
        InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );

    let result = oracle_program.send(FAKE_OWNER, Action::ChangeManager(FAKE_MANAGER.into()));
    assert!(result.main_failed());
}
