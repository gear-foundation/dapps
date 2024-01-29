#![no_std]

use concert_io::*;
use gstd::{
    collections::{HashMap, HashSet},
    msg,
    prelude::*,
    ActorId,
};
use multi_token_io::{BalanceReply, MtkAction, MtkEvent, TokenId, TokenMetadata};

const ZERO_ID: ActorId = ActorId::zero();
const NFT_COUNT: u128 = 1;

#[derive(Default)]
struct Concert {
    owner_id: ActorId,
    contract_id: ActorId,
    name: String,
    description: String,
    ticket_ft_id: u128,
    creator: ActorId,
    number_of_tickets: u128,
    tickets_left: u128,
    date: u128,
    buyers: HashSet<ActorId>,
    id_counter: u128,
    concert_id: u128,
    running: bool,
    metadata: HashMap<ActorId, HashMap<u128, Option<TokenMetadata>>>,
    token_id: u128,
}

static mut CONTRACT: Option<Concert> = None;

#[no_mangle]
unsafe extern fn init() {
    let config: InitConcert = msg::load().expect("Unable to decode InitConfig");
    let concert = Concert {
        owner_id: config.owner_id,
        contract_id: config.mtk_contract,
        ..Default::default()
    };
    CONTRACT = Some(concert);
}

#[gstd::async_main]
async unsafe fn main() {
    let action: ConcertAction = msg::load().expect("Could not load Action");
    let concert: &mut Concert = unsafe { CONTRACT.get_or_insert(Default::default()) };
    let reply = match action {
        ConcertAction::Create {
            creator,
            name,
            description,
            number_of_tickets,
            date,
            token_id,
        } => concert.create_concert(
            name,
            description,
            creator,
            number_of_tickets,
            date,
            token_id,
        ),
        ConcertAction::Hold => concert.hold_concert().await,
        ConcertAction::BuyTickets { amount, metadata } => {
            concert.buy_tickets(amount, metadata).await
        }
    };
    msg::reply(reply, 0)
        .expect("Failed to encode or reply with `Result<ConcertEvent, ConcertError>`.");
}

impl Concert {
    fn create_concert(
        &mut self,
        name: String,
        description: String,
        creator: ActorId,
        number_of_tickets: u128,
        date: u128,
        token_id: u128,
    ) -> Result<ConcertEvent, ConcertError> {
        if self.running {
            return Err(ConcertError::AlreadyRegistered);
        }
        self.creator = creator;
        self.concert_id = self.id_counter;
        self.ticket_ft_id = self.concert_id;
        self.name = name;
        self.description = description;
        self.number_of_tickets = number_of_tickets;
        self.date = date;
        self.running = true;
        self.tickets_left = number_of_tickets;
        self.token_id = token_id;

        Ok(ConcertEvent::Creation {
            creator,
            concert_id: self.concert_id,
            number_of_tickets,
            date,
        })
    }

    async fn buy_tickets(
        &mut self,
        amount: u128,
        mtd: Vec<Option<TokenMetadata>>,
    ) -> Result<ConcertEvent, ConcertError> {
        if msg::source() == ZERO_ID {
            return Err(ConcertError::ZeroAddress);
        }

        if amount < 1 {
            return Err(ConcertError::LessThanOneTicket);
        }

        if self.tickets_left < amount {
            return Err(ConcertError::NotEnoughTickets);
        }

        if mtd.len() != amount as usize {
            return Err(ConcertError::NotEnoughMetadata);
        }

        for meta in mtd {
            self.id_counter += 1;
            self.metadata
                .entry(msg::source())
                .or_default()
                .insert(self.id_counter + 1, meta);
        }

        self.buyers.insert(msg::source());
        self.tickets_left -= amount;
        msg::send_for_reply_as::<_, MtkEvent>(
            self.contract_id,
            MtkAction::Mint {
                id: self.token_id,
                amount,
                token_metadata: None,
            },
            0,
            0,
        )
        .expect("Error in async message to Mtk contract")
        .await
        .expect("CONCERT: Error minting concert tokens");

        Ok(ConcertEvent::Purchase {
            concert_id: self.concert_id,
            amount,
        })
    }

    // MINT SEVERAL FOR A USER
    async fn hold_concert(&mut self) -> Result<ConcertEvent, ConcertError> {
        if msg::source() != self.creator {
            return Err(ConcertError::NotCreator);
        }
        // get balances from a contract
        let accounts: Vec<_> = self.buyers.clone().into_iter().collect();
        let tokens: Vec<TokenId> = iter::repeat(self.ticket_ft_id)
            .take(accounts.len())
            .collect();

        let balance_response: MtkEvent = msg::send_for_reply_as(
            self.contract_id,
            MtkAction::BalanceOfBatch {
                accounts,
                ids: tokens,
            },
            0,
            0,
        )
        .expect("Error in async message to Mtk contract")
        .await
        .expect("CONCERT: Error getting balances from the contract");
        let balances: Vec<BalanceReply> =
            if let MtkEvent::BalanceOf(balance_response) = balance_response {
                balance_response
            } else {
                Vec::new()
            };
        // we know each user balance now
        for balance in &balances {
            msg::send_for_reply_as::<_, MtkEvent>(
                self.contract_id,
                MtkAction::Burn {
                    id: balance.id,
                    amount: balance.amount,
                },
                0,
                0,
            )
            .expect("Error in async message to Mtk contract")
            .await
            .expect("CONCERT: Error burning balances");
        }

        for actor in &self.buyers {
            let actor_metadata = self.metadata.get(actor);
            if let Some(actor_md) = actor_metadata.cloned() {
                let mut ids = Vec::with_capacity(actor_md.len());
                let amounts = vec![NFT_COUNT; actor_md.len()];
                let mut meta = vec![];
                for (token, token_meta) in actor_md {
                    ids.push(token);
                    meta.push(token_meta);
                }
                msg::send_for_reply_as::<_, MtkEvent>(
                    self.contract_id,
                    MtkAction::MintBatch {
                        ids,
                        amounts,
                        tokens_metadata: meta,
                    },
                    0,
                    0,
                )
                .expect("Error in async message to Mtk contract")
                .await
                .expect("CONCERT: Error minting tickets");
            }
        }
        self.running = false;

        Ok(ConcertEvent::Hold {
            concert_id: self.concert_id,
        })
    }
}

#[no_mangle]
extern fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(contract.into(), 0)
        .expect("Failed to encode or reply with `State` from `state()`");
}

impl From<Concert> for State {
    fn from(value: Concert) -> Self {
        let Concert {
            owner_id,
            contract_id,
            name,
            description,
            ticket_ft_id,
            creator,
            number_of_tickets,
            tickets_left,
            date,
            buyers,
            id_counter,
            concert_id,
            running,
            metadata,
            token_id,
        } = value;

        let buyers = buyers.into_iter().collect();

        let metadata = metadata
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();

        State {
            owner_id,
            contract_id,
            name,
            description,
            ticket_ft_id,
            creator,
            number_of_tickets,
            tickets_left,
            date,
            buyers,
            id_counter,
            concert_id,
            running,
            metadata,
            token_id,
        }
    }
}
