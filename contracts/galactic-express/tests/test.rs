use utils::prelude::*;

mod utils;

#[test]
fn test() {
    let system = utils::initialize_system();

    for admin_id in ADMINS {
        let mut rockets = GalEx::initialize(&system, admin_id);
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

        for (session_id, starter) in [admin_id, PLAYERS[0]].into_iter().enumerate() {
            rockets
                .create_new_session(starter)
                .succeed(session_id as u128);

            let player = Participant {
                fuel_amount: 42,
                payload_amount: 20,
            };

            for player_id in PLAYERS {
                rockets
                    .register(player_id, player)
                    .succeed((player_id, player));
            }
            #[allow(irrefutable_let_patterns)]
            if let State {
                participants,
                ..
            } = rockets.state()
            {
                assert_eq!(
                    HashMap::from_iter(
                        PLAYERS
                            .into_iter()
                            .map(|player_id| (player_id.into(), player))
                    ),
                    participants.into_iter().collect::<HashMap<_, _>>(),
                );
            } else {
                unreachable!()
            }

            rockets
                .start_game(admin_id, player)
                .succeed(PLAYERS.into_iter().chain(iter::once(admin_id)).collect());

        }
    }
}

#[test]
fn errors() {
    let system = utils::initialize_system();

    let mut rockets = GalEx::initialize(&system, ADMINS[0]);

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
        .failed(Error::SessionFull);
}
