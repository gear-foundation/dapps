#![no_std]

use gear_lib::non_fungible_token::token::TokenMetadata;
use gstd::{async_main, exec, msg, prelude::*, ActorId};

mod utils;

mod io;
use hashbrown::{HashMap, HashSet};
pub use io::*;

#[derive(Default)]
struct Item {
    info: ItemInfo,
    shipping_time: u64,
}

fn get_mut_item(items: &mut HashMap<ItemId, Item>, id: ItemId) -> &mut Item {
    items
        .get_mut(&id)
        .unwrap_or_else(|| panic!("Item not found by an ID"))
}

async fn receive(ft_program: ActorId, msg_source: ActorId, seller: ActorId, item: &Item) {
    let program_id = exec::program_id();
    let elapsed_time = exec::block_timestamp() - item.shipping_time;
    // By default, all fungible tokens are transferred to a seller,
    let (mut to, mut amount) = (seller, item.info.price);

    // but if the seller spends more time than was agreed...
    if elapsed_time > item.info.delivery_time {
        // ...and is extremely late (more than or exactly 2 times in this example),
        if elapsed_time >= item.info.delivery_time * 2 {
            // then all fungible tokens are refunded to a buyer...
            to = msg_source;
        } else {
            // ...or another half is transferred to a seller...
            amount /= 2;

            // ...and a half of tokens is refunded to a buyer.
            utils::transfer_ftokens(ft_program, program_id, msg_source, item.info.price - amount)
                .await;
        }
    }

    utils::transfer_ftokens(ft_program, program_id, to, amount).await;
}

#[derive(Default)]
struct SupplyChain {
    items: HashMap<ItemId, Item>,

    producers: HashSet<ActorId>,
    distributors: HashSet<ActorId>,
    retailers: HashSet<ActorId>,

    ft_program: ActorId,
    nft_program: ActorId,
}

impl SupplyChain {
    fn check_producer(&self, actor_id: ActorId) {
        if !self.producers.contains(&actor_id) {
            panic!("Actor must be a producer");
        }
    }

    fn check_distributor(&self, actor_id: ActorId) {
        if !self.distributors.contains(&actor_id) {
            panic!("Actor must be a distributor");
        }
    }

    fn check_retailer(&self, actor_id: ActorId) {
        if !self.retailers.contains(&actor_id) {
            panic!("Actor must be a retailer");
        }
    }

