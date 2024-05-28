use super::{
    utils::{Result, *},
    Event,
};
use gstd::{exec, prelude::*, ActorId};

static mut SEED: u8 = 0;

pub fn create_session(
    session_map: &mut SessionMap,
    source: ActorId,
    block_timestamp: u64,
    key: ActorId,
    duration: u64,
    allowed_actions: Vec<ActionsForSession>,
) -> Result<bool> {
    if allowed_actions.is_empty() {
        return Err(Error::AllowedActionsIsEmpty);
    }
    if let Some(Session {
        key: _,
        expires,
        allowed_actions: _,
    }) = session_map.get(&source)
    {
        if *expires > block_timestamp {
            return Err(Error::AlreadyHaveActiveSession);
        }
    }
    session_map.entry(source).or_insert_with(|| Session {
        key,
        expires: block_timestamp + duration,
        allowed_actions,
    });
    Ok(true)
}

pub fn delete_session(session_map: &mut SessionMap, source: ActorId) -> Result<()> {
    if session_map.remove(&source).is_none() {
        return Err(Error::NoActiveSession);
    }
    Ok(())
}

pub fn start_single_game(games: &mut SingleGamesMap, player: ActorId) -> Result<()> {
    if let Some(game) = games.get(&player) {
        if game.result.is_none() {
            return Err(Error::SeveralGames);
        }
    }

    let bot_ships = generate_field();

    let game_instance = SingleGame {
        player_board: vec![Entity::Unknown; 25],
        bot_ships,
        status: Status::PendingMove,
        start_time: exec::block_timestamp(),
        end_time: None,
        result: None,
        total_shots: 0,
    };
    games.insert(player, game_instance);
    Ok(())
}

pub fn make_move(games: &mut SingleGamesMap, player: ActorId, step: u8) -> Result<Event> {
    let game = games.get_mut(&player).ok_or(Error::NoSuchGame)?;

    if matches!(game.status, Status::PendingVerificationOfTheMove(_)) {
        return Err(Error::StatusIsPendingVerification);
    }

    if game.result.is_some() {
        return Err(Error::GameIsAlreadyOver);
    }

    if step > 24 {
        return Err(Error::WrongStep);
    }

    let step_result = game.bot_ships.bang(step);
    game.total_shots += 1;

    if game.bot_ships.check_end_game() {
        game.result = Some(BattleshipParticipants::Player);
        game.end_time = Some(exec::block_timestamp());
        return Ok(Event::EndGame(BattleshipParticipants::Player));
    }
    let bot_step = move_analysis(&game.player_board);
    game.status = Status::PendingVerificationOfTheMove(bot_step);

    Ok(Event::MoveMade {
        step_result,
        bot_step,
    })
}

pub fn check_game(games: &SingleGamesMap, player: ActorId, hit: u8) -> Result<()> {
    let game = games.get(&player).ok_or(Error::NoSuchGame)?;

    if game.status != Status::PendingVerificationOfTheMove(hit) {
        // TODO: UNCOMMENT AFTER TESTING!!!!!
        // return Err(Error::WrongStatusOrHit);
    }

    if game.result.is_some() {
        return Err(Error::GameIsAlreadyOver);
    }
    Ok(())
}

pub fn verified_move(
    games: &mut SingleGamesMap,
    player: ActorId,
    res: u8,
    hit: u8,
) -> Result<Event> {
    let game = games.get_mut(&player).ok_or(Error::NoSuchGame)?;

    match res {
        0 => game.player_board[hit as usize] = Entity::Boom,
        1 => game.player_board[hit as usize] = Entity::BoomShip,
        _ => unimplemented!(),
    }

    if game.check_end_game() {
        game.result = Some(BattleshipParticipants::Bot);
        game.end_time = Some(exec::block_timestamp());
        return Ok(Event::EndGame(BattleshipParticipants::Bot));
    }
    game.status = Status::PendingMove;

    Ok(Event::MoveVerified {
        step: hit,
        result: res,
    })
}

pub fn get_player(
    session_map: &SessionMap,
    source: ActorId,
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
                session.key, source,
                "The account is not approved for this session"
            );
            *account
        }
        None => source,
    };
    player
}

fn generate_field() -> Ships {
    // let board = vec![Entity::Empty; 25];
    let mut ships = vec![];
    // Each ship is randomized and it can happen that at some point there is no room for a ship on the field,
    // so you need this cycle
    'mark: loop {
        let mut board = vec![Entity::Empty; 25];
        let ship_sizes = [3, 2, 2, 1];
        for &size in &ship_sizes {
            if check_empty_field(&board, size) {
                ships.push(place_ship(&mut board, size));
            } else {
                ships = vec![];
                continue 'mark;
            }
        }
        break;
    }
    Ships {
        ship_1: ships[0].clone(),
        ship_2: ships[1].clone(),
        ship_3: ships[2].clone(),
        ship_4: ships[3].clone(),
    }
}

