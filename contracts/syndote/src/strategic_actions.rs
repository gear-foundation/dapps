use crate::game::*;
use crate::*;

pub trait StrategicActions {
    fn throw_roll(&mut self, pay_fine: bool, properties_for_sale: Option<Vec<u8>>);
    fn add_gear(&mut self, properties_for_sale: Option<Vec<u8>>);
    fn upgrade(&mut self, properties_for_sale: Option<Vec<u8>>);
    fn buy_cell(&mut self, properties_for_sale: Option<Vec<u8>>);
    fn pay_rent(&mut self, properties_for_sale: Option<Vec<u8>>);
}
impl StrategicActions for Game {
    // to throw rolls to go out from the prison
    // `pay_fine`: to pay fine or not if there is not double
    fn throw_roll(&mut self, pay_fine: bool, properties_for_sale: Option<Vec<u8>>) {
        let player = self.current_player;
        let player_info = self.players.get_mut(&player).expect("Can't be None");

        // If a player is not in the jail
        if !player_info.in_jail {
            player_info.penalty += 1;
            return;
        }

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin_id,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                player_info.penalty += 1;
                return;
            };
        }

        let (r1, r2) = get_rolls();
        if r1 == r2 {
            player_info.in_jail = false;
            player_info.position = r1 + r2;
        } else if pay_fine {
            if player_info.balance < FINE {
                player_info.penalty += 1;
                return;
            }
            player_info.balance -= FINE;
            player_info.in_jail = false;
        }
        player_info.round = self.round;
    }

    fn add_gear(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let player = self.current_player;
        let player_info = self.players.get_mut(&player).expect("Can't be None");

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin_id,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                return;
            };
        }

        // if player did not check his balance itself
        if player_info.balance < COST_FOR_UPGRADE {
            player_info.penalty += 1;
            return;
        }

        let position = player_info.position;

        let gears = if let Some((account, gears, _, _)) = &mut self.properties[position as usize] {
            if account != &player {
                player_info.penalty += 1;
                return;
            }
            gears
        } else {
            player_info.penalty += 1;
            return;
        };

        // maximum amount of gear is reached
        if gears.len() == 3 {
            player_info.penalty += 1;
            return;
        }

        gears.push(Gear::Bronze);
        player_info.balance -= COST_FOR_UPGRADE;
        player_info.round = self.round;
    }

    fn upgrade(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let player = self.current_player;
        let player_info = self.players.get_mut(&player).expect("Can't be None");

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin_id,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                return;
            };
        }

        // if player did not check his balance itself
        if player_info.balance < COST_FOR_UPGRADE {
            player_info.penalty += 1;
            return;
        }

        let position = player_info.position;

        if let Some((account, gears, price, rent)) = &mut self.properties[position as usize] {
            if account != &player {
                player_info.penalty += 1;
                return;
            }
            // if nothing to upgrade
            if gears.is_empty() {
                player_info.penalty += 1;
                return;
            }
            for gear in gears {
                if *gear != Gear::Gold {
                    *gear = gear.upgrade();
                    // add 10 percent to the price of cell
                    *price += *price / 10;
                    // add 10 percent to the price of rent
                    *rent += *rent / 10;
                    break;
                }
            }
        } else {
            player_info.penalty += 1;
            return;
        };

        player_info.balance -= COST_FOR_UPGRADE;
        player_info.round = self.round;
    }

    // if a cell is free, a player can buy it
    fn buy_cell(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let player = self.current_player;
        let player_info = self.players.get_mut(&player).expect("Can't be None");

        let position = player_info.position;

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin_id,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                player_info.penalty += 1;
                return;
            };
        }

        // if a player on the field that can't be sold (for example, jail)
        if let Some((account, _, price, _)) = self.properties[position as usize].as_mut() {
            if !account.is_zero() {
                player_info.penalty += 1;
                return;
            }
            // if a player has not enough balance
            if player_info.balance < *price {
                player_info.penalty += 1;
                return;
            }
            player_info.balance -= *price;
            *account = msg::source();
        } else {
            player_info.penalty += 1;
            return;
        };
        player_info.cells.insert(position);
        self.ownership[position as usize] = player;
        player_info.round = self.round;
    }

    fn pay_rent(&mut self, properties_for_sale: Option<Vec<u8>>) {
        let player = self.current_player;
        let player_info = self.players.get_mut(&player).expect("Can't be None");
        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin_id,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                return;
            };
        }

        let position = player_info.position;
        let account = self.ownership[position as usize];

        if account == player {
            player_info.penalty += 1;
            return;
        }

        let rent = if let Some((_, _, _, rent)) = self.properties[position as usize] {
            rent
        } else {
            0
        };
        if rent == 0 {
            player_info.penalty += 1;
            return;
        };

        if player_info.balance < rent {
            player_info.penalty += 1;
            return;
        }
        player_info.balance -= rent;
        player_info.debt = 0;
        player_info.round = self.round;
        self.players.entry(account).and_modify(|player_info| {
            player_info.balance = player_info.balance.saturating_add(rent)
        });
    }
}
