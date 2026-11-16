use anchor_lang::prelude::*;
use anchor_spl::memo::Memo;
use anchor_spl::token::Token;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use raydium_clmm_cpi::{
    cpi,
    states::{AmmConfig, ObservationState, PoolState},
};

use crate::errors::ReserveError;
use crate::instructions::{clear_delegate, set_delegate};
use crate::state::ReservePool;

#[derive(Accounts)]
pub struct ExecuteRebalanceSwap<'info> {
    #[account(mut)]
    pub curator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"kinetix_pool", pool.base_mint.as_ref()],
        bump = pool.bump,
        constraint = pool.curator == curator.key() @ ReserveError::Unauthorized,
    )]
    pub pool: Account<'info, ReservePool>,

    #[account(mut, token::authority = pool)]
    pub reserve_input_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::authority = pool)]
    pub reserve_output_account: InterfaceAccount<'info, TokenAccount>,

    pub amm_config: Box<Account<'info, AmmConfig>>,

    #[account(mut)]
    pub clmm_pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub input_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub output_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub observation_state: AccountLoader<'info, ObservationState>,

    pub input_vault_mint: Box<InterfaceAccount<'info, Mint>>,
    pub output_vault_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: Verified against official Raydium CLMM program ID
    #[account(address = raydium_clmm_cpi::id())]
    pub clmm_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub memo_program: Program<'info, Memo>,
}

pub fn handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ExecuteRebalanceSwap<'info>>,
    amount_in: u64,
    min_amount_out: u64,
    sqrt_price_limit_x64: u128,
) -> Result<()> {
    require!(amount_in > 0, ReserveError::ZeroAmount);
    require!(!ctx.accounts.pool.paused, ReserveError::PoolPaused);

    let base_mint_key = ctx.accounts.pool.base_mint;
    let bump = ctx.accounts.pool.bump;
    let pool_seeds: &[&[&[u8]]] = &[&[b"kinetix_pool", base_mint_key.as_ref(), &[bump]]];

    set_delegate(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.reserve_input_account.to_account_info(),
        ctx.accounts.curator.to_account_info(),
        ctx.accounts.pool.to_account_info(),
        pool_seeds,
        amount_in,
    )?;

    let remaining = ctx.remaining_accounts.to_vec();
    let cpi_accounts = cpi::accounts::SwapSingleV2 {
        payer: ctx.accounts.curator.to_account_info(),
        amm_config: ctx.accounts.amm_config.to_account_info(),
        pool_state: ctx.accounts.clmm_pool_state.to_account_info(),
        input_token_account: ctx.accounts.reserve_input_account.to_account_info(),
        output_token_account: ctx.accounts.reserve_output_account.to_account_info(),
        input_vault: ctx.accounts.input_vault.to_account_info(),
        output_vault: ctx.accounts.output_vault.to_account_info(),
        observation_state: ctx.accounts.observation_state.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
        memo_program: ctx.accounts.memo_program.to_account_info(),
        input_vault_mint: ctx.accounts.input_vault_mint.to_account_info(),
        output_vault_mint: ctx.accounts.output_vault_mint.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.clmm_program.to_account_info(),
        cpi_accounts,
        pool_seeds,
    )
    .with_remaining_accounts(remaining);

    cpi::swap_v2(cpi_ctx, amount_in, min_amount_out, sqrt_price_limit_x64, true)?;

    clear_delegate(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.reserve_input_account.to_account_info(),
        ctx.accounts.pool.to_account_info(),
        pool_seeds,
    )?;

    if ctx.accounts.reserve_input_account.mint == base_mint_key {
        let pool = &mut ctx.accounts.pool;
        pool.managed_assets = pool.managed_assets.saturating_sub(amount_in);
    }

    Ok(())
}
