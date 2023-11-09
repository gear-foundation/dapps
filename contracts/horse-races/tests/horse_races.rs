mod utils;

use gstd::collections::BTreeMap;
use gtest::System;
use horse_races_io::*;
use utils::*;

#[test]
fn success_init() {
    let sys = System::new();
    let state_wasm = get_state();

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetRuns))
        .unwrap();
    match meta_state {
        MetaResponse::Runs(runs) => assert!(runs.is_empty()),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetManager))
        .unwrap();
    match meta_state {
        MetaResponse::Manager(manager) => assert_eq!(manager, MANAGER.into()),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetOwner))
        .unwrap();
    match meta_state {
        MetaResponse::Owner(owner) => assert_eq!(owner, OWNER.into()),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetToken))
        .unwrap();
    match meta_state {
        MetaResponse::Token(token) => assert_eq!(token, TOKEN_ID.into()),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetOracle))
        .unwrap();
    match meta_state {
        MetaResponse::Oracle(oracle) => assert_eq!(oracle, ORACLE_ID.into()),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetFeeBps))
        .unwrap();
    match meta_state {
        MetaResponse::FeeBps(fee_bps) => assert_eq!(fee_bps, FEE_BPS),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetRunNonce))
        .unwrap();
    match meta_state {
        MetaResponse::RunNonce(run_nonce) => assert_eq!(run_nonce, 0),
        _ => std::panic!("Invalid meta state!"),
    }
}

#[test]
fn success_update() {
    let sys = System::new();
    let state_wasm = get_state();

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let result = horse_races_program.send(MANAGER, Action::UpdateManager(NEW_MANAGER.into()));
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::ManagerUpdated(NEW_MANAGER.into()).encode())));

    let result = horse_races_program.send(NEW_MANAGER, Action::UpdateFeeBps(NEW_FEE_BPS));
    assert!(!result.main_failed());
    assert!(result.contains(&(NEW_MANAGER, Event::FeeBpsUpdated(NEW_FEE_BPS).encode())));

    let result = horse_races_program.send(NEW_MANAGER, Action::UpdateOracle(NEW_ORACLE.into()));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        NEW_MANAGER,
        Event::OracleUpdated(NEW_ORACLE.into()).encode()
    )));

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetManager))
        .unwrap();
    match meta_state {
        MetaResponse::Manager(manager) => assert_eq!(manager, NEW_MANAGER.into()),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetOracle))
        .unwrap();
    match meta_state {
        MetaResponse::Oracle(oracle) => assert_eq!(oracle, NEW_ORACLE.into()),
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetFeeBps))
        .unwrap();
    match meta_state {
        MetaResponse::FeeBps(fee_bps) => assert_eq!(fee_bps, NEW_FEE_BPS),
        _ => std::panic!("Invalid meta state!"),
    }
}

#[test]
fn success_create_run() {
    let sys = System::new();
    let state_wasm = get_state();

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let start_timestamp = sys.block_timestamp();
    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses: horses.clone(),
        },
    );
    assert!(!result.main_failed());
    assert!(result.contains(&(
        MANAGER,
        Event::RunCreated {
            run_id: 1,
            bidding_duration_ms: 1000,
            horses
        }
        .encode()
    )));

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm.clone(), Some(MetaQuery::GetRuns))
        .unwrap();
    match meta_state {
        MetaResponse::Runs(runs) => {
            assert!(!runs.is_empty());

            let (id, run) = &runs[0];

            assert_eq!(*id, 1);
            assert_eq!(run.start_timestamp, start_timestamp);
            assert_eq!(run.end_bidding_timestamp, start_timestamp + 1000);
            assert_eq!(
                *run.horses.get("Pegasus").unwrap(),
                (Horse { max_speed: 50 }, 0u128)
            );
            assert_eq!(
                *run.horses.get("Max").unwrap(),
                (Horse { max_speed: 65 }, 0u128)
            );
            assert_eq!(
                *run.horses.get("Vitalik").unwrap(),
                (Horse { max_speed: 80 }, 0u128)
            );
            assert!(run.bidders.is_empty());
            assert_eq!(run.status, RunStatus::Created);
        }
        _ => std::panic!("Invalid meta state!"),
    }

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetRunNonce))
        .unwrap();
    match meta_state {
        MetaResponse::RunNonce(run_nonce) => assert_eq!(run_nonce, 1),
        _ => std::panic!("Invalid meta state!"),
    }
}

