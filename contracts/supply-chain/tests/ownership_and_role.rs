use utils::{prelude::*, FungibleToken, NonFungibleToken};

pub mod utils;

// Pairs of participants are needed here to test ownership of items.
const PRODUCER: [u64; 2] = [5, 6];
const DISTRIBUTOR: [u64; 2] = [7, 8];
const RETAILER: [u64; 2] = [9, 10];

#[test]
fn ownership_and_role() {
    let system = utils::initialize_system();

    let non_fungible_token = NonFungibleToken::initialize(&system);
    let mut fungible_token = FungibleToken::initialize(&system);
    let mut supply_chain = SupplyChain::initialize_custom(
        &system,
        Initialize {
            producers: [PRODUCER[0].into(), PRODUCER[1].into()].into(),
            distributors: [DISTRIBUTOR[0].into(), DISTRIBUTOR[1].into()].into(),
            retailers: [RETAILER[0].into(), RETAILER[1].into()].into(),

            fungible_token: fungible_token.actor_id(),
            non_fungible_token: non_fungible_token.actor_id(),
        },
    )
    .succeed();

    for from in [DISTRIBUTOR[0], RETAILER[0]] {
        fungible_token.mint(from, ITEM_PRICE);
        fungible_token.approve(from, supply_chain.actor_id(), ITEM_PRICE);
    }

    // Should fail because `msg::source()` must be a producer.
    supply_chain
        .produce(FOREIGN_USER)
        .failed(Error::AccessRestricted);
    supply_chain.produce(PRODUCER[0]).succeed(0);

    supply_chain
        .put_up_for_sale_by_producer(FOREIGN_USER, 0, ITEM_PRICE)
        .failed(Error::AccessRestricted);
    supply_chain
        .put_up_for_sale_by_producer(PRODUCER[1], 0, ITEM_PRICE)
        .failed(Error::AccessRestricted);
    supply_chain
        .put_up_for_sale_by_producer(PRODUCER[0], 0, ITEM_PRICE)
        .succeed(0);

    supply_chain
        .purchase_by_distributor(FOREIGN_USER, 0, DELIVERY_TIME)
        .failed(Error::AccessRestricted);
    supply_chain
        .purchase_by_distributor(DISTRIBUTOR[0], 0, DELIVERY_TIME)
        .succeed(0);

    supply_chain
        .approve_by_producer(FOREIGN_USER, 0, true)
        .failed(Error::AccessRestricted);
    supply_chain
        .approve_by_producer(PRODUCER[1], 0, true)
        .failed(Error::AccessRestricted);
    supply_chain
        .approve_by_producer(PRODUCER[0], 0, true)
        .succeed((0, true));

    supply_chain
        .ship_by_producer(FOREIGN_USER, 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .ship_by_producer(PRODUCER[1], 0)
        .failed(Error::AccessRestricted);
    supply_chain.ship_by_producer(PRODUCER[0], 0).succeed(0);

    supply_chain
        .receive_by_distributor(FOREIGN_USER, 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .receive_by_distributor(DISTRIBUTOR[1], 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .receive_by_distributor(DISTRIBUTOR[0], 0)
        .succeed(0);

    supply_chain
        .process(FOREIGN_USER, 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .process(DISTRIBUTOR[1], 0)
        .failed(Error::AccessRestricted);
    supply_chain.process(DISTRIBUTOR[0], 0).succeed(0);

    supply_chain
        .package(FOREIGN_USER, 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .package(DISTRIBUTOR[1], 0)
        .failed(Error::AccessRestricted);
    supply_chain.package(DISTRIBUTOR[0], 0).succeed(0);

    supply_chain
        .put_up_for_sale_by_distributor(FOREIGN_USER, 0, ITEM_PRICE)
        .failed(Error::AccessRestricted);
    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR[1], 0, ITEM_PRICE)
        .failed(Error::AccessRestricted);
    supply_chain
        .put_up_for_sale_by_distributor(DISTRIBUTOR[0], 0, ITEM_PRICE)
        .succeed(0);

    supply_chain
        .purchase_by_retailer(FOREIGN_USER, 0, DELIVERY_TIME)
        .failed(Error::AccessRestricted);
    supply_chain
        .purchase_by_retailer(RETAILER[0], 0, DELIVERY_TIME)
        .succeed(0);

    supply_chain
        .approve_by_distributor(FOREIGN_USER, 0, true)
        .failed(Error::AccessRestricted);
    supply_chain
        .approve_by_distributor(DISTRIBUTOR[1], 0, true)
        .failed(Error::AccessRestricted);
    supply_chain
        .approve_by_distributor(DISTRIBUTOR[0], 0, true)
        .succeed((0, true));

    supply_chain
        .ship_by_distributor(FOREIGN_USER, 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .ship_by_distributor(DISTRIBUTOR[1], 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .ship_by_distributor(DISTRIBUTOR[0], 0)
        .succeed(0);

    supply_chain
        .receive_by_retailer(FOREIGN_USER, 0)
        .failed(Error::AccessRestricted);
    supply_chain
        .receive_by_retailer(RETAILER[1], 0)
        .failed(Error::AccessRestricted);
    supply_chain.receive_by_retailer(RETAILER[0], 0).succeed(0);

    supply_chain
        .put_up_for_sale_by_retailer(FOREIGN_USER, 0, ITEM_PRICE)
        .failed(Error::AccessRestricted);
    supply_chain
        .put_up_for_sale_by_retailer(RETAILER[1], 0, ITEM_PRICE)
        .failed(Error::AccessRestricted);
}

#[test]
fn query_roles() {
    let system = utils::initialize_system();

    let fungible_token = FungibleToken::initialize(&system);
    let non_fungible_token = NonFungibleToken::initialize(&system);

    let mut supply_chain = SupplyChain::initialize_custom(
        &system,
        Initialize {
            producers: vec![FOREIGN_USER.into()],
            distributors: vec![FOREIGN_USER.into()],
            retailers: vec![FOREIGN_USER.into()],

            fungible_token: fungible_token.actor_id(),
            non_fungible_token: non_fungible_token.actor_id(),
        },
    )
    .succeed();
    supply_chain.meta_state().roles(FOREIGN_USER).eq([
        Role::Consumer,
        Role::Producer,
        Role::Distributor,
        Role::Retailer,
    ]
    .into());

    supply_chain = SupplyChain::initialize_custom(
        &system,
        Initialize {
            producers: [].into(),
            distributors: [].into(),
            retailers: [].into(),

            fungible_token: fungible_token.actor_id(),
            non_fungible_token: non_fungible_token.actor_id(),
        },
    )
    .succeed();
    supply_chain
        .meta_state()
        .roles(FOREIGN_USER)
        .eq([Role::Consumer].into());
}
