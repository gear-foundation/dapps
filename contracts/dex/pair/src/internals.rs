use crate::*;

impl Pair {
    // INTERNAL METHODS

    // A simple wrapper for balance calculations to facilitate mint & burn.
    pub fn update_balance(&mut self, to: ActorId, amount: u128, increase: bool) {
        self.get_mut()
            .allowances
            .entry(to)
            .or_default()
            .insert(to, amount);
        if increase {
            self.get_mut()
                .balances
                .entry(to)
                .and_modify(|balance| *balance += amount)
                .or_insert(amount);
            self.get_mut().total_supply += amount;
        } else {
            self.get_mut()
                .balances
                .entry(to)
                .and_modify(|balance| *balance -= amount)
                .or_insert(amount);
            self.get_mut().total_supply -= amount;
        }
    }

    // Mints the liquidity.
    // `to` - MUST be a non-zero address
    // Arguments:
    // * `to` - is the operation performer
    pub async fn mint(&mut self, to: ActorId) -> u128 {
        let amount0 = self.balance0.saturating_sub(self.reserve0);
        let amount1 = self.balance1.saturating_sub(self.reserve1);
        let fee_on = self.mint_fee(self.reserve0, self.reserve1).await;
        let total_supply = self.get().total_supply;
        let liquidity: u128;
        if total_supply == 0 {
            liquidity = amount0
                .wrapping_mul(amount1)
                .sqrt()
                .saturating_add(MINIMUM_LIQUIDITY);
            // Lock a minimum liquidity to a zero address.
            self.update_balance(ActorId::zero(), liquidity, true);
            // FTCore::mint(self, &ActorId::zero(), liquidity);
        } else {
            liquidity = cmp::min(
                amount0
                    .wrapping_mul(total_supply)
                    .wrapping_div(self.reserve0),
                amount1
                    .wrapping_mul(total_supply)
                    .wrapping_div(self.reserve1),
            )
        }
        if liquidity == 0 {
            panic!("PAIR: Liquidity MUST be greater than 0.");
        }
        self.update_balance(to, liquidity, true);
        // FTCore::mint(self, &to, liquidity);
        self.update(self.balance0, self.balance1, self.reserve0, self.reserve1);
        if fee_on {
            // Calculate the K which is the product of reserves.
            self.k_last = self.reserve0.wrapping_mul(self.reserve1);
        }
        liquidity
    }

    // Mint liquidity if fee is on.
    // If fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k). So the math if the following.
    // Calculate the sqrt of current k using the reserves. Compare it.
    // If the current one is greater, than calculate the liquidity using the following formula:
    // liquidity = (total_supply * (root_k - last_root_k)) / (root_k * 5 + last_root_k).
    // where root_k - is the sqrt of the current product of reserves, and last_root_k - is the sqrt of the previous product.
    // Multiplication by 5 comes from the 1/6 of growrth is sqrt.
    // Arguments:
    // * `reserve0` - the available amount of token0
    // * `reserve1` - the available amount of token1
    pub async fn mint_fee(&mut self, reserve0: u128, reserve1: u128) -> bool {
        // get fee_to from factory
        let fee_to: ActorId = messages::get_fee_to(&self.factory).await;
        let fee_on = fee_to != ActorId::zero();
        if fee_on {
            if self.k_last != 0 {
                // Calculate the sqrt of current K.
                let root_k = reserve0.wrapping_mul(reserve1).sqrt();
                // Get the sqrt of previous K.
                let root_k_last = self.k_last.sqrt();
                if root_k > root_k_last {
                    let numerator = self
                        .get()
                        .total_supply
                        .wrapping_mul(root_k.saturating_sub(root_k_last));
                    // Calculate the 1/6 of a fee is the fee is turned on.
                    let denominator = root_k.wrapping_mul(5).wrapping_add(root_k_last);
                    let liquidity = numerator.wrapping_div(denominator);
                    if liquidity > 0 {
                        self.update_balance(fee_to, liquidity, true);
                        // FTCore::mint(self, &fee_to, liquidity);
                    }
                }
            }
        } else if self.k_last != 0 {
            self.k_last = 0;
        }
        fee_on
    }

