#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{
    collections::{BTreeMap, BTreeSet},
    prelude::*,
    ActorId,
};
use primitive_types::U256;

pub type ContractId = ActorId;
pub type TokenId = U256;
pub type Price = u128;
pub type TransactionId = u64;

pub struct MarketMetadata;

impl Metadata for MarketMetadata {
    type Init = In<InitMarket>;
    type Handle = InOut<MarketAction, MarketEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<Market>;
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Market {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u16,
    pub items: BTreeMap<(ContractId, TokenId), Item>,
    pub approved_nft_contracts: BTreeSet<ActorId>,
    pub approved_ft_contracts: BTreeSet<ActorId>,
    pub tx_id: TransactionId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct ItemInfoArgs {
    nft_contract_id: ActorId,
    token_id: TokenId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitMarket {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u16,
}

#[derive(Debug, PartialEq, Eq, Default, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Auction {
    pub bid_period: u64,
    pub started_at: u64,
    pub ended_at: u64,
    pub current_price: Price,
    pub current_winner: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MarketTx {
    CreateAuction,
    Bid {
        account: ActorId,
        price: Price,
    },
    SettleAuction,
    Sale {
        buyer: ActorId,
    },
    Offer {
        ft_id: ContractId,
        price: Price,
        account: ActorId,
    },
    AcceptOffer,
    Withdraw {
        ft_id: ContractId,
        price: Price,
        account: ActorId,
    },
}

#[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo, Clone, Default)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Item {
    pub token_id: TokenId,
    pub owner: ActorId,
    pub ft_contract_id: Option<ContractId>,
    pub price: Option<Price>,
    pub auction: Option<Auction>,
    pub offers: BTreeMap<(Option<ContractId>, Price), ActorId>,
    pub tx: Option<(TransactionId, MarketTx)>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MarketAction {
    /// Adds NFT contract addresses that can be listed on marketplace.
    ///
    /// # Requirements:
    /// Only admin can add approved NFT accounts.
    ///
    /// On success replies [`MarketEvent::NftContractAdded`].
    AddNftContract(
        /// the NFT contract address
        ContractId,
    ),

    /// Adds the contract addresses of fungible tokens with which users can pay for NFTs.
    ///
    /// # Requirements:
    /// Only admin can add approved fungible-token accounts.
    ///
    /// On success replies [`MarketEvent::FtContractAdded`].
    AddFTContract(
        /// the FT contract address
        ContractId,
    ),

    /// Adds data on market item.
    /// If the item of that NFT does not exist on the marketplace then it will be listed.
    /// If the item exists then that action is used to change the price or suspend the sale.
    ///
    /// # Requirements
    /// * [`msg::source()`](gstd::msg::source) must be the NFT owner
    /// * `nft_contract_id` must be in the list of `approved_nft_contracts`
    /// * if item already exists, then it cannot be changed if there is an active auction
    ///
    /// On success replies [`MarketEvent::MarketDataAdded`].
    AddMarketData {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the fungible token contract address (If it is `None` then the item is traded for the native value)
        ft_contract_id: Option<ContractId>,
        /// the NFT id
        token_id: TokenId,
        /// the NFT price (if it is `None` then the item is not on the sale)
        price: Option<Price>,
    },

    /// Sells the NFT.
    ///
    /// # Requirements:
    /// * The NFT item must exists and be on sale.
    /// * If the NFT is sold for a native Gear value, then a buyer must attach value equals to the price.
    /// * If the NFT is sold for fungible tokens then a buyer must have enough tokens in the fungible token contract.
    /// * There must be no an opened auction on the item.
    ///
    /// On success replies [`MarketEvent::ItemSold`].
    BuyItem {
        /// NFT contract address
        nft_contract_id: ContractId,
        /// the token ID
        token_id: TokenId,
    },

    /// Creates an auction for selected item.
    /// If the NFT item doesn't exist on the marketplace then it will be listed
    ///
    /// Requirements:
    /// * Only the item owner can start auction.
    /// * `nft_contract_id` must be in the list of `approved_nft_contracts`
    /// *  There must be no active auction.
    ///
    /// On success replies [`MarketEvent::AuctionCreated`].
    CreateAuction {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the fungible token contract address (If it is `None` then the item is traded for the native value)
        ft_contract_id: Option<ContractId>,
        /// the NFT id
        token_id: TokenId,
        /// the starting price
        min_price: Price,
        /// the time interval the auction is extended if bid is made if the auction ends before `exec::blocktimestamp() + bid_period`
        bid_period: u64,
        /// the auction duration
        duration: u64,
    },

    /// Adds a bid to an ongoing auction.
    ///
    /// # Requirements:
    /// * The item must extsts.
    /// * The auction must exists on the item.
    /// * If the NFT is sold for a native Gear value, then a buyer must attach value equals to the price indicated in the arguments.
    /// * If the NFT is sold for fungible tokens then a buyer must have   enough tokens in the fungible token contract.
    /// * `price` must be greater then the current offered price for that item.
    ///
    /// On success replies [`MarketEvent::BidAdded`].
    AddBid {
        /// the NFT contract address.
        nft_contract_id: ContractId,
        /// * `token_id`: the NFT id.
        token_id: TokenId,
        /// the offered price.
        price: Price,
    },

    /// Settles the auction.
    ///
    /// Requirements:
    /// * The auction must be over.
    ///
    /// On successful auction replies [`MarketEvent::AuctionSettled`].
    /// If no bids were made replies [`MarketEvent::AuctionCancelled`].
    SettleAuction {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the NFT id
        token_id: TokenId,
    },

    /// Adds a price offer to the item.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * There must be no an ongoing auction on the item.
    /// * If a user makes an offer in native Gear value, then he must attach value equals to the price indicated in the arguments.
    /// * If a user makes an offer in fungible tokens then he must have  enough tokens in the fungible token contract.
    /// * The price can not be equal to 0.
    /// * There must be no identical offers on the item.
    ///
    /// On success replies [`MarketEvent::OfferAdded`].
    AddOffer {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the FT contract address (if it is `None, the offer is made for the native value)
        ft_contract_id: Option<ContractId>,
        /// the NFT id
        token_id: TokenId,
        /// the offer price
        price: Price,
    },

    /// Withdraws tokens.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only the offer creator can withdraw his tokens.
    /// * The offer with indicated hash must exist.
    ///
    /// On success replies [`MarketEvent::Withdraw`].
    Withdraw {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the FT contract address (if it is `None, the offer is made for the native value)
        ft_contract_id: Option<ContractId>,
        /// the NFT id
        token_id: TokenId,
        /// The offered price (native value)
        price: Price,
    },

    /// Accepts an offer.
    ///
    /// Requirements:
    /// * NFT item must exists and be listed on the marketplace.
    /// * Only owner can accept offer.
    /// * There must be no ongoing auction.
    /// * The offer with indicated hash must exist.
    ///
    /// On success replies [`MarketEvent::ItemSold`].
    AcceptOffer {
        /// the NFT contract address
        nft_contract_id: ContractId,
        /// the NFT id
        token_id: TokenId,
        /// the fungible token contract address
        ft_contract_id: Option<ContractId>,
        /// the offer price
        price: Price,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MarketEvent {
    NftContractAdded(ContractId),
    FtContractAdded(ContractId),
    MarketDataAdded {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Option<Price>,
    },
    ItemSold {
        owner: ActorId,
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
    BidAdded {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Price,
    },
    AuctionCreated {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Price,
    },
    AuctionSettled {
        nft_contract_id: ContractId,
        token_id: TokenId,
        price: Price,
    },
    AuctionCancelled {
        nft_contract_id: ContractId,
        token_id: TokenId,
    },
    NFTListed {
        nft_contract_id: ContractId,
        owner: ActorId,
        token_id: TokenId,
        price: Option<Price>,
    },
    OfferAdded {
        nft_contract_id: ContractId,
        ft_contract_id: Option<ActorId>,
        token_id: TokenId,
        price: Price,
    },
    OfferAccepted {
        nft_contract_id: ContractId,
        token_id: TokenId,
        new_owner: ActorId,
        price: Price,
    },
    Withdraw {
        nft_contract_id: ActorId,
        token_id: TokenId,
        price: Price,
    },
    TransactionFailed,
    RerunTransaction,
    TransferValue,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum MarketErr {
    NFTTransferFailed,
    TokenTransferFailed,
    WrongTransaction,
    RerunTransaction,
    WrongPrice,
    InvalidCaller,
    ItemOnAuction,
    ItemDoesNotExists,
    ItemIsNotOnSale,
    AuctionBidPeriodOrDurationIsInvalid,
    AuctionMinPriceIsZero,
    AuctionIsAlreadyExists,
    AuctionIsAlreadyEnded,
    AuctionIsNotOver,
    AuctionDoesNotExists,
    AuctionIsOpened,
    ContractNotApproved,
    OfferAlreadyExists,
    OfferShouldAcceptedByOwner,
    OfferIsNotExists,
}

pub fn all_items(state: Market) -> Vec<Item> {
    state.items.values().cloned().collect()
}

pub fn item_info(state: Market, args: &ItemInfoArgs) -> Option<Item> {
    state
        .items
        .get(&(args.nft_contract_id, args.token_id))
        .cloned()
}
