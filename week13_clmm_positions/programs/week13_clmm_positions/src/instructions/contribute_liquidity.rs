use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

use crate::errors::ReserveError;
use crate::state::{MemberLedger, ReservePool};

#[derive(Accounts)]
pub struct ContributeLiquidity<'info> {
    #[account(
        mut,
        seeds = [b"kinetix_pool", pool.base_mint.as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, ReservePool>,

    #[account(
        mut,
        seeds = [b"member_ledger", pool.key().as_ref(), participant.key().as_ref()],
        bump = ledger.bump,
        constraint = ledger.participant == participant.key() @ ReserveError::Unauthorized,
    )]
    pub ledger: Account<'info, MemberLedger>,

    #[account(
        mut,
        seeds = [b"reserve_vault", pool.base_mint.as_ref()],
        bump,
        token::mint = pool.base_mint,
        token::authority = pool,
    )]
    pub reserve_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"receipt_mint", pool.base_mint.as_ref()],
        bump,
        mint::authority = pool,
    )]
    pub receipt_mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = pool.base_mint,
        token::authority = participant,
    )]
    pub participant_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = receipt_mint,
        token::authority = participant,
    )]
    pub participant_receipt_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub participant: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ContributeLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, ReserveError::ZeroAmount);
    require!(!ctx.accounts.pool.paused, ReserveError::PoolPaused);

    let receipts_out = ctx
        .accounts
        .pool
        .compute_receipts_for_deposit(amount)
        .ok_or(ReserveError::MathOverflow)?;
    require!(receipts_out > 0, ReserveError::ReceiptsTooSmall);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.participant_token_account.to_account_info(),
                to: ctx.accounts.reserve_account.to_account_info(),
                authority: ctx.accounts.participant.to_account_info(),
            },
        ),
        amount,
    )?;

    let base_mint_key = ctx.accounts.pool.base_mint;
    let bump = ctx.accounts.pool.bump;
    let pool_seeds: &[&[&[u8]]] = &[&[b"kinetix_pool", base_mint_key.as_ref(), &[bump]]];

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.receipt_mint.to_account_info(),
                to: ctx.accounts.participant_receipt_account.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            pool_seeds,
        ),
        receipts_out,
    )?;

    let pool = &mut ctx.accounts.pool;
    pool.managed_assets = pool
        .managed_assets
        .checked_add(amount)
        .ok_or(ReserveError::MathOverflow)?;
    pool.issued_receipts = pool
        .issued_receipts
        .checked_add(receipts_out)
        .ok_or(ReserveError::MathOverflow)?;

    let clock = Clock::get()?;
    let ledger = &mut ctx.accounts.ledger;
    ledger.net_contributed = ledger
        .net_contributed
        .checked_add(amount)
        .ok_or(ReserveError::MathOverflow)?;
    ledger.receipt_balance = ledger
        .receipt_balance
        .checked_add(receipts_out)
        .ok_or(ReserveError::MathOverflow)?;
    ledger.last_action_ts = clock.unix_timestamp;

    Ok(())
}