#[test]
fn success_bid() {
    let sys = System::new();
    let state_wasm = get_state();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER,
        Event::NewBid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount_after_fees
        }
        .encode()
    )));

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetRuns))
        .unwrap();
    match meta_state {
        MetaResponse::Runs(runs) => {
            assert!(!runs.is_empty());

            let (id, run) = &runs[0];

            assert_eq!(*id, 1);
            assert!(!run.bidders.is_empty());
            assert_eq!(
                *run.horses.get(&String::from("Max")).unwrap(),
                (Horse { max_speed: 65 }, user_deposit_amount_after_fees)
            );
            assert_eq!(
                *run.bidders.get(&USER.into()).unwrap(),
                (String::from("Max"), user_deposit_amount_after_fees)
            );

            let result = token_program.send(
                OWNER,
                fungible_token_io::FTAction::BalanceOf(MANAGER.into()),
            );
            assert!(!result.main_failed());
            assert!(result.contains(&(
                OWNER,
                fungible_token_io::FTEvent::Balance(user_deposit_fees_amount).encode()
            )));
        }
        _ => std::panic!("Invalid meta state!"),
    }
}

#[test]
fn success_cancel_last_run() {
    let sys = System::new();
    let state_wasm = get_state();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::CancelLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunCanceled(1).encode())));

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetRuns))
        .unwrap();
    match meta_state {
        MetaResponse::Runs(runs) => {
            assert!(!runs.is_empty());

            let (id, run) = &runs[0];

            assert_eq!(*id, 1);
            assert_eq!(run.status, RunStatus::Canceled);
        }
        _ => std::panic!("Invalid meta state!"),
    }
}

#[test]
fn success_progress_last_run() {
    let sys = System::new();
    let state_wasm = get_state();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 2, 2141258129341892512471);

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::ProgressLastRun);
    assert!(!result.main_failed());
    assert!(!result.others_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunProgressed(1).encode())));

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetRuns))
        .unwrap();
    match meta_state {
        MetaResponse::Runs(runs) => {
            assert!(!runs.is_empty());

            let (id, run) = &runs[0];

            assert_eq!(*id, 1);
            assert_eq!(run.status, RunStatus::InProgress { oracle_round: 2 });
        }
        _ => std::panic!("Invalid meta state!"),
    }
}

#[test]
fn success_finish_last_run() {
    let state_wasm = get_state();
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::ProgressLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunProgressed(1).encode())));

    set_oracle_value(&oracle_program, 2, 2141258129341892512471);

    let result = horse_races_program.send(MANAGER, Action::FinishLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(
        MANAGER,
        Event::LastRunFinished {
            run_id: 1,
            winner: (
                String::from("Max"),
                Horse { max_speed: 65 },
                user_deposit_amount_after_fees
            )
        }
        .encode()
    )));

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetRuns))
        .unwrap();
    match meta_state {
        MetaResponse::Runs(runs) => {
            assert!(!runs.is_empty());

            let (id, run) = &runs[0];

            assert_eq!(*id, 1);
            assert_eq!(
                run.status,
                RunStatus::Finished {
                    horse_name: String::from("Max"),
                    run_id: 1
                }
            );
        }
        _ => std::panic!("Invalid meta state!"),
    }
}

#[test]
fn success_withdraw_finished() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::ProgressLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunProgressed(1).encode())));

    set_oracle_value(&oracle_program, 2, 2141258129341892512471);

    let result = horse_races_program.send(MANAGER, Action::FinishLastRun);
    assert!(!result.main_failed());

    let result = horse_races_program.send(USER, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER,
        Event::NewWithdrawFinished {
            user: USER.into(),
            run_id: 1,
            amount: user_deposit_amount_after_fees,
            profit_amount: 0
        }
        .encode()
    )));

    let result = token_program.send(OWNER, fungible_token_io::FTAction::BalanceOf(USER.into()));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        OWNER,
        fungible_token_io::FTEvent::Balance(user_deposit_amount_after_fees).encode()
    )));
}

