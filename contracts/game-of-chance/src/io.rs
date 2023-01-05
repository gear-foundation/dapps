use gstd::{errors::ContractError, prelude::*, ActorId};

/// The maximum number of participants for one game round.
///
/// The limited number of participants is required because this contract (like
/// all the others) has a limited amount of memory, so it can't store too many
/// participants.
pub const MAX_NUMBER_OF_PLAYERS: usize = 2usize.pow(16);

/// Initializes the Game of chance contract.
///
/// # Requirements
/// - `admin` mustn't be [`ActorId::zero()`].
#[derive(Debug, Default, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo)]
pub struct GOCInit {
    /// [`ActorId`] of the game administrator that'll have the rights to
    /// [start a game round](GOCAction::Start) and
    /// [pick a winner](GOCAction::PickWinner).
    pub admin: ActorId,
}

/// Sends a contract info about what it should do.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo)]
pub enum GOCAction {
    /// Starts a game round and allows to participate in it.
    ///
    /// # Requirements
    /// - [`msg::source()`](gstd::msg::source) must be the game administrator.
    /// - The current game round must be over.
    /// - `ft_actor_id` mustn't be [`ActorId::zero()`].
    ///
    /// On success, replies with [`GOCEvent::Started`].
    Start {
        /// The duration (in milliseconds) of the players entry stage.
        ///
        /// After that, no one will be able to enter a game round and a winner
        /// should be picked.
        duration: u64,
        /// The price of participation in a game round.
        participation_cost: u128,
        /// A currency (or FT contract [`ActorId`]) of a game round.
        ///
        /// Determines fungible tokens in which a prize fund and a participation
        /// cost will be collected. [`None`] means that the native value will be
        /// used instead of fungible tokens.
        ft_actor_id: Option<ActorId>,
    },

    /// Randomly picks a winner from current game round participants (players)
    /// and sends a prize fund to it.
    ///
    /// The randomness of a winner pick depends on
    /// [`exec::block_timestamp()`](gstd::exec::block_timestamp).
    /// Not the best source of entropy, but, in theory, it's impossible to
    /// exactly predict a winner if the time of an execution of this action is
    /// unknown.
    ///
    /// If no one participated in the round, then a winner will be
    /// [`ActorId::zero()`].
    ///
    /// # Requirements
    /// - [`msg::source()`](gstd::msg::source) must be the game administrator.
    /// - The players entry stage must be over.
    /// - A winner mustn't already be picked.
    ///
    /// On success, replies with [`GOCEvent::Winner`].
    PickWinner,

    /// Pays a participation cost and adds [`msg::source()`] to the current game
    /// round participants (players).
    ///
    /// A participation cost and its currency can be queried by the
    /// `meta_state()` entry function.
    ///
    /// # Requirements
    /// - The players entry stage mustn't be over.
    /// - [`msg::source()`] mustn't already participate.
    /// - [`msg::source()`] must have enough currency to pay a participation
    /// cost.
    /// - If the current game round currency is the native value (`ft_actor_id`
    /// is [`None`]), [`msg::source()`] must send this action with the amount of
    /// the value exactly equal to a participation cost.
    ///
    /// On success, replies with [`GOCEvent::PlayerAdded`].
    ///
    /// [`msg::source()`]: gstd::msg::source
    Enter,
}

/// A result of processed [`GOCAction`].
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo)]
pub enum GOCEvent {
    /// Should be returned from [`GOCAction::Start`].
    Started {
        /// The end time (in milliseconds) of the players entry stage.
        ///
        /// After that, the game administrator can pick a winner.
        ending: u64,
        /// See the documentation of [`GOCAction::Start`].
        participation_cost: u128,
        /// See the documentation of [`GOCAction::Start`].
        ft_actor_id: Option<ActorId>,
    },
    /// Should be returned from [`GOCAction::PickWinner`].
    Winner(ActorId),
    /// Should be returned from [`GOCAction::Enter`].
    PlayerAdded(ActorId),
}

/// Contract execution error variants.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub enum GOCError {
    /// [`msg::source()`](gstd::msg::source) isn't the administrator.
    AccessRestricted,
    /// The current game round wasn't in an expected game status.
    ///
    /// E.g. the game administrator can't pick a winner if the player entry
    /// stage isn't over, or an user can't entry a game round if the entry
    /// stage is over.
    UnexpectedGameStatus,
    /// [`ActorId::zero()`] was found where it's forbidden.
    ZeroActorId,
    /// The current FT contract failed to complete a transfer transaction.
    ///
    /// Most often, the reason is that a user didn't give an approval to the
    /// Game of chance contract or didn't have enough tokens for participating.
    TokenTransferFailed,
    /// The contract reached a limit of protection against the memory overflow.
    MemoryLimitExceeded,
    /// [`msg::source()`](gstd::msg::source) is already participating in the
    /// current game round.
    AlreadyParticipating,
    /// [`msg::source()`](gstd::msg::source) sent [`GOCAction::Enter`] with an
    /// incorrent amount of the native value.
    ///
    /// An user should set the value manually because the current game round is
    /// going without a FT contract (also see the [`GOCAction::Enter`]
    /// documentation).
    InvalidParticipationCost,
    /// See the [`ContractError`] documentation.
    ContractError(String),
}

impl From<ContractError> for GOCError {
    fn from(error: ContractError) -> Self {
        Self::ContractError(error.to_string())
    }
}

/// The current game round state.
#[derive(Debug, Default, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct GOCState {
    /// See the documentation of [`GOCInit`].
    pub admin: ActorId,
    /// The start time (in milliseconds) of the current game round and the
    /// players entry stage.
    pub started: u64,
    /// See the documentation of [`GOCEvent::Started`].
    pub ending: u64,
    /// Participants of the current game round.
    pub players: Vec<ActorId>,
    /// The current game round prize fund.
    ///
    /// It's calculated by multiplying `participation_cost` and the number
    /// of `players`.
    pub prize_fund: u128,
    /// See the documentation of [`GOCAction::Start`].
    pub participation_cost: u128,
    /// The winner of the current game round.
    pub winner: ActorId,
    /// A currency (or a FT contract [`ActorId`]) of the current game round.
    ///
    /// Also see the documentation of [`GOCAction::Start`].
    pub ft_actor_id: Option<ActorId>,
}
