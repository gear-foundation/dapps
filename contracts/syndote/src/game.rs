use crate::{
    bankrupt_and_penalty, get_rolls, init_properties, msg_to_play_game, take_your_turn, AdminId,
    Game, GameError, GameInfo, GameReply, GameStatus, PlayerInfo,
};
use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId, MessageId, ReservationId};
pub const NUMBER_OF_CELLS: u8 = 40;
pub const NUMBER_OF_PLAYERS: u8 = 4;
pub const JAIL_POSITION: u8 = 10;
pub const LOTTERY_POSITION: u8 = 20;
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;
pub const PENALTY: u8 = 5;
pub const INITIAL_BALANCE: u32 = 15_000;
pub const NEW_CIRCLE: u32 = 2_000;

pub trait GameSessionActions {
    fn init_properties(&mut self);
    fn make_reservation(
        &mut self,
        reservation_amount: u64,
        reservation_duration_in_block: u32,
    ) -> Result<(), GameError>;
    fn register(
        &mut self,
        player_id: &ActorId,
        strategy_id: &ActorId,
        name: &str,
        reservation_amount: u64,
        reservation_duration_in_block: u32,
    ) -> Result<(), GameError>;
    fn cancel_game_session(&mut self) -> Result<(), GameError>;
    fn exit_game(&mut self) -> Result<(), GameError>;
    fn delete_player(&mut self, player_id: &ActorId) -> Result<(), GameError>;
    fn play(
        &mut self,
        min_gas_limit: u64,
        time_for_step: u32,
        awaiting_reply_msg_id_to_session_id: &mut HashMap<MessageId, AdminId>,
        gas_refill_timeout: u32,
    ) -> Result<GameReply, GameError>;
    fn make_step(
        &mut self,
        time_for_step: u32,
        awaiting_reply_msg_id_to_session_id: &mut HashMap<MessageId, AdminId>,
        gas_refill_timeout: u32,
    ) -> Result<(), GameError>;
    fn check_amount_of_players(&mut self);
    fn finalize_turn_outcome(
        &mut self,
        gas_for_step: u64,
        min_gas_limit: u64,
        reservation_duration_in_block: u32,
    );
    fn add_gas_to_player_strategy(
        &mut self,
        reservation_amount: u64,
        reservation_duration_in_block: u32,
    ) -> Result<(), GameError>;
    fn check_status(&self, game_status: GameStatus) -> Result<(), GameError>;
    fn only_admin(&self) -> Result<(), GameError>;
    fn only_admin_or_program(
        &self,
        program_id: &ActorId,
        msg_source: &ActorId,
    ) -> Result<(), GameError>;
    fn get_game_info(&self) -> GameInfo;
    fn get_player_info(&self) -> Result<PlayerInfo, GameError>;
    fn exclude_player_from_game(&mut self, player: ActorId);
    fn player_already_registered(
        &self,
        owner_id: &ActorId,
        strategy_id: &ActorId,
    ) -> Result<(), GameError>;
    fn check_attached_value(&mut self) -> Result<(), GameError>;
    fn send_prize_pool_to_winner(&mut self);
}

impl GameSessionActions for Game {
    fn init_properties(&mut self) {
        init_properties(&mut self.properties, &mut self.ownership);
    }
    fn make_reservation(
        &mut self,
        reservation_amount: u64,
        reservation_duration_in_block: u32,
    ) -> Result<(), GameError> {
        let reservation_id = make_reservation(reservation_amount, reservation_duration_in_block)?;
        self.reservations.push(reservation_id);

        Ok(())
    }

    fn register(
        &mut self,
        player_id: &ActorId,
        strategy_id: &ActorId,
        name: &str,
        reservation_amount: u64,
        reservation_duration_in_block: u32,
    ) -> Result<(), GameError> {
        self.check_status(GameStatus::Registration)?;
        self.player_already_registered(player_id, strategy_id)?;
        self.check_attached_value()?;

        let reservation_id = make_reservation(reservation_amount, reservation_duration_in_block)?;

        self.owners_to_strategy_ids.insert(*player_id, *strategy_id);
        self.players.insert(
            *strategy_id,
            PlayerInfo {
                owner_id: *player_id,
                name: name.to_string(),
                balance: INITIAL_BALANCE,
                reservation_id: Some(reservation_id),
                ..Default::default()
            },
        );
        self.players_queue.push(*strategy_id);

        Ok(())
    }
    fn cancel_game_session(&mut self) -> Result<(), GameError> {
        self.only_admin()?;

        if let Some(fee) = self.entry_fee {
            for owner_id in self.owners_to_strategy_ids.keys() {
                msg::send_with_gas(*owner_id, "", 0, fee).expect("Error in sending a message");
            }
        }

        Ok(())
    }

