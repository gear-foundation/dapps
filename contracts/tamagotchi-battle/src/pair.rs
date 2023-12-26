use crate::{player::PlayerFunc, utils::generate_penalty_damage, Move, Pair, Player};
use gstd::{prelude::*, ActorId};
use tamagotchi_battle_io::Config;
pub trait PairFunc {
    fn process_round_outcome(
        &mut self,
        player_0: &mut Player,
        player_1: &mut Player,
        players_ids: &mut Vec<ActorId>,
        completed_games: &mut u8,
        config: &Config,
    ) -> (u16, u16);
}

impl PairFunc for Pair {
    fn process_round_outcome(
        &mut self,
        player_0: &mut Player,
        player_1: &mut Player,
        players_ids: &mut Vec<ActorId>,
        completed_games: &mut u8,
        config: &Config,
    ) -> (u16, u16) {
        let mut health_loss_0: u16 = 0;
        let mut health_loss_1: u16 = 0;
        let mut winner = None;
        match self.moves[..] {
            [Some(Move::Attack), Some(Move::Attack)] => {
                health_loss_1 = player_0.power / 6;
                player_1.decrease_health(health_loss_1);

                if player_1.health == 0 {
                    winner = Some(0);
                } else {
                    health_loss_0 = player_1.power / 6;
                    player_0.decrease_health(health_loss_0);
                    if player_0.health == 0 {
                        winner = Some(1);
                    }
                }
            }
            [Some(Move::Attack), Some(Move::Defence)] => {
                health_loss_1 = player_0.decrease_power(player_1.defence) / 6;
                player_1.decrease_health(health_loss_1);
                if player_1.health == 0 {
                    winner = Some(0);
                }
            }
            [Some(Move::Defence), Some(Move::Attack)] => {
                health_loss_0 = player_1.decrease_power(player_0.defence) / 6;
                player_0.decrease_health(health_loss_0);
                if player_0.health == 0 {
                    winner = Some(1);
                }
            }
            [Some(Move::Attack), None] => {
                // penalty for skipping the move
                health_loss_1 = player_0.power / 6 + generate_penalty_damage();
                player_1.decrease_health(health_loss_1);
                if player_1.health == 0 {
                    winner = Some(0);
                }
            }
            [None, Some(Move::Attack)] => {
                // penalty for skipping the move
                health_loss_0 = player_1.power / 6 + generate_penalty_damage();
                player_0.decrease_health(health_loss_0);
                if player_0.health == 0 {
                    winner = Some(1);
                }
            }
            [None, Some(Move::Defence)] => {
                // penalty for skipping the move
                health_loss_0 = generate_penalty_damage();
                player_0.decrease_health(health_loss_0);
                if player_0.health == 0 {
                    winner = Some(1);
                }
            }
            [Some(Move::Defence), None] => {
                // penalty for skipping the move
                health_loss_1 = generate_penalty_damage();
                player_1.decrease_health(health_loss_1);
                if player_1.health == 0 {
                    winner = Some(0);
                }
            }
            [None, None] => {
                health_loss_0 = generate_penalty_damage();
                health_loss_1 = generate_penalty_damage();
                player_0.decrease_health(health_loss_0);
                player_1.decrease_health(health_loss_1);
                if player_0.health == 0 {
                    winner = Some(1);
                } else if player_1.health == 0 {
                    winner = Some(0);
                }
            }
            [Some(Move::Defence), Some(Move::Defence)] => {}
            _ => unreachable!(),
        };

        if self.rounds == config.max_steps_in_round && winner.is_none() {
            winner = if player_0.health >= player_1.health {
                player_1.health = 0;
                Some(0)
            } else {
                player_0.health = 0;
                Some(1)
            };
        }

        if let Some(id) = winner {
            self.game_is_over = true;
            *completed_games += 1;
            if id == 0 {
                player_0.update_structure(config.min_power, config.max_power, true);
                self.winner = player_0.tmg_id;
                players_ids.push(player_0.tmg_id);
            } else {
                player_1.update_structure(config.min_power, config.max_power, true);
                self.winner = player_1.tmg_id;
                players_ids.push(player_1.tmg_id);
            }
        } else {
            player_0.update_structure(config.min_power, config.max_power, false);
            player_1.update_structure(config.min_power, config.max_power, false);
        }

        self.moves = Vec::new();
        self.rounds += 1;

        (health_loss_0, health_loss_1)
    }
}
