use anchor_lang::prelude::*;

declare_id!("5EWB2t83QvN9MMDB6fgCLatRZQBxHW1LSi1aYzSBpEu4");

#[program]
pub mod kinetix_quorum {
    use super::*;

    pub fn create_tally(
        ctx: Context<CreateTally>,
        proposal_id: u64,
        quorum_target: u64,
    ) -> Result<()> {
        require!(quorum_target > 0, QuorumError::InvalidTarget);
        let tally = &mut ctx.accounts.tally;
        tally.curator = ctx.accounts.curator.key();
        tally.proposal_id = proposal_id;
        tally.yes_votes = 0;
        tally.no_votes = 0;
        tally.quorum_target = quorum_target;
        tally.is_sealed = false;
        msg!("Proposal #{} initialized (target={})", proposal_id, quorum_target);
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, support: bool, weight: u64) -> Result<()> {
        require!(weight > 0, QuorumError::ZeroWeight);
        let tally = &mut ctx.accounts.tally;
        require!(!tally.is_sealed, QuorumError::TallySealed);

        if support {
            tally.yes_votes = tally.yes_votes.checked_add(weight).ok_or(QuorumError::Overflow)?;
        } else {
            tally.no_votes = tally.no_votes.checked_add(weight).ok_or(QuorumError::Overflow)?;
        }
        Ok(())
    }

    pub fn revoke_vote(ctx: Context<CastVote>, support: bool, weight: u64) -> Result<()> {
        require!(weight > 0, QuorumError::ZeroWeight);
        let tally = &mut ctx.accounts.tally;
        require!(!tally.is_sealed, QuorumError::TallySealed);

        if support {
            require!(tally.yes_votes >= weight, QuorumError::Underflow);
            tally.yes_votes = tally.yes_votes.checked_sub(weight).ok_or(QuorumError::Underflow)?;
        } else {
            require!(tally.no_votes >= weight, QuorumError::Underflow);
            tally.no_votes = tally.no_votes.checked_sub(weight).ok_or(QuorumError::Underflow)?;
        }
        Ok(())
    }

    pub fn seal_tally(ctx: Context<SealTally>) -> Result<()> {
        let tally = &mut ctx.accounts.tally;
        require!(!tally.is_sealed, QuorumError::TallySealed);
        tally.is_sealed = true;
        msg!(
            "Proposal #{} sealed (yes={}, no={}, quorum_met={})",
            tally.proposal_id,
            tally.yes_votes,
            tally.no_votes,
            tally.has_reached_quorum()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTally<'info> {
    #[account(
        init,
        payer = curator,
        space = ProposalTally::SIZE
    )]
    pub tally: Account<'info, ProposalTally>,

    #[account(mut)]
    pub curator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub tally: Account<'info, ProposalTally>,
}

#[derive(Accounts)]
pub struct SealTally<'info> {
    #[account(
        mut,
        has_one = curator @ QuorumError::Forbidden
    )]
    pub tally: Account<'info, ProposalTally>,

    pub curator: Signer<'info>,
}

#[account]
pub struct ProposalTally {
    pub curator: Pubkey,
    pub proposal_id: u64,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub quorum_target: u64,
    pub is_sealed: bool,
}

impl ProposalTally {
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 1;

    pub fn has_reached_quorum(&self) -> bool {
        self.yes_votes.saturating_add(self.no_votes) >= self.quorum_target
    }
}

#[error_code]
pub enum QuorumError {
    #[msg("Quorum target must be greater than zero")]
    InvalidTarget,
    #[msg("Vote weight must be positive")]
    ZeroWeight,
    #[msg("Arithmetic overflow while tallying votes")]
    Overflow,
    #[msg("Cannot subtract more votes than currently recorded")]
    Underflow,
    #[msg("Tally is already sealed and immutable")]
    TallySealed,
    #[msg("Only the proposal curator may seal the tally")]
    Forbidden,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tally_layout_and_quorum_check() {
        assert_eq!(ProposalTally::SIZE, 73);
        let tally = ProposalTally {
            curator: Pubkey::default(),
            proposal_id: 101,
            yes_votes: 60,
            no_votes: 45,
            quorum_target: 100,
            is_sealed: false,
        };
        assert!(tally.has_reached_quorum());
    }
}
