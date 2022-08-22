pub mod utils;
use gstd::ActorId;
use utils::{prelude::*, FungibleToken, NonFungibleToken};

#[test]
fn interact_with_unexistent_item() {
    const NONEXISTENT_ITEM: u128 = 99999999;

    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    // Should fail because an item must exist in a supply chain.
    schain_program
        .put_up_for_sale_by_producer(PRODUCER, NONEXISTENT_ITEM, ITEM_PRICE)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .purchase_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM, DELIVERY_TIME)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .approve_by_producer(PRODUCER, NONEXISTENT_ITEM, true)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .ship_by_producer(PRODUCER, NONEXISTENT_ITEM)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .receive_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .process_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .package_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .put_up_for_sale_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM, ITEM_PRICE)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .purchase_by_retailer(RETAILER, NONEXISTENT_ITEM, DELIVERY_TIME)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .approve_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM, true)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .ship_by_distributor(DISTRIBUTOR, NONEXISTENT_ITEM)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .receive_by_retailer(RETAILER, NONEXISTENT_ITEM)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .put_up_for_sale_by_retailer(RETAILER, NONEXISTENT_ITEM, ITEM_PRICE)
        .failed();
    // Should fail because an item must exist in a supply chain.
    schain_program
        .purchase_by_consumer(CONSUMER, NONEXISTENT_ITEM)
        .failed();

    // Should return the `Default` value because an item must exist in a supply
    // chain.
    schain_program
        .meta_state()
        .item_info(NONEXISTENT_ITEM)
        .check(Default::default());
    schain_program
        .meta_state()
        .existing_items()
        .check([].into());
}

#[test]
fn initialization() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    let nft_program = NonFungibleToken::initialize(&system);

    let mut supply_chain_config = InitSupplyChain {
        producers: [ActorId::zero()].into(),
        distributors: [ActorId::zero()].into(),
        retailers: [ActorId::zero()].into(),

        ft_program: ft_program.actor_id(),
        nft_program: nft_program.actor_id(),
    };
    //Should fail because each [`ActorId`] of `producers`, `distributors`, and
    // `retailers` mustn't equal `ActorId::zero()`.
    SupplyChain::initialize_custom(&system, supply_chain_config.clone()).failed();

    supply_chain_config.producers = [PRODUCER.into()].into();
    //Should fail because each [`ActorId`] of `producers`, `distributors`, and
    // `retailers` mustn't equal `ActorId::zero()`.
    SupplyChain::initialize_custom(&system, supply_chain_config.clone()).failed();

    supply_chain_config.distributors = [DISTRIBUTOR.into()].into();
    //Should fail because each [`ActorId`] of `producers`, `distributors`, and
    // `retailers` mustn't equal `ActorId::zero()`.
    SupplyChain::initialize_custom(&system, supply_chain_config.clone()).failed();

    supply_chain_config.retailers = [RETAILER.into()].into();
    let supply_chain_program =
        SupplyChain::initialize_custom(&system, supply_chain_config.clone()).succeed();

    supply_chain_program
        .meta_state()
        .participants()
        .check(Participants {
            producers: supply_chain_config.producers,
            distributors: supply_chain_config.distributors,
            retailers: supply_chain_config.retailers,
        });
    supply_chain_program
        .meta_state()
        .ft_program()
        .check(ft_program.actor_id());
    supply_chain_program
        .meta_state()
        .nft_program()
        .check(nft_program.actor_id());
}

#[test]
fn query_existing_items() {
    let system = utils::initialize_system();

    let ft_program = FungibleToken::initialize(&system);
    let nft_program = NonFungibleToken::initialize(&system);
    let schain_program =
        SupplyChain::initialize(&system, ft_program.actor_id(), nft_program.actor_id());

    let mut items_info = BTreeMap::new();
    for item_id in 0..=5 {
        schain_program.produce(PRODUCER).check(item_id);
        items_info.insert(
            item_id.into(),
            ItemInfo {
                producer: PRODUCER.into(),
                ..Default::default()
            },
        );
    }

    schain_program
        .meta_state()
        .existing_items()
        .check(items_info);
}
