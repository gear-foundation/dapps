use crate::services::game::game_actions::{GameSessionActions, COST_FOR_UPGRADE, FINE};
use crate::services::game::*;
use sails_rs::ActorId;

pub fn create_game_session(
    storage: &mut Storage,
    entry_fee: Option<u128>,
    name: &str,
    strategy_id: &ActorId,
) -> Result<Event, GameError> {
    if let Some(fee) = entry_fee {
        if fee < exec::env_vars().existential_deposit {
            return Err(GameError::FeeIsLessThanED);
        }
    }

    let admin_id = msg::source();
    if storage.game_sessions.contains_key(&admin_id) {
        return Err(GameError::GameSessionAlreadyExists);
    }

    let mut game = Game {
        admin_id,
        entry_fee,
        ..Default::default()
    };
    game.init_properties();
    game.register(
        &admin_id,
        strategy_id,
        name,
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    storage.game_sessions.insert(admin_id, game);
    storage.players_to_sessions.insert(admin_id, admin_id);
    Ok(Event::GameSessionCreated { admin_id })
}

pub fn make_reservation(storage: &mut Storage, admin_id: ActorId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.make_reservation(
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    Ok(Event::ReservationMade)
}

pub fn register(
    storage: &mut Storage,
    admin_id: AdminId,
    strategy_id: ActorId,
    name: String,
) -> Result<Event, GameError> {
    let player_id = msg::source();

    if storage.game_sessions.contains_key(&player_id)
        || storage.players_to_sessions.contains_key(&player_id)
    {
        return Err(GameError::AccountAlreadyRegistered);
    }

    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.register(
        &player_id,
        &strategy_id,
        &name,
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    storage.players_to_sessions.insert(player_id, admin_id);
    Ok(Event::StrategyRegistered)
}

pub fn play(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.play(
        storage.config.min_gas_limit,
        storage.config.time_for_step,
        storage.config.gas_refill_timeout,
    )
}

pub fn add_gas_to_player_strategy(
    storage: &mut Storage,
    admin_id: AdminId,
) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.add_gas_to_player_strategy(
        storage.config.reservation_amount,
        storage.config.reservation_duration_in_block,
    )?;
    game.game_status = GameStatus::Play;
    Ok(Event::GasForPlayerStrategyAdded)
}

pub fn cancel_game_session(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.cancel_game_session()?;
    for player_id in game.owners_to_strategy_ids.keys() {
        storage.players_to_sessions.remove(player_id);
    }
    storage.game_sessions.remove(&admin_id);
    Ok(Event::GameWasCancelled)
}

pub fn exit_game(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get_mut(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.exit_game()?;
    storage.players_to_sessions.remove(&msg::source());
    Ok(Event::PlayerLeftGame)
}

pub fn delete_game(storage: &mut Storage, admin_id: AdminId) -> Result<Event, GameError> {
    let game = storage
        .game_sessions
        .get(&admin_id)
        .ok_or(GameError::GameDoesNotExist)?;
    game.check_status(GameStatus::Finished)?;
    for player_id in game.owners_to_strategy_ids.keys() {
        storage.players_to_sessions.remove(player_id);
    }
    storage.game_sessions.remove(&admin_id);
    Ok(Event::GameDeleted)
}

pub fn delete_player(storage: &mut Storage, player_id: AdminId) -> Result<Event, GameError> {
    let admin_id = storage
        .players_to_sessions
        .get(&player_id)
        .ok_or(GameError::AccountAlreadyRegistered)?;

    if *admin_id != msg::source() {
        return Err(GameError::OnlyAdmin);
    }

    let game = storage
        .game_sessions
        .get_mut(admin_id)
        .ok_or(GameError::GameDoesNotExist)?;

    game.delete_player(&player_id)?;

    storage.players_to_sessions.remove(&player_id);

    Ok(Event::PlayerDeleted)
}

pub fn throw_roll(game: &mut Game, pay_fine: bool, properties_for_sale: Option<Vec<u8>>) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    // If a player is not in the jail
    if !player_info.in_jail {
        player_info.penalty += 1;
        return;
    }

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            player_info.penalty += 1;
            return;
        };
    }

    let (r1, r2) = get_rolls();
    if r1 == r2 {
        player_info.in_jail = false;
        player_info.position = r1 + r2;
    } else if pay_fine {
        if player_info.balance < FINE {
            player_info.penalty += 1;
            return;
        }
        player_info.balance -= FINE;
        player_info.in_jail = false;
    }
    player_info.round = game.round;
}

