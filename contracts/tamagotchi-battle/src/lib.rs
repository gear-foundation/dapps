#![no_std]

use gstd::{
    collections::{BTreeMap, BTreeSet},
    debug, exec, msg,
    prelude::*,
    ActorId, MessageId, ReservationId,
};
use tamagotchi_battle_io::*;
use tamagotchi_io::{TmgAction, TmgReply};
mod pair;
use pair::*;
mod utils;
use utils::{generate_penalty_damage, generate_power, get_random_value};
mod player;
use player::*;

const MAX_POWER: u16 = 10_000;
const MAX_RANGE: u16 = 7_000;
const MIN_RANGE: u16 = 3_000;
const HEALTH: u16 = 2_500;
const MAX_PARTICIPANTS: u8 = 50;
const MAX_STEPS_IN_ROUND: u8 = 5;
const COLORS: [&str; 6] = ["Green", "Red", "Blue", "Purple", "Orange", "Yellow"];
const TIME_FOR_MOVE: u32 = 20;
const GAS_AMOUNT: u64 = 10_000_000_000;
const RESERVATION_AMOUNT: u64 = 200_000_000_000;
const RESERVATION_DURATION: u32 = 86_400;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
struct Battle {
    admins: Vec<ActorId>,
    players: BTreeMap<ActorId, Player>,
    players_ids: Vec<ActorId>,
    current_players: Vec<ActorId>,
    state: BattleState,
    current_winner: ActorId,
    pairs: BTreeMap<PairId, Pair>,
    players_to_pairs: BTreeMap<ActorId, BTreeSet<PairId>>,
    completed_games: u8,
    reservations: BTreeMap<ActorId, ReservationId>,
    config: Config,
}

static mut BATTLE: Option<Battle> = None;

impl Battle {
    fn start_registration(&mut self) -> Result<BattleReply, BattleError> {
        assert_eq!(
            self.state,
            BattleState::GameIsOver,
            "The previous game must be over"
        );
        self.state = BattleState::Registration;
        self.current_winner = ActorId::zero();
        self.players_ids = Vec::new();
        self.completed_games = 0;
        self.players_to_pairs = BTreeMap::new();
        self.current_players = Vec::new();
        self.pairs = BTreeMap::new();
        Ok(BattleReply::RegistrationStarted)
    }

    async fn register(&mut self, tmg_id: &TamagotchiId) -> Result<BattleReply, BattleError> {
        match self.state {
            BattleState::Registration => {}
            _ => return Err(BattleError::WrongState),
        }

        assert!(
            self.players_ids.len() < MAX_PARTICIPANTS as usize,
            "Maximum number of players was reached"
        );

        if self.players_ids.contains(tmg_id) {
            panic!("This tamagotchi is already in game!");
        }
        let (owner, name, date_of_birth) = get_tmg_info(tmg_id).await;

        if owner != msg::source() {
            panic!("It is not your Tamagotchi");
        }

        if !self.players.contains_key(tmg_id) {
            let power = generate_power(self.config.min_range, self.config.max_range, *tmg_id);
            let defence = MAX_POWER - power;
            let color_index = get_random_value(COLORS.len() as u8);
            let player = Player {
                owner,
                name,
                date_of_birth,
                tmg_id: *tmg_id,
                defence,
                power,
                health: HEALTH,
                color: COLORS[color_index as usize].to_string(),
                victories: 0,
            };
            self.players.insert(*tmg_id, player);
        } else {
            self.players
                .entry(*tmg_id)
                .and_modify(|player| player.health = HEALTH);
        }

        self.players_ids.push(*tmg_id);
        self.current_players.push(*tmg_id);

        let reservation_id = ReservationId::reserve(RESERVATION_AMOUNT, RESERVATION_DURATION)
            .expect("reservation across executions");
        self.reservations.insert(*tmg_id, reservation_id);

        Ok(BattleReply::Registered { tmg_id: *tmg_id })
    }

