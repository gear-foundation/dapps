use crate::player::PlayerFunc;
use crate::Battle;
use collections::HashSet;
use gstd::{exec, prelude::*, ActorId};
use tamagotchi_battle_io::{BattleError, BattleState, Pair, PairId, TamagotchiId};
static mut SEED: u8 = 0;

pub fn get_random_value(range: u8) -> u8 {
    if range == 0 {
        return 0;
    }
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    random[0] % range
}

pub fn generate_power(min_range: u16, max_range: u16, tmg_id: ActorId) -> u16 {
    let random_input: [u8; 32] = tmg_id.into();
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let mut random_power = 5000;
    for i in 0..31 {
        let bytes: [u8; 2] = [random[i], random[i + 1]];
        random_power = u16::from_be_bytes(bytes) % max_range;
        if (min_range..=max_range).contains(&random_power) {
            break;
        }
    }
    random_power
}
pub fn generate_penalty_damage() -> u16 {
    let seed = unsafe { SEED };
    unsafe { SEED = SEED.wrapping_add(1) };
    let random_input: [u8; 32] = [seed; 32];
    let (random, _) = exec::random(random_input).expect("Error in getting random number");
    let bytes: [u8; 2] = [random[0], random[1]];
    u16::from_be_bytes(bytes) % 500
}

pub trait BattleUtils {
    fn check_max_participants(&self) -> Result<(), BattleError>;
    fn check_if_tmg_in_game(&self, tmg_id: &TamagotchiId) -> Result<(), BattleError>;
    fn check_min_player_amount(&self) -> Result<(), BattleError>;
    fn check_admin(&self, account: &ActorId) -> Result<(), BattleError>;
    fn check_state(&self, state: BattleState) -> Result<(), BattleError>;
    fn add_pair_id_for_player(&mut self, player: ActorId, pair_id: PairId);
    fn select_random_player(
        &mut self,
        players_len: &mut u8,
    ) -> Result<(ActorId, TamagotchiId), BattleError>;
    fn create_pair(
        &mut self,
        players_len: &mut u8,
        pair_id: PairId,
        last_updated: u64,
        move_deadline: u64,
    ) -> Result<(), BattleError>;
    fn remove_pair_id_from_player(&mut self, player: ActorId, pair_id: &PairId);
    fn remove_pair(&mut self, pair_id: &PairId, owners: Vec<ActorId>);
}

impl BattleUtils for Battle {
    fn check_max_participants(&self) -> Result<(), BattleError> {
        if self.players_ids.len() >= self.config.max_participants as usize {
            return Err(BattleError::MaxNumberWasReached);
        }
        Ok(())
    }
    fn check_if_tmg_in_game(&self, tmg_id: &TamagotchiId) -> Result<(), BattleError> {
        if self.players_ids.contains(tmg_id) {
            return Err(BattleError::TmgInGame);
        }
        Ok(())
    }

    fn check_min_player_amount(&self) -> Result<(), BattleError> {
        if self.players_ids.len() <= 1 {
            return Err(BattleError::NotEnoughPlayers);
        }
        Ok(())
    }
    fn check_admin(&self, account: &ActorId) -> Result<(), BattleError> {
        if !self.admins.contains(account) {
            return Err(BattleError::NotAdmin);
        }
        Ok(())
    }

    fn check_state(&self, state: BattleState) -> Result<(), BattleError> {
        if self.state != state {
            return Err(BattleError::WrongState);
        }
        Ok(())
    }
    fn add_pair_id_for_player(&mut self, player: ActorId, pair_id: PairId) {
        self.players_to_pairs
            .entry(player)
            .and_modify(|pair_ids| {
                pair_ids.insert(pair_id);
            })
            .or_insert_with(|| HashSet::from([pair_id]));
    }

    fn select_random_player(
        &mut self,
        players_len: &mut u8,
    ) -> Result<(ActorId, TamagotchiId), BattleError> {
        let tmg_num = get_random_value(*players_len) as usize;
        let tmg_id = self.players_ids.remove(tmg_num);

        if let Some(player) = self.players.get_mut(&tmg_id) {
            player.set_health(self.config.health);
            *players_len -= 1;
            Ok((player.owner, tmg_id))
        } else {
            Err(BattleError::PlayerDoesNotExist)
        }
    }

    fn create_pair(
        &mut self,
        players_len: &mut u8,
        pair_id: PairId,
        last_updated: u64,
        move_deadline: u64,
    ) -> Result<(), BattleError> {
        let (first_owner_id, first_tmg_id) = self.select_random_player(players_len)?;
        let (second_owner_id, second_tmg_id) = self.select_random_player(players_len)?;
        self.add_pair_id_for_player(first_owner_id, pair_id);
        self.add_pair_id_for_player(second_owner_id, pair_id);

        let pair = Pair {
            owner_ids: vec![first_owner_id, second_owner_id],
            tmg_ids: vec![first_tmg_id, second_tmg_id],
            last_updated,
            move_deadline,
            ..Default::default()
        };
        self.pairs.insert(pair_id, pair);
        Ok(())
    }

    fn remove_pair_id_from_player(&mut self, player: ActorId, pair_id: &PairId) {
        self.players_to_pairs.entry(player).and_modify(|pair_ids| {
            pair_ids.remove(pair_id);
        });
    }

    fn remove_pair(&mut self, pair_id: &PairId, owners: Vec<ActorId>) {
        self.pairs.remove(pair_id);
        self.remove_pair_id_from_player(owners[0], pair_id);
        self.remove_pair_id_from_player(owners[1], pair_id);
    }
}
