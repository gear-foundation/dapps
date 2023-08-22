use auction_io::auction::{
    Action, AuctionInfo, CreateConfig, Error, Event, Status, Transaction, TransactionId,
};
use auction_io::io::AuctionMetadata;
use core::cmp::min;
use gmeta::Metadata;
use gstd::ActorId;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, MessageId};
use nft_io::{NFTAction, NFTEvent};
use primitive_types::U256;

static mut AUCTION: Option<Auction> = None;

#[derive(Debug, Clone, Default)]
pub struct Nft {
    pub token_id: U256,
    pub owner: ActorId,
    pub contract_id: ActorId,
}

#[derive(Debug, Clone, Default)]
pub struct Auction {
    pub owner: ActorId,
    pub nft: Nft,
    pub starting_price: u128,
    pub discount_rate: u128,
    pub status: Status,
    pub started_at: u64,
    pub expires_at: u64,

    pub transactions: BTreeMap<ActorId, Transaction<Action>>,
    pub current_tid: TransactionId,
}

impl Auction {
    pub async fn buy(&mut self, transaction_id: TransactionId) -> Result<(Event, u128), Error> {
        if !matches!(self.status, Status::IsRunning) {
            return Err(Error::AlreadyStopped);
        }

        if exec::block_timestamp() >= self.expires_at {
            return Err(Error::Expired);
        }

        let price = self.token_price();
        let value = msg::value();
        if value < price {
            return Err(Error::InsufficientMoney);
        }

        self.status = Status::Purchased { price };

        let refund = value - price;
        let refund = if refund < 500 { 0 } else { refund };

        let reply = match msg::send_for_reply(
            self.nft.contract_id,
            NFTAction::Transfer {
                to: msg::source(),
                token_id: self.nft.token_id,
                transaction_id,
            },
            0,
            0,
        ) {
            Ok(reply) => reply,
            Err(_e) => {
                return Err(Error::NftTransferFailed);
            }
        };

        match reply.await {
            Ok(_reply) => {}
            Err(_e) => {
                return Err(Error::NftTransferFailed);
            }
        }

        Ok((Event::Bought { price }, refund))
    }

    pub fn token_price(&self) -> u128 {
        // time_elapsed is in seconds
        let time_elapsed = exec::block_timestamp().saturating_sub(self.started_at) / 1000;
        let discount = min(
            self.discount_rate * (time_elapsed as u128),
            self.starting_price,
        );

        self.starting_price - discount
    }

    pub async fn renew_contract(
        &mut self,
        transaction_id: TransactionId,
        config: &CreateConfig,
    ) -> Result<Event, Error> {
        if matches!(self.status, Status::IsRunning) {
            return Err(Error::AlreadyRunning);
        }

        let minutes_count = config.duration.hours * 60 + config.duration.minutes;
        let duration_in_seconds = minutes_count * 60 + config.duration.seconds;

        if config.starting_price < config.discount_rate * (duration_in_seconds as u128) {
            return Err(Error::StartPriceLessThatMinimal);
        }
        self.validate_nft_approve(config.nft_contract_actor_id, config.token_id)
            .await?;
        self.status = Status::IsRunning;
        self.started_at = exec::block_timestamp();
        self.expires_at = self.started_at + duration_in_seconds * 1000;
        self.nft.token_id = config.token_id;
        self.nft.contract_id = config.nft_contract_actor_id;
        self.nft.owner =
            Self::get_token_owner(config.nft_contract_actor_id, config.token_id).await?;

        self.discount_rate = config.discount_rate;
        self.starting_price = config.starting_price;

        msg::send_for_reply(
            self.nft.contract_id,
            NFTAction::Transfer {
                transaction_id,
                to: exec::program_id(),
                token_id: self.nft.token_id,
            },
            0,
            0,
        )
        .expect("Send NFTAction::Transfer at renew contract")
        .await
        .map_err(|_e| Error::NftTransferFailed)?;
        Ok(Event::AuctionStarted {
            token_owner: self.owner,
            price: self.starting_price,
            token_id: self.nft.token_id,
        })
    }

    pub async fn reward(&mut self) -> Result<Event, Error> {
        let price = match self.status {
            Status::Purchased { price } => price,
            _ => return Err(Error::WrongState),
        };
        if msg::source().ne(&self.nft.owner) {
            return Err(Error::IncorrectRewarder);
        }

        if let Err(_e) = msg::send(self.nft.owner, "REWARD", price) {
            return Err(Error::RewardSendFailed);
        }
        self.status = Status::Rewarded { price };
        Ok(Event::Rewarded { price })
    }

    pub async fn get_token_owner(contract_id: ActorId, token_id: U256) -> Result<ActorId, Error> {
        let reply: NFTEvent =
            msg::send_for_reply_as(contract_id, NFTAction::Owner { token_id }, 0, 0)
                .map_err(|_e| Error::SendingError)?
                .await
                .map_err(|_e| Error::NftOwnerFailed)?;

        if let NFTEvent::Owner { owner, .. } = reply {
            Ok(owner)
        } else {
            Err(Error::WrongReply)
        }
    }

