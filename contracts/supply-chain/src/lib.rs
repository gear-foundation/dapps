#![no_std]

use ft_io::{FTAction, FTEvent};
use gear_lib::non_fungible_token::{io::NFTTransfer, token::TokenMetadata};
use gstd::{async_main, exec, msg, prelude::*, ActorId};
use nft_io::NFTAction;
use supply_chain_io::*;

#[derive(Default)]
struct Item {
    info: ItemInfo,
    shipping_time: u64,
}

fn panic_item_not_exist(item_id: ItemId) -> ! {
    panic!("Item with the {item_id} ID doesn't exist")
}

fn get_mut_item(items: &mut BTreeMap<ItemId, Item>, id: ItemId) -> &mut Item {
    items
        .get_mut(&id)
        .unwrap_or_else(|| panic_item_not_exist(id))
}

async fn transfer_tokens(ft_program_id: ActorId, from: ActorId, to: ActorId, amount: u128) {
    msg::send_and_wait_for_reply::<FTEvent, _>(
        ft_program_id,
        FTAction::Transfer { from, to, amount },
        0,
    )
    .expect("Error in async message to FT contract")
    .await
    .expect("Unable to decode FTEvent");
}

async fn transfer_nft(nft_program_id: ActorId, to: ActorId, token_id: ItemId) {
    msg::send_and_wait_for_reply::<NFTTransfer, _>(
        nft_program_id,
        NFTAction::Transfer { to, token_id },
        0,
    )
    .expect("Error in async message to NFT contract")
    .await
    .expect("Unable to decode NFTTransfer");
}

async fn receive(ft_program_id: ActorId, seller: ActorId, item: &Item) {
    let elapsed_time = exec::block_timestamp() - item.shipping_time;
    // By default, all tokens are transferred to a seller,
    let (mut to, mut amount) = (seller, item.info.price);

    // but if a seller spends more time than agreed...
    if elapsed_time > item.info.delivery_time {
        // ...and is extremely late (more than or exactly 2 times in this example),
        if elapsed_time >= item.info.delivery_time * 2 {
            // then all tokens are refunded to a buyer...
            to = msg::source();
        } else {
            // ...or another half is transferred to a seller
            amount /= 2;

            // ...and a half of tokens is refunded to a buyer.
            transfer_tokens(
                ft_program_id,
                exec::program_id(),
                msg::source(),
                item.info.price - amount,
            )
            .await;
        }
    }

    transfer_tokens(ft_program_id, exec::program_id(), to, amount).await;
}

fn reply(supply_chain_event: SupplyChainEvent) {
    msg::reply(supply_chain_event, 0).expect("Error in message reply");
}

#[derive(Default)]
struct SupplyChain {
    items: BTreeMap<ItemId, Item>,

    producers: BTreeSet<ActorId>,
    distributors: BTreeSet<ActorId>,
    retailers: BTreeSet<ActorId>,

    ft_program_id: ActorId,
    nft_program_id: ActorId,
}

impl SupplyChain {
    fn check_producer(&self) {
        if !self.producers.contains(&msg::source()) {
            panic!("msg::source() must be a producer");
        }
    }

    fn check_distributor(&self) {
        if !self.distributors.contains(&msg::source()) {
            panic!("msg::source() must be a distributor");
        }
    }

    fn check_retailer(&self) {
        if !self.retailers.contains(&msg::source()) {
            panic!("msg::source() must be a retailer");
        }
    }

    async fn produce_item(&mut self, name: String, description: String) {
        self.check_producer();

        let raw_reply: Vec<u8> = msg::send_and_wait_for_reply(
            self.nft_program_id,
            NFTAction::Mint {
                token_metadata: TokenMetadata {
                    name,
                    description,
                    ..Default::default()
                },
            },
            0,
        )
        .expect("Error in sending Mint message to NFT contract")
        .await
        .expect("Unable to decode Vec<u8>");

        let decoded_reply =
            NFTTransfer::decode(&mut &raw_reply[..]).expect("Unable to decode NFTTransfer");

        // After minting NFT for an item,
        // an item gets an ID equal to the ID of its NFT.
        let item_id = match decoded_reply {
            NFTTransfer { to, token_id, .. } if to == exec::program_id() => token_id,
            smth_else => panic!(
                "NFTTransfer must be NFTTransfer {{ to: exec::program_id(), .. }} not {smth_else:?}"
            ),
        };
        transfer_nft(self.nft_program_id, msg::source(), item_id).await;

        self.items.insert(
            item_id,
            Item {
                info: ItemInfo {
                    producer: msg::source(),
                    ..Default::default()
                },
                ..Default::default()
            },
        );
        reply(SupplyChainEvent::Produced(item_id));
    }

    async fn put_up_for_sale_by_producer(&mut self, item_id: ItemId, price: u128) {
        self.check_producer();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::Produced);
        assert_eq!(item.info.producer, msg::source());

