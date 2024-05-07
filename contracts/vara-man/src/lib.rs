#![no_std]
mod sr25519;
use fungible_token_io::{FTAction, FTEvent};
use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId};
use vara_man_io::*;

// Minimum duration of session: 3 mins = 180_000 ms = 60 blocks
pub const MINIMUM_SESSION_SURATION_MS: u64 = 180_000;

#[derive(Debug, Default)]
struct VaraMan {
    tournaments: HashMap<ActorId, Tournament>,
    players_to_game_id: HashMap<ActorId, ActorId>,
    status: Status,
    config: Config,
    admins: Vec<ActorId>,
    sessions: HashMap<ActorId, Session>,
}

#[derive(Default, Debug)]
pub struct Tournament {
    tournament_name: String,
    admin: ActorId,
    level: Level,
    participants: HashMap<ActorId, Player>,
    bid: u128,
    stage: Stage,
    duration_ms: u32,
}

static mut VARA_MAN: Option<VaraMan> = None;

#[gstd::async_main]
async fn main() {
    let action: VaraManAction = msg::load().expect("Unexpected invalid action payload.");
    let vara_man: &mut VaraMan = unsafe { VARA_MAN.get_or_insert(VaraMan::default()) };

    let result = process_handle(action, vara_man).await;
    msg::reply(result, 0).expect("Unexpected invalid reply result.");
}