    pub async fn validate_nft_approve(
        &self,
        contract_id: ActorId,
        token_id: U256,
    ) -> Result<(), Error> {
        let to = exec::program_id();
        let reply: NFTEvent =
            msg::send_for_reply_as(contract_id, NFTAction::IsApproved { token_id, to }, 0, 0)
                .map_err(|_e| Error::SendingError)?
                .await
                .map_err(|_e| Error::NftNotApproved)?;

        if let NFTEvent::IsApproved { approved, .. } = reply {
            if !approved {
                return Err(Error::NftNotApproved);
            }
        } else {
            return Err(Error::WrongReply);
        }
        Ok(())
    }

    pub fn stop_if_time_is_over(&mut self) {
        if matches!(self.status, Status::IsRunning) && exec::block_timestamp() >= self.expires_at {
            self.status = Status::Expired;
        }
    }

    pub async fn force_stop(&mut self, transaction_id: TransactionId) -> Result<Event, Error> {
        if msg::source() != self.owner {
            return Err(Error::NotOwner);
        }
        if let Status::Purchased { price: _ } = self.status {
            return Err(Error::NotRewarded);
        }

        let stopped = Event::AuctionStopped {
            token_owner: self.owner,
            token_id: self.nft.token_id,
        };
        if let Status::Rewarded { price: _ } = self.status {
            return Ok(stopped);
        }
        if let Err(_e) = msg::send_for_reply(
            self.nft.contract_id,
            NFTAction::Transfer {
                transaction_id,
                to: self.nft.owner,
                token_id: self.nft.token_id,
            },
            0,
            0,
        )
        .expect("Can't send NFTAction::Transfer at force stop")
        .await
        {
            return Err(Error::NftTransferFailed);
        }

        self.status = Status::Stopped;

        Ok(stopped)
    }

    pub fn info(&mut self) -> AuctionInfo {
        self.stop_if_time_is_over();
        AuctionInfo {
            nft_contract_actor_id: self.nft.contract_id,
            token_id: self.nft.token_id,
            token_owner: self.nft.owner,
            auction_owner: self.owner,
            starting_price: self.starting_price,
            current_price: self.token_price(),
            discount_rate: self.discount_rate,
            time_left: self.expires_at.saturating_sub(exec::block_timestamp()),
            expires_at: self.expires_at,
            status: self.status.clone(),
            transactions: self.transactions.clone(),
            current_tid: self.current_tid,
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let auction = Auction {
        owner: msg::source(),
        ..Default::default()
    };

    unsafe { AUCTION = Some(auction) };
}

#[gstd::async_main]
async fn main() {
    let action: Action = msg::load().expect("Could not load Action");
    let auction: &mut Auction = unsafe { AUCTION.get_or_insert(Auction::default()) };

    auction.stop_if_time_is_over();

    let msg_source = msg::source();

    let r: Result<Action, Error> = Err(Error::PreviousTxMustBeCompleted);
    let transaction_id = if let Some(Transaction {
        id: tid,
        action: pend_action,
    }) = auction.transactions.get(&msg_source)
    {
        if action != *pend_action {
            reply(r, 0).expect("Failed to encode or reply with `Result<Action, Error>`");
            return;
        }
        *tid
    } else {
        let transaction_id = auction.current_tid;
        auction.transactions.insert(
            msg_source,
            Transaction {
                id: transaction_id,
                action: action.clone(),
            },
        );
        auction.current_tid = auction.current_tid.wrapping_add(1);
        transaction_id
    };

    let (result, value) = match &action {
        Action::Buy => {
            let reply = auction.buy(transaction_id).await;
            let result = match reply {
                Ok((event, refund)) => (Ok(event), refund),
                Err(_e) => (Err(_e), 0),
            };
            auction.transactions.remove(&msg_source);
            result
        }
        Action::Create(config) => {
            let result = (auction.renew_contract(transaction_id, config).await, 0);
            auction.transactions.remove(&msg_source);
            result
        }
        Action::ForceStop => {
            let result = (auction.force_stop(transaction_id).await, 0);
            auction.transactions.remove(&msg_source);
            result
        }
        Action::Reward => {
            let result = (auction.reward().await, 0);
            auction.transactions.remove(&msg_source);
            result
        }
    };
    reply(result, value).expect("Failed to encode or reply with `Result<Event, Error>`");
}

fn common_state() -> <AuctionMetadata as Metadata>::State {
    static_mut_state().info()
}

fn static_mut_state() -> &'static mut Auction {
    unsafe { AUCTION.get_or_insert(Default::default()) }
}

#[no_mangle]
extern "C" fn state() {
    reply(common_state(), 0).expect(
        "Failed to encode or reply with `<AuctionMetadata as Metadata>::State` from `state()`",
    );
}
fn reply(payload: impl Encode, value: u128) -> GstdResult<MessageId> {
    msg::reply(payload, value)
}