    /// Starts the battle.
    /// This message must be sent after the registration end (the contract is in the `BattleState::Registration` state)
    /// It must also be sent when the game is on but a round is ended (the contract is in the `BattleState::WaitNextRound` state)
    /// BattleState::WaitNextRound` state means the the battles in pairs are over and winners are expecting to play in the next round
    fn start_battle(&mut self) -> Result<BattleReply, BattleError> {
        debug!("Before starting the battle");
        match self.state {
            BattleState::Registration | BattleState::WaitNextRound => {
                debug!("Before registration");
                if self.players_ids.len() <= 1 {
                    return Err(BattleError::NotEnoughPlayers);
                }
                self.check_admin(&msg::source())?;

                // Clear the state if the state is `BattleState::WaitNextRound`
                self.pairs = BTreeMap::new();
                self.players_to_pairs = BTreeMap::new();
                self.completed_games = 0;

                self.split_into_pairs();

                self.state = BattleState::GameIsOn;

                // After the battle starts, the contract waits for a specific period of time (`time_for_move` from the config),
                // usually equivalent to one minute, to check whether all participants have made their move.
                debug!(" {:?}", self.config.time_for_move);
                exec::wait_for(self.config.time_for_move + 1);
            }
            BattleState::GameIsOn => {
                debug!("Checking moves");
                let mut number_of_missed_turns = 0;
                let mut pair_ids_to_remove = Vec::new();
                let timestamp = exec::block_timestamp();
                let time_for_move_ms =
                    self.config.block_duration_ms * u64::from(self.config.time_for_move);

                for (pair_id, pair) in self.pairs.iter_mut() {
                    // If the last update of the structure was more than the time_for_move ago,
                    //the contract sets the player's move to None and allows the second player to make their move.
                    debug!("time_for_move {:?}", time_for_move_ms);
                    debug!("delta {:?}", timestamp.saturating_sub(pair.last_updated));
                    if timestamp.saturating_sub(pair.last_updated) >= time_for_move_ms {
                        if pair.moves.is_empty() {
                            debug!("First player missed turn");
                            pair.moves.push(None);
                            pair.last_updated = timestamp;
                            number_of_missed_turns += 1;
                        } else {
                            // If the contract observes that both players have missed their turn,
                            // it removes that pair from the game.
                            debug!("Second player missed turn");
                            pair_ids_to_remove.push((
                                *pair_id,
                                pair.owner_ids[0],
                                pair.owner_ids[1],
                            ));
                        }
                    }
                }

                for (id, owner_0, owner_1) in pair_ids_to_remove.into_iter() {
                    self.pairs.remove(&id);
                    self.remove_pair_id_from_player(owner_0, id);
                    self.remove_pair_id_from_player(owner_1, id);
                }

                if number_of_missed_turns > 0 {
                    exec::wait_for(self.config.time_for_move + 1);
                }
            }
            _ => return Err(BattleError::WrongState),
        }
        Ok(BattleReply::BattleStarted)
    }

    fn split_into_pairs(&mut self) {
        let mut players_len = self.players_ids.len();

        let last_updated = exec::block_timestamp();

        for pair_id in 0..self.players_ids.len() {
            let first_tmg_id = get_random_value(players_len as u8);
            let first_tmg = self.players_ids.remove(first_tmg_id as usize);

            let first_owner = if let Some(player) = self.players.get_mut(&first_tmg) {
                player.health = HEALTH;
                player.owner
            } else {
                panic!("Can't be None: Tmg does not exsit");
            };

            players_len -= 1;

            let second_tmg_id = get_random_value(players_len as u8);
            let second_tmg = self.players_ids.remove(second_tmg_id as usize);
            let second_owner = if let Some(player) = self.players.get_mut(&second_tmg) {
                player.health = HEALTH;
                player.owner
            } else {
                panic!("Can't be None: Tmg does not exsit");
            };

            players_len -= 1;

            self.players_to_pairs
                .entry(first_owner)
                .and_modify(|pair_ids| {
                    pair_ids.insert(pair_id as u8);
                })
                .or_insert_with(|| BTreeSet::from([pair_id as u8]));
            self.players_to_pairs
                .entry(second_owner)
                .and_modify(|pair_ids| {
                    pair_ids.insert(pair_id as u8);
                })
                .or_insert_with(|| BTreeSet::from([pair_id as u8]));

            let pair = Pair {
                owner_ids: vec![first_owner, second_owner],
                tmg_ids: vec![first_tmg, second_tmg],
                moves: Vec::new(),
                rounds: 0,
                game_is_over: false,
                winner: ActorId::zero(),
                last_updated,
                msg_ids_in_waitlist: BTreeSet::new(),
            };
            self.pairs.insert(pair_id as u8, pair);

            if players_len == 1 || players_len == 0 {
                break;
            }
        }
    }

