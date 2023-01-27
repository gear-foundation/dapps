use utils::{prelude::*, FungibleToken, NonFungibleToken};

pub mod utils;

#[test]
fn state() {
    let system = utils::initialize_system();

    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut fungible_token = FungibleToken::initialize(&system);
    let mut supply_chain = SupplyChain::initialize(
        &system,
        fungible_token.actor_id(),
        non_fungible_token.actor_id(),
    );

    for from in [DISTRIBUTOR, RETAILER, CONSUMER] {
        // Double balances to catch bugs.
        fungible_token.mint(from, ITEM_PRICE * 2);
        fungible_token.approve(from, supply_chain.actor_id(), ITEM_PRICE * 2);
    }

    supply_chain.produce(PRODUCER).succeed(0);

    supply_chain
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .succeed(0);
    supply_chain
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .failed(Error::UnexpectedItemState);

    supply_chain
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .failed(Error::UnexpectedItemState);

    supply_chain
        .approve_by_producer(PRODUCER, 0, true)
        .succeed((0, true));
    supply_chain
        .approve_by_producer(PRODUCER, 0, true)
        .failed(Error::UnexpectedItemState);

    supply_chain.ship_by_producer(PRODUCER, 0).succeed(0);
    supply_chain
        .ship_by_producer(PRODUCER, 0)
        .failed(Error::UnexpectedItemState);

    supply_chain
        .receive_by_distributor(DISTRIBUTOR, 0)
        .succeed(0);
    supply_chain
        .receive_by_distributor(DISTRIBUTOR, 0)
        .failed(Error::UnexpectedItemState);

    supply_chain.process(DISTRIBUTOR, 0).succeed(0);
    supply_chain
        .process(DISTRIBUTOR, 0)
        .failed(Error::UnexpectedItemState);

    supply_chain.package(DISTRIBUTOR, 0).succeed(0);
    supply_chain
        .package(DISTRIBUTOR, 0)
        .failed(Error::UnexpectedItemState);

    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .succeed(0);
    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .failed(Error::UnexpectedItemState);

    supply_chain
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .failed(Error::UnexpectedItemState);

    supply_chain
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .succeed((0, true));
    supply_chain
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .failed(Error::UnexpectedItemState);

    supply_chain.ship_by_distributor(DISTRIBUTOR, 0).succeed(0);
    supply_chain
        .ship_by_distributor(DISTRIBUTOR, 0)
        .failed(Error::UnexpectedItemState);

    supply_chain.receive_by_retailer(RETAILER, 0).succeed(0);
    supply_chain
        .receive_by_retailer(RETAILER, 0)
        .failed(Error::UnexpectedItemState);

    supply_chain
        .put_up_for_sale_by_retailer(RETAILER, 0, ITEM_PRICE)
        .succeed(0);
    supply_chain
        .put_up_for_sale_by_retailer(RETAILER, 0, ITEM_PRICE)
        .failed(Error::UnexpectedItemState);

    supply_chain.purchase_by_consumer(CONSUMER, 0).succeed(0);
    supply_chain
        .purchase_by_consumer(CONSUMER, 0)
        .failed(Error::UnexpectedItemState);
}
