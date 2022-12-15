#![no_std]

use ft_logic_io::Action;
use ft_main_io::{FTokenAction, FTokenEvent};
use gstd::{async_main, exec, metadata, msg, prelude::*, util, ActorId};
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128PlusPlus;

mod io;

pub use io::*;

static mut CONTRACT: Option<Goc> = None;

static mut DNS_META: Option<DnsMeta> = None;

#[derive(Default, Debug)]
struct Goc {
    admin: ActorId,
    ft_actor_id: Option<ActorId>,
    started: u64,
    ending: u64,
    players: Vec<ActorId>,
    prize_fund: u128,
    participation_cost: u128,
    winner: ActorId,
    transactions: BTreeMap<ActorId, u64>,
    transaction_id_nonce: u64,
}

impl Goc {
    fn start(
        &mut self,
        duration: u64,
        participation_cost: u128,
        ft_actor_id: Option<ActorId>,
    ) -> GOCEvent {
        self.assert_admin();

        if self.winner.is_zero() {
            panic!("Current game round must be over");
        }

        if matches!(ft_actor_id, Some(ft_actor_id) if ft_actor_id.is_zero()) {
            panic!("`ft_actor_id` mustn't be `ActorId::zero()`");
        }

        self.players.clear();

        self.winner = ActorId::zero();
        self.prize_fund = 0;
        self.started = exec::block_timestamp();
        self.ending = self.started + duration;
        self.participation_cost = participation_cost;
        self.ft_actor_id = ft_actor_id;

        GOCEvent::Started {
            ending: self.ending,
            participation_cost,
            ft_actor_id,
        }
    }

    fn assert_admin(&self) {
        if msg::source() != self.admin {
            panic!("`msg::source()` must be the game administrator");
        }
    }

    async fn pick_winner(&mut self) -> GOCEvent {
        self.assert_admin();

        if !self.winner.is_zero() {
            panic!("Winner mustn't already be picked");
        }

        let block_timestamp = exec::block_timestamp();

        if self.ending > block_timestamp {
            panic!("Players entry stage must be over");
        }

        let winner = if self.players.is_empty() {
            ActorId::zero()
        } else {
            let mut random_data = [0; (usize::BITS / 8) as usize];

            Xoshiro128PlusPlus::seed_from_u64(block_timestamp).fill_bytes(&mut random_data);

            let mystical_number = usize::from_le_bytes(random_data);

            let winner = self.players[mystical_number % self.players.len()];

            if let Some(ft_actor_id) = self.ft_actor_id {
                let result = self
                    .transfer_tokens(
                        ft_actor_id,
                        self.admin,
                        exec::program_id(),
                        winner,
                        self.prize_fund,
                    )
                    .await;

                if let FTokenEvent::Err = result {
                    panic!("Failed to transfer fungible tokens to a winner")
                }
            } else {
                msg::send_bytes(winner, [], self.prize_fund)
                    .expect("Failed to send the native value to a winner");
            }

            winner
        };

        self.winner = winner;
        self.started = 0;

        GOCEvent::Winner(winner)
    }

    async fn transfer_tokens(
        &mut self,
        ft_actor_id: ActorId,
        msg_source: ActorId,
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    ) -> FTokenEvent {
        let transaction_id = *self.transactions.entry(msg_source).or_insert_with(|| {
            let id = self.transaction_id_nonce;

            self.transaction_id_nonce = self.transaction_id_nonce.wrapping_add(1);

            id
        });

        let result = msg::send_for_reply_as(
            ft_actor_id,
            FTokenAction::Message {
                transaction_id,
                payload: Action::Transfer {
                    sender,
                    recipient,
                    amount,
                }
                .encode(),
            },
            0,
        )
        .expect("Failed to send `FTokenAction`")
        .await
        .expect("Failed to decode `FTokenEvent`");

        self.transactions.remove(&msg_source);

        result
    }

    async fn enter(&mut self) -> GOCEvent {
        if self.ending <= exec::block_timestamp() {
            panic!("Players entry stage mustn't be over");
        }

        let msg_source = msg::source();

        if self.players.contains(&msg_source) {
            panic!("`msg::source()` mustn't already participate")
        }

        if let Some(ft_actor_id) = self.ft_actor_id {
            let result = self
                .transfer_tokens(
                    ft_actor_id,
                    msg_source,
                    msg_source,
                    exec::program_id(),
                    self.participation_cost,
                )
                .await;

            if let FTokenEvent::Err = result {
                panic!("Failed to transfer fungible tokens for a participation");
            }
        } else if msg::value() != self.participation_cost {
            panic!("`msg::source()` must send the amount of the native value exactly equal to a participation cost");
        }

        self.players.push(msg_source);
        self.prize_fund = self.prize_fund.saturating_add(self.participation_cost);

        GOCEvent::PlayerAdded(msg_source)
    }
}

#[no_mangle]
extern "C" fn init() {
    let GOCInit { admin } = msg::load().expect("Failed to decode `GOCInit`");

    if admin.is_zero() {
        panic!("`admin` mustn't be `ActorId::zero()`");
    }

    let contract = Goc {
        admin,
        winner: admin,
        ..Default::default()
    };
    unsafe { CONTRACT = Some(contract) }
}

#[async_main]
async fn main() {
    let action: GOCAction = msg::load().expect("Failed to load or decode `GOCAction`");
    let contract = contract();

    let event = match action {
        GOCAction::GetDnsMeta => unsafe { GOCEvent::DnsMeta(DNS_META.clone()) },
        GOCAction::SetDnsMeta(meta) => unsafe {
            if contract.admin != msg::source() {
                panic!("Dns metadata can be added only by admin")
            }
            DNS_META = Some(meta);
            GOCEvent::DnsMeta(DNS_META.clone())
        },
        GOCAction::Start {
            duration,
            participation_cost,
            ft_actor_id,
        } => contract.start(duration, participation_cost, ft_actor_id),
        GOCAction::PickWinner => contract.pick_winner().await,
        GOCAction::Enter => contract.enter().await,
    };

    msg::reply(event, 0).expect("Failed to encode or reply with `GOCEvent`");
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let Goc {
        admin,
        ft_actor_id,
        started,
        ending,
        players,
        prize_fund,
        participation_cost,
        winner: last_winner,
        ..
    } = contract();

    let reply = GOCState {
        admin: *admin,
        ft_actor_id: *ft_actor_id,
        started: *started,
        ending: *ending,
        players: BTreeSet::from_iter(players.clone()),
        prize_fund: *prize_fund,
        participation_cost: *participation_cost,
        winner: *last_winner,
    };

    util::to_leak_ptr(reply.encode())
}

fn contract() -> &'static mut Goc {
    unsafe { CONTRACT.get_or_insert(Default::default()) }
}

metadata! {
    title: "Game of chance",
    init:
        input: GOCInit,
    handle:
        input: GOCAction,
        output: GOCEvent,
    state:
        output: GOCState,
}
