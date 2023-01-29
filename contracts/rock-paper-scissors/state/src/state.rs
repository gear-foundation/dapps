use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use rps_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;

    fn config(state: Self::State) -> GameConfig {
        state.game_config
    }

    fn lobby_list(state: Self::State) -> Vec<ActorId> {
        state.lobby
    }

    fn game_stage(state: Self::State) -> GameStage {
        state.stage
    }

    fn current_stage_start_timestamp(state: Self::State) -> u64 {
        state.current_stage_start_timestamp
    }
}
