use anchor_lang::prelude::*;

declare_id!("9BDNVsLJfw4ShN4cMqrJJjEDWSEWEN7byMTerjBRAnWN");

pub const MAX_CALLSIGN_LEN: usize = 24;
pub const MAX_REGION_LEN: usize = 32;

#[program]
pub mod node_registry {
    use super::*;

    pub fn issue_passport(
        ctx: Context<IssuePassport>,
        callsign: String,
        region: String,
        role_tier: u8,
    ) -> Result<()> {
        require!(!callsign.is_empty() && callsign.len() <= MAX_CALLSIGN_LEN, RegistryError::InvalidCallsign);
        require!(!region.is_empty() && region.len() <= MAX_REGION_LEN, RegistryError::InvalidRegion);
        require!((1..=3).contains(&role_tier), RegistryError::InvalidTier);

        let passport = &mut ctx.accounts.passport;
        passport.owner = ctx.accounts.owner.key();
        passport.callsign = callsign;
        passport.region = region;
        passport.role_tier = role_tier;
        passport.reputation = 100;
        passport.bump = ctx.bumps.passport;

        msg!(
            "NodePassport issued: pda={}, callsign={}, tier={}",
            passport.key(),
            passport.callsign,
            passport.role_tier
        );
        Ok(())
    }

    pub fn upgrade_passport(
        ctx: Context<UpgradePassport>,
        new_region: String,
        new_tier: u8,
        reputation_delta: u32,
    ) -> Result<()> {
        require!(!new_region.is_empty() && new_region.len() <= MAX_REGION_LEN, RegistryError::InvalidRegion);
        require!((1..=3).contains(&new_tier), RegistryError::InvalidTier);

        let passport = &mut ctx.accounts.passport;
        passport.region = new_region;
        passport.role_tier = new_tier;
        passport.reputation = passport
            .reputation
            .checked_add(reputation_delta)
            .ok_or(RegistryError::ArithmeticOverflow)?;
        Ok(())
    }

    pub fn revoke_passport(ctx: Context<RevokePassport>) -> Result<()> {
        msg!(
            "NodePassport revoked and rent returned to {}",
            ctx.accounts.owner.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IssuePassport<'info> {
    #[account(
        init,
        payer = owner,
        space = NodePassport::SIZE,
        seeds = [b"node-passport", owner.key().as_ref()],
        bump
    )]
    pub passport: Account<'info, NodePassport>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpgradePassport<'info> {
    #[account(
        mut,
        seeds = [b"node-passport", owner.key().as_ref()],
        bump = passport.bump,
        has_one = owner @ RegistryError::Unauthorized
    )]
    pub passport: Account<'info, NodePassport>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokePassport<'info> {
    #[account(
        mut,
        seeds = [b"node-passport", owner.key().as_ref()],
        bump = passport.bump,
        has_one = owner @ RegistryError::Unauthorized,
        close = owner
    )]
    pub passport: Account<'info, NodePassport>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct NodePassport {
    pub owner: Pubkey,
    pub callsign: String,
    pub region: String,
    pub role_tier: u8,
    pub reputation: u32,
    pub bump: u8,
}

impl NodePassport {
    pub const SIZE: usize = 8
        + 32
        + 4 + MAX_CALLSIGN_LEN
        + 4 + MAX_REGION_LEN
        + 1
        + 4
        + 1;
}

#[error_code]
pub enum RegistryError {
    #[msg("Callsign must be 1..=24 bytes")]
    InvalidCallsign,
    #[msg("Region must be 1..=32 bytes")]
    InvalidRegion,
    #[msg("Role tier must be between 1 and 3")]
    InvalidTier,
    #[msg("Reputation counter overflow")]
    ArithmeticOverflow,
    #[msg("Signer does not own this passport PDA")]
    Unauthorized,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passport_size_and_pda_uniqueness() {
        assert_eq!(NodePassport::SIZE, 110);
        let owner_a = Pubkey::new_unique();
        let owner_b = Pubkey::new_unique();
        let (pda_a, _) = Pubkey::find_program_address(&[b"node-passport", owner_a.as_ref()], &ID);
        let (pda_b, _) = Pubkey::find_program_address(&[b"node-passport", owner_b.as_ref()], &ID);
        assert_ne!(pda_a, pda_b);
    }
}
