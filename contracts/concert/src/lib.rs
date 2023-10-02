#![no_std]

use concert_io::*;
use gear_lib_old::multitoken::io::*;
use gstd::{
    collections::{HashMap, HashSet},
    msg,
    prelude::*,
    ActorId,
};
use multi_token_io::MyMTKAction;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

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
    match action {
        ConcertAction::Create {
            creator,
            name,
            description,
            number_of_tickets,
            date,
        } => concert.create_concert(name, description, creator, number_of_tickets, date),
        ConcertAction::Hold => concert.hold_concert().await,
        ConcertAction::BuyTickets { amount, metadata } => {
            concert.buy_tickets(amount, metadata).await
        }
    }
}

impl Concert {
    fn create_concert(
        &mut self,
        name: String,
        description: String,
        creator: ActorId,
        number_of_tickets: u128,
        date: u128,
    ) {
        if self.running {
            panic!("CONCERT: There is already a concert registered.")
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
        msg::reply(
            ConcertEvent::Creation {
                creator,
                concert_id: self.concert_id,
                number_of_tickets,
                date,
            },
            0,
        )
        .expect("Error during a replying with ConcertEvent::Creation");
    }

    async fn buy_tickets(&mut self, amount: u128, mtd: Vec<Option<TokenMetadata>>) {
        if msg::source() == ZERO_ID {
            panic!("CONCERT: Message from zero address");
        }

        if amount < 1 {
            panic!("CONCERT: Can not buy less than 1 ticket");
        }

        if self.tickets_left < amount {
            panic!("CONCERT: Not enough tickets");
        }

        if mtd.len() != amount as usize {
            panic!("CONCERT: Metadata not provided for all the tickets");
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
        msg::send_for_reply_as::<_, MTKEvent>(
            self.contract_id,
            MyMTKAction::Mint {
                amount,
                token_metadata: None,
            },
            0,
            0,
        )
        .expect("Error in async message to MTK contract")
        .await
        .expect("CONCERT: Error minting concert tokens");

        msg::reply(
            ConcertEvent::Purchase {
                concert_id: self.concert_id,
                amount,
            },
            0,
        )
        .expect("Error during a replying with ConcertEvent::Purchase");
    }

    // MINT SEVERAL FOR A USER
    async fn hold_concert(&mut self) {
        if msg::source() != self.creator {
            panic!("CONCERT: Only creator can hold a concert");
        }
        // get balances from a contract
        let accounts: Vec<_> = self.buyers.clone().into_iter().collect();
        let tokens: Vec<TokenId> = iter::repeat(self.ticket_ft_id)
            .take(accounts.len())
            .collect();

        let balance_response: MTKEvent = msg::send_for_reply_as(
            self.contract_id,
            MyMTKAction::BalanceOfBatch {
                accounts,
                ids: tokens,
            },
            0,
            0,
        )
        .expect("Error in async message to MTK contract")
        .await
        .expect("CONCERT: Error getting balances from the contract");
        let balances: Vec<BalanceReply> =
            if let MTKEvent::BalanceOf(balance_response) = balance_response {
                balance_response
            } else {
                Vec::new()
            };
        // we know each user balance now
        for balance in &balances {
            msg::send_for_reply_as::<_, MTKEvent>(
                self.contract_id,
                MyMTKAction::Burn {
                    id: balance.id,
                    amount: balance.amount,
                },
                0,
                0,
            )
            .expect("Error in async message to MTK contract")
            .await
            .expect("CONCERT: Error burning balances");
        }

        for actor in &self.buyers {
            let mut ids = vec![];
            let mut amounts = vec![];
            let mut meta = vec![];
            let actor_metadata = self.metadata.get(actor);
            if let Some(actor_md) = actor_metadata.cloned() {
                for (token, token_meta) in actor_md {
                    ids.push(token);
                    amounts.push(NFT_COUNT);
                    meta.push(token_meta);
                }
                msg::send_for_reply_as::<_, MTKEvent>(
                    self.contract_id,
                    MyMTKAction::MintBatch {
                        ids,
                        amounts,
                        tokens_metadata: meta,
                    },
                    0,
                    0,
                )
                .expect("Error in async message to MTK contract")
                .await
                .expect("CONCERT: Error minting tickets");
            }
        }
        self.running = false;
        msg::reply(
            ConcertEvent::Hold {
                concert_id: self.concert_id,
            },
            0,
        )
        .expect("Error during a replying with ConcertEvent::Hold");
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
        } = value;

        let buyers = buyers.iter().copied().collect();

        let metadata = metadata
            .iter()
            .map(|(k, v)| (*k, v.iter().map(|(k, v)| (*k, v.clone())).collect()))
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
        }
    }
}