fn check_empty_field(board: &[Entity], size: usize) -> bool {
    let empty_count = board.iter().filter(|&entity| entity.is_empty()).count();
    empty_count >= size
}

fn place_ship(board: &mut [Entity], size: usize) -> Vec<u8> {
    let mut placed = false;
    let mut ship: Vec<u8> = vec![];
    // the ship can be positioned in four positions,
    // the direction is chosen randomly and a check is made
    while !placed {
        let empty_indices = get_empty_indices(board);
        let random_index = get_random_value(empty_indices.len() as u8);
        let position = empty_indices[random_index as usize];
        let directions = [-1, -5, 1, 5]; // 0-left 1-up 2-right 3-down
        let random_direction = get_random_value(3);
        let direction = directions[random_direction as usize];
        if can_place_ship(board, position as isize, direction, size) {
            for i in 0..size {
                let target_position = (position as isize + direction * i as isize) as usize;
                ship.push(target_position as u8);
                board[target_position] = Entity::Ship;
                occupy_cells(board, target_position);
            }
            placed = true;
        }
    }
    ship
}

fn get_empty_indices(board: &[Entity]) -> Vec<usize> {
    board
        .iter()
        .enumerate()
        .filter_map(|(index, entity)| if entity.is_empty() { Some(index) } else { None })
        .collect()
}

pub fn get_random_value(range: u8) -> u8 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let mut random_input: [u8; 32] = exec::program_id().into();
    random_input[0] = random_input[0].wrapping_add(seed);
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}

fn can_place_ship(board: &[Entity], position: isize, direction: isize, size: usize) -> bool {
    if size == 3 {
        let is_valid = match direction {
            -1 if position % 5 == 0 || (position - 1) % 5 == 0 => false,
            -5 if (position + direction) < 0 || (position + 2 * direction) < 0 => false,
            1 if (position + 1) % 5 == 0 || (position + 2) % 5 == 0 => false,
            5 if (position + direction) > 24 || (position + 2 * direction) > 24 => false,
            _ => true,
        };

        if is_valid {
            return check_cells(board, position + direction)
                && check_cells(board, position + 2 * direction);
        }
        return false;
    }

    if size == 2 {
        let is_valid = match direction {
            -1 if position % 5 == 0 => false,
            -5 if (position + direction) < 0 => false,
            1 if (position + 1) % 5 == 0 => false,
            5 if (position + direction) > 24 => false,
            _ => true,
        };

        if is_valid {
            return check_cells(board, position + direction);
        }
        return false;
    }

    true
}

fn occupy_cells(board: &mut [Entity], position: usize) {
    let cells = match position {
        0 => vec![1, 5, 6],
        4 => vec![-1, 4, 5],
        20 => vec![1, -4, -5],
        24 => vec![-1, -5, -6],
        p if p % 5 == 0 => vec![-4, -5, 1, 5, 6],
        p if (p + 1) % 5 == 0 => vec![-1, -5, -6, 4, 5],
        _ => vec![-1, -4, -5, -6, 1, 4, 5, 6],
    };

    for &cell in &cells {
        if position as isize + cell < 0 || position as isize + cell > 24 {
            continue;
        }
        let target_position = (position as isize + cell) as usize;
        match board[target_position] {
            Entity::Empty | Entity::Unknown => {
                board[target_position] = Entity::Occupied;
            }
            _ => (),
        }
    }
}

fn check_cells(board: &[Entity], position: isize) -> bool {
    let cells = match position {
        0 => vec![1, 5, 6],
        4 => vec![-1, 4, 5],
        20 => vec![1, -4, -5],
        24 => vec![-1, -5, -6],
        p if p % 5 == 0 => vec![-4, -5, 1, 5, 6],
        p if (p + 1) % 5 == 0 => vec![-1, -5, -6, 4, 5],
        _ => vec![-1, -4, -5, -6, 1, 4, 5, 6],
    };
    for &cell in &cells {
        if position + cell < 0 || position + cell > 24 {
            continue;
        }
        let target_position = (position + cell) as usize;
        if board[target_position] == Entity::Ship {
            return false;
        }
    }

    true
}

