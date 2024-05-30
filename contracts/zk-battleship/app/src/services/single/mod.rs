use self::storage::SingleGamesStorage;
use crate::admin::storage::{
    builtin_bls381::BuiltinStorage, verification_key::VerificationKeyStorage,
};
use crate::services;
use crate::services::session::storage::SessionsStorage;
use core::{fmt::Debug, marker::PhantomData};
use gstd::{msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
use sails_rtl::gstd::{
    events::{EventTrigger, GStdEventTrigger},
    gservice,
};

pub use utils::*;

pub mod funcs;
pub mod storage;
pub(crate) mod utils;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rtl::scale_codec)]
#[scale_info(crate = sails_rtl::scale_info)]
pub enum Event {
    SessionCreated,
    SessionDeleted,
    SingleGameStarted,
    EndGame(BattleshipParticipants),
    MoveMade {
        step_result: StepResult,
        bot_step: u8,
    },
    MoveVerified {
        step: u8,
        result: u8,
    },
}

pub type GstdDrivenService = Service<GStdEventTrigger<Event>>;

#[derive(Clone)]
pub struct Service<X>(PhantomData<X>);

impl<X> Service<X> {
    pub fn seed() -> Self {
        let _res = SingleGamesStorage::default();
        debug_assert!(_res.is_ok());
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

    pub async fn start_single_game(
        &mut self,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicStartInput,
        session_for_account: Option<ActorId>,
    ) {
        // get player
        let player = services::session::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::utils::ActionsForSession::PlaySingleGame,
        );

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

        // start single game
        services::utils::panicking(move || {
            funcs::start_single_game(SingleGamesStorage::as_mut(), player)
        });
        services::utils::deposit_event(Event::SingleGameStarted);
    }

    pub fn make_move(&mut self, step: u8, session_for_account: Option<ActorId>) {
        let player = services::session::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::utils::ActionsForSession::PlaySingleGame,
        );
        let event = services::utils::panicking(move || {
            funcs::make_move(SingleGamesStorage::as_mut(), player, step)
        });

        services::utils::deposit_event(event);
    }

    pub async fn verify_move(
        &mut self,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicMoveInput,
        session_for_account: Option<ActorId>,
    ) {
        // get player
        let player = services::session::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::utils::ActionsForSession::PlaySingleGame,
        );

        // check game state
        services::utils::panicking(move || {
            funcs::check_game(SingleGamesStorage::as_ref(), player, public_input.hit)
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

        let event = services::utils::panicking(move || {
            funcs::verified_move(
                SingleGamesStorage::as_mut(),
                player,
                public_input.out,
                public_input.hit,
            )
        });
        services::utils::deposit_event(event);
    }

    pub fn start_time(&self, player_id: ActorId) -> Option<u64> {
        crate::generate_getter_game!(start_time, player_id)
    }
    pub fn total_shots(&self, player_id: ActorId) -> Option<u64> {
        crate::generate_getter_game!(total_shots, player_id)
    }
    pub fn game_status(&self, player_id: ActorId) -> Option<Status> {
        crate::generate_getter_game!(status, player_id)
    }
    pub fn game(&self, player_id: ActorId) -> Option<SingleGame> {
        crate::generate_getter_game!(player_id)
    }

    pub fn games(&self) -> Vec<(ActorId, SingleGameState)> {
        SingleGamesStorage::as_ref()
            .iter()
            .map(|(actor_id, game)| {
                let game_state = SingleGameState {
                    player_board: game.player_board.clone(),
                    start_time: game.start_time,
                    status: game.status.clone(),
                    end_time: game.end_time,
                    total_shots: game.total_shots,
                };
                (*actor_id, game_state)
            })
            .collect()
    }
}
