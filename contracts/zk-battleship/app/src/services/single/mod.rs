use self::storage::SingleGamesStorage;
use crate::{
    admin::storage::{
        builtin_bls381::BuiltinStorage, configuration::ConfigurationStorage,
        verification_key::VerificationKeyStorage,
    },
    services,
    services::session::storage::SessionsStorage,
};
use core::fmt::Debug;
use gstd::{exec, ext, msg, ActorId, Decode, Encode, String, TypeInfo, Vec};
use sails_rs::{format, gstd::service, Box};

pub use utils::*;

pub mod funcs;
pub mod storage;
pub(crate) mod utils;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    SessionCreated,
    SingleGameStarted,
    EndGame {
        player: ActorId,
        winner: BattleshipParticipants,
        time: u64,
        total_shots: u8,
        succesfull_shots: u8,
        last_hit: Option<u8>,
    },
    MoveMade {
        player: ActorId,
        step: Option<u8>,
        step_result: Option<StepResult>,
        bot_step: Option<u8>,
    },
}

#[derive(Clone)]
pub struct SingleService(());

impl SingleService {
    pub fn seed() -> Self {
        let _res = SingleGamesStorage::default();
        debug_assert!(_res.is_ok());
        Self(())
    }
}

#[service(events = Event)]
impl SingleService {
    pub fn new() -> Self {
        Self(())
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
        services::utils::panicking(move || {
            funcs::start_single_game(
                SingleGamesStorage::as_mut(),
                player,
                public_input.hash,
                ConfigurationStorage::get(),
                exec::block_timestamp(),
            )
        });
        self.notify_on(Event::SingleGameStarted)
            .expect("Notification Error");
    }
    /// This function processes a move made by a player in a single-player game. It handles both
    /// regular moves and moves that require verification through a zero-knowledge proof (zk-proof).
    ///
    /// The function performs the following steps:
    /// 1. Validates the input to ensure that either a step or verification variables are provided.
    /// 2. Retrieves the `ActorId` of the player making the move, using session information.
    /// 3. If verification variables are provided, it performs the following sub-steps:
    ///    a. Validates the current game state to ensure that the move is allowed.
    ///    b. Prepares the input bytes required for zk-proof verification.
    ///    c. Verifies the move using zk-proof verification.
    ///    d. If the verification is successful, it processes the move by calling the `make_move` function
    ///       with the verified result.
    /// 4. If no verification is required, it directly processes the move by calling the `make_move` function.
    /// 5. Sends a notification containing the result of the move.
    ///
    /// # Arguments
    ///
    /// * `step` - An optional `u8` representing the move step made by the player.
    ///            If `None`, it indicates that a verification process is required.
    /// * `verify_variables` - An optional `VerificationVariables` struct containing
    ///                        proof bytes and public input required for zk-proof verification.
    /// * `session_for_account` - An optional `ActorId` representing the session account
    ///                           being used to make the move.
    pub async fn make_move(
        &mut self,
        step: Option<u8>,
        verify_variables: Option<services::verify::VerificationVariables>,
        session_for_account: Option<ActorId>,
    ) {
        if verify_variables.is_none() && step.is_none() {
            ext::panic("Verification variables and step cannot be at the same time `None`")
        }
        // get player ActorId
        let player = services::session::funcs::get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::utils::ActionsForSession::PlaySingleGame,
        );
        let event = if let Some(services::verify::VerificationVariables {
            proof_bytes,
            public_input,
        }) = verify_variables
        {
            // check game state
            let input = public_input.clone();
            services::utils::panicking(move || {
                funcs::check_game(SingleGamesStorage::as_ref(), player, input, step)
            });

            // get prepared inputs bytes
            let prepared_inputs_bytes = services::verify::get_move_prepared_inputs_bytes(
                public_input.clone(),
                VerificationKeyStorage::get_vk_for_move().ic.clone(),
            );

            // verify action
            services::verify::verify(
                VerificationKeyStorage::get_vk_for_move(),
                proof_bytes,
                prepared_inputs_bytes,
                BuiltinStorage::get(),
            )
            .await;
            // verified move after successful verification
            let verification_result = services::verify::VerificationResult {
                res: public_input.out,
                hit: public_input.hit,
            };
            services::utils::panicking(move || {
                funcs::make_move(
                    SingleGamesStorage::as_mut(),
                    player,
                    Some(verification_result),
                    step,
                    ConfigurationStorage::get(),
                    exec::block_timestamp(),
                )
            })
        } else {
            services::utils::panicking(move || {
                funcs::make_move(
                    SingleGamesStorage::as_mut(),
                    player,
                    None,
                    step,
                    ConfigurationStorage::get(),
                    exec::block_timestamp(),
                )
            })
        };
        self.notify_on(event).expect("Notification Error");
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

    pub fn check_out_timing(&mut self, actor_id: ActorId, check_time: u64) {
        if msg::source() != exec::program_id() {
            services::utils::panic("This message can be sent only by the program")
        }
        let event = services::utils::panicking(move || {
            funcs::check_out_timing(SingleGamesStorage::as_mut(), actor_id, check_time)
        });
        if let Some(event) = event {
            self.notify_on(event).expect("Notification Error");
        }
    }

    pub fn start_time(&self, player_id: ActorId) -> Option<u64> {
        crate::generate_getter_game!(start_time, player_id)
    }
    pub fn total_shots(&self, player_id: ActorId) -> Option<u8> {
        crate::generate_getter_game!(total_shots, player_id)
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
                    total_shots: game.total_shots,
                    succesfull_shots: game.succesfull_shots,
                    verification_requirement: game.verification_requirement,
                    last_move_time: game.last_move_time,
                };
                (*actor_id, game_state)
            })
            .collect()
    }
    pub fn get_remaining_time(&self, player_id: ActorId) -> Option<u64> {
        let current_time = exec::block_timestamp();
        let time_to_move = ConfigurationStorage::get().delay_for_check_time as u64 * 3_000;
        SingleGamesStorage::as_ref()
            .get(&player_id)
            .and_then(|game| time_to_move.checked_sub(current_time - game.last_move_time))
    }
}
