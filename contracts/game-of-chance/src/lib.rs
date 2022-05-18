#![no_std]

#[cfg(test)]
mod simple_tests;

#[cfg(test)]
mod panic_tests;

#[cfg(test)]
mod token_tests;

use codec::{Decode, Encode};
use ft_io::*;
use gstd::{debug, exec, msg, prelude::*, ActorId};
use lt_io::*;
use scale_info::TypeInfo;

const ZERO_ID: ActorId = ActorId::new([0u8; 32]);

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
struct Lottery {
    lottery_state: LotteryState,
    lottery_owner: ActorId,
    token_address: Option<ActorId>,
    players: BTreeMap<u32, Player>,
    lottery_history: BTreeMap<u32, ActorId>,
    lottery_id: u32,
    lottery_balance: u128,
}

impl Lottery {
    // checks that lottery has started and lottery time has not expired
    fn lottery_is_on(&mut self) -> bool {
        self.lottery_state.lottery_started
            && (self.lottery_state.lottery_start_time + self.lottery_state.lottery_duration)
                > exec::block_timestamp()
    }

    /// Launches a lottery
    /// Requirements:
    /// * Only owner can launch lottery
    /// * Lottery must not have been launched earlier
    /// Arguments:
    /// * `duration`: lottery duration in milliseconds
    /// * `token_address`: address of Fungible Token contract
    fn start_lottery(&mut self, duration: u64, token_address: Option<ActorId>) {
        if msg::source() == self.lottery_owner && !self.lottery_is_on() {
            self.lottery_state.lottery_started = true;
            self.lottery_state.lottery_start_time = exec::block_timestamp();
            self.lottery_state.lottery_duration = duration;
            self.token_address = token_address;
            self.lottery_id += 1;
            self.lottery_balance = 0;
        } else {
            panic!(
                "start_lottery(): Lottery on: {}  Owner message: {}",
                self.lottery_is_on(),
                msg::source() == self.lottery_owner
            );
        }
    }

    // checks that the player is already participating in the lottery
    fn player_exists(&mut self) -> bool {
        self.players
            .values()
            .any(|player| player.player_id == msg::source())
    }

    /// Transfers `amount` tokens from `sender` account to `recipient` account.
    /// Arguments:
    /// * `from`: sender account
    /// * `to`: recipient account
    /// * `amount`: amount of tokens
    async fn transfer_tokens(&mut self, from: &ActorId, to: &ActorId, amount_tokens: u128) {
        let _transfer_response: FTEvent = msg::send_and_wait_for_reply(
            self.token_address.unwrap(),
            FTAction::Transfer {
                from: *from,
                to: *to,
                amount: amount_tokens,
            },
            0,
        )
        .expect("Error in sending message")
        .await
        .expect("Error in transfer");
    }

    /// Called by a player in order to participate in lottery
    /// Requirements:
    /// * Lottery must be on
    /// * Contribution must be greater than zero
    /// * The player cannot enter the lottery more than once
    /// Arguments:
    /// * `amount`: value or tokens to participate in the lottery
    async fn enter(&mut self, amount: u128) {
        if self.lottery_is_on() && !self.player_exists() && amount > 0 {
            let player = Player {
                player_id: msg::source(),
                balance: amount,
            };

            if self.token_address.is_some() {
                self.transfer_tokens(&msg::source(), &exec::program_id(), amount)
                    .await;

                self.lottery_balance += amount;
                debug!("Add in Fungible Token: {}", amount);
            }

            let player_index = self.players.len() as u32;
            self.players.insert(player_index, player);
            msg::reply(LtEvent::PlayerAdded(player_index), 0).unwrap();
        } else {
            panic!(
                "enter(): Lottery on: {}  player exists: {} amount: {}",
                self.lottery_is_on(),
                self.player_exists(),
                amount
            );
        }
    }

    // Random number generation from block timestamp
    fn get_random_number(&mut self) -> u32 {
        let timestamp: u64 = exec::block_timestamp();
        let code_hash = sp_core_hashing::blake2_256(&timestamp.to_le_bytes());
        u32::from_le_bytes([code_hash[0], code_hash[1], code_hash[2], code_hash[3]])
    }

    /// Lottery winner calculation
    /// Requirements:
    /// * Only owner can pick the winner
    /// * Lottery has started and lottery time is expired
    /// * List of players must not be empty
    async fn pick_winner(&mut self) {
        if msg::source() == self.lottery_owner && !self.players.is_empty() {
            let index = (self.get_random_number() % (self.players.len() as u32)) as usize;
            let win_player_index = *self.players.keys().nth(index).expect("Player not found");
            let player = self.players[&win_player_index];

            if self.token_address.is_some() {
                debug!("Transfer tokens to the winner");
                self.transfer_tokens(&exec::program_id(), &player.player_id, self.lottery_balance)
                    .await;

                self.lottery_balance = 0;
            } else {
                msg::send_bytes(player.player_id, b"Winner", exec::value_available()).unwrap();
            }

            self.lottery_history
                .insert(self.lottery_id, player.player_id);
            msg::reply(LtEvent::Winner(win_player_index), 0).unwrap();

            debug!(
                "Winner: {} token_address(): {:?}",
                index, self.token_address
            );

            self.token_address = None;
            self.lottery_state = LotteryState::default();
            self.players = BTreeMap::new();
        } else {
            panic!(
                "pick_winner(): Owner message: {}  lottery_duration: {}  players.is_empty(): {}",
                msg::source() == self.lottery_owner,
                self.lottery_state.lottery_start_time + self.lottery_state.lottery_duration
                    > exec::block_timestamp(),
                self.players.is_empty()
            );
        }
    }
}

static mut LOTTERY: Option<Lottery> = None;

#[gstd::async_main]
async fn main() {
    if msg::source() == ZERO_ID {
        panic!("Message from zero address");
    }

    let action: LtAction = msg::load().expect("Could not load Action");
    let lottery: &mut Lottery = unsafe { LOTTERY.get_or_insert(Lottery::default()) };

    match action {
        LtAction::Enter(amount) => {
            lottery.enter(amount).await;
        }

        LtAction::StartLottery {
            duration,
            token_address,
        } => {
            lottery.start_lottery(duration, token_address);
        }

        LtAction::LotteryState => {
            msg::reply(LtEvent::LotteryState(lottery.lottery_state.clone()), 0).unwrap();
            debug!("LotteryState: {:?}", lottery.lottery_state);
        }

        LtAction::PickWinner => {
            lottery.pick_winner().await;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let lottery = Lottery {
        lottery_owner: msg::source(),
        ..Default::default()
    };

    LOTTERY = Some(lottery);
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: LtState = msg::load().expect("failed to decode input argument");
    let lottery: &mut Lottery = LOTTERY.get_or_insert(Lottery::default());

    let encoded = match query {
        LtState::GetPlayers => LtStateReply::Players(lottery.players.clone()).encode(),
        LtState::GetWinners => LtStateReply::Winners(lottery.lottery_history.clone()).encode(),
        LtState::LotteryState => LtStateReply::LotteryState(lottery.lottery_state.clone()).encode(),

        LtState::BalanceOf(index) => {
            if let Some(player) = lottery.players.get(&index) {
                LtStateReply::Balance(player.balance).encode()
            } else {
                LtStateReply::Balance(0).encode()
            }
        }
    };

    gstd::util::to_leak_ptr(encoded)
}

gstd::metadata! {
    title: "Lottery",
    handle:
        input: LtAction,
        output: LtEvent,
    state:
        input: LtState,
        output: LtStateReply,
}
