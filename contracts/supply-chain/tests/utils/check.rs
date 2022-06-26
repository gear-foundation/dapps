use super::*;

pub fn init_supply_chain_program(supply_chain_program: &Program) {
    assert!(supply_chain_program
        .send(
            FOREIGN_USER,
            InitSupplyChain {
                ft_program_id: FT_PROGRAM_ID.into(),
                nft_program_id: NFT_PROGRAM_ID.into(),

                producers: BTreeSet::from([PRODUCER[0].into(), PRODUCER[1].into()]),
                distributors: BTreeSet::from([DISTRIBUTOR[0].into(), DISTRIBUTOR[1].into()]),
                retailers: BTreeSet::from([RETAILER[0].into(), RETAILER[1].into()]),
            },
        )
        .log()
        .is_empty());
}

pub fn produce(
    supply_chain_program: &Program,
    producer: u64,
    name: &str,
    description: &str,

    item_id: u128,
) {
    assert!(supply_chain_program
        .send(
            producer,
            SupplyChainAction::Produce {
                name: name.into(),
                description: description.into(),
            },
        )
        .contains(&(
            producer,
            SupplyChainEvent::Produced(item_id.into()).encode(),
        )));
}

pub fn put_up_for_sale_by_producer(
    supply_chain_program: &Program,
    producer: u64,
    item_id: u128,
    price: u128,
) {
    assert!(supply_chain_program
        .send(
            producer,
            SupplyChainAction::PutUpForSaleByProducer {
                item_id: item_id.into(),
                price,
            },
        )
        .contains(&(
            producer,
            SupplyChainEvent::ForSaleByProducer(item_id.into()).encode(),
        )));
}

pub fn purchare_by_distributor(
    supply_chain_program: &Program,
    distributor: u64,
    item_id: u128,
    delivery_time: u64,
) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::PurchaseByDistributor {
                item_id: item_id.into(),
                delivery_time,
            },
        )
        .contains(&(
            distributor,
            SupplyChainEvent::PurchasedByDistributor(item_id.into()).encode(),
        )));
}

pub fn approve_by_producer(
    supply_chain_program: &Program,
    producer: u64,
    item_id: u128,
    approve: bool,
) {
    assert!(supply_chain_program
        .send(
            producer,
            SupplyChainAction::ApproveByProducer {
                item_id: item_id.into(),
                approve,
            },
        )
        .contains(&(
            producer,
            SupplyChainEvent::ApprovedByProducer(item_id.into()).encode(),
        )));
}

pub fn ship_by_producer(supply_chain_program: &Program, producer: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(producer, SupplyChainAction::ShipByProducer(item_id.into()))
        .contains(&(
            producer,
            SupplyChainEvent::ShippedByProducer(item_id.into()).encode(),
        )));
}

pub fn receive_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::ReceiveByDistributor(item_id.into()),
        )
        .contains(&(
            distributor,
            SupplyChainEvent::ReceivedByDistributor(item_id.into()).encode(),
        )));
}

pub fn process_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::ProcessByDistributor(item_id.into()),
        )
        .contains(&(
            distributor,
            SupplyChainEvent::ProcessedByDistributor(item_id.into()).encode(),
        )));
}

pub fn package_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::PackageByDistributor(item_id.into()),
        )
        .contains(&(
            distributor,
            SupplyChainEvent::PackagedByDistributor(item_id.into()).encode(),
        )));
}

pub fn put_up_for_sale_by_distributor(
    supply_chain_program: &Program,
    distributor: u64,
    item_id: u128,
    price: u128,
) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::PutUpForSaleByDistributor {
                item_id: item_id.into(),
                price,
            },
        )
        .contains(&(
            distributor,
            SupplyChainEvent::ForSaleByDistributor(item_id.into()).encode(),
        )));
}

pub fn purchare_by_retailer(
    supply_chain_program: &Program,
    retailer: u64,
    item_id: u128,
    delivery_time: u64,
) {
    assert!(supply_chain_program
        .send(
            retailer,
            SupplyChainAction::PurchaseByRetailer {
                item_id: item_id.into(),
                delivery_time,
            },
        )
        .contains(&(
            retailer,
            SupplyChainEvent::PurchasedByRetailer(item_id.into()).encode(),
        )));
}

pub fn approve_by_distributor(
    supply_chain_program: &Program,
    distributor: u64,
    item_id: u128,
    approve: bool,
) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::ApproveByDistributor {
                item_id: item_id.into(),
                approve,
            },
        )
        .contains(&(
            distributor,
            SupplyChainEvent::ApprovedByDistributor(item_id.into()).encode(),
        )));
}

pub fn ship_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::ShipByDistributor(item_id.into()),
        )
        .contains(&(
            distributor,
            SupplyChainEvent::ShippedByDistributor(item_id.into()).encode(),
        )));
}

pub fn receive_by_retailer(supply_chain_program: &Program, retailer: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            retailer,
            SupplyChainAction::ReceiveByRetailer(item_id.into()),
        )
        .contains(&(
            retailer,
            SupplyChainEvent::ReceivedByRetailer(item_id.into()).encode(),
        )));
}

pub fn put_up_for_sale_by_retailer(
    supply_chain_program: &Program,
    retailer: u64,
    item_id: u128,
    price: u128,
) {
    assert!(supply_chain_program
        .send(
            retailer,
            SupplyChainAction::PutUpForSaleByRetailer {
                item_id: item_id.into(),
                price,
            },
        )
        .contains(&(
            retailer,
            SupplyChainEvent::ForSaleByRetailer(item_id.into()).encode(),
        )));
}

pub fn purchare_by_consumer(supply_chain_program: &Program, consumer: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            consumer,
            SupplyChainAction::PurchaseByConsumer(item_id.into()),
        )
        .contains(&(
            consumer,
            SupplyChainEvent::PurchasedByConsumer(item_id.into()).encode(),
        )));
}

pub fn get_item_info(supply_chain_program: &Program, item_id: u128, item_info: ItemInfo) {
    assert_eq!(
        supply_chain_program
            .meta_state::<_, SupplyChainStateReply>(SupplyChainState::ItemInfo(item_id.into())),
        SupplyChainStateReply::ItemInfo(item_info),
    );
}
