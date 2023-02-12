#![no_std]

use battle_io::*;
use core::array;
use gstd::{exec, msg, prelude::*, ActorId};
use tmg_io::{TmgAction, TmgEvent};
const MAX_POWER: u16 = 10_000;
const MIN_POWER: u16 = 3_000;
const HEALTH: u16 = 2_500;
const MAX_PARTICIPANTS: u8 = 50;
const MAX_STEPS_IN_ROUND: u8 = 5;
const COLORS: [&str; 6] = ["Green", "Red", "Blue", "Purple", "Orange", "Yellow"];
#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Battle {
    admin: ActorId,
    players: BTreeMap<ActorId, Player>,
    players_ids: Vec<ActorId>,
    state: BattleState,
    current_winner: ActorId,
    round: Round,
}

static mut BATTLE: Option<Battle> = None;
impl Battle {
    async fn register(&mut self, tmg_id: &TamagotchiId) {
        assert_eq!(
            self.state,
            BattleState::Registration,
            "The game is not in Registration stage"
        );

        assert!(
            self.players_ids.len() < MAX_PARTICIPANTS as usize,
            "Maximum number of players was reached"
        );

        if self.players_ids.contains(tmg_id) {
            panic!("This tamagotchi is already in game!");
        }

        let (owner, name, date_of_birth) = get_tmg_info(tmg_id).await;

        // if owner != msg::source() {
        //     panic!("It is not your Tamagotchi");
        // }

        let power = generate_power(*tmg_id);
        let defence = MAX_POWER - power;
        let color_index = get_random_value(COLORS.len() as u8);
        let player = Player {
            owner,
            name,
            date_of_birth,
            tmg_id: *tmg_id,
            defence,
            power,
            health: HEALTH,
            color: COLORS[color_index as usize].to_string(),
        };
        self.players.insert(*tmg_id, player);
        self.players_ids.push(*tmg_id);
        if self.players.len() as u8 == MAX_PARTICIPANTS {}
        msg::reply(BattleEvent::Registered { tmg_id: *tmg_id }, 0)
            .expect("Error during a reply `BattleEvent::Registered");
    }

    fn start_battle(&mut self) {
        assert_eq!(
            self.state,
            BattleState::Registration,
            "The game is not in Registration stage"
        );

        if self.admin != msg::source() {
            panic!("Only admin can start the battle");
        }
        let players_len = self.players.len() as u8;
        let first_tmg_id = get_random_value(players_len);
        let first_tmg = self.players_ids.remove(first_tmg_id as usize);
        let first_player = self
            .players
            .get(&first_tmg)
            .expect("Can't be None: Tmg does not exsit")
            .owner;

        let second_tmg_id = get_random_value(players_len - 1);
        let second_tmg = self.players_ids.remove(second_tmg_id as usize);
        let second_player = self
            .players
            .get(&second_tmg)
            .expect("Can't be None: Tmg does not exsit")
            .owner;
        self.round = Round {
            players: vec![first_player, second_player],
            tmg_ids: vec![first_tmg, second_tmg],
            moves: Vec::new(),
            steps: 0,
        };
        self.state = BattleState::GameIsOn;
        msg::reply(BattleEvent::GameStarted, 0)
            .expect("Error in a reply `BattleEvent::GameStarted`");
    }

    fn make_move(&mut self, tmg_move: Move) {
        assert_eq!(
            self.state,
            BattleState::GameIsOn,
            "The game is not in `GameIsOn` state"
        );
        let current_turn = self.round.moves.len();
        let player = self.round.players[current_turn];
        assert_eq!(player, msg::source(), "It is not your turn!");

        self.round.moves.push(tmg_move);

        if self.round.moves.len() == 2 {
            self.resolve_battle();
        }
        msg::reply(BattleEvent::MoveMade, 0)
            .expect("Error in sending a reply `BattleEvent::MoveMade`");
    }

