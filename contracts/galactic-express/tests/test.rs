use galactic_express_io::*;
use utils::prelude::*;
mod utils;

#[test]
fn test() {
    let system = utils::initialize_system();

    for admin_id in ADMINS {
        let mut rockets = Rockets::initialize(&system, admin_id);
        if let State {
            admin,
            session: Session { session_id: 0, .. },
            is_session_ended: true,
            participants,
            turns,
            rankings,
        } = rockets.state()
        {
            assert_eq!(admin, admin_id.into());
            assert_eq!((participants, turns, rankings), (vec![], vec![], vec![]));
        } else {
            unreachable!()
        }

        for (session_id, starter) in [admin_id, PLAYERS[0]]
            .into_iter()
            .enumerate()
            .map(|e| ((e.0 + 1) as u128, e.1))
        {
            rockets.create_new_session(starter).succeed(session_id - 1);
            if let State {
                session:
                    Session {
                        altitude,
                        weather,
                        fuel_price,
                        reward,
                        ..
                    },
                is_session_ended: false,
                ..
            } = rockets.state()
            {
                assert!((weather as u8) < 6);
                assert!(
                    altitude >= MIN_TURN_ALTITUDE * (TOTAL_TURNS as u16)
                        && altitude < MAX_TURN_ALTITUDE * (TOTAL_TURNS as u16)
                );
                assert!((MIN_FUEL_PRICE..MAX_FUEL_PRICE).contains(&fuel_price));
                assert!((MIN_REWARD..MAX_REWARD).contains(&reward));
            } else {
                unreachable!()
            }

            let mut player = (
                ADMINS[0],
                Participant {
                    fuel_amount: 42,
                    payload_amount: 24,
                },
            );

            for player_id in PLAYERS {
                player.0 = player_id;

                rockets
                    .register(player.0, player.1)
                    .succeed((player.0, player.1));
            }
            let State { participants, .. } = rockets.state();
            assert_eq!(
                HashMap::from_iter(
                    PLAYERS
                        .into_iter()
                        .map(|player_id| (player_id.into(), player.1))
                ),
                participants.into_iter().collect::<HashMap<_, _>>(),
            );

            rockets
                .start_game(admin_id, player.1)
                .succeed(PLAYERS.into_iter().chain(iter::once(admin_id)).collect());
            if let State {
                participants,
                session:
                    Session {
                        session_id: true_sess_id,
                        reward,
                        ..
                    },
                is_session_ended: true,
                rankings,
                ..
            } = rockets.state()
            {
                assert_eq!(true_sess_id, session_id);
                assert_eq!(
                    HashMap::from_iter(
                        PLAYERS
                            .into_iter()
                            .map(|player_id| (player_id.into(), player.1))
                            .chain(iter::once((admin_id.into(), player.1)))
                    ),
                    participants.into_iter().collect::<HashMap<_, _>>(),
                );
                assert_eq!(rankings.len(), 4);
                assert!(rankings
                    .iter()
                    .all(|(_, true_reward)| *true_reward >= MIN_REWARD
                        && *true_reward < MAX_REWARD
                        && (*true_reward == reward
                            || *true_reward == 0
                            || *true_reward == reward / 10 * 8
                            || *true_reward == reward / 10 * 6
                            || *true_reward == reward / 10 * 4)));
            } else {
                unreachable!()
            };
        }
    }
}

#[test]
fn errors() {
    let system = utils::initialize_system();

    let mut rockets = Rockets::initialize(&system, ADMINS[0]);

    rockets
        .change_admin(PLAYERS[0], PLAYERS[0])
        .failed(Error::AccessDenied);
    rockets
        .change_admin(ADMINS[0], ADMINS[1])
        .succeed((ADMINS[0], ADMINS[1]));

    rockets
        .register(PLAYERS[0], Default::default())
        .failed(Error::EndedSession);
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
                fuel_amount: 101,
                payload_amount: 100,
            },
        )
        .failed(Error::FuelOrPayloadOverload);
    rockets
        .start_game(
            ADMINS[1],
            Participant {
                fuel_amount: 100,
                payload_amount: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload);
    rockets
        .start_game(
            ADMINS[1],
            Participant {
                fuel_amount: 101,
                payload_amount: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload);
    rockets
        .register(ADMINS[1], Default::default())
        .failed(Error::AccessDenied);
    rockets
        .register(FOREIGN_USER, Default::default())
        .failed(Error::FullSession);
}
