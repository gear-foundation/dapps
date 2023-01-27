#![no_std]

use gear_lib::non_fungible_token::token::{TokenId, TokenMetadata};
use gmeta::{InOut, Metadata};
use gstd::{errors::ContractError, prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = InOut<Initialize, Result<(), Error>>;
    type Handle = InOut<Action, Result<Event, Error>>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = State;
}

/// The contract state.
///
/// For more info about fields, see [`Initialize`].
#[derive(Encode, Decode, TypeInfo, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct State {
    pub items: Vec<(ItemId, ItemInfo)>,

    pub producers: Vec<ActorId>,
    pub distributors: Vec<ActorId>,
    pub retailers: Vec<ActorId>,

    pub fungible_token: ActorId,
    pub non_fungible_token: ActorId,

    /// Used by [`StateQuery::IsActionCached`]. Also see [`TransactionKind`].
    pub cached_actions: Vec<(ActorId, CachedAction)>,
}

#[doc(hidden)]
impl State {
    pub fn item_info(self, item_id: ItemId) -> Option<ItemInfo> {
        self.items
            .into_iter()
            .find_map(|(some_item_id, item_info)| (some_item_id == item_id).then_some(item_info))
    }

    pub fn participants(self) -> Participants {
        Participants {
            producers: self.producers,
            distributors: self.distributors,
            retailers: self.retailers,
        }
    }

    pub fn roles(self, actor: ActorId) -> Vec<Role> {
        let mut roles = vec![Role::Consumer];

        if self.producers.contains(&actor) {
            roles.push(Role::Producer);
        }
        if self.distributors.contains(&actor) {
            roles.push(Role::Distributor);
        }
        if self.retailers.contains(&actor) {
            roles.push(Role::Retailer);
        }

        roles
    }

    pub fn is_action_cached(self, actor: ActorId, action: InnerAction) -> bool {
        if let Some(action) = action.into() {
            self.cached_actions.contains(&(actor, action))
        } else {
            false
        }
    }
}

/// A counterpart for caching of some [`InnerAction`]'s variants.
///
/// See the source code of
/// [`impl From<InnerAction> for Option<CachedAction>`](enum.InnerAction.html#impl-From<InnerAction>-for-Option<CachedAction>)
/// to find out how the conversion works.
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CachedAction {
    Purchase(ItemId),
    PutUpForSale(ItemId),
    Approve(ItemId),
    Receive(ItemId),
    Other,
}

impl From<InnerAction> for Option<CachedAction> {
    fn from(action: InnerAction) -> Self {
        match action {
            InnerAction::Producer(ProducerAction::Produce { .. }) => Some(CachedAction::Other),
            InnerAction::Producer(ProducerAction::PutUpForSale { item_id, .. })
            | InnerAction::Distributor(DistributorAction::PutUpForSale { item_id, .. })
            | InnerAction::Retailer(RetailerAction::PutUpForSale { item_id, .. }) => {
                Some(CachedAction::PutUpForSale(item_id))
            }
            InnerAction::Producer(ProducerAction::Approve { item_id, .. })
            | InnerAction::Distributor(DistributorAction::Approve { item_id, .. }) => {
                Some(CachedAction::Approve(item_id))
            }
            InnerAction::Distributor(DistributorAction::Purchase { item_id, .. })
            | InnerAction::Retailer(RetailerAction::Purchase { item_id, .. })
            | InnerAction::Consumer(ConsumerAction::Purchase(item_id)) => {
                Some(CachedAction::Purchase(item_id))
            }
            InnerAction::Distributor(DistributorAction::Receive(item_id))
            | InnerAction::Retailer(RetailerAction::Receive(item_id)) => {
                Some(CachedAction::Receive(item_id))
            }
            _ => None,
        }
    }
}

/// The maximum number of items on a supply chain.
///
/// The limited number of items is required because this contract (like
/// all the others) has a limited amount of memory, so it can't store too many
/// items.
pub const MAX_NUMBER_OF_ITEMS: usize = 2usize.pow(17);