impl VaraMan {
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
}
#[allow(clippy::comparison_chain)]
async fn process_handle(
    action: VaraManAction,
    vara_man: &mut VaraMan,
) -> Result<VaraManEvent, VaraManError> {
    match action {
        VaraManAction::CreateNewTournament {
            tournament_name,
            name,
            level,
            duration_ms,
            session_for_account,
        } => {
            let msg_src = msg::source();
            let msg_value = msg::value();

            let player = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::CreateNewTournament,
            );
            if vara_man.tournaments.contains_key(&player) {
                msg::send_with_gas(player, "", 0, msg_value).expect("Error in sending the value");
                return Err(VaraManError::AlreadyHaveTournament);
            }
            let mut participants = HashMap::new();
            participants.insert(
                player,
                Player {
                    name: name.clone(),
                    time: 0,
                    points: 0,
                },
            );
            let game = Tournament {
                tournament_name: tournament_name.clone(),
                admin: player,
                level,
                participants,
                bid: msg_value,
                stage: Stage::Registration,
                duration_ms,
            };
            vara_man.tournaments.insert(player, game);
            vara_man.players_to_game_id.insert(player, player);
            Ok(VaraManEvent::NewTournamentCreated {
                tournament_name,
                name,
                level,
                bid: msg_value,
            })
        }
        VaraManAction::RegisterForTournament {
            admin_id,
            name,
            session_for_account,
        } => {
            let msg_src = msg::source();
            let msg_value = msg::value();
            let player = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::RegisterForTournament,
            );
            let reply = vara_man.register(player, msg_value, admin_id, name);
            if reply.is_err() {
                msg::send_with_gas(player, "", 0, msg_value).expect("Error in sending the value");
            }
            reply
        }
        VaraManAction::CancelRegister {
            session_for_account,
        } => {
            let msg_src = msg::source();
            let player = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::CancelRegister,
            );
            let admin_id = vara_man
                .players_to_game_id
                .get(&player)
                .ok_or(VaraManError::NoSuchPlayer)?;

            let game = vara_man
                .tournaments
                .get_mut(admin_id)
                .ok_or(VaraManError::NoSuchGame)?;

            if game.admin == player {
                return Err(VaraManError::AccessDenied);
            }
            if game.stage != Stage::Registration {
                return Err(VaraManError::WrongStage);
            }
            if game.bid != 0 {
                msg::send_with_gas(player, "", 0, game.bid).expect("Error in sending the value");
            }
            game.participants.remove(&player);
            vara_man.players_to_game_id.remove(&player);

            Ok(VaraManEvent::RegisterCanceled)
        }
        VaraManAction::CancelTournament {
            session_for_account,
        } => {
            let msg_src = msg::source();
            let player = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::CancelTournament,
            );
            let game = vara_man
                .tournaments
                .get(&player)
                .ok_or(VaraManError::NoSuchGame)?;

            game.participants.iter().for_each(|(id, _)| {
                if !matches!(game.stage, Stage::Finished(_)) && game.bid != 0 {
                    msg::send_with_gas(*id, "", 0, game.bid).expect("Error in sending the value");
                }
                vara_man.players_to_game_id.remove(id);
            });

            vara_man.tournaments.remove(&player);

            Ok(VaraManEvent::TournamentCanceled { admin_id: player })
        }
        VaraManAction::DeletePlayer {
            player_id,
            session_for_account,
        } => {
            let msg_src = msg::source();
            let player = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::DeletePlayer,
            );
            let game = vara_man
                .tournaments
                .get_mut(&player)
                .ok_or(VaraManError::NoSuchGame)?;

            if game.admin == player_id {
                return Err(VaraManError::AccessDenied);
            }

            if game.stage != Stage::Registration {
                return Err(VaraManError::WrongStage);
            }

            game.participants
                .remove(&player_id)
                .ok_or(VaraManError::NoSuchPlayer)?;
            vara_man
                .players_to_game_id
                .remove(&player_id)
                .ok_or(VaraManError::NoSuchPlayer)?;
            if game.bid != 0 {
                msg::send_with_gas(player_id, "", 0, game.bid).expect("Error in sending value");
            }

            Ok(VaraManEvent::PlayerDeleted { player_id })
        }
        VaraManAction::FinishSingleGame {
            gold_coins,
            silver_coins,
            level,
            session_for_account,
        } => {
            let msg_src = msg::source();
            let player_address = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::FinishSingleGame,
            );
            let (points_for_gold, points_for_silver) =
                vara_man.config.get_points_per_gold_coin_for_level(level);
            let points =
                points_for_gold * gold_coins as u128 + points_for_silver * silver_coins as u128;
            let maximum_possible_points = points_for_gold
                * vara_man.config.max_number_gold_coins as u128
                + points_for_silver * vara_man.config.max_number_silver_coins as u128;
            let prize = vara_man.config.one_point_in_value * points;

            if vara_man.status == Status::StartedWithNativeToken {
                msg::send_with_gas(player_address, "", 0, prize).expect("Error in sending value");
            } else if let Status::StartedWithFungibleToken { ft_address } = vara_man.status {
                let _transfer_response: FTEvent = msg::send_for_reply_as(
                    ft_address,
                    FTAction::Transfer {
                        from: exec::program_id(),
                        to: msg_src,
                        amount: prize,
                    },
                    0,
                    0,
                )
                .expect("Error in sending a message")
                .await
                .expect("Error in transfer Fungible Token");
            }
            Ok(VaraManEvent::SingleGameFinished {
                gold_coins,
                silver_coins,
                prize,
                points,
                maximum_possible_points,
                maximum_number_gold_coins: vara_man.config.max_number_gold_coins,
                maximum_number_silver_coins: vara_man.config.max_number_silver_coins,
                player_address,
            })
        }
        VaraManAction::StartTournament {
            session_for_account,
        } => {
            let msg_src = msg::source();
            let player = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::StartTournament,
            );
            if vara_man.status == Status::Paused {
                return Err(VaraManError::GameIsPaused);
            }
            let game = vara_man
                .tournaments
                .get_mut(&player)
                .ok_or(VaraManError::NoSuchGame)?;

            if game.stage != Stage::Registration {
                return Err(VaraManError::WrongStage);
            }
            let time_start = exec::block_timestamp();
            game.stage = Stage::Started(time_start);
            msg::send_with_gas_delayed(
                exec::program_id(),
                VaraManAction::FinishTournament {
                    admin_id: player,
                    time_start,
                },
                vara_man.config.gas_for_finish_tournament,
                0,
                game.duration_ms / 3_000 + 1,
            )
            .expect("Error in sending delayed message");
            Ok(VaraManEvent::GameStarted)
        }

        VaraManAction::FinishTournament {
            admin_id,
            time_start,
        } => {
            if msg::source() != exec::program_id() {
                return Err(VaraManError::AccessDenied);
            }
            let game = vara_man
                .tournaments
                .get_mut(&admin_id)
                .ok_or(VaraManError::NoSuchGame)?;

            if game.stage != Stage::Started(time_start) {
                return Err(VaraManError::WrongStage);
            }

            let mut winners = Vec::new();
            let mut max_points = 0;
            let mut min_time = u128::MAX;

            for (actor_id, player) in game.participants.iter() {
                if player.points > max_points {
                    max_points = player.points;
                    min_time = player.time;
                    winners.clear();
                    winners.push(*actor_id);
                } else if player.points == max_points {
                    if player.time < min_time {
                        min_time = player.time;
                        winners.clear();
                        winners.push(*actor_id);
                    } else if player.time == min_time {
                        winners.push(*actor_id);
                    }
                }
            }

            let prize = game.bid * game.participants.len() as u128 / winners.len() as u128;
            winners.iter().for_each(|id| {
                msg::send_with_gas(*id, "", 0, prize).expect("Error in sending value");
            });
            game.stage = Stage::Finished(winners.clone());
            let participants: Vec<_> = game.participants.keys().cloned().collect();

            msg::send(
                game.admin,
                Ok::<VaraManEvent, VaraManError>(VaraManEvent::GameFinished {
                    winners: winners.clone(),
                    participants: participants.clone(),
                    prize,
                }),
                0,
            )
            .expect("Error in sending message");

            Ok(VaraManEvent::GameFinished {
                winners,
                participants,
                prize,
            })
        }

        VaraManAction::RecordTournamentResult {
            time,
            gold_coins,
            silver_coins,
            session_for_account,
        } => {

            let msg_src = msg::source();

            let player_address = vara_man.get_player(
                &msg_src,
                &session_for_account,
                ActionsForSession::RecordTournamentResult,
            );
            let admin_id = vara_man
                .players_to_game_id
                .get(&player_address)
                .ok_or(VaraManError::NoSuchPlayer)?;
            let game = vara_man
                .tournaments
                .get_mut(admin_id)
                .ok_or(VaraManError::NoSuchGame)?;

            if !matches!(game.stage, Stage::Started(_)) {
                return Err(VaraManError::WrongStage);
            }

            let player = game
                .participants
                .get_mut(&player_address)
                .ok_or(VaraManError::NoSuchPlayer)?;

            let (points_for_gold, points_for_silver) = vara_man
                .config
                .get_points_per_gold_coin_for_level(game.level);
            let points =
                points_for_gold * gold_coins as u128 + points_for_silver * silver_coins as u128;
            let maximum_possible_points = points_for_gold
                * vara_man.config.max_number_gold_coins as u128
                + points_for_silver * vara_man.config.max_number_silver_coins as u128;
            player.time += time;
            player.points += points;


            Ok(VaraManEvent::ResultTournamentRecorded {
                gold_coins,
                silver_coins,
                time: player.time,
                points: player.points,
                maximum_possible_points,
                maximum_number_gold_coins: vara_man.config.max_number_gold_coins,
                maximum_number_silver_coins: vara_man.config.max_number_silver_coins,
                player_address,
            })
        }

        VaraManAction::LeaveGame {
            session_for_account,
        } => {
            let player = vara_man.get_player(
                &msg::source(),
                &session_for_account,
                ActionsForSession::LeaveGame,
            );
            vara_man.players_to_game_id.remove(&player);
            Ok(VaraManEvent::LeftGame)
        }
        VaraManAction::ChangeStatus(status) => {
            if vara_man.admins.contains(&msg::source()) {
                vara_man.status = status;
                Ok(VaraManEvent::StatusChanged(status))
            } else {
                Err(VaraManError::NotAdmin)
            }
        }
        VaraManAction::ChangeConfig(config) => {
            if !vara_man.admins.contains(&msg::source()) {
                Err(VaraManError::NotAdmin)
            } else {
                vara_man.config = config;
                Ok(VaraManEvent::ConfigChanged(config))
            }
        }
        VaraManAction::AddAdmin(admin) => {
            if vara_man.admins.contains(&msg::source()) {
                vara_man.admins.push(admin);
                Ok(VaraManEvent::AdminAdded(admin))
            } else {
                Err(VaraManError::NotAdmin)
            }
        }
        VaraManAction::CreateSession {
            key,
            duration,
            allowed_actions,
            signature,
        } => {
            assert!(
                duration >= MINIMUM_SESSION_SURATION_MS,
                "Duration is too small"
            );

            let msg_source = msg::source();
            let block_timestamp = exec::block_timestamp();
            let block_height = exec::block_height();

            let expires = block_timestamp + duration;

            let number_of_blocks = u32::try_from(duration.div_ceil(vara_man.config.block_duration_ms))
                .expect("Duration is too large");

            assert!(
                !allowed_actions.is_empty(),
                "No messages for approval were passed."
            );

            let account = match signature {
                Some(sig_bytes) => {
                    vara_man.check_if_session_exists(&key);
                    let pub_key: [u8; 32] = key.into();
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
                    vara_man.sessions.entry(key).insert(Session {
                        key: msg_source,
                        expires,
                        allowed_actions,
                        expires_at_block: block_height + number_of_blocks,
                    });
                    key
                }
                None => {
                    vara_man.check_if_session_exists(&msg_source);

                    vara_man.sessions.entry(msg_source).insert(Session {
                        key,
                        expires,
                        allowed_actions,
                        expires_at_block: block_height + number_of_blocks,
                    });
                    msg_source
                }
            };

            msg::send_with_gas_delayed(
                exec::program_id(),
                VaraManAction::DeleteSessionFromProgram { account },
                vara_man.config.gas_to_delete_session,
                0,
                number_of_blocks,
            )
            .expect("Error in sending a delayed msg");

            Ok(VaraManEvent::SessionCreated)
        }
        VaraManAction::DeleteSessionFromAccount => {
            assert!(
                vara_man.sessions.remove(&msg::source()).is_some(),
                "No session"
            );
            Ok(VaraManEvent::SessionDeleted)
        }
        VaraManAction::DeleteSessionFromProgram { account } => {
            assert_eq!(
                exec::program_id(),
                msg::source(),
                "The msg source must be the program"
            );

            if let Some(session) = vara_man.sessions.remove(&account) {
                assert!(
                    session.expires_at_block <= exec::block_height(),
                    "Too early to delete session"
                );
            }
            Ok(VaraManEvent::SessionDeleted)
        }
    }
}

