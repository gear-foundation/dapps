pub mod utils;
use utils::*;

#[test]
fn state() {
    let system = init_system();

    let ft_program = init_ft_program(&system);
    init_nft_program(&system);
    let supply_chain_program = Program::current(&system);
    check::init_supply_chain_program(&supply_chain_program);

    mint(&ft_program, DISTRIBUTOR[0], ITEM_PRICE_BY_PRODUCER[0]);
    mint(&ft_program, RETAILER[0], ITEM_PRICE_BY_DISTRIBUTOR[0]);
    mint(&ft_program, CONSUMER[0], ITEM_PRICE_BY_RETAILER[0]);

    check::produce(
        &supply_chain_program,
        PRODUCER[0],
        ITEM_NAME[0],
        ITEM_DESCRIPTION[0],
        ITEM_ID[0],
    );

    check::put_up_for_sale_by_producer(
        &supply_chain_program,
        PRODUCER[0],
        ITEM_ID[0],
        ITEM_PRICE_BY_PRODUCER[0],
    );
    fail::put_up_for_sale_by_producer(&supply_chain_program, PRODUCER[0], ITEM_ID[0]);

    check::purchare_by_distributor(
        &supply_chain_program,
        DISTRIBUTOR[0],
        ITEM_ID[0],
        DELIVERY_TIME[0],
    );
    fail::purchare_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);

    check::approve_by_producer(&supply_chain_program, PRODUCER[0], ITEM_ID[0], true);
    fail::approve_by_producer(&supply_chain_program, PRODUCER[0], ITEM_ID[0]);

    check::ship_by_producer(&supply_chain_program, PRODUCER[0], ITEM_ID[0]);
    fail::ship_by_producer(&supply_chain_program, PRODUCER[0], ITEM_ID[0]);

    check::receive_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    fail::receive_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);

    check::process_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    fail::process_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);

    check::package_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    fail::package_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);

    check::put_up_for_sale_by_distributor(
        &supply_chain_program,
        DISTRIBUTOR[0],
        ITEM_ID[0],
        ITEM_PRICE_BY_DISTRIBUTOR[0],
    );
    fail::put_up_for_sale_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);

    check::purchare_by_retailer(
        &supply_chain_program,
        RETAILER[0],
        ITEM_ID[0],
        DELIVERY_TIME[0],
    );
    fail::purchare_by_retailer(&supply_chain_program, RETAILER[0], ITEM_ID[0]);

    check::approve_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0], true);
    fail::approve_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);

    check::ship_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);
    fail::ship_by_distributor(&supply_chain_program, DISTRIBUTOR[0], ITEM_ID[0]);

    check::receive_by_retailer(&supply_chain_program, RETAILER[0], ITEM_ID[0]);
    fail::receive_by_retailer(&supply_chain_program, RETAILER[0], ITEM_ID[0]);

    check::put_up_for_sale_by_retailer(
        &supply_chain_program,
        RETAILER[0],
        ITEM_ID[0],
        ITEM_PRICE_BY_RETAILER[0],
    );
    fail::put_up_for_sale_by_retailer(&supply_chain_program, RETAILER[0], ITEM_ID[0]);

    check::purchare_by_consumer(&supply_chain_program, CONSUMER[0], ITEM_ID[0]);
    fail::purchare_by_consumer(&supply_chain_program, FOREIGN_USER, ITEM_ID[0]);
}