/// An item ID.
///
/// Should equal [`TokenId`] of an item's NFT.
pub type ItemId = TokenId;

/// Initializes the Supply chain contract.
///
/// # Requirements
/// - Each [`ActorId`] of `producers`, `distributors`, and `retailers` mustn't
/// equal [`ActorId::zero()`].
#[derive(Encode, Decode, Hash, TypeInfo, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Initialize {
    /// IDs of actors that'll have the right to interact with a supply chain on
    /// behalf of a producer.
    pub producers: Vec<ActorId>,
    /// IDs of actors that'll have the right to interact with a supply chain on
    /// behalf of a distributor.
    pub distributors: Vec<ActorId>,
    /// IDs of actors that'll have the right to interact with a supply chain on
    /// behalf of a retailer.
    pub retailers: Vec<ActorId>,

    /// A FT contract [`ActorId`].
    pub fungible_token: ActorId,
    /// An NFT contract [`ActorId`].
    pub non_fungible_token: ActorId,
}

/// Sends the contract info about what it should do.
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Action {
    pub action: InnerAction,
    pub kind: TransactionKind,
}

impl Action {
    pub fn new(action: InnerAction) -> Self {
        Self {
            action,
            kind: TransactionKind::New,
        }
    }

    pub fn to_retry(self) -> Self {
        Self {
            action: self.action,
            kind: TransactionKind::Retry,
        }
    }
}

/// A part of [`Action`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum InnerAction {
    Producer(ProducerAction),
    Distributor(DistributorAction),
    Retailer(RetailerAction),
    Consumer(ConsumerAction),
}

/// A part of [`Action`].
///
/// Determines how an action will be processed.
///
/// The contract has a transaction caching mechanism for a continuation of
/// partially processed asynchronous actions. Most often, the reason of an
/// underprocession is the lack of gas.
///
/// Important notes:
/// - Only the last sent asynchronous action for
/// [`msg::source()`](gstd::msg::source) is cached.
/// - Non-asynchronous actions are never cached.
/// - There's no guarantee every underprocessed asynchronous action will be
/// cached. Use [`StateQuery::IsActionCached`] to check if some action is cached
/// for some [`ActorId`].
/// - It's possible to send a retry action with a different payload, and it'll
/// continue with it because, for some action, not all payload is saved in the
/// cache (see [`CachedAction`]).
/// - The cache memory has a limit, so when it's reached every oldest cached
/// action is replaced with a new one.
#[derive(
    Default, Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash,
)]
pub enum TransactionKind {
    #[default]
    New,
    Retry,
}

/// Actions for a producer.
///
/// Should be used inside [`InnerAction::Producer`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ProducerAction {
    /// Produces one item and a corresponding NFT with given `token_metadata`.
    ///
    /// Transfers the created NFT for the item to a producer
    /// ([`msg::source()`]).
    ///
    /// # Requirements
    /// - [`msg::source()`] must be a producer in a supply chain.
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Produced`] & [`Role::Producer`].
    ///
    /// [`msg::source()`]: gstd::msg::source
    Produce { token_metadata: TokenMetadata },

    /// Puts a produced item up for sale to distributors for given `price` on
    /// behalf of a producer.
    ///
    /// Transfers an item's NFT to the Supply chain contract
    /// ([`exec::program_id()`](gstd::exec::program_id)).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the producer of the item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Produced`] &
    /// [`Role::Producer`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::ForSale`] & [`Role::Producer`].
    PutUpForSale { item_id: ItemId, price: u128 },

    /// Approves or not a distributor's purchase on behalf of a producer.
    ///
    /// If the purchase is approved, then item's [`ItemEventState`] changes to
    /// [`Approved`](ItemEventState::Approved) and, from that moment, an item
    /// can be shipped (by [`ProducerAction::Ship`]).
    ///
    /// If the purchase is **not** approved, then fungible tokens for it are
    /// refunded from the Supply chain contract
    /// ([`exec::program_id()`](gstd::exec::program_id)) to the item's
    /// distributor and item's [`ItemEventState`] changes back to
    /// [`ForSale`](ItemEventState::ForSale).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the producer of the item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Produced`] &
    /// [`Role::Distributor`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Approved`]/[`ItemEventState::ForSale`] &
    /// [`Role::Producer`].
    Approve {
        item_id: ItemId,
        /// Yes ([`true`]) or no ([`false`]).
        approve: bool,
    },

    /// Starts a shipping of a purchased item to a distributor on behalf of a
    /// producer.
    ///
    /// Starts the countdown for the delivery time specified for the item in
    /// [`DistributorAction::Purchase`].
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the producer of the item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Approved`] &
    /// [`Role::Producer`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Shipped`] & [`Role::Producer`].
    Ship(ItemId),
}

