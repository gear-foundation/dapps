use super::session::{ActionsForSession, SessionData, Storage as SessionStorage};
use crate::services::game::{
    Appearance, Battle, BattleError, BattleResult, Config, Event, Move, Pair, Player,
    PlayerSettings, State, Storage,
};
use gstd::{exec, prelude::*, ReservationId};
use sails_rs::{collections::HashMap, gstd::msg, prelude::*};

async fn check_owner(warrior_id: ActorId, msg_src: ActorId) -> Result<(), BattleError> {
    let request = [
        "Warrior".encode(),
        "GetOwner".to_string().encode(),
        ().encode(),
    ]
    .concat();

    let (_, _, owner): (String, String, ActorId) =
        msg::send_bytes_with_gas_for_reply_as(warrior_id, request, 10_000_000_000, 0, 0)
            .ok()
            .ok_or(BattleError::SendingMessageToWarrior)?
            .await
            .ok()
            .ok_or(BattleError::GetWarriorOwner)?;

    if owner != msg_src {
        return Err(BattleError::NotOwnerOfWarrior);
    }
    Ok(())
}

async fn get_appearance(warrior_id: ActorId) -> Result<Appearance, BattleError> {
    let request = [
        "Warrior".encode(),
        "GetAppearance".to_string().encode(),
        ().encode(),
    ]
    .concat();

    let (_, _, appearance): (String, String, Appearance) =
        msg::send_bytes_with_gas_for_reply_as(warrior_id, request, 5_000_000_000, 0, 0)
            .ok()
            .ok_or(BattleError::SendingMessageToWarrior)?
            .await
            .ok()
            .ok_or(BattleError::GetWarriorOwner)?;

    Ok(appearance)
}

pub async fn create_new_battle(
    storage: &mut Storage,
    warrior_id: Option<ActorId>,
    appearance: Option<Appearance>,
    battle_name: String,
    user_name: String,
    attack: u16,
    defence: u16,
    dodge: u16,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::CreateNewBattle,
    );

    let msg_value = msg::value();

    let reply = create(
        storage,
        warrior_id,
        appearance,
        user_name,
        battle_name,
        attack,
        defence,
        dodge,
        player_id,
        msg_value,
    )
    .await;
    if reply.is_err() {
        msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending the value");
    }
    reply
}

async fn create(
    storage: &mut Storage,
    warrior_id: Option<ActorId>,
    appearance: Option<Appearance>,
    user_name: String,
    battle_name: String,
    attack: u16,
    defence: u16,
    dodge: u16,
    player_id: ActorId,
    msg_value: u128,
) -> Result<Event, BattleError> {
    let time_creation = exec::block_timestamp();
    check_player_settings(attack, defence, dodge, &storage.config)?;
    let appearance = if let Some(id) = warrior_id {
        check_owner(id, player_id).await?;
        get_appearance(id).await?
    } else if let Some(app) = appearance {
        app
    } else {
        return Err(BattleError::IdAndAppearanceIsNone);
    };

    if storage.battles.contains_key(&player_id) {
        return Err(BattleError::AlreadyHaveBattle);
    }

    let mut battle = Battle::default();
    let player = Player {
        warrior_id,
        appearance,
        owner: player_id,
        user_name: user_name.clone(),
        player_settings: PlayerSettings {
            health: storage.config.health,
            attack,
            defence: defence * 10,
            dodge: dodge * 4,
        },
        number_of_victories: 0,
        ultimate_reload: 0,
        reflect_reload: 0,
    };
    battle.participants.insert(player_id, player);
    battle.bid = msg_value;
    battle.admin = player_id;
    battle.time_creation = time_creation;
    battle.battle_name = battle_name;
    storage.battles.insert(player_id, battle);
    storage.players_to_battle_id.insert(player_id, player_id);

    send_delayed_message_for_cancel_tournament(
        player_id,
        time_creation,
        storage.config.gas_to_cancel_the_battle,
        storage.config.time_to_cancel_the_battle,
    );
    Ok(Event::NewBattleCreated {
        battle_id: player_id,
        bid: msg_value,
    })
}