#[test]
fn success_withdraw_finished_more_users() {
    let sys = System::new();
    let state_wasm = get_state();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let user_1_deposit_amount: u128 = 1000000;
    let user_1_deposit_fees_amount: u128 =
        user_1_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_1_deposit_amount_after_fees: u128 = user_1_deposit_amount - user_1_deposit_fees_amount;

    let user_2_deposit_amount: u128 = 1000000;
    let user_2_deposit_fees_amount: u128 =
        user_2_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_2_deposit_amount_after_fees: u128 = user_2_deposit_amount - user_2_deposit_fees_amount;

    let user_3_deposit_amount: u128 = 1000000;
    let user_3_deposit_fees_amount: u128 =
        user_3_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_3_deposit_amount_after_fees: u128 = user_3_deposit_amount - user_3_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);
    mint_token(&token_program, USER_1, user_1_deposit_amount);
    mint_token(&token_program, USER_2, user_2_deposit_amount);
    mint_token(&token_program, USER_3, user_3_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_1,
        horse_races_program.id().into_bytes().into(),
        user_1_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_1,
        Action::Bid {
            horse_name: String::from("Pegasus"),
            amount: user_1_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_2,
        horse_races_program.id().into_bytes().into(),
        user_2_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_2,
        Action::Bid {
            horse_name: String::from("Vitalik"),
            amount: user_2_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_3,
        horse_races_program.id().into_bytes().into(),
        user_3_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_3,
        Action::Bid {
            horse_name: String::from("Vitalik"),
            amount: user_3_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    let meta_state: MetaResponse = horse_races_program
        .read_state_using_wasm(0, "query", state_wasm, Some(MetaQuery::GetRuns))
        .unwrap();
    match meta_state {
        MetaResponse::Runs(runs) => {
            assert!(!runs.is_empty());

            let (id, run) = &runs[0];

            assert_eq!(*id, 1);
            assert!(!run.bidders.is_empty());
            assert_eq!(
                *run.horses.get(&String::from("Max")).unwrap(),
                (Horse { max_speed: 65 }, user_deposit_amount_after_fees)
            );
            assert_eq!(
                *run.horses.get(&String::from("Pegasus")).unwrap(),
                (Horse { max_speed: 50 }, user_1_deposit_amount_after_fees)
            );
            assert_eq!(
                *run.horses.get(&String::from("Vitalik")).unwrap(),
                (
                    Horse { max_speed: 80 },
                    user_2_deposit_amount_after_fees + user_3_deposit_amount_after_fees
                )
            );

            assert_eq!(
                *run.bidders.get(&USER.into()).unwrap(),
                (String::from("Max"), user_deposit_amount_after_fees)
            );
            assert_eq!(
                *run.bidders.get(&USER_1.into()).unwrap(),
                (String::from("Pegasus"), user_1_deposit_amount_after_fees)
            );
            assert_eq!(
                *run.bidders.get(&USER_2.into()).unwrap(),
                (String::from("Vitalik"), user_2_deposit_amount_after_fees)
            );
            assert_eq!(
                *run.bidders.get(&USER_3.into()).unwrap(),
                (String::from("Vitalik"), user_3_deposit_amount_after_fees)
            );
        }
        _ => std::panic!("Invalid meta state!"),
    }

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::ProgressLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunProgressed(1).encode())));

    set_oracle_value(&oracle_program, 2, 2141258129341892512471);

    let result = horse_races_program.send(MANAGER, Action::FinishLastRun);
    assert!(!result.main_failed());

    let result = horse_races_program.send(USER, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER,
        Event::NewWithdrawFinished {
            user: USER.into(),
            run_id: 1,
            amount: user_deposit_amount_after_fees,
            profit_amount: user_1_deposit_amount_after_fees
                + user_2_deposit_amount_after_fees
                + user_3_deposit_amount_after_fees
        }
        .encode()
    )));

    let result = token_program.send(OWNER, fungible_token_io::FTAction::BalanceOf(USER.into()));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        OWNER,
        fungible_token_io::FTEvent::Balance(
            user_deposit_amount_after_fees
                + user_1_deposit_amount_after_fees
                + user_2_deposit_amount_after_fees
                + user_3_deposit_amount_after_fees
        )
        .encode()
    )));
}

