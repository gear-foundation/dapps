use crate::*;
impl Game {
    pub fn check_status(&self, game_status: GameStatus) {
        assert_eq!(self.game_status, game_status, "Wrong game status");
    }

    pub fn only_admin(&self) {
        assert_eq!(msg::source(), self.admin, "Only admin can start the game");
    }
    pub fn only_player(&self) {
        assert!(
            self.players.contains_key(&msg::source()),
            "You are not in the game"
        );
    }
}

pub fn get_player_info<'a>(
    player: &'a ActorId,
    players: &'a mut HashMap<ActorId, PlayerInfo>,
    current_round: u128,
) -> Result<&'a mut PlayerInfo, GameError> {
    if &msg::source() != player {
        //        debug!("PENALTY: WRONG MSG::SOURCE()");
        players.entry(msg::source()).and_modify(|player_info| {
            player_info.penalty += 1;
        });
        return Err(GameError::StrategicError);
    }
    let player_info = players.get_mut(player).expect("Cant be None: Get Player");
    if player_info.round >= current_round {
        //   debug!("PENALTY: MOVE ALREADY MADE");
        player_info.penalty += 1;
        return Err(GameError::StrategicError);
    }
    Ok(player_info)
}

pub fn sell_property(
    admin: &ActorId,
    ownership: &mut [ActorId],
    properties_for_sale: &Vec<u8>,
    properties_in_bank: &mut HashSet<u8>,
    properties: &[Option<(ActorId, Gears, u32, u32)>],
    player_info: &mut PlayerInfo,
) -> Result<(), GameError> {
    for property in properties_for_sale {
        if ownership[*property as usize] != msg::source() {
            //       debug!("PENALTY: TRY TO SELL NOT OWN PROPERTY");
            player_info.penalty += 1;
            return Err(GameError::StrategicError);
        }
    }

    for property in properties_for_sale {
        if let Some((_, _, price, _)) = properties[*property as usize] {
            player_info.cells.remove(property);
            player_info.balance += price / 2;
            ownership[*property as usize] = *admin;
            properties_in_bank.insert(*property);
        }
    }
    Ok(())
}

static mut SEED: u8 = 0;
pub fn get_rolls() -> (u8, u8) {
    let seed = unsafe {
        SEED = SEED.wrapping_add(1);
        SEED
    };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let r1: u8 = random[0] % 6 + 1;
    let r2: u8 = random[1] % 6 + 1;
    (r1, r2)
}

pub fn bankrupt_and_penalty(
    admin: &ActorId,
    players: &mut HashMap<ActorId, PlayerInfo>,
    players_queue: &mut Vec<ActorId>,
    properties: &mut [Option<(ActorId, Gears, Price, Rent)>],
    properties_in_bank: &mut HashSet<u8>,
    ownership: &mut [ActorId],
) {
    for (player, mut player_info) in players.clone() {
        if player_info.debt > 0 {
            for cell in &player_info.cells.clone() {
                if player_info.balance >= player_info.debt {
                    player_info.balance -= player_info.debt;
                    player_info.debt = 0;
                    player_info.penalty += 1;
                    players.insert(player, player_info);
                    break;
                }
                if let Some((_, _, price, _)) = &properties[*cell as usize] {
                    player_info.balance += price / 2;
                    player_info.cells.remove(cell);
                    ownership[*cell as usize] = *admin;
                    properties_in_bank.insert(*cell);
                }
            }
        }
    }

    for (player, mut player_info) in players.clone() {
        if (player_info.penalty >= PENALTY || player_info.debt > 0) && players_queue.len() > 1 {
            player_info.lost = true;
            player_info.balance = 0;
            players_queue.retain(|&p| p != player);
            for cell in &player_info.cells.clone() {
                ownership[*cell as usize] = *admin;
                properties_in_bank.insert(*cell);
            }
            players.insert(player, player_info);
        }
    }
}

