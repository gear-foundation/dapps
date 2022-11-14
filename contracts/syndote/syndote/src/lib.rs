#![no_std]
use gstd::{exec, msg, prelude::*, ActorId, ReservationId};
pub const NUMBER_OF_CELLS: u8 = 40;
pub const NUMBER_OF_PLAYERS: u8 = 4;
pub const JAIL_POSITION: u8 = 10;
pub const LOTTERY_POSITION: u8 = 20;
pub const COST_FOR_UPGRADE: u32 = 500;
pub const FINE: u32 = 1_000;
pub const PENALTY: u8 = 5;
pub const INITIAL_BALANCE: u32 = 15_000;
pub const NEW_CIRCLE: u32 = 2_000;
pub const WAIT_DURATION: u32 = 5;
pub mod strategic_actions;
pub mod utils;
use syndote_io::*;
use utils::*;
pub mod messages;
use messages::*;
const RESERVATION_AMOUNT: u64 = 245_000_000_000;
const GAS_FOR_ROUND: u64 = 60_000_000_000;

#[derive(Clone, Default, Encode, Decode, TypeInfo)]
pub struct Game {
    admin: ActorId,
    properties_in_bank: BTreeSet<u8>,
    round: u128,
    players: BTreeMap<ActorId, PlayerInfo>,
    players_queue: Vec<ActorId>,
    current_player: ActorId,
    current_step: u64,
    // mapping from cells to built properties,
    properties: Vec<Option<(ActorId, Gears, u32, u32)>>,
    // mapping from cells to accounts who have properties on it
    ownership: Vec<ActorId>,
    game_status: GameStatus,
    winner: ActorId,
}

static mut GAME: Option<Game> = None;
static mut RESERVATION: Option<Vec<ReservationId>> = None;

impl Game {
    fn reserve_gas(&self) {
        unsafe {
            let reservation_id = ReservationId::reserve(RESERVATION_AMOUNT, 864000)
                .expect("reservation across executions");
            let reservations = RESERVATION.get_or_insert(Default::default());
            reservations.push(reservation_id);
        }
        msg::reply(GameEvent::GasReserved, 0).expect("");
    }
    fn start_registration(&mut self) {
        self.check_status(GameStatus::Finished);
        self.only_admin();
        let mut game: Game = Game {
            admin: self.admin,
            ..Default::default()
        };
        init_properties(&mut game.properties, &mut game.ownership);
        *self = game;
        msg::reply(GameEvent::StartRegistration, 0)
            .expect("Error in sending a reply `GameEvent::StartRegistration");
    }

    fn register(&mut self, player: &ActorId) {
        self.check_status(GameStatus::Registration);
        assert!(
            !self.players.contains_key(player),
            "You have already registered"
        );
        self.players.insert(
            *player,
            PlayerInfo {
                balance: INITIAL_BALANCE,
                ..Default::default()
            },
        );
        self.players_queue.push(*player);
        if self.players_queue.len() == NUMBER_OF_PLAYERS as usize {
            self.game_status = GameStatus::Play;
        }
        msg::reply(GameEvent::Registered, 0)
            .expect("Error in sending a reply `GameEvent::Registered`");
    }

