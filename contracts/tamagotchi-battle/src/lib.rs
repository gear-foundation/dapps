#![no_std]

use collections::HashSet;
use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId, MessageId};
use tamagotchi_battle_io::*;
use tamagotchi_io::{Error, TmgAction, TmgReply};
mod pair;
use pair::*;
mod utils;
use utils::{generate_power, get_random_value, BattleUtils};
mod player;
mod sr25519;
const COLORS: [&str; 6] = ["Green", "Red", "Blue", "Purple", "Orange", "Yellow"];
// Minimum duration of session: 1 min = 60_000 ms = 20 blocks
pub const MINIMUM_SESSION_SURATION_MS: u64 = 60_000;

#[derive(Default)]
struct Battle {
    admins: Vec<ActorId>,
    players: HashMap<ActorId, Player>,
    players_ids: Vec<ActorId>,
    current_players: Vec<ActorId>,
    active_tmg_owners: Vec<ActorId>,
    state: BattleState,
    current_winner: ActorId,
    pairs: HashMap<PairId, Pair>,
    players_to_pairs: HashMap<ActorId, HashSet<PairId>>,
    completed_games: u8,
    waitlist: HashSet<MessageId>,
    config: Config,
    sessions: HashMap<ActorId, Session>,
}

static mut BATTLE: Option<Battle> = None;

impl Battle {
    fn create_session(
        &mut self,
        key: &ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
        signature: Option<Vec<u8>>,
    ) -> Result<BattleReply, BattleError> {
        assert!(
            duration >= MINIMUM_SESSION_SURATION_MS,
            "Duration is too small"
        );

        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        let block_height = exec::block_height();

        let expires = block_timestamp + duration;

        let number_of_blocks = u32::try_from(duration.div_ceil(self.config.block_duration_ms))
            .expect("Duration is too large");

        assert!(
            !allowed_actions.is_empty(),
            "No messages for approval were passed."
        );

        let account = match signature {
            Some(sig_bytes) => {
                self.check_if_session_exists(key);
                let pub_key: [u8; 32] = (*key).into();
                let mut prefix = b"<Bytes>".to_vec();
                let mut message = SignatureData {
                    key: msg_source,
                    duration,
                    allowed_actions: allowed_actions.clone(),
                }
                .encode();
                let mut postfix = b"</Bytes>".to_vec();
                prefix.append(&mut message);
                prefix.append(&mut postfix);

                if crate::sr25519::verify(&sig_bytes, prefix, pub_key).is_err() {
                    panic!("Failed sign verification");
                }
                self.sessions.entry(*key).insert(Session {
                    key: msg_source,
                    expires,
                    allowed_actions,
                    expires_at_block: block_height + number_of_blocks,
                });
                *key
            }
            None => {
                self.check_if_session_exists(&msg_source);

                self.sessions.entry(msg_source).insert(Session {
                    key: *key,
                    expires,
                    allowed_actions,
                    expires_at_block: block_height + number_of_blocks,
                });
                msg_source
            }
        };

        msg::send_with_gas_delayed(
            exec::program_id(),
            BattleAction::DeleteSessionFromProgram { account },
            self.config.gas_to_delete_session,
            0,
            number_of_blocks,
        )
        .expect("Error in sending a delayed msg");

        Ok(BattleReply::SessionCreated)
    }

    fn check_if_session_exists(&self, account: &ActorId) {
        if let Some(Session {
            key: _,
            expires: _,
            allowed_actions: _,
            expires_at_block,
        }) = self.sessions.get(account)
        {
            if *expires_at_block > exec::block_height() {
                panic!("You already have an active session. If you want to create a new one, please delete this one.")
            }
        }
    }

    fn delete_session_from_program(
        &mut self,
        session_for_account: &ActorId,
    ) -> Result<BattleReply, BattleError> {
        assert_eq!(
            exec::program_id(),
            msg::source(),
            "The msg source must be the program"
        );

        if let Some(session) = self.sessions.remove(session_for_account) {
            assert!(
                session.expires_at_block <= exec::block_height(),
                "Too early to delete session"
            );
        }
        Ok(BattleReply::SessionDeleted)
    }

    fn delete_session_from_account(&mut self) -> Result<BattleReply, BattleError> {
        assert!(self.sessions.remove(&msg::source()).is_some(), "No session");
        Ok(BattleReply::SessionDeleted)
    }

    fn start_registration(&mut self) -> Result<BattleReply, BattleError> {
        //self.check_state(BattleState::GameIsOver)?;
        self.check_admin(&msg::source())?;
        self.state = BattleState::Registration;
        self.current_winner = ActorId::zero();
        self.players_ids = Vec::new();
        self.completed_games = 0;
        self.players_to_pairs = HashMap::new();
        self.current_players = Vec::new();
        self.active_tmg_owners = Vec::new();
        self.pairs = HashMap::new();
        Ok(BattleReply::RegistrationStarted)
    }

