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
pub(crate) use instructions::deploy_range_position::{
    __client_accounts_deploy_range_position, __cpi_client_accounts_deploy_range_position,
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
#[allow(unused_imports)]
pub(crate) use instructions::unwind_range_position::{
    __client_accounts_unwind_range_position, __cpi_client_accounts_unwind_range_position,
};

declare_id!("Apa5ExppBoUYXAjtk7xsUQbx4SW86TF9oCVScnhHWYhj");

#[program]
pub mod week13_clmm_positions {
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

    pub fn deploy_range_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DeployRangePosition<'info>>,
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    ) -> Result<()> {
        instructions::deploy_range_position::handler(
            ctx,
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
        )
    }

    pub fn unwind_range_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UnwindRangePosition<'info>>,
        amount_0_min: u64,
        amount_1_min: u64,
    ) -> Result<()> {
        instructions::unwind_range_position::handler(ctx, amount_0_min, amount_1_min)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_impermanent_loss_and_fee_breakeven() {
        let ratio = 1.5_f64;
        let il = (2.0 * ratio.sqrt()) / (1.0 + ratio) - 1.0;
        // At +50% price shift, AMM v2 IL is approx -2.0204%
        assert!((il + 0.020204).abs() < 1e-4);

        let accrued_fees_pct = 0.038_f64; // 3.8% from concentrated range volume
        let net_pnl = accrued_fees_pct + il;
        assert!(net_pnl > 0.017);
    }
}
