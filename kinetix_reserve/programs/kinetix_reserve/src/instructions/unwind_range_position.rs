use anchor_lang::prelude::*;
use anchor_spl::memo::Memo;
use anchor_spl::token::Token;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use raydium_clmm_cpi::{
    cpi,
    states::{PersonalPositionState, PoolState, TickArrayState},
};

use crate::errors::ReserveError;
use crate::state::ReservePool;

#[derive(Accounts)]
pub struct UnwindRangePosition<'info> {
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

    #[account(mut)]
    pub clmm_pool_state: AccountLoader<'info, PoolState>,

    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    #[account(mut)]
    pub position_nft_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Validated by Raydium CLMM CPI
    #[account(mut)]
    pub protocol_position: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = personal_position.pool_id == clmm_pool_state.key(),
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    #[account(mut)]
    pub token_vault_0: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub token_vault_1: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,

    #[account(mut)]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,

    pub vault_0_mint: InterfaceAccount<'info, Mint>,
    pub vault_1_mint: InterfaceAccount<'info, Mint>,

    /// CHECK: Verified against official Raydium CLMM program ID
    #[account(address = raydium_clmm_cpi::id())]
    pub clmm_program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub memo_program: Program<'info, Memo>,
    pub system_program: Program<'info, System>,
}

pub fn handler<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, UnwindRangePosition<'info>>,
    amount_0_min: u64,
    amount_1_min: u64,
) -> Result<()> {
    let base_mint_key = ctx.accounts.pool.base_mint;
    let bump = ctx.accounts.pool.bump;
    let pool_seeds: &[&[&[u8]]] = &[&[b"kinetix_pool", base_mint_key.as_ref(), &[bump]]];

    let liquidity = ctx.accounts.personal_position.liquidity;
    let (recipient_0, recipient_1) = if ctx.accounts.vault_0_mint.key() == base_mint_key {
        (
            ctx.accounts.reserve_account.to_account_info(),
            ctx.accounts.reserve_quote_account.to_account_info(),
        )
    } else {
        (
            ctx.accounts.reserve_quote_account.to_account_info(),
            ctx.accounts.reserve_account.to_account_info(),
        )
    };

    if liquidity > 0 {
        let decrease_accounts = cpi::accounts::DecreaseLiquidityV2 {
            nft_owner: ctx.accounts.pool.to_account_info(),
            nft_account: ctx.accounts.position_nft_account.to_account_info(),
            personal_position: ctx.accounts.personal_position.to_account_info(),
            pool_state: ctx.accounts.clmm_pool_state.to_account_info(),
            protocol_position: ctx.accounts.protocol_position.to_account_info(),
            token_vault_0: ctx.accounts.token_vault_0.to_account_info(),
            token_vault_1: ctx.accounts.token_vault_1.to_account_info(),
            tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
            tick_array_upper: ctx.accounts.tick_array_upper.to_account_info(),
            recipient_token_account_0: recipient_0,
            recipient_token_account_1: recipient_1,
            token_program: ctx.accounts.token_program.to_account_info(),
            token_program_2022: ctx.accounts.token_program_2022.to_account_info(),
            memo_program: ctx.accounts.memo_program.to_account_info(),
            vault_0_mint: ctx.accounts.vault_0_mint.to_account_info(),
            vault_1_mint: ctx.accounts.vault_1_mint.to_account_info(),
        };

        let decrease_ctx = CpiContext::new_with_signer(
            ctx.accounts.clmm_program.to_account_info(),
            decrease_accounts,
            pool_seeds,
        )
        .with_remaining_accounts(ctx.remaining_accounts.to_vec());

        cpi::decrease_liquidity_v2(decrease_ctx, liquidity, amount_0_min, amount_1_min)?;
    }

    let close_accounts = cpi::accounts::ClosePosition {
        nft_owner: ctx.accounts.pool.to_account_info(),
        position_nft_mint: ctx.accounts.position_nft_mint.to_account_info(),
        position_nft_account: ctx.accounts.position_nft_account.to_account_info(),
        personal_position: ctx.accounts.personal_position.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program_2022.to_account_info(),
    };

    cpi::close_position(CpiContext::new_with_signer(
        ctx.accounts.clmm_program.to_account_info(),
        close_accounts,
        pool_seeds,
    ))?;

    Ok(())
}