    async fn register(
        &mut self,
        tmg_id: &TamagotchiId,
        session_for_account: Option<ActorId>,
    ) -> Result<BattleReply, BattleError> {
        let player = self.get_player(
            &msg::source(),
            &session_for_account,
            ActionsForSession::Register,
        );

        self.check_state(BattleState::Registration)?;

        self.check_max_participants()?;

        self.check_if_tmg_in_game(tmg_id)?;

        let (owner, name, date_of_birth) = get_tmg_info(tmg_id).await?;

        check_tmg_owner(owner, player)?;

        if !self.players.contains_key(tmg_id) {
            let power = generate_power(self.config.min_power, self.config.max_power, *tmg_id);
            let defence = self.config.max_power - power;
            let color_index = get_random_value(COLORS.len() as u8);
            let player = Player {
                owner,
                name,
                date_of_birth,
                tmg_id: *tmg_id,
                defence,
                power,
                health: self.config.health,
                color: COLORS[color_index as usize].to_string(),
                victories: 0,
            };
            self.players.insert(*tmg_id, player);
        } else {
            self.players
                .entry(*tmg_id)
                .and_modify(|player| player.health = self.config.health);
        }

        self.players_ids.push(*tmg_id);
        self.current_players.push(*tmg_id);
        self.active_tmg_owners.push(owner);

        Ok(BattleReply::Registered { tmg_id: *tmg_id })
    }

    /// Starts the battle.
    /// This message must be sent after the registration end (the contract is in the `BattleState::Registration` state)
    /// It must also be sent when the game is on but a round is ended (the contract is in the `BattleState::WaitNextRound` state)
    /// BattleState::WaitNextRound` state means the the battles in pairs are over and winners are expecting to play in the next round
    fn start_battle(
        &mut self
    ) -> Result<BattleReply, BattleError> {
        match self.state {
            BattleState::Registration | BattleState::WaitNextRound => {
                self.check_min_player_amount()?;
                self.check_admin(&msg::source())?;

                // Clear the state if the state is `BattleState::WaitNextRound`
                self.pairs = HashMap::new();
                self.players_to_pairs = HashMap::new();
                self.completed_games = 0;

                self.split_into_pairs()?;

                self.state = BattleState::GameIsOn;

                // After the battle starts, the contract waits for a specific period of time (`time_for_move` from the config),
                // usually equivalent to one minute, to check whether all participants have made their move.
                exec::wait_for(self.config.time_for_move + 1);
            }
            BattleState::GameIsOn => {
                let mut number_of_missed_turns = 0;
                // if both players missed their turns then pair is removed from the battle
                let mut pair_ids_to_remove = Vec::new();
                let timestamp = exec::block_timestamp();
                let time_for_move_ms =
                    self.config.block_duration_ms * u64::from(self.config.time_for_move);

                for (pair_id, pair) in self.pairs.iter_mut() {
                    // If the last update of the structure was more than the time_for_move ago,
                    //the contract sets the player's move to None and allows the second player to make their move.
                    if timestamp.saturating_sub(pair.last_updated) >= time_for_move_ms {
                        if pair.moves.is_empty() {
                            pair.moves.push(None);
                            pair.last_updated = timestamp;
                            pair.move_deadline = timestamp + time_for_move_ms;
                            number_of_missed_turns += 1;
                        } else {
                            // If the contract observes that both players have missed their turn,
                            // it removes that pair from the game.
                            pair_ids_to_remove.push((
                                *pair_id,
                                pair.owner_ids[0],
                                pair.owner_ids[1],
                            ));
                        }
                    }
                }

                for (id, owner_0, owner_1) in pair_ids_to_remove.into_iter() {
                    let pairs_len = self.pairs.len() - 1;
                    self.remove_pair(&id, vec![owner_0, owner_1]);
                    check_all_games_completion(
                        self.completed_games,
                        pairs_len as u8,
                        &mut self.state,
                        &mut self.current_winner,
                        &self.players_ids,
                    );
                }

                if number_of_missed_turns > 0 {
                    exec::wait_for(self.config.time_for_move + 1);
                }
                if self.pairs.is_empty() {
                    self.state = BattleState::GameIsOver;
                    // clear current players
                    self.current_players = Vec::new();
                    return Ok(BattleReply::BattleWasCancelled);
                }
            }
            _ => return Err(BattleError::WrongState),
        }
        Ok(BattleReply::BattleStarted)
    }

