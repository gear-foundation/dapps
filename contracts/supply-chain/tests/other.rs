use utils::{prelude::*, FungibleToken, NonFungibleToken};

pub mod utils;

#[test]
fn interact_with_unexistent_item() {
    const NONEXISTENT_ITEM: u128 = 99999999;

    let system = utils::initialize_system();

    let fungible_token = FungibleToken::initialize(&system);
    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut supply_chain = SupplyChain::initialize(
        &system,
        fungible_token.actor_id(),
        non_fungible_token.actor_id(),
    );

    supply_chain
        .put_up_for_sale_by_producer(PRODUCER, NONEXISTENT_ITEM, ITEM_PRICE)
        .failed(Error::ItemNotFound);
    supply_chain
        .purchase_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM, DELIVERY_TIME)
        .failed(Error::ItemNotFound);
    supply_chain
        .approve_by_producer(PRODUCER, NONEXISTENT_ITEM, true)
        .failed(Error::ItemNotFound);
    supply_chain
        .ship_by_producer(PRODUCER, NONEXISTENT_ITEM)
        .failed(Error::ItemNotFound);
    supply_chain
        .receive_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed(Error::ItemNotFound);
    supply_chain
        .process(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed(Error::ItemNotFound);
    supply_chain
        .package(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed(Error::ItemNotFound);
    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM, ITEM_PRICE)
        .failed(Error::ItemNotFound);
    supply_chain
        .purchase_by_retailer(RETAILER, NONEXISTENT_ITEM, DELIVERY_TIME)
        .failed(Error::ItemNotFound);
    supply_chain
        .approve_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM, true)
        .failed(Error::ItemNotFound);
    supply_chain
        .ship_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed(Error::ItemNotFound);
    supply_chain
        .receive_by_retailer(RETAILER, NONEXISTENT_ITEM)
        .failed(Error::ItemNotFound);
    supply_chain
        .put_up_for_sale_by_retailer(RETAILER, NONEXISTENT_ITEM, ITEM_PRICE)
        .failed(Error::ItemNotFound);
    supply_chain
        .purchase_by_consumer(CONSUMER, NONEXISTENT_ITEM)
        .failed(Error::ItemNotFound);

    supply_chain
        .meta_state()
        .item_info(NONEXISTENT_ITEM)
        .eq(None);
    supply_chain.meta_state().existing_items().eq([].into());
}

#[test]
fn initialization() {
    let system = utils::initialize_system();

    let fungible_token = FungibleToken::initialize(&system);
    let non_fungible_token = NonFungibleToken::initialize(&system);

    let mut supply_chain_config = Initialize {
        producers: vec![ActorId::zero()],
        distributors: vec![ActorId::zero()],
        retailers: vec![ActorId::zero()],

        fungible_token: fungible_token.actor_id(),
        non_fungible_token: non_fungible_token.actor_id(),
    };
    SupplyChain::initialize_custom_with_existential_deposit(&system, supply_chain_config.clone())
        .failed(Error::ZeroActorId);

    supply_chain_config.producers = [PRODUCER.into()].into();
    SupplyChain::initialize_custom_with_existential_deposit(&system, supply_chain_config.clone())
        .failed(Error::ZeroActorId);

    supply_chain_config.distributors = [DISTRIBUTOR.into()].into();
    SupplyChain::initialize_custom_with_existential_deposit(&system, supply_chain_config.clone())
        .failed(Error::ZeroActorId);

    supply_chain_config.retailers = [RETAILER.into()].into();
    let supply_chain =
        SupplyChain::initialize_custom(&system, supply_chain_config.clone()).succeed();

    supply_chain.meta_state().participants().eq(Participants {
        producers: supply_chain_config.producers,
        distributors: supply_chain_config.distributors,
        retailers: supply_chain_config.retailers,
    });
    supply_chain
        .meta_state()
        .fungible_token()
        .eq(fungible_token.actor_id());
    supply_chain
        .meta_state()
        .non_fungible_token()
        .eq(non_fungible_token.actor_id());
}

#[test]
fn query_existing_items() {
    let system = utils::initialize_system();

    let fungible_token = FungibleToken::initialize(&system);
    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut supply_chain = SupplyChain::initialize(
        &system,
        fungible_token.actor_id(),
        non_fungible_token.actor_id(),
    );

    let item_infos = (0..=5)
        .map(|item_id| {
            supply_chain.produce(PRODUCER).succeed(item_id);

            (
                item_id.into(),
                ItemInfo {
                    producer: PRODUCER.into(),
                    distributor: Default::default(),
                    retailer: Default::default(),

                    state: ItemState {
                        state: Default::default(),
                        by: Role::Producer,
                    },
                    price: Default::default(),
                    delivery_time: Default::default(),
                },
            )
        })
        .collect();

    supply_chain.meta_state().existing_items().eq(item_infos);
}
