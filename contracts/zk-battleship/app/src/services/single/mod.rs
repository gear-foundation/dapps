use self::storage::{
    admin::AdminStorage, builtin_bls381::BuiltinStorage, verification_key::VerificationKeyStorage,
    SessionsStorage, SingleGamesStorage,
};
use crate::services;
use core::{fmt::Debug, marker::PhantomData};
use gstd::{exec, msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
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
    pub fn seed(
        builtin_bls381: ActorId,
        verification_key_for_start: services::verify::VerifyingKeyBytes,
        verification_key_for_move: services::verify::VerifyingKeyBytes,
    ) -> Self {
        let _res = SingleGamesStorage::default();
        debug_assert!(_res.is_ok());
        let _res = SessionsStorage::default();
        debug_assert!(_res.is_ok());
        let source = msg::source();
        let _res = AdminStorage::set(source);
        debug_assert!(_res.is_ok());
        let _res = BuiltinStorage::set(builtin_bls381);
        debug_assert!(_res.is_ok());
        let _res =
            VerificationKeyStorage::set(verification_key_for_start, verification_key_for_move);
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
    pub fn create_session(
        &mut self,
        key: ActorId,
        duration: u64,
        allowed_actions: Vec<ActionsForSession>,
    ) {
        services::utils::panicking(move || {
            funcs::create_session(
                SessionsStorage::as_mut(),
                msg::source(),
                exec::block_timestamp(),
                key,
                duration,
                allowed_actions,
            )
        });
        services::utils::deposit_event(Event::SessionCreated);
    }

    pub fn delete_session(&mut self) {
        services::utils::panicking(move || {
            funcs::delete_session(SessionsStorage::as_mut(), msg::source())
        });
        services::utils::deposit_event(Event::SessionDeleted);
    }

    pub async fn start_single_game(
        &mut self,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicStartInput,
        session_for_account: Option<ActorId>,
    ) {
        // get player
        let player = services::single::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::StartSingleGame,
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
        let player = services::single::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::Move,
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
        let player = services::single::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::VerifyMove,
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

    pub fn delete_game(&mut self, player_address: ActorId) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the admin can change the program state"
        );
        SingleGamesStorage::as_mut().remove(&player_address);
    }
    pub fn change_admin(&mut self, new_admin: ActorId) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the administrator can change the configuration"
        );
        let admin = AdminStorage::get_mut();
        *admin = new_admin;
    }
    pub fn change_builtin_address(&mut self, new_builtin_address: ActorId) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the administrator can change the configuration"
        );
        let builtin = BuiltinStorage::get_mut();
        *builtin = new_builtin_address;
    }
    pub fn change_verification_key(
        &mut self,
        new_vk_for_start: Option<services::verify::VerifyingKeyBytes>,
        new_vk_for_move: Option<services::verify::VerifyingKeyBytes>,
    ) {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the administrator can change the configuration"
        );
        if let Some(new_vk) = new_vk_for_start {
            let vk_for_start = VerificationKeyStorage::get_mut_vk_for_start();
            *vk_for_start = new_vk;
        }
        if let Some(new_vk) = new_vk_for_move {
            let vk_for_move = VerificationKeyStorage::get_mut_vk_for_move();
            *vk_for_move = new_vk;
        }
    }

    pub fn start_time(&self, player_id: ActorId) -> Option<u64> {
        crate::generate_getter_game!(start_time, player_id)
    }
    pub fn total_shots(&self, player_id: ActorId) -> Option<u64> {
        crate::generate_getter_game!(total_shots, player_id)
    }
    pub fn game_result(&self, player_id: ActorId) -> Option<Option<BattleshipParticipants>> {
        crate::generate_getter_game!(result, player_id)
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
                    result: game.result.clone(),
                };
                (*actor_id, game_state)
            })
            .collect()
    }

    pub fn admin(&self) -> ActorId {
        AdminStorage::get()
    }
    pub fn builtin(&self) -> ActorId {
        BuiltinStorage::get()
    }
}