/// Actions for a distributor.
///
/// Should be used inside [`InnerAction::Distributor`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum DistributorAction {
    /// Purchases an item from a producer on behalf of a distributor.
    ///
    /// Transfers fungible tokens for purchasing the item to the Supply chain
    /// contract ([`exec::program_id()`](gstd::exec::program_id)) until the item
    /// is received (by [`DistributorAction::Receive`]).
    ///
    /// **Note:** the item's producer must approve or not this purchase by
    /// [`ProducerAction::Approve`].
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be a distributor.
    /// - Item's [`ItemState`] must contain [`ItemEventState::ForSale`] &
    /// [`Role::Producer`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Purchased`] & [`Role::Distributor`].
    Purchase {
        item_id: ItemId,
        /// Milliseconds during which the producer of an item should deliver it.
        /// A countdown starts after [`ProducerAction::Ship`] is executed.
        delivery_time: u64,
    },

    /// Receives a shipped item from a producer on behalf of a distributor.
    ///
    /// Depending on the time spent on a delivery, transfers fungible tokens for
    /// purchasing the item from the Supply chain contract
    /// ([`exec::program_id()`](gstd::exec::program_id)) to the item's producer
    /// or, as a penalty for being late, refunds a half or all of them to the
    /// item's distributor ([`msg::source()`]).
    ///
    /// Transfers an item's NFT to the distributor ([`msg::source()`]).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`] must be the distributor of the item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Shipped`] &
    /// [`Role::Producer`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Received`] & [`Role::Distributor`].
    ///
    /// [`msg::source()`]: gstd::msg::source
    Receive(ItemId),

    /// Processes a received item on behalf of a distributor.
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the distributor of the
    /// item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Received`] &
    /// [`Role::Distributor`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Processed`] & [`Role::Distributor`].
    Process(ItemId),

    /// Packages a processed item on behalf of a distributor.
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the distributor of the
    /// item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Processed`] &
    /// [`Role::Distributor`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Packaged`] & [`Role::Distributor`].
    Package(ItemId),

    /// Puts a packaged item up for sale to retailers for given `price` on
    /// behalf of a distributor.
    ///
    /// Transfers an item's NFT to the Supply chain contract
    /// ([`exec::program_id()`](gstd::exec::program_id)).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the distributor of the
    /// item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Packaged`] &
    /// [`Role::Distributor`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::ForSale`] & [`Role::Distributor`].
    PutUpForSale { item_id: ItemId, price: u128 },

    /// Approves or not a retailer's purchase on behalf of a distributor.
    ///
    /// If the purchase is approved, then item's [`ItemEventState`] changes to
    /// [`Approved`](ItemEventState::Approved) and, from that moment, an item
    /// can be shipped (by [`DistributorAction::Ship`]).
    ///
    /// If the purchase is **not** approved, then fungible tokens for it are
    /// refunded from the Supply chain contract
    /// ([`exec::program_id()`](gstd::exec::program_id)) to the item's retailer
    /// and item's [`ItemEventState`] changes back to
    /// [`ForSale`](ItemEventState::ForSale).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the distributor of the
    /// item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Purchased`] &
    /// [`Role::Retailer`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Approved`]/[`ItemEventState::ForSale`] &
    /// [`Role::Distributor`].
    Approve {
        item_id: ItemId,
        /// Yes ([`true`]) or no ([`false`]).
        approve: bool,
    },

    /// Starts a shipping of a purchased item to a retailer on behalf of a
    /// distributor.
    ///
    /// Starts the countdown for the delivery time specified for the item in
    /// [`RetailerAction::Purchase`].
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the distributor of the
    /// item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Approved`] &
    /// [`Role::Distributor`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Shipped`] & [`Role::Distributor`].
    Ship(ItemId),
}

