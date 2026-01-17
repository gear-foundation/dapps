use crate::{
    admin::storage::{builtin_bls381::BuiltinStorage, verification_key::VerificationKeyStorage},
    services,
    services::session::{ActionsForSession, funcs::get_player, storage::SessionsStorage},
};
use core::fmt::Debug;
use gstd::{ActorId, Decode, Encode, String, TypeInfo, Vec, exec, ext, msg};
use sails_rs::gstd::service;
use sails_rs::{event, export};
pub use utils::*;

use self::storage::{GamePairsStorage, MultipleGamesStorage};

use super::admin::storage::configuration::ConfigurationStorage;

pub mod funcs;
pub mod storage;
pub(crate) mod utils;

#[event]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    GameCreated {
        player_id: ActorId,
    },
    JoinedTheGame {
        player_id: ActorId,
        game_id: ActorId,
    },
    PlacementVerified {
        admin: ActorId,
    },
    GameCanceled {
        game_id: ActorId,
    },
    GameLeft {
        game_id: ActorId,
    },
    MoveMade {
        game_id: ActorId,
        step: Option<u8>,
        verified_result: Option<(u8, StepResult)>,
        turn: ActorId,
    },
    EndGame {
        admin: ActorId,
        winner: ActorId,
        total_time: u64,
        participants_info: Vec<(ActorId, ParticipantInfo)>,
        last_hit: Option<u8>,
    },
    GameDeleted {
        game_id: ActorId,
    },
    PlayerDeleted {
        game_id: ActorId,
        removable_player: ActorId,
    },
}

#[derive(Clone)]
pub struct MultipleService(());

impl MultipleService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn seed() -> Self {
        let _res = MultipleGamesStorage::default();
        debug_assert!(_res.is_ok());
        let _res = GamePairsStorage::default();
        debug_assert!(_res.is_ok());
        Self(())
    }
}

#[service(events = Event)]
impl MultipleService {
    /// Creates a new game instance for a player and stores it in the game storage.
    ///
    /// # Arguments
    ///
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    #[export]
    pub fn create_game(&mut self, name: String, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::PlayMultipleGame,
        );
        let bid = msg::value();
        let player_id = services::utils::panicking(move || {
            funcs::create_game(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                ConfigurationStorage::get(),
                player,
                name,
                bid,
            )
        });

