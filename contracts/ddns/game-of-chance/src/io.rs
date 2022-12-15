use gstd::{prelude::*, ActorId};

/// Initializes the Game of chance contract.
///
/// # Requirements
/// - `admin` mustn't be [`ActorId::zero()`].
#[derive(Debug, Default, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct GOCInit {
    /// [`ActorId`] of the game administrator that'll have the rights to
    /// [start a game round](GOCAction::Start) and
    /// [pick a winner](GOCAction::PickWinner).
    pub admin: ActorId,
}

#[derive(Debug, Default, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct DnsMeta {
    pub name: String,
    pub link: String,
    pub description: String,
}

/// Sends a contract info about what it should do.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub enum GOCAction {
    GetDnsMeta,
    SetDnsMeta(DnsMeta),
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
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub enum GOCEvent {
    DnsMeta(Option<DnsMeta>),
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

/// The current game round state.
#[derive(Debug, Default, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo)]
pub struct GOCState {
    /// See the documentation of [`GOCInit`].
    pub admin: ActorId,
    /// The start time (in milliseconds) of the current game round and the
    /// players entry stage.
    ///
    /// If it equals 0, a winner has picked and the round is over.
    pub started: u64,
    /// See the documentation of [`GOCEvent::Started`].
    pub ending: u64,
    /// Participants of the current game round.
    pub players: BTreeSet<ActorId>,
    /// The current game round prize fund.
    ///
    /// It's calculated by multiplying `participation_cost` and the number
    /// of `players`.
    pub prize_fund: u128,
    /// See the documentation of [`GOCAction::Start`].
    pub participation_cost: u128,
    /// The winner of the current game round.
    ///
    /// If it doesn't equal [`ActorId::zero()`], a winner has picked and the
    /// round is over.
    pub winner: ActorId,
    /// A currency (or a FT contract [`ActorId`]) of the current game round.
    ///
    /// See the documentation of [`GOCAction::Start`].
    pub ft_actor_id: Option<ActorId>,
}
