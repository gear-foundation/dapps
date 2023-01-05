#![no_std]

use ft_logic_io::Action;
use ft_main_io::{FTokenAction, FTokenEvent};
use gstd::{
    async_main, errors::Result as GstdResult, exec, metadata, msg, prelude::*, util, ActorId,
    MessageId,
};
use hashbrown::HashMap;
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128PlusPlus;

mod io;

pub use io::*;

const MAX_NUMBER_OF_TXS: usize = 2usize.pow(16);

static mut STATE: Option<Goc> = None;

#[derive(Default, Debug)]
struct Goc {
    admin: ActorId,

    ft_actor_id: Option<ActorId>,
    started: u64,
    ending: u64,
    players: Vec<ActorId>,
    prize_fund: u128,
    participation_cost: u128,
    is_active: bool,

    winner: Option<ActorId>,

    txs_for_actor: BTreeMap<u64, ActorId>,
    actors_for_tx: HashMap<ActorId, u64>,
    tx_id_nonce: u64,
}

impl Goc {
    fn start(
        &mut self,
        duration: u64,
        participation_cost: u128,
        ft_actor_id: Option<ActorId>,
    ) -> Result<GOCEvent, GOCError> {
        if self.admin != msg::source() {
            return Err(GOCError::AccessRestricted);
        }

        if self.is_active {
            return Err(GOCError::UnexpectedGameStatus);
        }

        if matches!(ft_actor_id, Some(ft_actor_id) if ft_actor_id.is_zero()) {
            return Err(GOCError::ZeroActorId);
        }

        self.players.clear();

        self.winner = None;
        self.prize_fund = 0;
        self.started = exec::block_timestamp();
        self.ending = self.started.saturating_add(duration);
        self.participation_cost = participation_cost;
        self.ft_actor_id = ft_actor_id;
        self.is_active = true;

        // TODO: uncomment and update doc & tests after closing
        // https://github.com/gear-tech/gear/issues/1781.
        // msg::send_delayed(
        //     exec::program_id(),
        //     GOCAction::PickWinner,
        //     0,
        //     (duration / 1000) as u32,
        // )?;

        Ok(GOCEvent::Started {
            ending: self.ending,
            participation_cost,
            ft_actor_id,
        })
    }

    async fn pick_winner(&mut self) -> Result<GOCEvent, GOCError> {
        if !self.is_active {
            return Err(GOCError::UnexpectedGameStatus);
        }

        let msg_source = msg::source();
        let exec_program = exec::program_id();
        let block_timestamp = exec::block_timestamp();

        if msg_source == self.admin {
            if self.ending > block_timestamp {
                return Err(GOCError::UnexpectedGameStatus);
            }
        } else if msg_source != exec_program {
            return Err(GOCError::AccessRestricted);
        }

        let winner = self.winner.unwrap_or_else(|| {
            let winner = if self.players.is_empty() {
                ActorId::zero()
            } else {
                let mut random_data = [0; (usize::BITS / 8) as usize];

                Xoshiro128PlusPlus::seed_from_u64(block_timestamp).fill_bytes(&mut random_data);

                let mystical_number = usize::from_le_bytes(random_data);
                self.players[mystical_number % self.players.len()]
            };

            self.winner = Some(winner);

            winner
        });

        if let Some(ft_actor_id) = self.ft_actor_id {
            self.transfer_tokens(
                ft_actor_id,
                self.admin,
                exec_program,
                winner,
                self.prize_fund,
            )
            .await?;
        } else {
            send_value(winner, self.prize_fund)?;
        }

        self.is_active = false;

        Ok(GOCEvent::Winner(winner))
    }

