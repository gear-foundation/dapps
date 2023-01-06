use ft_io::*;
use gstd::{ActorId, Encode};
use gtest::{Program, System};
use hashbrown::HashMap;
use staking::io::*;

const USERS: &[u64] = &[1, 2, 3, 4, 5, 6, 7, 8];
const DECIMALS_FACTOR: u128 = 10_u128.pow(20);

#[derive(Debug, Default)]
struct Staking {
    tokens_per_stake: u128,
    total_staked: u128,
    distribution_time: u64,
    produced_time: u64,
    reward_total: u128,
    all_produced: u128,
    reward_produced: u128,
    stakers: HashMap<ActorId, Staker>,
}

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
    let st_token = Program::from_file(sys, "./target/fungible_token-0.1.3.wasm");

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
    let rw_token = Program::from_file(sys, "./target/fungible_token-0.1.3.wasm");

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

/// Sets the reward to be distributed within distribution time
/// param 'reward' The value of the distributed reward
fn update_staking(staking: &mut Staking, reward: u128, time: u64) {
    if reward == 0 {
        panic!("update_staking(): reward is null");
    }

    staking.distribution_time = 10000;
    update_reward(staking, time);
    staking.all_produced = staking.reward_produced;
    staking.produced_time = time;
    staking.reward_total = reward;
}

/// Calculates the reward produced so far
fn produced(staking: &mut Staking, time: u64) -> u128 {
    let mut elapsed_time = time - staking.produced_time;

    if elapsed_time > staking.distribution_time {
        elapsed_time = staking.distribution_time;
    }

    staking.all_produced
        + staking.reward_total.saturating_mul(elapsed_time as u128)
            / staking.distribution_time as u128
}

/// Calculates the maximum possible reward
/// The reward that the depositor would have received if he had initially paid this amount
/// Arguments:
/// `amount`: the number of tokens
fn get_max_reward(staking: &Staking, amount: u128) -> u128 {
    (amount * staking.tokens_per_stake) / DECIMALS_FACTOR
}

/// Updates the reward produced so far and calculates tokens per stake
fn update_reward(staking: &mut Staking, time: u64) {
    let reward_produced_at_now = produced(staking, time);

    if reward_produced_at_now > staking.reward_produced {
        let produced_new = reward_produced_at_now - staking.reward_produced;

        if staking.total_staked > 0 {
            staking.tokens_per_stake = staking
                .tokens_per_stake
                .saturating_add((produced_new * DECIMALS_FACTOR) / staking.total_staked);
        }

        staking.reward_produced = staking.reward_produced.saturating_add(produced_new);
    }
}

/// Calculates the reward of the staker that is currently available
fn calc_reward(staking: &mut Staking, source: &ActorId) -> u128 {
    if let Some(staker) = staking.stakers.get(source) {
        return get_max_reward(staking, staker.balance) + staker.reward_allowed
            - staker.reward_debt
            - staker.distributed;
    }

    panic!("calc_reward(): Staker {source:?} not found");
}

#[test]
fn stake() {
    let sys = System::new();
    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(USERS[4], StakingAction::Stake(1000));
    assert!(res.contains(&(USERS[4], StakingEvent::StakeAccepted(1000).encode())));

    let res = staking.send(USERS[5], StakingAction::Stake(3000));
    assert!(res.contains(&(USERS[5], StakingEvent::StakeAccepted(3000).encode())));
}

#[test]
fn update_staking_test() {
    let sys = System::new();
    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(
        USERS[3],
        StakingAction::UpdateStaking(InitStaking {
            staking_token_address: USERS[1].into(),
            reward_token_address: USERS[2].into(),
            distribution_time: 10000,
            reward_total: 1000,
        }),
    );
    assert!(res.contains(&(USERS[3], StakingEvent::Updated.encode())));
}

#[test]
fn send_reward() {
    let sys = System::new();
    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let st = sys.get_program(1);

    let time = sys.block_timestamp();

    let mut staking = Staking {
        ..Default::default()
    };

    update_staking(&mut staking, 1000, time);

    let res = st.send(USERS[4], StakingAction::Stake(1500));
    assert!(res.contains(&(USERS[4], StakingEvent::StakeAccepted(1500).encode())));

    update_reward(&mut staking, time);
    staking.stakers.insert(
        USERS[4].into(),
        Staker {
            reward_debt: get_max_reward(&staking, 1500),
            balance: 1500,
            ..Default::default()
        },
    );

    staking.total_staked = 1500;

    sys.spend_blocks(2);

    let res = st.send(USERS[5], StakingAction::Stake(2000));
    assert!(res.contains(&(USERS[5], StakingEvent::StakeAccepted(2000).encode())));

    update_reward(&mut staking, time + 2000);
    staking.stakers.insert(
        USERS[5].into(),
        Staker {
            reward_debt: get_max_reward(&staking, 2000),
            balance: 2000,
            ..Default::default()
        },
    );

    staking.total_staked = 3500;

    sys.spend_blocks(1);

    update_reward(&mut staking, time + 3000);
    let reward = calc_reward(&mut staking, &USERS[4].into());

    staking
        .stakers
        .entry(USERS[4].into())
        .and_modify(|stake| stake.distributed = stake.distributed.saturating_add(reward));

    let res = st.send(USERS[4], StakingAction::GetReward);
    println!(
        "Reward[4]: {:?} calc: {}, staking: {:?}",
        res.decoded_log::<StakingEvent>(),
        reward,
        staking
    );
    assert!(res.contains(&(USERS[4], StakingEvent::Reward(reward).encode())));

    sys.spend_blocks(1);

    update_reward(&mut staking, time + 4000);
    let reward = calc_reward(&mut staking, &USERS[5].into());

    staking
        .stakers
        .entry(USERS[5].into())
        .and_modify(|stake| stake.distributed = stake.distributed.saturating_add(reward));

    let res = st.send(USERS[5], StakingAction::GetReward);
    println!(
        "Reward[5]: {:?} calc: {}, staking: {:?}",
        res.decoded_log::<StakingEvent>(),
        reward,
        staking
    );
    assert!(res.contains(&(USERS[5], StakingEvent::Reward(reward).encode())));
}

