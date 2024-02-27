#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{
    collections::{BTreeMap, BTreeSet},
    exec, msg,
    prelude::*,
    ActorId,
};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<Config>;
    type Handle = InOut<Command, Result<Event, Error>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<StateQuery, StateReply>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct GameLauncherState {
    pub games: Vec<(ActorId, Game)>,
    pub players_to_game_id: Vec<(ActorId, ActorId)>,
    pub config: Config,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum StateQuery {
    All,
    GetGame { player_id: ActorId },
}
#[derive(Encode, Decode, TypeInfo)]
pub enum StateReply {
    All(GameLauncherState),
    Game(Option<(Game, Option<u64>)>),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct Config {
    pub time_to_move: u32,
    pub gas_to_check_game: u64,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    enum_iterator::Sequence,
    TypeInfo,
    Encode,
    Decode,
)]
pub enum Face {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, TypeInfo, Encode, Decode)]
pub struct Tile {
    pub left: Face,
    pub right: Face,
}

impl Tile {
    pub fn new(left: Face, right: Face) -> Self {
        Self { left, right }
    }

    pub fn swap(self) -> Self {
        Self {
            left: self.right,
            right: self.left,
        }
    }

    pub fn is_double(&self) -> bool {
        self.left == self.right
    }

    pub fn can_adjoin(&self, other: &Tile) -> bool {
        self.right == other.left
    }
}

pub fn build_tile_collection() -> Vec<Tile> {
    enum_iterator::all::<Face>()
        .enumerate()
        .flat_map(|(i, face_first)| {
            enum_iterator::all::<Face>()
                .skip(i)
                .map(move |face_second| Tile::new(face_first, face_second))
        })
        .collect()
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub enum Command {
    CreateGame,
    Skip {
        creator: ActorId,
    },
    Place {
        creator: ActorId,
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
    },
    Register {
        creator: ActorId,
    },
    CancelRegistration {
        creator: ActorId,
    },
    DeletePlayer {
        player_id: ActorId,
    },
    CheckGame {
        game_id: ActorId,
        last_activity_time: u64,
    },
    StartGame,
    CancelGame,
    LeaveGame,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug)]
pub enum Event {
    GameFinished {
        winners: Vec<ActorId>,
        all_participants: Vec<ActorId>,
    },
    GameCreated,
    Skipped,
    Placed {
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
    },
    Registered {
        player: ActorId,
    },
    RegistrationCanceled,
    PlayerDeleted {
        player_id: ActorId,
    },
    GameStarted,
    GameCanceled,
    GameLeft,
    Checked,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Error {
    GameHasAlreadyStarted,
    GameHasNotStartedYet,
    YouAlreadyRegistered,
    LimitHasBeenReached,
    GameFinished,
    NotYourTurnOrYouLose,
    InvalidTile,
    InvalidTileId,
    InvalidTileOwner,
    InvalidTrack,
    AlreadyExists,
    GameDoesNotExist,
    NotRegistered,
    YouLose,
    WrongBid,
    NoSuchPlayer,
    StateIsNotPlaying,
    SeveralGames,
    YouAreAdmin,
    NotEnoughPlayers,
    GameIsGoing,
    OnlyProgramCanSend,
}

#[derive(Debug, TypeInfo, Encode, Decode, Clone, Default)]
pub struct TrackData {
    pub tiles: Vec<Tile>,
    pub has_train: bool,
}

#[derive(Debug, TypeInfo, Encode, Decode, Clone, Default)]
pub struct Game {
    pub admin: ActorId,
    pub game_state: Option<GameState>,
    pub initial_players: Vec<ActorId>,
    pub state: State,
    pub is_started: bool,
    pub bid: u128,
}

#[derive(Encode, Decode, TypeInfo, Debug, Clone)]
pub struct Player {
    pub id: ActorId,
    pub lose: bool,
}

#[derive(Debug, TypeInfo, Encode, Decode, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub tracks: Vec<TrackData>,
    pub shots: Vec<u32>,
    pub start_tile: u32,
    pub current_player: u32,
    pub tile_to_player: BTreeMap<u32, u32>,
    pub tiles: Vec<Tile>,
    pub remaining_tiles: BTreeSet<u32>,
    pub time_to_move: u32,
    pub last_activity_time: u64,
}

