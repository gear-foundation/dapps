use utils::{prelude::*, FungibleToken, NonFungibleToken};

pub mod utils;

const DELIVERY_TIME_IN_BLOCKS: u32 = (DELIVERY_TIME / 1000) as _;

#[test]
fn delivery_wo_delay() {
    const NO_DELAY: u32 = DELIVERY_TIME_IN_BLOCKS;

    let system = utils::initialize_system();

    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut fungible_token = FungibleToken::initialize(&system);
    let mut supply_chain = SupplyChain::initialize(
        &system,
        fungible_token.actor_id(),
        non_fungible_token.actor_id(),
    );

    for from in [DISTRIBUTOR, RETAILER] {
        fungible_token.mint(from, ITEM_PRICE);
        fungible_token.approve(from, supply_chain.actor_id(), ITEM_PRICE);
    }

    supply_chain.produce(PRODUCER).succeed(0);
    supply_chain
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .succeed(0);
    supply_chain
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_producer(PRODUCER, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_producer(PRODUCER, 0).succeed(0);

    system.spend_blocks(NO_DELAY);
    supply_chain
        .receive_by_distributor(DISTRIBUTOR, 0)
        .succeed(0);
    // Since the delivery is completed on time,
    // all fungible tokens are transferred to the producer (seller).
    fungible_token.balance(PRODUCER).contains(ITEM_PRICE);
    fungible_token.balance(DISTRIBUTOR).contains(0);

    supply_chain.process(DISTRIBUTOR, 0).succeed(0);
    supply_chain.package(DISTRIBUTOR, 0).succeed(0);
    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .succeed(0);
    supply_chain
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_distributor(DISTRIBUTOR, 0).succeed(0);

    system.spend_blocks(NO_DELAY);
    supply_chain.receive_by_retailer(RETAILER, 0).succeed(0);
    // Since the delivery is completed on time,
    // all fungible tokens are transferred to the distributor (seller).
    fungible_token.balance(DISTRIBUTOR).contains(ITEM_PRICE);
    fungible_token.balance(RETAILER).contains(0);
}

#[test]
fn delivery_with_delay() {
    // Even and odd prices required for a reliable penalty calculation check.
    const ITEM_PRICE: [u128; 2] = [123123, 12341234];
    const DELAY: u32 = DELIVERY_TIME_IN_BLOCKS * 2 - 1;

    let system = utils::initialize_system();

    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut fungible_token = FungibleToken::initialize(&system);
    let mut supply_chain = SupplyChain::initialize(
        &system,
        fungible_token.actor_id(),
        non_fungible_token.actor_id(),
    );

    for (from, amount) in [(DISTRIBUTOR, ITEM_PRICE[0]), (RETAILER, ITEM_PRICE[1])] {
        fungible_token.mint(from, amount);
        fungible_token.approve(from, supply_chain.actor_id(), amount);
    }

    supply_chain.produce(PRODUCER).succeed(0);
    supply_chain
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE[0])
        .succeed(0);
    supply_chain
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_producer(PRODUCER, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_producer(PRODUCER, 0).succeed(0);

    system.spend_blocks(DELAY);
    supply_chain
        .receive_by_distributor(DISTRIBUTOR, 0)
        .succeed(0);
    // Since the delivery is completed with the delay,
    // the half of fungible tokens is transferred to the producer (seller)
    // and the other half of them is refunded to the distributor (buyer).
    fungible_token.balance(PRODUCER).contains(ITEM_PRICE[0] / 2);
    fungible_token
        .balance(DISTRIBUTOR)
        .contains(ITEM_PRICE[0] - ITEM_PRICE[0] / 2);

    supply_chain.process(DISTRIBUTOR, 0).succeed(0);
    supply_chain.package(DISTRIBUTOR, 0).succeed(0);
    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE[1])
        .succeed(0);
    supply_chain
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_distributor(DISTRIBUTOR, 0).succeed(0);

    system.spend_blocks(DELAY);
    supply_chain.receive_by_retailer(RETAILER, 0).succeed(0);
    // Since the delivery is completed with the delay,
    // the half of fungible tokens is transferred to the distributor (seller)
    // and the other half of them is refunded to the retailer (buyer).
    fungible_token
        .balance(DISTRIBUTOR)
        .contains(ITEM_PRICE[0] - ITEM_PRICE[0] / 2 + ITEM_PRICE[1] / 2);
    fungible_token
        .balance(RETAILER)
        .contains(ITEM_PRICE[1] - ITEM_PRICE[1] / 2);
}

#[test]
fn delivery_with_big_delay() {
    const BIG_DELAY: u32 = DELIVERY_TIME_IN_BLOCKS * 2;

    let system = utils::initialize_system();

    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut fungible_token = FungibleToken::initialize(&system);

    let mut supply_chain = SupplyChain::initialize(
        &system,
        fungible_token.actor_id(),
        non_fungible_token.actor_id(),
    );

    for from in [DISTRIBUTOR, RETAILER] {
        fungible_token.mint(from, ITEM_PRICE);
        fungible_token.approve(from, supply_chain.actor_id(), ITEM_PRICE);
    }

    supply_chain.produce(PRODUCER).succeed(0);
    supply_chain
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .succeed(0);
    supply_chain
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_producer(PRODUCER, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_producer(PRODUCER, 0).succeed(0);

    system.spend_blocks(BIG_DELAY);
    supply_chain
        .receive_by_distributor(DISTRIBUTOR, 0)
        .succeed(0);
    // Since the delivery is completed with the big delay,
    // all fungible tokens are refunded to the distributor (buyer).
    fungible_token.balance(PRODUCER).contains(0);
    fungible_token.balance(DISTRIBUTOR).contains(ITEM_PRICE);

    supply_chain.process(DISTRIBUTOR, 0).succeed(0);
    supply_chain.package(DISTRIBUTOR, 0).succeed(0);
    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .succeed(0);
    supply_chain
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .succeed(0);
    supply_chain
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .succeed((0, true));
    supply_chain.ship_by_distributor(DISTRIBUTOR, 0).succeed(0);

    system.spend_blocks(BIG_DELAY);
    supply_chain.receive_by_retailer(RETAILER, 0).succeed(0);
    // Since the delivery is completed with the big delay,
    // all fungible tokens are refunded to the retailer (buyer).
    fungible_token.balance(DISTRIBUTOR).contains(ITEM_PRICE);
    fungible_token.balance(RETAILER).contains(ITEM_PRICE);
}
