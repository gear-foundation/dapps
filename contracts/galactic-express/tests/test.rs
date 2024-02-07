use utils::prelude::*;

mod utils;

#[test]
fn test() {
    let system = utils::initialize_system();
    let mut rockets = GalEx::initialize(&system, ADMINS[0]);

    for (i, admin_id) in ADMINS.into_iter().enumerate() {
        for session_id in 0..3 {
            let bid = 11_000_000_000_000;
            system.mint_to(admin_id, bid);
            rockets
                .create_new_session(admin_id, bid)
                .succeed(session_id as u128, 0);

            let player = Participant {
                fuel_amount: 42,
                payload_amount: 20,
            };

            for player_id in PLAYERS {
                system.mint_to(player_id, bid);
                rockets
                    .register(player_id, admin_id.into(), player, bid)
                    .succeed((player_id, player), 0);
            }

            let state = rockets.state();

            if let StageState::Registration(participants) = &state.games[i].1.stage {
                assert_eq!(participants.len(), 3);
            }

            rockets
                .start_game(admin_id, player)
                .succeed(PLAYERS.into_iter().chain(iter::once(admin_id)).collect(), 3); // 3 since three players win and msg::send_with_gas is sent to them

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
        .register(PLAYERS[0], ADMINS[0].into(), Default::default(), 0)
        .failed(Error::NoSuchGame, 0);

    rockets.create_new_session(ADMINS[0], 0).succeed(0, 0);

    rockets
        .register(ADMINS[0], ADMINS[0].into(), Default::default(), 0)
        .failed(Error::AccessDenied, 0);

    rockets
        .start_game(PLAYERS[0], Default::default())
        .failed(Error::NoSuchGame, 0);

    rockets
        .start_game(ADMINS[0], Default::default())
        .failed(Error::NotEnoughParticipants, 0);

    for player in PLAYERS {
        rockets
            .register(player, ADMINS[0].into(), Default::default(), 0)
            .succeed((player, Default::default()), 0);
    }

    rockets
        .start_game(
            ADMINS[0],
            Participant {
                fuel_amount: 101,
                payload_amount: 100,
            },
        )
        .failed(Error::FuelOrPayloadOverload, 0);

    rockets
        .start_game(
            ADMINS[0],
            Participant {
                fuel_amount: 100,
                payload_amount: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload, 0);
    rockets
        .start_game(
            ADMINS[0],
            Participant {
                fuel_amount: 101,
                payload_amount: 101,
            },
        )
        .failed(Error::FuelOrPayloadOverload, 0);

    rockets
        .register(FOREIGN_USER, ADMINS[0].into(), Default::default(), 0)
        .failed(Error::SessionFull, 0);
}
