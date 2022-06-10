#![no_std]

use gstd::{prelude::*, ActorId};
use primitive_types::U256;

pub type ItemId = U256;

#[derive(Encode, Decode, TypeInfo)]
pub struct InitSupplyChain {
    pub producers: BTreeSet<ActorId>,
    pub distributors: BTreeSet<ActorId>,
    pub retailers: BTreeSet<ActorId>,

    pub ft_program_id: ActorId,
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
    /// # Arguments
    /// * `name`: an item's name.
    /// * `description`: an item's description.
    ///
    /// On success returns [`SupplyChainEvent::Produced`].
    Produce { name: String, description: String },

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
    /// # Arguments
    /// * `item_id`: an item's ID.
    /// * `price`: an item's price.
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    PutUpForSaleByProducer { item_id: ItemId, price: u128 },

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
    /// # Arguments
    /// * `item_id`: an item's ID.
    /// * `delivery_time`: a time in milliseconds for which a producer must deliver an item.
    /// A countdown starts after [`SupplyChainAction::ShipByProducer`] is executed.
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    PurchaseByDistributor { item_id: ItemId, delivery_time: u64 },

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
    /// # Arguments
    /// * `item_id`: an item's ID.
    /// * `approve`: yes ([`true`]) or no ([`false`]).
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    ApproveByProducer { item_id: ItemId, approve: bool },

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
    /// On success returns [`SupplyChainEvent::Success`].
    ShipByProducer(ItemId),

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
    /// On success returns [`SupplyChainEvent::Success`].
    ReceiveByDistributor(ItemId),

    /// Processes a received item from a producer on behalf of a distributor.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`ReceivedByDistributor`](ItemState::ReceivedByDistributor).
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    ProcessByDistributor(ItemId),

    /// Packages a processed item on behalf of a distributor.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a distributor in a supply chain
    /// and a distributor of this item.
    /// * Item's [`ItemState`] must be [`ProcessedByDistributor`](ItemState::ProcessedByDistributor).
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    PackageByDistributor(ItemId),

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
    /// # Arguments
    /// * `item_id`: an item's ID.
    /// * `price`: an item's price.
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    PutUpForSaleByDistributor { item_id: ItemId, price: u128 },

    /// Purchases an item from a distributor on behalf of a retailer.
    ///
    /// Transfers tokens for purchasing an item to a supply chain
    /// until an item is received (by [`SupplyChainAction::ReceiveByRetailer`]).
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be a retailer in a supply chain.
    /// * Item's [`ItemState`] must be [`ForSaleByDistributor`](ItemState::ForSaleByDistributor).
    ///
    /// # Arguments
    /// * `item_id`: an item's ID.
    /// * `delivery_time`: a time in milliseconds for which a distributor must deliver an item.
    /// A countdown starts after [`SupplyChainAction::ShipByDistributor`] is executed.
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    PurchaseByRetailer { item_id: ItemId, delivery_time: u64 },

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
    /// # Arguments
    /// * `item_id`: an item's ID.
    /// * `approve`: yes ([`true`]) or no ([`false`]).
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    ApproveByDistributor { item_id: ItemId, approve: bool },

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
    /// On success returns [`SupplyChainEvent::Success`].
    ShipByDistributor(ItemId),

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
    /// On success returns [`SupplyChainEvent::Success`].
    ReceiveByRetailer(ItemId),

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
    /// # Arguments
    /// * `item_id`: an item's ID.
    /// * `price`: an item's price.
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    PutUpForSaleByRetailer { item_id: ItemId, price: u128 },

    /// Purchases an item from a retailer.
    ///
    /// Transfers tokens for purchasing an item to its retailer.
    ///
    /// Transfers item's NFT to a consumer.
    ///
    /// # Requirements
    /// * Item's [`ItemState`] must be [`ForSaleByRetailer`](ItemState::ForSaleByRetailer).
    ///
    /// On success returns [`SupplyChainEvent::Success`].
    PurchaseByConsumer(ItemId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum SupplyChainEvent {
    Produced(ItemId),
    Success,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum SupplyChainState {
    /// Gets [`ItemInfo`].
    ///
    /// # Arguments
    /// * `item_id`: an item's ID.
    ///
    /// On success returns [`SupplyChainStateReply::ItemInfo`].
    GetItemInfo(ItemId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum SupplyChainStateReply {
    ItemInfo(ItemInfo),
}

#[derive(Encode, Decode, Clone, TypeInfo, Default)]
pub struct ItemInfo {
    pub producer: ActorId,
    pub distributor: ActorId,
    pub retailer: ActorId,

    pub state: ItemState,
    pub price: u128,
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
