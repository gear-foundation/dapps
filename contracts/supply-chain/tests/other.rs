pub mod utils;
use utils::*;

#[test]
fn interact_with_unexistend_item() {
    const NONEXISTEND_ITEM: u128 = 999999;

    let system = init_system();
    let supply_chain_program = init_supply_chain_program(&system);
    fail::put_up_for_sale_by_producer(&supply_chain_program, PRODUCER[0], NONEXISTEND_ITEM);
    fail::purchare_by_distributor(&supply_chain_program, DISTRIBUTOR[0], NONEXISTEND_ITEM);
    fail::approve_by_producer(&supply_chain_program, PRODUCER[0], NONEXISTEND_ITEM);
    fail::ship_by_producer(&supply_chain_program, PRODUCER[0], NONEXISTEND_ITEM);
    fail::receive_by_distributor(&supply_chain_program, DISTRIBUTOR[0], NONEXISTEND_ITEM);
    fail::process_by_distributor(&supply_chain_program, DISTRIBUTOR[0], NONEXISTEND_ITEM);
    fail::package_by_distributor(&supply_chain_program, DISTRIBUTOR[0], NONEXISTEND_ITEM);
    fail::put_up_for_sale_by_distributor(&supply_chain_program, DISTRIBUTOR[0], NONEXISTEND_ITEM);
    fail::purchare_by_retailer(&supply_chain_program, RETAILER[0], NONEXISTEND_ITEM);
    fail::approve_by_distributor(&supply_chain_program, DISTRIBUTOR[0], NONEXISTEND_ITEM);
    fail::ship_by_distributor(&supply_chain_program, DISTRIBUTOR[0], NONEXISTEND_ITEM);
    fail::receive_by_retailer(&supply_chain_program, RETAILER[0], NONEXISTEND_ITEM);
    fail::put_up_for_sale_by_retailer(&supply_chain_program, RETAILER[0], NONEXISTEND_ITEM);
    fail::purchare_by_consumer(&supply_chain_program, CONSUMER[0], NONEXISTEND_ITEM);
    // TODO: replace with the state check function when it becomes available.
    // fail::get_item_info(&supply_chain_program, NONEXISTEND_ITEM);
}
