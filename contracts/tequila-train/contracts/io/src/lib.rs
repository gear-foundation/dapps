#![no_std]

use gmeta::{In, Metadata};
use gstd::{exec, msg, prelude::*, ActorId};

#[cfg(test)]
mod test;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<Vec<ActorId>>;
    type Handle = In<Command>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = GameState;
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

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct Players {
    players: Vec<ActorId>,
}

impl<const N: usize> From<[ActorId; N]> for Players {
    fn from(s: [ActorId; N]) -> Players {
        Players {
            players: s.to_vec(),
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Command {
    Skip,
    Place {
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
    },
}

#[derive(Debug, TypeInfo, Encode, Decode, Clone, Default)]
pub struct TrackData {
    pub tiles: Vec<Tile>,
    pub has_train: bool,
}

#[derive(Debug, TypeInfo, Encode, Decode, Clone)]
pub struct GameState {
    pub players: Vec<ActorId>,
    pub tracks: Vec<TrackData>,
    pub shots: Vec<u32>,
    pub start_tile: u32,
    pub current_player: u32,
    pub tile_to_player: BTreeMap<u32, u32>,
    pub tiles: Vec<Tile>,
    remaining_tiles: BTreeSet<u32>,
    state: State,
}

#[derive(Clone, Copy, Debug, Encode, Decode, TypeInfo)]
pub enum State {
    Playing,
    Stalled,
    Winner(ActorId),
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
    pub fn new(initial_data: &Players) -> Option<GameState> {
        // Check that players amount is allowed
        let players_amount = initial_data.players.len();
        if !(2..=8).contains(&players_amount) {
            return None;
        }

        let mut tile_to_player: BTreeMap<u32, u32> = Default::default();

        // Build all possible tiles
        let tiles = build_tile_collection();
        let mut remaining_tiles: BTreeSet<u32> = Default::default();
        for index in 0..tiles.len() {
            remaining_tiles.insert(index as u32);
        }

        // Spread tiles to players
        let tiles_per_person = tiles_per_person(players_amount);
        for player_index in 0..initial_data.players.len() {
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

        Some(GameState {
            players: initial_data.players.clone(),
            tracks: vec![Default::default(); players_amount],
            shots: vec![0u32; players_amount],
            start_tile,
            current_player: Self::next_player_impl(players_amount, start_player),
            tile_to_player,
            tiles,
            remaining_tiles,
            state: State::Playing,
        })
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn skip_turn(&mut self, player: ActorId) {
        let i = self.current_player as usize;
        if self.players[i] != player {
            unreachable!("it is not your turn");
        }

        self.tracks[i].has_train = true;

        self.post_actions();
    }

    fn post_actions(&mut self) {
        // check if the current player wins
        let remaining_tiles = self
            .tile_to_player
            .values()
            .filter(|&player| *player == self.current_player)
            .count();
        if remaining_tiles == 0 {
            self.state = State::Winner(self.players[self.current_player as usize]);
            return;
        }

        // check if any next player is able to make a turn
        let players_to_check = self.players.len();
        let check_result = (0..players_to_check).try_fold(self.current_player, |player, _| {
            let next_player = self.next_player(player);

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

            if self.tracks[player_index].has_train {
                // give the player randomly chosen tile
                let tile_id = get_random_from_set(&self.remaining_tiles);
                self.remaining_tiles.remove(&tile_id);

                self.tile_to_player.insert(tile_id, next_player);

                return None;
            }

            self.tracks[player_index].has_train = true;

            Some(next_player)
        });

        if check_result.is_some() {
            // no one can make turn. Game is over
            self.state = State::Stalled;
        }
    }

    fn next_player(&self, current_player: u32) -> u32 {
        Self::next_player_impl(self.players.len(), current_player)
    }

    fn next_player_impl(player_count: usize, current_player: u32) -> u32 {
        let i = current_player as usize + 1;
        match i < player_count {
            true => i as u32,
            false => 0,
        }
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

    pub fn make_turn(&mut self, player: ActorId, tile_id: u32, track_id: u32, remove_train: bool) {
        let i = self.current_player as usize;
        if self.players[i] != player {
            unreachable!("it is not your turn");
        }

        // check player owns the tile
        match self.tile_to_player.get(&tile_id) {
            None => unreachable!("invalid tile id"),
            Some(user_id) if *user_id != self.current_player => unreachable!("wrong tile owner"),
            _ => (),
        }

        // check tile can be put on the track
        if track_id != self.current_player
            && self
                .tracks
                .get(track_id as usize)
                .map_or(false, |data| data.has_train)
        {
            unreachable!("invalid track");
        }

        let tile = self.tiles[tile_id as usize];
        let track_index = track_id as usize;
        match self.can_put_tile(tile, track_index) {
            Some(tile) => self.tracks[track_index].tiles.push(tile),
            None => unreachable!("invalid tile"),
        }

        // remove train if all criterea met
        if remove_train && track_id == self.current_player {
            self.tracks[i].has_train = false;
            self.shots[i] += 1;
        }

        // remove tile from player's set
        self.tile_to_player.remove(&tile_id);

        self.post_actions();
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
}
