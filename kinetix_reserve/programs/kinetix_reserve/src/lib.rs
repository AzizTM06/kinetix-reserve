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

declare_id!("32SMYfjH3en7X99UydFab6gWFcRY4eLPRNNsrzLicFQ1");

#[program]
pub mod kinetix_reserve {
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
    use super::*;

    #[test]
    fn test_account_byte_layouts() {
        assert_eq!(ReservePool::SIZE, 156);
        assert_eq!(MemberLedger::SIZE, 97);
    }

    #[test]
    fn test_virtual_offset_initial_and_yield_accounting() {
        let mut pool = ReservePool {
            fee_bps: 0,
            ..Default::default()
        };

        let first = pool.compute_receipts_for_deposit(1_000_000).unwrap();
        assert_eq!(first, 1_000_000);
        pool.managed_assets += 1_000_000;
        pool.issued_receipts += first;

        // Simulate 25% CLMM fee accrual into reserve without minting receipts
        pool.managed_assets += 250_000;

        let second = pool.compute_receipts_for_deposit(500_000).unwrap();
        assert!(second < 500_000 && second > 399_000);

        let (net_first, fee) = pool.compute_assets_for_redemption(first).unwrap();
        assert_eq!(fee, 0);
        assert!(net_first > 1_249_000);
    }

    #[test]
    fn test_exit_fee_retention() {
        let pool = ReservePool {
            managed_assets: 2_000_000,
            issued_receipts: 2_000_000,
            fee_bps: 100, // 1.00%
            ..Default::default()
        };
        let (net, fee) = pool.compute_assets_for_redemption(1_000_000).unwrap();
        assert_eq!(fee, 10_000);
        assert_eq!(net, 990_000);
    }

    #[test]
    fn test_clmm_tick_alignment_and_il_bounds() {
        let spacing = 60;
        let lower = -600;
        let upper = 600;
        assert!(lower < upper);
        assert_eq!(lower % spacing, 0);
        assert_eq!(upper % spacing, 0);

        // IL(k = 2.0) = 2 * sqrt(2) / 3 - 1 ≈ -0.05719
        let k = 2.0_f64;
        let il = (2.0 * k.sqrt()) / (1.0 + k) - 1.0;
        assert!((il + 0.05719).abs() < 1e-4);
    }
}