        item.info.price = price;
        transfer_nft(self.nft_program_id, exec::program_id(), item_id).await;

        item.info.state = ItemState::ForSaleByProducer;
        reply(SupplyChainEvent::Success);
    }

    async fn purchase_by_distributor(&mut self, item_id: ItemId, delivery_time: u64) {
        self.check_distributor();
        let item = get_mut_item(&mut self.items, item_id);

        transfer_tokens(
            self.ft_program_id,
            msg::source(),
            exec::program_id(),
            item.info.price,
        )
        .await;
        item.info.delivery_time = delivery_time;
        item.info.distributor = msg::source();

        item.info.state = ItemState::PurchasedByDistributor;
        reply(SupplyChainEvent::Success);
    }

    async fn approve_by_producer(&mut self, item_id: ItemId, approve: bool) {
        self.check_producer();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::PurchasedByDistributor);
        assert_eq!(item.info.producer, msg::source());

        item.info.state = if approve {
            ItemState::ApprovedByProducer
        } else {
            transfer_tokens(
                self.ft_program_id,
                exec::program_id(),
                item.info.distributor,
                item.info.price,
            )
            .await;
            ItemState::ForSaleByProducer
        };

        reply(SupplyChainEvent::Success);
    }

    fn ship_by_producer(&mut self, item_id: ItemId) {
        self.check_producer();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ApprovedByProducer);
        assert_eq!(item.info.producer, msg::source());

        item.shipping_time = exec::block_timestamp();

        item.info.state = ItemState::ShippedByProducer;
        reply(SupplyChainEvent::Success);
    }

    async fn receive_by_distributor(&mut self, item_id: ItemId) {
        self.check_distributor();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ShippedByProducer);
        assert_eq!(item.info.distributor, msg::source());

        receive(self.ft_program_id, item.info.producer, item).await;
        transfer_nft(self.nft_program_id, msg::source(), item_id).await;

        item.info.state = ItemState::ReceivedByDistributor;
        reply(SupplyChainEvent::Success);
    }

    fn process_by_distributor(&mut self, item_id: ItemId) {
        self.check_distributor();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ReceivedByDistributor);
        assert_eq!(item.info.distributor, msg::source());

        item.info.state = ItemState::ProcessedByDistributor;
        reply(SupplyChainEvent::Success);
    }

    fn package_by_distributor(&mut self, item_id: ItemId) {
        self.check_distributor();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ProcessedByDistributor);
        assert_eq!(item.info.distributor, msg::source());

        item.info.state = ItemState::PackagedByDistributor;
        reply(SupplyChainEvent::Success);
    }

    async fn put_up_for_sale_by_distributor(&mut self, item_id: ItemId, price: u128) {
        self.check_distributor();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::PackagedByDistributor);
        assert_eq!(item.info.distributor, msg::source());

        item.info.price = price;
        transfer_nft(self.nft_program_id, exec::program_id(), item_id).await;

        item.info.state = ItemState::ForSaleByDistributor;
        reply(SupplyChainEvent::Success);
    }

    async fn purchase_by_retailer(&mut self, item_id: ItemId, delivery_time: u64) {
        self.check_retailer();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ForSaleByDistributor);

        transfer_tokens(
            self.ft_program_id,
            msg::source(),
            exec::program_id(),
            item.info.price,
        )
        .await;
        item.info.delivery_time = delivery_time;
        item.info.retailer = msg::source();

        item.info.state = ItemState::PurchasedByRetailer;
        reply(SupplyChainEvent::Success);
    }

    async fn approve_by_distributor(&mut self, item_id: ItemId, approve: bool) {
        self.check_distributor();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::PurchasedByRetailer);
        assert_eq!(item.info.distributor, msg::source());

        item.info.state = if approve {
            ItemState::ApprovedByDistributor
        } else {
            transfer_tokens(
                self.ft_program_id,
                exec::program_id(),
                item.info.retailer,
                item.info.price,
            )
            .await;
            ItemState::ForSaleByDistributor
        };

        reply(SupplyChainEvent::Success);
    }

    fn ship_by_distributor(&mut self, item_id: ItemId) {
        self.check_distributor();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ApprovedByDistributor);
        assert_eq!(item.info.distributor, msg::source());

        item.shipping_time = exec::block_timestamp();

        item.info.state = ItemState::ShippedByDistributor;
        reply(SupplyChainEvent::Success);
    }

    async fn receive_by_retailer(&mut self, item_id: ItemId) {
        self.check_retailer();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ShippedByDistributor);
        assert_eq!(item.info.retailer, msg::source());

        receive(self.ft_program_id, item.info.distributor, item).await;
        transfer_nft(self.nft_program_id, msg::source(), item_id).await;

        item.info.state = ItemState::ReceivedByRetailer;
        reply(SupplyChainEvent::Success);
    }

    async fn put_up_for_sale_by_retailer(&mut self, item_id: ItemId, price: u128) {
        self.check_retailer();
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ReceivedByRetailer);
        assert_eq!(item.info.retailer, msg::source());

        item.info.price = price;
        transfer_nft(self.nft_program_id, exec::program_id(), item_id).await;

        item.info.state = ItemState::ForSaleByRetailer;
        reply(SupplyChainEvent::Success);
    }

    async fn purchase_by_consumer(&mut self, item_id: ItemId) {
        let item = get_mut_item(&mut self.items, item_id);
        assert_eq!(item.info.state, ItemState::ForSaleByRetailer);

        transfer_tokens(
            self.ft_program_id,
            msg::source(),
            item.info.retailer,
            item.info.price,
        )
        .await;
        transfer_nft(self.nft_program_id, msg::source(), item_id).await;

        item.info.state = ItemState::PurchasedByConsumer;
        reply(SupplyChainEvent::Success);
    }

    fn get_item_info(&self, item_id: ItemId) -> ItemInfo {
        self.items
            .get(&item_id)
            .unwrap_or_else(|| panic_item_not_exist(item_id))
            .info
            .clone()
    }
}

