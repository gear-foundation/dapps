#![no_std]

use ft_main_io::{FTokenAction, FTokenEvent, LogicAction};
use game_of_chance_io::*;
use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::HashMap;
use rand::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro128PlusPlus;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

const MAX_NUMBER_OF_TXS: usize = 2usize.pow(16);

static mut STATE: Option<Contract> = None;

#[derive(Default, Debug)]
struct Contract {
    admin: ActorId,

    fungible_token: Option<ActorId>,
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

impl Contract {
    fn start(
        &mut self,
        duration: u64,
        participation_cost: u128,
        fungible_token: Option<ActorId>,
    ) -> Result<Event, Error> {
        if self.admin != msg::source() {
            return Err(Error::AccessRestricted);
        }

        if self.is_active {
            return Err(Error::UnexpectedGameStatus);
        }

        if matches!(fungible_token, Some(fungible_token) if fungible_token.is_zero()) {
            return Err(Error::ZeroActorId);
        }

        self.players.clear();

        self.winner = None;
        self.prize_fund = 0;
        self.started = exec::block_timestamp();
        self.ending = self.started.saturating_add(duration);
        self.participation_cost = participation_cost;
        self.fungible_token = fungible_token;
        self.is_active = true;

        // TODO: uncomment and update doc & tests after closing
        // https://github.com/gear-tech/gear/issues/1781.
        // msg::send_delayed(
        //     exec::program_id(),
        //     Action::PickWinner,
        //     0,
        //     (duration / 1000) as u32,
        // )?;

        Ok(Event::Started {
            ending: self.ending,
            participation_cost,
            fungible_token,
        })
    }

    async fn pick_winner(&mut self) -> Result<Event, Error> {
        if !self.is_active {
            return Err(Error::UnexpectedGameStatus);
        }

        let msg_source = msg::source();
        let exec_program = exec::program_id();
        let block_timestamp = exec::block_timestamp();

        if msg_source == self.admin {
            if self.ending > block_timestamp {
                return Err(Error::UnexpectedGameStatus);
            }
        } else if msg_source != exec_program {
            return Err(Error::AccessRestricted);
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

        if let Some(fungible_token) = self.fungible_token {
            self.transfer_tokens(
                fungible_token,
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

        Ok(Event::Winner(winner))
    }

    async fn transfer_tokens(
        &mut self,
        fungible_token: ActorId,
        msg_source: ActorId,
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    ) -> Result<(), Error> {
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
                    .unwrap_or_else(|| {
                        let key_value = self.txs_for_actor.first_key_value();

                        debug_assert!(key_value.is_some(), "tx cache cycle is corrupted, perhaps the `MAX_NUMBER_OF_TXS` constant is less than 2");

                        unsafe { key_value.unwrap_unchecked() }
                    });
                let (tx, actor) = (*tx, *actor);

                self.txs_for_actor.remove(&tx);
                self.actors_for_tx.remove(&actor);
            }

            self.txs_for_actor.insert(id, msg_source);
            self.actors_for_tx.insert(msg_source, id);

            id
        };

        let result = match msg::send_for_reply_as(
            fungible_token,
            FTokenAction::Message {
                transaction_id,
                payload: LogicAction::Transfer {
                    sender,
                    recipient,
                    amount,
                },
            },
            0,
        )?
        .await?
        {
            FTokenEvent::Ok => Ok(()),
            FTokenEvent::Err => Err(Error::TokenTransferFailed),
            _ => unreachable!("received an unexpected `FTokenEvent` variant"),
        };

        self.txs_for_actor.remove(&transaction_id);
        self.actors_for_tx.remove(&msg_source);

        result
    }

    async fn enter(&mut self) -> Result<Event, Error> {
        if self.ending <= exec::block_timestamp() {
            return Err(Error::UnexpectedGameStatus);
        }

        if self.players.len() == MAX_NUMBER_OF_PLAYERS {
            return Err(Error::MemoryLimitExceeded);
        }

        let msg_source = msg::source();

        if self.players.contains(&msg_source) {
            return Err(Error::AlreadyParticipating);
        }

        if let Some(fungible_token) = self.fungible_token {
            self.transfer_tokens(
                fungible_token,
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

                return Err(Error::InvalidParticipationCost);
            }
        }

        self.players.push(msg_source);
        self.prize_fund = self.prize_fund.saturating_add(self.participation_cost);

        Ok(Event::PlayerAdded(msg_source))
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

    reply(result).expect("Failed to encode or reply with `Result<(), Error>` from `init()`");

    if is_err {
        exec::exit(ActorId::zero());
    }
}

fn process_init() -> Result<(), Error> {
    let Initialize { admin } = msg::load()?;

    if admin.is_zero() {
        return Err(Error::ZeroActorId);
    }

    let contract = Contract {
        admin,
        ..Default::default()
    };

    unsafe { STATE = Some(contract) }

    Ok(())
}

#[gstd::async_main]
async fn main() {
    reply(process_handle().await).expect("failed to encode or reply from `handle()`");
}

async fn process_handle() -> Result<Event, Error> {
    let action: Action = msg::load()?;
    let contract = state_mut();

    match action {
        Action::Start {
            duration,
            participation_cost,
            fungible_token,
        } => contract.start(duration, participation_cost, fungible_token),
        Action::PickWinner => contract.pick_winner().await,
        Action::Enter => contract.enter().await,
    }
}

fn state_mut() -> &'static mut Contract {
    let state = unsafe { STATE.as_mut() };

    debug_assert!(state.is_some(), "state isn't initialized");

    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn state() {
    let Contract {
        admin,
        fungible_token,
        started,
        ending,
        players,
        prize_fund,
        participation_cost,
        winner,
        is_active,
        ..
    } = state_mut();

    let state = State {
        admin: *admin,
        fungible_token: *fungible_token,
        started: *started,
        ending: *ending,
        players: players.clone(),
        prize_fund: *prize_fund,
        participation_cost: *participation_cost,
        winner: winner.unwrap_or_default(),
        is_active: *is_active,
    };

    reply(state).expect("failed to encode or reply from `state()`");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");

    reply(metahash).expect("failed to encode or reply from `metahash()`");
}