pub async fn battle_registration(
    storage: &mut Storage,
    admin_id: ActorId,
    warrior_id: Option<ActorId>,
    appearance: Option<Appearance>,
    user_name: String,
    attack: u16,
    defence: u16,
    dodge: u16,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::Registration,
    );
    let msg_value = msg::value();

    let reply = register(
        storage, admin_id, warrior_id, appearance, user_name, attack, defence, dodge, player_id,
        msg_value,
    )
    .await;
    if reply.is_err() {
        msg::send_with_gas(msg_src, "", 0, msg_value).expect("Error in sending the value");
    }
    reply
}

async fn register(
    storage: &mut Storage,
    admin_id: ActorId,
    warrior_id: Option<ActorId>,
    appearance: Option<Appearance>,
    user_name: String,
    attack: u16,
    defence: u16,
    dodge: u16,
    player_id: ActorId,
    msg_value: u128,
) -> Result<Event, BattleError> {
    check_player_settings(attack, defence, dodge, &storage.config)?;

    let appearance = if let Some(id) = warrior_id {
        check_owner(id, player_id).await?;
        get_appearance(id).await?
    } else if let Some(app) = appearance {
        app
    } else {
        return Err(BattleError::IdAndAppearanceIsNone);
    };

    if storage.players_to_battle_id.contains_key(&player_id) {
        return Err(BattleError::SeveralRegistrations);
    }
    let battle = storage
        .battles
        .get_mut(&admin_id)
        .ok_or(BattleError::NoSuchGame)?;

    if battle.state != State::Registration {
        return Err(BattleError::WrongState);
    }
    if battle.participants.len() >= storage.config.max_participants.into() {
        return Err(BattleError::BattleFull);
    }
    if battle.bid != msg_value {
        return Err(BattleError::WrongBid);
    }

    let reservation_id = ReservationId::reserve(
        storage.config.reservation_amount,
        storage.config.reservation_time,
    )
    .expect("Reservation across executions");

    battle.reservation.insert(player_id, reservation_id);
    battle.participants.insert(
        player_id,
        Player {
            warrior_id,
            appearance,
            owner: player_id,
            user_name: user_name.clone(),
            player_settings: PlayerSettings {
                health: storage.config.health,
                attack,
                defence: defence * 10,
                dodge: dodge * 4,
            },
            number_of_victories: 0,
            ultimate_reload: 0,
            reflect_reload: 0,
        },
    );
    storage.players_to_battle_id.insert(player_id, admin_id);
    Ok(Event::PlayerRegistered {
        admin_id,
        user_name,
        bid: msg_value,
    })
}

pub fn cancel_register(
    storage: &mut Storage,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::Registration,
    );
    let admin_id = storage
        .players_to_battle_id
        .get(&player_id)
        .ok_or(BattleError::NoSuchPlayer)?;

    let battle = storage
        .battles
        .get_mut(admin_id)
        .ok_or(BattleError::NoSuchGame)?;

    if battle.admin == player_id {
        return Err(BattleError::AccessDenied);
    }
    if battle.state != State::Registration {
        return Err(BattleError::WrongState);
    }
    let reservation_id = battle
        .reservation
        .get(&player_id)
        .ok_or(BattleError::NoSuchReservation)?;

    if battle.bid != 0 {
        msg::send_with_gas(msg_src, "", 0, battle.bid).expect("Error in sending the value");
    }
    reservation_id
        .unreserve()
        .expect("Unreservation across executions");
    battle.reservation.remove(&player_id);
    battle.participants.remove(&player_id);
    storage.players_to_battle_id.remove(&player_id);

    Ok(Event::RegisterCanceled { player_id })
}

