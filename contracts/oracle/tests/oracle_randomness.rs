pub mod utils;

use gstd::prelude::*;
use gtest::System;
use oracle_randomness_io::{state::*, *};
use utils::*;

#[test]
fn success_init() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);

    let result = oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );
    assert!(!result.main_failed());

    let oracle_state: RandomnessOracle = oracle_program
        .read_state(0)
        .expect("Unexpected invalid oracle state.");

    assert_eq!(oracle_state.owner, OWNER.into());
    assert_eq!(oracle_state.manager, MANAGER.into());
    assert!(oracle_state.values.is_empty());
    assert_eq!(oracle_state.last_round, 0);
}

#[test]
fn success_update_manager() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);

    oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );

    let result = oracle_program.send(OWNER, Action::UpdateManager(NEW_MANAGER.into()));
    assert!(result.contains(&(OWNER, Event::NewManager(NEW_MANAGER.into()).encode())));

    let result = oracle_program.send(OWNER, Action::UpdateManager(OWNER.into()));
    assert!(result.contains(&(OWNER, Event::NewManager(OWNER.into()).encode())));
}

#[test]
fn success_set_random_value() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);

    oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );

    let value = Random {
        randomness: (1337, 133700000000001337),
        signature: String::from(""),
        prev_signature: String::from(""),
    };

    let result = oracle_program.send(
        MANAGER,
        Action::SetRandomValue {
            round: 1,
            value: value.clone(),
        },
    );
    assert!(result.contains(&(MANAGER, Event::NewRandomValue { round: 1, value }.encode())));
}

#[test]
fn fail_set_random_value_invalid_manager() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);

    oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );

    let value = Random {
        randomness: (0, 0),
        signature: String::from(""),
        prev_signature: String::from(""),
    };

    let result = oracle_program.send(FAKE_MANAGER, Action::SetRandomValue { round: 1, value });
    assert!(result.main_failed());
}

#[test]
fn fail_set_random_value_invalid_round() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);

    oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );

    let value = Random {
        randomness: (0, 0),
        signature: String::from(""),
        prev_signature: String::from(""),
    };

    let result = oracle_program.send(
        MANAGER,
        Action::SetRandomValue {
            round: 1,
            value: value.clone(),
        },
    );
    assert!(!result.main_failed());

    let result = oracle_program.send(MANAGER, Action::SetRandomValue { round: 1, value });
    assert!(result.main_failed());
}

#[test]
fn fail_update_manager_invalid_owner() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);

    oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );

    let result = oracle_program.send(FAKE_OWNER, Action::UpdateManager(NEW_MANAGER.into()));
    assert!(result.main_failed());
}
