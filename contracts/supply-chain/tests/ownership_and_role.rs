pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken};

// Pairs of participants are needed here to test ownership of items.
const PRODUCER: [u64; 2] = [5, 6];
const DISTRIBUTOR: [u64; 2] = [7, 8];
const RETAILER: [u64; 2] = [9, 10];

#[test]
fn ownership_and_role() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    ft_program.mint(DISTRIBUTOR[0], ITEM_PRICE);
    ft_program.mint(RETAILER[0], ITEM_PRICE);
    ft_program.mint(CONSUMER, ITEM_PRICE);

    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program = SupplyChain::initialize_custom(
        &system,
        InitSupplyChain {
            producers: [PRODUCER[0].into(), PRODUCER[1].into()].into(),
            distributors: [DISTRIBUTOR[0].into(), DISTRIBUTOR[1].into()].into(),
            retailers: [RETAILER[0].into(), RETAILER[1].into()].into(),

            ft_program: ft_program.actor_id(),
            nft_program: nft_program.actor_id(),
        },
    )
    .succeed();

    // Should fail because `msg::source()` must be a producer in the supply
    // chain.
    schain_program.produce(FOREIGN_USER).failed();
    schain_program.produce(PRODUCER[0]).check(0);

    // Should fail because `msg::source()` must be a producer in the supply
    // chain.
    schain_program
        .put_up_for_sale_by_producer(FOREIGN_USER, 0, ITEM_PRICE)
        .failed();
    // Should fail because `msg::source()` must be the producer of the item in
    // the supply chain.
    schain_program
        .put_up_for_sale_by_producer(PRODUCER[1], 0, ITEM_PRICE)
        .failed();
    schain_program
        .put_up_for_sale_by_producer(PRODUCER[0], 0, ITEM_PRICE)
        .check(0);

    // Should fail because `msg::source()` must be a distributor in the supply
    // chain.
    schain_program
        .purchase_by_distributor(FOREIGN_USER, 0, DELIVERY_TIME)
        .failed();
    schain_program
        .purchase_by_distributor(DISTRIBUTOR[0], 0, DELIVERY_TIME)
        .check(0);

    // Should fail because `msg::source()` must be a producer in the supply
    // chain.
    schain_program
        .approve_by_producer(FOREIGN_USER, 0, true)
        .failed();
    // Should fail because `msg::source()` must be the producer of the item in
    // the supply chain.
    schain_program
        .approve_by_producer(PRODUCER[1], 0, true)
        .failed();
    schain_program
        .approve_by_producer(PRODUCER[0], 0, true)
        .check(0);

    // Should fail because `msg::source()` must be a producer in the supply
    // chain.
    schain_program.ship_by_producer(FOREIGN_USER, 0).failed();
    // Should fail because `msg::source()` must be the producer of the item in
    // the supply chain.
    schain_program.ship_by_producer(PRODUCER[1], 0).failed();
    schain_program.ship_by_producer(PRODUCER[0], 0).check(0);

    // Should fail because `msg::source()` must be a distributor in the supply
    // chain.
    schain_program
        .receive_by_distributor(FOREIGN_USER, 0)
        .failed();
    // Should fail because `msg::source()` must be the distributor of the item
    // in the supply chain.
    schain_program
        .receive_by_distributor(DISTRIBUTOR[1], 0)
        .failed();
    schain_program
        .receive_by_distributor(DISTRIBUTOR[0], 0)
        .check(0);

    // Should fail because `msg::source()` must be a distributor in the supply
    // chain.
    schain_program
        .process_by_distributor(FOREIGN_USER, 0)
        .failed();
    // Should fail because `msg::source()` must be the distributor of the item
    // in the supply chain.
    schain_program
        .process_by_distributor(DISTRIBUTOR[1], 0)
        .failed();
    schain_program
        .process_by_distributor(DISTRIBUTOR[0], 0)
        .check(0);

    // Should fail because `msg::source()` must be a distributor in the supply
    // chain.
    schain_program
        .package_by_distributor(FOREIGN_USER, 0)
        .failed();
    // Should fail because `msg::source()` must be the distributor of the item
    // in the supply chain.
    schain_program
        .package_by_distributor(DISTRIBUTOR[1], 0)
        .failed();
    schain_program
        .package_by_distributor(DISTRIBUTOR[0], 0)
        .check(0);

    // Should fail because `msg::source()` must be a distributor in the supply
    // chain.
    schain_program
        .put_up_for_sale_by_distributor(FOREIGN_USER, 0, ITEM_PRICE)
        .failed();
    // Should fail because `msg::source()` must be the distributor of the item
    // in the supply chain.
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR[1], 0, ITEM_PRICE)
        .failed();
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR[0], 0, ITEM_PRICE)
        .check(0);

    // Should fail because `msg::source()` must be a retailer in the supply
    // chain.
    schain_program
        .purchase_by_retailer(FOREIGN_USER, 0, DELIVERY_TIME)
        .failed();
    schain_program
        .purchase_by_retailer(RETAILER[0], 0, DELIVERY_TIME)
        .check(0);

    // Should fail because `msg::source()` must be a distributor in the supply
    // chain.
    schain_program
        .approve_by_distributor(FOREIGN_USER, 0, true)
        .failed();
    // Should fail because `msg::source()` must be the distributor of the item
    // in the supply chain.
    schain_program
        .approve_by_distributor(DISTRIBUTOR[1], 0, true)
        .failed();
    schain_program
        .approve_by_distributor(DISTRIBUTOR[0], 0, true)
        .check(0);

    // Should fail because `msg::source()` must be a distributor in the supply
    // chain.
    schain_program.ship_by_distributor(FOREIGN_USER, 0).failed();
    // Should fail because `msg::source()` must be the distributor of the item
    // in the supply chain.
    schain_program
        .ship_by_distributor(DISTRIBUTOR[1], 0)
        .failed();
    schain_program
        .ship_by_distributor(DISTRIBUTOR[0], 0)
        .check(0);

    // Should fail because `msg::source()` must be a retailer in the supply
    // chain.
    schain_program.receive_by_retailer(FOREIGN_USER, 0).failed();
    // Should fail because `msg::source()` must be the retailer of the item in
    // the supply chain.
    schain_program.receive_by_retailer(RETAILER[1], 0).failed();
    schain_program.receive_by_retailer(RETAILER[0], 0).check(0);

    // Should fail because `msg::source()` must be a retailer in the supply
    // chain.
    schain_program
        .put_up_for_sale_by_retailer(FOREIGN_USER, 0, ITEM_PRICE)
        .failed();
    // Should fail because `msg::source()` must be the retailer of the item in
    // the supply chain.
    schain_program
        .put_up_for_sale_by_retailer(RETAILER[1], 0, ITEM_PRICE)
        .failed();
}

#[test]
fn query_roles() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    let nft_program = NonFungibleToken::initialize(&system);

    let mut schain_program = SupplyChain::initialize_custom(
        &system,
        InitSupplyChain {
            producers: [FOREIGN_USER.into()].into(),
            distributors: [FOREIGN_USER.into()].into(),
            retailers: [FOREIGN_USER.into()].into(),

            ft_program: ft_program.actor_id(),
            nft_program: nft_program.actor_id(),
        },
    )
    .succeed();
    schain_program
        .meta_state()
        .roles(FOREIGN_USER)
        .check([Role::Producer, Role::Distributor, Role::Retailer].into());

    schain_program = SupplyChain::initialize_custom(
        &system,
        InitSupplyChain {
            producers: [].into(),
            distributors: [].into(),
            retailers: [].into(),

            ft_program: ft_program.actor_id(),
            nft_program: nft_program.actor_id(),
        },
    )
    .succeed();
    schain_program
        .meta_state()
        .roles(FOREIGN_USER)
        .check([].into());
}