#[test]
fn success_withdraw_finished_more_winners() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let user_1_deposit_amount: u128 = 1000000;
    let user_1_deposit_fees_amount: u128 =
        user_1_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_1_deposit_amount_after_fees: u128 = user_1_deposit_amount - user_1_deposit_fees_amount;

    let user_2_deposit_amount: u128 = 1000000;
    let user_2_deposit_fees_amount: u128 =
        user_2_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_2_deposit_amount_after_fees: u128 = user_2_deposit_amount - user_2_deposit_fees_amount;

    let user_3_deposit_amount: u128 = 1000000;
    let user_3_deposit_fees_amount: u128 =
        user_3_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_3_deposit_amount_after_fees: u128 = user_3_deposit_amount - user_3_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);
    mint_token(&token_program, USER_1, user_1_deposit_amount);
    mint_token(&token_program, USER_2, user_2_deposit_amount);
    mint_token(&token_program, USER_3, user_3_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_1,
        horse_races_program.id().into_bytes().into(),
        user_1_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_1,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_1_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_2,
        horse_races_program.id().into_bytes().into(),
        user_2_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_2,
        Action::Bid {
            horse_name: String::from("Vitalik"),
            amount: user_2_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_3,
        horse_races_program.id().into_bytes().into(),
        user_3_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_3,
        Action::Bid {
            horse_name: String::from("Vitalik"),
            amount: user_3_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::ProgressLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunProgressed(1).encode())));

    set_oracle_value(&oracle_program, 2, 2141258129341892512471);

    let result = horse_races_program.send(MANAGER, Action::FinishLastRun);
    assert!(!result.main_failed());

    let result = horse_races_program.send(USER, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER,
        Event::NewWithdrawFinished {
            user: USER.into(),
            run_id: 1,
            amount: user_deposit_amount_after_fees,
            profit_amount: (user_2_deposit_amount_after_fees + user_3_deposit_amount_after_fees)
                / 2
        }
        .encode()
    )));

    let result = token_program.send(OWNER, fungible_token_io::FTAction::BalanceOf(USER.into()));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        OWNER,
        fungible_token_io::FTEvent::Balance(
            user_deposit_amount_after_fees
                + ((user_2_deposit_amount_after_fees + user_3_deposit_amount_after_fees) / 2)
        )
        .encode()
    )));

    let result = horse_races_program.send(USER_1, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER_1,
        Event::NewWithdrawFinished {
            user: USER_1.into(),
            run_id: 1,
            amount: user_1_deposit_amount_after_fees,
            profit_amount: (user_2_deposit_amount_after_fees + user_3_deposit_amount_after_fees)
                / 2
        }
        .encode()
    )));

    let result = token_program.send(OWNER, fungible_token_io::FTAction::BalanceOf(USER_1.into()));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        OWNER,
        fungible_token_io::FTEvent::Balance(
            user_1_deposit_amount_after_fees
                + ((user_2_deposit_amount_after_fees + user_3_deposit_amount_after_fees) / 2)
        )
        .encode()
    )));
}

#[test]
fn success_withdraw_finished_all_winners() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let user_1_deposit_amount: u128 = 1000000;
    let user_1_deposit_fees_amount: u128 =
        user_1_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_1_deposit_amount_after_fees: u128 = user_1_deposit_amount - user_1_deposit_fees_amount;

    let user_2_deposit_amount: u128 = 1000000;
    let user_2_deposit_fees_amount: u128 =
        user_2_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_2_deposit_amount_after_fees: u128 = user_2_deposit_amount - user_2_deposit_fees_amount;

    let user_3_deposit_amount: u128 = 1000000;
    let user_3_deposit_fees_amount: u128 =
        user_3_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_3_deposit_amount_after_fees: u128 = user_3_deposit_amount - user_3_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);
    mint_token(&token_program, USER_1, user_1_deposit_amount);
    mint_token(&token_program, USER_2, user_2_deposit_amount);
    mint_token(&token_program, USER_3, user_3_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_1,
        horse_races_program.id().into_bytes().into(),
        user_1_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_1,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_1_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_2,
        horse_races_program.id().into_bytes().into(),
        user_2_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_2,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_2_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER_3,
        horse_races_program.id().into_bytes().into(),
        user_3_deposit_amount,
    );

    let result = horse_races_program.send(
        USER_3,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_3_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::ProgressLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunProgressed(1).encode())));

    set_oracle_value(&oracle_program, 2, 2141258129341892512471);

    let result = horse_races_program.send(MANAGER, Action::FinishLastRun);
    assert!(!result.main_failed());

    let result = horse_races_program.send(USER, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER,
        Event::NewWithdrawFinished {
            user: USER.into(),
            run_id: 1,
            amount: user_deposit_amount_after_fees,
            profit_amount: 0,
        }
        .encode()
    )));

    let result = horse_races_program.send(USER_1, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER_1,
        Event::NewWithdrawFinished {
            user: USER_1.into(),
            run_id: 1,
            amount: user_1_deposit_amount_after_fees,
            profit_amount: 0,
        }
        .encode()
    )));

    let result = horse_races_program.send(USER_2, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER_2,
        Event::NewWithdrawFinished {
            user: USER_2.into(),
            run_id: 1,
            amount: user_2_deposit_amount_after_fees,
            profit_amount: 0,
        }
        .encode()
    )));

    let result = horse_races_program.send(USER_3, Action::WithdrawFinished(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER_3,
        Event::NewWithdrawFinished {
            user: USER_3.into(),
            run_id: 1,
            amount: user_3_deposit_amount_after_fees,
            profit_amount: 0,
        }
        .encode()
    )));
}