pub fn add_gear(game: &mut Game, properties_for_sale: Option<Vec<u8>>) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            return;
        };
    }

    // if player did not check his balance itself
    if player_info.balance < COST_FOR_UPGRADE {
        player_info.penalty += 1;
        return;
    }

    let position = player_info.position;

    let gears = if let Some((account, gears, _, _)) = &mut game.properties[position as usize] {
        if account != &player {
            player_info.penalty += 1;
            return;
        }
        gears
    } else {
        player_info.penalty += 1;
        return;
    };

    // maximum amount of gear is reached
    if gears.len() == 3 {
        player_info.penalty += 1;
        return;
    }

    gears.push(Gear::Bronze);
    player_info.balance -= COST_FOR_UPGRADE;
    player_info.round = game.round;
}

pub fn upgrade(game: &mut Game, properties_for_sale: Option<Vec<u8>>) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            return;
        };
    }

    // if player did not check his balance itself
    if player_info.balance < COST_FOR_UPGRADE {
        player_info.penalty += 1;
        return;
    }

    let position = player_info.position;

    if let Some((account, gears, price, rent)) = &mut game.properties[position as usize] {
        if account != &player {
            player_info.penalty += 1;
            return;
        }
        // if nothing to upgrade
        if gears.is_empty() {
            player_info.penalty += 1;
            return;
        }
        for gear in gears {
            if *gear != Gear::Gold {
                *gear = gear.upgrade();
                // add 10 percent to the price of cell
                *price += *price / 10;
                // add 10 percent to the price of rent
                *rent += *rent / 10;
                break;
            }
        }
    } else {
        player_info.penalty += 1;
        return;
    };

    player_info.balance -= COST_FOR_UPGRADE;
    player_info.round = game.round;
}

pub fn buy_cell(game: &mut Game, properties_for_sale: Option<Vec<u8>>) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");

    let position = player_info.position;

    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            player_info.penalty += 1;
            return;
        };
    }

    // if a player on the field that can't be sold (for example, jail)
    if let Some((account, _, price, _)) = game.properties[position as usize].as_mut() {
        if !account.is_zero() {
            player_info.penalty += 1;
            return;
        }
        // if a player has not enough balance
        if player_info.balance < *price {
            player_info.penalty += 1;
            return;
        }
        player_info.balance -= *price;
        *account = msg::source();
    } else {
        player_info.penalty += 1;
        return;
    };
    player_info.cells.insert(position);
    game.ownership[position as usize] = player;
    player_info.round = game.round;
}

pub fn pay_rent(game: &mut Game, properties_for_sale: Option<Vec<u8>>) {
    let player = game.current_player;
    let player_info = game.players.get_mut(&player).expect("Can't be None");
    if let Some(properties) = properties_for_sale {
        if sell_property(
            &game.admin_id,
            &mut game.ownership,
            &properties,
            &mut game.properties_in_bank,
            &game.properties,
            player_info,
        )
        .is_err()
        {
            return;
        };
    }

    let position = player_info.position;
    let account = game.ownership[position as usize];

    if account == player {
        player_info.penalty += 1;
        return;
    }

    let rent = if let Some((_, _, _, rent)) = game.properties[position as usize] {
        rent
    } else {
        0
    };
    if rent == 0 {
        player_info.penalty += 1;
        return;
    };

    if player_info.balance < rent {
        player_info.penalty += 1;
        return;
    }
    player_info.balance -= rent;
    player_info.debt = 0;
    player_info.round = game.round;
    game.players
        .entry(account)
        .and_modify(|player_info| player_info.balance = player_info.balance.saturating_add(rent));
}