        self.emit_event(Event::GameCreated { player_id })
            .expect("Notification Error");
    }
    /// Joins an existing game with the specified game ID for a player and updates the game storage.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The `ActorId` representing the ID of the game to join.
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    #[export]
    pub fn join_game(
        &mut self,
        game_id: ActorId,
        name: String,
        session_for_account: Option<ActorId>,
    ) {
        let value = msg::value();
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
                name,
                game_id,
                value,
            )
        });
        self.emit_event(Event::JoinedTheGame { player_id, game_id })
            .expect("Notification Error");
    }
    /// Allows a player to leave a game and updates the game storage accordingly.
    ///
    /// # Arguments
    ///
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    #[export]
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

        self.emit_event(event).expect("Notification Error");
    }
    /// Cancels an existing game for a player and updates the game storage accordingly.
    ///
    /// # Arguments
    ///
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    #[export]
    pub fn cancel_game(&mut self, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::PlayMultipleGame,
        );
        let event = services::utils::panicking(move || {
            funcs::cancel_game(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                player,
            )
        });
        self.emit_event(event).expect("Notification Error");
    }

    /// Verifies the placement of ships in a multiplayer game using zero-knowledge proofs.
    ///
    /// # Arguments
    ///
    /// * `proof` - A zero-knowledge proof in the form of `ProofBytes`.
    /// * `public_input` - Public input data required for the verification process.
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    /// * `game_id` - The `ActorId` representing the ID of the game to verify.
    #[export]
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
                ConfigurationStorage::get(),
                player,
                game_id,
                public_input.hash,
                exec::block_timestamp(),
            )
        });
        self.emit_event(event).expect("Notification Error");
    }
    /// Executes a player's move in the game, including verification of the move if required.
    ///
    /// This function handles the following steps:
    /// 1. Validates that either verification variables or a step is provided.
    /// 2. Retrieves the player associated with the current session.
    /// 3. Checks the current game state to ensure the move is valid.
    /// 4. If verification variables are provided, it verifies the move using zk proof verification:
    ///     - Prepares input bytes for verification.
    ///     - Verifies the proof against the public inputs.
    ///     - If the proof is valid, processes the move and updates the game state.
    /// 5. If no verification is required, directly processes the move.
    /// 6. Sends a notification based on the event generated by the move.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The unique identifier of the game.
    /// * `verify_variables` - Optional verification data used for proof verification.
    /// * `step` - Optional step value representing the player's move.
    /// * `session_for_account` - Optional session identifier for the account making the move.
    #[export]
    pub async fn make_move(
        &mut self,
        game_id: ActorId,
        verify_variables: Option<services::verify::VerificationVariables>,
        step: Option<u8>,
        session_for_account: Option<ActorId>,
    ) {
        if verify_variables.is_none() && step.is_none() {
            ext::panic("Verification variables and step cannot be at the same time `None`")
        }
        // get player
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::ActionsForSession::PlayMultipleGame,
        );

        // check game state
        let cloned_verify_variables = verify_variables.clone();
        services::utils::panicking(move || {
            funcs::check_game_for_move(
                MultipleGamesStorage::as_ref(),
                game_id,
                player,
                cloned_verify_variables,
                step,
            )
        });

        let event = if let Some(services::verify::VerificationVariables {
            proof_bytes,
            public_input,
        }) = verify_variables
        {
            // get prepared inputs bytes
            let prepared_inputs_bytes = services::verify::get_move_prepared_inputs_bytes(
                public_input.clone(),
                VerificationKeyStorage::get_vk_for_move().ic.clone(),
            );

            // check proof
            services::verify::verify(
                VerificationKeyStorage::get_vk_for_move(),
                proof_bytes,
                prepared_inputs_bytes,
                BuiltinStorage::get(),
            )
            .await;

            // make move
            let verification_result = services::verify::VerificationResult {
                res: public_input.out,
                hit: public_input.hit,
            };
            services::utils::panicking(move || {
                funcs::make_move(
                    MultipleGamesStorage::as_mut(),
                    GamePairsStorage::as_mut(),
                    ConfigurationStorage::get(),
                    player,
                    game_id,
                    step,
                    Some(verification_result),
                    exec::block_timestamp(),
                )
            })
        } else {
            services::utils::panicking(move || {
                funcs::make_move(
                    MultipleGamesStorage::as_mut(),
                    GamePairsStorage::as_mut(),
                    ConfigurationStorage::get(),
                    player,
                    game_id,
                    step,
                    None,
                    exec::block_timestamp(),
                )
            })
        };
        self.emit_event(event).expect("Notification Error");
    }

    /// Deletes an existing game from the storage based on the game ID and creation time.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The `ActorId` representing the ID of the game to delete.
    /// * `create_time` - A 64-bit unsigned integer representing the creation time of the game.
    ///
    /// # Note
    /// The source of the message can only be the program itself.
    #[export]
    pub fn delete_game(&mut self, game_id: ActorId, create_time: u64) {
        if msg::source() != exec::program_id() {
            services::utils::panic("This message can be sent only by the program")
        }
        let event = services::utils::panicking(move || {
            funcs::delete_game(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                game_id,
                create_time,
            )
        });
        self.emit_event(event).expect("Notification Error");
    }

    /// Checks the timing of a game and updates the game state accordingly.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The `ActorId` representing the ID of the game to check.
    /// * `check_time` - A 64-bit unsigned integer representing the time to check against.
    ///     
    /// # Note
    /// The source of the message can only be the program itself.
    #[export]
    pub fn check_out_timing(&mut self, game_id: ActorId, check_time: u64) {
        if msg::source() != exec::program_id() {
            services::utils::panic("This message can be sent only by the program")
        }
        let possible_event = services::utils::panicking(move || {
            funcs::check_out_timing(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                game_id,
                check_time,
            )
        });

        if let Some(event) = possible_event {
            self.emit_event(event).expect("Notification Error");
        }
    }

    #[export]
    pub fn delete_player(
        &mut self,
        removable_player: ActorId,
        session_for_account: Option<ActorId>,
    ) {
        // get player
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            services::session::ActionsForSession::PlayMultipleGame,
        );
        let event = services::utils::panicking(move || {
            funcs::delete_player(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                player,
                removable_player,
            )
        });
        self.emit_event(event).expect("Notification Error");
    }

    #[export]
    pub fn games(&self) -> Vec<(ActorId, MultipleGameState)> {
        MultipleGamesStorage::as_ref()
            .iter()
            .map(|(actor_id, game)| {
                let game = MultipleGameState {
                    admin: game.admin,
                    participants_data: game.participants_data.clone().into_iter().collect(),
                    create_time: game.create_time,
                    start_time: game.start_time,
                    last_move_time: game.last_move_time,
                    status: game.status.clone(),
                    bid: game.bid,
                };
                (*actor_id, game)
            })
            .collect()
    }

    #[export]
    pub fn games_pairs(&self) -> Vec<(ActorId, ActorId)> {
        GamePairsStorage::as_ref()
            .iter()
            .map(|(player_1, player_2)| (*player_1, *player_2))
            .collect()
    }

    #[export]
    pub fn game(&self, player_id: ActorId) -> Option<MultipleGameState> {
        GamePairsStorage::as_ref()
            .get(&player_id)
            .and_then(|game_id| MultipleGamesStorage::as_ref().get(game_id))
            .map(|game| MultipleGameState {
                admin: game.admin,
                participants_data: game.participants_data.clone().into_iter().collect(),
                create_time: game.create_time,
                start_time: game.start_time,
                last_move_time: game.last_move_time,
                status: game.status.clone(),
                bid: game.bid,
            })
    }

    #[export]
    pub fn get_remaining_time(&self, player_id: ActorId) -> Option<u64> {
        let current_time = exec::block_timestamp();
        let time_to_move = ConfigurationStorage::get().delay_for_check_time as u64 * 3_000;
        GamePairsStorage::as_ref()
            .get(&player_id)
            .and_then(|game_id| MultipleGamesStorage::as_ref().get(game_id))
            .and_then(|game| time_to_move.checked_sub(current_time - game.last_move_time))
    }
}
