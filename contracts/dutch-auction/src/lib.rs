#![no_std]

use codec::Encode;
use core::cmp::min;
use gstd::{exec, exec::block_timestamp, msg, prelude::*, ActorId};
use nft_io::{NFTAction, NFTEvent};
use primitive_types::U256;

pub mod state;
pub use state::*;

pub use auction_io::*;

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
    pub status: Status,
    pub started_at: u64,
    pub expires_at: u64,
}

static mut AUCTION: Option<Auction> = None;

impl Auction {
    async fn buy(&mut self) {
        if !self.is_active() {
            panic!("already bought or auction expired");
        }

        if block_timestamp() >= self.expires_at {
            panic!("auction expired");
        }

        let price = self.token_price();

        if msg::value() < price {
            panic!("value < price, {:?} < {:?}", msg::value(), price);
        }

        self.status = Status::Purchased { price };

        let refund = msg::value() - price;
        let refund = if refund < 500 { 0 } else { refund };

        msg::send_for_reply(
            self.nft.contract_id,
            NFTAction::Transfer {
                to: msg::source(),
                token_id: self.nft.token_id,
            },
            0,
        )
        .unwrap()
        .await
        .expect("Error in nft transfer");

        msg::send(self.nft.owner, "REWARD", price).expect("Couldn't send payment for nft owner");
        msg::reply(Event::Bought { price }, refund).expect("Can't send refund and reply");
    }

    fn token_price(&self) -> u128 {
        // time_elapsed is in seconds
        let time_elapsed = block_timestamp().saturating_sub(self.started_at) / 1000;
        let discount = min(
            self.discount_rate * (time_elapsed as u128),
            self.starting_price,
        );

        self.starting_price - discount
    }

    async fn renew_contract(&mut self, config: CreateConfig) {
        if self.is_active() {
            panic!("already in use")
        }

        let minutes_count = config.duration.hours * 60 + config.duration.minutes;
        let duration_in_seconds = minutes_count * 60 + config.duration.seconds;

        if config.starting_price < config.discount_rate * (duration_in_seconds as u128) {
            panic!("starting price < min");
        }

        self.validate_nft_approve(config.nft_contract_actor_id, config.token_id)
            .await;

        self.status = Status::IsRunning;
        self.started_at = block_timestamp();
        self.expires_at = block_timestamp() + duration_in_seconds * 1000;
        self.nft.token_id = config.token_id;
        self.nft.contract_id = config.nft_contract_actor_id;
        self.nft.owner = Self::get_token_owner(config.nft_contract_actor_id, config.token_id).await;

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

    async fn get_token_owner(contract_id: ActorId, token_id: U256) -> ActorId {
        let reply: NFTEvent = msg::send_for_reply_as(contract_id, NFTAction::Owner { token_id }, 0)
            .expect("Can't send message")
            .await
            .expect("Unable to decode `NFTEvent`");

        if let NFTEvent::Owner { owner, .. } = reply {
            owner
        } else {
            panic!("Wrong received message!")
        }
    }

    async fn validate_nft_approve(&self, contract_id: ActorId, token_id: U256) {
        let reply: NFTEvent = msg::send_for_reply_as(
            contract_id,
            NFTAction::IsApproved {
                token_id,
                to: exec::program_id(),
            },
            0,
        )
        .expect("Can't send message")
        .await
        .expect("Unable to decode `NFTEvent`");

        if let NFTEvent::IsApproved { approved, .. } = reply {
            if !approved {
                panic!("You must approve your NFT to this contract before")
            }
        } else {
            panic!("Wrong received message!")
        }
    }

    fn stop_if_time_is_over(&mut self) {
        if self.is_active() && block_timestamp() >= self.expires_at {
            self.status = Status::Expired;
        }
    }

    fn is_active(&self) -> bool {
        match self.status {
            Status::None | Status::Purchased { .. } | Status::Expired | Status::Stopped => false,
            Status::IsRunning => true,
        }
    }

    fn force_stop(&mut self) {
        if msg::source() != self.owner {
            panic!("Can't stop if sender is not owner")
        }

        self.status = Status::Stopped;

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
            auction_owner: self.owner,
            starting_price: self.starting_price,
            current_price: self.token_price(),
            discount_rate: self.discount_rate,
            time_left: self.expires_at.saturating_sub(block_timestamp()),
            status: self.status.clone(),
        }
    }
}

gstd::metadata! {
    title: "Auction",
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
        Action::Create(config) => auction.renew_contract(config).await,
        Action::ForceStop => auction.force_stop(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: State = msg::load().expect("failed to decode input argument");
    let auction: &mut Auction = AUCTION.get_or_insert(Auction::default());

    auction.stop_if_time_is_over();

    let encoded = match query {
        State::Info => StateReply::Info(auction.info()),
    }
    .encode();

    gstd::util::to_leak_ptr(encoded)
}