#[test]
fn withdraw() {
    let sys = System::new();

    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let st = sys.get_program(1);

    let time = sys.block_timestamp();

    let mut staking = Staking {
        ..Default::default()
    };

    update_staking(&mut staking, 1000, time);

    let res = st.send(USERS[4], StakingAction::Stake(1500));
    assert!(res.contains(&(USERS[4], StakingEvent::StakeAccepted(1500).encode())));

    update_reward(&mut staking, time);
    staking.stakers.insert(
        USERS[4].into(),
        Staker {
            reward_debt: get_max_reward(&staking, 1500),
            balance: 1500,
            ..Default::default()
        },
    );

    staking.total_staked = 1500;

    sys.spend_blocks(2);

    let res = st.send(USERS[5], StakingAction::Stake(2000));
    assert!(res.contains(&(USERS[5], StakingEvent::StakeAccepted(2000).encode())));

    update_reward(&mut staking, time + 2000);
    staking.stakers.insert(
        USERS[5].into(),
        Staker {
            reward_debt: get_max_reward(&staking, 2000),
            balance: 2000,
            ..Default::default()
        },
    );

    staking.total_staked = 3500;

    sys.spend_blocks(1);

    let res = st.send(USERS[4], StakingAction::Withdraw(500));
    assert!(res.contains(&(USERS[4], StakingEvent::Withdrawn(500).encode())));

    update_reward(&mut staking, time + 3000);
    let max_reward = get_max_reward(&staking, 500);
    let actor_id: &ActorId = &USERS[4].into();
    let opt = staking.stakers.get_mut(actor_id);
    if let Some(staker) = opt {
        staker.reward_allowed = staker.reward_allowed.saturating_add(max_reward);

        staker.balance = staker.balance.saturating_sub(500);
        staking.total_staked -= 500;
    }

    sys.spend_blocks(1);

    update_reward(&mut staking, time + 4000);
    let reward = calc_reward(&mut staking, &USERS[4].into());

    staking
        .stakers
        .entry(USERS[4].into())
        .and_modify(|stake| stake.distributed = stake.distributed.saturating_add(reward));

    let res = st.send(USERS[4], StakingAction::GetReward);
    assert!(res.contains(&(USERS[4], StakingEvent::Reward(reward).encode())));
    println!("Reward[4]: {:?}", res.decoded_log::<StakingEvent>());

    sys.spend_blocks(2);

    update_reward(&mut staking, time + 6000);
    let reward = calc_reward(&mut staking, &USERS[5].into());

    staking
        .stakers
        .entry(USERS[5].into())
        .and_modify(|stake| stake.distributed = stake.distributed.saturating_add(reward));

    let res = st.send(USERS[5], StakingAction::GetReward);
    assert!(res.contains(&(USERS[5], StakingEvent::Reward(reward).encode())));
    println!("Reward[5]: {:?}", res.decoded_log::<StakingEvent>());
}

#[test]
fn meta_tests() {
    let sys = System::new();
    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let st = sys.get_program(1);

    let time = sys.block_timestamp();

    let mut staking = Staking {
        distribution_time: 10000,
        ..Default::default()
    };

    update_staking(&mut staking, 1000, time);

    let res = st.send(USERS[4], StakingAction::Stake(1500));
    assert!(res.contains(&(USERS[4], StakingEvent::StakeAccepted(1500).encode())));

    update_reward(&mut staking, time);
    staking.stakers.insert(
        USERS[4].into(),
        Staker {
            reward_debt: get_max_reward(&staking, 1500),
            balance: 1500,
            ..Default::default()
        },
    );

    staking.total_staked = 1500;

    sys.spend_blocks(2);

    let res = st.send(USERS[5], StakingAction::Stake(2000));
    assert!(res.contains(&(USERS[5], StakingEvent::StakeAccepted(2000).encode())));

    update_reward(&mut staking, time + 2000);
    staking.stakers.insert(
        USERS[5].into(),
        Staker {
            reward_debt: get_max_reward(&staking, 2000),
            balance: 2000,
            ..Default::default()
        },
    );

    staking.total_staked = 3500;
    let stakers = staking.stakers.clone().into_iter().collect();
    assert_eq!(
        st.meta_state::<_, StakingStateReply>(StakingState::GetStakers)
            .expect("StakingState::GetStakers failure"),
        StakingStateReply::Stakers(stakers)
    );

    let actor_id: &ActorId = &USERS[4].into();
    let staker = staking.stakers.get(actor_id).unwrap();

    assert_eq!(
        st.meta_state::<_, StakingStateReply>(StakingState::GetStaker(USERS[4].into()))
            .expect("StakingState::GetStaker failure"),
        StakingStateReply::Staker(staker.clone())
    );
}
