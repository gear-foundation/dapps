use self::storage::{
    admin::AdminStorage, builtin_bls381::BuiltinStorage, SessionsStorage, SingleGamesStorage,
};
use crate::services;
use crate::single::verify::get_prepared_inputs_bytes;
use crate::single::verify::PublicInput;
use core::{fmt::Debug, marker::PhantomData};
use gstd::{debug, exec, msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
use sails_rtl::gstd::{
    events::{EventTrigger, GStdEventTrigger},
    gservice,
};

pub use utils::*;

pub mod funcs;
pub mod storage;
pub(crate) mod utils;
pub mod verify;

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
    MoveVerified,
}

pub type GstdDrivenService = Service<GStdEventTrigger<Event>>;

#[derive(Clone)]
pub struct Service<X>(PhantomData<X>);

impl<X> Service<X> {
    pub fn seed(builtin_bls381: ActorId) -> Self {
        debug!("efvkle");
        let _res = SingleGamesStorage::default();
        debug_assert!(_res.is_ok());
        let _res = SessionsStorage::default();
        debug_assert!(_res.is_ok());
        let source = msg::source();
        let _res = AdminStorage::set(source);
        debug_assert!(_res.is_ok());
        let _res = BuiltinStorage::set(builtin_bls381);
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
    ) -> () {
        let msg_source = msg::source();
        let block_timestamp = exec::block_timestamp();
        services::utils::panicking(move || {
            funcs::create_session(
                SessionsStorage::as_mut(),
                msg_source,
                block_timestamp,
                key,
                duration,
                allowed_actions,
            )
        });
        services::utils::deposit_event(Event::SessionCreated);
    }

    pub fn delete_session(&mut self) -> () {
        let msg_source = msg::source();
        services::utils::panicking(move || {
            funcs::delete_session(SessionsStorage::as_mut(), msg_source)
        });
        services::utils::deposit_event(Event::SessionDeleted);
    }

    pub fn start_single_game(&mut self, session_for_account: Option<ActorId>) -> () {
        let msg_source = msg::source();
        services::utils::panicking(move || {
            funcs::start_single_game(
                SessionsStorage::as_ref(),
                SingleGamesStorage::as_mut(),
                msg_source,
                session_for_account,
            )
        });
        services::utils::deposit_event(Event::SingleGameStarted);
    }

    pub fn make_move(&mut self, step: u8, session_for_account: Option<ActorId>) -> () {
        let msg_source = msg::source();
        let event = services::utils::panicking(move || {
            funcs::make_move(
                SessionsStorage::as_ref(),
                SingleGamesStorage::as_mut(),
                msg_source,
                step,
                session_for_account,
            )
        });

        services::utils::deposit_event(event);
    }

    pub async fn verify_move(
        &mut self,
        vk: verify::VerifyingKeyBytes,
        proof: verify::ProofBytes,
        public_input: PublicInput,
        ic: [Vec<u8>; 4],
        session_for_account: Option<ActorId>,
    ) -> () {
        let msg_source = msg::source();

        let prepared_inputs_bytes = get_prepared_inputs_bytes(public_input.clone(), ic);

        let res = funcs::verify_move(
            vk,
            proof,
            prepared_inputs_bytes,
            SessionsStorage::as_ref(),
            SingleGamesStorage::as_mut(),
            BuiltinStorage::get(),
            msg_source,
            public_input.out,
            public_input.hit,
            session_for_account,
        )
        .await;

        match res {
            Ok(event) => services::utils::deposit_event(event),
            Err(error) => services::utils::panic(error),
        };
    }

    pub fn delete_game(&mut self, player_address: ActorId) -> () {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the admin can change the program state"
        );
        SingleGamesStorage::as_mut().remove(&player_address);
    }
    pub fn change_admin(&mut self, new_admin: ActorId) -> () {
        assert!(
            msg::source() == AdminStorage::get(),
            "Only the admin can change the program state"
        );
        let admin = AdminStorage::get_mut();
        *admin = new_admin;
    }

    pub fn start_time(&self, player_id: ActorId) -> Option<u64> {
        if let Some(game) = SingleGamesStorage::as_ref().get(&player_id) {
            Some(game.start_time)
        } else {
            None
        }
    }

    pub fn total_shots(&self, player_id: ActorId) -> Option<u64> {
        if let Some(game) = SingleGamesStorage::as_ref().get(&player_id) {
            Some(game.total_shots)
        } else {
            None
        }
    }

    pub fn game_result(&self, player_id: ActorId) -> Option<BattleshipParticipants> {
        if let Some(game) = SingleGamesStorage::as_ref().get(&player_id) {
            game.result.clone()
        } else {
            None
        }
    }
    pub fn game_status(&self, player_id: ActorId) -> Option<Status> {
        if let Some(game) = SingleGamesStorage::as_ref().get(&player_id) {
            Some(game.status.clone())
        } else {
            None
        }
    }
    pub fn game(&self, player_id: ActorId) -> Option<SingleGame> {
        if let Some(game) = SingleGamesStorage::as_ref().get(&player_id) {
            Some(game.clone())
        } else {
            None
        }
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
}
