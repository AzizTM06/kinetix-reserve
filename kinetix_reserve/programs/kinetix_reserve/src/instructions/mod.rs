use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Approve, Revoke};

pub mod bootstrap_pool;
pub mod contribute_liquidity;
pub mod deploy_range_position;
pub mod execute_rebalance_swap;
pub mod redeem_liquidity;
pub mod register_member;
pub mod set_pool_pause;
pub mod unwind_range_position;

pub use bootstrap_pool::*;
pub use contribute_liquidity::*;
pub use deploy_range_position::*;
pub use execute_rebalance_swap::*;
pub use redeem_liquidity::*;
pub use register_member::*;
pub use set_pool_pause::*;
pub use unwind_range_position::*;

pub fn set_delegate<'info>(
    token_program: AccountInfo<'info>,
    source_account: AccountInfo<'info>,
    delegate: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    token_interface::approve(
        CpiContext::new_with_signer(
            token_program,
            Approve {
                to: source_account,
                delegate,
                authority,
            },
            signer_seeds,
        ),
        amount,
    )
}

pub fn clear_delegate<'info>(
    token_program: AccountInfo<'info>,
    source_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    token_interface::revoke(CpiContext::new_with_signer(
        token_program,
        Revoke {
            source: source_account,
            authority,
        },
        signer_seeds,
    ))
}