    fn resolve_battle(&mut self) {
        let mut player_0 = self
            .players
            .get(&self.round.tmg_ids[0])
            .expect("Player does not exist")
            .clone();
        let mut player_1 = self
            .players
            .get(&self.round.tmg_ids[1])
            .expect("Player does not exist")
            .clone();
        let mut health_loss_0: u16 = 0;
        let mut health_loss_1: u16 = 0;
        let (loss_0, loss_1) = match self.round.moves[..] {
            [Move::Attack, Move::Attack] => {
                health_loss_1 =
                    player_1.health - player_1.health.saturating_sub(player_0.power / 6);
                player_1.health = player_1.health.saturating_sub(player_0.power / 6);

                if player_1.health == 0 {
                    self.current_winner = self.round.tmg_ids[0];
                    self.state = BattleState::WaitNextRound;
                } else {
                    health_loss_0 =
                        player_0.health - player_0.health.saturating_sub(player_1.power / 6);
                    player_0.health = player_0.health.saturating_sub(player_1.power / 6);
                    if player_0.health == 0 {
                        self.current_winner = self.round.tmg_ids[1];
                        self.state = BattleState::WaitNextRound;
                    }
                }
                (health_loss_0, health_loss_1)
            }
            [Move::Attack, Move::Defence] => {
                let player_0_power = player_0.power.saturating_sub(player_1.defence) / 6;
                health_loss_1 = player_1
                    .health
                    .saturating_sub(player_1.health.saturating_sub(player_0_power));
                player_1.health = player_1.health.saturating_sub(player_0_power);
                if player_1.health == 0 {
                    self.current_winner = self.round.tmg_ids[0];
                    self.state = BattleState::WaitNextRound;
                }
                (health_loss_0, health_loss_1)
            }
            [Move::Defence, Move::Attack] => {
                let player_1_power = player_1.power.saturating_sub(player_0.defence) / 6;
                health_loss_0 = player_0
                    .health
                    .saturating_sub(player_0.health.saturating_sub(player_1_power));
                player_0.health = player_0.health.saturating_sub(player_1_power);
                if player_0.health == 0 {
                    self.current_winner = self.round.tmg_ids[1];
                    self.state = BattleState::WaitNextRound;
                }
                (health_loss_0, health_loss_1)
            }
            [Move::Defence, Move::Defence] => (health_loss_0, health_loss_1),
            _ => unreachable!(),
        };
        self.round.steps += 1;
        if self.round.steps == MAX_STEPS_IN_ROUND && self.state == BattleState::GameIsOn {
            self.current_winner = if player_0.health >= player_1.health {
                self.round.tmg_ids[0]
            } else {
                self.round.tmg_ids[1]
            };
            self.state = BattleState::WaitNextRound;
        }
        player_0.power = generate_power(self.round.tmg_ids[0]);
        player_0.defence = MAX_POWER - player_0.power;
        player_1.power = generate_power(self.round.tmg_ids[1]);
        player_1.defence = MAX_POWER - player_1.power;

        self.players.insert(self.round.tmg_ids[0], player_0);
        self.players.insert(self.round.tmg_ids[1], player_1);
        self.round.moves = Vec::new();
        msg::send(self.admin, BattleEvent::RoundResult((loss_0, loss_1)), 0)
            .expect("Error in sending a message `TmgEvent::RoundResult`");
        if self.players_ids.len() == 0 && self.state == BattleState::WaitNextRound {
            self.state = BattleState::GameIsOver;
        }
    }

