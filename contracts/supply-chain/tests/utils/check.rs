use super::*;

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
        .contains(&(producer, SupplyChainEvent::Success.encode())));
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
        .contains(&(distributor, SupplyChainEvent::Success.encode())));
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
        .contains(&(producer, SupplyChainEvent::Success.encode())));
}

pub fn ship_by_producer(supply_chain_program: &Program, producer: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(producer, SupplyChainAction::ShipByProducer(item_id.into()))
        .contains(&(producer, SupplyChainEvent::Success.encode(),)));
}

pub fn receive_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::ReceiveByDistributor(item_id.into()),
        )
        .contains(&(distributor, SupplyChainEvent::Success.encode())));
}

pub fn process_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::ProcessByDistributor(item_id.into()),
        )
        .contains(&(distributor, SupplyChainEvent::Success.encode())));
}

pub fn package_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::PackageByDistributor(item_id.into()),
        )
        .contains(&(distributor, SupplyChainEvent::Success.encode())));
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
        .contains(&(distributor, SupplyChainEvent::Success.encode())));
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
        .contains(&(retailer, SupplyChainEvent::Success.encode())));
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
        .contains(&(distributor, SupplyChainEvent::Success.encode())));
}

pub fn ship_by_distributor(supply_chain_program: &Program, distributor: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            distributor,
            SupplyChainAction::ShipByDistributor(item_id.into()),
        )
        .contains(&(distributor, SupplyChainEvent::Success.encode())));
}

pub fn receive_by_retailer(supply_chain_program: &Program, retailer: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            retailer,
            SupplyChainAction::ReceiveByRetailer(item_id.into()),
        )
        .contains(&(retailer, SupplyChainEvent::Success.encode())));
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
        .contains(&(retailer, SupplyChainEvent::Success.encode())));
}

pub fn purchare_by_consumer(supply_chain_program: &Program, consumer: u64, item_id: u128) {
    assert!(supply_chain_program
        .send(
            consumer,
            SupplyChainAction::PurchaseByConsumer(item_id.into()),
        )
        .contains(&(consumer, SupplyChainEvent::Success.encode())));
}
