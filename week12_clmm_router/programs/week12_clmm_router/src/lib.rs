use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;

#[allow(unused_imports)]
pub(crate) use instructions::bootstrap_pool::{
    __client_accounts_bootstrap_pool, __cpi_client_accounts_bootstrap_pool,
};
#[allow(unused_imports)]
pub(crate) use instructions::contribute_liquidity::{
    __client_accounts_contribute_liquidity, __cpi_client_accounts_contribute_liquidity,
};
#[allow(unused_imports)]
pub(crate) use instructions::execute_rebalance_swap::{
    __client_accounts_execute_rebalance_swap, __cpi_client_accounts_execute_rebalance_swap,
};
#[allow(unused_imports)]
pub(crate) use instructions::redeem_liquidity::{
    __client_accounts_redeem_liquidity, __cpi_client_accounts_redeem_liquidity,
};
#[allow(unused_imports)]
pub(crate) use instructions::register_member::{
    __client_accounts_register_member, __cpi_client_accounts_register_member,
};
#[allow(unused_imports)]
pub(crate) use instructions::set_pool_pause::{
    __client_accounts_set_pool_pause, __cpi_client_accounts_set_pool_pause,
};

declare_id!("FTLgYnW8feoh9CsZz1s7CvBs7bL3y1Go5GXRNNZxHcsx");

#[program]
pub mod week12_clmm_router {
    use super::*;

    pub fn bootstrap_pool(ctx: Context<BootstrapPool>, fee_bps: u16) -> Result<()> {
        instructions::bootstrap_pool::handler(ctx, fee_bps)
    }

    pub fn register_member(ctx: Context<RegisterMember>) -> Result<()> {
        instructions::register_member::handler(ctx)
    }

    pub fn contribute_liquidity(ctx: Context<ContributeLiquidity>, amount: u64) -> Result<()> {
        instructions::contribute_liquidity::handler(ctx, amount)
    }

    pub fn redeem_liquidity(ctx: Context<RedeemLiquidity>, receipts_amount: u64) -> Result<()> {
        instructions::redeem_liquidity::handler(ctx, receipts_amount)
    }

    pub fn set_pool_pause(ctx: Context<SetPoolPause>, paused: bool) -> Result<()> {
        instructions::set_pool_pause::handler(ctx, paused)
    }

    pub fn execute_rebalance_swap<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ExecuteRebalanceSwap<'info>>,
        amount_in: u64,
        min_amount_out: u64,
        sqrt_price_limit_x64: u128,
    ) -> Result<()> {
        instructions::execute_rebalance_swap::handler(
            ctx,
            amount_in,
            min_amount_out,
            sqrt_price_limit_x64,
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_slippage_floor_and_price_limit() {
        let amount_in: u64 = 10_000_000;
        let slippage_bps: u64 = 75; // 0.75%
        let min_out = amount_in * (10_000 - slippage_bps) / 10_000;
        assert_eq!(min_out, 9_925_000);

        let sqrt_price_limit_x64: u128 = 18_262_276_632_972_456_100;
        assert!(sqrt_price_limit_x64 < (1u128 << 64));
    }
}