pub fn init_properties(
    properties: &mut Vec<Option<(ActorId, Gears, Price, Rent)>>,
    ownership: &mut Vec<ActorId>,
) {
    // 0
    properties.push(None);
    // 1
    properties.push(Some((ActorId::zero(), Vec::new(), 1_000, 100)));
    // 2
    properties.push(None);
    // 3
    properties.push(Some((ActorId::zero(), Vec::new(), 1_050, 105)));
    // 4
    properties.push(None);
    // 5
    properties.push(Some((ActorId::zero(), Vec::new(), 1_100, 110)));
    // 6
    properties.push(Some((ActorId::zero(), Vec::new(), 1_500, 150)));
    // 7
    properties.push(None);
    // 8
    properties.push(Some((ActorId::zero(), Vec::new(), 1_550, 155)));
    // 9
    properties.push(Some((ActorId::zero(), Vec::new(), 1_700, 170)));

    // 10
    properties.push(None);
    // 11
    properties.push(Some((ActorId::zero(), Vec::new(), 2_000, 200)));
    // 12
    properties.push(Some((ActorId::zero(), Vec::new(), 2_050, 205)));
    // 13
    properties.push(Some((ActorId::zero(), Vec::new(), 2_100, 210)));
    // 14
    properties.push(Some((ActorId::zero(), Vec::new(), 2_200, 220)));
    // 15
    properties.push(Some((ActorId::zero(), Vec::new(), 2_300, 230)));
    // 16
    properties.push(None);
    // 17
    properties.push(Some((ActorId::zero(), Vec::new(), 2_400, 240)));
    // 18
    properties.push(Some((ActorId::zero(), Vec::new(), 2_450, 245)));
    // 19
    properties.push(Some((ActorId::zero(), Vec::new(), 2_500, 250)));

    // 20
    properties.push(None);
    // 21
    properties.push(Some((ActorId::zero(), Vec::new(), 3_000, 300)));
    // 22
    properties.push(None);
    // 23
    properties.push(Some((ActorId::zero(), Vec::new(), 3_100, 310)));
    // 24
    properties.push(Some((ActorId::zero(), Vec::new(), 3_150, 315)));
    // 25
    properties.push(Some((ActorId::zero(), Vec::new(), 3_200, 320)));
    // 26
    properties.push(Some((ActorId::zero(), Vec::new(), 3_250, 325)));
    // 27
    properties.push(Some((ActorId::zero(), Vec::new(), 3_300, 330)));
    // 28
    properties.push(Some((ActorId::zero(), Vec::new(), 3_350, 334)));
    // 29
    properties.push(Some((ActorId::zero(), Vec::new(), 3_400, 340)));

    // 30
    properties.push(None);
    // 31
    properties.push(Some((ActorId::zero(), Vec::new(), 4_000, 400)));
    // 32
    properties.push(Some((ActorId::zero(), Vec::new(), 4_050, 405)));
    // 33
    properties.push(None);
    // 34
    properties.push(Some((ActorId::zero(), Vec::new(), 4_100, 410)));
    // 35
    properties.push(Some((ActorId::zero(), Vec::new(), 4_150, 415)));
    // 36
    properties.push(None);
    // 37
    properties.push(Some((ActorId::zero(), Vec::new(), 4_200, 420)));
    // 38
    properties.push(None);
    // 39
    properties.push(Some((ActorId::zero(), Vec::new(), 4_500, 450)));

    for _i in 0..40 {
        ownership.push(ActorId::zero());
    }
}

pub enum GameError {
    StrategicError,
}

impl From<&Game> for GameState {
    fn from(game: &Game) -> GameState {
        GameState {
            admin: game.admin,
            properties_in_bank: game.properties_in_bank.iter().copied().collect(),
            round: game.round,
            players: game
                .players
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
            players_queue: game.players_queue.clone(),
            current_player: game.current_player,
            current_step: game.current_step,
            properties: game.properties.clone(),
            ownership: game.ownership.clone(),
            game_status: game.game_status.clone(),
            winner: game.winner,
        }
    }
}
