pub mod utils;

use gstd::prelude::*;
use gtest::System;
use oracle_randomness_io::{state::*, *};
use utils::*;

#[test]
fn success_init() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);
    sys.mint_to(MANAGER, 100_000_000_000_000);
    let mid = oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

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
    sys.mint_to(OWNER, 100_000_000_000_000);
    oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );

    oracle_program.send(OWNER, Action::UpdateManager(NEW_MANAGER.into()));
    let res = sys.run_next_block();
    assert!(res.contains(&(OWNER, Event::NewManager(NEW_MANAGER.into()).encode())));

    oracle_program.send(OWNER, Action::UpdateManager(OWNER.into()));
    let res = sys.run_next_block();
    assert!(res.contains(&(OWNER, Event::NewManager(OWNER.into()).encode())));
}

#[test]
fn success_set_random_value() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);
    sys.mint_to(MANAGER, 100_000_000_000_000);
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

    oracle_program.send(
        MANAGER,
        Action::SetRandomValue {
            round: 1,
            value: value.clone(),
        },
    );
    let res = sys.run_next_block();
    assert!(res.contains(&(MANAGER, Event::NewRandomValue { round: 1, value }.encode())));
}

#[test]
fn fail_set_random_value_invalid_manager() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);
    sys.mint_to(FAKE_MANAGER, 100_000_000_000_000);
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

    let mid = oracle_program.send(FAKE_MANAGER, Action::SetRandomValue { round: 1, value });
    let res = sys.run_next_block();
    assert!(res.failed.contains(&mid));
}

#[test]
fn fail_set_random_value_invalid_round() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);
    sys.mint_to(MANAGER, 100_000_000_000_000);

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

    let mid = oracle_program.send(
        MANAGER,
        Action::SetRandomValue {
            round: 1,
            value: value.clone(),
        },
    );
    let res = sys.run_next_block();
    assert!(res.succeed.contains(&mid));

    let mid = oracle_program.send(MANAGER, Action::SetRandomValue { round: 1, value });
    let res = sys.run_next_block();
    assert!(res.failed.contains(&mid));
}

#[test]
fn fail_update_manager_invalid_owner() {
    let sys = System::new();
    let oracle_program = load_randomness_program(&sys);
    sys.mint_to(OWNER, 100_000_000_000_000);
    sys.mint_to(FAKE_OWNER, 100_000_000_000_000);

    oracle_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
        },
    );

    let mid = oracle_program.send(FAKE_OWNER, Action::UpdateManager(NEW_MANAGER.into()));
    let res = sys.run_next_block();
    assert!(res.failed.contains(&mid));
}
