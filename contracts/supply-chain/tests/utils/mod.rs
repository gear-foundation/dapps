use common::{InitResult, Program, RunResult, StateReply, TransactionalProgram};
use gstd::{
    collections::{HashMap, HashSet},
    prelude::*,
    ActorId,
};
use gtest::{Program as InnerProgram, System, EXISTENTIAL_DEPOSIT};
use supply_chain_io::*;
use supply_chain_state::{WASM_BINARY, WASM_EXPORTS};

mod common;
mod fungible_token;
mod non_fungible_token;

pub mod prelude;

pub use self::non_fungible_token::NonFungibleToken;
pub use common::initialize_system;
pub use fungible_token::FungibleToken;

pub const FOREIGN_USER: u64 = 1029384756123;
pub const PRODUCER: u64 = 5;
pub const DISTRIBUTOR: u64 = 7;
pub const RETAILER: u64 = 9;

type SupplyChainRunResult<T> = RunResult<T, Event, Error>;

pub struct SupplyChain<'a>(InnerProgram<'a>);

impl Program for SupplyChain<'_> {
    fn inner_program(&self) -> &InnerProgram<'_> {
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
        Self::common_initialize_custom(system, config, false)
    }

    pub fn initialize_custom_with_existential_deposit(
        system: &'a System,
        config: Initialize,
    ) -> InitResult<SupplyChain<'a>, Error> {
        Self::common_initialize_custom(system, config, true)
    }

    fn common_initialize_custom(
        system: &'a System,
        config: Initialize,
        is_exdep_needed: bool,
    ) -> InitResult<SupplyChain<'a>, Error> {
        let program = InnerProgram::current_opt(system);

        if is_exdep_needed {
            system.mint_to(program.id(), EXISTENTIAL_DEPOSIT);
        }

        let result = program.send(FOREIGN_USER, config);
        let is_active = system.is_active_program(program.id());

        InitResult::new(Self(program), result, is_active)
    }

    pub fn state(&self) -> SupplyChainState<'_> {
        SupplyChainState(&self.0)
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

pub struct SupplyChainState<'a>(&'a InnerProgram<'a>);

impl SupplyChainState<'_> {
    fn query_state_common<A: Encode, T: Decode>(
        self,
        fn_index: usize,
        argument: Option<A>,
    ) -> StateReply<T> {
        StateReply(
            self.0
                .read_state_using_wasm( 0, WASM_EXPORTS[fn_index], WASM_BINARY.into(), argument)
                .unwrap(),
        )
    }

    fn query_state_with_argument<A: Encode, T: Decode>(
        self,
        fn_index: usize,
        argument: A,
    ) -> StateReply<T> {
        self.query_state_common(fn_index, Some(argument))
    }

    fn query_state<T: Decode>(self, fn_index: usize) -> StateReply<T> {
        self.query_state_common::<(), _>(fn_index, None)
    }

    pub fn item_price(self, item_id: u128) -> StateReply<Option<u128>> {
        StateReply(self.item_info(item_id).0.map(|item_info| item_info.price))
    }

    pub fn item_info(self, item_id: u128) -> StateReply<Option<ItemInfo>> {
        self.query_state_with_argument(1, ItemId::from(item_id))
    }

    pub fn participants(self) -> StateReply<Participants> {
        self.query_state(2)
    }

    pub fn existing_items(self) -> StateReply<HashMap<ItemId, ItemInfo>> {
        let result: StateReply<Vec<_>> = self.query_state(4);

        result.into()
    }

    pub fn fungible_token(self) -> StateReply<ActorId> {
        self.query_state(5)
    }

    pub fn non_fungible_token(self) -> StateReply<ActorId> {
        self.query_state(6)
    }

    pub fn roles(self, actor_id: u64) -> StateReply<HashSet<Role>> {
        let result: StateReply<Vec<_>> = self.query_state_with_argument(3, ActorId::from(actor_id));

        result.into()
    }
}

fn bool_to_event(is_approved: bool) -> ItemEventState {
    if is_approved {
        ItemEventState::Approved
    } else {
        ItemEventState::ForSale
    }
}
