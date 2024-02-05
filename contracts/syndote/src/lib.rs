#![no_std]

use gstd::{
    collections::{HashMap, HashSet},
    debug, exec, msg,
    prelude::*,
    ActorId, MessageId, ReservationId,
};
use messages::*;
use syndote_io::*;
use utils::*;

pub mod game;
pub mod messages;
pub mod strategic_actions;
pub mod utils;
use game::Game;

#[derive(Clone, Default)]
pub struct GameManager {
    game_sessions: HashMap<SessionId, Game>,
    current_session_id: SessionId,
    config: Config,
    awaiting_reply_msg_id_to_session_id: HashMap<MessageId, SessionId>,
}

static mut GAME_MANAGER: Option<GameManager> = None;

impl GameManager {
    fn create_game_session(&mut self) -> Result<GameReply, GameError> {
        let session_id = self.current_session_id;
        self.current_session_id = self.current_session_id.wrapping_add(1);
        let mut game = Game {
            admin: msg::source(),
            ..Default::default()
        };
        game.init_properties();
        self.game_sessions.insert(session_id, game);
        Ok(GameReply::GameSessionCreated { session_id })
    }

    fn make_reservation(&mut self, session_id: SessionId) -> Result<GameReply, GameError> {
        let game = get_game_session(&mut self.game_sessions, session_id)?;
        game.make_reservation(
            self.config.reservation_amount,
            self.config.reservation_duration_in_block,
        )?;
        Ok(GameReply::ReservationMade)
    }

    fn register(
        &mut self,
        session_id: SessionId,
        strategy_id: &ActorId,
    ) -> Result<GameReply, GameError> {
        let game = get_game_session(&mut self.game_sessions, session_id)?;
        game.register(
            strategy_id,
            self.config.reservation_amount,
            self.config.reservation_duration_in_block,
        )?;
        Ok(GameReply::StrategyRegistered)
    }

    fn play(&mut self, session_id: SessionId) -> Result<GameReply, GameError> {
        let game = get_game_session(&mut self.game_sessions, session_id)?;
        game.play(
            session_id,
            self.config.min_gas_limit,
            self.config.time_for_step,
            &mut self.awaiting_reply_msg_id_to_session_id,
        )
    }

    fn add_gas_to_player_strategy(
        &mut self,
        session_id: SessionId,
    ) -> Result<GameReply, GameError> {
        let game = get_game_session(&mut self.game_sessions, session_id)?;
        game.add_gas_to_player_strategy(
            self.config.reservation_amount,
            self.config.reservation_duration_in_block,
        )?;
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
        GameAction::CreateGameSession => game_manager.create_game_session(),
        GameAction::MakeReservation { session_id } => game_manager.make_reservation(session_id),
        GameAction::Register {
            session_id,
            strategy_id,
        } => game_manager.register(session_id, &strategy_id),
        GameAction::Play { session_id } => game_manager.play(session_id),
        GameAction::AddGasToPlayerStrategy { session_id } => {
            game_manager.add_gas_to_player_strategy(session_id)
        }
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

// #[no_mangle]
// extern fn state() {
//     let game = unsafe { GAME.take().expect("Game is not initialized") };
//     let query: StateQuery = msg::load().expect("Unable to load query");

//     let reply = match query {
//         StateQuery::MessageId => StateReply::MessageId(game.current_msg_id),
//         StateQuery::GameState => StateReply::GameState(GameState {
//             admin: game.admin,
//             properties_in_bank: game.properties_in_bank.into_iter().collect(),
//             round: game.round,
//             players: game.players.into_iter().collect(),
//             players_queue: game.players_queue,
//             current_player: game.current_player,
//             current_step: game.current_step,
//             properties: game.properties,
//             ownership: game.ownership,
//             game_status: game.game_status,
//             winner: game.winner,
//             current_turn: game.current_turn,
//         }),
//     };
//     msg::reply(reply, 0).expect("Failed to share state");
// }

#[no_mangle]
extern fn handle_reply() {
    let reply_to = msg::reply_to().expect("Unable to get the msg id");

    let game_manager: &mut GameManager = unsafe {
        GAME_MANAGER
            .as_mut()
            .expect("Unexpected: Contract is not initialized")
    };

    let session_id = game_manager
        .awaiting_reply_msg_id_to_session_id
        .remove(&reply_to)
        .expect("Received a reply to a msg that does not need to be processed in handle reply.");

    let game = get_game_session(&mut game_manager.game_sessions, session_id)
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