pub fn delete_player(
    storage: &mut Storage,
    player_id_to_delete: ActorId,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::Registration,
    );
    let admin_id = storage
        .players_to_battle_id
        .get(&player_id)
        .ok_or(BattleError::NoSuchPlayer)?;

    let battle = storage
        .battles
        .get_mut(admin_id)
        .ok_or(BattleError::NoSuchGame)?;

    if battle.admin != player_id {
        return Err(BattleError::AccessDenied);
    }

    if battle.state != State::Registration {
        return Err(BattleError::WrongState);
    }

    if !battle.participants.contains_key(&player_id_to_delete) {
        return Err(BattleError::NoSuchPlayer);
    }

    let reservation_id = battle
        .reservation
        .get(&player_id_to_delete)
        .ok_or(BattleError::NoSuchReservation)?;

    if battle.bid != 0 {
        msg::send_with_gas(player_id_to_delete, "", 0, battle.bid)
            .expect("Error in sending the value");
    }
    reservation_id
        .unreserve()
        .expect("Unreservation across executions");
    battle.reservation.remove(&player_id_to_delete);
    battle.participants.remove(&player_id_to_delete);
    storage.players_to_battle_id.remove(&player_id_to_delete);

    Ok(Event::RegisterCanceled {
        player_id: player_id_to_delete,
    })
}

pub fn cancel_tournament(
    storage: &mut Storage,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::Registration,
    );
    let battle = storage
        .battles
        .get(&player_id)
        .ok_or(BattleError::NoSuchGame)?;

    let game_is_over = matches!(battle.state, State::GameIsOver { .. });

    battle.participants.iter().for_each(|(id, _)| {
        if !game_is_over && battle.bid != 0 {
            msg::send_with_gas(*id, "", 0, battle.bid).expect("Error in sending the value");
        }
        storage.players_to_battle_id.remove(id);
    });

    battle.defeated_participants.iter().for_each(|(id, _)| {
        if !game_is_over && battle.bid != 0 {
            msg::send_with_gas(*id, "", 0, battle.bid).expect("Error in sending the value");
        }
        storage.players_to_battle_id.remove(id);
    });

    battle.reservation.iter().for_each(|(_, id)| {
        let _ = id.unreserve();
    });

    storage.battles.remove(&player_id);

    Ok(Event::BattleCanceled { game_id: player_id })
}

pub fn delayed_cancel_tournament(
    storage: &mut Storage,
    game_id: ActorId,
    time_creation: u64,
) -> Result<Event, BattleError> {
    if msg::source() != exec::program_id() {
        return Err(BattleError::AccessDenied);
    }

    let battle = storage
        .battles
        .get(&game_id)
        .ok_or(BattleError::NoSuchGame)?;

    if battle.time_creation != time_creation {
        return Err(BattleError::WrongTimeCreation);
    }
    if !matches!(battle.state, State::Registration) {
        return Err(BattleError::WrongState);
    }
    battle.participants.iter().for_each(|(id, _)| {
        if battle.bid != 0 {
            msg::send_with_gas(*id, "", 0, battle.bid).expect("Error in sending the value");
        }
        storage.players_to_battle_id.remove(id);
    });

    battle.reservation.iter().for_each(|(_, id)| {
        let _ = id.unreserve();
    });

    storage.battles.remove(&game_id);

    Ok(Event::BattleCanceled { game_id })
}

pub fn start_battle(
    storage: &mut Storage,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::StartBattle,
    );

    let battle = storage
        .battles
        .get_mut(&player_id)
        .ok_or(BattleError::NoSuchGame)?;

    let reservation_id = ReservationId::reserve(
        storage.config.reservation_amount,
        storage.config.reservation_time,
    )
    .expect("Reservation across executions");

    battle.reservation.insert(player_id, reservation_id);

    match battle.state {
        State::Registration => {
            battle.check_min_player_amount()?;
            battle.split_into_pairs()?;
            battle.send_delayed_message_make_move_from_reservation(
                storage.config.time_for_move_in_blocks,
            );
            battle.state = State::Started;
        }
        _ => return Err(BattleError::WrongState),
    }
    Ok(Event::BattleStarted)
}

