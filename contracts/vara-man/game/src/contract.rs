use gstd::{exec, msg, prelude::*, ActorId};
use hashbrown::HashMap;
use vara_man_io::{
    Config, GameInstance, Player, Status, VaraMan as VaraManState, VaraManAction, VaraManEvent,
    VaraManInit, BPS_SCALE,
};

#[derive(Debug, Default)]
struct VaraMan {
    pub games: Vec<GameInstance>,
    pub players: HashMap<ActorId, Player>,
    pub status: Status,
    pub config: Config,
}

impl From<&VaraMan> for VaraManState {
    fn from(value: &VaraMan) -> Self {
        VaraManState {
            games: value.games.clone(),
            players: value
                .players
                .iter()
                .map(|(actor_id, player)| (*actor_id, player.clone()))
                .collect(),
            status: value.status,
            config: value.config,
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

            if vara_man
                .players
                .insert(
                    actor_id,
                    Player {
                        name,
                        retries: 0,
                        claimed_gold_coins: 0,
                        claimed_silver_coins: 0,
                    },
                )
                .is_some()
            {
                VaraManEvent::Error("Player is already registered.".to_owned())
            } else {
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

            if !player.is_have_retries() {
                return VaraManEvent::Error("Player has exhausted all his attempts.".to_owned());
            }

            player.retries += 1;

            vara_man.games.push(GameInstance::new(
                level,
                player_address,
                exec::block_timestamp() as i64,
                seed,
            ));

            VaraManEvent::GameStarted((vara_man.games.len() - 1) as u64)
        }
        VaraManAction::ClaimReward {
            game_id,
            silver_coins,
            gold_coins,
        } => {
            if let Some(game) = vara_man.games.get_mut(game_id as usize) {
                let player_address = msg::source();

                if vara_man.status == Status::Paused {
                    return VaraManEvent::Error("Incorrect whole game status.".to_owned());
                }

                let Some(player) = vara_man.players.get_mut(&player_address) else {
                    return VaraManEvent::Error("Player must be registered to claim.".to_owned());
                };

                // Check that player address is equal to tx signer(initiator)
                if player_address != game.player_address {
                    return VaraManEvent::Error(
                        "Caller `msg::source` is not eq to actual game player.".to_owned(),
                    );
                }

                // Check that game is ended by time
                if !game.is_timeout(exec::block_timestamp() as i64) {
                    return VaraManEvent::Error("Game is not ended.".to_owned());
                }

                // Check that game rewards are not claimed already
                if game.is_claimed {
                    return VaraManEvent::Error("Rewards are already claimed.".to_owned());
                }

                let reward_scale_bps = vara_man.config.get_reward_scale_bps(game.level);

                let gold_coins = gold_coins
                    .checked_add(
                        gold_coins
                            .checked_mul(reward_scale_bps.into())
                            .expect("Math overflow!")
                            .checked_div(BPS_SCALE.into())
                            .expect("Math overflow!"),
                    )
                    .expect("Math overflow!");

                let silver_coins = silver_coins
                    .checked_add(
                        silver_coins
                            .checked_mul(reward_scale_bps.into())
                            .expect("Math overflow!")
                            .checked_div(BPS_SCALE.into())
                            .expect("Math overflow!"),
                    )
                    .expect("Math overflow!");

                let _tokens_amount = vara_man
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

                // TODO: Transfer tokens

                player.claimed_gold_coins += player
                    .claimed_gold_coins
                    .checked_add(gold_coins)
                    .expect("Math overflow!");
                player.claimed_silver_coins = player
                    .claimed_silver_coins
                    .checked_add(silver_coins)
                    .expect("Math overflow!");

                game.is_claimed = true;

                VaraManEvent::RewardClaimed {
                    player_address,
                    game_id,
                    silver_coins,
                    gold_coins,
                }
            } else {
                VaraManEvent::Error("Invalid game id.".to_owned())
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
            } else {
                vara_man.config = config;
                VaraManEvent::ConfigChanged(config)
            }
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let init: VaraManInit = msg::load().expect("Unexpected invalid init payload.");

    unsafe {
        VARA_MAN = Some(VaraMan {
            config: init.config,
            ..Default::default()
        })
    };
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}

#[no_mangle]
extern "C" fn state() {
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
