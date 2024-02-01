#![no_std]
use battleship_io::{
    BattleshipAction, BattleshipError, BattleshipInit, BattleshipParticipants, BattleshipReply,
    BattleshipState, BotBattleshipAction, Entity, Game, GameState, Ships, StateQuery, StateReply,
    Step,
};
use gstd::{
    collections::{BTreeMap, HashMap},
    exec, msg,
    prelude::*,
    ActorId, MessageId,
};

static mut BATTLESHIP: Option<Battleship> = None;
pub const GAS_FOR_START: u64 = 100_000_000_000;
pub const GAS_FOR_MOVE: u64 = 100_000_000_000;

#[derive(Debug, Default)]
struct Battleship {
    pub games: HashMap<ActorId, Game>,
    pub msg_id_to_game_id: BTreeMap<MessageId, ActorId>,
    pub bot_address: ActorId,
    pub admin: ActorId,
}

impl Battleship {
    fn start_game(&mut self, mut ships: Ships) -> Result<BattleshipReply, BattleshipError> {
        let msg_source = msg::source();

        if let Some(game) = self.games.get(&msg_source) {
            if !game.game_over {
                return Err(BattleshipError::GameIsAlreadyStarted);
            }
        }
        ships.check_correct_location()?;
        let player_board = ships.get_field()?;
        ships.sort_by_length();
        let game_instance = Game {
            player_board,
            player_ships: ships,
            turn: Some(BattleshipParticipants::Player),
            start_time: exec::block_timestamp(),
            game_over: false,
            game_result: None,
            total_shots: 0,
            ..Default::default()
        };
        self.games.insert(msg_source, game_instance);

        let msg_id = msg::send_with_gas(
            self.bot_address,
            BotBattleshipAction::Start,
            GAS_FOR_START,
            0,
        )
        .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, msg_source);
        Ok(BattleshipReply::MessageSentToBot)
    }

    fn player_move(&mut self, step: u8) -> Result<BattleshipReply, BattleshipError> {
        let msg_source = msg::source();
        if step > 24 {
            return Err(BattleshipError::OutOfBounds);
        }

        let Some(game) = self.games.get_mut(&msg_source) else {
            return Err(BattleshipError::GameIsNotStarted);
        };
        if game.game_over {
            return Err(BattleshipError::GameIsAlreadyOver);
        }

        if game.bot_board.is_empty() {
            return Err(BattleshipError::BotDidNotInitializeBoard);
        }

        if game.turn != Some(BattleshipParticipants::Player) {
            return Err(BattleshipError::NotYourTurn);
        }

        if game.bot_board[step as usize] != Entity::Empty
            && game.bot_board[step as usize] != Entity::Ship
        {
            return Err(BattleshipError::ThisCellAlreadyKnown);
        }

        let res = game.bot_ships.bang(step);
        match res {
            Step::Missed => game.bot_board[step as usize] = Entity::Boom,
            Step::Injured => game.bot_board[step as usize] = Entity::BoomShip,
            Step::Killed => game.dead_ship(step, 1),
        }
        game.total_shots += 1;
        if game.bot_ships.check_end_game() {
            game.game_over = true;
            game.game_result = Some(BattleshipParticipants::Player);
            game.end_time = exec::block_timestamp();
            return Ok(BattleshipReply::GameFinished(
                BattleshipParticipants::Player,
            ));
        }
        game.turn = Some(BattleshipParticipants::Bot);

        let board = game.get_hidden_field();
        let msg_id = msg::send_with_gas(
            self.bot_address,
            BotBattleshipAction::Turn(board),
            GAS_FOR_MOVE,
            0,
        )
        .expect("Error in sending a message");

        self.msg_id_to_game_id.insert(msg_id, msg_source);
        Ok(BattleshipReply::MessageSentToBot)
    }

    fn change_bot(&mut self, bot: ActorId) -> Result<BattleshipReply, BattleshipError> {
        if msg::source() != self.admin {
            return Err(BattleshipError::NotAdmin);
        }
        self.bot_address = bot;

        Ok(BattleshipReply::BotChanged(bot))
    }

    fn clear_state(
        &mut self,
        leave_active_games: bool,
    ) -> Result<BattleshipReply, BattleshipError> {
        if msg::source() != self.admin {
            return Err(BattleshipError::NotAdmin);
        }
        if leave_active_games {
            self.games.retain(|_actor_id, game| !game.game_over);
        } else {
            self.games.clear();
        }
        Ok(BattleshipReply::StateCleared)
    }

    fn delete_game(&mut self, player_address: ActorId) -> Result<BattleshipReply, BattleshipError> {
        if msg::source() != self.admin {
            return Err(BattleshipError::NotAdmin);
        }
        self.games.remove(&player_address);
        Ok(BattleshipReply::GameDeleted)
    }
}

