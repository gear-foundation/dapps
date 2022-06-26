#![no_std]

use gear_lib::non_fungible_token::token::TokenId;
use gstd::{prelude::*, ActorId};

pub type ItemId = TokenId;

/// Initializes a supply chain.
///
/// # Requirements
/// * There mustn't be the zero address among `producers`, `distributors`, and `retailers` addresses.
#[derive(Encode, Decode, TypeInfo)]
pub struct InitSupplyChain {
    /// Producers addresses who'll have a right to interact with a supply chain.
    pub producers: BTreeSet<ActorId>,
    /// Distributors addresses who'll have a right to interact with a supply chain.
    pub distributors: BTreeSet<ActorId>,
    /// Retailers addresses who'll have a right to interact with a supply chain.
    pub retailers: BTreeSet<ActorId>,

    /// A FT program ID.
    pub ft_program_id: ActorId,
    /// An NFT program ID.
    pub nft_program_id: ActorId,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum SupplyChainAction {
    /// Produces one item with a name and description and replies with its ID.
    ///
    /// Transfers created NFT for an item to a producer.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a producer in a supply chain.
    ///
    /// On success, returns [`SupplyChainEvent::Produced`].
    Produce {
        /// An item's name.
        name: String,
        /// An item's description.
        description: String,
    },

    /// Puts an item up for a sale to a distributor for a given price
    /// on behalf of a producer.
    ///
    /// Transfers item's NFT to a supply chain.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a producer in a supply chain
    /// and a producer of this item.
    /// * Item's [`ItemState`] must be [`Produced`](ItemState::Produced).
    ///
    /// On success, returns [`SupplyChainEvent::ForSaleByProducer`].
    PutUpForSaleByProducer {
        /// An item's ID.
        item_id: ItemId,
        /// An item's price.
        price: u128,
    },

    /// Purchases an item from a producer on behalf of a distributor.
    ///
    /// Transfers tokens for purchasing an item to a supply chain
    /// until an item is received (by [`SupplyChainAction::ReceiveByDistributor`]).
    ///
    /// Note that an item's producer must approve or not this purchase by
    /// [`SupplyChainAction::ApproveByProducer`].
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain.
    /// * Item's [`ItemState`] must be [`ForSaleByProducer`](ItemState::ForSaleByProducer).
    ///
    /// On success, returns [`SupplyChainEvent::PurchasedByDistributor`].
    PurchaseByDistributor {
        /// An item's ID.
        item_id: ItemId,
        /// A time in milliseconds for which a producer must deliver an item.
        /// A countdown starts after [`SupplyChainAction::ShipByProducer`] is executed.
        delivery_time: u64,
    },

    /// Approves or not a purchase from a distributor on behalf of a producer.
    ///
    /// If a purchase is approved, then item's [`ItemState`] changes to
    /// [`ApprovedByProducer`](ItemState::ApprovedByProducer) and an item can be shipped.
    ///
    /// If a purchase is **not** approved, then tokens for a purchase are refunded to an item's producer.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a producer in a supply chain
    /// and a producer of this item.
    /// * Item's [`ItemState`] must be [`PurchasedByDistributor`](ItemState::PurchasedByDistributor).
    ///
    /// On success, returns [`SupplyChainEvent::ApprovedByProducer`].
    ApproveByProducer {
        /// An item's ID.
        item_id: ItemId,
        /// Yes ([`true`]) or no ([`false`]).
        approve: bool,
    },

    /// Starts shipping a purchased item to a distributor on behalf of a producer.
    ///
    /// Starts a countdown for a delivery time that was specified for this item in
    /// [`SupplyChainAction::PurchaseByDistributor`].
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a producer in a supply chain
    /// and a producer of this item.
    /// * Item's [`ItemState`] must be [`PurchasedByDistributor`](ItemState::PurchasedByDistributor).
    ///
    /// On success, returns [`SupplyChainEvent::ShippedByProducer`].
    ShipByProducer(
        /// An item's ID.
        ItemId,
    ),

    /// Receives a shipped item from a producer on behalf of a distributor.
    ///
    /// Depending on a counted delivery time, transfers tokens for purchasing an item
    /// from a supply chain to a producer or as a penalty for being late refunds some or
    /// all of them to a distributor.
    ///
    /// Transfers item's NFT to a distributor.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`ShippedByProducer`](ItemState::ShippedByProducer).
    ///
    /// On success, returns [`SupplyChainEvent::ReceivedByDistributor`].
    ReceiveByDistributor(
        /// An item's ID.
        ItemId,
    ),

    /// Processes a received item from a producer on behalf of a distributor.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`ReceivedByDistributor`](ItemState::ReceivedByDistributor).
    ///
    /// On success, returns [`SupplyChainEvent::ProcessedByDistributor`].
    ProcessByDistributor(
        /// An item's ID.
        ItemId,
    ),

    /// Packages a processed item on behalf of a distributor.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`ProcessedByDistributor`](ItemState::ProcessedByDistributor).
    ///
    /// On success, returns [`SupplyChainEvent::PackagedByDistributor`].
    PackageByDistributor(
        /// An item's ID.
        ItemId,
    ),

    /// Puts a packaged item up for a sale to a retailer
    /// for a given price on behalf of a distributor.
    ///
    /// Transfers item's NFT to a supply chain.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`PackagedByDistributor`](ItemState::PackagedByDistributor).
    ///
    /// On success, returns [`SupplyChainEvent::ForSaleByDistributor`].
    PutUpForSaleByDistributor {
        /// An item's ID.
        item_id: ItemId,
        /// An item's price.
        price: u128,
    },

    /// Purchases an item from a distributor on behalf of a retailer.
    ///
    /// Transfers tokens for purchasing an item to a supply chain
    /// until an item is received (by [`SupplyChainAction::ReceiveByRetailer`]).
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a retailer in a supply chain.
    /// * Item's [`ItemState`] must be [`ForSaleByDistributor`](ItemState::ForSaleByDistributor).
    ///
    /// On success, returns [`SupplyChainEvent::PurchasedByRetailer`].
    PurchaseByRetailer {
        /// An item's ID.
        item_id: ItemId,
        /// A time in milliseconds for which a distributor must deliver an item.
        /// A countdown starts after [`SupplyChainAction::ShipByDistributor`] is executed.
        delivery_time: u64,
    },

    /// Approves or not a purchase from a retailer on behalf of a distributor.
    ///
    /// If a purchase is approved, then item's [`ItemState`] changes to
    /// [`ApprovedByDistributor`](ItemState::ApprovedByDistributor) and an item can be shipped.
    ///
    /// If a purchase is **not** approved, then tokens for a purchase are refunded to an item's distributor.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`PurchasedByRetailer`](ItemState::PurchasedByRetailer).
    ///
    /// On success, returns [`SupplyChainEvent::ApprovedByDistributor`].
    ApproveByDistributor {
        /// An item's ID.
        item_id: ItemId,
        /// Yes ([`true`]) or no ([`false`]).
        approve: bool,
    },

    /// Starts shipping a purchased item to a retailer on behalf of a distributor.
    ///
    /// Starts a countdown for a delivery time that was specified for this item in
    /// [`SupplyChainAction::PurchaseByRetailer`].
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`PurchasedByRetailer`](ItemState::PurchasedByRetailer).
    ///
    /// On success, returns [`SupplyChainEvent::ShippedByDistributor`].
    ShipByDistributor(
        /// An item's ID.
        ItemId,
    ),

