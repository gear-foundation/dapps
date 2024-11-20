use sails_rs::collections::{HashMap, HashSet};
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub type ContractId = ActorId;
pub type TokenId = U256;
pub type Price = u128;

pub const MINIMUM_VALUE: u64 = 1_000_000_000_000;

#[derive(Debug, Default, Clone)]
pub struct Market {
    pub admin_id: ActorId,
    pub items: HashMap<(ContractId, TokenId), Item>,
    pub approved_nft_contracts: HashSet<ActorId>,
    pub approved_ft_contracts: HashSet<ActorId>,
}
impl Market {
    pub fn check_approved_nft_contract(&self, nft_contract_id: &ActorId) {
        if !self.approved_nft_contracts.contains(nft_contract_id) {
            panic!("that nft contract is not approved");
        }
    }
    pub fn check_approved_ft_contract(&self, ft_contract_id: Option<ActorId>) {
        if ft_contract_id.is_some()
            && !self
                .approved_ft_contracts
                .contains(&ft_contract_id.expect("Must not be an error here"))
        {
            panic!("that ft contract is not approved");
        }
    }
    pub fn check_admin(&self) {
        if msg::source() != self.admin_id {
            panic!("Only owner can make that action");
        }
    }
}
#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ItemInfoArgs {
    nft_contract_id: ActorId,
    token_id: TokenId,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct InitMarket {
    pub admin_id: ActorId,
    pub treasury_id: ActorId,
    pub treasury_fee: u16,
}

#[derive(Debug, PartialEq, Eq, Default, Encode, Decode, TypeInfo, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Auction {
    pub started_at: u64,
    pub ended_at: u64,
    pub current_price: Price,
    pub current_winner: ActorId,
}

#[derive(Debug, Encode, Decode, TypeInfo, Clone, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Item {
    pub frozen: bool,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub ft_contract_id: Option<ContractId>,
    pub price: Option<Price>,
    pub auction: Option<Auction>,
    pub offers: HashMap<(Option<ContractId>, Price), ActorId>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
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
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
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

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct ItemState {
    pub frozen: bool,
    pub token_id: TokenId,
    pub owner: ActorId,
    pub ft_contract_id: Option<ContractId>,
    pub price: Option<Price>,
    pub auction: Option<Auction>,
    pub offers: Vec<((Option<ContractId>, Price), ActorId)>,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct MarketState {
    pub admin_id: ActorId,
    pub items: Vec<((ContractId, TokenId), ItemState)>,
    pub approved_nft_contracts: Vec<ActorId>,
    pub approved_ft_contracts: Vec<ActorId>,
}

impl From<Market> for MarketState {
    fn from(value: Market) -> Self {
        let Market {
            admin_id,
            items,
            approved_nft_contracts,
            approved_ft_contracts,
        } = value;

        let items = items
            .into_iter()
            .map(|(id, item)| {
                let item_state = ItemState {
                    frozen: item.frozen,
                    token_id: item.token_id,
                    owner: item.owner,
                    ft_contract_id: item.ft_contract_id,
                    price: item.price,
                    auction: item.auction,
                    offers: item.offers.into_iter().collect(),
                };
                (id, item_state)
            })
            .collect();
        let approved_nft_contracts = approved_nft_contracts.into_iter().collect();
        let approved_ft_contracts = approved_ft_contracts.into_iter().collect();

        Self {
            admin_id,
            items,
            approved_nft_contracts,
            approved_ft_contracts,
        }
    }
}
