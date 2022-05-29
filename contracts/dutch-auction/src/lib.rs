#![no_std]

use codec::Encode;
use gstd::{exec::block_timestamp, msg, prelude::*, ActorId};
use primitive_types::U256;

pub mod state;
pub use state::{AuctionInfo, State, StateReply};

pub use auction_io::{Action, CreateConfig, Event, InitConfig};

#[derive(Debug, Default)]
pub struct NFT {
    pub token_id: U256,
    pub owner: ActorId,
    pub contract_id: ActorId,
}

#[derive(Debug, Default)]
pub struct Auction {
    pub owner: ActorId,
    pub nft: NFT,
    pub starting_price: u128,
    pub discount_rate: u128,
    pub is_active: bool,
    pub started_at: u64,
    pub expires_at: u64,
}

static mut AUCTION: Option<Auction> = None;

impl Auction {
    async fn buy(&mut self) {
        if !self.is_active {
            panic!("already bought or auction expired");
        }

        if block_timestamp() >= self.expires_at {
            panic!("auction expired");
        }

        let price = self.token_price();

        if msg::value() < price {
            panic!("value < price, {:?} < {:?}", msg::value(), price);
        }

        self.is_active = false;
        let refund = msg::value() - price;

        let _transfer_response: nft_example_io::Event = msg::send_and_wait_for_reply(
            self.nft.contract_id,
            nft_example_io::Action::Transfer {
                to: msg::source(),
                token_id: self.nft.token_id,
            },
            0,
        )
        .unwrap()
        .await
        .expect("Error in nft transfer");

        msg::send(msg::source(), "", refund).expect("Couldn't send refund");
        msg::send(self.nft.owner, "", price).expect("Couldn't send payment for nft owner");
    }

    fn token_price(&self) -> u128 {
        let time_elapsed = block_timestamp() - self.started_at;
        let discount = self.discount_rate * (time_elapsed as u128);

        self.starting_price - discount
    }

    fn renew_contract(&mut self, config: CreateConfig) {
        if self.is_active {
            panic!("already in use")
        }

        let hours_count = config.duration.days * 24 + config.duration.hours;
        let minutes_count = hours_count * 60 + config.duration.minutes;
        let duration = minutes_count * 60 * 1000;

        if config.starting_price < config.discount_rate * (duration as u128) {
            panic!("starting price < min");
        }

        self.is_active = true;
        self.started_at = block_timestamp();
        self.expires_at = block_timestamp() + duration;
        self.nft.token_id = config.token_id;
        self.nft.contract_id = config.nft_contract_actor_id;
        self.nft.owner = config.token_owner;
        self.discount_rate = config.discount_rate;
        self.starting_price = config.starting_price;

        msg::reply(
            Event::AuctionStarted {
                token_owner: self.nft.owner,
                price: self.starting_price,
                token_id: self.nft.token_id,
            },
            0,
        )
        .unwrap();
    }

    fn stop_if_time_is_over(&mut self) {
        if block_timestamp() >= self.expires_at {
            self.is_active = false
        }
    }

    fn force_stop(&mut self) {
        if msg::source() != self.owner {
            panic!("Can't stop if sender is not owner")
        }

        self.is_active = false;

        msg::reply(
            Event::AuctionStoped {
                token_owner: self.owner,
                token_id: self.nft.token_id,
            },
            0,
        )
        .unwrap();
    }

    fn info(&self) -> AuctionInfo {
        AuctionInfo {
            nft_contract_actor_id: self.nft.contract_id,
            token_id: self.nft.token_id,
            token_owner: self.nft.owner,
            starting_price: self.starting_price,
        }
    }
}

gstd::metadata! {
    title: "Auction",
    init:
        input: InitConfig,
    handle:
        input: Action,
        output: Event,
    state:
        input: State,
        output: StateReply,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let auction = Auction {
        owner: msg::source(),
        ..Default::default()
    };

    AUCTION = Some(auction)
}

#[gstd::async_main]
async unsafe fn main() {
    let action: Action = msg::load().expect("Could not load Action");
    let auction: &mut Auction = unsafe { AUCTION.get_or_insert(Auction::default()) };

    auction.stop_if_time_is_over();

    match action {
        Action::Buy => auction.buy().await,
        Action::Create(config) => auction.renew_contract(config),
        Action::ForceStop => auction.force_stop(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: State = msg::load().expect("failed to decode input argument");
    let auction: &mut Auction = AUCTION.get_or_insert(Auction::default());

    auction.stop_if_time_is_over();

    let encoded = match query {
        State::TokenPrice() => StateReply::TokenPrice(auction.token_price()),
        State::IsActive() => StateReply::IsActive(auction.is_active),
        State::Info() => StateReply::Info(auction.info()),
    }
    .encode();

    gstd::util::to_leak_ptr(encoded)
}