    fn start_new_round(&mut self) {
        assert_eq!(
            self.state,
            BattleState::WaitNextRound,
            "The game is not in `WaitNextRound` state"
        );

        let players_len = self.players_ids.len() as u8;
        let first_tmg = self.current_winner;
        let first_player = self.players.get(&first_tmg).expect("Can't be None").owner;
        self.players
            .entry(first_tmg)
            .and_modify(|player| player.health = 2500);

        let second_tmg_id = get_random_value(players_len);
        let second_tmg = self.players_ids.remove(second_tmg_id as usize);
        let second_player = self
            .players
            .get(&second_tmg)
            .expect("Can't be None: Tmg does not exsit")
            .owner;
        self.round = Round {
            players: vec![first_player, second_player],
            tmg_ids: vec![first_tmg, second_tmg],
            moves: Vec::new(),
            steps: 0,
        };
        self.state = BattleState::GameIsOn;
        msg::reply(BattleEvent::NewRound, 0).expect("Error in a reply `BattleEvent::NewRound`");
    }

    fn start_new_game(&mut self) {
        assert_eq!(
            self.state,
            BattleState::GameIsOver,
            "The previous game must be over"
        );
        self.current_winner = ActorId::zero();
        self.players = BTreeMap::new();
        self.round = Default::default();
        self.state = BattleState::Registration;
        msg::reply(BattleEvent::NewGame, 0).expect("Error during a reply `BattleEvent::NewGame");
    }

    fn start_new_game_force(&mut self) {
        assert_eq!(
            self.admin,
            msg::source(),
            "Only admin can force start a new game"
        );
        self.current_winner = ActorId::zero();
        self.players = BTreeMap::new();
        self.players_ids = Vec::new();
        self.round = Default::default();
        self.state = BattleState::Registration;
        msg::reply(BattleEvent::NewGame, 0).expect("Error during a reply `BattleEvent::NewGame");
    }

    fn update_admin(&mut self, new_admin: &ActorId) {
        assert_eq!(
            self.admin,
            msg::source(),
            "Only admin can update the contract admin"
        );
        self.admin = *new_admin;
        msg::reply(BattleEvent::AdminUpdated, 0)
            .expect("Error during a reply `BattleEvent::AdminUpdated");
    }
}

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load().expect("Unable to decode `BattleAction`");
    let battle = unsafe { BATTLE.get_or_insert(Default::default()) };
    match action {
        BattleAction::Register { tmg_id } => battle.register(&tmg_id).await,
        BattleAction::MakeMove(tmg_move) => battle.make_move(tmg_move),
        BattleAction::StartNewGame => battle.start_new_game(),
        BattleAction::StartNewRound => battle.start_new_round(),
        BattleAction::StartBattle => battle.start_battle(),
        BattleAction::StartNewGameForce => battle.start_new_game_force(),
        BattleAction::UpdateAdmin(new_admin) => battle.update_admin(&new_admin),
    }
}

#[no_mangle]
unsafe extern "C" fn init() {
    let battle = Battle {
        admin: msg::source(),
        ..Default::default()
    };
    BATTLE = Some(battle);
}

pub async fn get_tmg_info(tmg_id: &ActorId) -> (ActorId, String, u64) {
    let reply: TmgEvent = msg::send_for_reply_as(*tmg_id, TmgAction::TmgInfo, 0)
        .expect("Error in sending a message `TmgAction::TmgInfo")
        .await
        .expect("Unable to decode TmgEvent");
    if let TmgEvent::TmgInfo {
        owner,
        name,
        date_of_birth,
    } = reply
    {
        (owner, name, date_of_birth)
    } else {
        panic!("Wrong received message");
    }
}

pub fn get_random_value(range: u8) -> u8 {
    let random_input: [u8; 32] = array::from_fn(|i| i as u8 + 1);
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}


pub fn generate_power(tmg_id: ActorId) -> u16 {
    let random_input: [u8; 32] = tmg_id.into();
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let bytes: [u8; 2] = [random[0], random[1]];
    let random_power: u16 = u16::from_be_bytes(bytes) % MAX_POWER;
    if random_power < MIN_POWER {
        return MAX_POWER / 2;
    }
    random_power
}

#[no_mangle]
extern "C" fn state() {
    let battle = unsafe { BATTLE.get_or_insert(Default::default()) };
    msg::reply(battle, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}
