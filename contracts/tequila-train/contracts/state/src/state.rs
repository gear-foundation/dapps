use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use tequila_io::{GameState as GameStateRaw, TrackData};

use self::helpers::map_tile_face_into_u32;

#[derive(Encode, Decode, TypeInfo)]
pub struct GameState {
    /// List of all players
    pub players: Vec<ActorId>,
    /// Index of a current player inside the `players` list
    pub current_player: u32,
    /// Start tile of the game
    pub start_tile: (u32, u32),
    /// Tracks
    pub tracks: Vec<TrackData>,
    /// Tiles on tracks
    pub players_tiles: Vec<Vec<(u32, u32)>>,
    /// Shot counters
    pub shot_counters: Vec<u32>,
}

#[metawasm]
pub trait Metawasm {
    type State = GameStateRaw;

    fn game_state(state: Self::State) -> GameState {
        let current_tile = state.tiles[state.start_tile as usize];
        let mut players_tiles = vec![Vec::<(u32, u32)>::new(); state.players.len()];
        for pair in state.tile_to_player.iter() {
            players_tiles[*pair.1 as usize].push((
                map_tile_face_into_u32(state.tiles[*pair.0 as usize].left),
                map_tile_face_into_u32(state.tiles[*pair.0 as usize].right),
            ));
        }
        GameState {
            players: state.players,
            current_player: state.current_player,
            start_tile: (
                map_tile_face_into_u32(current_tile.left),
                map_tile_face_into_u32(current_tile.right),
            ),
            tracks: state.tracks,
            players_tiles,
            shot_counters: state.shots,
        }
    }
}

mod helpers {
    use tequila_io::Face;

    pub fn map_tile_face_into_u32(face: Face) -> u32 {
        match face {
            Face::Zero => 0,
            Face::One => 1,
            Face::Two => 2,
            Face::Three => 3,
            Face::Four => 4,
            Face::Five => 5,
            Face::Six => 6,
            Face::Seven => 7,
            Face::Eight => 8,
            Face::Nine => 9,
            Face::Ten => 10,
            Face::Eleven => 11,
            Face::Twelve => 12,
        }
    }
}
