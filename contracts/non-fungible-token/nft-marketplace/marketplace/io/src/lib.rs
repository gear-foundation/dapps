#![no_std]
use codec::{Decode, Encode};
use gstd::{prelude::*, ActorId};
use primitive_types::{H256, U256};
use scale_info::TypeInfo;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitMarket {
    pub owner_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Offer {
    pub hash: H256,
    pub id: ActorId,
    pub ft_contract_id: Option<ActorId>,
    pub price: u128,
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, Clone)]
pub struct Auction {
    pub bid_period: u64,
    pub started_at: u64,
    pub ended_at: u64,
    pub current_price: u128,
    pub bids: Vec<Bid>,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone)]
pub struct Bid {
    pub id: ActorId,
    pub price: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, Default)]
pub struct Item {
    pub owner_id: ActorId,
    pub nft_contract_id: ActorId,
    pub ft_contract_id: Option<ActorId>,
    pub token_id: U256,
    pub price: Option<u128>,
    pub auction: Option<Auction>,
    pub offers: Vec<Offer>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketAction {
    AddNftContract(ActorId),
    AddFTContract(ActorId),
    AddMarketData {
        nft_contract_id: ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        price: Option<u128>,
    },
    BuyItem {
        nft_contract_id: ActorId,
        token_id: U256,
    },
    CreateAuction {
        nft_contract_id: ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        min_price: u128,
        bid_period: u64,
        duration: u64,
    },
    AddBid {
        nft_contract_id: ActorId,
        token_id: U256,
        price: u128,
    },
    AddOffer {
        nft_contract_id: ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        price: u128,
    },
    Withdraw {
        nft_contract_id: ActorId,
        token_id: U256,
        hash: H256,
    },
    AcceptOffer {
        nft_contract_id: ActorId,
        token_id: U256,
        offer_hash: H256,
    },
    SettleAuction {
        nft_contract_id: ActorId,
        token_id: U256,
    },
    Item {
        nft_contract_id: ActorId,
        token_id: U256,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum MarketEvent {
    MarketDataAdded {
        nft_contract_id: ActorId,
        owner: ActorId,
        token_id: U256,
        price: Option<u128>,
    },
    ItemSold {
        owner: ActorId,
        nft_contract_id: ActorId,
        token_id: U256,
    },
    BidAdded {
        nft_contract_id: ActorId,
        token_id: U256,
        price: u128,
    },
    AuctionCreated {
        nft_contract_id: ActorId,
        token_id: U256,
        price: u128,
    },
    AuctionSettled {
        nft_contract_id: ActorId,
        token_id: U256,
        price: u128,
    },
    AuctionCancelled {
        nft_contract_id: ActorId,
        token_id: U256,
    },
    NFTListed {
        nft_contract_id: ActorId,
        owner: ActorId,
        token_id: U256,
        price: Option<u128>,
    },
    ItemInfo(Item),
    OfferAdded {
        nft_contract_id: ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: U256,
        price: u128,
    },
    OfferAccepted {
        nft_contract_id: ActorId,
        token_id: U256,
        new_owner: ActorId,
        price: u128,
    },
    TokensWithdrawn {
        nft_contract_id: ActorId,
        token_id: U256,
        price: u128,
    },
}
