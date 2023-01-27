use common::{InitResult, MetaStateReply, Program, RunResult, TransactionalProgram};
use gstd::{prelude::*, ActorId};
use gtest::{Program as InnerProgram, System, EXISTENTIAL_DEPOSIT};
use hashbrown::{HashMap, HashSet};
use supply_chain_io::*;

mod common;
mod fungible_token;
mod non_fungible_token;

pub mod prelude;

pub use common::initialize_system;
pub use fungible_token::FungibleToken;
pub use non_fungible_token::NonFungibleToken;

pub const FOREIGN_USER: u64 = 1029384756123;
pub const PRODUCER: u64 = 5;
pub const DISTRIBUTOR: u64 = 7;
pub const RETAILER: u64 = 9;

type SupplyChainRunResult<T> = RunResult<T, Event, Error>;

pub struct SupplyChain<'a>(InnerProgram<'a>);

impl Program for SupplyChain<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> SupplyChain<'a> {
    pub fn initialize(
        system: &'a System,
        fungible_token: ActorId,
        non_fungible_token: ActorId,
    ) -> Self {
        Self::initialize_custom(
            system,
            Initialize {
                producers: vec![PRODUCER.into()],
                distributors: vec![DISTRIBUTOR.into()],
                retailers: vec![RETAILER.into()],

                fungible_token,
                non_fungible_token,
            },
        )
        .succeed()
    }

    pub fn initialize_custom(
        system: &'a System,
        config: Initialize,
    ) -> InitResult<SupplyChain<'a>, Error> {
        Self::common_initialize_custom(system, config, |_, _| {})
    }

    pub fn initialize_custom_with_existential_deposit(
        system: &'a System,
        config: Initialize,
    ) -> InitResult<SupplyChain<'a>, Error> {
        Self::common_initialize_custom(system, config, |system, program| {
            system.mint_to(program.id(), EXISTENTIAL_DEPOSIT)
        })
    }

    fn common_initialize_custom(
        system: &'a System,
        config: Initialize,
        mint: fn(&System, &InnerProgram),
    ) -> InitResult<SupplyChain<'a>, Error> {
        let program = InnerProgram::current(system);

        mint(system, &program);

        let result = program.send(FOREIGN_USER, config);
        let is_active = system.is_active_program(program.id());

        InitResult::new(Self(program), result, is_active)
    }

    pub fn meta_state(&self) -> SupplyChainMetaState {
        SupplyChainMetaState(&self.0)
    }

    pub fn produce(&mut self, from: u64) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Producer(ProducerAction::Produce {
                    token_metadata: Default::default(),
                })),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Produced,
                    by: Role::Producer,
                },
            },
        )
    }

    pub fn put_up_for_sale_by_producer(
        &mut self,
        from: u64,
        item_id: u128,
        price: u128,
    ) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Producer(ProducerAction::PutUpForSale {
                    item_id: item_id.into(),
                    price,
                })),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::ForSale,
                    by: Role::Producer,
                },
            },
        )
    }

    pub fn purchase_by_distributor(
        &mut self,
        from: u64,
        item_id: u128,
        delivery_time: u64,
    ) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Distributor(DistributorAction::Purchase {
                    item_id: item_id.into(),
                    delivery_time,
                })),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Purchased,
                    by: Role::Distributor,
                },
            },
        )
    }

    pub fn approve_by_producer(
        &mut self,
        from: u64,
        item_id: u128,
        approve: bool,
    ) -> SupplyChainRunResult<(u128, bool)> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Producer(ProducerAction::Approve {
                    item_id: item_id.into(),
                    approve,
                })),
            ),
            |(item_id, approved)| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: bool_to_event(approved),
                    by: Role::Producer,
                },
            },
        )
    }

    pub fn ship_by_producer(&mut self, from: u64, item_id: u128) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Producer(ProducerAction::Ship(item_id.into()))),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Shipped,
                    by: Role::Producer,
                },
            },
        )
    }

    pub fn receive_by_distributor(
        &mut self,
        from: u64,
        item_id: u128,
    ) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Distributor(DistributorAction::Receive(
                    item_id.into(),
                ))),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Received,
                    by: Role::Distributor,
                },
            },
        )
    }

    pub fn process(&mut self, from: u64, item_id: u128) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Distributor(DistributorAction::Process(
                    item_id.into(),
                ))),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Processed,
                    by: Role::Distributor,
                },
            },
        )
    }

    pub fn package(&mut self, from: u64, item_id: u128) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Distributor(DistributorAction::Package(
                    item_id.into(),
                ))),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Packaged,
                    by: Role::Distributor,
                },
            },
        )
    }

    pub fn put_up_for_sale_by_distributor(
        &mut self,
        from: u64,
        item_id: u128,
        price: u128,
    ) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Distributor(DistributorAction::PutUpForSale {
                    item_id: item_id.into(),
                    price,
                })),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::ForSale,
                    by: Role::Distributor,
                },
            },
        )
    }

    pub fn purchase_by_retailer(
        &mut self,
        from: u64,
        item_id: u128,
        delivery_time: u64,
    ) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Retailer(RetailerAction::Purchase {
                    item_id: item_id.into(),
                    delivery_time,
                })),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Purchased,
                    by: Role::Retailer,
                },
            },
        )
    }

    pub fn approve_by_distributor(
        &mut self,
        from: u64,
        item_id: u128,
        approve: bool,
    ) -> SupplyChainRunResult<(u128, bool)> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Distributor(DistributorAction::Approve {
                    item_id: item_id.into(),
                    approve,
                })),
            ),
            |(item_id, approved)| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: bool_to_event(approved),
                    by: Role::Distributor,
                },
            },
        )
    }

    pub fn ship_by_distributor(&mut self, from: u64, item_id: u128) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Distributor(DistributorAction::Ship(
                    item_id.into(),
                ))),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Shipped,
                    by: Role::Distributor,
                },
            },
        )
    }

    pub fn receive_by_retailer(&mut self, from: u64, item_id: u128) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Retailer(RetailerAction::Receive(
                    item_id.into(),
                ))),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Received,
                    by: Role::Retailer,
                },
            },
        )
    }

    pub fn put_up_for_sale_by_retailer(
        &mut self,
        from: u64,
        item_id: u128,
        price: u128,
    ) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Retailer(RetailerAction::PutUpForSale {
                    item_id: item_id.into(),
                    price,
                })),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::ForSale,
                    by: Role::Retailer,
                },
            },
        )
    }

    pub fn purchase_by_consumer(&mut self, from: u64, item_id: u128) -> SupplyChainRunResult<u128> {
        RunResult::new(
            self.0.send(
                from,
                Action::new(InnerAction::Consumer(ConsumerAction::Purchase(
                    item_id.into(),
                ))),
            ),
            |item_id| Event {
                item_id: item_id.into(),
                item_state: ItemState {
                    state: ItemEventState::Purchased,
                    by: Role::Consumer,
                },
            },
        )
    }
}

