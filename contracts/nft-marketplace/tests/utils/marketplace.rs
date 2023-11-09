use super::{prelude::*, MetaStateReply, RunResult};
use gstd::ActorId;
use gtest::{Program as InnerProgram, System};
use nft_marketplace_io::*;

pub struct Market<'a>(InnerProgram<'a>);

impl Program for Market<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
        &self.0
    }
}

type MarketRunResult<T> = RunResult<T, MarketEvent, MarketErr>;

impl<'a> Market<'a> {
    pub fn initialize(system: &'a System) -> Self {
        Self::initialize_custom(
            system,
            InitMarket {
                admin_id: ADMIN.into(),
                treasury_id: TREASURY_ID.into(),
                treasury_fee: TREASURY_FEE,
            },
        )
        .succeed()
    }

    pub fn initialize_custom(system: &System, config: InitMarket) -> MarketInit<'_> {
        let program = InnerProgram::current_opt(system);

        let failed = program.send(ADMIN, config).main_failed();

        MarketInit(program, failed)
    }

    pub fn meta_state(&self) -> MarketMetaState<'_> {
        MarketMetaState(&self.0)
    }

    pub fn add_nft_contract(
        &self,
        from: u64,
        nft_contract_id: ActorId,
    ) -> MarketRunResult<ContractId> {
        RunResult::new(
            self.0
                .send(from, MarketAction::AddNftContract(nft_contract_id)),
            MarketEvent::NftContractAdded,
        )
    }

    pub fn add_ft_contract(
        &self,
        from: u64,
        ft_contract_id: ActorId,
    ) -> MarketRunResult<ContractId> {
        RunResult::new(
            self.0
                .send(from, MarketAction::AddFTContract(ft_contract_id)),
            MarketEvent::FtContractAdded,
        )
    }

    pub fn add_market_data(
        &self,
        _sys: &System,
        from: u64,
        nft_contract_id: ActorId,
        ft_contract_id: Option<ActorId>,
        token_id: TokenId,
        price: Option<u128>,
    ) -> MarketRunResult<(ContractId, TokenId, Option<Price>)> {
        RunResult::new(
            self.0.send(
                from,
                MarketAction::AddMarketData {
                    nft_contract_id,
                    ft_contract_id,
                    token_id,
                    price,
                },
            ),
            |(nft_contract_id, token_id, price)| MarketEvent::MarketDataAdded {
                nft_contract_id,
                token_id,
                price,
            },
        )
    }

    pub fn buy_item(
        &self,
        from: u64,
        nft_contract_id: ActorId,
        token_id: TokenId,
        value: u128,
    ) -> MarketRunResult<(ActorId, ContractId, TokenId)> {
        RunResult::new(
            self.0.send_with_value(
                from,
                MarketAction::BuyItem {
                    nft_contract_id,
                    token_id,
                },
                value,
            ),
            |(owner, nft_contract_id, token_id)| MarketEvent::ItemSold {
                owner,
                nft_contract_id,
                token_id,
            },
        )
    }

    pub fn add_offer(
        &self,
        from: ActorId,
        nft_contract_id: ContractId,
        token_id: TokenId,
        ft_contract_id: Option<ContractId>,
        price: Price,
        value: u128,
    ) -> MarketRunResult<(ContractId, Option<ContractId>, TokenId, Price)> {
        RunResult::new(
            self.0.send_with_value(
                from.as_ref(),
                MarketAction::AddOffer {
                    nft_contract_id,
                    ft_contract_id,
                    token_id,
                    price,
                },
                value,
            ),
            |(nft_contract_id, ft_contract_id, token_id, price)| MarketEvent::OfferAdded {
                nft_contract_id,
                ft_contract_id,
                token_id,
                price,
            },
        )
    }

    pub fn accept_offer(
        &self,
        from: ActorId,
        nft_contract_id: ContractId,
        token_id: TokenId,
        ft_contract_id: Option<ContractId>,
        price: Price,
    ) -> MarketRunResult<(ContractId, TokenId, ActorId, Price)> {
        RunResult::new(
            self.0.send(
                from.as_ref(),
                MarketAction::AcceptOffer {
                    nft_contract_id,
                    token_id,
                    ft_contract_id,
                    price,
                },
            ),
            |(nft_contract_id, token_id, new_owner, price)| MarketEvent::OfferAccepted {
                nft_contract_id,
                token_id,
                new_owner,
                price,
            },
        )
    }

    pub fn withdraw(
        &self,
        from: ActorId,
        nft_contract_id: ContractId,
        token_id: TokenId,
        ft_contract_id: Option<ContractId>,
        price: Price,
    ) -> MarketRunResult<(ContractId, TokenId, Price)> {
        RunResult::new(
            self.0.send(
                from.as_ref(),
                MarketAction::Withdraw {
                    nft_contract_id,
                    token_id,
                    ft_contract_id,
                    price,
                },
            ),
            |(nft_contract_id, token_id, price)| MarketEvent::Withdraw {
                nft_contract_id,
                token_id,
                price,
            },
        )
    }

    pub fn create_auction(
        &self,
        _sys: &System,
        from: u64,
        (nft_contract_id, token_id, ft_contract_id): (ContractId, TokenId, Option<ContractId>),
        min_price: u128,
        bid_period: u64,
        duration: u64,
    ) -> MarketRunResult<(ContractId, TokenId, Price)> {
        RunResult::new(
            self.0.send(
                from,
                MarketAction::CreateAuction {
                    nft_contract_id,
                    ft_contract_id,
                    token_id,
                    min_price,
                    bid_period,
                    duration,
                },
            ),
            |(nft_contract_id, token_id, price)| MarketEvent::AuctionCreated {
                nft_contract_id,
                token_id,
                price,
            },
        )
    }

    pub fn add_bid(
        &self,
        from: u64,
        nft_contract_id: ActorId,
        token_id: TokenId,
        price: u128,
        value: u128,
    ) -> MarketRunResult<(ContractId, TokenId, Price)> {
        RunResult::new(
            self.0.send_with_value(
                from,
                MarketAction::AddBid {
                    nft_contract_id,
                    token_id,
                    price,
                },
                value,
            ),
            |(nft_contract_id, token_id, price)| MarketEvent::BidAdded {
                nft_contract_id,
                token_id,
                price,
            },
        )
    }

    pub fn settle_auction(
        &self,
        from: u64,
        nft_contract_id: ActorId,
        token_id: TokenId,
    ) -> MarketRunResult<MarketEvent> {
        RunResult::new(
            self.0.send(
                from,
                MarketAction::SettleAuction {
                    nft_contract_id,
                    token_id,
                },
            ),
            |market_event| market_event,
        )
    }
}

pub struct MarketMetaState<'a>(&'a InnerProgram<'a>);

impl MarketMetaState<'_> {
    pub fn state(&self) -> MetaStateReply<nft_marketplace_io::Market> {
        MetaStateReply(self.0.read_state(0).expect("Unexpected invalid state."))
    }
}

pub struct MarketInit<'a>(InnerProgram<'a>, bool);

impl<'a> MarketInit<'a> {
    #[track_caller]
    pub fn failed(self) {
        assert!(self.1)
    }

    #[track_caller]
    pub fn succeed(self) -> Market<'a> {
        assert!(!self.1);
        Market(self.0)
    }
}
