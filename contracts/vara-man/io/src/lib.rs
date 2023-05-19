#![no_std]

mod rand;

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};
pub use rand::*;

pub const MAP_WIDTH: usize = 17;
pub const MAP_HEIGHT: usize = 12;
pub const MAP_CELLS: usize = MAP_WIDTH * MAP_HEIGHT;
pub const MAX_PERCENTAGE: u128 = 100;
pub const GAME_TIMEOUT_MS: i64 = 300_000;
pub const MAX_RETRIES_COUNT: u8 = 3;
pub const BPS_SCALE: u16 = 10_000;
pub const COINS_SCALE: u8 = 6;

pub type GameSeed = u64;
pub type Map = [[Entity; MAP_WIDTH]; MAP_HEIGHT];

pub struct VaraManMetadata;

impl Metadata for VaraManMetadata {
    type Init = In<VaraManInit>;
    type Handle = InOut<VaraManAction, VaraManEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = VaraMan;
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct VaraMan {
    pub games: Vec<GameInstance>,
    pub players: Vec<(ActorId, Player)>,
    pub status: Status,
    pub config: Config,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Status {
    #[default]
    Paused,
    Started,
}

#[derive(Debug, Default, Clone, Copy, Encode, Decode, TypeInfo)]
pub struct Config {
    pub operator: ActorId,

    /// Should be scaled by `reward_token_id` precision.
    pub tokens_per_gold_coin: u64,
    /// Should be scaled by `reward_token_id` precision.
    pub tokens_per_silver_coin: u64,

    pub easy_reward_scale_bps: u16,
    pub medium_reward_scale_bps: u16,
    pub hard_reward_scale_bps: u16,

    pub gold_coins: u64,
    pub silver_coins: u64,
}

impl Config {
    pub fn is_valid(&self) -> bool {
        !self.operator.is_zero() && self.gold_coins + self.silver_coins <= MAP_CELLS as u64
    }

    pub fn get_reward_scale_bps(&self, level: Level) -> u16 {
        match level {
            Level::Easy => self.easy_reward_scale_bps,
            Level::Medium => self.medium_reward_scale_bps,
            Level::Hard => self.hard_reward_scale_bps,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct Player {
    pub name: String,
    pub retries: u64,
    pub claimed_gold_coins: u64,
    pub claimed_silver_coins: u64,
}

impl Player {
    pub fn is_have_retries(&self) -> bool {
        self.retries < MAX_RETRIES_COUNT as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Effect {
    Speed,
    Slow,
    Blind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum Entity {
    /// 25% chance to spawn.
    Empty,
    /// 10% chance to spawn.
    GoldCoin(Option<Effect>),
    /// 65% chance to spawn.
    SilverCoin,
    /// 25% chance to spawn.
    ZombieCat,
    /// 25% chance to spawn.
    BatCat,
    /// 10% chance to spawn.
    BullyCat,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GameInstance {
    pub level: Level,
    pub player_address: ActorId,
    pub gold_coins: u64,
    pub silver_coins: u64,
    pub start_time_ms: i64,
    pub is_claimed: bool,
    pub map: Map,
}

impl GameInstance {
    pub fn new(
        level: Level,
        player_address: ActorId,
        start_time_ms: i64,
        seed: GameSeed,
    ) -> GameInstance {
        let mut map: Map = [[Entity::Empty; MAP_WIDTH]; MAP_HEIGHT];
        let mut rnd = Rand { seed };

        let mut gold_coins = 0u64;
        let mut silver_coins = 0u64;
        let mut effects = vec![Effect::Speed, Effect::Slow, Effect::Blind];

        #[allow(clippy::needless_range_loop)]
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let entity = {
                    let c: u64 = rnd.range(100);

                    if c <= 10 {
                        if c % 2 == 0 {
                            gold_coins += 1;
                            Entity::GoldCoin(effects.pop())
                        } else {
                            Entity::BullyCat
                        }
                    } else if c <= 25 {
                        let p = [Entity::Empty, Entity::ZombieCat, Entity::BatCat];
                        p[c as usize % p.len()]
                    } else {
                        silver_coins += 1;
                        Entity::SilverCoin
                    }
                };

                map[y][x] = entity;
            }
        }

        Self {
            level,
            player_address,
            gold_coins,
            silver_coins,
            start_time_ms,
            map,
            is_claimed: false,
        }
    }

    pub fn new_with_coins(
        level: Level,
        gold_coins: u64,
        silver_coins: u64,
        player_address: ActorId,
        start_time_ms: i64,
        seed: GameSeed,
    ) -> GameInstance {
        let mut map: Map = [[Entity::Empty; MAP_WIDTH]; MAP_HEIGHT];
        let mut effects = vec![Effect::Speed, Effect::Slow, Effect::Blind];
        let mut rnd = Rand { seed };

        // 1. Transform game map
        let mut cells = Vec::new();
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                cells.push((Entity::Empty, y, x));
            }
        }

        // 2. Pick N random positions for gold coins
        for _ in 0..=gold_coins {
            let i = rnd.range(cells.len() as u64) as usize;
            let (_, y, x) = cells.remove(i);

            map[y][x] = Entity::GoldCoin(None);
        }

        // 3. Pick N random positions for silver coins
        for _ in 0..=silver_coins {
            let i = rnd.range(cells.len() as u64) as usize;
            let (_, y, x) = cells.remove(i);

            map[y][x] = Entity::SilverCoin;
        }

        // 4. Fill remaining map with monsters and effects
        #[allow(clippy::needless_range_loop)]
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let r = rnd.rand();
                let entity = map[y][x];

                let new_entity = if entity == Entity::Empty {
                    let p = [
                        Entity::Empty,
                        Entity::ZombieCat,
                        Entity::BatCat,
                        Entity::BullyCat,
                    ];
                    p[r as usize % p.len()]
                } else if entity == Entity::GoldCoin(None) {
                    if r % 2 == 0 {
                        Entity::GoldCoin(effects.pop())
                    } else {
                        /* let p = [
                            Entity::GoldCoin(None),
                            Entity::GoldCoin(Some(Effect::Speed)),
                            Entity::GoldCoin(Some(Effect::Slow)),
                            Entity::GoldCoin(Some(Effect::Blind)),
                        ];
                        p[r as usize % p.len()] */

                        entity
                    }
                } else {
                    entity
                };

                map[y][x] = new_entity;
            }
        }

        Self {
            level,
            player_address,
            gold_coins,
            silver_coins,
            start_time_ms,
            map,
            is_claimed: false,
        }
    }

    pub fn is_timeout(&self, time_ms: i64) -> bool {
        time_ms >= self.start_time_ms + GAME_TIMEOUT_MS
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum VaraManAction {
    StartGame {
        level: Level,
        seed: GameSeed,
    },
    RegisterPlayer {
        name: String,
    },
    ClaimReward {
        game_id: u64,
        silver_coins: u64,
        gold_coins: u64,
    },
    ChangeStatus(Status),
    ChangeConfig(Config),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum VaraManEvent {
    GameStarted(u64),
    RewardClaimed {
        player_address: ActorId,
        game_id: u64,
        silver_coins: u64,
        gold_coins: u64,
    },
    PlayerRegistered(ActorId),
    StatusChanged(Status),
    ConfigChanged(Config),
    Error(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct VaraManInit {
    pub config: Config,
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;

    #[test]
    fn success_game_instance() {
        let game_instance = GameInstance::new(Level::Easy, ActorId::zero(), 0, u64::MAX);

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let e = game_instance.map[y][x];
                let c = match e {
                    Entity::Empty => ' ',
                    Entity::ZombieCat => '🐱',
                    Entity::BatCat => '😼',
                    Entity::BullyCat => '😾',
                    Entity::GoldCoin(_) => '🥇',
                    Entity::SilverCoin => '🪙',
                };
                std::print!("{c}");
            }
            std::println!();
        }
    }

    #[test]
    fn success_game_instance_with_coins() {
        let game_instance =
            GameInstance::new_with_coins(Level::Easy, 5, 20, ActorId::zero(), 0, u64::MAX);

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let e = game_instance.map[y][x];
                let c = match e {
                    Entity::Empty => ' ',
                    Entity::ZombieCat => '🐱',
                    Entity::BatCat => '😼',
                    Entity::BullyCat => '😾',
                    Entity::GoldCoin(_) => '🥇',
                    Entity::SilverCoin => '🪙',
                };
                std::print!("{c}");
            }
            std::println!();
        }

        std::println!(
            "gold_coins: {}, silver_coins: {}",
            game_instance.gold_coins,
            game_instance.silver_coins
        );
    }
}
