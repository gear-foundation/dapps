#![no_std]

use gear_lib_old::non_fungible_token::token::TokenMetadata;
use gstd::{
    collections::{HashMap, HashSet},
    exec, msg,
    prelude::*,
    ActorId,
};
use supply_chain_io::*;
use tx_manager::{TransactionGuard, TransactionManager};

mod tx_manager;
mod utils;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

fn get_mut_item(
    items: &mut HashMap<ItemId, Item>,
    item_id: ItemId,
    expected_item_state: ItemState,
) -> Result<&mut Item, Error> {
    let item = items.get_mut(&item_id).ok_or(Error::ItemNotFound)?;

    if item.info.state != expected_item_state {
        return Err(Error::UnexpectedItemState);
    }

    Ok(item)
}

fn role_to_set_item_dr(role: Role) -> fn(&mut Item, ActorId) {
    match role {
        Role::Distributor => Item::set_distributor,
        Role::Retailer => Item::set_retailer,
        _ => unreachable!(),
    }
}

type IsPdr = fn(&Item, ActorId) -> Result<(), Error>;

fn role_to_is_pdr(role: Role) -> IsPdr {
    match role {
        Role::Producer => Item::is_producer,
        Role::Distributor => Item::is_distributor,
        Role::Retailer => Item::is_retailer,
        _ => unreachable!(),
    }
}

fn role_to_item_pdr(role: Role) -> fn(&Item) -> ActorId {
    match role {
        Role::Producer => Item::producer,
        Role::Distributor => Item::distributor,
        Role::Retailer => Item::retailer,
        _ => unreachable!(),
    }
}

#[derive(Default)]
struct Item {
    info: ItemInfo,
    shipping_time: u64,
}

impl Item {
    fn set_retailer(&mut self, retailer: ActorId) {
        self.info.retailer = retailer
    }

    fn set_distributor(&mut self, distributor: ActorId) {
        self.info.distributor = distributor
    }

    fn set_state_and_get_event(&mut self, item_id: ItemId, item_state: ItemState) -> Event {
        self.info.state = item_state;

        Event {
            item_id,
            item_state,
        }
    }

    fn is_pdr(pdr: ActorId, actor_id: ActorId) -> Result<(), Error> {
        if pdr != actor_id {
            Err(Error::AccessRestricted)
        } else {
            Ok(())
        }
    }

    fn is_producer(&self, actor_id: ActorId) -> Result<(), Error> {
        Self::is_pdr(self.info.producer, actor_id)
    }

    fn is_distributor(&self, actor_id: ActorId) -> Result<(), Error> {
        Self::is_pdr(self.info.distributor, actor_id)
    }

    fn is_retailer(&self, actor_id: ActorId) -> Result<(), Error> {
        Self::is_pdr(self.info.retailer, actor_id)
    }

    fn producer(&self) -> ActorId {
        self.info.producer
    }

    fn retailer(&self) -> ActorId {
        self.info.retailer
    }

    fn distributor(&self) -> ActorId {
        self.info.distributor
    }
}

#[derive(Default)]
struct Contract {
    items: HashMap<ItemId, Item>,

    producers: HashSet<ActorId>,
    distributors: HashSet<ActorId>,
    retailers: HashSet<ActorId>,

    fungible_token: ActorId,
    non_fungible_token: ActorId,
}

