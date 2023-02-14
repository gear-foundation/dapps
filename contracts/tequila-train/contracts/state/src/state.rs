use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use tequila_io::GameState as GameStateRaw;

use self::helpers::map_tile_face_into_u32;

#[derive(Encode, Decode, TypeInfo)]
pub struct GameState {
    /// List of all players
    pub players: Vec<ActorId>,
    /// Index of a current player inside the `players` list
    pub current_player: u32,
    /// Start tile of the game
    pub start_tile: (u32, u32),
    /// Tracks with trains
    pub trains_on_board: Vec<bool>,
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
        GameState {
            players: state.players,
            current_player: state.current_player,
            start_tile: (
                map_tile_face_into_u32(current_tile.left),
                map_tile_face_into_u32(current_tile.right),
            ),
            trains_on_board: state.tracks.iter().map(|td| td.has_train).collect(),
            players_tiles: state
                .tracks
                .iter()
                .map(|td| {
                    td.tiles
                        .iter()
                        .map(|t| {
                            (
                                map_tile_face_into_u32(t.left),
                                map_tile_face_into_u32(t.right),
                            )
                        })
                        .collect()
                })
                .collect(),
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