    fn split_into_pairs(&mut self) -> Result<(), BattleError> {
        let mut players_len = self.players_ids.len() as u8;

        let last_updated = exec::block_timestamp();
        let time_for_move_ms = self.config.block_duration_ms * u64::from(self.config.time_for_move);
        for pair_id in 0..self.players_ids.len() as u8 {
            self.create_pair(
                &mut players_len,
                pair_id,
                last_updated,
                last_updated + time_for_move_ms,
            )?;

            if players_len == 1 || players_len == 0 {
                return Ok(());
            }
        }
        Ok(())
    }

    fn make_move(
        &mut self,
        pair_id: PairId,
        tmg_move: Move,
        session_for_account: Option<ActorId>,
    ) -> Result<BattleReply, BattleError> {
        self.check_state(BattleState::GameIsOn)?;
        let pairs_len = self.pairs.len() as u8;

        let player = self.get_player(
            &msg::source(),
            &session_for_account,
            ActionsForSession::MakeMove,
        );

        let pair = get_mut_pair(&mut self.pairs, pair_id)?;

        if pair.game_is_over {
            return Err(BattleError::GameIsOver);
        }
        let current_msg_id = msg::id();
        let timestamp = exec::block_timestamp();

        // Check whether the message is being executed for the first time or was in the waitlist.
        // This is necessary to verify whether a player has missed their turn.
        if pair.msg_id_in_waitlist == current_msg_id {
            let time_for_move_ms =
                self.config.block_duration_ms * u64::from(self.config.time_for_move);
            if timestamp.saturating_sub(pair.last_updated) >= time_for_move_ms {
                // the move was skipped
                pair.moves.push(None);
                pair.amount_of_skipped_moves += 1;

                // if two turns are missed in a row
                if pair.amount_of_skipped_moves >= 2 {
                    let owners = pair.owner_ids.clone();
                    self.remove_pair(&pair_id, owners);
                    let pairs_len = pairs_len - 1;
                    check_all_games_completion(
                        self.completed_games,
                        pairs_len,
                        &mut self.state,
                        &mut self.current_winner,
                        &self.players_ids,
                    );
                    self.waitlist.remove(&current_msg_id);
                    return Ok(BattleReply::BattleWasCancelled);
                }
            } else {
                return Ok(BattleReply::MoveMade);
            }
        } else {
            if self.waitlist.remove(&current_msg_id) {
                return Ok(BattleReply::WaitlistMsgCancelled);
            }
            // Player's new move.
            // All necessary checks must be performed.
            let current_turn = pair.moves.len();
            let tmg_owner = pair.owner_ids[current_turn];
            
            check_tmg_owner(tmg_owner, player)?;
            is_pair_id_in_player_pair_ids(&self.players_to_pairs, &player, pair_id)?;
            pair.moves.push(Some(tmg_move));
            pair.amount_of_skipped_moves = 0;
        }

        if pair.moves.len() == 2 {
            let (tmg_id_0, tmg_id_1) = (pair.tmg_ids[0], pair.tmg_ids[1]);
            let mut player_0 = get_player(&self.players, &tmg_id_0)?;
            let mut player_1 = get_player(&self.players, &tmg_id_1)?;

            // save moves fo event
            let moves = pair.moves.clone();
            let (health_loss_0, health_loss_1) = pair.process_round_outcome(
                &mut player_0,
                &mut player_1,
                &mut self.players_ids,
                &mut self.completed_games,
                &self.config,
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
                &moves,
            );
        }
        if pair.msg_id_in_waitlist != MessageId::from([0; 32]) {
            exec::wake(pair.msg_id_in_waitlist).expect("Error during wake");
        }
        pair.msg_id_in_waitlist = current_msg_id;
        if !pair.game_is_over {
            // After the move was made, the contract waits for a specific period of time (`time_for_move` from the config),
            // usually equivalent to one minute, to check whether the next player has made his move.
            self.waitlist.insert(current_msg_id);
            pair.last_updated = timestamp;
            let time_for_move_ms =
                self.config.block_duration_ms * u64::from(self.config.time_for_move);
            pair.move_deadline = timestamp + time_for_move_ms;
            exec::wait_for(self.config.time_for_move + 1);
        }

        Ok(BattleReply::GameFinished {
            players: self.active_tmg_owners.clone(),
        })
    }

    fn add_admin(&mut self, new_admin: &ActorId) -> Result<BattleReply, BattleError> {
        if !self.admins.contains(&msg::source()) {
            panic!("Only admin can add another admin");
        }
        self.admins.push(*new_admin);
        Ok(BattleReply::AdminAdded)
    }

