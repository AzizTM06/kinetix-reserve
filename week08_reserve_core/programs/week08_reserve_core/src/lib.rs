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
pub(crate) use instructions::register_member::{
    __client_accounts_register_member, __cpi_client_accounts_register_member,
};

declare_id!("E4HxS1wXiHxZPDoeJpjWtk8jU3E7YaLwWbzQUhUZMQvg");

#[program]
pub mod week08_reserve_core {
    use super::*;

    pub fn bootstrap_pool(ctx: Context<BootstrapPool>, fee_bps: u16) -> Result<()> {
        instructions::bootstrap_pool::handler(ctx, fee_bps)
    }

    pub fn register_member(ctx: Context<RegisterMember>) -> Result<()> {
        instructions::register_member::handler(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_account_sizes() {
        assert_eq!(ReservePool::SIZE, 156);
        assert_eq!(MemberLedger::SIZE, 97);
    }

    #[test]
    fn test_pool_pda_determinism() {
        let mint_a = Pubkey::new_unique();
        let mint_b = Pubkey::new_unique();
        let (pda_a, _) = Pubkey::find_program_address(&[b"kinetix_pool", mint_a.as_ref()], &ID);
        let (pda_b, _) = Pubkey::find_program_address(&[b"kinetix_pool", mint_b.as_ref()], &ID);
        assert_ne!(pda_a, pda_b);
    }
}
