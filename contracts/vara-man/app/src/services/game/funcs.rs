use crate::services::game::{
    Config, Event, GameError, GameStorage, Level, Player, Stage, Status, Tournament,
    MAX_PARTICIPANTS,
};
use crate::services::session::utils::{ActionsForSession, SessionData};
use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId};
use sails_rs::U256;

pub fn create_new_tournament(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    tournament_name: String,
    name: String,
    level: Level,
    duration_ms: u32,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let msg_value = msg::value();

    let player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::CreateNewTournament,
    );
    if storage.status == Status::Paused {
        msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending the value");
        return Err(GameError::GameIsPaused);
    }

    if storage.tournaments.contains_key(&player) {
        msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending the value");
        return Err(GameError::AlreadyHaveTournament);
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
    storage.tournaments.insert(player, game);
    storage.players_to_game_id.insert(player, player);
    Ok(Event::NewTournamentCreated {
        tournament_name,
        name,
        level,
        bid: msg_value,
    })
}

pub fn register_for_tournament(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    admin_id: ActorId,
    name: String,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let msg_value = msg::value();
    let player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::RegisterForTournament,
    );
    let reply = register(storage, player, msg_value, admin_id, name);
    if reply.is_err() {
        msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending the value");
    }
    reply
}

fn register(
    storage: &mut GameStorage,
    player: ActorId,
    msg_value: u128,
    admin_id: ActorId,
    name: String,
) -> Result<Event, GameError> {
    if storage.status == Status::Paused {
        return Err(GameError::GameIsPaused);
    }

    if storage.players_to_game_id.contains_key(&player) {
        return Err(GameError::SeveralRegistrations);
    }
    let game = storage
        .tournaments
        .get_mut(&admin_id)
        .ok_or(GameError::NoSuchGame)?;

    if game.stage != Stage::Registration {
        return Err(GameError::WrongStage);
    }
    if game.participants.len() >= MAX_PARTICIPANTS.into() {
        return Err(GameError::SessionFull);
    }
    if game.bid != msg_value {
        return Err(GameError::WrongBid);
    }

    game.participants.insert(
        player,
        Player {
            name: name.clone(),
            time: 0,
            points: 0,
        },
    );
    storage.players_to_game_id.insert(player, admin_id);
    Ok(Event::PlayerRegistered {
        admin_id,
        name,
        bid: msg_value,
    })
}

pub fn cancel_register(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::CancelRegister,
    );
    let admin_id = storage
        .players_to_game_id
        .get(&player)
        .ok_or(GameError::NoSuchPlayer)?;

    let game = storage
        .tournaments
        .get_mut(admin_id)
        .ok_or(GameError::NoSuchGame)?;

    if game.admin == player {
        return Err(GameError::AccessDenied);
    }
    if game.stage != Stage::Registration {
        return Err(GameError::WrongStage);
    }
    if game.bid != 0 {
        msg::send_with_gas(msg_src, "", 0, game.bid).expect("Error in sending the value");
    }
    game.participants.remove(&player);
    storage.players_to_game_id.remove(&player);

    Ok(Event::RegisterCanceled)
}

pub fn cancel_tournament(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::CancelTournament,
    );
    let game = storage
        .tournaments
        .get(&player)
        .ok_or(GameError::NoSuchGame)?;

    game.participants.iter().for_each(|(id, _)| {
        if !matches!(game.stage, Stage::Finished(_)) && game.bid != 0 {
            msg::send_with_gas(*id, "", 0, game.bid).expect("Error in sending the value");
        }
        storage.players_to_game_id.remove(id);
    });

    storage.tournaments.remove(&player);

    Ok(Event::TournamentCanceled { admin_id: player })
}

pub fn delete_player(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    player_id: ActorId,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::DeletePlayer,
    );
    let game = storage
        .tournaments
        .get_mut(&player)
        .ok_or(GameError::NoSuchGame)?;

    if game.admin == player_id {
        return Err(GameError::AccessDenied);
    }

    if game.stage != Stage::Registration {
        return Err(GameError::WrongStage);
    }

    game.participants
        .remove(&player_id)
        .ok_or(GameError::NoSuchPlayer)?;
    storage
        .players_to_game_id
        .remove(&player_id)
        .ok_or(GameError::NoSuchPlayer)?;
    if game.bid != 0 {
        msg::send_with_gas(player_id, "", 0, game.bid).expect("Error in sending value");
    }

    Ok(Event::PlayerDeleted { player_id })
}

pub async fn finish_single_game(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    gold_coins: u16,
    silver_coins: u16,
    level: Level,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    if gold_coins > storage.config.max_number_gold_coins
        || silver_coins > storage.config.max_number_silver_coins
    {
        return Err(GameError::ExceededLimit);
    }

    let msg_src = msg::source();
    let _player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::FinishSingleGame,
    );
    let (points_for_gold, points_for_silver) =
        storage.config.get_points_per_gold_coin_for_level(level);
    let points = points_for_gold * gold_coins as u128 + points_for_silver * silver_coins as u128;
    let maximum_possible_points = points_for_gold * storage.config.max_number_gold_coins as u128
        + points_for_silver * storage.config.max_number_silver_coins as u128;
    let prize = storage.config.one_point_in_value * points;

    if storage.status == Status::StartedWithNativeToken {
        msg::send_with_gas(msg_src, "", 0, prize).expect("Error in sending value");
    } else if let Status::StartedWithFungibleToken { ft_address } = storage.status {
        let value: U256 = prize.into();
        let request = [
            "Vft".encode(),
            "Mint".to_string().encode(),
            (msg_src, value).encode(),
        ]
        .concat();
        // msg::send_bytes_with_gas(ft_address, request, storage.config.gas_for_mint_fungible_token, 0).expect("Error in sending a message");
        msg::send_bytes_with_gas_for_reply(
            ft_address,
            request,
            storage.config.gas_for_mint_fungible_token,
            0,
            0,
        )
        .expect("Error in sending a message")
        .await
        .expect("Error in mint Fungible Token");
    }
    Ok(Event::SingleGameFinished {
        gold_coins,
        silver_coins,
        prize,
        points,
        maximum_possible_points,
        maximum_number_gold_coins: storage.config.max_number_gold_coins,
        maximum_number_silver_coins: storage.config.max_number_silver_coins,
    })
}

