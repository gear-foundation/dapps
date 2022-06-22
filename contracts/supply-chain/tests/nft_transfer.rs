pub mod utils;
use utils::*;

#[test]
fn nft_transfer() {
    let system = init_system();

    let ft_program = init_ft_program(&system);
    let nft_program = init_nft_program(&system);
    let supply_chain_program = Program::current(&system);
    check::init_supply_chain_program(&supply_chain_program);

    mint(&ft_program, DISTRIBUTOR[0], ITEM_PRICE_BY_PRODUCER[0]);
    mint(&ft_program, DISTRIBUTOR[1], ITEM_PRICE_BY_PRODUCER[1]);
    mint(&ft_program, RETAILER[0], ITEM_PRICE_BY_DISTRIBUTOR[0]);
    mint(&ft_program, RETAILER[1], ITEM_PRICE_BY_DISTRIBUTOR[1]);
    mint(&ft_program, CONSUMER[0], ITEM_PRICE_BY_RETAILER[0]);
    mint(&ft_program, CONSUMER[1], ITEM_PRICE_BY_RETAILER[1]);

    check::produce(
        &supply_chain_program,
        PRODUCER[0],
        ITEM_NAME[0],
        ITEM_DESCRIPTION[0],
        ITEM_ID[0],
    );
    check::produce(
        &supply_chain_program,
        PRODUCER[1],
        ITEM_NAME[1],
        ITEM_DESCRIPTION[1],
        ITEM_ID[1],
    );
    check_nft_owner(&nft_program, ITEM_ID[0], PRODUCER[0]);
    check_nft_owner(&nft_program, ITEM_ID[1], PRODUCER[1]);

    check::put_up_for_sale_by_producer(
        &supply_chain_program,
        PRODUCER[0],
        ITEM_ID[0],
        ITEM_PRICE_BY_PRODUCER[0],
    );
    check::put_up_for_sale_by_producer(
        &supply_chain_program,
        PRODUCER[1],
        ITEM_ID[1],
        ITEM_PRICE_BY_PRODUCER[1],
    );
    check_nft_owner(&nft_program, ITEM_ID[0], SUPPLY_CHAIN_PROGRAM_ID);
    check_nft_owner(&nft_program, ITEM_ID[1], SUPPLY_CHAIN_PROGRAM_ID);

    check::purchare_by_distributor(
        &supply_chain_program,
        DISTRIBUTOR[0],
        ITEM_ID[0],
        DELIVERY_TIME[0],
    );
    check::purchare_by_distributor(
        &supply_chain_program,
        DISTRIBUTOR[1],
        ITEM_ID[1],
        DELIVERY_TIME[1],
    );

    check::approve_by_producer(&supply_chain_program, PRODUCER[0], ITEM_ID[0], true);
    check::approve_by_producer(&supply_chain_program, PRODUCER[1], ITEM_ID[1], true);

    check::ship_by_producer(&supply_chain_program, PRODUCER[0], ITEM_ID[0]);
    check::ship_by_producer(&supply_chain_program, PRODUCER[1], ITEM_ID[1]);

    check::receive_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    check::receive_by_distributor(&supply_chain_program, DISTRIBUTOR[1], ITEM_ID[1]);
    check_nft_owner(&nft_program, ITEM_ID[0], DISTRIBUTOR[0]);
    check_nft_owner(&nft_program, ITEM_ID[1], DISTRIBUTOR[1]);

    check::process_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    check::process_by_distributor(&supply_chain_program, DISTRIBUTOR[1], ITEM_ID[1]);

    check::package_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    check::package_by_distributor(&supply_chain_program, DISTRIBUTOR[1], ITEM_ID[1]);

    check::put_up_for_sale_by_distributor(
        &supply_chain_program,
        DISTRIBUTOR[0],
        ITEM_ID[0],
        ITEM_PRICE_BY_DISTRIBUTOR[0],
    );
    check::put_up_for_sale_by_distributor(
        &supply_chain_program,
        DISTRIBUTOR[1],
        ITEM_ID[1],
        ITEM_PRICE_BY_DISTRIBUTOR[1],
    );
    check_nft_owner(&nft_program, ITEM_ID[0], SUPPLY_CHAIN_PROGRAM_ID);
    check_nft_owner(&nft_program, ITEM_ID[1], SUPPLY_CHAIN_PROGRAM_ID);

    check::purchare_by_retailer(
        &supply_chain_program,
        RETAILER[0],
        ITEM_ID[0],
        DELIVERY_TIME[0],
    );
    check::purchare_by_retailer(
        &supply_chain_program,
        RETAILER[1],
        ITEM_ID[1],
        DELIVERY_TIME[1],
    );

    check::approve_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0], true);
    check::approve_by_distributor(&supply_chain_program, DISTRIBUTOR[1], ITEM_ID[1], true);

    check::ship_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    check::ship_by_distributor(&supply_chain_program, DISTRIBUTOR[1], ITEM_ID[1]);

    check::receive_by_retailer(&supply_chain_program, RETAILER[0], ITEM_ID[0]);
    check::receive_by_retailer(&supply_chain_program, RETAILER[1], ITEM_ID[1]);
    check_nft_owner(&nft_program, ITEM_ID[0], RETAILER[0]);
    check_nft_owner(&nft_program, ITEM_ID[1], RETAILER[1]);

    check::put_up_for_sale_by_retailer(
        &supply_chain_program,
        RETAILER[0],
        ITEM_ID[0],
        ITEM_PRICE_BY_RETAILER[0],
    );
    check::put_up_for_sale_by_retailer(
        &supply_chain_program,
        RETAILER[1],
        ITEM_ID[1],
        ITEM_PRICE_BY_RETAILER[1],
    );
    check_nft_owner(&nft_program, ITEM_ID[0], SUPPLY_CHAIN_PROGRAM_ID);
    check_nft_owner(&nft_program, ITEM_ID[1], SUPPLY_CHAIN_PROGRAM_ID);

    check::purchare_by_consumer(&supply_chain_program, CONSUMER[0], ITEM_ID[0]);
    check::purchare_by_consumer(&supply_chain_program, CONSUMER[1], ITEM_ID[1]);
    check_nft_owner(&nft_program, ITEM_ID[0], CONSUMER[0]);
    check_nft_owner(&nft_program, ITEM_ID[1], CONSUMER[1]);

    check::get_item_info(
        &supply_chain_program,
        ITEM_ID[0],
        ItemInfo {
            producer: PRODUCER[0].into(),
            distributor: DISTRIBUTOR[0].into(),
            retailer: RETAILER[0].into(),

            state: ItemState::PurchasedByConsumer,
            price: ITEM_PRICE_BY_RETAILER[0],
            delivery_time: DELIVERY_TIME[0],
        },
    );
    check_nft_name_n_description(&nft_program, ITEM_ID[0], ITEM_NAME[0], ITEM_DESCRIPTION[0]);

    check::get_item_info(
        &supply_chain_program,
        ITEM_ID[1],
        ItemInfo {
            producer: PRODUCER[1].into(),
            distributor: DISTRIBUTOR[1].into(),
            retailer: RETAILER[1].into(),

            state: ItemState::PurchasedByConsumer,
            price: ITEM_PRICE_BY_RETAILER[1],
            delivery_time: DELIVERY_TIME[1],
        },
    );
    check_nft_name_n_description(&nft_program, ITEM_ID[1], ITEM_NAME[1], ITEM_DESCRIPTION[1]);
}