#[no_mangle]
extern fn init() {
    let BattleshipInit { bot_address } = msg::load().expect("Unable to decode BattleshipInit");
    unsafe {
        BATTLESHIP = Some(Battleship {
            bot_address,
            admin: msg::source(),
            ..Default::default()
        });
    }
}

#[no_mangle]
extern fn handle() {
    let battleship = unsafe {
        BATTLESHIP
            .as_mut()
            .expect("`Battleship` is not initialized.")
    };
    let action: BattleshipAction =
        msg::load().expect("Failed to decode `BattleshipAction` message.");
    let reply = match action {
        BattleshipAction::StartGame { ships } => battleship.start_game(ships),
        BattleshipAction::Turn { step } => battleship.player_move(step),
        BattleshipAction::ChangeBot { bot } => battleship.change_bot(bot),
        BattleshipAction::ClearState { leave_active_games } => {
            battleship.clear_state(leave_active_games)
        }
        BattleshipAction::DeleteGame { player_address } => battleship.delete_game(player_address),
    };
    msg::reply(reply, 0)
        .expect("Failed to encode or reply with `Result<BattleshipReply, BattleshipError>`.");
}

#[no_mangle]
extern fn handle_reply() {
    let reply_to = msg::reply_to().expect("Failed to query reply_to data");
    let battleship = unsafe { BATTLESHIP.as_mut().expect("The game is not initialized") };
    let game_id = battleship
        .msg_id_to_game_id
        .remove(&reply_to)
        .expect("Unexpected reply");

    let game = battleship
        .games
        .get_mut(&game_id)
        .expect("Unexpected: Game does not exist");

    let action: BattleshipAction =
        msg::load().expect("Failed to decode `BattleshipAction` message.");
    match action {
        BattleshipAction::StartGame { ships } => game.start_bot(ships),
        BattleshipAction::Turn { step } => {
            game.turn(step);
            game.turn = Some(BattleshipParticipants::Player);
            if game.player_ships.check_end_game() {
                game.game_over = true;
                game.game_result = Some(BattleshipParticipants::Bot);
                game.end_time = exec::block_timestamp();
            }
        }
        _ => (),
    }
}

#[no_mangle]
extern fn state() {
    let battleship = unsafe { BATTLESHIP.take().expect("Unexpected error in taking state") };
    let query: StateQuery = msg::load().expect("Unable to load the state query");
    match query {
        StateQuery::All => {
            msg::reply(StateReply::All(battleship.into()), 0).expect("Unable to share the state");
        }
        StateQuery::Game(player_id) => {
            let game_state = battleship.games.get(&player_id).map(|game| GameState {
                player_board: game.player_board.clone(),
                bot_board: game.bot_board.clone(),
                player_ships: game.player_ships.count_alive_ships(),
                bot_ships: game.bot_ships.count_alive_ships(),
                turn: game.turn.clone(),
                start_time: game.start_time,
                total_shots: game.total_shots,
                end_time: game.end_time,
                game_over: game.game_over,
                game_result: game.game_result.clone(),
            });

            msg::reply(StateReply::Game(game_state), 0).expect("Unable to share the state");
        }
        StateQuery::BotContractId => {
            msg::reply(StateReply::BotContractId(battleship.bot_address), 0)
                .expect("Unable to share the state");
        }
    }
}

impl From<Battleship> for BattleshipState {
    fn from(value: Battleship) -> Self {
        let Battleship {
            games,
            bot_address,
            admin,
            ..
        } = value;

        let games: Vec<(ActorId, GameState)> = games
            .iter()
            .map(|(id, game)| {
                let game = GameState {
                    player_board: game.player_board.clone(),
                    bot_board: game.bot_board.clone(),
                    player_ships: game.player_ships.clone().count_alive_ships(),
                    bot_ships: game.bot_ships.clone().count_alive_ships(),
                    turn: game.turn.clone(),
                    start_time: game.start_time,
                    total_shots: game.total_shots,
                    end_time: game.end_time,
                    game_over: game.game_over,
                    game_result: game.game_result.clone(),
                };
                (*id, game)
            })
            .collect();

        Self {
            games,
            bot_address,
            admin,
        }
    }
}
