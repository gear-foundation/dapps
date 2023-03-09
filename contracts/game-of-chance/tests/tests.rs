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

    let mut fungible_token = FungibleToken::initialize(&system);
    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    let admin = ADMIN.into();

    goc.state().all().eq(State {
        admin,
        ..Default::default()
    });

    for player in PLAYERS {
        fungible_token.mint(player, AMOUNT);
        fungible_token.approve(player, goc.actor_id(), PARTICIPATION_COST);
    }

    let mut started = system.block_timestamp();
    let mut ending = started + DURATION;
    let ft_actor_id = Some(fungible_token.actor_id());
    let is_active = true;

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, ft_actor_id)
        .succeed((ending, PARTICIPATION_COST, ft_actor_id));
    goc.state().all().eq(State {
        admin,
        started,
        ending,
        participation_cost: PARTICIPATION_COST,
        fungible_token: ft_actor_id,
        is_active,
        ..Default::default()
    });

    let mut players = vec![];

    for (index, player) in PLAYERS.into_iter().enumerate() {
        let prize_fund = PARTICIPATION_COST * (index + 1) as u128;

        players.push(player.into());

        goc.enter(player).succeed(player);
        fungible_token.balance(goc.actor_id()).contains(prize_fund);
        goc.state().all().eq(State {
            admin,
            started,
            ending,
            players: players.clone(),
            prize_fund,
            participation_cost: PARTICIPATION_COST,
            fungible_token: ft_actor_id,
            is_active,
            ..Default::default()
        });
    }

    system.spend_blocks(DURATION_IN_SECS);

    let winner = utils::predict_winner(&system, &PLAYERS);

    goc.pick_winner(ADMIN).succeed(winner);
    fungible_token
        .balance(winner)
        .contains(PARTICIPATION_COST * 2 + AMOUNT);
    goc.state().all().eq(State {
        admin,
        started,
        ending,
        players: players.clone(),
        prize_fund: PARTICIPATION_COST * 3,
        participation_cost: PARTICIPATION_COST,
        winner,
        fungible_token: ft_actor_id,
        ..Default::default()
    });

    for player in PLAYERS {
        system.mint_to(player, AMOUNT);
    }

    started = system.block_timestamp();
    ending = started + DURATION;

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, None)
        .succeed((ending, PARTICIPATION_COST, None));
    goc.state().all().eq(State {
        admin,
        started,
        ending,
        participation_cost: PARTICIPATION_COST,
        is_active,
        ..Default::default()
    });

    players.clear();

    for (index, player) in PLAYERS.into_iter().enumerate() {
        let prize_fund = PARTICIPATION_COST * (index + 1) as u128;

        players.push(player.into());

        goc.enter_with_value(player, PARTICIPATION_COST)
            .succeed(player);
        assert_eq!(system.balance_of(goc.actor_id().as_ref()), prize_fund);
        goc.state().all().eq(State {
            admin,
            started,
            ending,
            players: players.clone(),
            prize_fund,
            participation_cost: PARTICIPATION_COST,
            is_active,
            ..Default::default()
        });
    }

    system.spend_blocks(DURATION_IN_SECS);

    let winner: [u8; 32] = utils::predict_winner(&system, &PLAYERS).into();

    goc.pick_winner(ADMIN).succeed(winner.into());
    system.claim_value_from_mailbox(winner);
    assert_eq!(system.balance_of(winner), PARTICIPATION_COST * 2 + AMOUNT);
    goc.state().all().eq(State {
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

    Goc::initialize_with_existential_deposit(&system, ActorId::zero()).failed(Error::ZeroActorId);

    let mut goc = Goc::initialize(&system, ADMIN).succeed();
    goc.start(FOREIGN_USER, 0, 0, None)
        .failed(Error::AccessRestricted);

    goc.start(ADMIN, 0, 0, Some(ActorId::zero()))
        .failed(Error::ZeroActorId);

    goc.enter(PLAYERS[0]).failed(Error::UnexpectedGameStatus);

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, None)
        .succeed((
            system.block_timestamp() + DURATION,
            PARTICIPATION_COST,
            None,
        ));
    goc.start(ADMIN, 0, 0, None)
        .failed(Error::UnexpectedGameStatus);

    system.mint_to(PLAYERS[0], AMOUNT);
    goc.enter_with_value(PLAYERS[0], PARTICIPATION_COST)
        .succeed(PLAYERS[0]);
    goc.enter(PLAYERS[0]).failed(Error::AlreadyParticipating);

    system.mint_to(PLAYERS[1], AMOUNT);
    goc.enter_with_value(PLAYERS[1], PARTICIPATION_COST + 1)
        .failed(Error::InvalidParticipationCost);

    system.claim_value_from_mailbox(PLAYERS[1]);
    goc.enter_with_value(PLAYERS[1], PARTICIPATION_COST - 1)
        .failed(Error::InvalidParticipationCost);

    goc.pick_winner(FOREIGN_USER)
        .failed(Error::AccessRestricted);

    goc.pick_winner(ADMIN).failed(Error::UnexpectedGameStatus);

    system.spend_blocks(DURATION_IN_SECS);
    goc.pick_winner(ADMIN).succeed(PLAYERS[0].into());
    goc.pick_winner(ADMIN).failed(Error::UnexpectedGameStatus);
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

    let mut fungible_token = FungibleToken::initialize(&system);
    let mut goc = Goc::initialize(&system, ADMIN).succeed();

    let ending = DURATION;
    let ft_actor_id = Some(fungible_token.actor_id());

    goc.start(ADMIN, DURATION, PARTICIPATION_COST, ft_actor_id)
        .succeed((ending, PARTICIPATION_COST, ft_actor_id));

    for player in PLAYERS.into_iter().take(2) {
        fungible_token.mint(player, AMOUNT);
        fungible_token.approve(player, goc.actor_id(), PARTICIPATION_COST);

        goc.enter(player).succeed(player);
    }

    goc.state().all().eq(State {
        admin: ADMIN.into(),
        started: system.block_timestamp(),
        ending,
        players: vec![PLAYERS[0].into(), PLAYERS[1].into()],
        prize_fund: u128::MAX,
        participation_cost: PARTICIPATION_COST,
        fungible_token: ft_actor_id,
        is_active: true,
        ..Default::default()
    })
}
