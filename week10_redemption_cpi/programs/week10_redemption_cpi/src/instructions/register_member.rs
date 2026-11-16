use anchor_lang::prelude::*;

use crate::state::{MemberLedger, ReservePool};

#[derive(Accounts)]
pub struct RegisterMember<'info> {
    #[account(
        seeds = [b"kinetix_pool", pool.base_mint.as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, ReservePool>,

    #[account(
        init,
        payer = participant,
        space = MemberLedger::SIZE,
        seeds = [b"member_ledger", pool.key().as_ref(), participant.key().as_ref()],
        bump,
    )]
    pub ledger: Account<'info, MemberLedger>,

    #[account(mut)]
    pub participant: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterMember>) -> Result<()> {
    let clock = Clock::get()?;
    let ledger = &mut ctx.accounts.ledger;
    ledger.pool = ctx.accounts.pool.key();
    ledger.participant = ctx.accounts.participant.key();
    ledger.net_contributed = 0;
    ledger.receipt_balance = 0;
    ledger.last_action_ts = clock.unix_timestamp;
    ledger.bump = ctx.bumps.ledger;

    msg!("MemberLedger registered for participant {}", ledger.participant);
    Ok(())
}