    fn delete_player(&mut self, player_id: &ActorId) -> Result<(), GameError> {
        let strategy_id = self
            .owners_to_strategy_ids
            .get(player_id)
            .ok_or(GameError::StrategyDoesNotExist)?;

        match self.game_status {
            GameStatus::WaitingForGasForGameContract | GameStatus::WaitingForGasForStrategy(_) => {
                self.exclude_player_from_game(*strategy_id);
                self.current_turn = self.current_turn.saturating_sub(1);
            }
            GameStatus::Registration => {
                self.players.remove(strategy_id);
                self.players_queue.retain(|&p| p != *strategy_id);
            }
            GameStatus::Finished => {
                self.owners_to_strategy_ids.remove(player_id);
                return Ok(());
            }
            _ => return Err(GameError::WrongGameStatus),
        }
        self.owners_to_strategy_ids.remove(player_id);
        self.check_amount_of_players();
        if let Some(fee) = self.entry_fee {
            msg::send_with_gas(*player_id, "", 0, fee).expect("Error in sending a message");
            self.prize_pool -= fee;
        }

        Ok(())
    }
    fn exit_game(&mut self) -> Result<(), GameError> {
        let owner_id = msg::source();
        let strategy_id = self
            .owners_to_strategy_ids
            .get(&owner_id)
            .ok_or(GameError::StrategyDoesNotExist)?;

        match self.game_status {
            GameStatus::WaitingForGasForGameContract | GameStatus::WaitingForGasForStrategy(_) => {
                self.exclude_player_from_game(*strategy_id);
                self.current_turn = self.current_turn.saturating_sub(1);
            }
            GameStatus::Registration => {
                self.players.remove(strategy_id);
                self.players_queue.retain(|&p| p != *strategy_id);
            }
            GameStatus::Finished => {
                self.owners_to_strategy_ids.remove(&owner_id);
                return Ok(());
            }
            _ => return Err(GameError::WrongGameStatus),
        }
        self
            .owners_to_strategy_ids
            .remove(&owner_id);
        self.check_amount_of_players();
        if let Some(fee) = self.entry_fee {
            msg::send_with_gas(owner_id, "", 0, fee).expect("Error in sending a message");
            self.prize_pool -= fee;
        }

        Ok(())
    }

    fn play(
        &mut self,
        min_gas_limit: u64,
        time_for_step: u32,
        awaiting_reply_msg_id_to_session_id: &mut HashMap<MessageId, AdminId>,
        gas_refill_timeout: u32,
    ) -> Result<GameReply, GameError> {
        let program_id = exec::program_id();
        let msg_source = msg::source();
        self.only_admin_or_program(&program_id, &msg_source)?;

        if exec::gas_available() < min_gas_limit {
            if let Some(id) = self.reservations.pop() {
                self.current_msg_id = msg_to_play_game(id, &program_id, &self.admin_id)?;
                return Ok(GameReply::NextRoundFromReservation);
            } else {
                self.current_msg_id = MessageId::zero();
                self.game_status = GameStatus::WaitingForGasForGameContract;
                return Err(GameError::AddGasToGameContract);
            }
        }

        if self.players_queue.len() == NUMBER_OF_PLAYERS as usize  && self.game_status == GameStatus::Registration {
            self.game_status = GameStatus::Play;
        }

        match self.game_status {
            GameStatus::Play | GameStatus::WaitingForGasForGameContract => {
                while self.game_status != GameStatus::Finished {
                    self.make_step(
                        time_for_step,
                        awaiting_reply_msg_id_to_session_id,
                        gas_refill_timeout,
                    )?;
                }

                Ok(GameReply::GameFinished {
                    admin_id: self.admin_id,
                    winner: self.winner,
                })
            }
            GameStatus::Wait | GameStatus::WaitingForGasForStrategy(_) => {
                // ` GameStatus::Wait` means that the player has missed his turn
                // or his strategy did not manage to make a move within the allotted time.
                // `GameStatus::WaitingForGasForStrategy(_)` means that player didn't manage to reserve a gas
                // for his strate within the alloted time
                // The player is removed from the game.
                self.exclude_player_from_game(self.current_player);

                // If the value of current_turn was 0 (meaning the player who missed their turn and was removed was the last in the array),
                // then this value remains the same.
                // If the value was 1, 2, or 3, then it is properly decreased by one.
                self.current_turn = self.current_turn.saturating_sub(1);
                self.check_amount_of_players();
                while self.game_status != GameStatus::Finished {
                    self.make_step(
                        time_for_step,
                        awaiting_reply_msg_id_to_session_id,
                        gas_refill_timeout,
                    )?;
                }

                Ok(GameReply::GameFinished {
                    admin_id: self.admin_id,
                    winner: self.winner,
                })
            }
            GameStatus::Finished => Ok(GameReply::GameFinished {
                admin_id: self.admin_id,
                winner: self.winner,
            }),
            _ => Err(GameError::WrongGameStatus),
        }
    }

