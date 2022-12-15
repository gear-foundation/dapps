use utils::{prelude::*, Sft};

mod utils;

const ADMIN: u64 = 3;
const PLAYERS: [u64; 3] = [4, 5, 6];
const AMOUNT: u128 = 12345;
const PARTICIPATION_COST: u128 = 10000;
const DURATION: u64 = 2000;
const DURATION_IN_SECS: u32 = (DURATION / 1000) as _;

#[test]
fn two_rounds_and_meta_state() {
    let system = utils::initialize_system();

    let mut sft = Sft::initialize(&system);
    let mut goc = Goc::initialize(&system, ADMIN).succeed();
    let admin = ADMIN.into();
    let winner = ActorId::zero();

    goc.meta_state().state().eq(GOCState {
        admin,
        started: 0,
        ending: 0,
        players: BTreeSet::new(),
        prize_fund: 0,
        participation_cost: 0,
        winner: admin,
        ft_actor_id: None,
    });

    for player in [0, 1, 2] {
        sft.mint(PLAYERS[player], AMOUNT);

        sft.approve(PLAYERS[player], goc.actor_id(), PARTICIPATION_COST);
    }

    let mut started = system.block_timestamp();
    let mut ending = started + DURATION;
    let mut ft_actor_id = Some(sft.actor_id());
    let winner = ActorId::zero();

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, ft_actor_id)
        .contains((ending, PARTICIPATION_COST, ft_actor_id));
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::new(),
        prize_fund: 0,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    goc.enter(PLAYERS[0]).contains(PLAYERS[0]);
    sft.balance(goc.actor_id()).contains(PARTICIPATION_COST);
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into()]),
        prize_fund: PARTICIPATION_COST,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    goc.enter(PLAYERS[1]).contains(PLAYERS[1]);
    sft.balance(goc.actor_id()).contains(PARTICIPATION_COST * 2);
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into(), PLAYERS[1].into()]),
        prize_fund: PARTICIPATION_COST * 2,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    goc.enter(PLAYERS[2]).contains(PLAYERS[2]);
    sft.balance(goc.actor_id()).contains(PARTICIPATION_COST * 3);
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into(), PLAYERS[1].into(), PLAYERS[2].into()]),
        prize_fund: PARTICIPATION_COST * 3,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    system.spend_blocks(DURATION_IN_SECS);

    let winner = utils::predict_winner(&system, &PLAYERS);

    goc.pick_winner(ADMIN).contains(winner.into());
    started = 0;
    sft.balance(winner)
        .contains(PARTICIPATION_COST * 2 + AMOUNT);
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into(), PLAYERS[1].into(), PLAYERS[2].into()]),
        prize_fund: PARTICIPATION_COST * 3,
        participation_cost: PARTICIPATION_COST,
        winner: winner.into(),
        ft_actor_id,
    });

    for player in [0, 1, 2] {
        system.mint_to(PLAYERS[player], AMOUNT);
    }

    ft_actor_id = None;
    started = system.block_timestamp();
    ending = started + DURATION;

    let winner = ActorId::zero();

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, ft_actor_id)
        .contains((ending, PARTICIPATION_COST, ft_actor_id));
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::new(),
        prize_fund: 0,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    goc.enter_with_value(PLAYERS[0], PARTICIPATION_COST)
        .contains(PLAYERS[0]);
    assert_eq!(
        system.balance_of(goc.actor_id().as_ref()),
        PARTICIPATION_COST
    );
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into()]),
        prize_fund: PARTICIPATION_COST,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    goc.enter_with_value(PLAYERS[1], PARTICIPATION_COST)
        .contains(PLAYERS[1]);
    assert_eq!(
        system.balance_of(goc.actor_id().as_ref()),
        PARTICIPATION_COST * 2
    );
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into(), PLAYERS[1].into()]),
        prize_fund: PARTICIPATION_COST * 2,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    goc.enter_with_value(PLAYERS[2], PARTICIPATION_COST)
        .contains(PLAYERS[2]);
    assert_eq!(
        system.balance_of(goc.actor_id().as_ref()),
        PARTICIPATION_COST * 3
    );
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into(), PLAYERS[1].into(), PLAYERS[2].into()]),
        prize_fund: PARTICIPATION_COST * 3,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    system.spend_blocks(DURATION_IN_SECS);

    let winner = utils::predict_winner(&system, &PLAYERS);

    goc.pick_winner(ADMIN).contains(winner.into());
    system.claim_value_from_mailbox(winner);
    assert_eq!(system.balance_of(winner), PARTICIPATION_COST * 2 + AMOUNT);
    let started = 0;
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into(), PLAYERS[1].into(), PLAYERS[2].into()]),
        prize_fund: PARTICIPATION_COST * 3,
        participation_cost: PARTICIPATION_COST,
        winner: winner.into(),
        ft_actor_id,
    });
}