pub fn automatic_move(
    storage: &mut Storage,
    player_id: ActorId,
    number_of_victories: u8,
    round: u8,
) -> Result<Event, BattleError> {
    if msg::source() != exec::program_id() {
        return Err(BattleError::AccessDenied);
    }
    let game_id = storage
        .players_to_battle_id
        .get(&player_id)
        .ok_or(BattleError::NoSuchGame)?;
    let battle = storage
        .battles
        .get_mut(game_id)
        .ok_or(BattleError::NoSuchGame)?;

    battle.check_state(State::Started)?;
    let player = battle
        .participants
        .get(&player_id)
        .ok_or(BattleError::NoSuchPlayer)?;
    // check the number of victories, if equal, then the game is not over
    if player.number_of_victories == number_of_victories {
        let pair_id = battle
            .players_to_pairs
            .get(&player_id)
            .ok_or(BattleError::NoSuchPair)?;
        let pair = battle
            .pairs
            .get_mut(pair_id)
            .ok_or(BattleError::NoSuchPair)?;

        // round check
        if round == pair.round {
            if let Some(opponent_info) = pair.action {
                if opponent_info.0 == player_id {
                    send_delayed_automatic_move(
                        player_id,
                        number_of_victories,
                        pair.round,
                        storage.config.time_for_move_in_blocks,
                    );
                    return Ok(Event::AutomaticMoveMade);
                }
                let player_1_ptr = battle
                    .participants
                    .get_mut(&opponent_info.0)
                    .expect("The player must exist") as *mut _;
                let player_2_ptr = battle
                    .participants
                    .get_mut(&player_id)
                    .expect("The player must exist") as *mut _;

                let (round_result, player_1, player_2) = unsafe {
                    let player_1 = &mut *player_1_ptr;
                    let player_2 = &mut *player_2_ptr;

                    (
                        pair.recap_round((player_1, opponent_info.1), (player_2, Move::Attack)),
                        player_1,
                        player_2,
                    )
                };
                pair.action = None;
                let current_round = pair.round;
                let (player_1_health, player_2_health) = if let Some(battle_result) = round_result {
                    match battle_result {
                        BattleResult::PlayerWin(winner) => {
                            let loser = pair.get_opponent(&winner);
                            let player_loser = battle
                                .participants
                                .remove(&loser)
                                .expect("The player must exist");
                            let player_winner = battle
                                .participants
                                .get_mut(&winner)
                                .expect("The player must exist");

                            let healths = if player_1.owner == winner {
                                (player_winner.player_settings.health, 0)
                            } else {
                                (0, player_winner.player_settings.health)
                            };
                            player_winner.player_settings.health = storage.config.health;
                            player_winner.reflect_reload = 0;
                            player_winner.ultimate_reload = 0;
                            player_winner.number_of_victories += 1;
                            battle.defeated_participants.insert(loser, player_loser);
                            battle.pairs.remove(pair_id);
                            battle.players_to_pairs.remove(&winner);
                            battle.players_to_pairs.remove(&loser);
                            battle.check_end_game();
                            healths
                        }
                        BattleResult::Draw(id_1, id_2) => {
                            let player_1 = battle
                                .participants
                                .get_mut(&id_1)
                                .expect("The player must exist");
                            player_1.player_settings.health = storage.config.health;
                            player_1.reflect_reload = 0;
                            player_1.ultimate_reload = 0;
                            let player_2 = battle
                                .participants
                                .get_mut(&id_2)
                                .expect("The player must exist");

                            player_2.player_settings.health = storage.config.health;
                            player_2.reflect_reload = 0;
                            player_2.ultimate_reload = 0;
                            battle.pairs.remove(pair_id);
                            battle.players_to_pairs.remove(&id_1);
                            battle.players_to_pairs.remove(&id_2);
                            battle.check_draw_end_game();
                            (0, 0)
                        }
                    }
                } else {
                    pair.round += 1;
                    pair.round_start_time = exec::block_timestamp();
                    send_delayed_automatic_move(
                        player_id,
                        number_of_victories,
                        pair.round,
                        storage.config.time_for_move_in_blocks,
                    );
                    (
                        player_1.player_settings.health,
                        player_2.player_settings.health,
                    )
                };

                return Ok(Event::RoundAction {
                    round: current_round,
                    player_1: (opponent_info.0, opponent_info.1, player_1_health),
                    player_2: (player_id, Move::Attack, player_2_health),
                });
            } else {
                pair.action = Some((player_id, Move::Attack));
                send_delayed_automatic_move(
                    player_id,
                    number_of_victories,
                    pair.round + 1,
                    storage.config.time_for_move_in_blocks,
                );
            }
        } else {
            // if the round is different, we need to see when it started and calculate the time for the next pending message
            let delay = storage.config.time_for_move_in_blocks
                - ((exec::block_timestamp() - pair.round_start_time)
                    / storage.config.block_duration_ms as u64) as u32
                + 1;

            send_delayed_automatic_move(player_id, number_of_victories, pair.round, delay);
        }
    }

    Ok(Event::AutomaticMoveMade)
}

