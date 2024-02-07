#![no_std]

use gstd::{
    collections::{HashMap, HashSet},
    debug, exec,
    ext::debug,
    msg,
    prelude::*,
    ActorId, MessageId, ReservationId,
};
use messages::*;
use syndote_io::*;
use utils::*;

pub mod game;
use crate::game::GameSessionActions;

pub mod messages;
pub mod strategic_actions;
use crate::strategic_actions::StrategicActions;
pub mod utils;

const EXISTENTIAL_DEPOSIT: u128 = 10_000_000_000_000;

#[derive(Clone, Default)]
pub struct GameManager {
    game_sessions: HashMap<AdminId, Game>,
    config: Config,
    awaiting_reply_msg_id_to_session_id: HashMap<MessageId, AdminId>,
}

static mut GAME_MANAGER: Option<GameManager> = None;

impl GameManager {
    fn create_game_session(&mut self, entry_fee: Option<u128>) -> Result<GameReply, GameError> {
        if let Some(fee) = entry_fee {
            if fee < EXISTENTIAL_DEPOSIT {
                return Err(GameError::FeeIsLessThanED);
            }
        }

        let admin_id = msg::source();
        if self.game_sessions.contains_key(&admin_id) {
            return Err(GameError::GameSessionAlreadyExists);
        }

        let mut game = Game {
            admin_id,
            entry_fee,
            ..Default::default()
        };
        game.init_properties();
        self.game_sessions.insert(admin_id, game);
        Ok(GameReply::GameSessionCreated { admin_id })
    }

    fn make_reservation(&mut self, admin_id: &AdminId) -> Result<GameReply, GameError> {
        let game = self
            .game_sessions
            .get_mut(admin_id)
            .ok_or(GameError::GameDoesNotExist)?;
        game.make_reservation(
            self.config.reservation_amount,
            self.config.reservation_duration_in_block,
        )?;
        Ok(GameReply::ReservationMade)
    }

    fn register(
        &mut self,
        admin_id: &AdminId,
        strategy_id: &ActorId,
    ) -> Result<GameReply, GameError> {
        let game = self
            .game_sessions
            .get_mut(admin_id)
            .ok_or(GameError::GameDoesNotExist)?;

        game.register(
            strategy_id,
            self.config.reservation_amount,
            self.config.reservation_duration_in_block,
        )?;
        Ok(GameReply::StrategyRegistered)
    }

    fn play(&mut self, admin_id: &AdminId) -> Result<GameReply, GameError> {
        let game = self
            .game_sessions
            .get_mut(admin_id)
            .ok_or(GameError::GameDoesNotExist)?;
        game.play(
            self.config.min_gas_limit,
            self.config.time_for_step,
            &mut self.awaiting_reply_msg_id_to_session_id,
            self.config.gas_refill_timeout,
        )
    }

    fn cancel_game_session(&mut self, admin_id: &AdminId) -> Result<GameReply, GameError> {
        let game = self
            .game_sessions
            .get_mut(admin_id)
            .ok_or(GameError::GameDoesNotExist)?;
        game.cancel_game_session()?;
        self.game_sessions.remove(admin_id);
        Ok(GameReply::GameWasCancelled)
    }

    fn exit_game(&mut self, admin_id: &AdminId) -> Result<GameReply, GameError> {
        let game = self
            .game_sessions
            .get_mut(admin_id)
            .ok_or(GameError::GameDoesNotExist)?;
        game.exit_game()?;
        Ok(GameReply::PlayerLeftGame)
    }

    fn add_gas_to_player_strategy(&mut self, admin_id: &AdminId) -> Result<GameReply, GameError> {
        let game = self
            .game_sessions
            .get_mut(admin_id)
            .ok_or(GameError::GameDoesNotExist)?;

        game.add_gas_to_player_strategy(
            self.config.reservation_amount,
            self.config.reservation_duration_in_block,
        )?;
        game.game_status = GameStatus::Play;
        Ok(GameReply::GasForPlayerStrategyAdded)
    }
}

