#![no_std]

use gstd::{collections::HashMap, msg, prelude::*, ActorId};
use vara_man_io::{
    Config, GameInstance, Player, StateQuery, StateReply, Status, VaraMan as VaraManState,
    VaraManAction, VaraManError, VaraManEvent, VaraManInit,
};

#[derive(Debug, Default)]
struct VaraMan {
    games: HashMap<ActorId, GameInstance>,
    players: HashMap<ActorId, Player>,
    status: Status,
    config: Config,
    admins: Vec<ActorId>,
}

static mut VARA_MAN: Option<VaraMan> = None;

#[gstd::async_main]
async fn main() {
    let action: VaraManAction = msg::load().expect("Unexpected invalid action payload.");
    let vara_man: &mut VaraMan = unsafe { VARA_MAN.get_or_insert(VaraMan::default()) };

    let result = process_handle(action, vara_man).await;

    msg::reply(result, 0).expect("Unexpected invalid reply result.");
}

async fn process_handle(
    action: VaraManAction,
    vara_man: &mut VaraMan,
) -> Result<VaraManEvent, VaraManError> {
    match action {
        VaraManAction::RegisterPlayer { name } => {
            let actor_id = msg::source();

            if vara_man.status == Status::Paused {
                return Err(VaraManError::WrongStatus);
            }

            if name.is_empty() {
                return Err(VaraManError::EmptyName);
            }

            if vara_man.players.contains_key(&actor_id) {
                Err(VaraManError::AlreadyRegistered)
            } else {
                vara_man.players.insert(
                    actor_id,
                    Player {
                        name,
                        lives: vara_man.config.number_of_lives,
                        claimed_gold_coins: 0,
                        claimed_silver_coins: 0,
                    },
                );

                Ok(VaraManEvent::PlayerRegistered(actor_id))
            }
        }
        VaraManAction::StartGame { level } => {
            let player_address = msg::source();

            if vara_man.status == Status::Paused {
                return Err(VaraManError::WrongStatus);
            }

            let Some(player) = vara_man.players.get_mut(&player_address) else {
                return Err(VaraManError::NotRegistered);
            };

            if vara_man.games.get(&player_address).is_some() {
                return Err(VaraManError::AlreadyStartGame);
            };

            if !player.is_have_lives() && !vara_man.admins.contains(&player_address) {
                return Err(VaraManError::LivesEnded);
            }

            vara_man.games.insert(
                player_address,
                GameInstance {
                    level,
                    gold_coins: vara_man.config.gold_coins,
                    silver_coins: vara_man.config.silver_coins,
                },
            );

            Ok(VaraManEvent::GameStarted)
        }
        VaraManAction::ClaimReward {
            silver_coins,
            gold_coins,
        } => {
            let player_address = msg::source();

            if let Some(game) = vara_man.games.get(&player_address) {
                // Check that game is not paused
                if vara_man.status == Status::Paused {
                    return Err(VaraManError::WrongStatus);
                }

                // Check that player is registered
                let Some(player) = vara_man.players.get_mut(&player_address) else {
                    return Err(VaraManError::NotRegistered);
                };

                // Check passed coins range
                if silver_coins > game.silver_coins || gold_coins > game.gold_coins {
                    return Err(VaraManError::AmountGreaterThanAllowed);
                }

                let (tokens_per_gold_coin, tokens_per_silver_coin) = vara_man
                    .config
                    .get_tokens_per_gold_coin_for_level(game.level);

                let tokens_amount = vara_man
                    .config
                    .one_coin_in_value
                    .checked_mul(tokens_per_gold_coin)
                    .expect("Math overflow!")
                    .checked_mul(gold_coins)
                    .expect("Math overflow!")
                    .checked_add(
                        vara_man
                            .config
                            .one_coin_in_value
                            .checked_mul(tokens_per_silver_coin)
                            .expect("Math overflow!")
                            .checked_mul(silver_coins)
                            .expect("Math overflow!"),
                    )
                    .expect("Math overflow!");

                if msg::send(player_address, 0u8, tokens_amount as u128).is_err() {
                    return Err(VaraManError::TransferFailed);
                }

                player.claimed_gold_coins = player
                    .claimed_gold_coins
                    .checked_add(gold_coins)
                    .expect("Math overflow!");
                player.claimed_silver_coins = player
                    .claimed_silver_coins
                    .checked_add(silver_coins)
                    .expect("Math overflow!");

                vara_man.games.remove(&player_address);

                if !vara_man.admins.contains(&player_address) {
                    player.lives -= 1;
                }

                Ok(VaraManEvent::RewardClaimed {
                    player_address,
                    silver_coins,
                    gold_coins,
                })
            } else {
                Err(VaraManError::GameDoesNotExist)
            }
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
            } else if !config.is_valid() {
                return Err(VaraManError::ConfigIsInvalid);
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

#[no_mangle]
extern fn init() {
    let init: VaraManInit = msg::load().expect("Unexpected invalid init payload.");
    assert!(init.config.is_valid());
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
        StateQuery::AllGames => {
            let games = contract
                .games
                .into_iter()
                .map(|(id, game)| (id, game))
                .collect();
            StateReply::AllGames(games)
        }
        StateQuery::AllPlayers => {
            let players = contract
                .players
                .into_iter()
                .map(|(id, player)| (id, player))
                .collect();
            StateReply::AllPlayers(players)
        }
        StateQuery::Game { player_address } => {
            let game: Option<GameInstance> = contract.games.get(&player_address).cloned();
            StateReply::Game(game)
        }
        StateQuery::Player { player_address } => {
            let player: Option<Player> = contract.players.get(&player_address).cloned();
            StateReply::Player(player)
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
            games,
            players,
            status,
            config,
            admins,
        } = value;

        let games = games.into_iter().map(|(id, game)| (id, game)).collect();

        let players = players
            .into_iter()
            .map(|(actor_id, player)| (actor_id, player))
            .collect();

        Self {
            games,
            players,
            status,
            config,
            admins,
        }
    }
}
