use super::{
    utils::{Result, *},
    Event,
};
use crate::single::{funcs::get_player, ActionsForSession, Entity, SessionMap};
use gstd::{exec, msg, prelude::*, ActorId};

pub fn create_game(
    session_map: &mut SessionMap,
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    source: ActorId,
    session_for_account: Option<ActorId>,
) -> Result<ActorId> {
    let player = get_player(
        session_map,
        source,
        &session_for_account,
        ActionsForSession::StartMultipleGame,
    );
    if game_pair.contains_key(&player) {
        return Err(Error::SeveralGames);
    }

    let game_instance = MultipleGame {
        first_player_board: (player, vec![Entity::Unknown; 25]),
        second_player_board: None,
        participants: (player, ActorId::zero()),
        start_time: None,
        turn: player,
        end_time: None,
        result: None,
    };
    games.insert(player, game_instance);
    game_pair.insert(player, player);
    Ok(player)
}

pub fn cancel_game(
    session_map: &mut SessionMap,
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    source: ActorId,
    session_for_account: Option<ActorId>,
) -> Result<ActorId> {
    let player = get_player(
        session_map,
        source,
        &session_for_account,
        ActionsForSession::StartMultipleGame,
    );
    let game = games.get(&player).ok_or(Error::NoSuchGame)?;
    if let Some((second_player, _)) = game.second_player_board {
        game_pair.remove(&second_player);
    }
    games.remove(&player);
    game_pair.remove(&player);
    Ok(player)
}

pub fn join_game(
    session_map: &mut SessionMap,
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    source: ActorId,
    game_id: ActorId,
    session_for_account: Option<ActorId>,
) -> Result<ActorId> {
    let player = get_player(
        session_map,
        source,
        &session_for_account,
        ActionsForSession::StartMultipleGame,
    );
    if game_pair.contains_key(&player) {
        return Err(Error::SeveralGames);
    }
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    if game.start_time.is_some() {
        return Err(Error::GameAlreadyStarted);
    }
    game.second_player_board = Some((player, vec![Entity::Unknown; 25]));
    game.start_time = Some(exec::block_timestamp());
    game.participants.1 = player;
    game_pair.insert(player, game_id);
    Ok(player)
}

pub async fn make_move(
    session_map: &SessionMap,
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    source: ActorId,
    game_id: ActorId,
    step: u8,
    session_for_account: Option<ActorId>,
) -> Result<Event> {
    let player = get_player(
        session_map,
        source,
        &session_for_account,
        ActionsForSession::StartSingleGame,
    );
    if step > 24 {
        return Err(Error::WrongStep);
    }

    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    if game.start_time.is_none() {
        return Err(Error::MissingSecondPlayer);
    }
    if game.turn != player {
        return Err(Error::AccessDenied);
    }
    if game.result.is_some() {
        return Err(Error::GameIsAlreadyOver);
    }
    let opponent = game.get_opponent(&player);

    let reply: Сonfirmation = msg::send_for_reply_as(opponent, Shot { step }, 0, 0)
        .expect("Unable to send message to `opponent`")
        .await
        .expect("Unable to decode reply payload from `opponent`");

    game.shot(&opponent, step, &reply.step_result);

    if game.check_end_game(&opponent) {
        game.result = Some(player);
        game.end_time = Some(exec::block_timestamp());
        game_pair.remove(&player);
        game_pair.remove(&opponent);
        return Ok(Event::EndGame { winner: player });
    }
    Ok(Event::MoveMade {
        step_result: reply.step_result,
    })
}
