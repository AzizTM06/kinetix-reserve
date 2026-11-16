use anchor_lang::prelude::*;

use crate::errors::ReserveError;
use crate::state::ReservePool;

#[derive(Accounts)]
pub struct SetPoolPause<'info> {
    #[account(
        mut,
        seeds = [b"kinetix_pool", pool.base_mint.as_ref()],
        bump = pool.bump,
        constraint = pool.curator == curator.key() @ ReserveError::Unauthorized,
    )]
    pub pool: Account<'info, ReservePool>,

    pub curator: Signer<'info>,
}

pub fn handler(ctx: Context<SetPoolPause>, paused: bool) -> Result<()> {
    ctx.accounts.pool.paused = paused;
    msg!("ReservePool paused={}", paused);
    Ok(())
}
