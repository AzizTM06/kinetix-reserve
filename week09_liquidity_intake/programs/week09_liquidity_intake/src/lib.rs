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
pub(crate) use instructions::register_member::{
    __client_accounts_register_member, __cpi_client_accounts_register_member,
};

declare_id!("U1Z9MA1VEJJmwLcR5LkrXpVwH9KKpKB4oAtKawCTpMW");

#[program]
pub mod week09_liquidity_intake {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_offset_initial_parity_and_dilution_guard() {
        let mut pool = ReservePool::default();
        let initial = pool.compute_receipts_for_deposit(1_000_000).unwrap();
        assert_eq!(initial, 1_000_000);

        pool.managed_assets = 1_500_000;
        pool.issued_receipts = 1_000_000;
        let second = pool.compute_receipts_for_deposit(750_000).unwrap();
        assert!(second >= 499_000 && second <= 500_500);
    }
}
