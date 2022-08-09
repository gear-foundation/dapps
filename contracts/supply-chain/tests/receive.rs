pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken};

const DELIVERY_TIME_IN_BLOCKS: u32 = (DELIVERY_TIME / 1000) as _;

#[test]
fn delivery_wo_delay() {
    const NO_DELAY: u32 = DELIVERY_TIME_IN_BLOCKS;

    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    ft_program.mint(DISTRIBUTOR, ITEM_PRICE);
    ft_program.mint(RETAILER, ITEM_PRICE);

    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    schain_program.produce(PRODUCER).check(0);
    schain_program
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .check(0);
    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_producer(PRODUCER, 0, true)
        .check(0);
    schain_program.ship_by_producer(PRODUCER, 0).check(0);

    system.spend_blocks(NO_DELAY);
    schain_program
        .receive_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    // Since the delivery is completed on time,
    // all fungible tokens are transferred to the producer (seller).
    ft_program.balance_of(PRODUCER).check(ITEM_PRICE);
    ft_program.balance_of(DISTRIBUTOR).check(0);

    schain_program
        .process_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .package_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .check(0);
    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .check(0);
    schain_program.ship_by_distributor(DISTRIBUTOR, 0).check(0);

    system.spend_blocks(NO_DELAY);
    schain_program.receive_by_retailer(RETAILER, 0).check(0);
    // Since the delivery is completed on time,
    // all fungible tokens are transferred to the distributor (seller).
    ft_program.balance_of(DISTRIBUTOR).check(ITEM_PRICE);
    ft_program.balance_of(RETAILER).check(0);
}

#[test]
fn delivery_with_delay() {
    // Even and odd prices required for a reliable penalty calculation check.
    const ITEM_PRICE: [u128; 2] = [123123, 12341234];
    const DELAY: u32 = DELIVERY_TIME_IN_BLOCKS * 2 - 1;

    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    ft_program.mint(DISTRIBUTOR, ITEM_PRICE[0]);
    ft_program.mint(RETAILER, ITEM_PRICE[1]);

    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    schain_program.produce(PRODUCER).check(0);
    schain_program
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE[0])
        .check(0);
    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_producer(PRODUCER, 0, true)
        .check(0);
    schain_program.ship_by_producer(PRODUCER, 0).check(0);

    system.spend_blocks(DELAY);
    schain_program
        .receive_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    // Since the delivery is completed with the delay,
    // the half of fungible tokens is transferred to the producer (seller)
    // and the other half of them is refunded to the distributor (buyer).
    ft_program.balance_of(PRODUCER).check(ITEM_PRICE[0] / 2);
    ft_program
        .balance_of(DISTRIBUTOR)
        .check(ITEM_PRICE[0] - ITEM_PRICE[0] / 2);

    schain_program
        .process_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .package_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE[1])
        .check(0);
    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .check(0);
    schain_program.ship_by_distributor(DISTRIBUTOR, 0).check(0);

    system.spend_blocks(DELAY);
    schain_program.receive_by_retailer(RETAILER, 0).check(0);
    // Since the delivery is completed with the delay,
    // the half of fungible tokens is transferred to the distributor (seller)
    // and the other half of them is refunded to the retailer (buyer).
    ft_program
        .balance_of(DISTRIBUTOR)
        .check(ITEM_PRICE[0] - ITEM_PRICE[0] / 2 + ITEM_PRICE[1] / 2);
    ft_program
        .balance_of(RETAILER)
        .check(ITEM_PRICE[1] - ITEM_PRICE[1] / 2);
}

#[test]
fn delivery_with_big_delay() {
    const BIG_DELAY: u32 = DELIVERY_TIME_IN_BLOCKS * 2;

    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    ft_program.mint(DISTRIBUTOR, ITEM_PRICE);
    ft_program.mint(RETAILER, ITEM_PRICE);

    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    schain_program.produce(PRODUCER).check(0);
    schain_program
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .check(0);
    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_producer(PRODUCER, 0, true)
        .check(0);
    schain_program.ship_by_producer(PRODUCER, 0).check(0);

    system.spend_blocks(BIG_DELAY);
    schain_program
        .receive_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    // Since the delivery is completed with the big delay,
    // all fungible tokens are refunded to the distributor (buyer).
    ft_program.balance_of(PRODUCER).check(0);
    ft_program.balance_of(DISTRIBUTOR).check(ITEM_PRICE);

    schain_program
        .process_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .package_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .check(0);
    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .check(0);
    schain_program.ship_by_distributor(DISTRIBUTOR, 0).check(0);

    system.spend_blocks(BIG_DELAY);
    schain_program.receive_by_retailer(RETAILER, 0).check(0);
    // Since the delivery is completed with the big delay,
    // all fungible tokens are refunded to the retailer (buyer).
    ft_program.balance_of(DISTRIBUTOR).check(ITEM_PRICE);
    ft_program.balance_of(RETAILER).check(ITEM_PRICE);
}