pub fn make_move(
    storage: &mut Storage,
    warrior_move: Move,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::MakeMove,
    );
    let game_id = storage
        .players_to_battle_id
        .get(&player_id)
        .ok_or(BattleError::NoSuchGame)?;
    let battle = storage
        .battles
        .get_mut(game_id)
        .ok_or(BattleError::NoSuchGame)?;

    battle.check_state(State::Started)?;

    let pair_id = battle
        .players_to_pairs
        .get(&player_id)
        .ok_or(BattleError::NoSuchPair)?;
    let pair = battle
        .pairs
        .get_mut(pair_id)
        .ok_or(BattleError::NoSuchPair)?;

    let timestamp = exec::block_timestamp();
    let time_for_move_ms =
        storage.config.block_duration_ms * storage.config.time_for_move_in_blocks;
    if timestamp.saturating_sub(pair.round_start_time) >= time_for_move_ms as u64 {
        return Err(BattleError::TimeExpired);
    }
    match warrior_move {
        Move::Ultimate => check_reload_ultimate(
            battle
                .participants
                .get(&player_id)
                .expect("The player must exist"),
        )?,
        Move::Reflect => check_reload_reflect(
            battle
                .participants
                .get(&player_id)
                .expect("The player must exist"),
        )?,
        Move::Attack => (),
    }

    if let Some(opponent_info) = pair.action {
        if opponent_info.0 == player_id {
            return Err(BattleError::MoveHasAlreadyBeenMade);
        }

        let player_1_ptr = battle
            .participants
            .get_mut(&opponent_info.0)
            .expect("The player must exist") as *mut _;
        let player_2_ptr = battle
            .participants
            .get_mut(&player_id)
            .expect("The player must exist") as *mut _;

        let (round_result, player_1, player_2) = unsafe {
            let player_1 = &mut *player_1_ptr;
            let player_2 = &mut *player_2_ptr;

            (
                pair.recap_round((player_1, opponent_info.1), (player_2, warrior_move)),
                player_1,
                player_2,
            )
        };
        pair.action = None;
        let current_round = pair.round;
        let (player_1_health, player_2_health) = if let Some(battle_result) = round_result {
            match battle_result {
                BattleResult::PlayerWin(winner) => {
                    let loser = pair.get_opponent(&winner);
                    let player_loser = battle
                        .participants
                        .remove(&loser)
                        .expect("The player must exist");
                    battle.defeated_participants.insert(loser, player_loser);
                    let player_winner = battle
                        .participants
                        .get_mut(&winner)
                        .expect("The player must exist");
                    let healths = if player_1.owner == winner {
                        (player_winner.player_settings.health, 0)
                    } else {
                        (0, player_winner.player_settings.health)
                    };
                    player_winner.player_settings.health = storage.config.health;
                    player_winner.reflect_reload = 0;
                    player_winner.ultimate_reload = 0;
                    player_winner.number_of_victories += 1;
                    battle.pairs.remove(pair_id);
                    battle.players_to_pairs.remove(&winner);
                    battle.players_to_pairs.remove(&loser);
                    battle.check_end_game();
                    healths
                }
                BattleResult::Draw(id_1, id_2) => {
                    let player_1 = battle
                        .participants
                        .get_mut(&id_1)
                        .expect("The player must exist");
                    player_1.player_settings.health = storage.config.health;
                    player_1.reflect_reload = 0;
                    player_1.ultimate_reload = 0;
                    let player_2 = battle
                        .participants
                        .get_mut(&id_2)
                        .expect("The player must exist");

                    player_2.player_settings.health = storage.config.health;
                    player_2.reflect_reload = 0;
                    player_2.ultimate_reload = 0;
                    battle.pairs.remove(pair_id);
                    battle.players_to_pairs.remove(&id_1);
                    battle.players_to_pairs.remove(&id_2);
                    battle.check_draw_end_game();
                    (0, 0)
                }
            }
        } else {
            pair.round += 1;
            pair.round_start_time = exec::block_timestamp();
            (
                player_1.player_settings.health,
                player_2.player_settings.health,
            )
        };
        Ok(Event::RoundAction {
            round: current_round,
            player_1: (opponent_info.0, opponent_info.1, player_1_health),
            player_2: (player_id, warrior_move, player_2_health),
        })
    } else {
        pair.action = Some((player_id, warrior_move));
        Ok(Event::MoveMade)
    }
}

