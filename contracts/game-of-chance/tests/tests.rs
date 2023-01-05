use utils::{prelude::*, FungibleToken};

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

    let mut sft = FungibleToken::initialize(&system);
    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    let admin = ADMIN.into();

    goc.meta_state().state().eq(GOCState {
        admin,
        ..Default::default()
    });

    for player in PLAYERS {
        sft.mint(player, AMOUNT);
        sft.approve(player, goc.actor_id(), PARTICIPATION_COST);
    }

    let mut started = system.block_timestamp();
    let mut ending = started + DURATION;
    let ft_actor_id = Some(sft.actor_id());

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, ft_actor_id)
        .succeed((ending, PARTICIPATION_COST, ft_actor_id));
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        participation_cost: PARTICIPATION_COST,
        ft_actor_id,
        ..Default::default()
    });

    let mut players = vec![];

    for (index, player) in PLAYERS.into_iter().enumerate() {
        let prize_fund = PARTICIPATION_COST * (index + 1) as u128;

        players.push(player.into());

        goc.enter(player).succeed(player);
        sft.balance(goc.actor_id()).contains(prize_fund);
        goc.meta_state().state().eq(GOCState {
            admin,
            started,
            ending,
            players: players.clone(),
            prize_fund,
            participation_cost: PARTICIPATION_COST,
            ft_actor_id,
            ..Default::default()
        });
    }

    system.spend_blocks(DURATION_IN_SECS);

    let winner = utils::predict_winner(&system, &PLAYERS);

    goc.pick_winner(ADMIN).succeed(winner);
    sft.balance(winner)
        .contains(PARTICIPATION_COST * 2 + AMOUNT);
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players: players.clone(),
        prize_fund: PARTICIPATION_COST * 3,
        participation_cost: PARTICIPATION_COST,
        winner,
        ft_actor_id,
    });

    for player in PLAYERS {
        system.mint_to(player, AMOUNT);
    }

    started = system.block_timestamp();
    ending = started + DURATION;

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, None)
        .succeed((ending, PARTICIPATION_COST, None));
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        participation_cost: PARTICIPATION_COST,
        ..Default::default()
    });

    players.clear();

    for (index, player) in PLAYERS.into_iter().enumerate() {
        let prize_fund = PARTICIPATION_COST * (index + 1) as u128;

        players.push(player.into());

        goc.enter_with_value(player, PARTICIPATION_COST)
            .succeed(player);
        assert_eq!(system.balance_of(goc.actor_id().as_ref()), prize_fund);
        goc.meta_state().state().eq(GOCState {
            admin,
            started,
            ending,
            players: players.clone(),
            prize_fund,
            participation_cost: PARTICIPATION_COST,
            ..Default::default()
        });
    }

    system.spend_blocks(DURATION_IN_SECS);

    let winner: [u8; 32] = utils::predict_winner(&system, &PLAYERS).into();

    goc.pick_winner(ADMIN).succeed(winner.into());
    system.claim_value_from_mailbox(winner);
    assert_eq!(system.balance_of(winner), PARTICIPATION_COST * 2 + AMOUNT);
    goc.meta_state().state().eq(GOCState {
        admin,
        started,
        ending,
        players,
        prize_fund: PARTICIPATION_COST * 3,
        participation_cost: PARTICIPATION_COST,
        winner: winner.into(),
        ..Default::default()
    });
}

#[test]
fn failures() {
    let system = utils::initialize_system();

    Goc::initialize_with_existential_deposit(&system, ActorId::zero())
        .failed(GOCError::ZeroActorId);

    let mut goc = Goc::initialize(&system, ADMIN).succeed();
    goc.start(FOREIGN_USER, 0, 0, None)
        .failed(GOCError::AccessRestricted);

    goc.start(ADMIN, 0, 0, Some(ActorId::zero()))
        .failed(GOCError::ZeroActorId);

    goc.enter(PLAYERS[0]).failed(GOCError::UnexpectedGameStatus);

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, None)
        .succeed((
            system.block_timestamp() + DURATION,
            PARTICIPATION_COST,
            None,
        ));
    goc.start(ADMIN, 0, 0, None)
        .failed(GOCError::UnexpectedGameStatus);

    system.mint_to(PLAYERS[0], AMOUNT);
    goc.enter_with_value(PLAYERS[0], PARTICIPATION_COST)
        .succeed(PLAYERS[0]);
    goc.enter(PLAYERS[0]).failed(GOCError::AlreadyParticipating);

    system.mint_to(PLAYERS[1], AMOUNT);
    goc.enter_with_value(PLAYERS[1], PARTICIPATION_COST + 1)
        .failed(GOCError::InvalidParticipationCost);

    system.claim_value_from_mailbox(PLAYERS[1]);
    goc.enter_with_value(PLAYERS[1], PARTICIPATION_COST - 1)
        .failed(GOCError::InvalidParticipationCost);

    goc.pick_winner(FOREIGN_USER)
        .failed(GOCError::AccessRestricted);

    goc.pick_winner(ADMIN)
        .failed(GOCError::UnexpectedGameStatus);

    system.spend_blocks(DURATION_IN_SECS);
    goc.pick_winner(ADMIN).succeed(PLAYERS[0].into());
    goc.pick_winner(ADMIN)
        .failed(GOCError::UnexpectedGameStatus);
}

#[test]
fn round_without_players() {
    let system = utils::initialize_system();
    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    goc.start(ADMIN, 0, 0, None)
        .succeed((system.block_timestamp(), 0, None));
    goc.pick_winner(ADMIN).succeed(ActorId::zero());
}

#[test]
fn overflow() {
    const AMOUNT: u128 = u128::MAX;
    const PARTICIPATION_COST: u128 = u128::MAX;
    const DURATION: u64 = u64::MAX;

    let system = utils::initialize_system();

    let mut sft = FungibleToken::initialize(&system);
    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    let ending = DURATION;
    let ft_actor_id = Some(sft.actor_id());

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, ft_actor_id)
        .succeed((ending, PARTICIPATION_COST, ft_actor_id));

    for player in PLAYERS.into_iter().take(2) {
        sft.mint(player, AMOUNT);
        sft.approve(player, goc.actor_id(), PARTICIPATION_COST);

        goc.enter(player).succeed(player);
    }

    goc.meta_state().state().eq(GOCState {
        admin: ADMIN.into(),
        started: system.block_timestamp(),
        ending,
        players: vec![PLAYERS[0].into(), PLAYERS[1].into()],
        prize_fund: u128::MAX,
        participation_cost: PARTICIPATION_COST,
        ft_actor_id,
        ..Default::default()
    })
}
