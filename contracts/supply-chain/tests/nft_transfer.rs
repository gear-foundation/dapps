pub mod utils;
use gear_lib::non_fungible_token::token::Token;
use utils::{prelude::*, FungibleToken, NonFungibleToken};

#[test]
fn nft_transfer() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    ft_program.mint(DISTRIBUTOR, ITEM_PRICE);
    ft_program.mint(RETAILER, ITEM_PRICE);
    ft_program.mint(CONSUMER, ITEM_PRICE);

    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    schain_program.produce(PRODUCER).check(0);
    nft_program.meta_state().owner_id(0).check(PRODUCER.into());

    schain_program
        .put_up_for_sale_by_producer(PRODUCER, 0, ITEM_PRICE)
        .check(0);
    nft_program
        .meta_state()
        .owner_id(0)
        .check(schain_program.actor_id());

    schain_program
        .purchase_by_distributor(DISTRIBUTOR, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_producer(PRODUCER, 0, true)
        .check(0);
    schain_program.ship_by_producer(PRODUCER, 0).check(0);

    schain_program
        .receive_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    nft_program
        .meta_state()
        .owner_id(0)
        .check(DISTRIBUTOR.into());

    schain_program
        .process_by_distributor(DISTRIBUTOR, 0)
        .check(0);
    schain_program
        .package_by_distributor(DISTRIBUTOR, 0)
        .check(0);

    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, 0, ITEM_PRICE)
        .check(0);
    nft_program
        .meta_state()
        .owner_id(0)
        .check(schain_program.actor_id());

    schain_program
        .purchase_by_retailer(RETAILER, 0, DELIVERY_TIME)
        .check(0);
    schain_program
        .approve_by_distributor(DISTRIBUTOR, 0, true)
        .check(0);
    schain_program.ship_by_distributor(DISTRIBUTOR, 0).check(0);

    schain_program.receive_by_retailer(RETAILER, 0).check(0);
    nft_program.meta_state().owner_id(0).check(RETAILER.into());

    schain_program
        .put_up_for_sale_by_retailer(RETAILER, 0, ITEM_PRICE)
        .check(0);
    nft_program
        .meta_state()
        .owner_id(0)
        .check(schain_program.actor_id());

    schain_program.purchase_by_consumer(CONSUMER, 0).check(0);
    nft_program.meta_state().owner_id(0).check(CONSUMER.into());

    schain_program.meta_state().item_info(0).check(ItemInfo {
        producer: PRODUCER.into(),
        distributor: DISTRIBUTOR.into(),
        retailer: RETAILER.into(),

        state: ItemState::PurchasedByConsumer,
        price: ITEM_PRICE,
        delivery_time: DELIVERY_TIME,
    });
    nft_program.meta_state().token(0).check(Token {
        owner_id: CONSUMER.into(),
        ..Default::default()
    });
}