    fn make_move(&mut self, pair_id: PairId, tmg_move: Move) -> Result<BattleReply, BattleError> {
        self.check_state(BattleState::GameIsOn)?;
        let pairs_len = self.pairs.len() as u8;
        let pair = get_mut_pair(&mut self.pairs, pair_id)?;

        if pair.game_is_over {
            return Err(BattleError::GameIsOver);
        }

        let current_msg_id = msg::id();
        let timestamp = exec::block_timestamp();

        // Check whether the message is being executed for the first time or was in the waitlist.
        // This is necessary to verify whether a player has missed their turn.
        if pair.msg_ids_in_waitlist.remove(&current_msg_id) {
            let time_for_move_ms =
                self.config.block_duration_ms * u64::from(self.config.time_for_move);

            if timestamp.saturating_sub(pair.last_updated) >= time_for_move_ms {
                // the move was skipped
                pair.moves.push(None)
            } else {
                return Ok(BattleReply::MoveMade);
            }
        } else {
            // Player's new move.
            // All necessary checks must be performed.
            let current_turn = pair.moves.len();
            let tmg_owner = pair.owner_ids[current_turn];
            let msg_source = msg::source();
            check_tmg_owner(tmg_owner, msg_source)?;
            is_pair_id_in_player_pair_ids(&self.players_to_pairs, &msg_source, pair_id)?;
            pair.moves.push(Some(tmg_move));
        }

        if pair.moves.len() == 2 {
            let (tmg_id_0, tmg_id_1) = (pair.tmg_ids[0], pair.tmg_ids[1]);
            let mut player_0 = get_player(&self.players, &tmg_id_0)?;
            let mut player_1 = get_player(&self.players, &tmg_id_1)?;
            let (health_loss_0, health_loss_1) = pair.process_round_outcome(
                &mut player_0,
                &mut player_1,
                pairs_len,
                self.completed_games,
                self.config.max_steps_in_round,
                self.config.min_range,
                self.config.max_power,
            );

            self.players.insert(tmg_id_0, player_0);
            self.players.insert(tmg_id_1, player_1);

            check_all_games_completion(
                self.completed_games,
                pairs_len,
                &mut self.state,
                &mut self.current_winner,
                &self.players_ids,
            );

            send_round_result(
                &self.admins[0],
                pair_id,
                &[health_loss_0, health_loss_1],
                &pair.moves,
            );
        }

        if !pair.game_is_over {
            // After the move was made, the contract waits for a specific period of time (`time_for_move` from the config),
            // usually equivalent to one minute, to check whether the next player has made his move.
            pair.msg_ids_in_waitlist.insert(current_msg_id);
            exec::wait_for(self.config.time_for_move);
        }

        Ok(BattleReply::MoveMade)
    }

    fn add_admin(&mut self, new_admin: &ActorId) -> Result<BattleReply, BattleError> {
        if !self.admins.contains(&msg::source()) {
            panic!("Only admin can add another admin");
        }
        self.admins.push(*new_admin);
        Ok(BattleReply::AdminAdded)
    }

    fn check_admin(&self, account: &ActorId) -> Result<(), BattleError> {
        if !self.admins.contains(&msg::source()) {
            return Err(BattleError::NotAdmin);
        }
        Ok(())
    }

    fn check_state(&self, state: BattleState) -> Result<(), BattleError> {
        if self.state != state {
            return Err(BattleError::WrongState);
        }
        Ok(())
    }

