#![no_std]

use core::fmt::Debug;
use gstd::{ext, format};
use sails_rs::gstd::msg;
use sails_rs::{
    collections::{HashMap, HashSet},
    prelude::*,
};

const ZERO_ID: ActorId = ActorId::zero();
const NFT_COUNT: U256 = U256::one();

#[derive(Default, Clone)]
pub struct Storage {
    owner_id: ActorId,
    contract_id: ActorId,
    name: String,
    description: String,
    ticket_ft_id: U256,
    creator: ActorId,
    number_of_tickets: U256,
    tickets_left: U256,
    date: u128,
    buyers: HashSet<ActorId>,
    id_counter: U256,
    concert_id: U256,
    running: bool,
    metadata: HashMap<ActorId, HashMap<U256, Option<TokenMetadata>>>,
    token_id: U256,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub reference: Option<String>,
}

static mut STORAGE: Option<Storage> = None;

#[derive(Debug, Clone, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Event {
    Creation {
        creator: ActorId,
        concert_id: U256,
        number_of_tickets: U256,
        date: u128,
    },
    Hold {
        concert_id: U256,
    },
    Purchase {
        concert_id: U256,
        amount: U256,
    },
}

#[derive(Debug)]
pub enum ConcertError {
    AlreadyRegistered,
    ZeroAddress,
    LessThanOneTicket,
    NotEnoughTickets,
    NotEnoughMetadata,
    NotCreator,
}

struct ConcertService(());

impl ConcertService {
    pub fn init(owner_id: ActorId, vmt_contract: ActorId) -> Self {
        let storage = Storage {
            owner_id,
            contract_id: vmt_contract,
            ..Default::default()
        };
        unsafe { STORAGE = Some(storage) };
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Storage is not initialized") }
    }
    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Storage is not initialized") }
    }
}

#[service(events = Event)]
impl ConcertService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn create(
        &mut self,
        creator: ActorId,
        name: String,
        description: String,
        number_of_tickets: U256,
        date: u128,
        token_id: U256,
    ) {
        let storage = self.get_mut();
        if storage.running {
            panic(ConcertError::AlreadyRegistered);
        }
        storage.creator = creator;
        storage.concert_id = storage.id_counter;
        storage.ticket_ft_id = storage.concert_id;
        storage.name = name;
        storage.description = description;
        storage.number_of_tickets = number_of_tickets;
        storage.date = date;
        storage.running = true;
        storage.tickets_left = number_of_tickets;
        storage.token_id = token_id;

        self.notify_on(Event::Creation {
            creator,
            concert_id: storage.concert_id,
            number_of_tickets,
            date,
        })
        .expect("Notification Error");
    }

    pub async fn hold_concert(&mut self) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        if msg_src != storage.creator {
            panic(ConcertError::NotCreator);
        }
        // get balances from a contract
        let accounts: Vec<_> = storage.buyers.clone().into_iter().collect();
        let tokens: Vec<U256> = iter::repeat(storage.token_id)
            .take(accounts.len())
            .collect();

        let request = [
            "Vmt".encode(),
            "BalanceOfBatch".to_string().encode(),
            (accounts.clone(), tokens.clone()).encode(),
        ]
        .concat();

        let bytes_reply_balances = msg::send_bytes_for_reply(storage.contract_id, request, 0, 0)
            .expect("Error in async message to Mtk contract")
            .await
            .expect("CONCERT: Error getting balances from the contract");

        let (_, _, balances) =
            <(String, String, Vec<U256>)>::decode(&mut bytes_reply_balances.as_ref())
                .expect("Unable to decode reply");
        // we know each user balance now
        for (i, balance) in balances.iter().enumerate() {
            let request = [
                "Vmt".encode(),
                "Burn".to_string().encode(),
                (msg_src, tokens[i], balance).encode(),
            ]
            .concat();

            msg::send_bytes_for_reply(storage.contract_id, request, 0, 0)
                .expect("Error in async message to Mtk contract")
                .await
                .expect("CONCERT: Error burning balances");
        }

        for actor in &storage.buyers {
            let actor_metadata = storage.metadata.get(actor);
            if let Some(actor_md) = actor_metadata.cloned() {
                let mut ids: Vec<U256> = Vec::with_capacity(actor_md.len());
                let amounts: Vec<U256> = vec![NFT_COUNT; actor_md.len()];
                let mut meta = vec![];
                for (token, token_meta) in actor_md {
                    ids.push(token);
                    meta.push(token_meta);
                }

                let request = [
                    "Vmt".encode(),
                    "MintBatch".to_string().encode(),
                    (actor, ids, amounts, meta).encode(),
                ]
                .concat();

                msg::send_bytes_for_reply(storage.contract_id, request, 0, 0)
                    .expect("Error in async message to Mtk contract")
                    .await
                    .expect("CONCERT: Error minting tickets");
            }
        }
        storage.running = false;

        self.notify_on(Event::Hold {
            concert_id: storage.concert_id,
        })
        .expect("Notification Error");
    }

    pub async fn buy_tickets(&mut self, amount: U256, mtd: Vec<Option<TokenMetadata>>) {
        let storage = self.get_mut();
        let msg_src = msg::source();
        if msg_src == ZERO_ID {
            panic(ConcertError::ZeroAddress);
        }

        if amount < U256::one() {
            panic(ConcertError::LessThanOneTicket);
        }

        if storage.tickets_left < amount {
            panic(ConcertError::NotEnoughTickets);
        }

        if U256::from(mtd.len()) != amount {
            panic(ConcertError::NotEnoughMetadata);
        }

        for meta in mtd {
            storage.id_counter += U256::one();
            storage
                .metadata
                .entry(msg_src)
                .or_default()
                .insert(storage.id_counter + U256::one(), meta);
        }

        storage.buyers.insert(msg_src);
        storage.tickets_left -= amount;
        let request = [
            "Vmt".encode(),
            "Mint".to_string().encode(),
            (msg_src, storage.token_id, amount, None::<TokenMetadata>).encode(),
        ]
        .concat();

        msg::send_bytes_for_reply(storage.contract_id, request, 0, 0)
            .expect("Error in async message to Mtk contract")
            .await
            .expect("CONCERT: Error minting concert tokens");

        self.notify_on(Event::Purchase {
            concert_id: storage.concert_id,
            amount,
        })
        .expect("Notification Error");
    }

    pub fn get_storage(&self) -> State {
        self.get().clone().into()
    }
}

pub struct ConcertProgram(());

#[sails_rs::program]
impl ConcertProgram {
    #[allow(clippy::new_without_default)]
    pub fn new(owner_id: ActorId, vmt_contract: ActorId) -> Self {
        ConcertService::init(owner_id, vmt_contract);
        Self(())
    }

    pub fn concert(&self) -> ConcertService {
        ConcertService::new()
    }
}

pub fn panic(err: impl Debug) -> ! {
    ext::panic(&format!("{err:?}"))
}

pub type Tickets = Vec<(U256, Option<TokenMetadata>)>;

#[derive(Debug, Default, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct State {
    pub owner_id: ActorId,
    pub contract_id: ActorId,

    pub name: String,
    pub description: String,

    pub ticket_ft_id: U256,
    pub creator: ActorId,
    pub number_of_tickets: U256,
    pub tickets_left: U256,
    pub date: u128,

    pub buyers: Vec<ActorId>,

    pub id_counter: U256,
    pub concert_id: U256,
    pub running: bool,
    /// user to token id to metadata
    pub metadata: Vec<(ActorId, Tickets)>,
    pub token_id: U256,
}

impl From<Storage> for State {
    fn from(value: Storage) -> Self {
        let Storage {
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