#[test]
fn failures() {
    let system = utils::initialize_system();

    // Should fail because `admin` mustn't be `ActorId::zero()`.
    Goc::initialize(&system, ActorId::zero()).failed();

    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    // Should fail because `msg::source()` must be the game administrator.
    goc.start(FOREIGN_USER, 0, 0, None).failed();

    // Should fail because `ft_actor_id` mustn't be `ActorId::zero()`.
    goc.start(ADMIN, 0, 0, Some(ActorId::zero())).failed();

    //Should fail because the players entry stage mustn't be over.
    goc.enter(PLAYERS[0]).failed();

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, None)
        .contains((
            system.block_timestamp() + DURATION,
            PARTICIPATION_COST,
            None,
        ));

    // Should fail because the current game round must be over.
    goc.start(ADMIN, 0, 0, None).failed();

    system.mint_to(PLAYERS[0], AMOUNT);
    goc.enter_with_value(PLAYERS[0], PARTICIPATION_COST)
        .contains(PLAYERS[0]);

    // Should fail because `msg::source()` mustn't already participate.
    goc.enter(PLAYERS[0]).failed();

    system.mint_to(PLAYERS[1], AMOUNT);

    // Should fail because `msg::source()` must send the amount of the native
    // value exactly equal to a participation cost.
    goc.enter_with_value(PLAYERS[1], PARTICIPATION_COST + 1)
        .failed();
    goc.enter_with_value(PLAYERS[1], PARTICIPATION_COST - 1)
        .failed();

    // Should fail because `msg::source()` must be the game administrator.
    goc.pick_winner(FOREIGN_USER).failed();

    // Should fail because the players entry stage must be over.
    goc.pick_winner(ADMIN).failed();

    system.spend_blocks(DURATION_IN_SECS);
    goc.pick_winner(ADMIN).contains(PLAYERS[0].into());

    // Should fail because a winner mustn't already be picked.
    goc.pick_winner(ADMIN).failed();

    // Should fail because the players entry stage mustn't be over.
    goc.enter(PLAYERS[1]).failed();
}

#[test]
fn round_without_players() {
    let system = utils::initialize_system();

    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    goc.start(ADMIN, 0, 0, None)
        .contains((system.block_timestamp(), 0, None));

    goc.pick_winner(ADMIN).contains(ActorId::zero());
}

#[test]
fn prize_fund_overflow() {
    const AMOUNT: u128 = u128::MAX;
    const PARTICIPATION_COST: u128 = u128::MAX;

    let system = utils::initialize_system();

    let mut sft = Sft::initialize(&system);
    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    let started = system.block_timestamp();
    let ending = started + DURATION;
    let ft_actor_id = Some(sft.actor_id());

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, ft_actor_id)
        .contains((ending, PARTICIPATION_COST, ft_actor_id));

    sft.mint(PLAYERS[0], AMOUNT);
    sft.mint(PLAYERS[1], AMOUNT);

    sft.approve(PLAYERS[0], goc.actor_id(), PARTICIPATION_COST);
    sft.approve(PLAYERS[1], goc.actor_id(), PARTICIPATION_COST);

    goc.enter(PLAYERS[0]).contains(PLAYERS[0]);
    goc.enter(PLAYERS[1]).contains(PLAYERS[1]);

    goc.meta_state().state().eq(GOCState {
        admin: ADMIN.into(),
        started,
        ending,
        players: BTreeSet::from([PLAYERS[0].into(), PLAYERS[1].into()]),
        prize_fund: u128::MAX,
        participation_cost: PARTICIPATION_COST,
        winner: ActorId::zero(),
        ft_actor_id,
    })
}