#[no_mangle]
extern fn handle() {
    let action: GameAction = msg::load().expect("Could not load `GameAction`");
    let game_manager: &mut GameManager = unsafe {
        GAME_MANAGER
            .as_mut()
            .expect("Unexpected: Contract is not initialized")
    };
    let reply = match action {
        GameAction::CreateGameSession { entry_fee } => game_manager.create_game_session(entry_fee),
        GameAction::MakeReservation { admin_id } => game_manager.make_reservation(&admin_id),
        GameAction::Register {
            admin_id,
            strategy_id,
        } => game_manager.register(&admin_id, &strategy_id),
        GameAction::Play { admin_id } => game_manager.play(&admin_id),
        GameAction::AddGasToPlayerStrategy { admin_id } => {
            game_manager.add_gas_to_player_strategy(&admin_id)
        }
        GameAction::CancelGameSession { admin_id } => game_manager.cancel_game_session(&admin_id),
        GameAction::ExitGame { admin_id } => game_manager.exit_game(&admin_id),
    };
    msg::reply(reply, 0).expect("Error during sending a reply");
}

#[no_mangle]
unsafe extern fn init() {
    let config: Config = msg::load().expect("Error during init msg");
    let game_manager = GameManager {
        config,
        ..Default::default()
    };
    GAME_MANAGER = Some(game_manager);
}

#[no_mangle]
extern fn state() {
    let game_manager = unsafe { GAME_MANAGER.take().expect("Game is not initialized") };
    let query: StateQuery = msg::load().expect("Unable to load query");

    let reply = match query {
        StateQuery::GetGameSession { admin_id } => {
            if let Some(game_session) = game_manager.game_sessions.get(&admin_id) {
                let game_session: GameState = game_session.clone().into();
                StateReply::GameSession {
                    game_session: Some(game_session),
                }
            } else {
                StateReply::GameSession { game_session: None }
            }
        }
        StateQuery::GetPlayerInfo {
            admin_id,
            account_id,
        } => {
            if let Some(game_session) = game_manager.game_sessions.get(&admin_id) {
                if let Some(strategy_id) = game_session.owners_to_strategy_ids.get(&account_id) {
                    let player_info = game_session.players.get(strategy_id).cloned();
                    StateReply::PlayerInfo { player_info }
                } else {
                    StateReply::PlayerInfo { player_info: None }
                }
            } else {
                StateReply::PlayerInfo { player_info: None }
            }
        }
    };
    msg::reply(reply, 0).expect("Failed to share state");
}

#[no_mangle]
extern fn handle_reply() {
    let reply_to = msg::reply_to().expect("Unable to get the msg id");

    let game_manager: &mut GameManager = unsafe {
        GAME_MANAGER
            .as_mut()
            .expect("Unexpected: Contract is not initialized")
    };

    let admin_id = game_manager
        .awaiting_reply_msg_id_to_session_id
        .remove(&reply_to)
        .expect("Received a reply to a msg that does not need to be processed in handle reply.");

    let game = game_manager
        .game_sessions
        .get_mut(&admin_id)
        .expect("Unexpected: Game does not exist");

    let reply: Result<StrategicAction, gstd::errors::Error> = msg::load();
    match reply {
        Ok(strategic_action) => match strategic_action {
            StrategicAction::AddGear {
                properties_for_sale,
            } => game.add_gear(properties_for_sale),
            StrategicAction::BuyCell {
                properties_for_sale,
            } => game.buy_cell(properties_for_sale),
            StrategicAction::PayRent {
                properties_for_sale,
            } => game.pay_rent(properties_for_sale),
            StrategicAction::ThrowRoll {
                pay_fine,
                properties_for_sale,
            } => game.throw_roll(pay_fine, properties_for_sale),
            StrategicAction::Upgrade {
                properties_for_sale,
            } => game.upgrade(properties_for_sale),
            StrategicAction::Skip => {}
        },
        _ => {
            game.exclude_player_from_game(game.current_player);
            game.current_turn = game.current_turn.saturating_sub(1);
        }
    };

    game.finalize_turn_outcome(
        game_manager.config.min_gas_limit,
        game_manager.config.reservation_duration_in_block,
    )
}