/// Actions for a retailer.
///
/// Should be used inside [`InnerAction::Retailer`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum RetailerAction {
    /// Purchases an item from a distributor on behalf of a retailer.
    ///
    /// Transfers fungible tokens for purchasing the item to the Supply chain
    /// contract ([`exec::program_id()`](gstd::exec::program_id)) until the item
    /// is received (by [`RetailerAction::Receive`]).
    ///
    /// **Note:** the item's distributor must approve or not this purchase by
    /// [`DistributorAction::Approve`].
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be a retailer.
    /// - Item's [`ItemState`] must contain [`ItemEventState::ForSale`] &
    /// [`Role::Distributor`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Purchased`] & [`Role::Retailer`].
    Purchase {
        item_id: ItemId,
        /// Milliseconds during which the distributor of an item should deliver
        /// it. A countdown starts after [`DistributorAction::Ship`] is
        /// executed.
        delivery_time: u64,
    },

    /// Receives a shipped item from a distributor on behalf of a retailer.
    ///
    /// Depending on the time spent on a delivery, transfers fungible tokens for
    /// purchasing the item from the Supply chain contract
    /// ([`exec::program_id()`](gstd::exec::program_id)) to the item's
    /// distributor or, as a penalty for being late, refunds a half or all of
    /// them to the item's retailer ([`msg::source()`]).
    ///
    /// Transfers an item's NFT to the retailer ([`msg::source()`]).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`] must be the retailer of the item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Shipped`] &
    /// [`Role::Distributor`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Received`] & [`Role::Retailer`].
    ///
    /// [`msg::source()`]: gstd::msg::source
    Receive(ItemId),

    /// Puts a received item up for sale to consumers for given `price` on
    /// behalf of a retailer.
    ///
    /// Transfers an item's NFT to the Supply chain contract
    /// ([`exec::program_id()`](gstd::exec::program_id)).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - [`msg::source()`](gstd::msg::source) must be the retailer of the item.
    /// - Item's [`ItemState`] must contain [`ItemEventState::Received`] &
    /// [`Role::Retailer`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::ForSale`] & [`Role::Retailer`].
    PutUpForSale { item_id: ItemId, price: u128 },
}

/// Actions for a consumer.
///
/// Should be used inside [`InnerAction::Consumer`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum ConsumerAction {
    /// Purchases an item from a retailer.
    ///
    /// Transfers fungible tokens for purchasing the item to its retailer.
    ///
    /// Transfers an item's NFT to the consumer
    /// ([`msg::source()`](gstd::msg::source)).
    ///
    /// # Requirements
    /// - The item must exist in a supply chain.
    /// - Item's [`ItemState`] must contain [`ItemEventState::ForSale`] &
    /// [`Role::Retailer`].
    ///
    /// On success, replies with [`Event`] where [`ItemState`] contains
    /// [`ItemEventState::Purchased`] & [`Role::Consumer`].
    Purchase(ItemId),
}

/// A result of processed [`Action`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Event {
    pub item_id: ItemId,
    pub item_state: ItemState,
}

/// Contract execution error variants.
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
pub enum Error {
    /// [`ActorId::zero()`] was found where it's forbidden.
    ZeroActorId,
    /// An item with given [`ItemId`] doesn't exist in a supply chain.
    ItemNotFound,
    /// An item with given [`ItemId`] has an invalid state for a requested
    /// action.
    UnexpectedItemState,
    /// [`msg::source`](gstd::msg::source) doesn't have enough rights for a
    /// requested action or for an item with given [`ItemId`].
    AccessRestricted,
    /// The FT contract failed to complete a transfer transaction.
    ///
    /// Most often, the reason is that a user didn't give an approval to the
    /// Supply chain contract or didn't have enough tokens for a requested
    /// action.
    FTTransferFailed,
    /// The NFT contract failed to complete a transfer transaction.
    NFTTransferFailed,
    /// The NFT contract failed to complete a minting transaction.
    NFTMintingFailed,
    /// The contract reached a limit of protection against the memory overflow.
    MemoryLimitExceeded,
    /// See [`ContractError`].
    ContractError(String),
    TxCacheError(TransactionCacheError),
}