impl VaraMan {
    fn register(
        &mut self,
        msg_src: ActorId,
        msg_value: u128,
        admin_id: ActorId,
        name: String,
    ) -> Result<VaraManEvent, VaraManError> {
        if self.status == Status::Paused {
            return Err(VaraManError::GameIsPaused);
        }

        if self.players_to_game_id.contains_key(&msg_src) {
            return Err(VaraManError::SeveralRegistrations);
        }
        let game = self
            .tournaments
            .get_mut(&admin_id)
            .ok_or(VaraManError::NoSuchGame)?;

        if game.stage != Stage::Registration {
            return Err(VaraManError::WrongStage);
        }
        if game.participants.len() >= MAX_PARTICIPANTS.into() {
            return Err(VaraManError::SessionFull);
        }
        if game.bid != msg_value {
            return Err(VaraManError::WrongBid);
        }

        game.participants.insert(
            msg_src,
            Player {
                name: name.clone(),
                time: 0,
                points: 0,
            },
        );
        self.players_to_game_id.insert(msg_src, admin_id);
        Ok(VaraManEvent::PlayerRegistered {
            admin_id,
            name,
            bid: msg_value,
        })
    }
}

#[no_mangle]
extern fn init() {
    let init: VaraManInit = msg::load().expect("Unexpected invalid init payload.");
    unsafe {
        VARA_MAN = Some(VaraMan {
            config: init.config,
            admins: vec![msg::source()],
            ..Default::default()
        })
    };
}

