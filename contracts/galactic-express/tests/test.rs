use utils::prelude::*;

mod utils;

#[test]
fn test() {
    let system = utils::initialize_system();
    let mut rockets = GalEx::initialize(&system, ADMINS[0]);

    for (i, admin_id) in ADMINS.into_iter().enumerate() {
        for session_id in 0..3 {
            rockets
                .create_new_session(admin_id)
                .succeed(session_id as u128);

            let player = Participant {
                fuel_amount: 42,
                payload_amount: 20,
            };

            for player_id in PLAYERS {
                rockets
                    .register(player_id, admin_id.into(), player)
                    .succeed((player_id, player));
            }

            let state = rockets.state();

            if let StageState::Registration(participants) = &state.games[i].1.stage {
                assert_eq!(participants.len(), 3);
            }

            rockets
                .start_game(admin_id, player)
                .succeed(PLAYERS.into_iter().chain(iter::once(admin_id)).collect());

            let state = rockets.state();

            if let StageState::Results(results) = &state.games[i].1.stage {
                assert_eq!(results.rankings.len(), 4);
            }
        }
    }
}

#[test]
fn errors() {
    let system = utils::initialize_system();

    let mut rockets = GalEx::initialize(&system, ADMINS[0]);

    rockets
        .register(PLAYERS[0], ADMINS[0].into(), Default::default())
        .failed(Error::NoSuchGame);

    rockets.create_new_session(ADMINS[0]).succeed(0);

    rockets
        .register(ADMINS[0], ADMINS[0].into(), Default::default())
        .failed(Error::AccessDenied);

    rockets
        .start_game(PLAYERS[0], Default::default())
        .failed(Error::NoSuchGame);

    rockets
        .start_game(ADMINS[0], Default::default())
        .failed(Error::NotEnoughParticipants);

    for player in PLAYERS {
        rockets
            .register(player, ADMINS[0].into(), Default::default())
            .succeed((player, Default::default()));
    }

    rockets
        .start_game(
            ADMINS[0],
            Participant {
                fuel_amount: 101,
                payload_amount: 100,
            },
        )
        .failed(Error::FuelOrPayloadOverload);

    rockets
        .start_game(
            ADMINS[0],
            Participant {
                fuel_amount: 100,
                payload_amount: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload);
    rockets
        .start_game(
            ADMINS[0],
            Participant {
                fuel_amount: 101,
                payload_amount: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload);

    rockets
        .register(FOREIGN_USER, ADMINS[0].into(), Default::default())
        .failed(Error::SessionFull);
}
