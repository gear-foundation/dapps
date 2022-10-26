#![no_std]

#[cfg(test)]
mod simple_tests;

#[cfg(test)]
mod panic_tests;

#[cfg(test)]
mod token_tests;

#[cfg(test)]
mod meta_tests;

use ft_io::*;
use gstd::{debug, exec, msg, prelude::*, ActorId};
use lt_io::*;

#[derive(Debug, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
struct Lottery {
    lottery_owner: ActorId,
    lottery_started: bool,
    lottery_start_time: u64,
    lottery_duration: u64,
    participation_cost: u128,
    prize_fund: u128,
    token_address: Option<ActorId>,
    players: BTreeMap<u32, Player>,
    lottery_history: BTreeMap<u32, ActorId>,
    lottery_id: u32,
}

static mut LOTTERY: Option<Lottery> = None;

impl Lottery {
    // checks that lottery has started and lottery time has not expired
    fn lottery_is_on(&mut self) -> bool {
        self.lottery_started
            && (self.lottery_start_time + self.lottery_duration) > exec::block_timestamp()
    }

    /// Launches a lottery
    /// Requirements:
    /// * Only owner can launch lottery
    /// * Lottery must not have been launched earlier
    /// Arguments:
    /// * `duration`: lottery duration in milliseconds
    /// * `token_address`: address of Fungible Token contract
    fn start_lottery(
        &mut self,
        duration: u64,
        token_address: Option<ActorId>,
        participation_cost: u128,
        prize_fund: u128,
    ) {
        if msg::source() != self.lottery_owner {
            panic!(
                "start_lottery(): Owner message: {}  source(): {:?}  owner: {:?}",
                msg::source() == self.lottery_owner,
                msg::source(),
                self.lottery_owner
            );
        }

        if self.lottery_is_on() {
            self.players = BTreeMap::new();
        }

        self.lottery_started = true;
        self.lottery_start_time = exec::block_timestamp();
        self.lottery_duration = duration;
        self.participation_cost = participation_cost;
        self.prize_fund = prize_fund;
        self.token_address = token_address;
        self.lottery_id = self.lottery_id.saturating_add(1);
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
    async fn transfer_tokens(
        token_address: &ActorId,
        from: &ActorId,
        to: &ActorId,
        amount_tokens: u128,
    ) {
        msg::send_for_reply(
            *token_address,
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
        if self.lottery_is_on() && !self.player_exists() && amount == self.participation_cost {
            if let Some(ref token_address) = self.token_address {
                Self::transfer_tokens(token_address, &msg::source(), &exec::program_id(), amount)
                    .await;

                debug!("Add in Fungible Token: {}", amount);
            }

            let player = Player {
                player_id: msg::source(),
                balance: amount,
            };

            let player_index = self.players.len() as u32;
            self.players.insert(player_index, player);
            msg::reply(LtEvent::PlayerAdded(player_index), 0).expect("reply: 'PlayerAdded' error");
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
        if msg::source() != self.lottery_owner {
            panic!("pick_winner(): Only owner can pick the winner");
        }

        if self.players.is_empty() {
            panic!("pick_winner(): players.is_empty()");
        }

        let index = (self.get_random_number() % (self.players.len() as u32)) as usize;
        let win_player_index = *self.players.keys().nth(index).expect("Player not found");
        let player = self.players[&win_player_index].clone();

        if let Some(ref token_address) = self.token_address {
            debug!("Transfer tokens to the winner");
            Self::transfer_tokens(
                token_address,
                &exec::program_id(),
                &player.player_id,
                self.prize_fund,
            )
            .await;
        } else {
            msg::send_bytes(player.player_id, b"Winner", self.prize_fund)
                .expect("pick_winner(): send_bytes() error!");
        }

        self.lottery_history
            .insert(self.lottery_id, player.player_id);
        msg::reply(LtEvent::Winner(win_player_index), 0).expect("reply: 'Winner' error");

        debug!(
            "Winner: {} token_address(): {:?}",
            index, self.token_address
        );
        self.lottery_started = false;
    }

    //Sending the 'LotteryState' message
    fn send_state(&mut self) {
        msg::reply(
            LtEvent::LotteryState {
                lottery_owner: self.lottery_owner,
                lottery_started: self.lottery_started,
                lottery_start_time: self.lottery_start_time,
                lottery_duration: self.lottery_duration,
                participation_cost: self.participation_cost,
                prize_fund: self.prize_fund,
                token_address: self.token_address,
                players: self.players.clone(),
                lottery_id: self.lottery_id,
            },
            0,
        )
        .expect("reply: 'LotteryState' error");
    }
}

#[gstd::async_main]
async fn main() {
    let lottery = unsafe { LOTTERY.get_or_insert(Default::default()) };
    let action: LtAction = msg::load().expect("Could not load Action");

    match action {
        LtAction::Enter(amount) => {
            lottery.enter(amount).await;
        }

        LtAction::StartLottery {
            duration,
            token_address,
            participation_cost,
            prize_fund,
        } => {
            lottery.start_lottery(duration, token_address, participation_cost, prize_fund);
        }

        LtAction::LotteryState => {
            lottery.send_state();
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
    let lottery = LOTTERY.get_or_insert(Default::default());

    let encoded = match query {
        LtState::GetPlayers => LtStateReply::Players(lottery.players.clone()).encode(),
        LtState::GetWinners => LtStateReply::Winners(lottery.lottery_history.clone()).encode(),
        LtState::LotteryState => {
            let winner: ActorId = if lottery.lottery_started {
                ActorId::zero()
            } else {
                *lottery
                    .lottery_history
                    .get(&lottery.lottery_id)
                    .unwrap_or(&ActorId::zero())
            };
            LtStateReply::LotteryState {
                lottery_owner: lottery.lottery_owner,
                lottery_started: lottery.lottery_started,
                lottery_start_time: lottery.lottery_start_time,
                lottery_duration: lottery.lottery_duration,
                participation_cost: lottery.participation_cost,
                prize_fund: lottery.prize_fund,
                token_address: lottery.token_address,
                players: lottery.players.clone(),
                lottery_id: lottery.lottery_id,
                winner,
            }
            .encode()
        }

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