    async fn play(&mut self) {
        //self.check_status(GameStatus::Play);
        assert!(
            msg::source() == self.admin || msg::source() == exec::program_id(),
            "Only admin or the program can send that message"
        );

        while self.game_status == GameStatus::Play {
            if exec::gas_available() <= GAS_FOR_ROUND {
                unsafe {
                    let reservations = RESERVATION.get_or_insert(Default::default());
                    if let Some(id) = reservations.pop() {
                        msg::send_from_reservation(id, exec::program_id(), GameAction::Play, 0)
                            .expect("Failed to send message");
                        msg::reply(GameEvent::NextRoundFromReservation, 0).expect("");

                        break;
                    } else {
                        panic!("GIVE ME MORE GAS");
                    };
                }
            }

            // check penalty and debt of the players for the previous round
            // if penalty is equal to 5 points we remove the player from the game
            // if a player has a debt and he has not enough balance to pay it
            // he is also removed from the game
            bankrupt_and_penalty(
                &self.admin,
                &mut self.players,
                &mut self.players_queue,
                &mut self.properties,
                &mut self.properties_in_bank,
                &mut self.ownership,
            );

            if self.players_queue.len() <= 1 {
                self.winner = self.players_queue[0];
                self.game_status = GameStatus::Finished;
                msg::reply(
                    GameEvent::GameFinished {
                        winner: self.winner,
                    },
                    0,
                )
                .expect("Error in sending a reply `GameEvent::GameFinished`");
                break;
            }
            self.round = self.round.wrapping_add(1);
            for player in self.players_queue.clone() {
                self.current_player = player;
                self.current_step += 1;
                // we save the state before the player's step in case
                // the player's contract does not reply or is executed with a panic.
                // Then we roll back all the changes that the player could have made.
                let mut state = self.clone();
                let player_info = self
                    .players
                    .get_mut(&player)
                    .expect("Cant be None: Get Player");

                // if a player is in jail we don't throw rolls for him
                let position = if player_info.in_jail {
                    player_info.position
                } else {
                    let (r1, r2) = get_rolls();
                    //     debug!("ROOLS {:?} {:?}", r1, r2);
                    let roll_sum = r1 + r2;
                    (player_info.position + roll_sum) % NUMBER_OF_CELLS
                };

                // If a player is on a cell that belongs to another player
                // we write down a debt on him in the amount of the rent.
                // This is done in order to penalize the participant's contract
                // if he misses the rent
                let account = self.ownership[position as usize];
                if account != player && account != ActorId::zero() {
                    if let Some((_, _, _, rent)) = self.properties[position as usize] {
                        player_info.debt = rent;
                    }
                }
                player_info.position = position;
                player_info.in_jail = position == JAIL_POSITION;
                state.players.insert(player, player_info.clone());
                match position {
                    0 => {
                        player_info.balance += NEW_CIRCLE;
                        player_info.round = self.round;
                    }
                    // free cells (it can be lottery or penalty): TODO as a task on hackathon
                    2 | 4 | 7 | 16 | 20 | 30 | 33 | 36 | 38 => {
                        player_info.round = self.round;
                    }
                    _ => {
                        let reply = take_your_turn(&player, &state).await;

                        if reply.is_err() {
                            player_info.penalty = PENALTY;
                        }
                    }
                }

                msg::send(
                    self.admin,
                    GameEvent::Step {
                        players: self.players.clone(),
                        properties: self.properties.clone(),
                        current_player: self.current_player,
                        current_step: self.current_step,
                        ownership: self.ownership.clone(),
                    },
                    0,
                )
                .expect("Error in sending a message `GameEvent::Step`");
            }
        }
    }
}

#[gstd::async_main]
async fn main() {
    let action: GameAction = msg::load().expect("Could not load `GameAction`");
    let game: &mut Game = unsafe { GAME.get_or_insert(Default::default()) };
    match action {
        GameAction::Register { player } => game.register(&player),
        GameAction::ReserveGas => game.reserve_gas(),
        GameAction::StartRegistration => game.start_registration(),
        GameAction::Play => game.play().await,
        GameAction::ThrowRoll {
            pay_fine,
            properties_for_sale,
        } => game.throw_roll(pay_fine, properties_for_sale),
        GameAction::AddGear {
            properties_for_sale,
        } => game.add_gear(properties_for_sale),
        GameAction::Upgrade {
            properties_for_sale,
        } => game.upgrade(properties_for_sale),
        GameAction::BuyCell {
            properties_for_sale,
        } => game.buy_cell(properties_for_sale),
        GameAction::PayRent {
            properties_for_sale,
        } => game.pay_rent(properties_for_sale),
    }
}

#[no_mangle]
extern "C" fn meta_state() -> *mut [i32; 2] {
    let game: &mut Game = unsafe { GAME.get_or_insert(Default::default()) };
    let encoded = game.encode();
    gstd::util::to_leak_ptr(encoded)
}

#[no_mangle]
unsafe extern "C" fn init() {
    let mut game = Game {
        admin: msg::source(),
        ..Default::default()
    };
    init_properties(&mut game.properties, &mut game.ownership);
    GAME = Some(game);
}

gstd::metadata! {
title: "Syncdote",
    handle:
        input: GameAction,
        output: GameEvent,
   state:
       output: Game,
}

// TODO: possible realization with journal handling

#[derive(Clone, Encode, Decode, TypeInfo)]
pub enum Step {
    BuyCell { cell: u8, account: ActorId },
    AddGear { cell: u8 },
    Upgrade { cell: u8 },
    Sell { cell: u8 },
}
