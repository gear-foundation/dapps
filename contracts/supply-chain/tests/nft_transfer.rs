use utils::{prelude::*, FungibleToken, NonFungibleToken};

pub mod utils;

#[test]
fn nft_transfer() {
    let system = utils::initialize_system();

    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut fungible_token = FungibleToken::initialize(&system);
    let mut supply_chain = SupplyChain::initialize(
        &system,
        fungible_token.actor_id(),
        non_fungible_token.actor_id(),
    );

    for from in [DISTRIBUTOR, RETAILER, CONSUMER] {
        fungible_token.mint(from, ITEM_PRICE);
        fungible_token.approve(from, supply_chain.actor_id(), ITEM_PRICE);
    }

    supply_chain.produce(PRODUCER).succeed(0);
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(PRODUCER.into());

    supply_chain
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .succeed(0);
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(supply_chain.actor_id());

    supply_chain
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_producer(PRODUCER, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_producer(PRODUCER, 0).succeed(0);

    supply_chain
        .receive_by_distributor(DISTRIBUTOR, 0)
        .succeed(0);
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(DISTRIBUTOR.into());

    supply_chain.process(DISTRIBUTOR, 0).succeed(0);
    supply_chain.package(DISTRIBUTOR, 0).succeed(0);

    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .succeed(0);
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(supply_chain.actor_id());

    supply_chain
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_distributor(DISTRIBUTOR, 0).succeed(0);

    supply_chain.receive_by_retailer(RETAILER, 0).succeed(0);
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(RETAILER.into());

    supply_chain
        .put_up_for_sale_by_retailer(RETAILER, 0, ITEM_PRICE)
        .succeed(0);
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(supply_chain.actor_id());

    supply_chain.purchase_by_consumer(CONSUMER, 0).succeed(0);
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(CONSUMER.into());

    supply_chain.meta_state().item_info(0).eq(Some(ItemInfo {
        producer: PRODUCER.into(),
        distributor: DISTRIBUTOR.into(),
        retailer: RETAILER.into(),

        state: ItemState {
            state: ItemEventState::Purchased,
            by: Role::Consumer,
        },
        price: ITEM_PRICE,
        delivery_time: DELIVERY_TIME,
    }));
    non_fungible_token
        .meta_state()
        .owner_id(0)
        .eq(CONSUMER.into())
}