    // Updates reserves and, on the first call per block, price accumulators
    // Arguments:
    // * `balance0` - token0 balance
    // * `balance1` - token1 balance
    // * `reserve0` - the available amount of token0
    // * `reserve1` - the available amount of token1
    pub fn update(&mut self, balance0: u128, balance1: u128, reserve0: u128, reserve1: u128) {
        let current_ts = (exec::block_timestamp() & 0xFFFFFFFF) as u32;
        let time_elapsed = current_ts as u128 - self.last_block_ts;
        // Update the prices if we actually update the balances later.
        if time_elapsed > 0 && reserve0 != 0 && reserve1 != 0 {
            self.price0_cl = self.price0_cl.wrapping_add(
                self.price0_cl
                    .wrapping_div(reserve0)
                    .wrapping_mul(time_elapsed),
            );
            self.price1_cl = self.price1_cl.wrapping_add(
                self.price1_cl
                    .wrapping_div(reserve1)
                    .wrapping_mul(time_elapsed),
            );
        }
        self.reserve0 = balance0;
        self.reserve1 = balance1;
        self.last_block_ts = current_ts as u128;
    }

    // Burns the liquidity.
    // `to` - MUST be a non-zero address
    // Arguments:
    // * `to` - is the operation performer
    pub async fn burn(&mut self, to: ActorId) -> (u128, u128) {
        let fee_on = self.mint_fee(self.reserve0, self.reserve1).await;
        // get liquidity

        let liquidity: u128 = *self
            .get()
            .balances
            .get(&exec::program_id())
            .expect("The pair has no liquidity at all");
        let amount0 = liquidity
            .wrapping_mul(self.balance0)
            .wrapping_div(self.get().total_supply);
        let amount1 = liquidity
            .wrapping_mul(self.balance1)
            .wrapping_div(self.get().total_supply);

        if amount0 == 0 || amount1 == 0 {
            panic!("PAIR: Insufficient liquidity burnt.");
        }
        // add this later to ft_core
        self.update_balance(to, liquidity, false);
        // FTCore::burn(self, liquidity);
        messages::transfer_tokens(&self.token0, &exec::program_id(), &to, amount0).await;
        messages::transfer_tokens(&self.token1, &exec::program_id(), &to, amount1).await;
        self.balance0 -= amount0;
        self.balance1 -= amount1;
        self.update(self.balance0, self.balance1, self.reserve0, self.reserve1);
        if fee_on {
            // If fee is on recalculate the K.
            self.k_last = self.reserve0.wrapping_mul(self.reserve1);
        }
        (amount0, amount1)
    }

    // Swaps two tokens just by calling transfer_tokens from the token contracts.
    // Also maintains the balances and updates the reservers to match the balances.
    // `amount0` - MUST be more than self.reserve0
    // `amount1` - MUST be more than self.reserve1
    // `to` - MUST be a non-zero address
    // Arguments:
    // * `amount0` - amount of token0
    // * `amount1` - amount of token1
    // * `to` - is the operation performer
    // * `forward` - is the direction. If true - user inputs token0 and gets token1, otherwise - token1 -> token0
    pub async fn _swap(&mut self, amount0: u128, amount1: u128, to: ActorId, forward: bool) {
        if amount0 > self.reserve0 && forward {
            panic!("PAIR: Insufficient liquidity.");
        }
        if amount1 > self.reserve1 && !forward {
            panic!("PAIR: Insufficient liquidity.");
        }
        // carefully, not forward
        if !forward {
            messages::transfer_tokens(&self.token0, &exec::program_id(), &to, amount0).await;
            messages::transfer_tokens(&self.token1, &to, &exec::program_id(), amount1).await;
            self.balance0 -= amount0;
            self.balance1 += amount1;
        } else {
            messages::transfer_tokens(&self.token0, &to, &exec::program_id(), amount0).await;
            messages::transfer_tokens(&self.token1, &exec::program_id(), &to, amount1).await;
            self.balance0 += amount0;
            self.balance1 -= amount1;
        }
        self.update(self.balance0, self.balance1, self.reserve0, self.reserve1);
    }
}
