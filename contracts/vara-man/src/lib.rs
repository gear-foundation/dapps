#![no_std]
use gstd::{collections::HashMap, msg, prelude::*, ActorId};
use vara_man_io::{
    Config, GameInstance, Player, StateQuery, StateReply, Status, VaraMan as VaraManState,
    VaraManAction, VaraManEvent, VaraManInit,
};

#[derive(Debug, Default)]
struct VaraMan {
    pub games: HashMap<ActorId, GameInstance>,
    pub players: HashMap<ActorId, Player>,
    pub status: Status,
    pub config: Config,
    pub admins: Vec<ActorId>,
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

        let games = games.iter().map(|(id, game)| (*id, game.clone())).collect();

        let players = players
            .iter()
            .map(|(actor_id, player)| (*actor_id, player.clone()))
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

static mut VARA_MAN: Option<VaraMan> = None;

#[gstd::async_main]
async fn main() {
    let action: VaraManAction = msg::load().expect("Unexpected invalid action payload.");
    let vara_man: &mut VaraMan = unsafe { VARA_MAN.get_or_insert(VaraMan::default()) };

    let result = process_handle(action, vara_man).await;

    msg::reply(result, 0).expect("Unexpected invalid reply result.");
}

async fn process_handle(action: VaraManAction, vara_man: &mut VaraMan) -> VaraManEvent {
    match action {
        VaraManAction::RegisterPlayer { name } => {
            let actor_id = msg::source();

            if vara_man.status == Status::Paused {
                return VaraManEvent::Error("Incorrect whole game status.".to_owned());
            }

            if name.is_empty() {
                return VaraManEvent::Error("Username is empty.".to_owned());
            }

            if vara_man.players.contains_key(&actor_id) {
                VaraManEvent::Error("Player is already registered.".to_owned())
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

                VaraManEvent::PlayerRegistered(actor_id)
            }
        }
        VaraManAction::StartGame { level, seed } => {
            let player_address = msg::source();

            if vara_man.status == Status::Paused {
                return VaraManEvent::Error("Incorrect whole game status.".to_owned());
            }

            let Some(player) = vara_man.players.get_mut(&player_address) else {
                return VaraManEvent::Error("Player must be registered to play.".to_owned());
            };

            if vara_man.games.get(&player_address).is_some() {
                return VaraManEvent::Error("Player is already StartGame".to_owned());
            };

            if !player.is_have_lives() {
                return VaraManEvent::Error("Player has exhausted all his lives.".to_owned());
            }

            vara_man.games.insert(
                player_address,
                GameInstance::new_with_coins(
                    level,
                    vara_man.config.gold_coins,
                    vara_man.config.silver_coins,
                    seed,
                ),
            );

            if !vara_man.admins.contains(&player_address) {
                player.lives -= 1;
            }

            VaraManEvent::GameStarted
        }
        VaraManAction::ClaimReward {
            silver_coins,
            gold_coins,
        } => {
            let player_address = msg::source();

            if let Some(game) = vara_man.games.get(&player_address) {
                // Check that game is not paused
                if vara_man.status == Status::Paused {
                    return VaraManEvent::Error("Incorrect whole game status.".to_owned());
                }

                // Check that player is registered
                let Some(player) = vara_man.players.get_mut(&player_address) else {
                    return VaraManEvent::Error("Player must be registered to claim.".to_owned());
                };

                // Check passed coins range
                if silver_coins > game.silver_coins || gold_coins > game.gold_coins {
                    return VaraManEvent::Error("Coin(s) amount is gt than allowed.".to_owned());
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
                    return VaraManEvent::Error("Native tokens transfer failed.".to_owned());
                }

                player.claimed_gold_coins += player
                    .claimed_gold_coins
                    .checked_add(gold_coins)
                    .expect("Math overflow!");
                player.claimed_silver_coins = player
                    .claimed_silver_coins
                    .checked_add(silver_coins)
                    .expect("Math overflow!");

                vara_man.games.remove(&player_address);

                VaraManEvent::RewardClaimed {
                    player_address,
                    silver_coins,
                    gold_coins,
                }
            } else {
                VaraManEvent::Error(
                    "The reward has already been claimed, start a new game".to_owned(),
                )
            }
        }
        VaraManAction::ChangeStatus(status) => {
            if msg::source() != vara_man.config.operator {
                VaraManEvent::Error("Only operator can change whole game status.".to_owned())
            } else {
                vara_man.status = status;
                VaraManEvent::StatusChanged(status)
            }
        }
        VaraManAction::ChangeConfig(config) => {
            if msg::source() != vara_man.config.operator {
                VaraManEvent::Error("Only operator can change whole game config.".to_owned())
            } else if !config.is_valid() {
                VaraManEvent::Error("Provided config is invalid.".to_owned())
            } else {
                vara_man.config = config;
                VaraManEvent::ConfigChanged(config)
            }
        }
        VaraManAction::AddAdmin(admin) => {
            let msg_source = msg::source();
            if vara_man.admins.contains(&msg_source) {
                vara_man.admins.push(admin);
                VaraManEvent::AdminAdded(admin)
            } else {
                VaraManEvent::Error("Only an admin can add another admin.".to_owned())
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

    match query {
        StateQuery::All => {
            msg::reply(StateReply::All(contract.into()), 0).expect("Unable to share the state")
        }
        StateQuery::AllGames => {
            let games = contract
                .games
                .iter()
                .map(|(id, game)| (*id, game.clone()))
                .collect();
            msg::reply(StateReply::AllGames(games), 0).expect("Unable to share the state")
        }
        StateQuery::AllPlayers => {
            let players = contract
                .players
                .iter()
                .map(|(id, player)| (*id, player.clone()))
                .collect();
            msg::reply(StateReply::AllPlayers(players), 0).expect("Unable to share the state")
        }
        StateQuery::Game { player_address } => {
            let game: Option<GameInstance> = contract.games.get(&player_address).cloned();
            msg::reply(StateReply::Game(game), 0).expect("Unable to share the state")
        }
        StateQuery::Player { player_address } => {
            let player: Option<Player> = contract.players.get(&player_address).cloned();
            msg::reply(StateReply::Player(player), 0).expect("Unable to share the state")
        }
        StateQuery::Config => {
            msg::reply(StateReply::Config(contract.config), 0).expect("Unable to share the state")
        }
        StateQuery::Admins => {
            msg::reply(StateReply::Admins(contract.admins), 0).expect("Unable to share the state")
        }
        StateQuery::Status => {
            msg::reply(StateReply::Status(contract.status), 0).expect("Unable to share the state")
        }
    };
}
