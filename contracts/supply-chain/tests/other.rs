pub mod utils;
use utils::*;

#[test]
fn interact_with_unexistend_item() {
    let system = init_system();
    let supply_chain_program = Program::current(&system);
    check::init_supply_chain_program(&supply_chain_program);

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
}

#[test]
#[should_panic]
fn interact_with_unexistend_item_meta_state() {
    let system = init_system();
    let supply_chain_program = Program::current(&system);
    check::init_supply_chain_program(&supply_chain_program);

    fail::get_item_info(&supply_chain_program, NONEXISTEND_ITEM);
}

#[test]
fn init_with_zero_address() {
    let system = init_system();
    let supply_chain_program = Program::current(&system);
    check::init_supply_chain_program(&supply_chain_program);

    fail::init_supply_chain_program(
        &supply_chain_program,
        InitSupplyChain {
            ft_program_id: FT_PROGRAM_ID.into(),
            nft_program_id: NFT_PROGRAM_ID.into(),

            producers: BTreeSet::from([PRODUCER[0].into(), PRODUCER[1].into()]),
            distributors: BTreeSet::from([DISTRIBUTOR[0].into(), DISTRIBUTOR[1].into()]),
            retailers: BTreeSet::from([RETAILER[0].into(), RETAILER[1].into()]),
        },
    );
}
