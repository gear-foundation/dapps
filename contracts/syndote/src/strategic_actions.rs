use crate::*;

impl Game {
    // to throw rolls to go out from the prison
    // `pay_fine`: to pay fine or not if there is not double
    pub fn throw_roll(&mut self, pay_fine: bool, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player_info = match get_player_info(&self.current_player, &mut self.players, self.round)
        {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };

        // If a player is not in the jail
        if !player_info.in_jail {
            //     debug!("PENALTY: PLAYER IS NOT IN JAIL");
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
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
                reply_strategic_error();
                return;
            }
            player_info.balance -= FINE;
            player_info.in_jail = false;
        }
        player_info.round = self.round;
        msg::reply(
            GameEvent::Jail {
                in_jail: player_info.in_jail,
                position: player_info.position,
            },
            0,
        )
        .expect("Error in sending a reply `GameEvent::Jail`");
    }

    pub fn add_gear(&mut self, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player_info = match get_player_info(&self.current_player, &mut self.players, self.round)
        {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        // if player did not check his balance itself
        if player_info.balance < COST_FOR_UPGRADE {
            //  debug!("PENALTY: NOT ENOUGH BALANCE FOR UPGRADE");
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        let position = player_info.position;

        let gears = if let Some((account, gears, _, _)) = &mut self.properties[position as usize] {
            if account != &msg::source() {
                //       debug!("PENALTY: TRY TO UPGRADE NOT OWN CELL");
                player_info.penalty += 1;
                reply_strategic_error();
                return;
            }
            gears
        } else {
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        };

        // maximum amount of gear is reached
        if gears.len() == 3 {
            //        debug!("PENALTY: MAXIMUM AMOUNT OF GEARS ON CELL");
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        gears.push(Gear::Bronze);
        player_info.balance -= COST_FOR_UPGRADE;
        player_info.round = self.round;
        reply_strategic_success();
    }

    pub fn upgrade(&mut self, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player_info = match get_player_info(&self.current_player, &mut self.players, self.round)
        {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        // if player did not check his balance itself
        if player_info.balance < COST_FOR_UPGRADE {
            //       debug!("PENALTY: NOT ENOUGH BALANCE FOR UPGRADE");
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }

        let position = player_info.position;

        if let Some((account, gears, price, rent)) = &mut self.properties[position as usize] {
            if account != &msg::source() {
                player_info.penalty += 1;
                reply_strategic_error();
                return;
            }
            // if nothing to upgrade
            if gears.is_empty() {
                //        debug!("PENALTY: NOTHING TO UPGRADE");
                player_info.penalty += 1;
                reply_strategic_error();
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
            reply_strategic_error();
            return;
        };

        player_info.balance -= COST_FOR_UPGRADE;
        player_info.round = self.round;
        reply_strategic_success();
    }

    // if a cell is free, a player can buy it
    pub fn buy_cell(&mut self, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player_info = match get_player_info(&self.current_player, &mut self.players, self.round)
        {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };
        let position = player_info.position;

        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        // if a player on the field that can't be sold (for example, jail)
        if let Some((account, _, price, _)) = self.properties[position as usize].as_mut() {
            if account != &mut ActorId::zero() {
                //       debug!("PENALTY: THAT CELL IS ALREDY BOUGHT");
                player_info.penalty += 1;
                reply_strategic_error();
                return;
            }
            // if a player has not enough balance
            if player_info.balance < *price {
                player_info.penalty += 1;
                //      debug!("PENALTY: NOT ENOUGH BALANCE FOR BUYING");
                reply_strategic_error();
                return;
            }
            player_info.balance -= *price;
            *account = msg::source();
        } else {
            player_info.penalty += 1;
            //       debug!("PENALTY: THAT FIELD CAN'T BE SOLD");
            reply_strategic_error();
            return;
        };
        player_info.cells.insert(position);
        self.ownership[position as usize] = msg::source();
        player_info.round = self.round;
        reply_strategic_success();
    }

    pub fn pay_rent(&mut self, properties_for_sale: Option<Vec<u8>>) {
        self.only_player();
        let player_info = match get_player_info(&self.current_player, &mut self.players, self.round)
        {
            Ok(player_info) => player_info,
            Err(_) => {
                reply_strategic_error();
                return;
            }
        };
        if let Some(properties) = properties_for_sale {
            if sell_property(
                &self.admin,
                &mut self.ownership,
                &properties,
                &mut self.properties_in_bank,
                &self.properties,
                player_info,
            )
            .is_err()
            {
                reply_strategic_error();
                return;
            };
        }

        let position = player_info.position;
        let account = self.ownership[position as usize];

        if account == msg::source() {
            player_info.penalty += 1;
            //   debug!("PENALTY: PAY RENT TO HIMSELF");
            reply_strategic_error();
            return;
        }

        let (_, _, _, rent) = self.properties[position as usize]
            .clone()
            .unwrap_or_default();
        if rent == 0 {
            //    debug!("PENALTY: CELL WITH NO PROPERTIES");
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        };

        if player_info.balance < rent {
            //    debug!("PENALTY: NOT ENOUGH BALANCE TO PAY RENT");
            player_info.penalty += 1;
            reply_strategic_error();
            return;
        }
        player_info.balance -= rent;
        player_info.debt = 0;
        player_info.round = self.round;
        self.players.entry(account).and_modify(|player_info| {
            player_info.balance = player_info.balance.saturating_add(rent)
        });
        reply_strategic_success();
    }
}

fn reply_strategic_error() {
    msg::reply(GameEvent::StrategicError, 0).expect("Error in a reply `GameEvent::StrategicError`");
}

fn reply_strategic_success() {
    msg::reply(GameEvent::StrategicSuccess, 0)
        .expect("Error in a reply `GameEvent::StrategicSuccess`");
}