/// Transaction cache error variants.
///
/// Also see [`TransactionKind`].
#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, TypeInfo, Hash)]
pub enum TransactionCacheError {
    /// There's no cached transaction for
    /// [`msg::source()`](gstd::msg::source()). The reason may be a
    /// transaction's action wasn't asynchronous or just wasn't cached, or a
    /// cached transaction has been removed because it was completed or too old.
    TransactionNotFound,
    /// An action for retrying doesn't match its cached counterpart.
    MismatchedAction,
    /// Too many transaction IDs were acquired in one action. The maximum amount
    /// is 256.
    StepOverflow,
}

impl From<TransactionCacheError> for Error {
    fn from(error: TransactionCacheError) -> Self {
        Self::TxCacheError(error)
    }
}

impl From<ContractError> for Error {
    fn from(error: ContractError) -> Self {
        Self::ContractError(error.to_string())
    }
}

/// Roles of supply chain participants.
#[derive(
    Encode, Decode, TypeInfo, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash,
)]
pub enum Role {
    Producer,
    Distributor,
    Retailer,
    #[default]
    Consumer,
}

/// Queries the contract state.
///
/// Replies with [`StateReply`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum StateQuery {
    ItemInfo(ItemId),
    Participants,
    Roles(ActorId),
    ExistingItems,
    FungibleToken,
    NonFungibleToken,
    IsActionCached(ActorId, InnerAction),
}

/// A reply for [`StateQuery`].
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum StateReply {
    ItemInfo(Option<ItemInfo>),
    Participants(Participants),
    FungibleToken(ActorId),
    NonFungibleToken(ActorId),
    ExistingItems(Vec<(ItemId, ItemInfo)>),
    Roles(Vec<Role>),
    IsActionCached(bool),
}

/// Supply chain patricipants.
///
/// For more info about fields, see [`Initialize`].
#[derive(Encode, Decode, TypeInfo, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Participants {
    pub producers: Vec<ActorId>,
    pub distributors: Vec<ActorId>,
    pub retailers: Vec<ActorId>,
}

/// Item info.
#[derive(
    Encode, Decode, TypeInfo, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash,
)]
pub struct ItemInfo {
    /// Item’s producer [`ActorId`].
    pub producer: ActorId,
    /// [`ActorId`] of an item’s current or past distributor (depends on item’s
    /// `state`). If it equals [`ActorId::zero()`], then it means that an item
    /// has never had a distributor.
    pub distributor: ActorId,
    /// [`ActorId`] of an item’s current or past retailer (depends on item’s
    /// `state`). If it equals [`ActorId::zero()`], then it means that an item
    /// has never had a retailer.
    pub retailer: ActorId,

    pub state: ItemState,
    /// An item’s price. If it equals 0, then, depending on item’s `state`, an
    /// item is sold for free or has never been put up for sale.
    pub price: u128,
    /// Milliseconds during which a current seller should deliver an item.
    pub delivery_time: u64,
}

/// An item’s state.
#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct ItemState {
    pub state: ItemEventState,
    pub by: Role,
}

impl Default for ItemState {
    fn default() -> Self {
        Self {
            state: Default::default(),
            by: Role::Producer,
        }
    }
}

/// A part of [`ItemState`].
#[derive(
    Encode, Decode, TypeInfo, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash,
)]
pub enum ItemEventState {
    #[default]
    Produced,
    Purchased,
    Received,
    Processed,
    Packaged,
    ForSale,
    Approved,
    Shipped,
}
