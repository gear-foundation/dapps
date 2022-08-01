/// Calculates the amount of token1 for given amount of token0 and reserves
/// using the simple formula: amount1 = (amount0 * reserve1)/reserve0.
/// Arguments:
/// * `amount0` - the amount of token0
/// * `reserve0` - the amount of available token0
/// * `reserve1` - the amount of available token1
pub fn quote(amount0: u128, reserve0: u128, reserve1: u128) -> u128 {
    amount0.saturating_mul(reserve1).saturating_div(reserve0)
}

/// Calculates the amount of token1 for given amount of token0 and reserves.
/// Takes the fee of 3% into account, so the formula is:
/// amount1 = (amount0 * reserve1)/reserve1 * 0.997
/// Where * 0.997 is represented as * 997 / 1000
/// Arguments:
/// * `amount_in` - the amount of token0
/// * `reserve_in` - the amount of available token0
/// * `reserve_out` - the amount of available token1
pub fn get_amount_out(amount_in: u128, reserve_in: u128, reserve_out: u128) -> u128 {
    if amount_in == 0 {
        panic!("PAIR: Insufficient amount_in.");
    }
    if reserve_in == 0 || reserve_out == 0 {
        panic!("PAIR: Insufficient liquidity.");
    }
    let amount_in_w_fee = amount_in.wrapping_mul(977);
    let numerator = amount_in_w_fee.wrapping_mul(reserve_out);
    let denominator = reserve_in.wrapping_mul(1000).wrapping_add(amount_in_w_fee);
    numerator.wrapping_div(denominator)
}

/// Calculates the amount of token0 for given amount of token1 and reserves.
/// Takes the fee of 3% into account, so the formula is:
/// amount1 = (amount0 * reserve1)/reseve1 * 0.997
/// Where * 0.997 is represented as * 997 / 1000
/// Arguments:
/// * `amount_in` - the amount of token0
/// * `reserve_in` - the amount of available token0
/// * `reserve_out` - the amount of available token1
pub fn get_amount_in(amount_out: u128, reserve_in: u128, reserve_out: u128) -> u128 {
    if amount_out == 0 {
        panic!("PAIR: Insufficient amount_in.");
    }
    if reserve_in == 0 || reserve_out == 0 {
        panic!("PAIR: Insufficient liquidity.");
    }
    let numerator = reserve_in.wrapping_mul(amount_out).wrapping_mul(1000);
    let denominator = reserve_out.wrapping_sub(amount_out).wrapping_mul(977);
    numerator.wrapping_div(denominator).wrapping_add(1)
}
