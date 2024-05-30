use super::{
    utils::{Result, Status, *},
    Event,
};
use crate::single::Entity;
use gstd::{exec, msg, prelude::*, ActorId};

pub fn create_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
) -> Result<ActorId> {
    if let Some(game_id) = game_pair.get(&player) {
        let old_game = games.get_mut(game_id).ok_or(Error::NoSuchGame)?;
        if !matches!(old_game.status, Status::GameOver(_)) {
            return Err(Error::SeveralGames);
        }
    }
    let game_instance = MultipleGame {
        first_player_board: (player, vec![Entity::Unknown; 25]),
        second_player_board: None,
        participants: (player, ActorId::zero()),
        start_time: None,
        status: Status::Registration,
        end_time: None,
    };
    games.insert(player, game_instance);
    game_pair.insert(player, player);
    Ok(player)
}

pub fn cancel_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
) -> Result<ActorId> {
    let game = games.get(&player).ok_or(Error::NoSuchGame)?;
    if !game.participants.1.is_zero() {
        game_pair.remove(&game.participants.1);
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
    if let Some(game_id) = game_pair.get(&player) {
        let old_game = games.get_mut(game_id).ok_or(Error::NoSuchGame)?;
        if !matches!(old_game.status, Status::GameOver(_)) {
            return Err(Error::SeveralGames);
        }
    }

    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;
    if game.status != Status::Registration {
        return Err(Error::WrongStatus);
    }
    game.second_player_board = Some((player, vec![Entity::Unknown; 25]));

    game.participants.1 = player;
    game_pair.insert(player, game_id);
    game.status = Status::VerificationPlacement(None);
    Ok(player)
}

pub fn leave_game(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
) -> Result<Event> {
    let game_id = game_pair.get(&player).ok_or(Error::NoSuchGame)?;

    if *game_id == player {
        return Err(Error::AccessDenied);
    }

    let game = games.get_mut(game_id).ok_or(Error::NoSuchGame)?;

    let event = match game.status {
        Status::VerificationPlacement(_) => {
            game.second_player_board = None;
            game.participants.1 = ActorId::zero();
            game.status = Status::Registration;
            Event::GameLeft { game_id: *game_id }
        }
        Status::Turn(_) | Status::PendingVerificationOfTheMove(_) => {
            game.status = Status::GameOver(*game_id);
            Event::EndGame { winner: *game_id }
        }
        Status::GameOver(_) => Event::GameLeft { game_id: *game_id },
        Status::Registration => {
            return Err(Error::WrongStatus);
        }
    };

    // delete player from game pair
    game_pair.remove(&player);

    Ok(event)
}

pub fn set_verify_placement(
    games: &mut MultipleGamesMap,
    player: ActorId,
    game_id: ActorId,
    block_timestamp: u64,
) -> Result<Event> {
    let game = games.get_mut(&game_id).ok_or(Error::NoSuchGame)?;

    match &game.status {
        Status::VerificationPlacement(None) => {
            game.status = Status::VerificationPlacement(Some(player));
            Ok(Event::PlacementVerified)
        }
        Status::VerificationPlacement(Some(verified_player)) if verified_player != &player => {
            game.status = Status::Turn(*verified_player);
            game.start_time = Some(block_timestamp);
            Ok(Event::PlacementVerified)
        }
        Status::VerificationPlacement(Some(_)) => Err(Error::AlreadyVerified),
        _ => Err(Error::WrongStatus),
    }
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
    if game.status == Status::Turn(player) {
        return Err(Error::WrongStatus);
    }

    let opponent = game.get_opponent(&player);
    msg::send(opponent, Event::MoveMade { step }, 0).expect("Error send message");
    game.status = Status::PendingVerificationOfTheMove((opponent, step));
    Ok(Event::MoveMade { step })
}

pub fn check_game_for_verify_placement(
    games: &MultipleGamesMap,
    player: ActorId,
    game_id: ActorId,
) -> Result<()> {
    let game = games.get(&game_id).ok_or(Error::NoSuchGame)?;

    if game.participants.0 != player && game.participants.1 != player {
        return Err(Error::NotPlayer);
    }

    if !matches!(game.status, Status::VerificationPlacement(_)) {
        return Err(Error::WrongStatus);
    }

    Ok(())
}

pub fn check_game_for_verify_move(
    games: &MultipleGamesMap,
    game_id: ActorId,
    player: ActorId,
    hit: u8,
) -> Result<()> {
    let game = games.get(&game_id).ok_or(Error::NoSuchGame)?;

    if game.status != Status::PendingVerificationOfTheMove((player, hit)) {
        return Err(Error::WrongStatus);
    }

    Ok(())
}

pub fn verified_move(
    games: &mut MultipleGamesMap,
    game_pair: &mut GamePairsMap,
    player: ActorId,
    res: u8,
    hit: u8,
) -> Result<Event> {
    let game = games.get_mut(&player).ok_or(Error::NoSuchGame)?;
    game.shot(&player, hit, res);

    if game.check_end_game(&player) {
        let winner = game.get_opponent(&player);
        game.status = Status::GameOver(winner);
        game.end_time = Some(exec::block_timestamp());
        game_pair.remove(&player);
        game_pair.remove(&winner);
        return Ok(Event::EndGame { winner });
    }
    game.status = Status::Turn(player);

    Ok(Event::MoveVerified {
        step: hit,
        result: res,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::multiple::funcs;
    use utils::*;

    #[test]
    fn create_game() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating game_pair_map.
        let alice = alice();
        let mut game_pair_map = game_pair_map([]);
        assert!(game_pair_map.is_empty());
        let mut multiple_game_map = multiple_game_map([]);
        assert!(multiple_game_map.is_empty());

        // # Test case #1.
        // Ok: Create game
        {
            funcs::create_game(&mut multiple_game_map, &mut game_pair_map, alice).unwrap();
            assert_eq!(*game_pair_map.get(&alice).unwrap(), alice);
            assert_eq!(
                *multiple_game_map.get(&alice).unwrap(),
                multiple_game(alice)
            );
        }
        // # Test case #2.
        // Error: Several games
        {
            let res = funcs::create_game(&mut multiple_game_map, &mut game_pair_map, alice);
            assert!(res.is_err_and(|err| err == Error::SeveralGames));
        }
    }

    #[test]
    fn cancel_game() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating game_pair_map.
        let alice = alice();
        let bob = bob();
        let mut game_pair_map = game_pair_map([(alice, alice), (bob, alice)]);
        assert!(!game_pair_map.is_empty());
        let mut game = multiple_game(alice);
        game.second_player_board = Some((alice, vec![Entity::Unknown; 25]));
        game.participants.1 = bob;
        let mut multiple_game_map = multiple_game_map([(alice, game)]);
        assert!(!multiple_game_map.is_empty());

        // # Test case #1.
        // Ok: Cancel game
        {
            funcs::cancel_game(&mut multiple_game_map, &mut game_pair_map, alice).unwrap();
            assert!(game_pair_map.is_empty());
            assert!(multiple_game_map.is_empty());
        }
        // # Test case #2.
        // Error: No such game
        {
            let res = funcs::cancel_game(&mut multiple_game_map, &mut game_pair_map, alice);
            assert!(res.is_err_and(|err| err == Error::NoSuchGame));
        }
    }

    #[test]
    fn join_game() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating game_pair_map.
        let alice = alice();
        let bob = bob();
        let mut pair_map = game_pair_map([(alice, alice), (bob, bob)]);
        assert!(!pair_map.is_empty());
        let mut game = multiple_game(alice);
        let mut game_map = multiple_game_map([(alice, game.clone()), (bob, game.clone())]);
        assert!(!game_map.is_empty());

        // # Test case #1.
        // Error: several games
        {
            let res = funcs::join_game(&mut game_map, &mut pair_map, bob, alice);
            assert!(res.is_err_and(|err| err == Error::SeveralGames));
        }

        // # Test case #2.
        // Ok: join to game
        let mut pair_map = game_pair_map([(alice, alice)]);
        let mut game_map = multiple_game_map([(alice, game.clone())]);
        {
            funcs::join_game(&mut game_map, &mut pair_map, bob, alice).unwrap();
            game.second_player_board = Some((bob, vec![Entity::Unknown; 25]));
            game.participants.1 = bob;
            game.status = Status::VerificationPlacement(None);
            assert_eq!(*pair_map.get(&bob).unwrap(), alice);
            assert_eq!(*game_map.get(&alice).unwrap(), game);
        }
        // # Test case #3.
        // Error: there's already a player registered
        let john = john();
        {
            let res = funcs::join_game(&mut game_map, &mut pair_map, john, alice);
            assert!(res.is_err_and(|err| err == Error::WrongStatus));
        }
    }

    #[test]
    fn leave_game() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating game_pair_map.
        let alice = alice();
        let bob = bob();
        let mut pair_map = game_pair_map([(alice, alice), (bob, alice)]);
        assert!(!pair_map.is_empty());
        let mut game = multiple_game(alice);
        game.second_player_board = Some((alice, vec![Entity::Unknown; 25]));
        game.participants.1 = bob;
        let mut game_map = multiple_game_map([(alice, game.clone())]);
        assert!(!game_map.is_empty());

        // # Test case #1.
        // Error: wrong status
        {
            let res = funcs::leave_game(&mut game_map, &mut pair_map, bob);
            assert!(res.is_err_and(|err| err == Error::WrongStatus));
        }

        // # Test case #2.
        // Ok: leave game when status is verification placement
        game.status = Status::VerificationPlacement(None);
        let mut game_map = multiple_game_map([(alice, game.clone())]);
        {
            funcs::leave_game(&mut game_map, &mut pair_map, bob).unwrap();
            game.second_player_board = None;
            game.participants.1 = ActorId::zero();
            game.status = Status::Registration;
            assert_eq!(pair_map.len(), 1);
            assert_eq!(*game_map.get(&alice).unwrap(), game);
        }
        // # Test case #3.
        // Ok: leave game when status is turn
        game.status = Status::Turn(alice);
        let mut game_map = multiple_game_map([(alice, game.clone())]);
        let mut pair_map = game_pair_map([(alice, alice), (bob, alice)]);
        {
            let event = funcs::leave_game(&mut game_map, &mut pair_map, bob).unwrap();
            assert_eq!(event, Event::EndGame { winner: alice });
            assert_eq!(pair_map.len(), 1);
        }
        // # Test case #4.
        // Ok: leave game when status is game over
        game.status = Status::GameOver(alice);
        let mut game_map = multiple_game_map([(alice, game.clone())]);
        let mut pair_map = game_pair_map([(alice, alice), (bob, alice)]);
        {
            let event = funcs::leave_game(&mut game_map, &mut pair_map, bob).unwrap();
            assert_eq!(event, Event::GameLeft { game_id: alice });
            assert_eq!(pair_map.len(), 1);
        }
    }

    #[test]
    fn set_verify_placement() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating game_pair_map.
        let alice = alice();
        let bob = bob();
        let mut game = multiple_game(alice);
        let mut game_map = multiple_game_map([(alice, game.clone())]);
        assert!(!game_map.is_empty());

        // # Test case #1.
        // Error: wrong status
        {
            let res = funcs::set_verify_placement(&mut game_map, alice, alice, 0);

            assert!(res.is_err_and(|err| err == Error::WrongStatus));
        }
        // # Test case #2.
        // Ok: status VerificationPlacement(None)
        game.status = Status::VerificationPlacement(None);
        let mut game_map = multiple_game_map([(alice, game.clone())]);
        {
            funcs::set_verify_placement(&mut game_map, alice, alice, 0).unwrap();
            game.status = Status::VerificationPlacement(Some(alice));
            assert_eq!(*game_map.get(&alice).unwrap(), game);
        }
        // # Test case #3.
        // Error: a case where Alice wants to double verify ships
        {
            let res = funcs::set_verify_placement(&mut game_map, alice, alice, 0);
            assert!(res.is_err_and(|err| err == Error::AlreadyVerified));
        }
        // # Test case #4.
        // Ok: status VerificationPlacement(Some(alice))
        {
            funcs::set_verify_placement(&mut game_map, bob, alice, 0).unwrap();
            game.status = Status::Turn(alice);
            game.start_time = Some(0);
            assert_eq!(*game_map.get(&alice).unwrap(), game);
        }
    }

    mod utils {
        use super::{GamePairsMap, MultipleGame, MultipleGamesMap};
        use crate::multiple::Status;
        use crate::single::Entity;
        use gstd::{vec, ActorId};

        pub fn game_pair_map<const N: usize>(content: [(ActorId, ActorId); N]) -> GamePairsMap {
            content.into_iter().collect()
        }
        pub fn multiple_game_map<const N: usize>(
            content: [(ActorId, MultipleGame); N],
        ) -> MultipleGamesMap {
            content.into_iter().collect()
        }
        pub fn multiple_game(player: ActorId) -> MultipleGame {
            MultipleGame {
                first_player_board: (player, vec![Entity::Unknown; 25]),
                second_player_board: None,
                participants: (player, ActorId::zero()),
                start_time: None,
                end_time: None,
                status: Status::Registration,
            }
        }

        pub fn alice() -> ActorId {
            1u64.into()
        }
        pub fn bob() -> ActorId {
            2u64.into()
        }
        pub fn john() -> ActorId {
            3u64.into()
        }
    }
}
