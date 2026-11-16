use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use raydium_clmm_cpi::{cpi, states::PoolState};

use crate::errors::ReserveError;
use crate::instructions::{clear_delegate, set_delegate};
use crate::state::ReservePool;

#[derive(Accounts)]
pub struct DeployRangePosition<'info> {
    #[account(mut)]
    pub curator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"kinetix_pool", pool.base_mint.as_ref()],
        bump = pool.bump,
        constraint = pool.curator == curator.key() @ ReserveError::Unauthorized,
    )]
    pub pool: Account<'info, ReservePool>,

    #[account(
        mut,
        seeds = [b"reserve_vault", pool.base_mint.as_ref()],
        bump,
        token::mint = pool.base_mint,
        token::authority = pool,
    )]
    pub reserve_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::authority = pool)]
    pub reserve_quote_account: InterfaceAccount<'info, TokenAccount>,

    pub quote_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub clmm_pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// CHECK: Initialized and validated by Raydium CLMM CPI
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// CHECK: Initialized and validated by Raydium CLMM CPI
    #[account(mut)]
    pub protocol_position: UncheckedAccount<'info>,

    /// CHECK: Initialized and validated by Raydium CLMM CPI
    #[account(mut)]
    pub personal_position: UncheckedAccount<'info>,

    /// CHECK: Validated by Raydium CLMM CPI
    #[account(mut)]
    pub tick_array_lower: UncheckedAccount<'info>,

    /// CHECK: Validated by Raydium CLMM CPI
    #[account(mut)]
    pub tick_array_upper: UncheckedAccount<'info>,

    #[account(mut)]
    pub token_vault_0: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub token_vault_1: InterfaceAccount<'info, TokenAccount>,

    pub vault_0_mint: InterfaceAccount<'info, Mint>,
    pub vault_1_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Verified against official Raydium CLMM program ID
    #[account(address = raydium_clmm_cpi::id())]
    pub clmm_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, DeployRangePosition<'info>>,
    tick_lower_index: i32,
    tick_upper_index: i32,
    tick_array_lower_start_index: i32,
    tick_array_upper_start_index: i32,
    liquidity: u128,
    amount_0_max: u64,
    amount_1_max: u64,
) -> Result<()> {
    require!(tick_lower_index < tick_upper_index, ReserveError::InvalidTickRange);
    require!(!ctx.accounts.pool.paused, ReserveError::PoolPaused);

    let base_mint_key = ctx.accounts.pool.base_mint;
    let bump = ctx.accounts.pool.bump;
    let pool_seeds: &[&[&[u8]]] = &[&[b"kinetix_pool", base_mint_key.as_ref(), &[bump]]];

    let pool_mint_0 = ctx.accounts.vault_0_mint.key();
    let (token_account_0, token_account_1, base_max, quote_max) = if pool_mint_0 == base_mint_key {
        (
            ctx.accounts.reserve_account.to_account_info(),
            ctx.accounts.reserve_quote_account.to_account_info(),
            amount_0_max,
            amount_1_max,
        )
    } else {
        (
            ctx.accounts.reserve_quote_account.to_account_info(),
            ctx.accounts.reserve_account.to_account_info(),
            amount_1_max,
            amount_0_max,
        )
    };

    set_delegate(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.reserve_account.to_account_info(),
        ctx.accounts.curator.to_account_info(),
        ctx.accounts.pool.to_account_info(),
        pool_seeds,
        base_max,
    )?;

    set_delegate(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.reserve_quote_account.to_account_info(),
        ctx.accounts.curator.to_account_info(),
        ctx.accounts.pool.to_account_info(),
        pool_seeds,
        quote_max,
    )?;

    let remaining = ctx.remaining_accounts.to_vec();
    let cpi_accounts = cpi::accounts::OpenPositionWithToken22Nft {
        payer: ctx.accounts.curator.to_account_info(),
        position_nft_owner: ctx.accounts.pool.to_account_info(),
        position_nft_mint: ctx.accounts.position_nft_mint.to_account_info(),
        position_nft_account: ctx.accounts.position_nft_account.to_account_info(),
        pool_state: ctx.accounts.clmm_pool_state.to_account_info(),
        protocol_position: ctx.accounts.protocol_position.to_account_info(),
        tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
        tick_array_upper: ctx.accounts.tick_array_upper.to_account_info(),
        personal_position: ctx.accounts.personal_position.to_account_info(),
        token_account_0,
        token_account_1,
        token_vault_0: ctx.accounts.token_vault_0.to_account_info(),
        token_vault_1: ctx.accounts.token_vault_1.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
        vault_0_mint: ctx.accounts.vault_0_mint.to_account_info(),
        vault_1_mint: ctx.accounts.vault_1_mint.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.clmm_program.to_account_info(),
        cpi_accounts,
        pool_seeds,
    )
    .with_remaining_accounts(remaining);

    cpi::open_position_with_token22_nft(
        cpi_ctx,
        tick_lower_index,
        tick_upper_index,
        tick_array_lower_start_index,
        tick_array_upper_start_index,
        liquidity,
        amount_0_max,
        amount_1_max,
        true,
        Some(false),
    )?;

    clear_delegate(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.reserve_account.to_account_info(),
        ctx.accounts.pool.to_account_info(),
        pool_seeds,
    )?;

    clear_delegate(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.reserve_quote_account.to_account_info(),
        ctx.accounts.pool.to_account_info(),
        pool_seeds,
    )?;

    Ok(())
}
