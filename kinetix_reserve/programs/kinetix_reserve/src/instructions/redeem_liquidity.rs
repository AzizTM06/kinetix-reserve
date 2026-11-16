use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};

use crate::errors::ReserveError;
use crate::state::{MemberLedger, ReservePool};

#[derive(Accounts)]
pub struct RedeemLiquidity<'info> {
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

pub fn handler(ctx: Context<RedeemLiquidity>, receipts_amount: u64) -> Result<()> {
    require!(receipts_amount > 0, ReserveError::ZeroAmount);
    require!(ctx.accounts.pool.issued_receipts > 0, ReserveError::PoolEmpty);
    require!(
        ctx.accounts.ledger.receipt_balance >= receipts_amount,
        ReserveError::InsufficientReceipts
    );

    let (net_assets, fee_retained) = ctx
        .accounts
        .pool
        .compute_assets_for_redemption(receipts_amount)
        .ok_or(ReserveError::MathOverflow)?;
    require!(net_assets > 0, ReserveError::RedemptionTooSmall);

    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.receipt_mint.to_account_info(),
                from: ctx.accounts.participant_receipt_account.to_account_info(),
                authority: ctx.accounts.participant.to_account_info(),
            },
        ),
        receipts_amount,
    )?;

    let base_mint_key = ctx.accounts.pool.base_mint;
    let bump = ctx.accounts.pool.bump;
    let pool_seeds: &[&[&[u8]]] = &[&[b"kinetix_pool", base_mint_key.as_ref(), &[bump]]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.reserve_account.to_account_info(),
                to: ctx.accounts.participant_token_account.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            pool_seeds,
        ),
        net_assets,
    )?;

    let pool = &mut ctx.accounts.pool;
    // Retained exit fee stays inside managed_assets, accruing value to remaining receipt holders.
    pool.managed_assets = pool
        .managed_assets
        .checked_sub(net_assets)
        .ok_or(ReserveError::MathOverflow)?;
    pool.issued_receipts = pool
        .issued_receipts
        .checked_sub(receipts_amount)
        .ok_or(ReserveError::MathOverflow)?;

    let clock = Clock::get()?;
    let ledger = &mut ctx.accounts.ledger;
    ledger.receipt_balance = ledger
        .receipt_balance
        .checked_sub(receipts_amount)
        .ok_or(ReserveError::MathOverflow)?;
    ledger.net_contributed = ledger.net_contributed.saturating_sub(net_assets);
    ledger.last_action_ts = clock.unix_timestamp;

    msg!(
        "Redeemed {} receipts -> {} net assets (retained fee={})",
        receipts_amount,
        net_assets,
        fee_retained
    );
    Ok(())
}
