use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use tequila_io::{GameLauncher, State as GameStatus};

use self::helpers::map_tile_face_into_u32;

#[derive(Encode, Decode, TypeInfo)]
pub struct TrackData {
    pub tiles: Vec<(u32, u32)>,
    pub has_train: bool,
}

#[derive(Encode, Decode, TypeInfo, Default)]
pub struct GameState {
    /// List of all players
    pub players: Vec<(ActorId, String)>,
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
    /// Game state
    pub state: GameStatus,
}

#[metawasm]
pub mod metafns {
    pub type State = GameLauncher;

    pub fn is_started(state: State) -> bool {
        state.is_started
    }

    pub fn get_limit(state: State) -> Option<u64> {
        state.maybe_limit
    }

    pub fn players(state: State) -> Vec<(ActorId, String)> {
        state.players
    }

    pub fn game_state(state: State) -> GameState {
        let game_state = state
            .game_state
            .expect("Invalid game state. Game is not started.");

        if game_state.state == GameStatus::Registration {
            return GameState {
                players: game_state.players,
                ..Default::default()
            };
        }

        let current_tile = game_state.tiles[game_state.start_tile as usize];
        let mut players_tiles = vec![Vec::<(u32, u32)>::new(); game_state.players.len()];

        for pair in game_state.tile_to_player.iter() {
            players_tiles[*pair.1 as usize].push((
                map_tile_face_into_u32(game_state.tiles[*pair.0 as usize].left),
                map_tile_face_into_u32(game_state.tiles[*pair.0 as usize].right),
            ));
        }

        let current_state = game_state.state();

        GameState {
            players: game_state.players,
            current_player: game_state.current_player,
            start_tile: (
                map_tile_face_into_u32(current_tile.left),
                map_tile_face_into_u32(current_tile.right),
            ),
            tracks: game_state
                .tracks
                .iter()
                .map(|td| TrackData {
                    has_train: td.has_train,
                    tiles: td
                        .tiles
                        .iter()
                        .map(|t| {
                            (
                                map_tile_face_into_u32(t.left),
                                map_tile_face_into_u32(t.right),
                            )
                        })
                        .collect(),
                })
                .collect(),
            players_tiles,
            shot_counters: game_state.shots,
            state: current_state,
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