    fn make_step(
        &mut self,
        time_for_step: u32,
        awaiting_reply_msg_id_to_session_id: &mut HashMap<MessageId, AdminId>,
        gas_refill_timeout: u32,
    ) -> Result<(), GameError> {
        let current_player: ActorId = self.players_queue[self.current_turn as usize];
        self.current_player = current_player;
        let mut player_info = self.get_player_info()?;
        let position = if player_info.in_jail {
            player_info.position
        } else {
            let (r1, r2) = get_rolls();
            let roll_sum = r1 + r2;
            (player_info.position + roll_sum) % NUMBER_OF_CELLS
        };

        // If a player is on a cell that belongs to another player
        // we write down a debt on him in the amount of the rent.
        // This is done in order to penalize the participant's contract
        // if he misses the rent
        let account = self.ownership[position as usize];
        if account != current_player && !account.is_zero() {
            if let Some((_, _, _, rent)) = self.properties[position as usize] {
                player_info.debt = rent;
            }
        }
        // If the new position is behind the previous one, it indicates that the player has completed a circuit around the board,
        // and his balance should be accordingly updated.
        if position <= player_info.position {
            player_info.balance += NEW_CIRCLE;
        }
        player_info.position = position;
        player_info.in_jail = position == JAIL_POSITION;
        player_info.round = self.round;
        self.players.insert(current_player, player_info.clone());

        self.current_turn = (self.current_turn + 1) % self.players_queue.len() as u8;
        match position {
            // free cells (it can be lottery or penalty): TODO as a task on hackathon
            0 | 2 | 4 | 7 | 16 | 20 | 30 | 33 | 36 | 38 => Ok(()),
            _ => {
                let game_info = self.get_game_info();

                // If the player's reservation is invalid,
                // we remove the player from the game.
                if let Some(id) = player_info.reservation_id {
                    match take_your_turn(id, &current_player, game_info) {
                        Ok(awaiting_reply_msg_id) => {
                            awaiting_reply_msg_id_to_session_id
                                .insert(awaiting_reply_msg_id, self.admin_id);
                            if self.current_msg_id == MessageId::zero() {
                                self.current_msg_id = msg::id();
                            }
                            self.game_status = GameStatus::Wait;
                            exec::wait_for(time_for_step);
                        }
                        Err(_) => {
                            self.exclude_player_from_game(current_player);
                            self.current_turn = self.current_turn.saturating_sub(1);
                            self.check_amount_of_players();
                            Ok(())
                        }
                    }
                } else {
                    self.game_status = GameStatus::WaitingForGasForStrategy(current_player);
                    exec::wait_for(gas_refill_timeout);
                }
            }
        }
    }
    
    fn check_amount_of_players(&mut self)  {
        if self.players_queue.len() == 0 {
            self.game_status = GameStatus::Finished;
        }
        if self.players_queue.len() == 1 {
            self.winner = self.players_queue[0];
                self.game_status = GameStatus::Finished;
                self.send_prize_pool_to_winner();
        }
    }