pub fn start_next_fight(
    storage: &mut Storage,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::MakeMove,
    );
    let game_id = storage
        .players_to_battle_id
        .get(&player_id)
        .ok_or(BattleError::NoSuchGame)?;
    let battle = storage
        .battles
        .get_mut(game_id)
        .ok_or(BattleError::NoSuchGame)?;

    battle.check_state(State::Started)?;

    if battle.players_to_pairs.contains_key(&player_id) {
        return Err(BattleError::AlreadyHaveBattle);
    }

    let reservation_id = ReservationId::reserve(
        storage.config.reservation_amount,
        storage.config.reservation_time,
    )
    .expect("Reservation across executions");

    battle.reservation.insert(player_id, reservation_id);

    let player = battle
        .participants
        .get(&player_id)
        .ok_or(BattleError::NoSuchPlayer)?;

    if let Some((opponent, pair_id)) = battle.waiting_player {
        let pair = battle
            .pairs
            .get_mut(&pair_id)
            .expect("The pair must be created");
        pair.player_2 = player.owner;
        pair.round_start_time = exec::block_timestamp();
        battle.players_to_pairs.insert(player.owner, pair_id);
        battle.waiting_player = None;
        send_delayed_message_make_move_from_reservation(
            reservation_id,
            storage.config.time_for_move_in_blocks,
            player_id,
            player.number_of_victories,
        );

        let reservation_id = battle
            .reservation
            .get(&opponent)
            .expect("Reservation must be exist");
        let opponent_player = battle
            .participants
            .get(&opponent)
            .expect("Player must be exist");
        send_delayed_message_make_move_from_reservation(
            *reservation_id,
            storage.config.time_for_move_in_blocks,
            opponent_player.owner,
            opponent_player.number_of_victories,
        );
        Ok(Event::NextBattleStarted)
    } else {
        let pair = Pair {
            player_1: player.owner,
            round: 1,
            ..Default::default()
        };
        battle.pairs.insert(battle.pair_id, pair);
        battle.players_to_pairs.insert(player.owner, battle.pair_id);
        battle.waiting_player = Some((player.owner, battle.pair_id));
        battle.pair_id += 1;
        Ok(Event::EnemyWaiting)
    }
}

