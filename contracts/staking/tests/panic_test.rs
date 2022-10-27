#[cfg(test)]
extern crate std;

use ft_io::*;
use gstd::Encode;
use gtest::{Program, System};
use staking_io::*;

const USERS: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8];

fn init_staking(sys: &System) {
    let staking = Program::current(sys);

    let res = staking.send(
        USERS[3],
        InitStaking {
            staking_token_address: USERS[1].into(),
            reward_token_address: USERS[2].into(),
            distribution_time: 10000,
            reward_total: 1000,
        },
    );

    assert!(res.log().is_empty());
}

fn init_staking_token(sys: &System) {
    let st_token = Program::from_file(sys, "./target/fungible_token-0.1.2.wasm");

    let res = st_token.send(
        USERS[3],
        InitConfig {
            name: String::from("StakingToken"),
            symbol: String::from("STK"),
            decimals: 18,
        },
    );

    assert!(res.log().is_empty());

    let res = st_token.send(USERS[3], FTAction::Mint(100000));
    assert!(!res.main_failed());
    let res = st_token.send(
        USERS[3],
        FTAction::Transfer {
            from: USERS[3].into(),
            to: USERS[0].into(),
            amount: 100000,
        },
    );
    assert!(!res.main_failed());

    let res = st_token.send(USERS[3], FTAction::BalanceOf(USERS[0].into()));
    assert!(res.contains(&(USERS[3], FTEvent::Balance(100000).encode())));

    let res = st_token.send(USERS[4], FTAction::Mint(10000));
    assert!(!res.main_failed());

    let res = st_token.send(USERS[3], FTAction::BalanceOf(USERS[4].into()));
    assert!(res.contains(&(USERS[3], FTEvent::Balance(10000).encode())));

    let res = st_token.send(USERS[5], FTAction::Mint(20000));
    assert!(!res.main_failed());

    let res = st_token.send(USERS[3], FTAction::BalanceOf(USERS[5].into()));
    assert!(res.contains(&(USERS[3], FTEvent::Balance(20000).encode())));

    let res = st_token.send(USERS[6], FTAction::Mint(20000));
    assert!(!res.main_failed());

    let res = st_token.send(USERS[3], FTAction::BalanceOf(USERS[6].into()));
    assert!(res.contains(&(USERS[3], FTEvent::Balance(20000).encode())));

    let res = st_token.send(USERS[7], FTAction::Mint(20000));
    assert!(!res.main_failed());

    let res = st_token.send(USERS[3], FTAction::BalanceOf(USERS[7].into()));
    assert!(res.contains(&(USERS[3], FTEvent::Balance(20000).encode())));
}

fn init_reward_token(sys: &System) {
    let rw_token = Program::from_file(sys, "./target/fungible_token-0.1.2.wasm");

    let res = rw_token.send(
        USERS[3],
        InitConfig {
            name: String::from("RewardToken"),
            symbol: String::from("RTK"),
            decimals: 18,
        },
    );

    assert!(res.log().is_empty());

    let res = rw_token.send(USERS[3], FTAction::Mint(100000));
    assert!(!res.main_failed());
    let res = rw_token.send(
        USERS[3],
        FTAction::Transfer {
            from: USERS[3].into(),
            to: USERS[0].into(),
            amount: 100000,
        },
    );
    assert!(!res.main_failed());

    let res = rw_token.send(USERS[3], FTAction::BalanceOf(USERS[0].into()));
    assert!(res.contains(&(USERS[3], FTEvent::Balance(100000).encode())));
}

#[test]
fn stake() {
    let sys = System::new();
    init_staking(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(USERS[4], StakingAction::Stake(0));
    assert!(res.main_failed());
}

#[test]
fn update_staking() {
    let sys = System::new();
    init_staking(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(
        USERS[4],
        StakingAction::UpdateStaking(InitStaking {
            staking_token_address: USERS[1].into(),
            reward_token_address: USERS[2].into(),
            distribution_time: 10000,
            reward_total: 1000,
        }),
    );
    assert!(res.main_failed());

    let res = staking.send(
        USERS[3],
        StakingAction::UpdateStaking(InitStaking {
            staking_token_address: USERS[1].into(),
            reward_token_address: USERS[2].into(),
            distribution_time: 10000,
            reward_total: 0,
        }),
    );
    assert!(res.main_failed());

    let res = staking.send(
        USERS[3],
        StakingAction::UpdateStaking(InitStaking {
            staking_token_address: USERS[1].into(),
            reward_token_address: USERS[2].into(),
            distribution_time: 0,
            reward_total: 1000,
        }),
    );
    assert!(res.main_failed());
}

#[test]
fn send_reward() {
    let sys = System::new();
    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(USERS[4], StakingAction::GetReward);
    assert!(res.main_failed());
}

#[test]
fn withdraw() {
    let sys = System::new();

    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(USERS[4], StakingAction::Stake(1500));
    assert!(res.contains(&(USERS[4], StakingEvent::StakeAccepted(1500).encode())));

    let res = staking.send(USERS[5], StakingAction::Stake(2000));
    assert!(res.contains(&(USERS[5], StakingEvent::StakeAccepted(2000).encode())));

    let res = staking.send(USERS[4], StakingAction::Withdraw(0));
    assert!(res.main_failed());

    let res = staking.send(USERS[6], StakingAction::Withdraw(1000));
    assert!(res.main_failed());

    let res = staking.send(USERS[5], StakingAction::Withdraw(5000));
    assert!(res.main_failed());
}