    fn finalize_turn_outcome(
        &mut self,
        gas_for_step: u64,
        min_gas_limit: u64,
        reservation_duration_in_block: u32,
    ) {
        
        match self.players_queue.len() {
            0 => {
                // All players have been removed from the game (either penalized or bankrupt)
                self.game_status = GameStatus::Finished;
            }
            1 => {
                self.winner = self.players_queue[0];
                self.game_status = GameStatus::Finished;
                self.send_prize_pool_to_winner();
            }
            _ => {
                let gas_available = exec::gas_available();

                let reservation_id = if gas_available.saturating_sub(gas_for_step) > min_gas_limit {
                    match ReservationId::reserve(
                        gas_available - min_gas_limit,
                        reservation_duration_in_block,
                    ) {
                        Ok(id) => Some(id),
                        Err(_) => None,
                    }
                } else {
                    None
                };
                self.players
                    .entry(self.current_player)
                    .and_modify(|info| info.reservation_id = reservation_id);
                self.game_status = GameStatus::Play;
            }
        }
        self.current_step += 1;
        if self.current_step % self.players_queue.len() as u64 == 0 {
            self.round += 1;
            // check penalty and debt of the players for the previous round
            // if penalty is equal to 5 points we remove the player from the game
            // if a player has a debt and he has not enough balance to pay it
            // he is also removed from the game
            bankrupt_and_penalty(
                &self.admin_id,
                &mut self.players,
                &mut self.players_queue,
                &self.properties,
                &mut self.properties_in_bank,
                &mut self.ownership,
                &mut self.current_turn,
            );
        }

        // message for front end
        msg::send_with_gas(
            self.admin_id,
            GameReply::Step {
                players: self
                    .players
                    .iter()
                    .map(|(key, value)| (*key, value.clone()))
                    .collect(),
                properties: self.properties.clone(),
                current_player: self.current_player,
                current_step: self.current_step,
                ownership: self.ownership.clone(),
            },
            0,
            0,
        )
        .expect("Error in sending a message `GameEvent::Step`");
        
        exec::wake(self.current_msg_id).expect("Unable to wake the msg");
    }

    fn add_gas_to_player_strategy(
        &mut self,
        reservation_amount: u64,
        reservation_duration_in_block: u32,
    ) -> Result<(), GameError> {
        let strategy_id =
            if let GameStatus::WaitingForGasForStrategy(strategy_id) = self.game_status {
                strategy_id
            } else {
                return Err(GameError::WrongGameStatus);
            };
        let reservation_id = make_reservation(reservation_amount, reservation_duration_in_block)?;
        self.players
            .entry(strategy_id)
            .and_modify(|player_info| player_info.reservation_id = Some(reservation_id));
        self.game_status = GameStatus::Play;
        Ok(())
    }

    fn send_prize_pool_to_winner(&mut self) {
        if self.prize_pool > 0 {
            msg::send_with_gas(self.winner, "", 0, self.prize_pool)
                .expect("Error in sending prize pool to winner");
        }
    }

    fn check_status(&self, game_status: GameStatus) -> Result<(), GameError> {
        if self.game_status != game_status {
            return Err(GameError::WrongGameStatus);
        }
        Ok(())
    }

    fn only_admin(&self) -> Result<(), GameError> {
        if msg::source() != self.admin_id {
            return Err(GameError::OnlyAdmin);
        }
        Ok(())
    }

    fn only_admin_or_program(
        &self,
        program_id: &ActorId,
        msg_source: &ActorId,
    ) -> Result<(), GameError> {
        if *msg_source != self.admin_id && *msg_source != *program_id {
            return Err(GameError::MsgSourceMustBeAdminOrProgram);
        }
        Ok(())
    }

    fn get_game_info(&self) -> GameInfo {
        GameInfo {
            properties_in_bank: self.properties_in_bank.clone().into_iter().collect(),
            players: self.players.clone().into_iter().collect(),
            players_queue: self.players_queue.clone(),
            properties: self.properties.clone(),
            ownership: self.ownership.clone(),
        }
    }

    fn exclude_player_from_game(&mut self, player: ActorId) {
        self.players_queue.retain(|&p| p != player);
        self.players.entry(player).and_modify(|info| {
            info.lost = true;
            info.balance = 0;

            for cell in info.cells.iter() {
                self.ownership[*cell as usize] = self.admin_id;
                self.properties_in_bank.insert(*cell);
            }
        });
    }

    fn get_player_info(&self) -> Result<PlayerInfo, GameError> {
        if let Some(player_info) = self.players.get(&self.current_player) {
            Ok(player_info.clone())
        } else {
            Err(GameError::PlayerDoesNotExist)
        }
    }

    fn player_already_registered(
        &self,
        owner_id: &ActorId,
        strategy_id: &ActorId,
    ) -> Result<(), GameError> {
        if self.players.contains_key(strategy_id) {
            return Err(GameError::StrategyAlreadyReistered);
        }
        if self.owners_to_strategy_ids.contains_key(owner_id) {
            return Err(GameError::AccountAlreadyRegistered);
        }
        Ok(())
    }

    fn check_attached_value(&mut self) -> Result<(), GameError> {
        if let Some(fee) = self.entry_fee {
            if msg::value() != fee {
                return Err(GameError::WrongValueAmount);
            }
            self.prize_pool += fee;
        }
        Ok(())
    }
}

fn make_reservation(
    reservation_amount: u64,
    reservation_duration_in_block: u32,
) -> Result<ReservationId, GameError> {
    ReservationId::reserve(reservation_amount, reservation_duration_in_block)
        .map_err(|_| GameError::ReservationError)
}
