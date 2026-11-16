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

declare_id!("AHQjFFEEtV378q7PbQ2FQ1bzP2FNg8yoUCErba6VG4sk");

#[program]
pub mod week10_redemption_cpi {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_redemption_with_retained_fee() {
        let pool = ReservePool {
            managed_assets: 1_000_000,
            issued_receipts: 1_000_000,
            fee_bps: 100, // 1%
            ..Default::default()
        };
        let (net, fee) = pool.compute_assets_for_redemption(500_000).unwrap();
        assert_eq!(fee, 5_000);
        assert_eq!(net, 495_000);
    }
}