#[derive(Clone, Debug, Encode, Decode, Default, TypeInfo, PartialEq, Eq)]
pub enum State {
    Playing,
    Winners(Vec<ActorId>),
    #[default]
    Registration,
}

#[cfg(not(test))]
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("internal error: random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

/// mock for test
#[cfg(test)]
fn get_random_u32() -> u32 {
    0u32
}

/// - 2..4 players: 8 tiles
/// - 5 players: 7 tiles
/// - 6 players: 6 tiles
/// - 7 players: 5 tiles
/// - 8 players: 4 tiles
fn tiles_per_person(players_amount: usize) -> usize {
    match players_amount {
        2..=4 => 8,
        5..=8 => 12 - players_amount,
        _ => unreachable!("Invalid player amount reached"),
    }
}

/// Get random number from BTreeSet
fn get_random_from_set<T: Copy>(set: &BTreeSet<T>) -> T {
    let max_index = set.len();
    let index = (get_random_u32() as usize) % max_index;
    *set.iter().nth(index).unwrap()
}

/// Check if 'current_tile' tile is double and bigger than 'stored_tile'
fn is_double_tile_bigger(
    current_tile_id: u32,
    stored_tile_id: Option<(u32, u32)>,
    tiles: &[Tile],
) -> bool {
    let current_tile = tiles[current_tile_id as usize];
    if !current_tile.is_double() {
        return false;
    }

    if let Some((stored_id, _)) = stored_tile_id {
        let stored_tile = tiles[stored_id as usize];
        if stored_tile.left >= current_tile.left {
            return false;
        }
    }

    true
}

/// Gives everyone 1 tile
/// Stops if someone get double
///
/// Returns matching tile id if it's given
/// otherwise returns None
fn give_tiles_until_double(
    remaining_tiles: &mut BTreeSet<u32>,
    tiles: &[Tile],
    tile_to_player: &mut BTreeMap<u32, u32>,
    players_amount: usize,
) -> Option<(u32, u32)> {
    let mut starting_pair = None;

    for player_index in 0..players_amount {
        // giving a new tile to player
        let tile_id = get_random_from_set(remaining_tiles);
        remaining_tiles.remove(&tile_id);
        tile_to_player.insert(tile_id, player_index as u32);

        // check if it matchs or not
        if is_double_tile_bigger(tile_id, starting_pair, tiles) {
            starting_pair = Some((tile_id, player_index as u32));
        }
    }

    starting_pair
}

impl GameState {
    // TODO: cover it with tests
    pub fn new(
        initial_players: Vec<ActorId>,
        time_to_move: u32,
        block_timestamp: u64,
    ) -> Option<GameState> {
        let players_amount = initial_players.len();

        let mut tile_to_player: BTreeMap<u32, u32> = Default::default();
        // Build all possible tiles
        let tiles = build_tile_collection();
        let mut remaining_tiles: BTreeSet<u32> = Default::default();
        for index in 0..tiles.len() {
            remaining_tiles.insert(index as u32);
        }

        // Spread tiles to players
        let tiles_per_person = tiles_per_person(players_amount);
        for player_index in 0..initial_players.len() {
            for _ in 1..=tiles_per_person {
                let tile_id = get_random_from_set(&remaining_tiles);
                remaining_tiles.remove(&tile_id);

                tile_to_player.insert(tile_id, player_index as u32);
            }
        }

        // Recognize starting person and tile
        let mut starting_pair: Option<(u32, u32)> = None;

        for (tile_index, person_index) in &tile_to_player {
            if is_double_tile_bigger(*tile_index, starting_pair, &tiles) {
                starting_pair = Some((*tile_index, *person_index));
            }
        }

        // Add tiles if no matching starting tile exists
        while starting_pair.is_none() {
            starting_pair = give_tiles_until_double(
                &mut remaining_tiles,
                &tiles,
                &mut tile_to_player,
                players_amount,
            );
        }

        let (start_tile, start_player) =
            starting_pair.expect("failed to determine initial game state");

        // Remove starting tile from set
        tile_to_player.remove(&start_tile);

        let players = initial_players
            .into_iter()
            .map(|id| Player { id, lose: false })
            .collect();

        let current_player = (start_player + 1) % players_amount as u32;

        Some(GameState {
            players,
            tracks: vec![Default::default(); players_amount],
            shots: vec![0u32; players_amount],
            start_tile,
            current_player,
            tile_to_player,
            tiles,
            remaining_tiles,
            time_to_move,
            last_activity_time: block_timestamp,
        })
    }

    pub fn skip_turn(&mut self, player: ActorId, bid: u128) -> Result<Event, Error> {
        let i = self.current_player as usize;

        if self.players[i].id != player {
            return Err(Error::NotYourTurnOrYouLose);
        }

        let count_players_is_live = self.players.iter().filter(|&player| !player.lose).count();
        let time = exec::block_timestamp();
        if count_players_is_live == 1 {
            self.last_activity_time = time;
            send_value(player, bid * self.players.len() as u128);
            let all_participants = self.players.iter().map(|player| player.id).collect();
            return Ok(Event::GameFinished {
                winners: vec![player],
                all_participants,
            });
        }

        self.tracks[i].has_train = true;

        if let Some(event) = self.post_actions(bid) {
            self.last_activity_time = time;
            return Ok(event);
        }
        self.last_activity_time = time;
        Ok(Event::Skipped)
    }

    fn post_actions(&mut self, bid: u128) -> Option<Event> {
        // check if the current player wins
        let remaining_tiles = self
            .tile_to_player
            .values()
            .filter(|&player| *player == self.current_player)
            .count();
        if remaining_tiles == 0 {
            let player = self.players[self.current_player as usize].clone();
            send_value(player.id, bid * self.players.len() as u128);
            let all_participants = self.players.iter().map(|player| player.id).collect();
            return Some(Event::GameFinished {
                winners: vec![player.id],
                all_participants,
            });
        }

        // check if any next player is able to make a turn
        let players_to_check = self.players.len();
        let check_result = (0..players_to_check).try_fold(self.current_player, |player, _| {
            let next_player = self
                .next_player(player)
                .expect("Error: there is no next player");
            let remaining_tiles = self
                .tile_to_player
                .iter()
                .filter_map(|(&tile, &player)| (player == next_player).then_some(tile as usize))
                .collect::<Vec<_>>();

            let player_index = next_player as usize;
            let available_tracks =
                [player_index]
                    .iter()
                    .copied()
                    .chain(self.tracks.iter().enumerate().filter_map(|(i, track)| {
                        (i != player_index && track.has_train).then_some(i)
                    }))
                    .collect::<Vec<_>>();
            if self.check_tiles(&remaining_tiles, &available_tracks) {
                self.current_player = next_player;
                return None;
            }
            if !self.remaining_tiles.is_empty() && self.tracks[player_index].has_train {
                // give the player randomly chosen tile
                let tile_id = get_random_from_set(&self.remaining_tiles);
                self.remaining_tiles.remove(&tile_id);

                self.tile_to_player.insert(tile_id, next_player);
                self.current_player = next_player;

                return None;
            }

            self.tracks[player_index].has_train = true;

            Some(next_player)
        });

        if check_result.is_some() && self.remaining_tiles.is_empty() {
            // no one can make turn. Point scoring
            let winners = self.point_scoring();

            let prize = bid * self.players.len() as u128 / winners.len() as u128;

            if bid != 0 {
                winners.iter().for_each(|player| {
                    send_value(*player, prize);
                });
            }
            let all_participants = self.players.iter().map(|player| player.id).collect();
            return Some(Event::GameFinished {
                winners,
                all_participants,
            });
        }
        None
    }

    pub fn next_player(&self, current_player: u32) -> Option<u32> {
        for i in 1..=self.players.len() {
            let index = (current_player as usize + i) % self.players.len();
            if !self.players[index].lose {
                return Some(index as u32);
            }
        }
        None
    }

    // Helper function to check if any of the tiles can be put on any track.
    fn check_tiles(&self, tiles: &[usize], tracks: &[usize]) -> bool {
        for tile_index in tiles {
            let tile = self.tiles[*tile_index];
            for track_id in tracks {
                if self.can_put_tile(tile, *track_id).is_some() {
                    return true;
                }
            }
        }

        false
    }

    pub fn make_turn(
        &mut self,
        player: ActorId,
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
        bid: u128,
    ) -> Result<Event, Error> {
        let i = self.current_player as usize;

        if self.players[i].id != player {
            return Err(Error::NotYourTurnOrYouLose);
        }
        let count_players_is_live = self.players.iter().filter(|&player| !player.lose).count();
        let time = exec::block_timestamp();
        if count_players_is_live == 1 {
            self.last_activity_time = time;
            send_value(player, bid * self.players.len() as u128);
            let all_participants = self.players.iter().map(|player| player.id).collect();
            return Ok(Event::GameFinished {
                winners: vec![player],
                all_participants,
            });
        }

        // check player owns the tile
        match self.tile_to_player.get(&tile_id) {
            None => return Err(Error::InvalidTileId),
            Some(user_id) if *user_id != self.current_player => {
                return Err(Error::InvalidTileOwner)
            }
            _ => (),
        }

        // check tile can be put on the track
        if track_id != self.current_player
            && !self
                .tracks
                .get(track_id as usize)
                .map_or(false, |data| data.has_train)
        {
            return Err(Error::InvalidTrack);
        }

        let tile = self.tiles[tile_id as usize];
        let track_index = track_id as usize;
        match self.can_put_tile(tile, track_index) {
            Some(tile) => self.tracks[track_index].tiles.push(tile),
            None => return Err(Error::InvalidTile),
        }

        // remove train if all criterea met
        if remove_train && track_id == self.current_player {
            self.tracks[i].has_train = false;
            self.shots[i] += 1;
        }

        // remove tile from player's set
        self.tile_to_player.remove(&tile_id);
        self.last_activity_time = exec::block_timestamp();

        if let Some(event) = self.post_actions(bid) {
            self.last_activity_time = time;
            return Ok(event);
        }
        self.last_activity_time = time;
        Ok(Event::Placed {
            tile_id,
            track_id,
            remove_train,
        })
    }

    fn can_put_tile(&self, tile: Tile, track_id: usize) -> Option<Tile> {
        let track = &self.tracks[track_id];
        let last_tile = match track.tiles.last() {
            None => &self.tiles[self.start_tile as usize],
            Some(tile) => tile,
        };

        if last_tile.can_adjoin(&tile) {
            return Some(tile);
        }

        let tile = tile.swap();
        if last_tile.can_adjoin(&tile) {
            return Some(tile);
        }

        None
    }
    fn point_scoring(&self) -> Vec<ActorId> {
        let mut scores: BTreeMap<ActorId, u16> = BTreeMap::new();

        for (tile, player) in &self.tile_to_player {
            if !self.players[*player as usize].lose {
                let tile_score =
                    self.tiles[*tile as usize].left as u8 + self.tiles[*tile as usize].right as u8;
                scores
                    .entry(self.players[*player as usize].id)
                    .and_modify(|scores| *scores += tile_score as u16)
                    .or_insert(tile_score as u16);
            }
        }

        let min_score = scores.values().min().cloned();

        scores
            .iter()
            .filter_map(|(actor_id, &score)| {
                if Some(score) == min_score {
                    Some(*actor_id)
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn send_value(destination: ActorId, value: u128) {
    if value != 0 {
        msg::send_with_gas(destination, "", 0, value).expect("Error in sending value");
    }
}

#[cfg(test)]
mod test;
