#![no_std]

use horse_races_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = horse_races_io::State;

    pub fn query(state: State, query: MetaQuery) -> MetaResponse {
        match query {
            MetaQuery::GetRuns => MetaResponse::Runs(
                state
                    .runs
                    .iter()
                    .map(|(id, run)| (*id, run.clone()))
                    .collect(),
            ),
            MetaQuery::GetHorses(run_id) => MetaResponse::Horses(
                state
                    .runs
                    .get(&run_id)
                    .expect("Run is not found!")
                    .horses
                    .iter()
                    .map(|(name, (horse, amount))| (name.clone(), horse.clone(), *amount))
                    .collect(),
            ),
            MetaQuery::GetManager => MetaResponse::Manager(state.manager),
            MetaQuery::GetOwner => MetaResponse::Owner(state.owner),
            MetaQuery::GetToken => MetaResponse::Token(state.token),
            MetaQuery::GetOracle => MetaResponse::Oracle(state.oracle),
            MetaQuery::GetFeeBps => MetaResponse::FeeBps(state.fee_bps),
            MetaQuery::GetRunNonce => MetaResponse::RunNonce(state.run_nonce),
            MetaQuery::GetRun(run_id) => {
                MetaResponse::Run(state.runs.get(&run_id).expect("Run is not found!").clone())
            }
        }
    }
}
