use crate::services;
use crate::services::single::storage::SessionsStorage;
use crate::single::StepResult;
use core::{fmt::Debug, marker::PhantomData};
use gstd::{msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
use sails_rtl::gstd::{
    events::{EventTrigger, GStdEventTrigger},
    gservice,
};

pub use utils::*;

use self::storage::{GamePairsStorage, MultipleGamesStorage};

pub mod funcs;
pub mod storage;
pub(crate) mod utils;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Event {
    GameCreated {
        player_id: ActorId,
    },
    JoinedTheGame {
        player_id: ActorId,
        game_id: ActorId,
    },
    GameCanceled {
        game_id: ActorId,
    },
    MoveMade {
        step_result: StepResult,
    },
    EndGame {
        winner: ActorId,
    },
}

pub type GstdDrivenService = Service<GStdEventTrigger<Event>>;

#[derive(Clone)]
pub struct Service<X>(PhantomData<X>);

impl<X> Service<X> {
    pub fn seed() -> Self {
        Self(PhantomData)
    }
}

#[gservice]
impl<X> Service<X>
where
    X: EventTrigger<Event>,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn create_game(&mut self, session_for_account: Option<ActorId>) -> () {
        let msg_source = msg::source();
        let player_id = services::utils::panicking(move || {
            funcs::create_game(
                SessionsStorage::as_mut(),
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                msg_source,
                session_for_account,
            )
        });

        services::utils::deposit_event(Event::GameCreated { player_id });
    }
    pub fn join_game(&mut self, game_id: ActorId, session_for_account: Option<ActorId>) -> () {
        let msg_source = msg::source();
        let player_id = services::utils::panicking(move || {
            funcs::join_game(
                SessionsStorage::as_mut(),
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                msg_source,
                game_id,
                session_for_account,
            )
        });

        services::utils::deposit_event(Event::JoinedTheGame { player_id, game_id });
    }
    pub fn cancel_game(&mut self, session_for_account: Option<ActorId>) -> () {
        let msg_source = msg::source();
        let game_id = services::utils::panicking(move || {
            funcs::cancel_game(
                SessionsStorage::as_mut(),
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                msg_source,
                session_for_account,
            )
        });

        services::utils::deposit_event(Event::GameCanceled { game_id });
    }
    pub async fn make_move(
        &mut self,
        game_id: ActorId,
        step: u8,
        session_for_account: Option<ActorId>,
    ) -> () {
        let msg_source = msg::source();
        let res = funcs::make_move(
            SessionsStorage::as_ref(),
            MultipleGamesStorage::as_mut(),
            GamePairsStorage::as_mut(),
            msg_source,
            game_id,
            step,
            session_for_account,
        )
        .await;

        match res {
            Ok(event) => services::utils::deposit_event(event),
            Err(error) => services::utils::panic(error),
        };
    }
    pub fn games(&self) -> Vec<(ActorId, MultipleGame)> {
        MultipleGamesStorage::as_ref()
            .iter()
            .map(|(actor_id, game)| (*actor_id, game.clone()))
            .collect()
    }
    pub fn games_pairs(&self) -> Vec<(ActorId, ActorId)> {
        GamePairsStorage::as_ref()
            .iter()
            .map(|(player_1, player_2)| (*player_1, player_2.clone()))
            .collect()
    }
    pub fn game(&self, player_id: ActorId) -> Option<MultipleGame> {
        let pairs = GamePairsStorage::as_ref();
        if let Some(game_id) = pairs.get(&player_id) {
            MultipleGamesStorage::as_ref().get(game_id).cloned()
        } else {
            None
        }
    }
}