    async fn produce_item(&mut self, token_metadata: TokenMetadata) {
        let msg_source = msg::source();
        self.check_producer(msg_source);

        let item_id = utils::mint_nft(self.nft_program, token_metadata).await;
        utils::transfer_nft(self.nft_program, msg_source, item_id).await;

        self.items.insert(
            item_id,
            Item {
                info: ItemInfo {
                    producer: msg_source,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        utils::reply(SupplyChainEvent::Produced(item_id));
    }

    async fn put_up_for_sale_by_producer(&mut self, item_id: ItemId, price: u128) {
        let msg_source = msg::source();
        self.check_producer(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::Produced);
        assert_eq!(item.info.producer, msg_source);

        item.info.price = price;
        utils::transfer_nft(self.nft_program, exec::program_id(), item_id).await;

        item.info.state = ItemState::ForSaleByProducer;
        utils::reply(SupplyChainEvent::ForSaleByProducer(item_id));
    }

    async fn purchase_by_distributor(&mut self, item_id: ItemId, delivery_time: u64) {
        let msg_source = msg::source();
        self.check_distributor(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ForSaleByProducer);

        utils::transfer_ftokens(
            self.ft_program,
            msg_source,
            exec::program_id(),
            item.info.price,
        )
        .await;
        item.info.delivery_time = delivery_time;
        item.info.distributor = msg_source;

        item.info.state = ItemState::PurchasedByDistributor;
        utils::reply(SupplyChainEvent::PurchasedByDistributor(item_id));
    }

    async fn approve_by_producer(&mut self, item_id: ItemId, approve: bool) {
        let msg_source = msg::source();
        self.check_producer(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::PurchasedByDistributor);
        assert_eq!(item.info.producer, msg_source);

        item.info.state = if approve {
            ItemState::ApprovedByProducer
        } else {
            utils::transfer_ftokens(
                self.ft_program,
                exec::program_id(),
                item.info.distributor,
                item.info.price,
            )
            .await;
            ItemState::ForSaleByProducer
        };

        utils::reply(SupplyChainEvent::ApprovedByProducer(item_id));
    }

    fn ship_by_producer(&mut self, item_id: ItemId) {
        let msg_source = msg::source();
        self.check_producer(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ApprovedByProducer);
        assert_eq!(item.info.producer, msg_source);

        item.shipping_time = exec::block_timestamp();

        item.info.state = ItemState::ShippedByProducer;
        utils::reply(SupplyChainEvent::ShippedByProducer(item_id));
    }

    async fn receive_by_distributor(&mut self, item_id: ItemId) {
        let msg_source = msg::source();
        self.check_distributor(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ShippedByProducer);
        assert_eq!(item.info.distributor, msg_source);

        receive(self.ft_program, msg_source, item.info.producer, item).await;
        utils::transfer_nft(self.nft_program, msg_source, item_id).await;

        item.info.state = ItemState::ReceivedByDistributor;
        utils::reply(SupplyChainEvent::ReceivedByDistributor(item_id));
    }

    fn process_by_distributor(&mut self, item_id: ItemId) {
        let msg_source = msg::source();
        self.check_distributor(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ReceivedByDistributor);
        assert_eq!(item.info.distributor, msg_source);

        item.info.state = ItemState::ProcessedByDistributor;
        utils::reply(SupplyChainEvent::ProcessedByDistributor(item_id));
    }

    fn package_by_distributor(&mut self, item_id: ItemId) {
        let msg_source = msg::source();
        self.check_distributor(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ProcessedByDistributor);
        assert_eq!(item.info.distributor, msg_source);

        item.info.state = ItemState::PackagedByDistributor;
        utils::reply(SupplyChainEvent::PackagedByDistributor(item_id));
    }

    async fn put_up_for_sale_by_distributor(&mut self, item_id: ItemId, price: u128) {
        let msg_source = msg::source();
        self.check_distributor(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::PackagedByDistributor);
        assert_eq!(item.info.distributor, msg_source);

        item.info.price = price;
        utils::transfer_nft(self.nft_program, exec::program_id(), item_id).await;

        item.info.state = ItemState::ForSaleByDistributor;
        utils::reply(SupplyChainEvent::ForSaleByDistributor(item_id));
    }

    async fn purchase_by_retailer(&mut self, item_id: ItemId, delivery_time: u64) {
        let msg_source = msg::source();
        self.check_retailer(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ForSaleByDistributor);

        utils::transfer_ftokens(
            self.ft_program,
            msg_source,
            exec::program_id(),
            item.info.price,
        )
        .await;
        item.info.delivery_time = delivery_time;
        item.info.retailer = msg_source;

        item.info.state = ItemState::PurchasedByRetailer;
        utils::reply(SupplyChainEvent::PurchasedByRetailer(item_id));
    }

    async fn approve_by_distributor(&mut self, item_id: ItemId, approve: bool) {
        let msg_source = msg::source();
        self.check_distributor(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::PurchasedByRetailer);
        assert_eq!(item.info.distributor, msg_source);

        item.info.state = if approve {
            ItemState::ApprovedByDistributor
        } else {
            utils::transfer_ftokens(
                self.ft_program,
                exec::program_id(),
                item.info.retailer,
                item.info.price,
            )
            .await;
            ItemState::ForSaleByDistributor
        };

        utils::reply(SupplyChainEvent::ApprovedByDistributor(item_id));
    }

    fn ship_by_distributor(&mut self, item_id: ItemId) {
        let msg_source = msg::source();
        self.check_distributor(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ApprovedByDistributor);
        assert_eq!(item.info.distributor, msg_source);

        item.shipping_time = exec::block_timestamp();

        item.info.state = ItemState::ShippedByDistributor;
        utils::reply(SupplyChainEvent::ShippedByDistributor(item_id));
    }

    async fn receive_by_retailer(&mut self, item_id: ItemId) {
        let msg_source = msg::source();
        self.check_retailer(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ShippedByDistributor);
        assert_eq!(item.info.retailer, msg_source);

        receive(self.ft_program, msg_source, item.info.distributor, item).await;
        utils::transfer_nft(self.nft_program, msg_source, item_id).await;

        item.info.state = ItemState::ReceivedByRetailer;
        utils::reply(SupplyChainEvent::ReceivedByRetailer(item_id));
    }

    async fn put_up_for_sale_by_retailer(&mut self, item_id: ItemId, price: u128) {
        let msg_source = msg::source();
        self.check_retailer(msg_source);
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ReceivedByRetailer);
        assert_eq!(item.info.retailer, msg_source);

        item.info.price = price;
        utils::transfer_nft(self.nft_program, exec::program_id(), item_id).await;

        item.info.state = ItemState::ForSaleByRetailer;
        utils::reply(SupplyChainEvent::ForSaleByRetailer(item_id));
    }

    async fn purchase_by_consumer(&mut self, item_id: ItemId) {
        let msg_source = msg::source();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ForSaleByRetailer);

        utils::transfer_ftokens(
            self.ft_program,
            msg_source,
            item.info.retailer,
            item.info.price,
        )
        .await;
        utils::transfer_nft(self.nft_program, msg_source, item_id).await;

        item.info.state = ItemState::PurchasedByConsumer;
        utils::reply(SupplyChainEvent::PurchasedByConsumer(item_id));
    }
}

static mut PROGRAM: Option<SupplyChain> = None;

#[no_mangle]
extern "C" fn init() {
    let InitSupplyChain {
        producers,
        distributors,
        retailers,
        ft_program,
        nft_program,
    } = msg::load().expect("Unable to decode `InitSupplyChain`");

    if [&producers, &distributors, &retailers]
        .iter()
        .any(|actor_ids| actor_ids.contains(&ActorId::zero()))
    {
        panic!("Each `ActorId` of `producers`, `distributors`, and `retailers` mustn't equal `ActorId::zero()`");
    }
    let producers = HashSet::from_iter(producers.into_iter());
    let distributors = HashSet::from_iter(distributors.into_iter());
    let retailers = HashSet::from_iter(retailers.into_iter());
    let supply_chain = SupplyChain {
        producers,
        distributors,
        retailers,
        ft_program,
        nft_program,
        ..Default::default()
    };
    unsafe {
        PROGRAM = Some(supply_chain);
    }
}

#[async_main]
async fn main() {
    let action = msg::load().expect("Unable to decode `SupplyChainAction`");
    let program = unsafe { PROGRAM.get_or_insert(Default::default()) };
    match action {
        SupplyChainAction::Produce { token_metadata } => program.produce_item(token_metadata).await,
        SupplyChainAction::PutUpForSaleByProducer { item_id, price } => {
            program.put_up_for_sale_by_producer(item_id, price).await
        }
        SupplyChainAction::PurchaseByDistributor {
            item_id,
            delivery_time,
        } => {
            program
                .purchase_by_distributor(item_id, delivery_time)
                .await
        }
        SupplyChainAction::ApproveByProducer { item_id, approve } => {
            program.approve_by_producer(item_id, approve).await
        }
        SupplyChainAction::ShipByProducer(item_id) => program.ship_by_producer(item_id),
        SupplyChainAction::ReceiveByDistributor(item_id) => {
            program.receive_by_distributor(item_id).await
        }
        SupplyChainAction::ProcessByDistributor(item_id) => program.process_by_distributor(item_id),
        SupplyChainAction::PackageByDistributor(item_id) => program.package_by_distributor(item_id),
        SupplyChainAction::PutUpForSaleByDistributor { item_id, price } => {
            program.put_up_for_sale_by_distributor(item_id, price).await
        }
        SupplyChainAction::PurchaseByRetailer {
            item_id,
            delivery_time,
        } => program.purchase_by_retailer(item_id, delivery_time).await,
        SupplyChainAction::ApproveByDistributor { item_id, approve } => {
            program.approve_by_distributor(item_id, approve).await
        }
        SupplyChainAction::ShipByDistributor(item_id) => program.ship_by_distributor(item_id),
        SupplyChainAction::ReceiveByRetailer(item_id) => program.receive_by_retailer(item_id).await,
        SupplyChainAction::PutUpForSaleByRetailer { item_id, price } => {
            program.put_up_for_sale_by_retailer(item_id, price).await
        }
        SupplyChainAction::PurchaseByConsumer(item_id) => {
            program.purchase_by_consumer(item_id).await
        }
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let query = msg::load().expect("Unable to decode `SupplyChainStateQuery`");
    let program = unsafe { PROGRAM.get_or_insert(Default::default()) };
    let encoded = match query {
        SupplyChainStateQuery::ItemInfo(item_id) => {
            SupplyChainStateReply::ItemInfo(if let Some(item) = program.items.get(&item_id) {
                item.info
            } else {
                Default::default()
            })
        }
        SupplyChainStateQuery::Participants => SupplyChainStateReply::Participants(Participants {
            producers: BTreeSet::from_iter(program.producers.clone().into_iter()),
            distributors: BTreeSet::from_iter(program.distributors.clone().into_iter()),
            retailers: BTreeSet::from_iter(program.retailers.clone().into_iter()),
        }),
        SupplyChainStateQuery::FTProgram => SupplyChainStateReply::FTProgram(program.ft_program),
        SupplyChainStateQuery::NFTProgram => SupplyChainStateReply::NFTProgram(program.nft_program),
        SupplyChainStateQuery::ExistingItems => SupplyChainStateReply::ExistingItems(
            program
                .items
                .iter()
                .map(|item| (*item.0, item.1.info))
                .collect(),
        ),
        SupplyChainStateQuery::Roles(actor_id) => {
            let mut roles = BTreeSet::new();

            if program.producers.contains(&actor_id) {
                roles.insert(Role::Producer);
            }
            if program.distributors.contains(&actor_id) {
                roles.insert(Role::Distributor);
            }
            if program.retailers.contains(&actor_id) {
                roles.insert(Role::Retailer);
            }

            SupplyChainStateReply::Roles(roles)
        }
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}

gstd::metadata! {
    title: "Supply chain",
    init:
        input: InitSupplyChain,
    handle:
        input: SupplyChainAction,
        output: SupplyChainEvent,
    state:
        input: SupplyChainStateQuery,
        output: SupplyChainStateReply,
}
