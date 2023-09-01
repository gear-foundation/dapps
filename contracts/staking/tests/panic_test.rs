use gstd::{ActorId, Encode};
use gtest::{Program, System};
use staking_io::*;

mod utils;
use utils::{FungibleToken, PROGRAMS};

fn init_staking(sys: &System) {
    let staking = Program::current_opt(sys);

    let res = staking.send(
        4,
        InitStaking {
            staking_token_address: PROGRAMS[1].into(),
            reward_token_address: PROGRAMS[2].into(),
            distribution_time: 10000,
            reward_total: 1000,
        },
    );

    assert!(res.contains(&(4, Ok::<StakingEvent, Error>(StakingEvent::Updated).encode())));
}

fn init_staking_token(sys: &System) -> FungibleToken<'_> {
    let mut st_token = FungibleToken::initialize(sys);

    st_token.mint(1, 100000);
    st_token.balance(1).contains(100000);

    st_token.mint(5, 10000);
    st_token.balance(5).contains(10000);

    st_token.mint(6, 20000);
    st_token.balance(6).contains(20000);

    st_token.mint(7, 20000);
    st_token.balance(7).contains(20000);

    st_token.mint(8, 20000);
    st_token.balance(8).contains(20000);

    st_token
}

fn init_reward_token(sys: &System) {
    let mut rw_token = FungibleToken::initialize(sys);

    rw_token.mint(1, 100000);

    rw_token.balance(1).contains(100000);
}

#[test]
fn stake() {
    let sys = System::new();
    init_staking(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(5, StakingAction::Stake(0));
    assert!(res.contains(&(5, Err::<StakingEvent, Error>(Error::ZeroAmount).encode())));
}

#[test]
fn update_staking() {
    let sys = System::new();
    init_staking(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(
        5,
        StakingAction::UpdateStaking(InitStaking {
            staking_token_address: PROGRAMS[1].into(),
            reward_token_address: PROGRAMS[2].into(),
            distribution_time: 10000,
            reward_total: 1000,
        }),
    );
    assert!(res.contains(&(5, Err::<StakingEvent, Error>(Error::NotOwner).encode())));

    let res = staking.send(
        4,
        StakingAction::UpdateStaking(InitStaking {
            staking_token_address: PROGRAMS[1].into(),
            reward_token_address: PROGRAMS[2].into(),
            distribution_time: 10000,
            reward_total: 0,
        }),
    );
    assert!(res.contains(&(4, Err::<StakingEvent, Error>(Error::ZeroReward).encode())));

    let res = staking.send(
        4,
        StakingAction::UpdateStaking(InitStaking {
            staking_token_address: PROGRAMS[1].into(),
            reward_token_address: PROGRAMS[2].into(),
            distribution_time: 0,
            reward_total: 1000,
        }),
    );
    println!("{:?}", res.decoded_log::<Result<StakingEvent, Error>>());
    assert!(res.contains(&(4, Err::<StakingEvent, Error>(Error::ZeroTime).encode())));
}

#[test]
fn send_reward() {
    let sys = System::new();
    init_staking(&sys);
    init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let res = staking.send(5, StakingAction::GetReward);

    assert!(res.contains(&(
        5,
        Err::<StakingEvent, Error>(Error::StakerNotFound).encode()
    )));
}

#[test]
fn withdraw() {
    let sys = System::new();

    init_staking(&sys);
    let mut st_token = init_staking_token(&sys);
    init_reward_token(&sys);
    sys.init_logger();
    let staking = sys.get_program(1);

    let id: ActorId = staking.id().into_bytes().into();
    st_token.approve(5, id, 1500);

    let res = staking.send(5, StakingAction::Stake(1500));
    assert!(res.contains(&(
        5,
        Ok::<StakingEvent, Error>(StakingEvent::StakeAccepted(1500)).encode()
    )));
    st_token.approve(6, id, 2000);
    let res = staking.send(6, StakingAction::Stake(2000));
    assert!(res.contains(&(
        6,
        Ok::<StakingEvent, Error>(StakingEvent::StakeAccepted(2000)).encode()
    )));

    let res = staking.send(5, StakingAction::Withdraw(0));
    assert!(res.contains(&(5, Err::<StakingEvent, Error>(Error::ZeroAmount).encode())));

    let res = staking.send(7, StakingAction::Withdraw(1000));
    assert!(res.contains(&(
        7,
        Err::<StakingEvent, Error>(Error::StakerNotFound).encode()
    )));

    let res = staking.send(6, StakingAction::Withdraw(5000));
    assert!(res.contains(&(
        6,
        Err::<StakingEvent, Error>(Error::InsufficentBalance).encode()
    )));
}
