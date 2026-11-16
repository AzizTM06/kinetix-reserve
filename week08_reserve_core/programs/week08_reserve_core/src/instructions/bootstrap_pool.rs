use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::ReserveError;
use crate::state::{ReservePool, MAX_FEE_BPS};

#[derive(Accounts)]
pub struct BootstrapPool<'info> {
    #[account(
        init,
        payer = curator,
        space = ReservePool::SIZE,
        seeds = [b"kinetix_pool", base_mint.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, ReservePool>,

    #[account(
        init,
        payer = curator,
        token::mint = base_mint,
        token::authority = pool,
        seeds = [b"reserve_vault", base_mint.key().as_ref()],
        bump,
    )]
    pub reserve_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = curator,
        mint::decimals = 6,
        mint::authority = pool,
        seeds = [b"receipt_mint", base_mint.key().as_ref()],
        bump,
    )]
    pub receipt_mint: Account<'info, Mint>,

    pub base_mint: Account<'info, Mint>,

    #[account(mut)]
    pub curator: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<BootstrapPool>, fee_bps: u16) -> Result<()> {
    require!(fee_bps <= MAX_FEE_BPS, ReserveError::FeeTooHigh);

    let pool = &mut ctx.accounts.pool;
    pool.curator = ctx.accounts.curator.key();
    pool.base_mint = ctx.accounts.base_mint.key();
    pool.reserve_account = ctx.accounts.reserve_account.key();
    pool.receipt_mint = ctx.accounts.receipt_mint.key();
    pool.managed_assets = 0;
    pool.issued_receipts = 0;
    pool.fee_bps = fee_bps;
    pool.paused = false;
    pool.bump = ctx.bumps.pool;

    msg!("Kinetix ReservePool initialized for base mint {}", pool.base_mint);
    Ok(())
}
