use crate::admin::storage::{
    builtin_bls381::BuiltinStorage, verification_key::VerificationKeyStorage,
};
use crate::services;
use crate::services::session::storage::SessionsStorage;
use crate::services::session::{funcs::get_player, ActionsForSession};
use core::{fmt::Debug, marker::PhantomData};
use gstd::{exec, msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
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
    PlacementVerified,
    GameCanceled {
        game_id: ActorId,
    },
    GameLeft {
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
            ActionsForSession::PlayMultipleGame,
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
            ActionsForSession::PlayMultipleGame,
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
    pub fn leave_game(&mut self, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::PlayMultipleGame,
        );
        let event = services::utils::panicking(move || {
            funcs::leave_game(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                player,
            )
        });

        services::utils::deposit_event(event);
    }
    pub fn cancel_game(&mut self, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::PlayMultipleGame,
        );
        let game_id = services::utils::panicking(move || {
            funcs::cancel_game(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                player,
            )
        });

        services::utils::deposit_event(Event::GameCanceled { game_id });
    }
    pub async fn verify_placement(
        &mut self,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicStartInput,
        session_for_account: Option<ActorId>,
        game_id: ActorId,
    ) {
        // get player
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::utils::ActionsForSession::PlayMultipleGame,
        );

        // check game state
        services::utils::panicking(move || {
            funcs::check_game_for_verify_placement(MultipleGamesStorage::as_ref(), player, game_id)
        });

        // get prepared inputs bytes
        let prepared_inputs_bytes = services::verify::get_start_prepared_inputs_bytes(
            public_input.clone(),
            VerificationKeyStorage::get_vk_for_start().ic.clone(),
        );

        // verify
        services::verify::verify(
            VerificationKeyStorage::get_vk_for_start(),
            proof,
            prepared_inputs_bytes,
            BuiltinStorage::get(),
        )
        .await;

        // set verify placement
        let event = services::utils::panicking(move || {
            funcs::set_verify_placement(
                MultipleGamesStorage::as_mut(),
                player,
                game_id,
                exec::block_timestamp(),
            )
        });
        services::utils::deposit_event(event);
    }

    pub fn make_move(&mut self, game_id: ActorId, step: u8, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::PlayMultipleGame,
        );

        let event = services::utils::panicking(move || {
            funcs::make_move(MultipleGamesStorage::as_mut(), player, game_id, step)
        });

        services::utils::deposit_event(event);
    }
    pub async fn verify_move(
        &mut self,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicMoveInput,
        session_for_account: Option<ActorId>,
        game_id: ActorId,
    ) {
        // get player
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::ActionsForSession::PlayMultipleGame,
        );

        // check game state
        services::utils::panicking(move || {
            funcs::check_game_for_verify_move(
                MultipleGamesStorage::as_ref(),
                game_id,
                player,
                public_input.hit,
            )
        });

        // get prepared inputs bytes
        let prepared_inputs_bytes = services::verify::get_move_prepared_inputs_bytes(
            public_input.clone(),
            VerificationKeyStorage::get_vk_for_move().ic.clone(),
        );

        // check proof
        services::verify::verify(
            VerificationKeyStorage::get_vk_for_move(),
            proof,
            prepared_inputs_bytes,
            BuiltinStorage::get(),
        )
        .await;

        // verified move
        let event = services::utils::panicking(move || {
            funcs::verified_move(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
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