    fn remove_pair_id_from_player(&mut self, player: ActorId, pair_id: PairId) {
        self.players_to_pairs.entry(player).and_modify(|pair_ids| {
            pair_ids.remove(&pair_id);
        });
    }
}

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load().expect("Unable to decode `BattleAction`");
    let battle = unsafe { BATTLE.get_or_insert(Default::default()) };
    let reply = match action {
        BattleAction::StartRegistration => battle.start_registration(),
        BattleAction::Register { tmg_id } => battle.register(&tmg_id).await,
        BattleAction::MakeMove { pair_id, tmg_move } => battle.make_move(pair_id, tmg_move),
        BattleAction::StartBattle => battle.start_battle(),
        BattleAction::AddAdmin(new_admin) => battle.add_admin(&new_admin),
    };
    msg::reply(reply, 0).expect("Error in sending a reply");
}

#[no_mangle]
unsafe extern fn init() {
    let config: Config = msg::load().expect("Unable to decode the init msg");
    let battle = Battle {
        admins: vec![msg::source()],
        config,
        ..Default::default()
    };
    BATTLE = Some(battle);
}

pub async fn get_tmg_info(tmg_id: &ActorId) -> (ActorId, String, u64) {
    let reply: TmgReply = msg::send_for_reply_as(*tmg_id, TmgAction::TmgInfo, 0, 0)
        .expect("Error in sending a message `TmgAction::TmgInfo")
        .await
        .expect("Unable to decode TmgReply");
    if let TmgReply::TmgInfo {
        owner,
        name,
        date_of_birth,
    } = reply
    {
        (owner, name, date_of_birth)
    } else {
        panic!("Wrong received message");
    }
}

#[no_mangle]
extern fn state() {
    let query: BattleQuery = msg::load().expect("Unable to load the query");
    let battle = unsafe { BATTLE.take().expect("Unexpected error in taking state") };
    match query {
        BattleQuery::GetPlayer { tmg_id } => {
            let player = battle.players.get(&tmg_id).cloned();
            msg::reply(BattleQueryReply::Player { player }, 0).expect("Failed to share state");
        }
        BattleQuery::PlayersIds => {
            msg::reply(
                BattleQueryReply::PlayersIds {
                    players_ids: battle.players_ids,
                },
                0,
            )
            .expect("Failed to share state");
        }
    }
}

fn send_round_result(admin: &ActorId, pair_id: PairId, losses: &[u16], moves: &[Option<Move>]) {
    msg::send(
        *admin,
        BattleReply::RoundResult((
            pair_id,
            losses[0],
            losses[1],
            moves[0].clone(),
            moves[1].clone(),
        )),
        0,
    )
    .expect("Error in sending a message `TmgEvent::RoundResult`");
}

fn get_mut_pair(
    pairs: &mut BTreeMap<PairId, Pair>,
    pair_id: PairId,
) -> Result<&mut Pair, BattleError> {
    if let Some(pair) = pairs.get_mut(&pair_id) {
        Ok(pair)
    } else {
        Err(BattleError::PairDoesNotExist)
    }
}

fn get_player(
    players: &BTreeMap<ActorId, Player>,
    tmg_id: &ActorId,
) -> Result<Player, BattleError> {
    if let Some(player) = players.get(tmg_id) {
        Ok(player.clone())
    } else {
        Err(BattleError::PlayerDoesNotExist)
    }
}

fn check_all_games_completion(
    completed_games: u8,
    pairs_len: u8,
    state: &mut BattleState,
    current_winner: &mut ActorId,
    players_ids: &Vec<ActorId>,
) {
    if completed_games == pairs_len {
        if players_ids.len() == 1 {
            *state = BattleState::GameIsOver;
            *current_winner = players_ids[0];
        } else {
            *state = BattleState::WaitNextRound;
        }
    }
}

fn check_tmg_owner(tmg_owner: ActorId, account: ActorId) -> Result<(), BattleError> {
    if tmg_owner != account {
        return Err(BattleError::NotTmgOwner);
    }
    Ok(())
}

fn is_pair_id_in_player_pair_ids(
    players_to_pairs: &BTreeMap<ActorId, BTreeSet<u8>>,
    player: &ActorId,
    pair_id: u8,
) -> Result<(), BattleError> {
    if let Some(pair_ids) = players_to_pairs.get(player) {
        if !pair_ids.contains(&pair_id) {
            return Err(BattleError::NotPlayerGame);
        }
    } else {
        return Err(BattleError::NoGamesForPlayer);
    }

    Ok(())
}
