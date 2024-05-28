use super::{
    utils::{Result, Status, *},
    Event,
};
use crate::single::{funcs::get_player, ActionsForSession, Entity, SessionMap};
use gstd::{exec, msg, prelude::*, ActorId};

pub fn create_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
) -> Result<ActorId> {
    if game_pair.contains_key(&player) {
        return Err(Error::SeveralGames);
    }

    let game_instance = MultipleGame {
        first_player_board: (player, vec![Entity::Unknown; 25]),
        second_player_board: None,
        participants: (player, ActorId::zero()),
        start_time: None,
        status: Status::Registration,
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
    session_for_account: Option<ActorId>,
) -> Result<ActorId> {
    let player = get_player(
        session_map,
        msg::source(),
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
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
    game_id: ActorId,
) -> Result<ActorId> {
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

pub fn make_move(
    games: &mut MultipleGamesMap,
    player: ActorId,
    game_id: ActorId,
    step: u8,
) -> Result<Event> {
    if step > 24 {
        return Err(Error::WrongStep);
    }

    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    if game.start_time.is_none() {
        return Err(Error::MissingSecondPlayer);
    }
    if game.status == Status::Turn(player) {
        return Err(Error::AccessDenied);
    }
    if game.result.is_some() {
        return Err(Error::GameIsAlreadyOver);
    }
    let opponent = game.get_opponent(&player);

    msg::send(opponent, Event::MoveMade { step }, 0).expect("Error send message");
    game.status = Status::PendingVerificationOfTheMove((opponent, step));
    Ok(Event::MoveMade { step })
}

pub fn check_game(games: &MultipleGamesMap, player: ActorId, hit: u8) -> Result<()> {
    let game = games.get(&player).ok_or(Error::NoSuchGame)?;

    if game.status != Status::PendingVerificationOfTheMove((player, hit)) {
        return Err(Error::WrongStatus);
    }

    if game.result.is_some() {
        return Err(Error::GameIsAlreadyOver);
    }
    Ok(())
}

pub fn verified_move(
    games: &mut MultipleGamesMap,
    player: ActorId,
    res: u8,
    hit: u8,
) -> Result<Event> {
    let game = games.get_mut(&player).ok_or(Error::NoSuchGame)?;
    game.shot(&player, hit, res);

    if game.check_end_game(&player) {
        let winner = game.get_opponent(&player);
        game.result = Some(winner);
        game.end_time = Some(exec::block_timestamp());
        return Ok(Event::EndGame { winner });
    }
    game.status = Status::Turn(player);

    Ok(Event::MoveVerified {
        step: hit,
        result: res,
    })
}