fn move_analysis(board: &[Entity]) -> u8 {
    // Firstly, if we hit a ship, we have to finish it off and kill it
    for (index, status) in board.iter().enumerate() {
        if *status == Entity::BoomShip {
            return bang(board, index as u8);
        }
    }
    // If there are no wounded ships, randomly select a free cell
    let mut possible_bang: Vec<u8> = vec![];
    for (index, status) in board.iter().enumerate() {
        if *status == Entity::Unknown {
            possible_bang.push(index as u8)
        }
    }
    let random_index = get_random_value(possible_bang.len() as u8);
    possible_bang[random_index as usize]
}

fn bang(board: &[Entity], position: u8) -> u8 {
    let directions: Vec<i8> = match position {
        0 => vec![1, 5],
        4 => vec![-1, 5],
        20 => vec![-5, 1],
        24 => vec![-5, -1],
        p if p % 5 == 0 => vec![-5, 1, 5],
        p if (p + 1) % 5 == 0 => vec![-5, -1, 5],
        _ => vec![-5, -1, 1, 5],
    };

    let mut possible_bang: Vec<u8> = vec![];
    for &direction in &directions {
        let current_position = position as i8 + direction;
        if !(0..=24).contains(&current_position) {
            continue;
        }
        if board[current_position as usize] == Entity::BoomShip {
            if check_bang(current_position as u8, direction)
                && board[(current_position + direction) as usize] == Entity::Unknown
            {
                possible_bang.push((current_position + direction) as u8);
            }
            if check_bang(position, -direction)
                && board[(position as i8 - direction) as usize] == Entity::Unknown
            {
                possible_bang.push((position as i8 - direction) as u8);
            }
        }
    }
    if possible_bang.is_empty() {
        for &direction in &directions {
            let current_position = position as i8 + direction;
            if !(0..=24).contains(&current_position) {
                continue;
            }
            if board[current_position as usize] == Entity::Unknown {
                possible_bang.push(current_position as u8);
            }
        }
    }
    let random_index = get_random_value(possible_bang.len() as u8);
    possible_bang[random_index as usize]
}

fn check_bang(position: u8, direction: i8) -> bool {
    let check_cell = position as i8 + direction;
    if !(0..=24).contains(&check_cell) {
        return false;
    }
    match direction {
        -1 if position % 5 == 0 => return false,
        1 if (position + 1) % 5 == 0 => return false,
        _ => (),
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::single::funcs;
    use crate::single::utils::Session;
    use utils::*;
    #[test]
    fn create_session() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating empty sessions map.
        let mut sessions_map = sessions_map([]);
        assert!(sessions_map.is_empty());

        let source = alice();
        let key: ActorId = 1.into();
        let duration = 100;
        let allowed_actions = vec![ActionsForSession::StartSingleGame];

        // # Test case #1.
        // Ok: Create session
        {
            funcs::create_session(
                &mut sessions_map,
                source,
                0,
                key,
                duration,
                allowed_actions.clone(),
            )
            .unwrap();
            assert_eq!(
                *sessions_map.get(&source).unwrap(),
                Session {
                    key,
                    expires: duration,
                    allowed_actions: allowed_actions.clone()
                }
            );
        }
        // # Test case #2.
        // Error: Allowed actions is empty
        {
            let res = funcs::create_session(&mut sessions_map, source, 0, key, duration, vec![]);
            assert_eq!(
                res.is_err_and(|err| err == Error::AllowedActionsIsEmpty),
                true
            );
        }

        // # Test case #3.
        // Error: Already have active session
        {
            let res = funcs::create_session(
                &mut sessions_map,
                source,
                0,
                key,
                duration,
                allowed_actions.clone(),
            );
            assert_eq!(
                res.is_err_and(|err| err == Error::AlreadyHaveActiveSession),
                true
            );
        }
    }

    #[test]
    fn delete_session() {
        // Initializing thread logger.
        let _ = env_logger::try_init();

        // Creating session map.
        let source = alice();
        let session = Session {
            key: 1.into(),
            expires: 100,
            allowed_actions: vec![ActionsForSession::StartSingleGame],
        };
        let mut sessions_map = sessions_map([(source, session)]);
        assert!(!sessions_map.is_empty());

        // # Test case #1.
        // Ok: delete session
        {
            funcs::delete_session(&mut sessions_map, source).unwrap();
            assert!(sessions_map.is_empty())
        }
        // # Test case #2.
        // Error: No active session
        {
            let res = funcs::delete_session(&mut sessions_map, source);
            assert_eq!(res.is_err_and(|err| err == Error::NoActiveSession), true);
        }
    }

    mod utils {
        use super::{Session, SessionMap};
        use gstd::ActorId;

        pub fn sessions_map<const N: usize>(content: [(ActorId, Session); N]) -> SessionMap {
            content
                .into_iter()
                .map(|(id, session)| (id, session))
                .collect()
        }

        pub fn alice() -> ActorId {
            1u64.into()
        }
    }
}
