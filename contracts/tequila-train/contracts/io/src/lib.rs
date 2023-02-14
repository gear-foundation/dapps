#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum PingPong {
    Ping,
    Pong,
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<PingPong, PingPong>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = State;
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug, Default)]
pub struct State(pub Vec<(ActorId, u128)>);

#[doc(hidden)]
impl State {
    pub fn pingers(self) -> Vec<ActorId> {
        self.0.into_iter().map(|pingers| pingers.0).collect()
    }

    pub fn ping_count(self, actor: ActorId) -> u128 {
        self.0
            .into_iter()
            .find_map(|(pinger, ping_count)| (pinger == actor).then_some(ping_count))
            .unwrap_or_default()
    }
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum StateQuery {
    AllState,
    Pingers,
    PingCount(ActorId),
}

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum StateQueryReply {
    AllState(<ContractMetadata as Metadata>::State),
    Pingers(Vec<ActorId>),
    PingCount(u128),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, enum_iterator::Sequence)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tile {
    left: Face,
    right: Face,
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

#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Command {
    Skip,
    Place {
        tile_id: u32,
        track_id: u32,
        remove_train: bool,
    },
}

#[derive(Clone, Debug, Default)]
pub struct TrackData {
    tiles: Vec<Tile>,
    has_train: bool,
}

pub struct GameState {
    players: Vec<ActorId>,
    tracks: Vec<TrackData>,
    shots: Vec<u32>,
    start_tile: u32,
    current_player: u32,
    tile_to_player: BTreeMap<u32, u32>,
    tiles: Vec<Tile>,
    _remaining_tiles: BTreeSet<u32>,
    winner: Option<ActorId>,
}

impl GameState {
    pub fn winner(&self) -> Option<ActorId> {
        self.winner
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
        let i = self.current_player as usize;
        self.current_player = match i + 1 >= self.players.len() {
            true => 0,
            false => (i + 1) as u32,
        };

        todo!()
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
        if !self.put_tile(tile, track_id as usize) {
            unreachable!("invalid tile");
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

    fn put_tile(&mut self, tile: Tile, track_id: usize) -> bool {
        let track = &mut self.tracks[track_id];
        let last_tile = match track.tiles.last() {
            None => &self.tiles[self.start_tile as usize],
            Some(tile) => tile,
        };

        if last_tile.can_adjoin(&tile) {
            track.tiles.push(tile);
            return true;
        }

        let tile = tile.swap();
        if last_tile.can_adjoin(&tile) {
            track.tiles.push(tile);
            return true;
        }

        false
    }
}