pub fn exit_game(
    storage: &mut Storage,
    session_for_account: Option<ActorId>,
) -> Result<Event, BattleError> {
    let msg_src = msg::source();

    let sessions = SessionStorage::get_session_map();
    let player_id = get_player(
        sessions,
        &msg_src,
        &session_for_account,
        ActionsForSession::Registration,
    );

    let game_id = storage
        .players_to_battle_id
        .get(&player_id)
        .ok_or(BattleError::NoSuchGame)?;
    let battle = storage
        .battles
        .get_mut(game_id)
        .ok_or(BattleError::NoSuchGame)?;

    if battle.defeated_participants.contains_key(&player_id) {
        storage.players_to_battle_id.remove(&player_id);
    } else {
        let player = battle
            .participants
            .get(&player_id)
            .expect("The player must exist");
        if let Some(pair_id) = battle.players_to_pairs.get(&player_id) {
            let pair = battle.pairs.remove(pair_id).expect("The pair must exist");

            battle.players_to_pairs.remove(&player_id);
            battle
                .defeated_participants
                .insert(player_id, player.clone());

            let opponent_id = pair.get_opponent(&player_id);
            battle.players_to_pairs.remove(&opponent_id);
            let opponent = battle
                .participants
                .get_mut(&opponent_id)
                .expect("The player must exist");

            opponent.number_of_victories += 1;
            opponent.player_settings.health = storage.config.health;

            battle.participants.remove(&player_id);
            storage.players_to_battle_id.remove(&player_id);
            battle.check_end_game();
        } else {
            if let Some((id, _)) = battle.waiting_player {
                if id == player_id {
                    battle.waiting_player = None;
                }
            }
            battle
                .defeated_participants
                .insert(player_id, player.clone());
            battle.participants.remove(&player_id);
            storage.players_to_battle_id.remove(&player_id);
        }
    }
    Ok(Event::GameLeft)
}

fn check_reload_ultimate(player: &Player) -> Result<(), BattleError> {
    if player.ultimate_reload != 0 {
        return Err(BattleError::UltimateReload);
    }
    Ok(())
}

fn check_reload_reflect(player: &Player) -> Result<(), BattleError> {
    if player.reflect_reload != 0 {
        return Err(BattleError::ReflectReload);
    }
    Ok(())
}

fn check_player_settings(
    attack: u16,
    defence: u16,
    dodge: u16,
    config: &Config,
) -> Result<(), BattleError> {
    let attack_in_range = config.attack_range.0 <= attack && attack <= config.attack_range.1;
    let defence_in_range = config.defence_range.0 <= defence && defence <= config.defence_range.1;
    let dodge_in_range = config.dodge_range.0 <= dodge && dodge <= config.dodge_range.1;

    let total_points = attack + defence + dodge
        - config.attack_range.0
        - config.defence_range.0
        - config.dodge_range.0;

    if !(attack_in_range
        && defence_in_range
        && dodge_in_range
        && total_points == config.available_points)
    {
        return Err(BattleError::MisallocationOfPoints);
    }
    Ok(())
}

fn send_delayed_message_make_move_from_reservation(
    reservation_id: ReservationId,
    time_for_move: u32,
    player_id: ActorId,
    number_of_victories: u8,
) {
    let round: u8 = 1;
    let request = [
        "Battle".encode(),
        "AutomaticMove".to_string().encode(),
        (player_id, number_of_victories, round).encode(),
    ]
    .concat();

    msg::send_bytes_delayed_from_reservation(
        reservation_id,
        exec::program_id(),
        request,
        0,
        time_for_move,
    )
    .expect("Error in sending message");
}

fn send_delayed_automatic_move(player_id: ActorId, number_of_victories: u8, round: u8, delay: u32) {
    let gas = exec::gas_available() - 5_000_000_000;
    let request = [
        "Battle".encode(),
        "AutomaticMove".to_string().encode(),
        (player_id, number_of_victories, round).encode(),
    ]
    .concat();

    msg::send_bytes_with_gas_delayed(exec::program_id(), request, gas, 0, delay)
        .expect("Error in sending message");
}

fn send_delayed_message_for_cancel_tournament(
    game_id: ActorId,
    time_creation: u64,
    gas_to_cancel_the_battle: u64,
    time_to_cancel_the_battle: u32,
) {
    let request = [
        "Battle".encode(),
        "DelayedCancelTournament".to_string().encode(),
        (Some((game_id, time_creation))).encode(),
    ]
    .concat();

    msg::send_bytes_with_gas_delayed(
        exec::program_id(),
        request,
        gas_to_cancel_the_battle,
        0,
        time_to_cancel_the_battle,
    )
    .expect("Error in sending message");
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