#[no_mangle]
extern fn state() {
    let contract = unsafe { VARA_MAN.take().expect("Unexpected error in taking state") };

    let query: StateQuery = msg::load().expect("Unable to load the state query");

    let reply = match query {
        StateQuery::All => StateReply::All(contract.into()),
        StateQuery::GetTournament { player_id } => {
            if let Some(admin_id) = contract.players_to_game_id.get(&player_id) {
                if let Some(tournament) = contract.tournaments.get(admin_id) {
                    let tournament_state = TournamentState {
                        tournament_name: tournament.tournament_name.clone(),
                        admin: tournament.admin,
                        level: tournament.level,
                        participants: tournament.participants.clone().into_iter().collect(),
                        bid: tournament.bid,
                        stage: tournament.stage.clone(),
                        duration_ms: tournament.duration_ms,
                    };
                    let time = match tournament.stage {
                        Stage::Started(start_time) => Some(exec::block_timestamp() - start_time),
                        _ => None,
                    };
                    StateReply::Tournament(Some((tournament_state, time)))
                } else {
                    StateReply::Tournament(None)
                }
            } else {
                StateReply::Tournament(None)
            }
        }
        StateQuery::Config => StateReply::Config(contract.config),
        StateQuery::Admins => StateReply::Admins(contract.admins),
        StateQuery::Status => StateReply::Status(contract.status),
        StateQuery::SessionForTheAccount(account) => {
            StateReply::SessionForTheAccount(contract.sessions.get(&account).cloned())
        }
    };
    msg::reply(reply, 0).expect("Unable to share the state");
}

impl From<VaraMan> for VaraManState {
    fn from(value: VaraMan) -> Self {
        let VaraMan {
            tournaments,
            players_to_game_id,
            status,
            config,
            admins,
            sessions: _,
        } = value;

        let tournaments = tournaments
            .into_iter()
            .map(|(id, tournament)| {
                let tournament_state = TournamentState {
                    tournament_name: tournament.tournament_name,
                    admin: tournament.admin,
                    level: tournament.level,
                    participants: tournament.participants.into_iter().collect(),
                    bid: tournament.bid,
                    stage: tournament.stage,
                    duration_ms: tournament.duration_ms,
                };
                (id, tournament_state)
            })
            .collect();

        let players_to_game_id = players_to_game_id.into_iter().collect();

        Self {
            tournaments,
            players_to_game_id,
            status,
            config,
            admins,
        }
    }
}