    /// Receives a shipped item from a distributor on behalf of a retailer.
    ///
    /// Depending on a counted delivery time, transfers tokens for purchasing an item
    /// from a supply chain to a distributor or as a penalty for being late refunds some or
    /// all of them to a retailer.
    ///
    /// Transfers item's NFT to a retailer.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a retailer in a supply chain
    /// and a retailer of this item.
    /// * Item's [`ItemState`] must be [`ShippedByDistributor`](ItemState::ShippedByDistributor).
    ///
    /// On success, returns [`SupplyChainEvent::ReceivedByRetailer`].
    ReceiveByRetailer(
        /// An item's ID.
        ItemId,
    ),

    /// Puts a received item from a distributor up for a sale to a consumer
    /// for a given price on behalf of a retailer.
    ///
    /// Transfers item's NFT to a supply chain.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a retailer in a supply chain
    /// and a retailer of this item.
    /// * Item's [`ItemState`] must be [`ReceivedByRetailer`](ItemState::ReceivedByRetailer).
    ///
    /// On success, returns [`SupplyChainEvent::ForSaleByRetailer`].
    PutUpForSaleByRetailer {
        /// An item's ID.
        item_id: ItemId,
        /// An item's price.
        price: u128,
    },

    /// Purchases an item from a retailer.
    ///
    /// Transfers tokens for purchasing an item to its retailer.
    ///
    /// Transfers item's NFT to a consumer.
    ///
    /// # Requirements
    /// * Item's [`ItemState`] must be [`ForSaleByRetailer`](ItemState::ForSaleByRetailer).
    ///
    /// On success, returns [`SupplyChainEvent::PurchasedByConsumer`].
    PurchaseByConsumer(
        /// An item's ID.
        ItemId,
    ),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum SupplyChainEvent {
    Produced(
        /// An ID of a produced item.
        ItemId,
    ),
    ForSaleByProducer(
        /// An ID of an item put up for a sale by a producer.
        ItemId,
    ),
    PurchasedByDistributor(
        /// An ID of an item purchased by a distributor.
        ItemId,
    ),
    ApprovedByProducer(
        /// An ID of an item approved by a producer for a purchase.
        ItemId,
    ),
    ShippedByProducer(
        /// An ID of an item shipped by a producer.
        ItemId,
    ),
    ReceivedByDistributor(
        /// An ID of an item received by a distributor.
        ItemId,
    ),
    ProcessedByDistributor(
        /// An ID of an item processed by a distributor.
        ItemId,
    ),
    PackagedByDistributor(
        /// An ID of an item packaged by a distributor.
        ItemId,
    ),
    ForSaleByDistributor(
        /// An ID of an item put up for a sale by a distributor.
        ItemId,
    ),
    PurchasedByRetailer(
        /// An ID of an item purchased by a retailer.
        ItemId,
    ),
    ApprovedByDistributor(
        /// An ID of an item approved by a retailer for a purchase.
        ItemId,
    ),
    ShippedByDistributor(
        /// An ID of an item shipped by a distributor.
        ItemId,
    ),
    ReceivedByRetailer(
        /// An ID of an item received by a retailer.
        ItemId,
    ),
    ForSaleByRetailer(
        /// An ID of an item put up for a sale by a retailer.
        ItemId,
    ),
    PurchasedByConsumer(
        /// An ID of an item purchased by a consumer.
        ItemId,
    ),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum SupplyChainState {
    /// Gets [`ItemInfo`].
    ///
    /// On success, returns [`SupplyChainStateReply::ItemInfo`].
    ItemInfo(
        /// An item's ID.
        ItemId,
    ),
}

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
pub enum SupplyChainStateReply {
    ItemInfo(ItemInfo),
}

#[derive(Encode, Decode, Clone, TypeInfo, Default, Debug, PartialEq, Eq)]
pub struct ItemInfo {
    /// An item's producer address.
    pub producer: ActorId,
    /// An item's distributor address. If it equals the zero address, then it's no here yet.
    pub distributor: ActorId,
    /// An item's retailer address. If it equals the zero address, then it's no here yet.
    pub retailer: ActorId,

    pub state: ItemState,
    /// An item's price. If it equals zero, then an item is sold for free or has never been put up for sale yet.
    pub price: u128,
    /// A delivery time during which a seller should deliver an item.
    pub delivery_time: u64,
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy, Debug, TypeInfo, Default)]
pub enum ItemState {
    #[default]
    Produced,
    ForSaleByProducer,
    PurchasedByDistributor,
    ApprovedByProducer,
    ShippedByProducer,
    ReceivedByDistributor,
    ProcessedByDistributor,
    PackagedByDistributor,
    ForSaleByDistributor,
    PurchasedByRetailer,
    ApprovedByDistributor,
    ShippedByDistributor,
    ReceivedByRetailer,
    ForSaleByRetailer,
    PurchasedByConsumer,
}
