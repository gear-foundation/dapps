use utils::{prelude::*, Sft};

mod utils;

#[test]
fn test() {
    let system = utils::initialize_system();

    let sft = Sft::initialize(&system);

    let mut balances: HashMap<ActorId, u128> = PLAYERS
        .into_iter()
        .chain(ADMINS)
        .map(|actor| (actor.into(), 0))
        .collect();

    for admin_id in ADMINS {
        let mut rockets = GalEx::initialize(&system, admin_id, sft.actor_id());
        if let State {
            admin,
            session: Session { id: 0, .. },
            sft: true_sft,
            stage: Stage::Results(results),
        } = rockets.state()
        {
            assert_eq!(admin, admin_id.into());
            assert_eq!(true_sft, sft.actor_id());
            assert_eq!(results, Results::default());
        } else {
            unreachable!()
        }

        for (session_id, starter) in [admin_id, PLAYERS[0]].into_iter().enumerate() {
            rockets
                .create_new_session(starter)
                .succeed(session_id as u128);

            let player = (
                ADMINS[0],
                Participant {
                    fuel: 42,
                    payload: 24,
                },
            );

            for player_id in PLAYERS {
                rockets
                    .register(player_id, player.1)
                    .succeed((player_id, player.1));
            }
            if let State {
                stage: Stage::Registration(participants),
                ..
            } = rockets.state()
            {
                assert_eq!(
                    HashMap::from_iter(
                        PLAYERS
                            .into_iter()
                            .map(|player_id| (player_id.into(), player.1))
                    ),
                    participants.into_iter().collect::<HashMap<_, _>>(),
                );
            } else {
                unreachable!()
            }

            rockets
                .start_game(admin_id, player.1)
                .succeed(PLAYERS.into_iter().chain(iter::once(admin_id)).collect());
            let rankings = if let State {
                session:
                    Session {
                        id: true_session_id,
                        reward,
                        ..
                    },
                stage: Stage::Results(Results { rankings, .. }),
                ..
            } = rockets.state()
            {
                assert_eq!(true_session_id, session_id as u128 + 1);

                let deductible = reward / 10 * 6 / PARTICIPANTS as u128;

                assert!(rankings.iter().all(|(_, true_reward)| (REWARD.0..REWARD.1)
                    .contains(true_reward)
                    && (*true_reward == reward
                        || *true_reward == 0
                        || *true_reward == reward - deductible
                        || *true_reward == reward - deductible * 2
                        || *true_reward == reward - deductible * 3)));

                rankings
            } else {
                unreachable!()
            };

            for (actor, reward) in rankings {
                let bal = balances.get_mut(&actor).unwrap();

                *bal += reward;

                sft.balance(actor).contains(*bal)
            }
        }
    }
}

#[test]
fn errors() {
    let system = utils::initialize_system();

    let sft = Sft::initialize(&system);
    let mut rockets = GalEx::initialize(&system, ADMINS[0], sft.actor_id());

    rockets
        .change_admin(PLAYERS[0], PLAYERS[0])
        .failed(Error::AccessDenied);
    rockets
        .change_admin(ADMINS[0], ADMINS[1])
        .succeed((ADMINS[0], ADMINS[1]));

    rockets
        .register(PLAYERS[0], Default::default())
        .failed(Error::SessionEnded);
    rockets
        .start_game(PLAYERS[0], Default::default())
        .failed(Error::AccessDenied);

    rockets.create_new_session(ADMINS[1]).succeed(0);

    rockets
        .start_game(ADMINS[1], Default::default())
        .failed(Error::NotEnoughParticipants);

    for player in PLAYERS {
        rockets
            .register(player, Default::default())
            .succeed((player, Default::default()));
    }

    rockets
        .start_game(
            ADMINS[1],
            Participant {
                fuel: 101,
                payload: 100,
            },
        )
        .failed(Error::FuelOrPayloadOverload);
    rockets
        .start_game(
            ADMINS[1],
            Participant {
                fuel: 100,
                payload: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload);
    rockets
        .start_game(
            ADMINS[1],
            Participant {
                fuel: 101,
                payload: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload);
    rockets
        .register(ADMINS[1], Default::default())
        .failed(Error::AccessDenied);
    rockets
        .register(FOREIGN_USER, Default::default())
        .failed(Error::SessionFull);
}