impl Contract {
    async fn produce(
        &mut self,
        tx_guard: &mut TransactionGuard<'_, CachedAction>,
        msg_source: ActorId,
        token_metadata: TokenMetadata,
    ) -> Result<Event, Error> {
        if self.items.len() == MAX_NUMBER_OF_ITEMS {
            return Err(Error::MemoryLimitExceeded);
        }

        let item_id = utils::mint_nft(tx_guard, self.non_fungible_token, token_metadata).await?;

        utils::transfer_nft(tx_guard, self.non_fungible_token, msg_source, item_id).await?;

        self.items.insert(
            item_id,
            Item {
                info: ItemInfo {
                    producer: msg_source,
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        Ok(Event {
            item_id,
            item_state: Default::default(),
        })
    }

    async fn purchase(
        &mut self,
        tx_guard: &mut TransactionGuard<'_, CachedAction>,
        msg_source: ActorId,
        item_id: ItemId,
        expected_by: Role,
        by: Role,
        delivery_time: u64,
    ) -> Result<Event, Error> {
        let item = get_mut_item(
            &mut self.items,
            item_id,
            ItemState {
                state: ItemEventState::ForSale,
                by: expected_by,
            },
        )?;

        utils::transfer_ftokens(
            tx_guard,
            self.fungible_token,
            msg_source,
            exec::program_id(),
            item.info.price,
        )
        .await?;

        role_to_set_item_dr(by)(item, msg_source);
        item.info.delivery_time = delivery_time;

        Ok(item.set_state_and_get_event(
            item_id,
            ItemState {
                state: ItemEventState::Purchased,
                by,
            },
        ))
    }

    async fn put_up_for_sale(
        &mut self,
        tx_guard: &mut TransactionGuard<'_, CachedAction>,
        msg_source: ActorId,
        item_id: ItemId,
        expected_item_event_state: ItemEventState,
        by: Role,
        price: u128,
    ) -> Result<Event, Error> {
        let item = get_mut_item(
            &mut self.items,
            item_id,
            ItemState {
                state: expected_item_event_state,
                by,
            },
        )?;
        role_to_is_pdr(by)(item, msg_source)?;

        utils::transfer_nft(
            tx_guard,
            self.non_fungible_token,
            exec::program_id(),
            item_id,
        )
        .await?;
        item.info.price = price;

        Ok(item.set_state_and_get_event(
            item_id,
            ItemState {
                state: ItemEventState::ForSale,
                by,
            },
        ))
    }

    async fn approve(
        &mut self,
        tx_guard: &mut TransactionGuard<'_, CachedAction>,
        msg_source: ActorId,
        item_id: ItemId,
        expected_by: Role,
        by: Role,
        approve: bool,
    ) -> Result<Event, Error> {
        let item = get_mut_item(
            &mut self.items,
            item_id,
            ItemState {
                state: ItemEventState::Purchased,
                by: expected_by,
            },
        )?;
        role_to_is_pdr(by)(item, msg_source)?;

        let item_state = if approve {
            ItemState {
                state: ItemEventState::Approved,
                by,
            }
        } else {
            utils::transfer_ftokens(
                tx_guard,
                self.fungible_token,
                exec::program_id(),
                role_to_item_pdr(expected_by)(item),
                item.info.price,
            )
            .await?;

            ItemState {
                state: ItemEventState::ForSale,
                by,
            }
        };

        Ok(item.set_state_and_get_event(item_id, item_state))
    }

    fn ship(&mut self, msg_source: ActorId, item_id: ItemId, by: Role) -> Result<Event, Error> {
        let item = get_mut_item(
            &mut self.items,
            item_id,
            ItemState {
                state: ItemEventState::Approved,
                by,
            },
        )?;
        role_to_is_pdr(by)(item, msg_source)?;

        item.shipping_time = exec::block_timestamp();

        Ok(item.set_state_and_get_event(
            item_id,
            ItemState {
                state: ItemEventState::Shipped,
                by,
            },
        ))
    }

    async fn receive(
        &mut self,
        tx_guard: &mut TransactionGuard<'_, CachedAction>,
        msg_source: ActorId,
        item_id: ItemId,
        expected_by: Role,
        by: Role,
    ) -> Result<Event, Error> {
        let item = get_mut_item(
            &mut self.items,
            item_id,
            ItemState {
                state: ItemEventState::Shipped,
                by: expected_by,
            },
        )?;
        role_to_is_pdr(by)(item, msg_source)?;

        let program_id = exec::program_id();
        let elapsed_time = tx_guard.timestamp - item.shipping_time;
        // By default, all fungible tokens are transferred to a seller,
        let (mut to, mut amount) = (role_to_item_pdr(expected_by)(item), item.info.price);

        // but if the seller spends more time than was agreed...
        if elapsed_time > item.info.delivery_time {
            // ...and is extremely late (more than or exactly 2 times in this example),
            if elapsed_time >= item.info.delivery_time * 2 {
                // then all fungible tokens are refunded to a buyer...
                to = msg_source;
            } else {
                // ...or another half is transferred to the seller...
                amount /= 2;

                // ...and a half of tokens is refunded to the buyer.
                utils::transfer_ftokens(
                    tx_guard,
                    self.fungible_token,
                    program_id,
                    msg_source,
                    item.info.price - amount,
                )
                .await?;
            }
        }

        utils::transfer_ftokens(tx_guard, self.fungible_token, program_id, to, amount).await?;
        utils::transfer_nft(tx_guard, self.non_fungible_token, msg_source, item_id).await?;

        Ok(item.set_state_and_get_event(
            item_id,
            ItemState {
                state: ItemEventState::Received,
                by,
            },
        ))
    }

    fn process_or_package(
        &mut self,
        msg_source: ActorId,
        item_id: ItemId,
        expected_item_event_state: ItemEventState,
        state: ItemEventState,
    ) -> Result<Event, Error> {
        let item = get_mut_item(
            &mut self.items,
            item_id,
            ItemState {
                state: expected_item_event_state,
                by: Role::Distributor,
            },
        )?;
        item.is_distributor(msg_source)?;

        Ok(item.set_state_and_get_event(
            item_id,
            ItemState {
                state,
                by: Role::Distributor,
            },
        ))
    }
}

static mut STATE: Option<(Contract, TransactionManager<CachedAction>)> = None;

fn state_mut() -> &'static mut (Contract, TransactionManager<CachedAction>) {
    let state = unsafe { STATE.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
extern fn init() {
    let result = process_init();
    let is_err = result.is_err();

    msg::reply(result, 0).expect("failed to encode or reply from `init()`");

    if is_err {
        exec::exit(ActorId::zero());
    }
}

fn process_init() -> Result<(), Error> {
    let Initialize {
        producers,
        distributors,
        retailers,
        fungible_token,
        non_fungible_token,
    } = msg::load()?;

    if producers
        .iter()
        .chain(&distributors)
        .chain(&retailers)
        .chain(&[fungible_token, non_fungible_token])
        .any(|actor| actor.is_zero())
    {
        return Err(Error::ZeroActorId);
    }

    let [producers, distributors, retailers] =
        [producers, distributors, retailers].map(|actors| actors.into_iter().collect());

    unsafe {
        STATE = Some((
            Contract {
                producers,
                distributors,
                retailers,
                fungible_token,
                non_fungible_token,
                ..Default::default()
            },
            Default::default(),
        ));
    }

    Ok(())
}

#[gstd::async_main]
async fn main() {
    msg::reply(process_handle().await, 0).expect("failed to encode or reply `handle()`");
}

async fn process_handle() -> Result<Event, Error> {
    let Action {
        action,
        kind: tx_kind,
    } = msg::load()?;

    let msg_source = msg::source();
    let (contract, tx_manager) = state_mut();

    match action {
        InnerAction::Consumer(action) => match action {
            ConsumerAction::Purchase(item_id) => {
                let mut tx_guard = tx_manager.asquire_transaction(
                    tx_kind,
                    msg_source,
                    CachedAction::Purchase(item_id),
                )?;

                let item = get_mut_item(
                    &mut contract.items,
                    item_id,
                    ItemState {
                        state: ItemEventState::ForSale,
                        by: Role::Retailer,
                    },
                )?;

                utils::transfer_ftokens(
                    &mut tx_guard,
                    contract.fungible_token,
                    msg_source,
                    item.info.retailer,
                    item.info.price,
                )
                .await?;
                utils::transfer_nft(
                    &mut tx_guard,
                    contract.non_fungible_token,
                    msg_source,
                    item_id,
                )
                .await?;

                Ok(item.set_state_and_get_event(
                    item_id,
                    ItemState {
                        state: ItemEventState::Purchased,
                        by: Role::Consumer,
                    },
                ))
            }
        },
        InnerAction::Producer(action) => {
            if !contract.producers.contains(&msg_source) {
                return Err(Error::AccessRestricted);
            }

            match action {
                ProducerAction::Produce { token_metadata } => {
                    let mut tx_guard =
                        tx_manager.asquire_transaction(tx_kind, msg_source, CachedAction::Other)?;

                    contract
                        .produce(&mut tx_guard, msg_source, token_metadata)
                        .await
                }
                ProducerAction::PutUpForSale { item_id, price } => {
                    let mut tx_guard = tx_manager.asquire_transaction(
                        tx_kind,
                        msg_source,
                        CachedAction::PutUpForSale(item_id),
                    )?;

                    contract
                        .put_up_for_sale(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            ItemEventState::Produced,
                            Role::Producer,
                            price,
                        )
                        .await
                }
                ProducerAction::Approve { item_id, approve } => {
                    let mut tx_guard = tx_manager.asquire_transaction(
                        tx_kind,
                        msg_source,
                        CachedAction::Approve(item_id),
                    )?;

                    contract
                        .approve(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            Role::Distributor,
                            Role::Producer,
                            approve,
                        )
                        .await
                }
                ProducerAction::Ship(item_id) => contract.ship(msg_source, item_id, Role::Producer),
            }
        }
        InnerAction::Distributor(action) => {
            if !contract.distributors.contains(&msg_source) {
                return Err(Error::AccessRestricted);
            }

            match action {
                DistributorAction::Purchase {
                    item_id,
                    delivery_time,
                } => {
                    let mut tx_guard = tx_manager.asquire_transaction(
                        tx_kind,
                        msg_source,
                        CachedAction::Purchase(item_id),
                    )?;

                    contract
                        .purchase(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            Role::Producer,
                            Role::Distributor,
                            delivery_time,
                        )
                        .await
                }
                DistributorAction::Receive(item_id) => {
                    let mut tx_guard = tx_manager.asquire_transaction_with_timestamp(
                        tx_kind,
                        msg_source,
                        CachedAction::Receive(item_id),
                    )?;

                    contract
                        .receive(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            Role::Producer,
                            Role::Distributor,
                        )
                        .await
                }
                DistributorAction::Process(item_id) => contract.process_or_package(
                    msg_source,
                    item_id,
                    ItemEventState::Received,
                    ItemEventState::Processed,
                ),
                DistributorAction::Package(item_id) => contract.process_or_package(
                    msg_source,
                    item_id,
                    ItemEventState::Processed,
                    ItemEventState::Packaged,
                ),
                DistributorAction::PutUpForSale { item_id, price } => {
                    let mut tx_guard = tx_manager.asquire_transaction(
                        tx_kind,
                        msg_source,
                        CachedAction::PutUpForSale(item_id),
                    )?;

                    contract
                        .put_up_for_sale(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            ItemEventState::Packaged,
                            Role::Distributor,
                            price,
                        )
                        .await
                }
                DistributorAction::Approve { item_id, approve } => {
                    let mut tx_guard = tx_manager.asquire_transaction(
                        tx_kind,
                        msg_source,
                        CachedAction::Approve(item_id),
                    )?;

                    contract
                        .approve(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            Role::Retailer,
                            Role::Distributor,
                            approve,
                        )
                        .await
                }
                DistributorAction::Ship(item_id) => {
                    contract.ship(msg_source, item_id, Role::Distributor)
                }
            }
        }
        InnerAction::Retailer(action) => {
            if !contract.retailers.contains(&msg_source) {
                return Err(Error::AccessRestricted);
            }

            match action {
                RetailerAction::Purchase {
                    item_id,
                    delivery_time,
                } => {
                    let mut tx_guard = tx_manager.asquire_transaction(
                        tx_kind,
                        msg_source,
                        CachedAction::Purchase(item_id),
                    )?;

                    contract
                        .purchase(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            Role::Distributor,
                            Role::Retailer,
                            delivery_time,
                        )
                        .await
                }
                RetailerAction::Receive(item_id) => {
                    let mut tx_guard = tx_manager.asquire_transaction_with_timestamp(
                        tx_kind,
                        msg_source,
                        CachedAction::Receive(item_id),
                    )?;

                    contract
                        .receive(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            Role::Distributor,
                            Role::Retailer,
                        )
                        .await
                }
                RetailerAction::PutUpForSale { item_id, price } => {
                    let mut tx_guard = tx_manager.asquire_transaction(
                        tx_kind,
                        msg_source,
                        CachedAction::PutUpForSale(item_id),
                    )?;

                    contract
                        .put_up_for_sale(
                            &mut tx_guard,
                            msg_source,
                            item_id,
                            ItemEventState::Received,
                            Role::Retailer,
                            price,
                        )
                        .await
                }
            }
        }
    }
}

#[no_mangle]
extern fn state() {
    let state: State = generate_state();
    msg::reply(state, 0).expect("Failed to encode or reply with `State` from `state()`");
}

fn generate_state() -> State {
    let (
        Contract {
            items,
            producers,
            distributors,
            retailers,
            fungible_token,
            non_fungible_token,
        },
        tx_manager,
    ) = unsafe { STATE.take().expect("Unexpected error in taking state") };

    let [producers, distributors, retailers] =
        [producers, distributors, retailers].map(|actors| actors.iter().cloned().collect());

    State {
        items: items.iter().map(|item| (*item.0, item.1.info)).collect(),
        producers,
        distributors,
        retailers,
        fungible_token,
        non_fungible_token,
        cached_actions: tx_manager
            .cached_actions()
            .map(|(actor, action)| (*actor, *action))
            .collect(),
    }
}