static mut SUPPLY_CHAIN: Option<SupplyChain> = None;

#[no_mangle]
pub extern "C" fn init() {
    let InitSupplyChain {
        producers,
        distributors,
        retailers,
        ft_program_id,
        nft_program_id,
    } = msg::load().expect("Unable to decode InitSupplyChain");
    let supply_chain = SupplyChain {
        producers,
        distributors,
        retailers,
        ft_program_id,
        nft_program_id,
        ..Default::default()
    };
    unsafe {
        SUPPLY_CHAIN = Some(supply_chain);
    }
}

#[async_main]
pub async fn main() {
    let action = msg::load().expect("Unable to decode SupplyChainAction");
    let supply_chain = unsafe { SUPPLY_CHAIN.get_or_insert(Default::default()) };
    match action {
        SupplyChainAction::Produce { name, description } => {
            supply_chain.produce_item(name, description).await;
        }
        SupplyChainAction::PutUpForSaleByProducer { item_id, price } => {
            supply_chain
                .put_up_for_sale_by_producer(item_id, price)
                .await;
        }
        SupplyChainAction::PurchaseByDistributor {
            item_id,
            delivery_time,
        } => {
            supply_chain
                .purchase_by_distributor(item_id, delivery_time)
                .await;
        }
        SupplyChainAction::ApproveByProducer { item_id, approve } => {
            supply_chain.approve_by_producer(item_id, approve).await;
        }
        SupplyChainAction::ShipByProducer(item_id) => supply_chain.ship_by_producer(item_id),
        SupplyChainAction::ReceiveByDistributor(item_id) => {
            supply_chain.receive_by_distributor(item_id).await;
        }
        SupplyChainAction::ProcessByDistributor(item_id) => {
            supply_chain.process_by_distributor(item_id);
        }
        SupplyChainAction::PackageByDistributor(item_id) => {
            supply_chain.package_by_distributor(item_id);
        }
        SupplyChainAction::PutUpForSaleByDistributor { item_id, price } => {
            supply_chain
                .put_up_for_sale_by_distributor(item_id, price)
                .await;
        }
        SupplyChainAction::PurchaseByRetailer {
            item_id,
            delivery_time,
        } => {
            supply_chain
                .purchase_by_retailer(item_id, delivery_time)
                .await;
        }
        SupplyChainAction::ApproveByDistributor { item_id, approve } => {
            supply_chain.approve_by_distributor(item_id, approve).await;
        }
        SupplyChainAction::ShipByDistributor(item_id) => {
            supply_chain.ship_by_distributor(item_id);
        }
        SupplyChainAction::ReceiveByRetailer(item_id) => {
            supply_chain.receive_by_retailer(item_id).await;
        }
        SupplyChainAction::PutUpForSaleByRetailer { item_id, price } => {
            supply_chain
                .put_up_for_sale_by_retailer(item_id, price)
                .await;
        }
        SupplyChainAction::PurchaseByConsumer(item_id) => {
            supply_chain.purchase_by_consumer(item_id).await;
        }
    }
}

#[no_mangle]
pub extern "C" fn meta_state() -> *mut [i32; 2] {
    let state: SupplyChainState = msg::load().expect("Unable to decode SupplyChainState");
    let supply_chain = unsafe { SUPPLY_CHAIN.get_or_insert(Default::default()) };
    let encoded = match state {
        SupplyChainState::GetItemInfo(item_id) => {
            SupplyChainStateReply::ItemInfo(supply_chain.get_item_info(item_id)).encode()
        }
    };
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
        input: SupplyChainState,
        output: SupplyChainStateReply,
}
