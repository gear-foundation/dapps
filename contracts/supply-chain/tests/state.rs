pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken};

#[test]
fn state() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    // Double the balances to catch bugs.
    ft_program.mint(DISTRIBUTOR, ITEM_PRICE * 2);
    ft_program.mint(RETAILER, ITEM_PRICE * 2);
    ft_program.mint(CONSUMER, ITEM_PRICE * 2);

    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    schain_program.produce(PRODUCER).check(0);

    schain_program
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .check(0);
    // Should fail because item's `ItemState` must be `Produced`.
    schain_program
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .failed();

    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .check(0);
    // Should fail because item's `ItemState` must be `ForSaleByProducer`.
    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .failed();

    schain_program
        .approve_by_producer(PRODUCER, 0, true)
        .check(0);
    //Should fail because item's `ItemState` must be `PurchasedByDistributor`.
    schain_program
        .approve_by_producer(PRODUCER, 0, true)
        .failed();

    schain_program.ship_by_producer(PRODUCER, 0).check(0);
    // Should fail because item's `ItemState` must be `ApprovedByProducer`.
    schain_program.ship_by_producer(PRODUCER, 0).failed();

    schain_program
        .receive_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    // Should fail because item's `ItemState` must be `ShippedByProducer`.
    schain_program
        .receive_by_distributor(DISTRIBUTOR, 0)
        .failed();

    schain_program
        .process_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    // Should fail because item's `ItemState` must be `ReceivedByDistributor`.
    schain_program
        .process_by_distributor(DISTRIBUTOR, 0)
        .failed();

    schain_program
        .package_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    // Should fail because item's `ItemState` must be `ProcessedByDistributor`.
    schain_program
        .package_by_distributor(DISTRIBUTOR, 0)
        .failed();

    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .check(0);
    // Should fail because item's `ItemState` must be `PackagedByDistributor`.
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .failed();

    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .check(0);
    // Should fail because item's `ItemState` must be `ForSaleByDistributor`.
    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .failed();

    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .check(0);
    // Should fail because item's `ItemState` must be `PurchasedByRetailer`.
    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .failed();

    schain_program.ship_by_distributor(DISTRIBUTOR, 0).check(0);
    // Should fail because item's `ItemState` must be `ApprovedByDistributor`.
    schain_program.ship_by_distributor(DISTRIBUTOR, 0).failed();

    schain_program.receive_by_retailer(RETAILER, 0).check(0);
    // Should fail because item's `ItemState` must be `ShippedByDistributor`.
    schain_program.receive_by_retailer(RETAILER, 0).failed();

    schain_program
        .put_up_for_sale_by_retailer(RETAILER, 0, ITEM_PRICE)
        .check(0);
    // Should fail because item's `ItemState` must be `ReceivedByRetailer`.
    schain_program
        .put_up_for_sale_by_retailer(RETAILER, 0, ITEM_PRICE)
        .failed();

    schain_program.purchase_by_consumer(CONSUMER, 0).check(0);
    // Should fail because item's `ItemState` must be `ForSaleByRetailer`.
    schain_program.purchase_by_consumer(CONSUMER, 0).failed();
}