    fn get_player(
        &self,
        msg_source: &ActorId,
        session_for_account: &Option<ActorId>,
        actions_for_session: ActionsForSession,
    ) -> ActorId {
        let player = match session_for_account {
            Some(account) => {
                let session = self
                    .sessions
                    .get(account)
                    .expect("This account has no valid session");
                assert!(
                    session.expires > exec::block_timestamp(),
                    "The session has already expired"
                );
                assert!(
                    session.allowed_actions.contains(&actions_for_session),
                    "This message is not allowed"
                );
                assert_eq!(
                    session.key, *msg_source,
                    "The account is not approved for this session"
                );
                *account
            }
            None => *msg_source,
        };
        player
    }
}

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load().expect("Unable to decode `BattleAction`");
    let battle = unsafe { BATTLE.get_or_insert(Default::default()) };
    let reply = match action {
        BattleAction::StartRegistration => battle.start_registration(),
        BattleAction::Register {
            tmg_id,
            session_for_account,
        } => battle.register(&tmg_id, session_for_account).await,
        BattleAction::MakeMove {
            pair_id,
            tmg_move,
            session_for_account,
        } => battle.make_move(pair_id, tmg_move, session_for_account),
        BattleAction::StartBattle => battle.start_battle(),
        BattleAction::AddAdmin(new_admin) => battle.add_admin(&new_admin),
        BattleAction::CreateSession {
            key,
            duration,
            allowed_actions,
            signature,
        } => battle.create_session(&key, duration, allowed_actions, signature),
        BattleAction::DeleteSessionFromAccount => battle.delete_session_from_account(),
        BattleAction::DeleteSessionFromProgram { account } => {
            battle.delete_session_from_program(&account)
        }
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

pub async fn get_tmg_info(tmg_id: &ActorId) -> Result<(ActorId, String, u64), BattleError> {
    let reply: Result<TmgReply, Error> = msg::send_for_reply_as(*tmg_id, TmgAction::TmgInfo, 0, 0)
        .expect("Error in sending a message `TmgAction::TmgInfo")
        .await
        .expect("Unable to decode TmgReply");

    match reply {
        Ok(TmgReply::TmgInfo {
            owner,
            name,
            date_of_birth,
        }) => Ok((owner, name, date_of_birth)),
        Err(Error::TamagotchiHasDied) => Err(BattleError::TamagotchiHasDied),
        _ => panic!("Wrong received message"),
    }
}

#[no_mangle]
extern fn state() {
    let query: BattleQuery = msg::load().expect("Unable to load the query");
    let battle = unsafe { BATTLE.take().expect("Unexpected error in taking state") };
    let reply = match query {
        BattleQuery::GetPlayer { tmg_id } => {
            let player = battle.players.get(&tmg_id).cloned();
            BattleQueryReply::Player { player }
        }
        BattleQuery::PlayersIds => BattleQueryReply::PlayersIds {
            players_ids: battle.players_ids,
        },
        BattleQuery::State => BattleQueryReply::State {
            state: battle.state,
        },
        BattleQuery::GetPairs => BattleQueryReply::Pairs {
            pairs: battle.pairs.into_iter().collect(),
        },
        BattleQuery::GetPair { pair_id } => {
            let pair = battle.pairs.get(&pair_id).cloned();
            BattleQueryReply::Pair { pair }
        }
        BattleQuery::Admins => BattleQueryReply::Admins {
            admins: battle.admins,
        },
        BattleQuery::CurrentPlayers => BattleQueryReply::CurrentPlayers {
            current_players: battle.current_players,
        },
        BattleQuery::Players => BattleQueryReply::Players {
            players: battle.players.into_iter().collect(),
        },
        BattleQuery::CompletedGames => BattleQueryReply::CompletedGames {
            completed_games: battle.completed_games,
        },
        BattleQuery::Winner => BattleQueryReply::Winner {
            winner: battle.current_winner,
        },
        BattleQuery::SessionForTheAccount(account) => {
            BattleQueryReply::SessionForTheAccount(battle.sessions.get(&account).cloned())
        }
    };
    msg::reply(reply, 0).expect("Failed to share state");
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
    pairs: &mut HashMap<PairId, Pair>,
    pair_id: PairId,
) -> Result<&mut Pair, BattleError> {
    if let Some(pair) = pairs.get_mut(&pair_id) {
        Ok(pair)
    } else {
        Err(BattleError::PairDoesNotExist)
    }
}

fn get_player(players: &HashMap<ActorId, Player>, tmg_id: &ActorId) -> Result<Player, BattleError> {
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
        match players_ids.len() {
            0 => {
                *state = BattleState::GameIsOver;
            }
            1 => {
                *state = BattleState::GameIsOver;
                *current_winner = players_ids[0];
            }
            _ => {
                *state = BattleState::WaitNextRound;
            }
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
    players_to_pairs: &HashMap<ActorId, HashSet<u8>>,
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