#[test]
fn success_withdraw_canceled() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::CancelLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunCanceled(1).encode())));

    let result = horse_races_program.send(USER, Action::WithdrawCanceled(1));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        USER,
        Event::NewWithdrawCanceled {
            user: USER.into(),
            run_id: 1,
            amount: user_deposit_amount_after_fees
        }
        .encode()
    )));

    let result = token_program.send(OWNER, fungible_token_io::FTAction::BalanceOf(USER.into()));
    assert!(!result.main_failed());
    assert!(result.contains(&(
        OWNER,
        fungible_token_io::FTEvent::Balance(user_deposit_amount_after_fees).encode()
    )));
}

#[test]
fn fail_create_run_not_manager() {
    let sys = System::new();

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    let result = horse_races_program.send(
        FAKE_MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses: horses.clone(),
        },
    );
    assert!(result.main_failed());
}

#[test]
fn fail_create_run_last_run_not_ended() {
    let sys = System::new();

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses: horses.clone(),
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses: horses.clone(),
        },
    );
    assert!(result.main_failed());
}

#[test]
fn fail_bid_manager() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);
    mint_token(&token_program, MANAGER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        MANAGER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        MANAGER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(result.main_failed());
}

#[test]
fn fail_bid_last_run_not_bidding() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    sys.spend_blocks(1);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(result.main_failed());
}

#[test]
fn fail_cancel_last_run_not_manager() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(FAKE_MANAGER, Action::CancelLastRun);
    assert!(result.main_failed());
}

#[test]
fn fail_cancel_last_run_bidding_not_finished() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    let result = horse_races_program.send(MANAGER, Action::CancelLastRun);
    assert!(result.main_failed());
}

#[test]
fn fail_withdraw_finished_not_finished() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(MANAGER, Action::ProgressLastRun);
    assert!(!result.main_failed());
    assert!(result.contains(&(MANAGER, Event::LastRunProgressed(1).encode())));

    let result = horse_races_program.send(USER, Action::WithdrawFinished(1));
    assert!(result.main_failed());
}

#[test]
fn fail_withdraw_canceled_not_canceled() {
    let sys = System::new();

    let user_deposit_amount: u128 = 1000000;
    let user_deposit_fees_amount: u128 = user_deposit_amount * FEE_BPS as u128 / MAX_BPS as u128;
    let _user_deposit_amount_after_fees: u128 = user_deposit_amount - user_deposit_fees_amount;

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    mint_token(&token_program, USER, user_deposit_amount);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let mut horses = BTreeMap::new();
    horses.insert(String::from("Pegasus"), Horse { max_speed: 50 });
    horses.insert(String::from("Max"), Horse { max_speed: 65 });
    horses.insert(String::from("Vitalik"), Horse { max_speed: 80 });

    let result = horse_races_program.send(
        MANAGER,
        Action::CreateRun {
            bidding_duration_ms: 1000,
            horses,
        },
    );
    assert!(!result.main_failed());

    set_oracle_value(&oracle_program, 1, 2141258129341892512471);

    approve(
        &token_program,
        USER,
        horse_races_program.id().into_bytes().into(),
        user_deposit_amount,
    );

    let result = horse_races_program.send(
        USER,
        Action::Bid {
            horse_name: String::from("Max"),
            amount: user_deposit_amount,
        },
    );
    assert!(!result.main_failed());

    sys.spend_blocks(1);

    let result = horse_races_program.send(USER, Action::WithdrawCanceled(1));
    assert!(result.main_failed());
}

#[test]
fn fail_update() {
    let sys = System::new();

    let (horse_races_program, oracle_program, token_program) = get_programs(&sys);
    init_oracle(&oracle_program);
    init_token(&token_program);

    let result = horse_races_program.send(
        OWNER,
        InitConfig {
            manager: MANAGER.into(),
            token: TOKEN_ID.into(),
            oracle: ORACLE_ID.into(),
            fee_bps: FEE_BPS,
        },
    );
    assert!(!result.main_failed());

    let result = horse_races_program.send(FAKE_MANAGER, Action::UpdateManager(NEW_MANAGER.into()));
    assert!(result.main_failed());

    let result = horse_races_program.send(FAKE_MANAGER, Action::UpdateOracle(NEW_ORACLE.into()));
    assert!(result.main_failed());

    let result = horse_races_program.send(FAKE_MANAGER, Action::UpdateFeeBps(NEW_FEE_BPS));
    assert!(result.main_failed());

    let result = horse_races_program.send(MANAGER, Action::UpdateFeeBps(10001));
    assert!(result.main_failed());

    let result = horse_races_program.send(MANAGER, Action::UpdateFeeBps(30000));
    assert!(result.main_failed());
}
