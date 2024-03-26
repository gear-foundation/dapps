#![no_std]

use fungible_token_io::{FTAction, FTEvent};
use gstd::{collections::HashMap, debug, exec, msg, prelude::*, ActorId};
use vara_man_io::*;

#[derive(Debug, Default)]
struct VaraMan {
    tournaments: HashMap<ActorId, Tournament>,
    players_to_game_id: HashMap<ActorId, ActorId>,
    status: Status,
    config: Config,
    admins: Vec<ActorId>,
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
    debug!("RESULT: {:?}", result);
    msg::reply(result, 0).expect("Unexpected invalid reply result.");
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
        } => {
            let msg_src = msg::source();
            let msg_value = msg::value();

            if vara_man.tournaments.contains_key(&msg_src) {
                msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending the value");
                return Err(VaraManError::AlreadyHaveTournament);
            }
            let mut participants = HashMap::new();
            participants.insert(
                msg_src,
                Player {
                    name: name.clone(),
                    time: 0,
                    points: 0,
                },
            );
            let game = Tournament {
                tournament_name: tournament_name.clone(),
                admin: msg_src,
                level,
                participants,
                bid: msg_value,
                stage: Stage::Registration,
                duration_ms,
            };
            vara_man.tournaments.insert(msg_src, game);
            vara_man.players_to_game_id.insert(msg_src, msg_src);
            Ok(VaraManEvent::NewTournamentCreated {
                tournament_name,
                name,
                level,
                bid: msg_value,
            })
        }
        VaraManAction::RegisterForTournament { admin_id, name } => {
            let msg_src = msg::source();
            let msg_value = msg::value();
            let reply = vara_man.register(msg_src, msg_value, admin_id, name);
            if reply.is_err() {
                msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending the value");
            }
            reply
        }
        VaraManAction::CancelRegister => {
            let msg_src = msg::source();
            let admin_id = vara_man
                .players_to_game_id
                .get(&msg_src)
                .ok_or(VaraManError::NoSuchPlayer)?;

            let game = vara_man
                .tournaments
                .get_mut(admin_id)
                .ok_or(VaraManError::NoSuchGame)?;

            if game.admin == msg_src {
                return Err(VaraManError::AccessDenied);
            }
            if game.stage != Stage::Registration {
                return Err(VaraManError::WrongStage);
            }
            if game.bid != 0 {
                msg::send_with_gas(msg_src, "", 0, game.bid).expect("Error in sending the value");
            }
            game.participants.remove(&msg_src);
            vara_man.players_to_game_id.remove(&msg_src);

            Ok(VaraManEvent::RegisterCanceled)
        }
        VaraManAction::CancelTournament => {
            let msg_src = msg::source();
            let game = vara_man
                .tournaments
                .get(&msg_src)
                .ok_or(VaraManError::NoSuchGame)?;

            game.participants.iter().for_each(|(id, _)| {
                if !matches!(game.stage, Stage::Finished(_)) && game.bid != 0 {
                    msg::send_with_gas(*id, "", 0, game.bid).expect("Error in sending the value");
                }
                vara_man.players_to_game_id.remove(id);
            });

            vara_man.tournaments.remove(&msg_src);

            Ok(VaraManEvent::TournamentCanceled { admin_id: msg_src })
        }
        VaraManAction::DeletePlayer { player_id } => {
            let msg_src = msg::source();
            let game = vara_man
                .tournaments
                .get_mut(&msg_src)
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
        } => {
            let msg_src = msg::source();

            let (points_for_gold, points_for_silver) =
                vara_man.config.get_points_per_gold_coin_for_level(level);
            let points = points_for_gold * gold_coins + points_for_silver * silver_coins;
            let prize = vara_man.config.one_point_in_value * points;

            if vara_man.status == Status::StartedWithNativeToken {
                msg::send_with_gas(msg_src, "", 0, prize).expect("Error in sending value");
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
            Ok(VaraManEvent::GameFinished {
                winners: vec![msg_src],
                prize,
            })
        }
        VaraManAction::StartTournament => {
            let msg_src = msg::source();
            if vara_man.status == Status::Paused {
                return Err(VaraManError::GameIsPaused);
            }
            let game = vara_man
                .tournaments
                .get_mut(&msg_src)
                .ok_or(VaraManError::NoSuchGame)?;

            if game.stage != Stage::Registration {
                return Err(VaraManError::WrongStage);
            }
            let time_start = exec::block_timestamp();
            game.stage = Stage::Started(time_start);
            msg::send_with_gas_delayed(
                exec::program_id(),
                VaraManAction::FinishTournament {
                    admin_id: msg_src,
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

            Ok(VaraManEvent::GameFinished { winners, prize })
        }

        VaraManAction::RecordTournamentResult {
            time,
            gold_coins,
            silver_coins,
        } => {
            let msg_src = msg::source();
            let admin_id = vara_man
                .players_to_game_id
                .get(&msg_src)
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
                .get_mut(&msg_src)
                .ok_or(VaraManError::NoSuchPlayer)?;

            let (points_for_gold, points_for_silver) = vara_man
                .config
                .get_points_per_gold_coin_for_level(game.level);
            let points = points_for_gold * gold_coins + points_for_silver * silver_coins;
            player.time += time;
            player.points += points;

            Ok(VaraManEvent::ResultTournamentRecorded {
                time: player.time,
                points: player.points,
            })
        }

        VaraManAction::LeaveGame => {
            vara_man.players_to_game_id.remove(&msg::source());
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