    async fn transfer_tokens(
        &mut self,
        ft_actor_id: ActorId,
        msg_source: ActorId,
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    ) -> Result<(), GOCError> {
        let transaction_id = if let Some(id) = self.actors_for_tx.get(&msg_source) {
            *id
        } else {
            let id = self.tx_id_nonce;

            self.tx_id_nonce = id.wrapping_add(1);

            if self.txs_for_actor.len() == MAX_NUMBER_OF_TXS {
                let (tx, actor) = self
                    .txs_for_actor
                    .range(self.tx_id_nonce..)
                    .next()
                    .unwrap_or_else(|| self.txs_for_actor.first_key_value().unwrap());
                let (tx, actor) = (*tx, *actor);

                self.txs_for_actor.remove(&tx);
                self.actors_for_tx.remove(&actor);
            }

            self.txs_for_actor.insert(id, msg_source);
            self.actors_for_tx.insert(msg_source, id);

            id
        };

        let result = match msg::send_for_reply_as(
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
        )?
        .await?
        {
            FTokenEvent::Ok => Ok(()),
            FTokenEvent::Err => Err(GOCError::TokenTransferFailed),
            _ => unreachable!("Received an unexpected `FTokenEvent` variant"),
        };

        self.txs_for_actor.remove(&transaction_id);
        self.actors_for_tx.remove(&msg_source);

        result
    }

    async fn enter(&mut self) -> Result<GOCEvent, GOCError> {
        if self.ending <= exec::block_timestamp() {
            return Err(GOCError::UnexpectedGameStatus);
        }

        if self.players.len() == MAX_NUMBER_OF_PLAYERS {
            return Err(GOCError::MemoryLimitExceeded);
        }

        let msg_source = msg::source();

        if self.players.contains(&msg_source) {
            return Err(GOCError::AlreadyParticipating);
        }

        if let Some(ft_actor_id) = self.ft_actor_id {
            self.transfer_tokens(
                ft_actor_id,
                msg_source,
                msg_source,
                exec::program_id(),
                self.participation_cost,
            )
            .await?;
        } else {
            let msg_value = msg::value();

            if msg_value != self.participation_cost {
                send_value(msg_source, msg_value)?;

                return Err(GOCError::InvalidParticipationCost);
            }
        }

        self.players.push(msg_source);
        self.prize_fund = self.prize_fund.saturating_add(self.participation_cost);

        Ok(GOCEvent::PlayerAdded(msg_source))
    }
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}

fn send_value(program: ActorId, value: u128) -> GstdResult<MessageId> {
    msg::send_bytes(program, [], value)
}

#[no_mangle]
extern "C" fn init() {
    let result = process_init();
    let is_err = result.is_err();

    reply(result).expect("Failed to encode or reply with `Result<(), GOCError>`");

    if is_err {
        exec::exit(ActorId::zero());
    }
}

fn process_init() -> Result<(), GOCError> {
    let GOCInit { admin } = msg::load()?;

    if admin.is_zero() {
        return Err(GOCError::ZeroActorId);
    }

    let contract = Goc {
        admin,
        ..Default::default()
    };

    unsafe { STATE = Some(contract) }

    Ok(())
}

#[async_main]
async fn main() {
    reply(process_handle().await)
        .expect("Failed to encode or reply with `Result<GOCEvent, GOCError>`");
}

async fn process_handle() -> Result<GOCEvent, GOCError> {
    let action: GOCAction = msg::load()?;
    let contract = state();

    match action {
        GOCAction::Start {
            duration,
            participation_cost,
            ft_actor_id,
        } => contract.start(duration, participation_cost, ft_actor_id),
        GOCAction::PickWinner => contract.pick_winner().await,
        GOCAction::Enter => contract.enter().await,
    }
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
        winner,
        ..
    } = state();

    let reply = GOCState {
        admin: *admin,
        ft_actor_id: *ft_actor_id,
        started: *started,
        ending: *ending,
        players: players.clone(),
        prize_fund: *prize_fund,
        participation_cost: *participation_cost,
        winner: winner.unwrap_or_default(),
    };

    util::to_leak_ptr(reply.encode())
}

fn state() -> &'static mut Goc {
    unsafe { STATE.get_or_insert(Default::default()) }
}

metadata! {
    title: "Game of chance",
    init:
        input: GOCInit,
        output: Result<(), GOCError>,
    handle:
        input: GOCAction,
        output: Result<GOCEvent, GOCError>,
    state:
        output: GOCState,
}
