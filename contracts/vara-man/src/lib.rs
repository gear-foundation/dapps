#![no_std]

use gstd::{msg, prelude::*, ActorId};
use vara_man_io::{
    Config, GameInstance, Player, Status, VaraMan as VaraManState, VaraManAction, VaraManEvent,
    VaraManInit, BPS_SCALE,
};

#[derive(Debug, Default)]
struct VaraMan {
    pub games: HashMap<ActorId, GameInstance>,
    pub players: HashMap<ActorId, Player>,
    pub status: Status,
    pub config: Config,
}

impl From<&VaraMan> for VaraManState {
    fn from(value: &VaraMan) -> Self {
        let VaraMan {
            games,
            players,
            status,
            config,
        } = value;

        let games = games.iter().map(|(id, game)| (*id, game.clone())).collect();

        let players = players
            .iter()
            .map(|(actor_id, player)| (*actor_id, player.clone()))
            .collect();

        Self {
            games,
            players,
            status: *status,
            config: *config,
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
                        retries: 0,
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

            if !player.is_have_retries() {
                return VaraManEvent::Error("Player has exhausted all his attempts.".to_owned());
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

            player.retries += 1;

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

                let reward_scale_bps = vara_man.config.get_reward_scale_bps(game.level);

                let base_tokens_amount = vara_man
                    .config
                    .tokens_per_gold_coin
                    .checked_mul(gold_coins)
                    .expect("Math overflow!")
                    .checked_add(
                        vara_man
                            .config
                            .tokens_per_silver_coin
                            .checked_mul(silver_coins)
                            .expect("Math overflow!"),
                    )
                    .expect("Math overflow!");

                let tokens_amount = base_tokens_amount
                    .checked_add(
                        base_tokens_amount
                            .checked_mul(reward_scale_bps.into())
                            .expect("Math overflow!")
                            .checked_div(BPS_SCALE.into())
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
    }
}

#[no_mangle]
extern fn init() {
    let init: VaraManInit = msg::load().expect("Unexpected invalid init payload.");

    assert!(init.config.is_valid());

    unsafe {
        VARA_MAN = Some(VaraMan {
            config: init.config,
            ..Default::default()
        })
    };
}

#[no_mangle]
extern fn state() {
    msg::reply(
        unsafe {
            let vara_man = VARA_MAN.as_ref().expect("Uninitialized vara man state.");
            let vara_man_state: VaraManState = vara_man.into();
            vara_man_state
        },
        0,
    )
    .expect("Unexpected invalid reply result.");
}