pub fn start_tournament(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    let msg_src = msg::source();
    let player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::StartTournament,
    );
    if storage.status == Status::Paused {
        return Err(GameError::GameIsPaused);
    }
    let game = storage
        .tournaments
        .get_mut(&player)
        .ok_or(GameError::NoSuchGame)?;

    if game.stage != Stage::Registration {
        return Err(GameError::WrongStage);
    }
    let time_start = exec::block_timestamp();
    game.stage = Stage::Started(time_start);

    let request = [
        "VaraMan".encode(),
        "FinishTournament".to_string().encode(),
        (player, time_start).encode(),
    ]
    .concat();
    msg::send_bytes_with_gas_delayed(
        exec::program_id(),
        request,
        storage.config.gas_for_finish_tournament,
        0,
        game.duration_ms / 3_000 + 1,
    )
    .expect("Error in sending message");
    Ok(Event::GameStarted)
}

pub fn finish_tournament(
    storage: &mut GameStorage,
    admin_id: ActorId,
    time_start: u64,
) -> Result<Event, GameError> {
    if msg::source() != exec::program_id() {
        return Err(GameError::AccessDenied);
    }
    let game = storage
        .tournaments
        .get_mut(&admin_id)
        .ok_or(GameError::NoSuchGame)?;

    if game.stage != Stage::Started(time_start) {
        return Err(GameError::WrongStage);
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
    if prize != 0 {
        winners.iter().for_each(|id| {
            msg::send_with_gas(*id, "", 0, prize).expect("Error in sending value");
        });
    }
    game.stage = Stage::Finished(winners.clone());
    
    let participants: Vec<(ActorId, Player)> = game.participants.clone().into_iter().collect();

    Ok(Event::GameFinished {
        winners,
        participants,
        prize,
    })
}

pub fn record_tournament_result(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    time: u128,
    gold_coins: u16,
    silver_coins: u16,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    if gold_coins > storage.config.max_number_gold_coins
        || silver_coins > storage.config.max_number_silver_coins
    {
        return Err(GameError::ExceededLimit);
    }
    let msg_src = msg::source();
    let player = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::RecordTournamentResult,
    );
    let admin_id = storage
        .players_to_game_id
        .get(&player)
        .ok_or(GameError::NoSuchPlayer)?;
    let game = storage
        .tournaments
        .get_mut(admin_id)
        .ok_or(GameError::NoSuchGame)?;

    if !matches!(game.stage, Stage::Started(_)) {
        return Err(GameError::WrongStage);
    }

    let player = game
        .participants
        .get_mut(&player)
        .ok_or(GameError::NoSuchPlayer)?;

    let (points_for_gold, points_for_silver) = storage
        .config
        .get_points_per_gold_coin_for_level(game.level);
    let points = points_for_gold * gold_coins as u128 + points_for_silver * silver_coins as u128;
    let maximum_possible_points = points_for_gold * storage.config.max_number_gold_coins as u128
        + points_for_silver * storage.config.max_number_silver_coins as u128;
    player.time += time;
    player.points += points;

    Ok(Event::ResultTournamentRecorded {
        gold_coins,
        silver_coins,
        time: player.time,
        points: player.points,
        maximum_possible_points,
        maximum_number_gold_coins: storage.config.max_number_gold_coins,
        maximum_number_silver_coins: storage.config.max_number_silver_coins,
    })
}

pub fn leave_game(
    storage: &mut GameStorage,
    sessions: &HashMap<ActorId, SessionData>,
    session_for_account: Option<ActorId>,
) -> Result<Event, GameError> {
    let player = get_player(
        sessions,
        &msg::source(),
        &session_for_account,
        ActionsForSession::LeaveGame,
    );
    storage.players_to_game_id.remove(&player);
    Ok(Event::LeftGame)
}

pub fn change_status(storage: &mut GameStorage, status: Status) -> Result<Event, GameError> {
    if storage.admins.contains(&msg::source()) {
        storage.status = status;
        Ok(Event::StatusChanged(status))
    } else {
        Err(GameError::NotAdmin)
    }
}

pub fn change_config(storage: &mut GameStorage, config: Config) -> Result<Event, GameError> {
    if storage.admins.contains(&msg::source()) {
        storage.config = config;
        Ok(Event::ConfigChanged(config))
    } else {
        Err(GameError::NotAdmin)
    }
}

pub fn add_admin(storage: &mut GameStorage, new_admin_id: ActorId) -> Result<Event, GameError> {
    if storage.admins.contains(&msg::source()) {
        storage.admins.push(new_admin_id);
        Ok(Event::AdminAdded(new_admin_id))
    } else {
        Err(GameError::NotAdmin)
    }
}

fn get_player(
    session_map: &HashMap<ActorId, SessionData>,
    msg_source: &ActorId,
    session_for_account: &Option<ActorId>,
    actions_for_session: ActionsForSession,
) -> ActorId {
    let player = match session_for_account {
        Some(account) => {
            let session = session_map
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