pub struct SupplyChainMetaState<'a>(&'a InnerProgram<'a>);

impl SupplyChainMetaState<'_> {
    pub fn item_price(self, item_id: u128) -> MetaStateReply<Option<u128>> {
        MetaStateReply(self.item_info(item_id).0.map(|item_info| item_info.price))
    }

    pub fn item_info(self, item_id: u128) -> MetaStateReply<Option<ItemInfo>> {
        if let StateReply::ItemInfo(reply) = self
            .0
            .meta_state(StateQuery::ItemInfo(item_id.into()))
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn participants(self) -> MetaStateReply<Participants> {
        if let StateReply::Participants(reply) =
            self.0.meta_state(StateQuery::Participants).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn fungible_token(self) -> MetaStateReply<ActorId> {
        if let StateReply::FungibleToken(reply) =
            self.0.meta_state(StateQuery::FungibleToken).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn non_fungible_token(self) -> MetaStateReply<ActorId> {
        if let StateReply::NonFungibleToken(reply) =
            self.0.meta_state(StateQuery::NonFungibleToken).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn existing_items(self) -> MetaStateReply<HashMap<ItemId, ItemInfo>> {
        if let StateReply::ExistingItems(reply) =
            self.0.meta_state(StateQuery::ExistingItems).unwrap()
        {
            MetaStateReply(reply.into_iter().collect())
        } else {
            unreachable!()
        }
    }

    pub fn roles(self, actor_id: u64) -> MetaStateReply<HashSet<Role>> {
        if let StateReply::Roles(reply) = self
            .0
            .meta_state(StateQuery::Roles(actor_id.into()))
            .unwrap()
        {
            MetaStateReply(reply.into_iter().collect())
        } else {
            unreachable!()
        }
    }
}

fn bool_to_event(is_approved: bool) -> ItemEventState {
    const EVENTS: [ItemEventState; 2] = [ItemEventState::ForSale, ItemEventState::Approved];

    EVENTS[is_approved as usize]
}
