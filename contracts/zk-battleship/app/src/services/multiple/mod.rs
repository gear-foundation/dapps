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

use super::admin::storage::configuration::ConfigurationStorage;

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
        step: u8,
        target_address: ActorId,
    },
    MoveVerified {
        admin: ActorId,
        opponent: ActorId,
        step: u8,
        result: u8,
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

pub type GstdDrivenService = Service<GStdEventTrigger<Event>>;

#[derive(Clone)]
pub struct Service<X>(PhantomData<X>);

impl<X> Service<X> {
    pub fn seed() -> Self {
        let _res = MultipleGamesStorage::default();
        debug_assert!(_res.is_ok());
        let _res = GamePairsStorage::default();
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
    /// Creates a new game instance for a player and stores it in the game storage.
    ///
    /// # Arguments
    ///
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
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

        services::utils::deposit_event(Event::GameCreated { player_id });
    }
    /// Joins an existing game with the specified game ID for a player and updates the game storage.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The `ActorId` representing the ID of the game to join.
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
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

        services::utils::deposit_event(Event::JoinedTheGame { player_id, game_id });
    }
    /// Allows a player to leave a game and updates the game storage accordingly.
    ///
    /// # Arguments
    ///
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
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
    /// Cancels an existing game for a player and updates the game storage accordingly.
    ///
    /// # Arguments
    ///
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
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

    /// Verifies the placement of ships in a multiplayer game using zero-knowledge proofs.
    ///
    /// # Arguments
    ///
    /// * `proof` - A zero-knowledge proof in the form of `ProofBytes`.
    /// * `public_input` - Public input data required for the verification process.
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    /// * `game_id` - The `ActorId` representing the ID of the game to verify.
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
        services::utils::deposit_event(event);
    }

    /// Makes a move in a multiplayer game and updates the game state accordingly.
    ///
    /// # Arguments
    ///
    /// * `game_id` - The `ActorId` representing the ID of the game where the move is made.
    /// * `step` - An unsigned 8-bit integer representing the move to be made.
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    pub fn make_move(&mut self, game_id: ActorId, step: u8, session_for_account: Option<ActorId>) {
        let player = get_player(
            SessionsStorage::as_ref(),
            msg::source(),
            &session_for_account,
            ActionsForSession::PlayMultipleGame,
        );

        let event = services::utils::panicking(move || {
            funcs::make_move(
                MultipleGamesStorage::as_mut(),
                ConfigurationStorage::get(),
                player,
                game_id,
                step,
                exec::block_timestamp(),
            )
        });

        services::utils::deposit_event(event);
    }

    /// Verifies a move in a multiplayer game using zero-knowledge proofs.
    ///
    /// # Arguments
    ///
    /// * `proof` - A zero-knowledge proof in the form of `ProofBytes`.
    /// * `public_input` - Public input data required for the move verification.
    /// * `session_for_account` - An optional `ActorId` representing an account session abstraction.
    /// * `game_id` - The `ActorId` representing the ID of the game to verify.
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

        // get prepared inputs bytes
        let prepared_inputs_bytes = services::verify::get_move_prepared_inputs_bytes(
            public_input.clone(),
            VerificationKeyStorage::get_vk_for_move().ic.clone(),
        );

        // check game state
        services::utils::panicking(move || {
            funcs::check_game_for_verify_move(
                MultipleGamesStorage::as_ref(),
                game_id,
                player,
                public_input.hit,
                public_input.hash,
            )
        });

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
                game_id,
                player,
                public_input.out,
                public_input.hit,
            )
        });
        services::utils::deposit_event(event);
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

        services::utils::deposit_event(event);
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
    pub fn check_out_timing(&mut self, game_id: ActorId, check_time: u64) {
        if msg::source() != exec::program_id() {
            services::utils::panic("This message can be sent only by the program")
        }
        services::utils::panicking(move || {
            funcs::check_out_timing(
                MultipleGamesStorage::as_mut(),
                GamePairsStorage::as_mut(),
                game_id,
                check_time,
            )
        });
    }

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

        services::utils::deposit_event(event);
    }

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
    pub fn games_pairs(&self) -> Vec<(ActorId, ActorId)> {
        GamePairsStorage::as_ref()
            .iter()
            .map(|(player_1, player_2)| (*player_1, *player_2))
            .collect()
    }
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
    pub fn get_remaining_time(&self, player_id: ActorId) -> Option<u64> {
        let current_time = exec::block_timestamp();
        let time_to_move = ConfigurationStorage::get().delay_for_check_time as u64 * 3_000;
        GamePairsStorage::as_ref()
            .get(&player_id)
            .and_then(|game_id| MultipleGamesStorage::as_ref().get(game_id))
            .and_then(|game| time_to_move.checked_sub(current_time - game.last_move_time))
    }
}
