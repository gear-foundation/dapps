use self::storage::SingleGamesStorage;
use crate::admin::storage::{
    builtin_bls381::BuiltinStorage, configuration::ConfigurationStorage,
    verification_key::VerificationKeyStorage,
};
use crate::services;
use crate::services::session::storage::SessionsStorage;
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
    SingleGameStarted,
    EndGame {
        player: ActorId,
        winner: BattleshipParticipants,
        time: u64,
        total_shots: u8,
        succesfull_shots: u8,
        last_hit: u8,
    },
    MoveMade {
        player: ActorId,
        step: u8,
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

    /// Function for creating a single-player game using Zero Knowledge (ZK) proofs.
    ///
    /// # Arguments
    ///
    /// * `proof` - Zero Knowledge proof represented as a byte array. Used to verify the correctness of the public input.
    /// * `public_input` - Public input data to start the game.
    /// * `session_for_account` - An optional parameter representing an account associated with the game session. This is an account abstraction that can be used for identification or session data storage.
    pub async fn start_single_game(
        &mut self,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicStartInput,
        session_for_account: Option<ActorId>,
    ) {
        // get player ActorId
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
        // verify action
        services::verify::verify(
            VerificationKeyStorage::get_vk_for_start(),
            proof,
            prepared_inputs_bytes,
            BuiltinStorage::get(),
        )
        .await;
        // start single game
        let config = ConfigurationStorage::get();
        services::utils::panicking(move || {
            funcs::start_single_game(
                SingleGamesStorage::as_mut(),
                player,
                public_input.hash,
                config.gas_for_delete_single_game,
                config.delay_for_delete_single_game,
            )
        });
        services::utils::deposit_event(Event::SingleGameStarted);
    }

    /// Function for making a move in the game.
    ///
    /// # Arguments
    ///
    /// * `step` - A step or move to be made in the game, represented as an 8-bit unsigned integer. It denotes the position where the shot will be made.
    /// * `session_for_account` - An optional parameter representing the account associated with the game session. It is an abstraction of the account that can be used to identify or store session data.
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

    /// Function for verifying a move in the game using Zero Knowledge (ZK) proofs.
    ///
    /// # Arguments
    ///
    /// * `proof` - Zero Knowledge proof represented as a byte array. Used to verify the correctness of the move.
    /// * `public_input` - Public input data for the move.
    /// * `session_for_account` - An optional parameter representing an account associated with the game session. This is an account abstraction that can be used for identification or session data storage.
    pub async fn verify_move(
        &mut self,
        proof: services::verify::ProofBytes,
        public_input: services::verify::PublicMoveInput,
        session_for_account: Option<ActorId>,
    ) {
        // get player ActorId
        let player = services::session::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::utils::ActionsForSession::PlaySingleGame,
        );

        // get prepared inputs bytes
        let prepared_inputs_bytes = services::verify::get_move_prepared_inputs_bytes(
            public_input.clone(),
            VerificationKeyStorage::get_vk_for_move().ic.clone(),
        );

        // check game state
        let input = public_input.clone();
        services::utils::panicking(move || {
            funcs::check_game(SingleGamesStorage::as_ref(), player, input)
        });

        // verify action
        services::verify::verify(
            VerificationKeyStorage::get_vk_for_move(),
            proof,
            prepared_inputs_bytes,
            BuiltinStorage::get(),
        )
        .await;
        // verified move after successful verification
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

    /// Function for deleting a game. This function is called by a delayed message from the program itself
    /// to delete the game after a certain time, thereby cleaning up the program's state.
    ///
    /// # Arguments
    ///
    /// * `player` - The `ActorId` representing the player associated with the game to be deleted.
    /// * `start_time` - The start time of the game, represented as a 64-bit unsigned integer. This is used to identify the specific game instance to be deleted.
    ///
    /// # Note
    ///
    /// This function checks that the message source is the program itself (`exec::program_id()`).
    /// If not, it panics with a message indicating that the function can only be called by the program.
    pub fn delete_game(&mut self, player: ActorId, start_time: u64) {
        if msg::source() != exec::program_id() {
            services::utils::panic("This message can be sent only by the program")
        }
        services::utils::panicking(move || {
            funcs::delete_game(SingleGamesStorage::as_mut(), player, start_time)
        });
    }

    pub fn start_time(&self, player_id: ActorId) -> Option<u64> {
        crate::generate_getter_game!(start_time, player_id)
    }
    pub fn total_shots(&self, player_id: ActorId) -> Option<u8> {
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
                    ship_hash: game.ship_hash.clone(),
                    start_time: game.start_time,
                    status: game.status.clone(),
                    total_shots: game.total_shots,
                    succesfull_shots: game.succesfull_shots,
                };
                (*actor_id, game_state)
            })
            .collect()
    }
}
