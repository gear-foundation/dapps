mod utils;

use gstd::prelude::*;
use gtest::{Program, ProgramBuilder, System};
use roll_the_dice_io::*;
use utils::*;

// TODO: reimplement tests after fixing Oracle.

#[test]
#[ignore]
fn success_roll() {
    let sys = System::new();
    sys.init_logger();
    let oracle_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/release/oracle.opt.wasm")
            .with_id(ORACLE_ID)
            .build(&sys);
    let roll_dice_program = Program::current_with_id(&sys, ROLL_DICE_ID);

    let result = oracle_program.send(
        OWNER,
        oracle_io::InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );
    assert!(!result.main_failed());

    let result = roll_dice_program.send(
        OWNER,
        InitConfig {
            oracle: ORACLE_ID.into(),
        },
    );
    assert!(!result.main_failed());

    let result = roll_dice_program.send(USER, Action::Roll);
    assert!(!result.main_failed());
    assert!(!result.others_failed());
    // assert!(result.contains(&(USER, Event::RollValueRequested(1u128).encode())));
    println!("{:?}", result);
}

#[test]
#[ignore]
fn success_roll_finished() {
    let sys = System::new();
    sys.init_logger();

    let state_wasm = get_state();
    let oracle_program =
        ProgramBuilder::from_file("../target/wasm32-unknown-unknown/release/oracle.opt.wasm")
            .with_id(ORACLE_ID)
            .build(&sys);

    let roll_dice_program = Program::current_with_id(&sys, ROLL_DICE_ID);

    let result = oracle_program.send(
        OWNER,
        oracle_io::InitConfig {
            owner: OWNER.into(),
            manager: MANAGER.into(),
        },
    );
    assert!(!result.main_failed());

    let result = roll_dice_program.send(
        OWNER,
        InitConfig {
            oracle: ORACLE_ID.into(),
        },
    );
    assert!(!result.main_failed());

    let result = roll_dice_program.send(USER, Action::Roll);
    assert!(!result.main_failed());
    assert!(!result.others_failed());
    assert!(result.contains(&(USER, Event::RollValueRequested(1u128).encode())));

    let meta_result: StateResponse = roll_dice_program
        .read_state_using_wasm(
            0,
            "query",
            state_wasm.clone(),
            Some(StateQuery::GetUsersData),
        )
        .unwrap();
    match meta_result {
        StateResponse::UsersData(users_data) => {
            assert_eq!(users_data[0].0, 1u128);
            assert_eq!(users_data[0].1, USER.into());
            assert_eq!(users_data[0].2, RollStatus::Rolling);
        }
    }

    sys.spend_blocks(2);

    /* let result = oracle_program.send(
        MANAGER,
        oracle_io::Action::UpdateValue { id: 1, value: 1337 },
    );
    assert!(!result.main_failed());
    assert!(!result.others_failed()); */

    let meta_result: StateResponse = roll_dice_program
        .read_state_using_wasm(0, "query", state_wasm, Some(StateQuery::GetUsersData))
        .unwrap();
    match meta_result {
        StateResponse::UsersData(users_data) => {
            assert_eq!(users_data[0].0, 1u128);
            assert_eq!(users_data[0].1, USER.into());
            assert_eq!(users_data[0].2, RollStatus::Finished(false));
        }
    }
}
