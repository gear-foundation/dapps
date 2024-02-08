use gstd::prelude::*;
use gtest::{Program, RunResult, System};
use syndote_io::*;
pub const ADMIN_ID: u64 = 10;
pub const PLAYERS: [u64; 4] = [11, 12, 13, 14];
pub trait SyndoteTestFunctions {
    fn game(system: &System, from: u64, config: Config) -> Program<'_>;
    fn create_game_session(&self, from: u64, entry_fee: Option<u128>, error: Option<GameError>);
    fn make_reservation(&self, from: u64, admin_id: u64, error: Option<GameError>);
    fn register(
        &self,
        from: u64,
        admin_id: u64,
        strategy_id: [u8; 32],
        entry_fee: Option<u128>,
        error: Option<GameError>,
    );
    fn play(&self, from: u64, admin_id: u64);
    fn cancel_game_session(&self, from: u64, admin_id: u64, error: Option<GameError>);
    fn exit_game(&self, from: u64, admin_id: u64, error: Option<GameError>);
    fn get_game_session(&self, admin_id: u64) -> Option<GameState>;
    fn get_player_info(&self, admin_id: u64, account_id: u64) -> Option<PlayerInfo>;
}

impl SyndoteTestFunctions for Program<'_> {
    fn game(system: &System, from: u64, config: Config) -> Program<'_> {
        let game = Program::current(system);
        let result = game.send(from, config);
        assert!(!result.main_failed());
        game
    }

    fn create_game_session(&self, from: u64, entry_fee: Option<u128>, error: Option<GameError>) {
        let result = self.send(from, GameAction::CreateGameSession { entry_fee });
        check_reply(
            &result,
            from,
            GameReply::GameSessionCreated {
                admin_id: from.into(),
            },
            error,
        );
    }
    fn make_reservation(&self, from: u64, admin_id: u64, error: Option<GameError>) {
        let result = self.send(
            from,
            GameAction::MakeReservation {
                admin_id: admin_id.into(),
            },
        );
        check_reply(&result, from, GameReply::ReservationMade, error);
    }
    fn register(
        &self,
        from: u64,
        admin_id: u64,
        strategy_id: [u8; 32],
        entry_fee: Option<u128>,
        error: Option<GameError>,
    ) {
        let result = if let Some(fee) = entry_fee {
            self.send_with_value(
                from,
                GameAction::Register {
                    admin_id: admin_id.into(),
                    strategy_id: strategy_id.into(),
                },
                fee,
            )
        } else {
            self.send(
                from,
                GameAction::Register {
                    admin_id: admin_id.into(),
                    strategy_id: strategy_id.into(),
                },
            )
        };
        check_reply(&result, from, GameReply::StrategyRegistered, error);
    }
    fn play(&self, from: u64, admin_id: u64) {
        let result = self.send(
            from,
            GameAction::Play {
                admin_id: admin_id.into(),
            },
        );
        assert!(!result.main_failed());
    }
    fn cancel_game_session(&self, from: u64, admin_id: u64, error: Option<GameError>) {
        let result = self.send(
            from,
            GameAction::CancelGameSession {
                admin_id: admin_id.into(),
            },
        );
        check_reply(&result, from, GameReply::GameWasCancelled, error);
    }

    fn exit_game(&self, from: u64, admin_id: u64, error: Option<GameError>) {
        let result = self.send(
            from,
            GameAction::ExitGame {
                admin_id: admin_id.into(),
            },
        );
        check_reply(&result, from, GameReply::PlayerLeftGame, error);
    }

    fn get_game_session(&self, admin_id: u64) -> Option<GameState> {
        let reply: StateReply = self
            .read_state(StateQuery::GetGameSession {
                admin_id: admin_id.into(),
            })
            .expect("Unable to read varatube state");
        if let StateReply::GameSession { game_session } = reply {
            game_session
        } else {
            std::panic!("Wrong received reply");
        }
    }

    fn get_player_info(&self, admin_id: u64, account_id: u64) -> Option<PlayerInfo> {
        let reply: StateReply = self
            .read_state(StateQuery::GetPlayerInfo {
                admin_id: admin_id.into(),
                account_id: account_id.into(),
            })
            .expect("Unable to read varatube state");
        if let StateReply::PlayerInfo { player_info } = reply {
            player_info
        } else {
            std::panic!("Wrong received reply");
        }
    }
}

fn check_reply(result: &RunResult, from: u64, expected_reply: GameReply, error: Option<GameError>) {
    let reply: Result<GameReply, GameError>;
    if let Some(error) = error {
        reply = Err(error);
    } else {
        reply = Ok(expected_reply);
    }
    assert!(result.contains(&(from, reply.encode())));
}

pub fn preconfigure(system: &System) -> Program<'_> {
    let syndote = Program::game(
        system,
        ADMIN_ID,
        Config {
            reservation_amount: 700_000_000_000,
            reservation_duration_in_block: 10_000,
            time_for_step: 10,
            min_gas_limit: 5_000_000_000,
            gas_refill_timeout: 20,
        },
    );

    syndote
}

pub fn upload_strategy(system: &System) -> Program<'_> {
    Program::from_file(
        system,
        "../target/wasm32-unknown-unknown/debug/syndote_player.opt.wasm",
    )
}
