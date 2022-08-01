export const payloads = {
    init: function(
        factory: string,
        token0: string,
        token1: string,
    ) {
        return {
            factory,
            token0,
            token1,
        }
    },
    add_liquidity: function(
        amount0_desired: number,
        amount1_desired: number,
        amount0_min: number,
        amount1_min: number,
        to: string,
    ) {
        return {
            AddLiquidity: {
                amount0_desired,
                amount1_desired,
                amount0_min,
                amount1_min,
                to,
            }
        }
    },
    remove_liquidity: function(
        liquidity: number,
        amount0_min: number,
        amount1_min: number,
        to: string,
    ) {
        return {
            RemoveLiquidity: {
                liquidity,
                amount0_min,
                amount1_min,
                to,
            }
        }
    },
    sync: function() {
        return {
            Sync: {},
        }
    },
    skim: function(
        to: string,
    ) {
        return {
            Skim: {
                to,
            }
        }
    },
    swap_exact_tokens_for: function(
        to: string,
        amount_in: number,
    ) {
        return {
            SwapExactTokensFor: {
                to,
                amount_in,
            }
        }
    },
    swap_tokens_for_exact: function(
        to: string,
        amount_out: number,
    ) {
        return {
            SwapTokensForExact: {
                to,
                amount_out,
            }
        }
    }
};
