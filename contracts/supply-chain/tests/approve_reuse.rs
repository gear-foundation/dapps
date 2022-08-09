pub mod utils;
use utils::{prelude::*, FungibleToken, NonFungibleToken};

const ITEM_PRICE_BY_PRODUCER: u128 = ITEM_PRICE;
const ITEM_PRICE_BY_DISTRIBUTOR: u128 = ITEM_PRICE * 2;
const ITEM_PRICE_BY_RETAILER: u128 = ITEM_PRICE * 3;

#[test]
fn approve_reuse_and_ft_transfer() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    ft_program.mint(DISTRIBUTOR, ITEM_PRICE_BY_PRODUCER);
    ft_program.mint(RETAILER, ITEM_PRICE_BY_DISTRIBUTOR);
    ft_program.mint(CONSUMER, ITEM_PRICE_BY_RETAILER);

    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    schain_program.produce(PRODUCER).check(0);
    schain_program
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE_BY_PRODUCER)
        .check(0);
    schain_program
        .meta_state()
        .item_price(0)
        .check(ITEM_PRICE_BY_PRODUCER);

    // There may be a case when a buyer puts an inconvenient delivery time for a
    // seller.
    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .check(0);
    ft_program
        .balance_of(schain_program.actor_id())
        .check(ITEM_PRICE_BY_PRODUCER);
    // Then a seller can cancel this purchase and put its item back up for sale.
    schain_program
        .approve_by_producer(PRODUCER, 0, false)
        .check(0);
    ft_program
        .balance_of(DISTRIBUTOR)
        .check(ITEM_PRICE_BY_PRODUCER);
    // Thereafter the same buyer or another can purchase this item again and put
    // a more convenient delivery time for a seller...
    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .check(0);
    ft_program
        .balance_of(schain_program.actor_id())
        .check(ITEM_PRICE_BY_PRODUCER);
    // ...who will approve this purchase and ship the item later.
    schain_program
        .approve_by_producer(PRODUCER, 0, true)
        .check(0);

    schain_program.ship_by_producer(PRODUCER, 0).check(0);
    schain_program
        .receive_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .process_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .package_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE_BY_DISTRIBUTOR)
        .check(0);
    schain_program
        .meta_state()
        .item_price(0)
        .check(ITEM_PRICE_BY_DISTRIBUTOR);

    // There may be a case when a buyer puts an inconvenient delivery time for a
    // seller.
    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .check(0);
    ft_program
        .balance_of(schain_program.actor_id())
        .check(ITEM_PRICE_BY_DISTRIBUTOR);
    // Then a seller can cancel this purchase and put its item back up for sale.
    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, false)
        .check(0);
    ft_program
        .balance_of(RETAILER)
        .check(ITEM_PRICE_BY_DISTRIBUTOR);
    // Thereafter the same buyer or another can purchase this item again and put
    // a more convenient delivery time for a seller...
    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .check(0);
    ft_program
        .balance_of(schain_program.actor_id())
        .check(ITEM_PRICE_BY_DISTRIBUTOR);
    // ...who will approve this purchase and ship the item later.
    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .check(0);

    schain_program.ship_by_distributor(DISTRIBUTOR, 0).check(0);
    schain_program.receive_by_retailer(RETAILER, 0).check(0);
    schain_program
        .put_up_for_sale_by_retailer(RETAILER, 0, ITEM_PRICE_BY_RETAILER)
        .check(0);
    schain_program
        .meta_state()
        .item_price(0)
        .check(ITEM_PRICE_BY_RETAILER);
    schain_program.purchase_by_consumer(CONSUMER, 0).check(0);
}
