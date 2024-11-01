use gstd::{exec, msg, ReservationId};
use sails_rs::{collections::HashMap, prelude::*};

pub type PairId = u16;
static mut SEED: u8 = 0;

pub fn get_random_value(range: u8) -> u8 {
    if range == 0 {
        return 0;
    }
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}
pub fn get_random_dodge(chance: u8) -> bool {
    assert!(chance <= 100, "The chance must be between 0 and 100");
    let random_value = get_random_value(101);
    random_value < chance
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum BattleError {
    ProgramInitializationFailedWithContext(String),
    AlreadyHaveBattle,
    NotOwnerOfWarrior,
    SendingMessageToWarrior,
    GetWarriorOwner,
    SeveralRegistrations,
    NoSuchGame,
    NoSuchPair,
    WrongState,
    WrongBid,
    NoSuchPlayer,
    AccessDenied,
    BattleFull,
    NotEnoughPlayers,
    NotAdmin,
    PlayerDoesNotExist,
    PairDoesNotExist,
    GameIsOver,
    NotWarriorOwner,
    NotPlayerGame,
    NoGamesForPlayer,
    TimeExpired,
    MoveHasAlreadyBeenMade,
    NoSuchReservation,
    WrongTimeCreation,
    UltimateReload,
    ReflectReload,
    MisallocationOfPoints,
    IdAndAppearanceIsNone,
}

#[derive(Debug, Default, Clone)]
pub struct Battle {
    pub admin: ActorId,
    pub battle_name: String,
    pub time_creation: u64,
    pub bid: u128,
    pub participants: HashMap<ActorId, Player>,
    pub defeated_participants: HashMap<ActorId, Player>,
    pub state: State,
    pub pairs: HashMap<PairId, Pair>,
    pub players_to_pairs: HashMap<ActorId, PairId>,
    pub reservation: HashMap<ActorId, ReservationId>,
    pub waiting_player: Option<(ActorId, PairId)>,
    pub pair_id: u16,
}

impl Battle {
    pub fn check_end_game(&mut self) {
        if self.participants.len() == 1 {
            if let Some((&winner, _)) = self.participants.iter().next() {
                if self.bid != 0 {
                    msg::send_with_gas(
                        winner,
                        "",
                        10_000,
                        self.bid * (self.defeated_participants.len() + 1) as u128,
                    )
                    .expect("Error send value");
                    // TODO: uncomment and switch https://github.com/gear-tech/gear/pull/4270
                    // msg::send_with_gas(winner, "", 0, self.bid * (self.defeated_participants.len() + 1) as u128).expect("Error send value");
                }
                self.state = State::GameIsOver {
                    winners: (winner, None),
                };
            }
        }
    }

    pub fn check_draw_end_game(&mut self) {
        if self.participants.len() == 2 {
            let mut winners: Vec<ActorId> = Vec::with_capacity(2);
            let prize = self.bid * (self.defeated_participants.len() + 2) as u128 / 2;
            for id in self.participants.keys() {
                if self.bid != 0 {
                    msg::send_with_gas(*id, "", 10_000, prize).expect("Error send value");
                    // TODO: uncomment and switch https://github.com/gear-tech/gear/pull/4270
                    // msg::send_with_gas(
                    //     *id,
                    //     "",
                    //     10_000,
                    //     prize,
                    // )
                    // .expect("Error send value");
                }
                winners.push(*id);
            }
            self.state = State::GameIsOver {
                winners: (winners[0], Some(winners[1])),
            };
        }
    }

    pub fn check_min_player_amount(&self) -> Result<(), BattleError> {
        if self.participants.len() <= 1 {
            return Err(BattleError::NotEnoughPlayers);
        }
        Ok(())
    }

    pub fn split_into_pairs(&mut self) -> Result<(), BattleError> {
        let round_start_time = exec::block_timestamp();
        self.create_pairs(round_start_time);
        Ok(())
    }

    pub fn create_pairs(&mut self, round_start_time: u64) {
        self.pairs = HashMap::new();
        self.players_to_pairs = HashMap::new();
        let mut participants_vec: Vec<(ActorId, Player)> =
            self.participants.clone().into_iter().collect();

        while participants_vec.len() > 1 {
            let range = participants_vec.len() as u8;
            let idx1 = get_random_value(range);
            let player1 = participants_vec.swap_remove(idx1 as usize).1;
            let idx2 = get_random_value(range - 1);
            let player2 = participants_vec.swap_remove(idx2 as usize).1;
            let pair = Pair {
                player_1: player1.owner,
                player_2: player2.owner,
                round_start_time,
                round: 1,
                action: None,
            };
            self.pairs.insert(self.pair_id, pair);
            self.players_to_pairs.insert(player1.owner, self.pair_id);
            self.players_to_pairs.insert(player2.owner, self.pair_id);
            self.pair_id += 1;
        }
        // If there are an odd number of participants left, one goes into standby mode
        if participants_vec.len() == 1 {
            let player = participants_vec.remove(0).1;
            let pair = Pair {
                player_1: player.owner,
                round: 1,
                ..Default::default()
            };
            self.pairs.insert(self.pair_id, pair);
            self.players_to_pairs.insert(player.owner, self.pair_id);
            self.waiting_player = Some((player.owner, self.pair_id));
            self.pair_id += 1;
        }
    }
    pub fn send_delayed_message_make_move_from_reservation(&mut self, time_for_move: u32) {
        let mut new_map_reservation = HashMap::new();
        self.reservation
            .iter()
            .for_each(|(actor_id, reservation_id)| {
                if let Some(waiting_player) = self.waiting_player {
                    if waiting_player.0 == *actor_id {
                        new_map_reservation.insert(waiting_player.0, *reservation_id);
                        return;
                    }
                }
                let number_of_victories = self
                    .participants
                    .get(actor_id)
                    .expect("The player must exist")
                    .number_of_victories;
                let round: u8 = 1;
                let request = [
                    "Battle".encode(),
                    "AutomaticMove".to_string().encode(),
                    (*actor_id, number_of_victories, round).encode(),
                ]
                .concat();

                msg::send_bytes_delayed_from_reservation(
                    *reservation_id,
                    exec::program_id(),
                    request,
                    0,
                    time_for_move,
                )
                .expect("Error in sending message");
            });
        self.reservation = new_map_reservation;
    }

    pub fn check_state(&self, state: State) -> Result<(), BattleError> {
        if self.state != state {
            return Err(BattleError::WrongState);
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Player {
    pub warrior_id: Option<ActorId>,
    pub owner: ActorId,
    pub user_name: String,
    pub player_settings: PlayerSettings,
    pub appearance: Appearance,
    pub number_of_victories: u8,
    pub ultimate_reload: u8,
    pub reflect_reload: u8,
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum State {
    #[default]
    Registration,
    Started,
    GameIsOver {
        winners: (ActorId, Option<ActorId>),
    },
}

#[derive(Default, Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Pair {
    pub player_1: ActorId,
    pub player_2: ActorId,
    pub action: Option<(ActorId, Move)>,
    pub round: u8,
    pub round_start_time: u64,
}

impl Pair {
    pub fn recap_round(
        &mut self,
        player1_info: (&mut Player, Move),
        player2_info: (&mut Player, Move),
    ) -> Option<BattleResult> {
        let dodge_1 = get_random_dodge(player1_info.0.player_settings.dodge as u8);
        let dodge_2 = get_random_dodge(player2_info.0.player_settings.dodge as u8);

        // Damage that players will receive (default 0)
        let mut damage_1 = 0;
        let mut damage_2 = 0;

        // Process the actions of both players
        match (player1_info.1, player2_info.1) {
            (Move::Attack, Move::Attack) => {
                if !dodge_2 {
                    damage_2 = player1_info.0.player_settings.attack; // Player 2 takes damage from Player 1
                }
                if !dodge_1 {
                    damage_1 = player2_info.0.player_settings.attack; // Player 1 takes damage from Player 2
                }
            }
            (Move::Attack, Move::Reflect) => {
                if !dodge_2 {
                    // Player 2 reflects the damage, reducing it by the value of his defence
                    damage_2 = player1_info
                        .0
                        .player_settings
                        .attack
                        .saturating_mul(100 - player2_info.0.player_settings.defence)
                        .saturating_div(100);
                }
                if !dodge_1 {
                    // Player 1 takes reflected damage
                    damage_1 = player1_info
                        .0
                        .player_settings
                        .attack
                        .saturating_sub(damage_2);
                }
                player2_info.0.reflect_reload = 2;
            }
            (Move::Reflect, Move::Attack) => {
                if !dodge_1 {
                    // Player 1 reflects the damage, reducing it by the value of their defence
                    damage_1 = player2_info
                        .0
                        .player_settings
                        .attack
                        .saturating_mul(100 - player1_info.0.player_settings.defence)
                        .saturating_div(100);
                }
                if !dodge_2 {
                    // Player 2 takes reflected damage
                    damage_2 = player2_info
                        .0
                        .player_settings
                        .attack
                        .saturating_sub(damage_1);
                }
                player1_info.0.reflect_reload = 2;
            }
            (Move::Reflect, Move::Reflect) => {
                // Both players deflect each other's attacks, no damage done
                damage_1 = 0;
                damage_2 = 0;
                player1_info.0.reflect_reload = 2;
                player2_info.0.reflect_reload = 2;
            }
            (Move::Ultimate, Move::Attack) => {
                if !dodge_1 {
                    // Player 1 receives a normal attack
                    damage_1 = player2_info.0.player_settings.attack;
                }
                if !dodge_2 {
                    // Player 2 takes double the damage from Ultimate
                    damage_2 = player1_info.0.player_settings.attack * 2;
                }
                player1_info.0.ultimate_reload = 2;
            }
            (Move::Attack, Move::Ultimate) => {
                if !dodge_1 {
                    // Player 1 takes double the damage from Ultimate
                    damage_1 = player2_info.0.player_settings.attack * 2;
                }
                if !dodge_2 {
                    // Player 2 receives a normal attack
                    damage_2 = player1_info.0.player_settings.attack;
                }
                player2_info.0.ultimate_reload = 2;
            }
            (Move::Ultimate, Move::Ultimate) => {
                if !dodge_1 {
                    // Player 1 takes double the damage from Ultimate
                    damage_1 = player2_info.0.player_settings.attack * 2;
                }
                if !dodge_2 {
                    // Player 2 takes double the damage from Ultimate
                    damage_2 = player1_info.0.player_settings.attack * 2;
                }
                player1_info.0.ultimate_reload = 2;
                player2_info.0.ultimate_reload = 2;
            }
            (Move::Reflect, Move::Ultimate) => {
                if !dodge_1 {
                    // Player 1 takes double damage from Ultimate, but can partially deflect it
                    damage_1 = (player2_info.0.player_settings.attack * 2)
                        .saturating_mul(100 - player1_info.0.player_settings.defence)
                        .saturating_div(100);
                }
                if !dodge_2 {
                    // Player 2 takes reflected damage
                    damage_2 = (player2_info.0.player_settings.attack * 2).saturating_sub(damage_1);
                }
                player1_info.0.reflect_reload = 2;
                player2_info.0.ultimate_reload = 2;
            }
            (Move::Ultimate, Move::Reflect) => {
                if !dodge_2 {
                    // Player 2 takes double damage from Ultimate, but can partially deflect it
                    damage_2 = (player1_info.0.player_settings.attack * 2)
                        .saturating_mul(100 - player2_info.0.player_settings.defence)
                        .saturating_div(100);
                }
                if !dodge_1 {
                    // Player 1 takes reflected damage
                    damage_1 = (player1_info.0.player_settings.attack * 2).saturating_sub(damage_2);
                }
                player1_info.0.ultimate_reload = 2;
                player2_info.0.reflect_reload = 2;
            }
        }

        match player2_info.1 {
            Move::Attack => {
                player2_info.0.reflect_reload = player2_info.0.reflect_reload.saturating_sub(1);
                player2_info.0.ultimate_reload = player2_info.0.ultimate_reload.saturating_sub(1);
            }
            Move::Reflect => {
                player2_info.0.ultimate_reload = player2_info.0.ultimate_reload.saturating_sub(1);
            }
            Move::Ultimate => {
                player2_info.0.reflect_reload = player2_info.0.reflect_reload.saturating_sub(1);
            }
        }

        match player1_info.1 {
            Move::Attack => {
                player1_info.0.reflect_reload = player1_info.0.reflect_reload.saturating_sub(1);
                player1_info.0.ultimate_reload = player1_info.0.ultimate_reload.saturating_sub(1);
            }
            Move::Reflect => {
                player1_info.0.ultimate_reload = player1_info.0.ultimate_reload.saturating_sub(1);
            }
            Move::Ultimate => {
                player1_info.0.reflect_reload = player1_info.0.reflect_reload.saturating_sub(1);
            }
        }
        // Damage application
        player1_info.0.player_settings.health = player1_info
            .0
            .player_settings
            .health
            .saturating_sub(damage_1);
        player2_info.0.player_settings.health = player2_info
            .0
            .player_settings
            .health
            .saturating_sub(damage_2);

        // Checking to see who won
        if player1_info.0.player_settings.health == 0 && player2_info.0.player_settings.health == 0
        {
            return Some(BattleResult::Draw(
                player1_info.0.owner,
                player2_info.0.owner,
            )); // Both players lost, a draw
        } else if player1_info.0.player_settings.health == 0 {
            return Some(BattleResult::PlayerWin(player2_info.0.owner)); // Player 2 wins
        } else if player2_info.0.player_settings.health == 0 {
            return Some(BattleResult::PlayerWin(player1_info.0.owner)); // Player 1 wins
        }
        None // Both players are still alive, no one has won
    }

    pub fn get_opponent(&self, player: &ActorId) -> ActorId {
        if self.player_1 != *player {
            self.player_1
        } else {
            self.player_2
        }
    }
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone, Copy)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Move {
    Attack,
    Reflect,
    Ultimate,
}

#[derive(Default, Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PlayerSettings {
    pub health: u16,
    pub attack: u16,
    pub defence: u16,
    pub dodge: u16,
}

#[derive(Default, Encode, Decode, TypeInfo, PartialEq, Eq, Debug, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Config {
    pub health: u16,
    pub max_participants: u8,
    pub attack_range: (u16, u16),
    pub defence_range: (u16, u16),
    pub dodge_range: (u16, u16),
    pub available_points: u16,
    pub time_for_move_in_blocks: u32,
    pub block_duration_ms: u32,
    pub gas_for_create_warrior: u64,
    pub gas_to_cancel_the_battle: u64,
    pub time_to_cancel_the_battle: u32,
    pub reservation_amount: u64,
    pub reservation_time: u32,
}

#[derive(Debug, Default, Clone, TypeInfo, Encode, Decode)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct BattleState {
    pub admin: ActorId,
    pub battle_name: String,
    pub time_creation: u64,
    pub bid: u128,
    pub participants: Vec<(ActorId, Player)>,
    pub defeated_participants: Vec<(ActorId, Player)>,
    pub state: State,
    pub pairs: Vec<(PairId, Pair)>,
    pub players_to_pairs: Vec<(ActorId, PairId)>,
    pub waiting_player: Option<(ActorId, PairId)>,
    pub pair_id: u16,
    pub reservation: Vec<(ActorId, ReservationId)>,
}

impl From<Battle> for BattleState {
    fn from(value: Battle) -> Self {
        let Battle {
            admin,
            battle_name,
            time_creation,
            bid,
            participants,
            defeated_participants,
            state,
            pairs,
            players_to_pairs,
            waiting_player,
            pair_id,
            reservation,
        } = value;

        let participants = participants.into_iter().collect();
        let defeated_participants = defeated_participants.into_iter().collect();
        let pairs = pairs.into_iter().collect();
        let players_to_pairs = players_to_pairs.into_iter().collect();
        let reservation = reservation.into_iter().collect();

        Self {
            admin,
            battle_name,
            time_creation,
            bid,
            participants,
            defeated_participants,
            state,
            pairs,
            players_to_pairs,
            reservation,
            pair_id,
            waiting_player,
        }
    }
}

pub enum BattleResult {
    PlayerWin(ActorId),
    Draw(ActorId, ActorId),
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Appearance {
    head_index: u16,
    hat_index: u16,
    body_index: u16,
    accessory_index: u16,
    body_color: String,
    back_color: String,
}
