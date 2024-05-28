use crate::services;
use crate::services::single::{
    funcs::get_player,
    storage::{builtin_bls381::BuiltinStorage, SessionsStorage},
    ActionsForSession,
};
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
        step: u8,
    },
    MoveVerified {
        step: u8,
        result: u8,
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

    pub fn create_game(&mut self, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::StartMultipleGame,
        );
        let player_id = services::utils::panicking(move || {
            funcs::create_game(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                player,
            )
        });

        services::utils::deposit_event(Event::GameCreated { player_id });
    }
    pub fn join_game(&mut self, game_id: ActorId, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::StartMultipleGame,
        );
        let player_id = services::utils::panicking(move || {
            funcs::join_game(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                player,
                game_id,
            )
        });

        services::utils::deposit_event(Event::JoinedTheGame { player_id, game_id });
    }
    pub fn cancel_game(&mut self, session_for_account: Option<ActorId>) {
        let game_id = services::utils::panicking(move || {
            funcs::cancel_game(
                SessionsStorage::as_mut(),
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                session_for_account,
            )
        });

        services::utils::deposit_event(Event::GameCanceled { game_id });
    }
    pub fn make_move(
        &mut self,
        game_id: ActorId,
        step: u8,
        session_for_account: Option<ActorId>,
    ) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::Move,
        );

        let event = services::utils::panicking(move || {
            funcs::make_move(MultipleGamesStorage::as_mut(), player, game_id, step)
        });

        services::utils::deposit_event(event);
    }
    pub async fn verify_move(
        &mut self,
        vk: services::verify::VerifyingKeyBytes,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicMoveInput,
        ic: [Vec<u8>; 4],
        session_for_account: Option<ActorId>,
    ) {
        // get player
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::single::ActionsForSession::VerifyMove,
        );

        // check game state
        services::utils::panicking(move || {
            funcs::check_game(MultipleGamesStorage::as_ref(), player, public_input.hit)
        });

        // check proof
        // services::utils::verify(vk, proof, public_input.clone(), ic, BuiltinStorage::get()).await;

        // verified move
        let event = services::utils::panicking(move || {
            funcs::verified_move(
                MultipleGamesStorage::as_mut(),
                player,
                public_input.out,
                public_input.hit,
            )
        });
        services::utils::deposit_event(event);
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
            .map(|(player_1, player_2)| (*player_1, *player_2))
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
